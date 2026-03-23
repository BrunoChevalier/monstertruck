---
phase: 31-deferred-ayam-port-completion
plan: 2
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs
  - monstertruck-meshing/tests/tessellation/main.rs
autonomous: true
must_haves:
  truths:
    - "User tessellates a surface with a near-zero-area trim loop and gets a valid mesh without panics"
    - "User tessellates a surface with self-touching trim boundaries and the output is a valid polygon mesh"
    - "User tessellates a trimmed surface and boundary edges are watertight within configured tolerance"
    - "User calls robust_triangulation on a shell with degenerate trims and no faces are silently dropped"
  artifacts:
    - path: "monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs"
      provides: "Integration tests for degenerate trim boundary handling"
      min_lines: 120
      contains: "near_zero_area"
    - path: "monstertruck-meshing/src/tessellation/triangulation.rs"
      provides: "Hardened trim tessellation with fallback heuristics for degenerate boundaries"
      min_lines: 1500
      contains: "degenerate"
  key_links:
    - from: "monstertruck-meshing/src/tessellation/triangulation.rs"
      to: "monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs"
      via: "PolyBoundary::new and trimming_tessellation handle degenerate loops"
      pattern: "trimming_tessellation"
    - from: "monstertruck-meshing/src/tessellation/triangulation.rs"
      to: "monstertruck-meshing/src/tessellation/mod.rs"
      via: "robust_triangulation delegates to shell_tessellation with allow_fallback=true"
      pattern: "allow_fallback"
---

<objective>
Harden trim tessellation to handle degenerate trimming boundary cases (near-zero-area loops, self-touching boundaries, collapsed edges) without panics, and ensure watertight output at boundary edges within tolerance. This extends Phase 19's UV interpolation fallback with structural robustness for pathological trim geometries required by PORT-02.
</objective>

<execution_context>
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-meshing/src/tessellation/triangulation.rs
@monstertruck-meshing/src/tessellation/mod.rs
@monstertruck-meshing/src/tessellation/boundary_stitching.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: RED - Write failing tests for degenerate trim boundary cases</name>
  <files>monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs, monstertruck-meshing/tests/tessellation/main.rs</files>
  <action>
Create a new test module `degenerate_trim_test.rs` and register it in `main.rs`.

Add `mod degenerate_trim_test;` to `monstertruck-meshing/tests/tessellation/main.rs`.

**monstertruck-meshing/tests/tessellation/degenerate_trim_test.rs:**

Build test helpers that construct `Shell` objects with known degenerate trim patterns using the modeling builder API:

1. `near_zero_area_trim_loop_no_panic` - Construct a face with a trim loop whose UV-space area is extremely small (e.g., a triangular trim with vertices within 1e-8 of each other). Call `robust_triangulation`. Assert the result does not panic and the face's polygon mesh is either `Some` (valid mesh) or `None` (gracefully skipped), but never a panic or infinite loop.

2. `self_touching_trim_boundary_no_panic` - Construct a face whose trim boundary forms a figure-8 or bowtie shape where the boundary touches itself at a point (but does not cross). Call `robust_triangulation`. Assert no panic and valid mesh output.

3. `collapsed_edge_in_trim_wire_no_panic` - Construct a face with a trim wire containing an edge where start == end (zero-length edge). Call `robust_triangulation`. Assert no panic.

4. `watertight_boundary_after_degenerate_trim` - Construct a simple shell (e.g., two adjacent planar faces sharing an edge) where one face has a small trim hole near the shared edge. Tessellate both faces. Verify that after `put_together_same_attrs` and `remove_degenerate_faces`, the shared boundary positions match exactly (watertight).

5. `robust_vs_regular_degenerate_trim` - Compare `triangulation` vs `robust_triangulation` on a surface with degenerate trim. The robust version should recover more faces than the regular version.

Use builder API (`builder::try_attach_plane`, `builder::vertex`, `builder::line`, etc.) to construct test geometries. Import `ShellCondition` for validation where applicable.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-meshing --test tessellation`. Tests should compile; some should fail or panic, demonstrating the degenerate handling gaps.</verify>
  <done>Failing tests created covering near-zero-area loops, self-touching boundaries, collapsed edges, watertightness, and robust vs regular comparison.</done>
</task>

<task type="auto">
  <name>Task 2: GREEN - Implement degenerate trim boundary handling</name>
  <files>monstertruck-meshing/src/tessellation/triangulation.rs</files>
  <action>
Fix failures by hardening the trim tessellation pipeline:

1. **Near-zero-area loop detection in `PolyBoundary::new`**: After assembling closed loops, compute the signed area of each loop in UV space using the shoelace formula (already implemented as `loop_orientation`). If a loop's absolute area is below a threshold (e.g., `UV_CLOSURE_TOLERANCE * UV_CLOSURE_TOLERANCE`), skip it instead of inserting it into the CDT. Log a warning via `log::warn!`.

2. **Self-touching boundary handling in `insert_to`**: When inserting constraint edges into the CDT via `triangulation.add_constraint(vi, vj)`, the `can_add_constraint` check already handles some conflicts. Add additional handling: if a boundary point coincides with another boundary point from the same loop (self-touching), skip the duplicate and merge the vertex handles. Use the existing `spade_round` function to detect near-coincident points.

3. **Collapsed edge handling in `PolyBoundaryPiece::try_new_inner`**: After building `bdry3d`, filter out consecutive duplicate points (within `TOLERANCE`) before UV parameter search. This prevents zero-length boundary segments from causing issues in the CDT.

4. **Guard `cdt_trimming_tessellation` against empty boundaries**: If after degenerate filtering `polyboundary.loops` is empty, fall back to untrimmed tessellation using the surface's full domain (if available) rather than attempting CDT with no constraints.

5. **Guard `trimming_tessellation` against CDT panics**: Wrap the CDT insertion in a `std::panic::catch_unwind` as a last resort for any remaining edge cases. If the CDT panics, log a warning and return a simple 2-triangle quad covering the UV domain.

Key: All modifications should preserve backward compatibility. The `allow_fallback=false` path should behave identically to before for non-degenerate inputs.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-meshing --test tessellation` -- all tests pass including the new degenerate tests. Also run `cargo nextest run -p monstertruck-meshing` to verify no regressions in existing tests (solid_is_closed, compare_occt_mesh, etc.).</verify>
  <done>All degenerate trim boundary tests pass. No regressions in existing tessellation tests.</done>
</task>

<task type="auto">
  <name>Task 3: REFACTOR - Clean up and verify watertightness</name>
  <files>monstertruck-meshing/src/tessellation/triangulation.rs, monstertruck-meshing/src/tessellation/mod.rs</files>
  <action>
1. Extract the near-zero-area loop detection into a named helper function `is_degenerate_loop(loop: &[SurfacePoint]) -> bool` with clear documentation explaining the threshold and rationale.

2. Extract the collapsed-edge filtering into a named helper `remove_collapsed_edges(points: &mut Vec<Point3>, tolerance: f64)` in the `PolyBoundaryPiece` impl.

3. Add doc comments to all new/modified functions explaining the degenerate handling strategy.

4. Run `cargo clippy -p monstertruck-meshing --all-targets -- -W warnings` and fix any warnings.

5. Verify watertightness: ensure the `stitch_boundaries` function in `boundary_stitching.rs` handles cases where a degenerate face was skipped (mesh is `None`) without affecting the stitching of adjacent valid faces.

6. Run the full meshing test suite to confirm stability.
  </action>
  <verify>Run `cargo clippy -p monstertruck-meshing --all-targets -- -W warnings` with zero warnings. Run `cargo nextest run -p monstertruck-meshing` with all tests passing. Verify `solid_is_closed` test still passes (watertightness of standard shapes).</verify>
  <done>Code refactored with named helpers, doc comments added, clippy clean, all tests passing, watertightness verified.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-meshing --test tessellation` -- all tests pass including degenerate cases
2. `cargo nextest run -p monstertruck-meshing` -- no regressions
3. `cargo clippy -p monstertruck-meshing --all-targets -- -W warnings` -- clean
4. Near-zero-area trim loops are gracefully skipped with a warning log
5. Self-touching boundaries produce valid mesh output without panics
6. Boundary stitching produces watertight output on shared edges
7. `solid_is_closed` test continues to pass (baseline watertightness)
</verification>

<success_criteria>
- Trim tessellation handles degenerate trimming boundary cases (near-zero-area loops, self-touching boundaries) without panics (PORT-02, success criterion 3)
- Trimmed surface tessellation output is watertight at boundary edges within configured tolerance (success criterion 4)
- Robust triangulation recovers more faces than regular triangulation on degenerate inputs
- All existing tessellation tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/31-deferred-ayam-port-completion/31-2-SUMMARY.md`
</output>
