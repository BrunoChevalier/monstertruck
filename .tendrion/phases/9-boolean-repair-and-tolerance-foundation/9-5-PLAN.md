---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 5
type: execute
wave: 3
depends_on: ["9-1", "9-2", "9-3"]
files_modified: []
autonomous: true
must_haves:
  truths:
    - "cargo nextest run -p monstertruck-solid --lib --no-fail-fast passes all boolean unit tests"
    - "cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast passes all edge case tests"
    - "cargo clippy --all-targets -- -W warnings produces zero warnings across the entire workspace"
    - "cargo fmt --all -- --check passes with no formatting issues"
    - "The MissingPolygon fix in Matrix4::from_translation did not introduce regressions"
  artifacts: []
  key_links: []
---

<objective>
Run a full verification sweep confirming that the MissingPolygon bug fix (Matrix4::from_translation column placement) resolved boolean test failures, and that no regressions exist across the workspace. This is a verification-only plan with no file modifications.
</objective>

<execution_context>
@AGENTS.md
</execution_context>

<context>
@monstertruck-math/src/lib.rs
@monstertruck-solid/tests/boolean_edge_cases.rs
@monstertruck-solid/src/transversal/integrate/tests.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Run boolean unit tests and edge case integration tests</name>
  <files>monstertruck-solid/src/transversal/integrate/tests.rs, monstertruck-solid/tests/boolean_edge_cases.rs</files>
  <action>
Run the following test suites in sequence and record the results:

1. `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` — Run all unit tests in monstertruck-solid including the boolean topology tests added in plan 9-3 (overlapping_cubes_and_topology, overlapping_cubes_or_topology, overlapping_cubes_difference_topology, chained_boolean_and_then_or, adjacent_cubes_or, punched_cube).

2. `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast` — Run the boolean edge case integration tests (tangent_face_and, tangent_face_or, coincident_face_and, coincident_face_or, pole_degeneration_sphere_and, pole_degeneration_sphere_difference, regression_standard_boolean).

3. `cargo nextest run -p monstertruck-math --lib --no-fail-fast` — Verify the Matrix4::from_translation fix passes math crate tests.

If any test fails, document the failure including the test name, error message, and backtrace. Do NOT modify any source files — this is a verification-only plan. If failures are found, record them in the summary as known issues requiring follow-up.
  </action>
  <verify>All three test commands exit with status 0 and report zero failures.</verify>
  <done>Boolean unit tests, edge case integration tests, and math crate tests all verified passing.</done>
</task>

<task type="auto">
  <name>Task 2: Run workspace-wide clippy and fmt checks</name>
  <files></files>
  <action>
Run workspace-wide quality checks:

1. `cargo clippy --all-targets -- -W warnings` — Full workspace lint with warnings promoted. Every crate must pass with zero warnings.

2. `cargo fmt --all -- --check` — Formatting check across all crates.

3. `cargo nextest run --workspace --no-fail-fast` — Full workspace test run to confirm no cross-crate regressions from the MissingPolygon fix or any phase 9 changes.

Record results. If clippy warnings are found, document them but do NOT fix them (this is a verification-only plan with no files_modified). If workspace tests reveal failures outside monstertruck-solid, document them as cross-crate regression findings.
  </action>
  <verify>All three commands exit with status 0. Zero clippy warnings, zero fmt issues, zero test failures.</verify>
  <done>Workspace-wide verification complete: clippy clean, fmt clean, all tests passing.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` — zero failures
2. `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast` — zero failures
3. `cargo nextest run -p monstertruck-math --lib --no-fail-fast` — zero failures
4. `cargo clippy --all-targets -- -W warnings` — zero warnings
5. `cargo fmt --all -- --check` — clean
6. `cargo nextest run --workspace --no-fail-fast` — zero failures
7. No source files modified (verification-only plan)
</verification>

<success_criteria>
- All boolean tests pass after MissingPolygon fix, confirming BOOL-01 is resolved
- Full workspace passes clippy and fmt
- No regressions introduced by phase 9 changes
- Results documented in summary for phase verification closure
</success_criteria>

<output>
After completion, create `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-5-SUMMARY.md`
</output>
