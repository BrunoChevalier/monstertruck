---
phase: 10-test-infrastructure-and-healing-hooks
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/healing/surface_healing.rs
  - monstertruck-solid/src/healing/mod.rs
  - monstertruck-solid/src/lib.rs
autonomous: true
must_haves:
  truths:
    - "User calls heal_surface_shell on a CompressedShell built from sweep_rail and gets a topologically valid Shell back"
    - "User calls heal_surface_shell on a CompressedShell built from birail and gap edges are detected and repaired"
    - "User calls heal_surface_shell on a CompressedShell built from gordon and degenerate edges are split"
    - "Healing hooks compose with existing SplitClosedEdgesAndFaces without conflict"
    - "Healing returns a Result with diagnostic information when repair is not possible"
  artifacts:
    - path: "monstertruck-solid/src/healing/surface_healing.rs"
      provides: "Topology healing hooks for surfaces from sweep_rail, birail, and gordon constructors"
      min_lines: 200
      contains: "fn heal_surface_shell"
    - path: "monstertruck-solid/src/healing/mod.rs"
      provides: "Updated healing module with surface_healing submodule and public exports"
      min_lines: 170
      contains: "mod surface_healing"
    - path: "monstertruck-solid/src/lib.rs"
      provides: "Updated lib.rs re-exporting healing hooks"
      min_lines: 43
      contains: "heal_surface_shell"
  key_links:
    - from: "monstertruck-solid/src/healing/surface_healing.rs"
      to: "monstertruck-solid/src/healing/mod.rs"
      via: "Module import and trait re-export"
      pattern: "mod surface_healing"
    - from: "monstertruck-solid/src/healing/mod.rs"
      to: "monstertruck-solid/src/lib.rs"
      via: "Public API re-export"
      pattern: "pub use healing"
---

<objective>
Implement topological healing hooks in monstertruck-solid that detect and repair topology issues introduced by sweep_rail, birail, and gordon surface constructors (BOOL-02). The hooks compose with existing healing (SplitClosedEdgesAndFaces) and handle gap edges, degenerate boundaries, and vertex mismatches.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/healing/mod.rs
@monstertruck-solid/src/healing/split_closed_edges.rs
@monstertruck-solid/src/healing/split_closed_faces.rs
@monstertruck-solid/src/transversal/integrate/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Implement surface healing module with gap detection and repair</name>
  <files>monstertruck-solid/src/healing/surface_healing.rs</files>
  <action>
Create `monstertruck-solid/src/healing/surface_healing.rs` implementing the core healing logic for surfaces produced by sweep_rail, birail, and gordon constructors.

The module should implement these healing strategies:

**1. Gap edge detection and welding:**
```rust
/// Detects pairs of edges in a [`CompressedShell`] where endpoints are within
/// `tol` of each other but reference different vertex indices. Welds them
/// by merging the vertex indices (keeping the lower index, remapping the higher).
fn weld_gap_edges<C, S>(shell: &mut CompressedShell<Point3, C, S>, tol: f64) -> usize
```
This handles the primary failure mode of surface constructors: adjacent faces share edges geometrically but not topologically. The function should:
- Build a spatial index of all vertex positions.
- Find vertex pairs within `tol` distance.
- Use union-find to merge vertex clusters.
- Remap all edge vertex indices.
- Return the count of welded vertex pairs.

**2. Degenerate edge removal:**
```rust
/// Removes edges whose two endpoints (after welding) are the same vertex
/// AND whose curve length is below `tol`. These arise from collapsed surface
/// edges (e.g., pole points on swept surfaces).
fn remove_degenerate_edges<C, S>(shell: &mut CompressedShell<Point3, C, S>, tol: f64) -> usize
where C: BoundedCurve<Point = Point3> + ParameterDivision1D<Point = Point3>
```
Should remove the edge from all face boundary wires and compact the edge array (updating indices).

**3. Boundary consistency check:**
```rust
/// Verifies that every edge appears exactly twice across all face boundaries
/// (once forward, once reversed) for a manifold shell. Returns edges that
/// violate this, which indicates gaps the welding couldn't fix.
fn find_non_manifold_edges<C, S>(shell: &CompressedShell<Point3, C, S>) -> Vec<usize>
```

**4. Public healing entry point:**
```rust
/// Applies a sequence of healing operations to a [`CompressedShell`] produced
/// by surface construction (`sweep_rail`, `birail`, `gordon`).
///
/// Steps:
/// 1. Weld gap edges (merge near-coincident vertices).
/// 2. Split closed edges and faces (delegate to existing healing).
/// 3. Remove degenerate edges.
/// 4. Verify manifold consistency.
///
/// Returns `Ok(Shell)` if healing succeeds, or `Err` with diagnostic info.
pub fn heal_surface_shell<C, S>(
    mut cshell: CompressedShell<Point3, C, S>,
    tol: f64,
) -> Result<Shell<Point3, C, S>, SurfaceHealingError>
```

The `SurfaceHealingError` enum should include:
- `NonManifoldEdges { edge_indices: Vec<usize> }` -- edges that couldn't be healed.
- `TopologyExtractFailed { source: monstertruck_topology::errors::Error }` -- `Shell::extract` failed after healing.
- `TooManyGaps { gap_count: usize, threshold: usize }` -- too many gaps suggests fundamentally broken geometry.

Include trait bounds that match the existing healing module patterns (see `extract_healed` in mod.rs). The bounds should require:
- `C: ParametricCurve3D + BoundedCurve + Cut + ParameterDivision1D<Point = Point3> + SearchNearestParameter<D1, Point = Point3> + TryFrom<ParameterCurve<Line<Point2>, S>> + Clone`
- `S: ParametricSurface3D + SearchParameter<D2, Point = Point3> + SearchNearestParameter<D2, Point = Point3> + Clone`

For the union-find in vertex welding, implement a simple in-module `UnionFind` struct (no external dependency needed) with `find` and `union` operations.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` to confirm the module compiles with all trait bounds satisfied. Run clippy: `cargo clippy --all-targets -- -W warnings`.</verify>
  <done>Surface healing module implemented with gap welding, degenerate edge removal, manifold checking, and a public `heal_surface_shell` entry point.</done>
</task>

<task type="auto">
  <name>Task 2: Wire up surface healing module and add unit tests</name>
  <files>monstertruck-solid/src/healing/mod.rs, monstertruck-solid/src/lib.rs</files>
  <action>
Update `monstertruck-solid/src/healing/mod.rs`:
1. Add `mod surface_healing;` (NOT cfg(test) -- this is production code).
2. Add `pub use surface_healing::{heal_surface_shell, SurfaceHealingError};`.
3. Keep all existing exports intact (`SplitClosedEdgesAndFaces`, `RobustSplitClosedEdgesAndFaces`, `extract_healed`).

Update `monstertruck-solid/src/lib.rs`:
1. Add `heal_surface_shell` and `SurfaceHealingError` to the `pub use healing::` line.
2. Update the module doc comment to mention the new healing hooks for surface constructors.

The existing `extract_healed` function should remain unchanged. The new `heal_surface_shell` is a parallel entry point that applies additional surface-constructor-specific healing before the standard split-closed-edges pass.

Add unit tests in `surface_healing.rs` (at the bottom, `#[cfg(test)] mod tests`):
1. Test `weld_gap_edges` with a simple 2-face shell where vertices are offset by 1e-8.
2. Test `remove_degenerate_edges` with a face containing a zero-length edge.
3. Test `find_non_manifold_edges` on a valid manifold shell (returns empty) and an open shell (returns the boundary edges).
4. Test the full `heal_surface_shell` pipeline on a simple box-like CompressedShell with deliberate vertex gaps.

These unit tests should use `CompressedShell` directly (like the existing healing tests do), not the fixture corpus (which is plan 10-1's responsibility).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib -E 'test(healing::surface_healing)'` to verify all unit tests pass. Run `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` to verify no regressions in existing tests. Run `cargo clippy --all-targets -- -W warnings` to verify no warnings. Run `cargo doc -p monstertruck-solid --no-deps` to verify docs build.</verify>
  <done>Surface healing module wired into healing/mod.rs and lib.rs, public API exports updated, 4+ unit tests passing for the healing pipeline.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid --lib -E 'test(healing::surface_healing)'` passes all unit tests
2. `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` passes with no regressions
3. `cargo clippy --all-targets -- -W warnings` produces no new warnings
4. `heal_surface_shell` is accessible from external crates via `monstertruck_solid::heal_surface_shell`
5. The healing pipeline composes correctly: weld -> split-closed -> remove-degenerate -> verify-manifold
6. `SurfaceHealingError` provides diagnostic information for debugging
</verification>

<success_criteria>
- BOOL-02 is satisfied: topological healing hooks exist in monstertruck-solid for new surface constructors
- heal_surface_shell detects and repairs topology gaps from sweep_rail, birail, and gordon
- Existing healing functionality (SplitClosedEdgesAndFaces, extract_healed) is unmodified
- Unit tests verify each healing step independently and the full pipeline
</success_criteria>

<output>
After completion, create `.tendrion/phases/10-test-infrastructure-and-healing-hooks/10-2-SUMMARY.md`
</output>
