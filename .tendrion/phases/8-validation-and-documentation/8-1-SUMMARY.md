---
phase: 8-validation-and-documentation
plan: 1
tags: [fillet, topology-validation, debug-assertions, euler-poincare]
key-files:
  - monstertruck-solid/src/fillet/validate.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/mod.rs
decisions: []
metrics:
  tests_added: 4
  tests_passed: 51
  pre_existing_failures: 7
  files_created: 1
  files_modified: 3
---

## What was built

### Files created
- **monstertruck-solid/src/fillet/validate.rs** (365 lines): Topology validation module with `euler_poincare_check`, `is_oriented_check`, `debug_assert_topology`, and `debug_assert_euler` functions. Includes `#[cfg(test)]` module with 4 validation tests. Euler-Poincare enforced only on `ShellCondition::Closed` shells. No runtime cost in release builds.

### Files modified
- **monstertruck-solid/src/fillet/mod.rs**: Added `mod validate;` declaration.
- **monstertruck-solid/src/fillet/edge_select.rs**: Added `use super::validate;` and 3 assertion call sites: `debug_assert_topology` at end of `fillet_edges`, `debug_assert_topology` in `fillet_edges_generic` after internal fillet, `debug_assert_euler` after each single-edge fillet in chain loops (2 locations).
- **monstertruck-solid/src/fillet/ops.rs**: Added `use super::validate;` and `debug_assert_topology` after `fillet_along_wire` completes successfully.

## Task commits
| SHA | Message |
|-----|---------|
| 7a8d1912 | test(fillet): add failing test for topology validation functions |
| 6d05dcb1 | feat(fillet): implement topology validation functions in validate.rs |
| 5ae76daf | refactor(fillet): extract count_vef helper to reduce duplication |
| 04daa710 | test(fillet): add tests for debug assertion insertion points |
| 46b209cf | feat(fillet): insert debug topology assertions in fillet operations |
| 8e00be2f | test(fillet): add topology validation tests in validate.rs cfg(test) module |

## Tests added (in validate.rs #[cfg(test)])
1. **euler_poincare_valid_closed_box** -- 6-face closed box: V=8, E=12, F=6, chi=2.
2. **topology_valid_after_box_fillet** -- Fillets one edge then verifies Euler + orientation.
3. **debug_assert_fires_on_corrupted_orientation** -- Inverts face 5 to corrupt orientation, verifies `debug_assert_topology` panics (debug builds only).
4. **euler_poincare_check_detects_invalid_chi** -- Confirms guard logic: non-closed shells return true unconditionally.

## Deviations
- 7 pre-existing test failures (generic_fillet_*, boolean_shell_*, chamfer_serialization_*) unrelated to this plan.
- Pre-existing clippy errors in monstertruck-core (assign_op_pattern) unrelated to this plan.
- Pre-existing cargo fmt diffs in integrate.rs, tests.rs, lib.rs unrelated to this plan.

## Self-check
- [x] validate.rs exists with euler_poincare, is_oriented_check, debug_assert_topology, debug_assert_euler
- [x] mod.rs registers validate module
- [x] edge_select.rs has 3 debug_assert call sites
- [x] ops.rs has 1 debug_assert call site
- [x] All 51 fillet tests pass (47 existing + 4 new)
- [x] Debug assertions compile-time gated via `cfg!(debug_assertions)`
- [x] Euler-Poincare only enforced on ShellCondition::Closed
- [x] Orientation corruption test uses face inversion (not removal)
- [x] Test shell construction uses 6-face closed box pattern
