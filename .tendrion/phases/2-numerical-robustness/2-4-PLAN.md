---
phase: 2-numerical-robustness
plan: 4
type: tdd
wave: 2
depends_on: ["2-1"]
files_modified:
  - monstertruck-solid/src/transversal/intersection_curve/mod.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-solid/src/transversal/edge_cases.rs
  - monstertruck-solid/src/transversal/mod.rs
  - monstertruck-solid/tests/boolean_edge_cases.rs
autonomous: true
must_haves:
  truths:
    - "User performs boolean AND on two solids sharing a tangent face and the operation produces valid topology without panics"
    - "User performs boolean OR on two solids sharing a coincident face and the operation produces valid topology without panics"
    - "User performs boolean operations on solids with pole-degenerate surfaces (e.g., sphere poles) and the operation succeeds"
    - "User performs standard boolean operations (non-degenerate) and results are identical to current behavior"
  artifacts:
    - path: "monstertruck-solid/tests/boolean_edge_cases.rs"
      provides: "Integration tests for tangent, coincident, and pole-degenerate boolean operations"
      min_lines: 120
      contains: "tangent_face"
    - path: "monstertruck-solid/src/transversal/edge_cases.rs"
      provides: "Detection and handling logic for degenerate boolean operation cases"
      min_lines: 80
      contains: "detect_tangent"
  key_links:
    - from: "monstertruck-solid/src/transversal/edge_cases.rs"
      to: "monstertruck-solid/src/transversal/integrate/mod.rs"
      via: "Edge case detection called before loop store creation in process_one_pair_of_shells"
      pattern: "detect_tangent\|detect_coincident\|handle_degenerate"
    - from: "monstertruck-solid/src/transversal/edge_cases.rs"
      to: "monstertruck-solid/src/transversal/intersection_curve/mod.rs"
      via: "Edge case handler may bypass or adjust intersection curve computation"
      pattern: "edge_cases"
    - from: "monstertruck-solid/tests/boolean_edge_cases.rs"
      to: "monstertruck-solid/src/transversal/integrate/mod.rs"
      via: "Tests call and/or/difference public API"
      pattern: "and\\(|or\\(|difference\\("
---

<objective>
Harden boolean operations in monstertruck-solid to handle tangent-face, coincident-face, and pole-degeneration inputs without panics, producing valid topology for all edge cases.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/transversal/mod.rs
@monstertruck-solid/src/transversal/integrate/mod.rs
@monstertruck-solid/src/transversal/intersection_curve/mod.rs
@monstertruck-solid/src/transversal/loops_store/mod.rs
@monstertruck-solid/src/transversal/faces_classification/mod.rs
@monstertruck-solid/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for boolean edge cases</name>
  <files>monstertruck-solid/tests/boolean_edge_cases.rs</files>
  <action>
Create `monstertruck-solid/tests/boolean_edge_cases.rs` with integration tests:

1. **Test `tangent_face_and`**: Create two unit cubes where one is translated so they share exactly one face in a tangent configuration (touching but not overlapping at a face). Call `and()` and verify it returns `Ok` (either empty or valid solid). It must not panic.

2. **Test `tangent_face_or`**: Same setup as above but call `or()`. The result should be a valid solid encompassing both cubes. Must not panic.

3. **Test `coincident_face_and`**: Create two cubes that share an entire coincident face (one cube is translated by exactly 1 unit along X so the right face of cube 0 and the left face of cube 1 overlap). Call `and()`. This should return `Ok` -- either empty (no interior overlap) or a degenerate thin solid. Must not panic.

4. **Test `coincident_face_or`**: Same coincident setup. Call `or()`. Result should be a valid merged solid (a 2x1x1 box). Must not panic.

5. **Test `pole_degeneration_sphere_and`**: Create a sphere solid (using `RevolutedCurve` of a semicircle -- the sphere has pole degeneration at the poles). Create a cube that intersects the sphere. Call `and()` and verify it returns `Ok` with a valid solid. Must not panic.

6. **Test `pole_degeneration_sphere_difference`**: Same sphere setup. Call `difference()`. Must not panic.

7. **Test `regression_standard_boolean`**: Two overlapping cubes (translated by 0.5 in each axis). Call `and()`, `or()`, and `difference()`. Verify results match expected topology (non-empty, valid solids). This guards against regressions in standard behavior.

Use `monstertruck_modeling::builder` to construct test solids. Use the `Solid` type from `monstertruck_topology`.
Use `monstertruck_solid::{and, or, difference}` for boolean operations.
All tests should catch panics using `std::panic::catch_unwind` for the edge case tests, asserting no panic occurs.
Standard tolerance: `0.05` (matching existing test patterns in the codebase).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --test boolean_edge_cases` and confirm edge case tests fail (panic or error) while regression test passes.</verify>
  <done>Failing tests written for tangent, coincident, and pole-degenerate boolean operations.</done>
</task>

<task type="auto">
  <name>Task 2: Implement edge case detection and handling</name>
  <files>monstertruck-solid/src/transversal/edge_cases.rs, monstertruck-solid/src/transversal/mod.rs</files>
  <action>
Create `monstertruck-solid/src/transversal/edge_cases.rs` with detection and handling logic:

1. **`detect_tangent_faces`**: Given two shells, detect pairs of faces that are tangent (surfaces touch but intersection curve degenerates to a point or has zero-length). Algorithm:
   - For each face pair across shells, sample surface normals at several points.
   - If normals are parallel (or anti-parallel) at all sample points and the surfaces are within tolerance at those points, the faces are tangent.
   - Return a list of tangent face pair indices.

2. **`detect_coincident_faces`**: Given two shells, detect faces with coincident surfaces. Algorithm:
   - For each face pair, if the surfaces are within tolerance at a grid of sample points AND the normals are parallel, the faces are coincident.
   - Coincident faces should be classified directly (skipping intersection curve computation) as either `And` or `Or` based on normal orientation agreement.
   - Return classified coincident face pairs.

3. **`handle_degenerate_intersection`**: When `create_loops_stores_with_tolerance` fails or produces an empty loops store:
   - Check if the failure is due to tangent or coincident faces.
   - If tangent: classify the tangent faces as `Or` (they form the boundary) and continue processing remaining faces normally.
   - If coincident: merge the coincident faces into the classification directly.

4. **`is_pole_degenerate`**: Check if a surface has pole degeneration (e.g., a surface where an entire edge of the parameter domain maps to a single point). Algorithm:
   - Evaluate the surface at the corners of the parameter domain.
   - If two adjacent corners map to the same 3D point (within tolerance), the surface has a degenerate edge.
   - When detected, adjust the intersection curve search to avoid the degenerate parameter region.

Register the module in `mod.rs` with `mod edge_cases;`.
All code comments must end with a period.
Use functional style and avoid unnecessary allocations.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --test boolean_edge_cases` and confirm edge case tests pass.</verify>
  <done>Edge case detection and handling implemented for tangent, coincident, and pole-degenerate boolean operations.</done>
</task>

<task type="auto">
  <name>Task 3: Integration and regression testing</name>
  <files>monstertruck-solid/src/transversal/integrate/mod.rs, monstertruck-solid/src/transversal/intersection_curve/mod.rs</files>
  <action>
1. **Update `process_one_pair_of_shells`** in `integrate/mod.rs`:
   - Add edge case detection calls at the beginning of the function, before loop store creation.
   - Handle the case where all faces are tangent/coincident (skip loop store entirely).
   - Add error recovery: if `create_loops_stores_with_tolerance` returns an error AND tangent/coincident faces were detected, retry without those faces.
   - Add pole degeneration handling in the intersection curve search (pass `is_pole_degenerate` info to the polyline construction).

2. **Update `IntersectionCurveWithParameters::try_new`** in `intersection_curve/mod.rs`:
   - When `search_nearest_point` fails for a point near a pole, use a fallback that clamps the parameter to avoid the degenerate region.
   - Add a tolerance parameter that comes from the `OperationTolerance` framework (plan 2-1) to make intersection curve search more robust near degenerate regions.

3. **Run full regression suite**:
   - `cargo nextest run -p monstertruck-solid` -- all existing tests must pass.
   - `cargo nextest run -p monstertruck-meshing` -- tessellation tests still pass.
   - Run the existing boolean operation tests in `monstertruck-solid/src/transversal/integrate/tests.rs` specifically.

4. **Handle edge cases in error types**: If needed, add new error variants to `ShapeOpsError` for degenerate cases (e.g., `TangentFacesDetected`, `CoincidentFacesDetected`). These should be informational, not blocking -- the operation should still succeed.

All code comments must end with a period.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid` and confirm all tests pass, including both new edge case tests and existing boolean operation tests.</verify>
  <done>Boolean operation hardening integrated with full regression verification.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid` -- all tests pass including edge case tests.
2. `cargo nextest run -p monstertruck-solid --test boolean_edge_cases` -- all 7 tests pass.
3. Boolean AND/OR/difference on tangent-face solids returns Ok without panics.
4. Boolean operations on coincident-face solids returns Ok without panics.
5. Boolean operations on sphere (pole-degenerate) solids returns Ok without panics.
6. Standard boolean operations (non-degenerate overlapping cubes) produce identical results to before.
7. No regressions in existing tests across the workspace.
</verification>

<success_criteria>
- ROBUST-04 complete: Boolean operations handle tangent-face, coincident-face, and pole-degeneration inputs without panics
- Valid topology produced for all edge cases
- No regressions in standard boolean operation behavior
- Edge case detection is efficient (sampling-based, not exhaustive)
</success_criteria>

<output>
After completion, create `.tendrion/phases/2-numerical-robustness/2-4-SUMMARY.md`
</output>
