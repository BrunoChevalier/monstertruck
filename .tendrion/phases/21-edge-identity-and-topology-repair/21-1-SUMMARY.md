---
phase: 21-edge-identity-and-topology-repair
plan: 1
tags: [fillet, edge-identity, tolerance, topology]
key-files:
  - monstertruck-solid/src/fillet/topology.rs
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-solid/src/fillet/tests.rs
decisions: []
metrics:
  tests_added: 2
  tests_passing: 110
  pre_existing_failures: 6
  tdd_violations: 0
---

## What Was Built

### Files Modified

- **monstertruck-solid/src/fillet/topology.rs**: Changed `ensure_cuttable_edge` to use `edge.set_curve()` for in-place curve mutation instead of `Edge::new()`. This preserves `EdgeId` so that `is_same()` returns true before and after conversion, fixing the boundary replacement in `cut_face_by_bezier`. Changed visibility to `pub(super)` for testability. Updated doc comment to reflect the in-place mutation behavior.

- **monstertruck-solid/src/fillet/convert.rs**: Replaced `f.near(ext_front)` (TOLERANCE=1e-6) with `f.abs_diff_eq(ext_front, SNAP_TOLERANCE)` (1e-5) in `convert_shell_in` endpoint matching. Removed unused `Tolerance` import. Added `SNAP_TOLERANCE` import.

- **monstertruck-solid/src/fillet/tests.rs**: Added two new tests:
  - `ensure_cuttable_edge_preserves_identity`: Verifies IntersectionCurve edge conversion preserves EdgeId via `is_same()`.
  - `convert_shell_in_tolerant_endpoint_matching`: Verifies endpoint matching works with 5e-6 offset (between TOLERANCE and SNAP_TOLERANCE).

## Task Commits

| Phase | SHA | Message |
|-------|-----|---------|
| RED | `aa8eb8f5` | test(fillet): add failing tests for edge identity preservation and tolerant endpoint matching |
| GREEN | `58b78600` | feat(fillet): preserve edge identity in ensure_cuttable_edge and widen convert_shell_in tolerance to SNAP_TOLERANCE |
| REFACTOR | `7faebbfa` | refactor(fillet): update ensure_cuttable_edge doc comment and remove unused Tolerance import |

## Deviations

- 6 pre-existing test failures (test_unit_circle, generic_fillet_unsupported, generic_fillet_identity, generic_fillet_mixed_surfaces, generic_fillet_modeling_types, generic_fillet_multi_chain) confirmed not caused by this plan's changes.

## Self-Check

- `ensure_cuttable_edge` uses `set_curve()`, no `Edge::new()` in function: PASS
- `convert_shell_in` uses `abs_diff_eq` with `SNAP_TOLERANCE`: PASS
- Both new tests pass: PASS
- No new test regressions: PASS (all 6 failures are pre-existing)
