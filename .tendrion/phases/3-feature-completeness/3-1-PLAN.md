---
phase: 3-feature-completeness
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-step/src/save/geometry.rs
  - monstertruck-step/src/lib.rs
  - monstertruck-step/examples/shape-to-step.rs
  - monstertruck-step/tests/output/topology.rs
  - monstertruck-step/tests/output/templates.rs
autonomous: true
must_haves:
  truths:
    - "User creates a solid via boolean union/difference/intersection and exports it to STEP; the file is valid and re-parsable"
    - "User exports a punched-cube-shapeops.json to STEP without errors"
    - "IntersectionCurve geometry references are correct in STEP output (surface indices match)"
    - "Round-trip: boolean-result STEP can be parsed back by ruststep"
    - "The shape-to-step example compiles and runs successfully"
  artifacts:
    - path: "monstertruck-step/src/save/geometry.rs"
      provides: "Fixed IntersectionCurve STEP output with correct surface1 index"
      min_lines: 800
      contains: "surface1_idx"
    - path: "monstertruck-step/tests/output/topology.rs"
      provides: "Integration test for boolean-result shapes exported to STEP"
      min_lines: 40
      contains: "punched-cube-shapeops"
  key_links:
    - from: "monstertruck-step/tests/output/topology.rs"
      to: "resources/shape/punched-cube-shapeops.json"
      via: "Test loads boolean-result JSON and exports to STEP"
      pattern: "punched-cube-shapeops.json"
    - from: "monstertruck-step/src/save/geometry.rs"
      to: "monstertruck-step/src/save/topology.rs"
      via: "IntersectionCurve DisplayByStep used during shell/solid export"
      pattern: "surface1_idx"
---

<objective>
Enable STEP export of shapes created by boolean set operations (union, difference, intersection). Fix the IntersectionCurve STEP index bug, add boolean-result shapes to STEP output tests, fix broken example, and update crate documentation.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-step/src/save/geometry.rs
@monstertruck-step/src/save/topology.rs
@monstertruck-step/src/lib.rs
@monstertruck-step/tests/output/topology.rs
@monstertruck-step/examples/shape-to-step.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for boolean STEP export and fix IntersectionCurve bug</name>
  <files>monstertruck-step/tests/output/topology.rs, monstertruck-step/src/save/geometry.rs</files>
  <action>
**TDD Red phase**: Add a test that loads `punched-cube-shapeops.json` (a boolean AND result containing IntersectionCurve edges) and attempts STEP export + re-parse.

In `monstertruck-step/tests/output/topology.rs`:
1. Add `"punched-cube-shapeops.json"` to the `SOLID_JSONS` list so it is tested by `parse_solid`, `parse_shell`, and `parse_solids`.
2. Add a dedicated test `parse_boolean_result_solid` that:
   - Loads `resources/shape/punched-cube-shapeops.json`
   - Exports to STEP string via `CompleteStepDisplay::new(StepModel::from(&solid), Default::default())`
   - Parses the STEP string with `ruststep::parser::parse`
   - Asserts success

Run the test -- it should fail due to the bug in `IntersectionCurve::fmt` (line 380 of geometry.rs uses `surface0_idx` instead of `surface1_idx`).

**TDD Green phase**: Fix the bug in `monstertruck-step/src/save/geometry.rs`:
- Line 380: change `self.surface1().fmt(surface0_idx, f)` to `self.surface1().fmt(surface1_idx, f)`

Re-run the test to confirm it passes.
  </action>
  <verify>
Run `cargo test -p monstertruck-step --test output -- topology` and confirm all topology tests pass including the new boolean-result test.
  </verify>
  <done>Boolean-result shapes (containing IntersectionCurve edges) export to valid STEP files. The surface1 index bug is fixed.</done>
</task>

<task type="auto">
  <name>Task 2: Fix shape-to-step example and template tests, update documentation</name>
  <files>monstertruck-step/examples/shape-to-step.rs, monstertruck-step/src/lib.rs, monstertruck-step/tests/output/templates.rs</files>
  <action>
1. Fix `monstertruck-step/examples/shape-to-step.rs`:
   - Replace all `out::` references with `save::` (the `save` module is already imported)
   - Specifically: `out::CompleteStepDisplay` -> `save::CompleteStepDisplay`, `out::StepModel` -> `save::StepModel`, `out::StepHeaderDescriptor` -> `save::StepHeaderDescriptor`

2. Fix `monstertruck-step/tests/output/templates.rs`:
   - Replace `"truck"` with `"monstertruck"` in the expected strings (the crate was renamed from truck to monstertruck)
   - The `FILE_DESCRIPTION` and `FILE_NAME` reference strings contain `'Shape Data from truck'` and `'truck'` -- update both to `'Shape Data from monstertruck'` and `'monstertruck'`

3. Update `monstertruck-step/src/lib.rs` documentation:
   - Remove or update the line "Shapes created by set operations cannot be output yet." since this is now supported
   - Replace with: "Both shapes modeled by monstertruck-modeling and shapes created by set operations (boolean union, difference, intersection) can be exported to STEP."

4. Verify the example compiles: `cargo build --example shape-to-step -p monstertruck-step`
  </action>
  <verify>
Run `cargo test -p monstertruck-step --test output` and confirm ALL tests pass (including template tests).
Run `cargo build --example shape-to-step -p monstertruck-step` and confirm compilation succeeds.
  </verify>
  <done>The shape-to-step example compiles, template tests pass with updated branding, and crate documentation reflects boolean export support.</done>
</task>

<task type="auto">
  <name>Task 3: Add STEP round-trip integration test for boolean operations</name>
  <files>monstertruck-step/tests/output/topology.rs</files>
  <action>
Add a more comprehensive integration test `boolean_step_round_trip` that:

1. Creates a cube and cylinder programmatically using `monstertruck_modeling::builder`
2. Performs a boolean AND operation using `monstertruck_solid::and`
3. Compresses the result to `CompressedSolid`
4. Exports to STEP string
5. Parses the STEP string with ruststep
6. Verifies the parse succeeds
7. Checks the STEP string contains expected STEP entities (INTERSECTION_CURVE, B_SPLINE_CURVE_WITH_KNOTS, PLANE, etc.)

This tests the full pipeline from modeling -> boolean -> compress -> STEP export -> parse.

Also add a similar test for boolean `or` (union) and `difference` operations if time permits, or at minimum test one more boolean operation type.

Note: The test file uses `monstertruck_modeling::*` which brings in `builder`, `Point3`, `Vector3`, `Rad`, `Solid` etc. The `monstertruck-step` crate has `monstertruck-solid` as a dev-dependency (check Cargo.toml). If not, the test can use the pre-built JSON files.
  </action>
  <verify>
Run `cargo test -p monstertruck-step --test output -- topology` and confirm all tests pass.
  </verify>
  <done>End-to-end STEP export round-trip test exists for boolean operation results, covering the full pipeline from shape creation through boolean ops to STEP output and parse.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-step --test output` -- all tests pass including boolean-result export
2. `cargo build --example shape-to-step -p monstertruck-step` -- example compiles
3. The IntersectionCurve STEP output uses correct `surface1_idx` on the second surface fmt call
4. No regression in existing STEP output tests (parse_solid, parse_shell, parse_solids)
5. Template tests pass with updated branding strings
</verification>

<success_criteria>
- A shape created by boolean union/difference/intersection can be written to a STEP file and re-imported with matching topology
- The IntersectionCurve STEP index bug is fixed (surface1 uses surface1_idx)
- The shape-to-step example compiles and runs
- All STEP output tests pass including new boolean-result tests
</success_criteria>

<output>
After completion, create `.tendrion/phases/3-feature-completeness/3-1-SUMMARY.md`
</output>
