---
phase: 21-edge-identity-and-topology-repair
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/topology.rs
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "ensure_cuttable_edge mutates the edge curve in-place so edge.is_same() returns true before and after conversion"
    - "cut_face_by_bezier boundary replacement finds converted edges via is_same() on the original edge references"
    - "convert_shell_in matches boolean-origin edges with ~1e-5 positional noise without false-negative rejection"
    - "All existing fillet tests continue to pass after both changes"
    - "New test verifies ensure_cuttable_edge preserves identity (same EdgeId before and after)"
    - "New test verifies convert_shell_in tolerant matching with SNAP_TOLERANCE"
  artifacts:
    - path: "monstertruck-solid/src/fillet/topology.rs"
      provides: "In-place edge curve mutation in ensure_cuttable_edge"
      min_lines: 30
      contains: "set_curve"
    - path: "monstertruck-solid/src/fillet/convert.rs"
      provides: "SNAP_TOLERANCE-based endpoint matching in convert_shell_in"
      min_lines: 120
      contains: "SNAP_TOLERANCE"
  key_links:
    - from: "monstertruck-solid/src/fillet/topology.rs"
      to: "monstertruck-solid/src/fillet/convert.rs"
      via: "ensure_cuttable_edge feeds edges into the same pipeline that convert_shell_in initializes"
      pattern: "ensure_cuttable_edge"
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-solid/src/fillet/convert.rs"
      via: "SNAP_TOLERANCE imported for widened endpoint matching"
      pattern: "SNAP_TOLERANCE"
---

<objective>
Fix edge identity preservation in the fillet conversion pipeline by making ensure_cuttable_edge mutate edges in-place (preserving EdgeId for is_same() checks) and widening endpoint matching tolerance in convert_shell_in from TOLERANCE (1e-6) to SNAP_TOLERANCE (1e-5) for boolean-origin edges.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/fillet/topology.rs
@monstertruck-solid/src/fillet/convert.rs
@monstertruck-core/src/tolerance_constants.rs
@monstertruck-core/src/tolerance.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Fix ensure_cuttable_edge to preserve edge identity via in-place mutation</name>
  <files>monstertruck-solid/src/fillet/topology.rs</files>
  <action>
Modify `ensure_cuttable_edge` in `monstertruck-solid/src/fillet/topology.rs` (lines 34-45) to mutate the edge's curve in-place using `edge.set_curve()` instead of creating a new Edge with `Edge::new()`.

Current code (BROKEN - creates new Edge, breaks is_same()):
```rust
fn ensure_cuttable_edge(edge: &Edge) -> Edge {
    if matches!(edge.curve(), Curve::IntersectionCurve(_)) {
        let nurbs = edge.curve().to_nurbs_curve();
        Edge::new(
            edge.absolute_front(),
            edge.absolute_back(),
            Curve::NurbsCurve(nurbs),
        )
    } else {
        edge.clone()
    }
}
```

New code (FIXED - mutates in-place, preserves EdgeId):
```rust
fn ensure_cuttable_edge(edge: &Edge) -> Edge {
    if matches!(edge.curve(), Curve::IntersectionCurve(_)) {
        let nurbs = edge.curve().to_nurbs_curve();
        edge.set_curve(Curve::NurbsCurve(nurbs));
    }
    edge.clone()
}
```

Key behavioral changes:
1. `edge.set_curve()` writes the NURBS curve into the existing Edge's Arc-RwLock, so the EdgeId is preserved.
2. Both branches now return `edge.clone()`, which shares the same identity (EdgeId).
3. Because `set_curve` modifies the shared curve through the RwLock, ALL clones of this edge (including the one stored in the face boundary) will see the updated curve. This means `is_same()` will return true when `cut_face_by_bezier` searches the boundary at lines 96-97.

IMPORTANT: After this change, the `cuttable_front` and `cuttable_back` variables in `cut_face_by_bezier` (lines 57-58) are now clones of the SAME edge (same EdgeId) as `front_edge` and `back_edge`. The boundary replacement at lines 96-97 (`e.is_same(&front_edge)` / `e.is_same(&back_edge)`) will correctly find the edges because identity is preserved.

Also note: since `set_curve` mutates through the RwLock, the boundary edges in `face.boundary_iters()` (line 53, used by `find_adjacent_edge`) will also have their curves updated. This is correct behavior -- the edges in the boundary now have NURBS curves, which is exactly what we want for the cutting operations.
  </action>
  <verify>
Run `cargo check -p monstertruck-solid` to verify the change compiles.
Run existing fillet tests: `cargo test -p monstertruck-solid -- fillet --no-fail-fast` to verify no regressions.
  </verify>
  <done>ensure_cuttable_edge uses set_curve() for in-place mutation, preserving EdgeId through the fillet pipeline.</done>
</task>

<task type="auto">
  <name>Task 2: Widen endpoint matching tolerance in convert_shell_in to SNAP_TOLERANCE</name>
  <files>monstertruck-solid/src/fillet/convert.rs</files>
  <action>
Modify `convert_shell_in` in `monstertruck-solid/src/fillet/convert.rs` (lines 148-162) to use `SNAP_TOLERANCE` (1e-5) instead of the default `near()` which uses `TOLERANCE` (1e-6).

The `near()` method from the `Tolerance` trait calls `abs_diff_eq(other, TOLERANCE)` where `TOLERANCE = 1e-6`. For edges originating from boolean operations with IntersectionCurve geometry, endpoint positions have inherent noise at ~1e-5 magnitude.

Step 1: Add the import for SNAP_TOLERANCE at the top of convert.rs. Add after the existing imports (around line 1-7):
```rust
use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
```

Step 2: Replace `f.near(ext_front)` and related calls with `f.abs_diff_eq(ext_front, SNAP_TOLERANCE)` in the endpoint matching closure (lines 153-157).

Current code:
```rust
.find(|ie| {
    let f = ie.absolute_front().point();
    let b = ie.absolute_back().point();
    (f.near(ext_front) && b.near(ext_back))
        || (f.near(ext_back) && b.near(ext_front))
})
```

New code:
```rust
.find(|ie| {
    let f = ie.absolute_front().point();
    let b = ie.absolute_back().point();
    (f.abs_diff_eq(ext_front, SNAP_TOLERANCE) && b.abs_diff_eq(ext_back, SNAP_TOLERANCE))
        || (f.abs_diff_eq(ext_back, SNAP_TOLERANCE) && b.abs_diff_eq(ext_front, SNAP_TOLERANCE))
})
```

This uses `abs_diff_eq` from the `approx` crate (already in scope via `AbsDiffEq` trait bound on `Tolerance`) with the explicit `SNAP_TOLERANCE = 1e-5` epsilon, providing the necessary slack for boolean-origin edge positions.
  </action>
  <verify>
Run `cargo check -p monstertruck-solid` to verify the change compiles and SNAP_TOLERANCE import resolves.
Run `cargo test -p monstertruck-solid -- fillet --no-fail-fast` to verify no regressions.
  </verify>
  <done>convert_shell_in endpoint matching uses SNAP_TOLERANCE (1e-5) instead of TOLERANCE (1e-6), accommodating boolean-origin positional noise.</done>
</task>

<task type="auto">
  <name>Task 3: Add targeted tests for edge identity preservation and tolerant matching</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
Add two new test functions to `monstertruck-solid/src/fillet/tests.rs` that directly validate the two fixes.

Test 1: `ensure_cuttable_edge_preserves_identity`
This test verifies that `ensure_cuttable_edge` preserves edge identity when converting IntersectionCurve edges.

```rust
#[test]
fn ensure_cuttable_edge_preserves_identity() {
    // Create an edge with an IntersectionCurve geometry.
    // After ensure_cuttable_edge, the returned edge must have the same EdgeId
    // (is_same returns true), and the curve must be NurbsCurve.
    use super::topology::ensure_cuttable_edge;
    // ... build a minimal IntersectionCurve edge using test fixtures ...
    // Call ensure_cuttable_edge
    // Assert: original.is_same(&result) == true
    // Assert: matches!(result.curve(), Curve::NurbsCurve(_))
}
```

IMPORTANT: Check visibility. `ensure_cuttable_edge` is currently `fn` (private). To test it, either:
- Change visibility to `pub(super)` so tests (which are `mod tests` inside `super`) can access it, OR
- Add `#[cfg(test)]` pub(super) visibility.

Preferred approach: Change `fn ensure_cuttable_edge` to `pub(super) fn ensure_cuttable_edge` in topology.rs. This is consistent with other functions in the same file (e.g., `find_adjacent_edge`, `take_ori` are all `pub(super)`).

For building the test IntersectionCurve edge, look at existing test patterns in the file. The test at line ~3201 (`ensures ensure_cuttable_edge works correctly`) already exercises this path -- check if it validates identity preservation. If it only checks curve conversion (not identity), add the `is_same` assertion.

If no convenient IntersectionCurve edge fixture exists, construct one minimally:
- Create two simple NURBS surfaces (planes)
- Create a ParameterCurve (line in UV space on surface0)
- Wrap as IntersectionCurve
- Create Edge with this curve
- Call ensure_cuttable_edge
- Assert identity preserved

Test 2: `convert_shell_in_tolerant_endpoint_matching`
This test verifies that convert_shell_in successfully matches edges whose endpoints differ by up to SNAP_TOLERANCE.

Look at the existing test at line ~1787 (`convert_shell_in successfully converts a boolean AND result`). Extend or add a companion test that:
- Creates a shell with known edge endpoints
- Creates external edge references with endpoints offset by ~5e-6 (between TOLERANCE and SNAP_TOLERANCE)
- Verifies convert_shell_in succeeds (would have failed with old TOLERANCE)

If constructing offset endpoints is complex, a simpler approach is to verify the existing boolean-result test passes (it may have been failing before the tolerance fix). In that case, add a comment documenting that the test exercises SNAP_TOLERANCE matching.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- ensure_cuttable_edge_preserves_identity` to verify the identity test passes.
Run `cargo test -p monstertruck-solid -- convert_shell_in` to verify the tolerance test passes.
Run full fillet test suite: `cargo test -p monstertruck-solid -- fillet --no-fail-fast` to verify no regressions.
  </verify>
  <done>Tests validate edge identity preservation in ensure_cuttable_edge and tolerant endpoint matching in convert_shell_in.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-solid` compiles without errors or warnings
2. `cargo test -p monstertruck-solid -- fillet --no-fail-fast` -- all fillet tests pass
3. `ensure_cuttable_edge` uses `set_curve()` not `Edge::new()` -- grep confirms no `Edge::new` in the function
4. `convert_shell_in` uses `SNAP_TOLERANCE` not `near()` -- grep confirms `abs_diff_eq.*SNAP_TOLERANCE` in matching
5. New tests exist and pass for both identity preservation and tolerant matching
6. `cut_face_by_bezier` boundary replacement (lines 96-97) correctly finds edges after ensure_cuttable_edge conversion
</verification>

<success_criteria>
- ensure_cuttable_edge mutates the existing Edge's curve in-place via set_curve(), so edge.is_same() returns true before and after conversion (ETOPO-01)
- Edges originating from boolean operations pass endpoint matching in convert_shell_in with SNAP_TOLERANCE (1e-5) (ETOPO-02)
- cut_face_by_bezier boundary replacement correctly locates edges converted by ensure_cuttable_edge, with no stale edges
- All existing fillet tests pass without regression
- New targeted tests validate both fixes
</success_criteria>

<output>
After completion, create `.tendrion/phases/21-edge-identity-and-topology-repair/21-1-SUMMARY.md`
</output>
