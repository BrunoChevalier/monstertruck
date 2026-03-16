---
phase: 6-topology-surgery-hardening
plan: 2
type: tdd
wave: 2
depends_on: ["6-1"]
files_modified:
  - monstertruck-solid/src/fillet/topology.rs
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-solid/src/fillet/error.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "cut_face_by_bezier succeeds on faces bounded by IntersectionCurve edges by converting them to NURBS approximations before cutting"
    - "Fillet applied to a boolean-union result produces topologically valid shells with no non-manifold edges"
    - "A test case filleting a boolean-subtraction result with multi-wire boundary faces completes without panic"
    - "IntersectionCurve boundary edges are converted to NURBS approximations before cutting, enabling reliable parameter search and curve splitting"
  artifacts:
    - path: "monstertruck-solid/src/fillet/topology.rs"
      provides: "Hardened cut_face_by_bezier with IntersectionCurve edge handling via NURBS conversion and parameter-space projection"
      min_lines: 300
      contains: "to_nurbs_curve"
    - path: "monstertruck-solid/src/fillet/tests.rs"
      provides: "Tests for boolean-result fillet operations including union and subtraction cases"
      min_lines: 1850
      contains: "fillet_boolean_union"
    - path: "monstertruck-solid/src/fillet/error.rs"
      provides: "Error variants for IntersectionCurve handling failures"
      min_lines: 50
      contains: "FilletError"
  key_links:
    - from: "monstertruck-solid/src/fillet/topology.rs"
      to: "monstertruck-solid/src/fillet/convert.rs"
      via: "FilletableCurve::to_nurbs_curve() for IntersectionCurve edge conversion"
      pattern: "to_nurbs_curve"
    - from: "monstertruck-solid/src/fillet/topology.rs"
      to: "monstertruck-solid/src/transversal/divide_face/mod.rs"
      via: "Parameter-space projection pattern (search_parameter on face surface)"
      pattern: "search_parameter"
---

<objective>
Harden `cut_face_by_bezier` to handle faces bounded by IntersectionCurve edges, enabling fillet operations on boolean-result shells. The approach converts IntersectionCurve boundary edges to NURBS approximations before the cutting algorithm runs, with parameter-space projection for the splitting bezier. This addresses TOPO-01.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@monstertruck-solid/src/fillet/topology.rs
@monstertruck-solid/src/fillet/convert.rs
@monstertruck-solid/src/fillet/types.rs
@monstertruck-solid/src/transversal/divide_face/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write tests for boolean-result fillet operations</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
Add test cases that exercise fillet operations on boolean-result shells:

**Test 1: `fillet_boolean_union`**
1. Create two overlapping unit cubes using `monstertruck_modeling::builder` (one at origin, one offset by 0.5 in x).
2. Compute boolean OR (union) using `crate::or()`.
3. Select one or more edges from the union result shell (edges where the two cubes meet -- these will have IntersectionCurve geometry).
4. Call `fillet_edges_generic` with a small radius (e.g., 0.05).
5. Assert the operation completes without panic.
6. Assert the result shell has `ShellCondition::Closed` (topologically valid, no non-manifold edges).

**Test 2: `fillet_boolean_subtraction_multi_wire`**
1. Create a cube and a smaller cylinder using builder.
2. Compute boolean AND (subtraction: `cube & !cylinder`) using `crate::and()`.
3. The result has faces with multi-wire boundaries (the cylinder hole creates inner boundary loops).
4. Select edges adjacent to the hole boundary.
5. Call `fillet_edges_generic` with a small radius.
6. Assert completion without panic.
7. Assert `ShellCondition::Closed`.

**Test 3: `cut_face_by_bezier_intersection_curve_edge`**
This is a lower-level unit test:
1. Construct a face whose boundary includes an IntersectionCurve edge (can reuse the boolean AND result from `boolean_shell_converts_for_fillet` and extract a face).
2. Create a bezier curve that crosses this face.
3. Call `cut_face_by_bezier` on the face with the bezier and the IntersectionCurve edge's ID.
4. Assert it returns `Some(...)` (currently would fail/panic).

All three tests should FAIL before the fix (either panic or return None/Err).
  </action>
  <verify>Run `cargo test -p monstertruck-solid fillet_boolean_union` -- expect failure. Run `cargo test -p monstertruck-solid cut_face_by_bezier_intersection_curve_edge` -- expect failure.</verify>
  <done>Tests written that expose cut_face_by_bezier failure on IntersectionCurve edges.</done>
</task>

<task type="auto">
  <name>Task 2: Harden cut_face_by_bezier for IntersectionCurve edges</name>
  <files>monstertruck-solid/src/fillet/topology.rs, monstertruck-solid/src/fillet/types.rs</files>
  <action>
Modify `cut_face_by_bezier` in `topology.rs` to handle IntersectionCurve boundary edges. The core problem is that `search_closest_parameter` and `not_strictly_cut_with_parameter` rely on the edge curve being a type that supports direct parameter search, but IntersectionCurve edges from boolean operations may fail these operations.

**Strategy: Convert IntersectionCurve edges to NURBS before cutting**

In `cut_face_by_bezier`, after retrieving `front_edge` and `back_edge` via `find_adjacent_edge`:

1. Check if the front_edge or back_edge curve is `Curve::IntersectionCurve`. If so, convert the edge to use a NURBS approximation:
   ```rust
   fn ensure_nurbs_edge(edge: &Edge) -> Edge {
       match edge.curve() {
           Curve::IntersectionCurve(_) => {
               let nurbs = sample_edge_to_nurbs(edge);
               Edge::new(edge.absolute_front(), edge.absolute_back(), Curve::NurbsCurve(nurbs))
           }
           _ => edge.clone(),
       }
   }
   ```

   The `sample_edge_to_nurbs` helper should sample the IntersectionCurve at ~24 points and fit a NURBS curve through them (similar pattern to `FilletableCurve::to_nurbs_curve()` in convert.rs which calls `sample_curve_to_nurbs`).

2. Use the NURBS-converted edges for the `search_closest_parameter` and `not_strictly_cut_with_parameter` calls.

3. When constructing the new boundary in the map closure at the bottom of `cut_face_by_bezier`, if the original boundary edge was an IntersectionCurve, use the NURBS-approximated edge in the replacement.

**Add helper function** `ensure_cuttable_edge` to topology.rs:
```rust
fn ensure_cuttable_edge(edge: &Edge) -> Edge {
    if let Curve::IntersectionCurve(ic) = &edge.curve() {
        // Sample the IntersectionCurve to produce a NURBS approximation
        let range = ic.range_tuple();
        let sample_count = 24;
        let (t0, t1) = range;
        let points: Vec<Point3> = (0..=sample_count)
            .map(|i| t0 + (t1 - t0) * (i as f64) / (sample_count as f64))
            .map(|t| ic.evaluate(t))
            .collect();
        let knot_vector = KnotVector::uniform_knot(1, sample_count);
        let nurbs = NurbsCurve::from(BsplineCurve::new(knot_vector, points));
        Edge::new(edge.absolute_front(), edge.absolute_back(), Curve::NurbsCurve(nurbs))
    } else {
        edge.clone()
    }
}
```

Apply this conversion to `front_edge` and `back_edge` before the parameter search operations. The `fillet_edge` created from the bezier remains unchanged (it's already a NurbsCurve).

Also apply the same conversion in `cut_face_by_last_bezier` which calls `cut_face_by_bezier` -- the face passed to it may have IntersectionCurve edges after boolean operations.

**Important**: The `Curve` enum in types.rs already has `IntersectionCurve` variant and derives `SearchNearestParameterD1`, but the actual search may fail on complex intersection curves. The NURBS approximation sidesteps this.
  </action>
  <verify>Run `cargo test -p monstertruck-solid cut_face_by_bezier_intersection_curve_edge` -- should pass. Run `cargo test -p monstertruck-solid --lib fillet` -- no regressions.</verify>
  <done>cut_face_by_bezier handles IntersectionCurve edges by converting them to NURBS approximations before cutting.</done>
</task>

<task type="auto">
  <name>Task 3: End-to-end boolean fillet validation</name>
  <files>monstertruck-solid/src/fillet/tests.rs, monstertruck-solid/src/fillet/edge_select.rs</files>
  <action>
Verify the end-to-end boolean fillet tests pass and add robustness:

1. Run `fillet_boolean_union` and `fillet_boolean_subtraction_multi_wire` tests.
2. If tests fail due to edge selection issues (IntersectionCurve edges not being found after conversion), check and fix `convert_shell_in` edge matching -- it currently matches by endpoint positions which should be sufficient.
3. If tests fail during `fillet_along_wire` (rather than `cut_face_by_bezier`), the issue is likely in `fillet_surfaces_along_wire` where the `oriented_curve()` of an IntersectionCurve edge is passed to `relay_spheres`. In this case, the same NURBS conversion strategy should be applied in `fillet_surfaces_along_wire` when the edge curve is an IntersectionCurve. However, since `convert_shell_in` already converts all curves to NURBS during shell conversion (line 141 of convert.rs: `Curve::NurbsCurve(c.to_nurbs_curve())`), internal edges should already be NURBS by the time we reach `fillet_edges`. Verify this is the case.
4. If any test hits a panic rather than returning an error, add appropriate error handling (return `None` or `Err` instead of unwrapping).
5. Ensure `ShellCondition::Closed` assertion holds for the boolean-union fillet result. If the shell is not closed, investigate whether the issue is in edge wiring (vertex matching) or face construction.
6. If the subtraction test is too complex to get passing reliably in one iteration (multi-wire boundaries add significant complexity), mark it with `#[ignore]` with a comment explaining the remaining work needed, but ensure it at least does not panic.

Final verification: run the full test suite.
  </action>
  <verify>Run `cargo test -p monstertruck-solid fillet_boolean_union` -- passes. Run `cargo test -p monstertruck-solid --lib fillet` -- all tests pass. Run `cargo test -p monstertruck-solid fillet_boolean_subtraction` -- passes or is `#[ignore]`d without panic.</verify>
  <done>Boolean fillet operations complete without panics, producing topologically valid shells.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-solid cut_face_by_bezier_intersection_curve_edge` passes
2. `cargo test -p monstertruck-solid fillet_boolean_union` passes
3. `cargo test -p monstertruck-solid fillet_boolean_subtraction_multi_wire` passes or is cleanly ignored
4. `cargo test -p monstertruck-solid --lib fillet` passes with no regressions
5. `cut_face_by_bezier` in topology.rs contains NURBS conversion logic for IntersectionCurve edges
6. Boolean-union fillet produces a closed shell with no non-manifold edges
</verification>

<success_criteria>
- cut_face_by_bezier succeeds on faces bounded by IntersectionCurve edges (TOPO-01)
- IntersectionCurve edges are converted to NURBS approximations before cutting operations
- Fillet applied to a boolean-union result produces topologically valid shells
- Boolean-subtraction fillet with multi-wire boundary faces completes without panic
- All existing fillet tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/6-topology-surgery-hardening/6-2-SUMMARY.md`
</output>
