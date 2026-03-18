---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 1
tags: [tolerance, documentation, regression-tests]
key-files:
  - monstertruck-core/src/tolerance.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-core/tests/tolerance_policy.rs
decisions:
  - "Regression tests pass at RED: tolerance_policy tests pin existing behavior (TOLERANCE constant, OperationTolerance, Tolerance trait)"
metrics:
  tests_added: 4
  tests_passed: 4
  files_modified: 2
  files_created: 1
---

## What was built

### Files modified
- **monstertruck-core/src/tolerance.rs**: Added module-level `//! # Numeric Tolerance Policy` doc comment (32 lines) documenting TOLERANCE, TOLERANCE2, Tolerance trait, Origin trait, OperationTolerance, and when to use local constants. No existing code changed.
- **monstertruck-solid/src/fillet/edge_select.rs**: Added `use monstertruck_core::tolerance::TOLERANCE;` import and replaced hardcoded `1.0e-6` with `TOLERANCE` constant in `rematch_selected_edge_id`.

### Files created
- **monstertruck-core/tests/tolerance_policy.rs**: 4 regression tests pinning tolerance values:
  - `tolerance_value_is_1e_minus_6` -- pins TOLERANCE == 1.0e-6
  - `tolerance2_is_tolerance_squared` -- pins TOLERANCE2 == TOLERANCE * TOLERANCE
  - `operation_tolerance_from_global_uses_tolerance` -- verifies OperationTolerance::from_global() integration
  - `near_trait_uses_tolerance` -- verifies Tolerance::near() threshold aligns with TOLERANCE

## Test results
- monstertruck-core lib: 10/10 passed
- monstertruck-core tolerance_policy: 4/4 passed
- monstertruck-solid lib: 75/101 passed (26 failures are pre-existing in draft/transversal/chamfer tests, unrelated to tolerance changes)

## Deviations
1. **RED tests pass immediately**: Tolerance policy tests pin existing behavior. The constants, traits, and OperationTolerance already exist -- tests are regression guards, not new-feature tests.
2. **Pre-existing clippy errors**: `assign_op_pattern` in `cgmath_extend_traits.rs:451` and `derivatives.rs:760`. Not introduced by this plan.
3. **Pre-existing test failures**: 26 failures in monstertruck-solid (draft, transversal, chamfer modules). Not related to tolerance changes.

## Self-check
- [x] monstertruck-core/src/tolerance.rs starts with `//! # Numeric Tolerance Policy`
- [x] No hardcoded `1.0e-6` in monstertruck-solid/src/fillet/edge_select.rs
- [x] TOLERANCE import present in edge_select.rs
- [x] All 4 tolerance_policy tests pass
- [x] All existing monstertruck-core tests pass
- [x] No new clippy warnings introduced
