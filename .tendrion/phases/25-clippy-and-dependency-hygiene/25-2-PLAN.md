---
phase: 25-clippy-and-dependency-hygiene
plan: 2
type: execute
wave: 2
depends_on: ["25-1"]
files_modified:
  - monstertruck-mesh/src/stl.rs
  - monstertruck-solid/src/fillet/validate.rs
autonomous: true
must_haves:
  truths:
    - "cargo clippy -p monstertruck-step -- -D warnings exits with code 0"
    - "cargo clippy -p monstertruck-mesh -- -D warnings exits with code 0"
    - "cargo clippy -p monstertruck-solid -- -D warnings exits with code 0"
    - "cargo clippy --workspace -- -D warnings exits with code 0"
    - "cargo nextest run --workspace passes after clippy fixes"
  artifacts:
    - path: "monstertruck-mesh/src/stl.rs"
      provides: "STL code with unnecessary qualification removed"
      min_lines: 200
      contains: "Vector3"
    - path: "monstertruck-solid/src/fillet/validate.rs"
      provides: "Validate module with dead code warnings resolved"
      min_lines: 50
      contains: "euler_poincare_check"
  key_links:
    - from: "monstertruck-solid/src/fillet/validate.rs"
      to: "monstertruck-solid/src/fillet/validate.rs"
      via: "functions used by debug_assert_topology or tests"
      pattern: "euler_poincare_check"
---

<objective>
Fix all clippy warnings across the workspace so that `cargo clippy --workspace -- -D warnings` exits with code 0. This must be done after the dependency update (Plan 25-1) to avoid fixing code that changes during the update.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-mesh/src/stl.rs
@monstertruck-solid/src/fillet/validate.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Fix clippy warnings in monstertruck-mesh and monstertruck-solid</name>
  <files>monstertruck-mesh/src/stl.rs, monstertruck-solid/src/fillet/validate.rs</files>
  <action>
Fix the known clippy warnings and any new ones that may have appeared after the vtkio update:

1. **monstertruck-mesh/src/stl.rs line 239** -- Unnecessary qualification warning:
   Change `let n: monstertruck_core::cgmath64::Vector3 = cross.normalize();` to `let n: Vector3 = cross.normalize();`
   - First verify that `Vector3` is already in scope (it should be via `use` imports in the file or prelude). If not, add the appropriate import.

2. **monstertruck-solid/src/fillet/validate.rs** -- Three dead_code warnings:
   - `euler_characteristic` (line 32) -- private function, only called by `euler_poincare_check` which is also unused outside tests
   - `euler_poincare_check` (line 43) -- pub(crate) function, used in tests but not in production code
   - `is_oriented_check` (line 54) -- pub(crate) function, used in tests but not in production code

   These functions ARE used by `debug_assert_topology` and `debug_assert_euler` (which call `count_vef` and inline the logic) and by the test module. The issue is that `euler_poincare_check` and `is_oriented_check` are only called from tests, not from the production `debug_assert_topology` function.

   Resolution: Add `#[cfg(test)]` to `euler_characteristic`, `euler_poincare_check`, and `is_oriented_check` since they are only used by the test module. This eliminates the dead_code warning without removing useful test utilities.

   Alternatively, if these functions should remain available for debug builds, add `#[allow(dead_code)]` with a comment explaining they are validation utilities used in tests and debug assertions.

   Preferred approach: Use `#[cfg(test)]` since the `debug_assert_topology` and `debug_assert_euler` functions already inline the same logic and don't call these functions directly.

3. Run `cargo clippy --workspace -- -D warnings 2>&1` to check for ANY additional warnings that may have appeared from the vtkio update in Plan 25-1. Fix all of them.

4. Also run `cargo clippy -p monstertruck-step -- -D warnings` specifically to verify RELY-03 (the phase's primary target crate).
  </action>
  <verify>
Run `cargo clippy -p monstertruck-mesh -- -D warnings` -- exits with code 0.
Run `cargo clippy -p monstertruck-solid -- -D warnings` -- exits with code 0.
Run `cargo clippy -p monstertruck-step -- -D warnings` -- exits with code 0.
  </verify>
  <done>All clippy warnings fixed in monstertruck-mesh and monstertruck-solid.</done>
</task>

<task type="auto">
  <name>Task 2: Full workspace clippy and test verification</name>
  <files>monstertruck-mesh/src/stl.rs, monstertruck-solid/src/fillet/validate.rs</files>
  <action>
1. Run `cargo clippy --workspace -- -D warnings` to verify zero warnings across the entire workspace. If any new warnings appear (from any crate), fix them.

2. Run `cargo nextest run --workspace` to ensure the clippy fixes did not break any tests. In particular, verify that the monstertruck-solid fillet validation tests still pass (since we may have moved functions under `#[cfg(test)]`).

3. Run `cargo clippy -p monstertruck-step -- -D warnings` one final time to confirm the phase's primary success criterion.
  </action>
  <verify>
Run `cargo clippy --workspace -- -D warnings` -- exits with code 0.
Run `cargo nextest run --workspace` -- all tests pass.
Run `cargo clippy -p monstertruck-step -- -D warnings` -- exits with code 0.
  </verify>
  <done>Workspace-wide clippy clean with -D warnings, all tests pass.</done>
</task>

</tasks>

<verification>
1. `cargo clippy -p monstertruck-step -- -D warnings` exits with code 0 (RELY-03 primary target)
2. `cargo clippy --workspace -- -D warnings` exits with code 0 (no new warnings from dependency updates)
3. `cargo nextest run --workspace` shows no test failures from clippy fixes
4. The monstertruck-solid fillet validation tests continue to pass
</verification>

<success_criteria>
- RELY-03 satisfied: zero clippy warnings in monstertruck-step with -D warnings
- All workspace clippy warnings eliminated including unnecessary qualifications and dead code
- No test regressions from the fixes
- Workspace-wide `cargo clippy -- -D warnings` passes clean
</success_criteria>

<output>
After completion, create `.tendrion/phases/25-clippy-and-dependency-hygiene/25-2-SUMMARY.md`
</output>
