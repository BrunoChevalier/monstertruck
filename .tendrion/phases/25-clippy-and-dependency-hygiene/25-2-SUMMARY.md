---
phase: 25-clippy-and-dependency-hygiene
plan: 2
tags: [clippy, lint, hygiene]
key-files:
  - monstertruck-mesh/src/stl.rs
  - monstertruck-solid/src/fillet/validate.rs
decisions:
  - "Used #[cfg(test)] for dead-code functions (euler_characteristic, euler_poincare_check, is_oriented_check) since they are only called from the test module, not from production debug_assert_topology/debug_assert_euler which inline the same logic."
metrics:
  tests_passed: 834
  tests_skipped: 4
  clippy_warnings_fixed: 4
---

## What Was Built

Fixed all clippy warnings across the workspace so that `cargo clippy --workspace -- -D warnings` exits with code 0.

### Files Modified

- **monstertruck-mesh/src/stl.rs**: Replaced unnecessary fully-qualified type `monstertruck_core::cgmath64::Vector3` with short `Vector3` (already in scope via `use crate::*`).
- **monstertruck-solid/src/fillet/validate.rs**: Added `#[cfg(test)]` to three functions (`euler_characteristic`, `euler_poincare_check`, `is_oriented_check`) that are only used by the test module, eliminating dead_code warnings.

## Task Commits

| Commit | Message |
|--------|---------|
| ae2f1ff1 | fix(clippy): remove unnecessary type qualification and dead-code warnings |

## Deviations

- TDD exemption logged: clippy lint fixes are pure refactoring with no behavioral change. Existing tests already cover the affected functions.

## Verification

- `cargo clippy -p monstertruck-mesh -- -D warnings` -- exit 0
- `cargo clippy -p monstertruck-solid -- -D warnings` -- exit 0
- `cargo clippy -p monstertruck-step -- -D warnings` -- exit 0
- `cargo clippy --workspace -- -D warnings` -- exit 0
- `cargo nextest run --workspace` -- 834 passed, 4 skipped, 0 failures
