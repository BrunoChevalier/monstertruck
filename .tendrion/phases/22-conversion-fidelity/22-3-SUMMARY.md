---
phase: 22-conversion-fidelity
plan: 3
tags: [fillet, endpoint-snapping, conversion, FCONV-02]
key-files:
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-solid/src/fillet/tests.rs
decisions:
  - Made sample_curve_to_nurbs pub(super) to enable direct testing from tests.rs
  - Extracted snap_shell_endpoints helper to deduplicate snapping loops in convert_shell_in/convert_shell_out
  - Reused param_points boundary values instead of redundant evaluate(t0)/evaluate(t1) calls in sample_curve_to_nurbs
metrics:
  tdd: strict
  red_commit: 9d47ccfe
  green_commit: d8562901
  refactor_commit: 1fd2632a
  tests_added: 3
  tests_passed: 3
  pre_existing_failures: 6
---

## What was built

### Files modified

- **monstertruck-solid/src/fillet/convert.rs**: Added `snap_curve_endpoints` helper that overwrites first/last NURBS control points with exact vertex positions (weight-preserving). Added `snap_shell_endpoints` helper that applies snapping to all NURBS edges in a shell. Integrated into `convert_shell_in` (post-conversion snap), `convert_shell_out` (pre-conversion snap), and `sample_curve_to_nurbs` (post-interpolation snap). Changed `sample_curve_to_nurbs` visibility from private to `pub(super)`.

- **monstertruck-solid/src/fillet/tests.rs**: Added three endpoint snapping tests:
  - `endpoint_snap_preserves_closure`: Cube round-trip through convert_shell_in/out preserves ShellCondition::Closed.
  - `endpoint_snap_after_interpolation`: Verifies sample_curve_to_nurbs produces control points exactly encoding boundary positions within 1e-14.
  - `endpoint_snap_intersection_curve_edge_roundtrip`: IntersectionCurve edges snap to vertex positions within 1e-14 after convert_shell_in.

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | 9d47ccfe | test(fillet): add failing endpoint snapping tests for conversion round-trip |
| GREEN | d8562901 | feat(fillet): implement endpoint snapping in convert_shell_in, convert_shell_out, and sample_curve_to_nurbs |
| REFACTOR | 1fd2632a | refactor(fillet): extract snap_shell_endpoints helper to deduplicate snapping loops |

## Deviations

- 6 pre-existing test failures (generic_fillet_identity, generic_fillet_modeling_types, generic_fillet_mixed_surfaces, generic_fillet_multi_chain, generic_fillet_unsupported, test_unit_circle) confirmed not caused by this plan's changes.

## Self-check

- `snap_curve_endpoints` called in convert_shell_in, convert_shell_out, and sample_curve_to_nurbs: PASS
- No `curve_mut()` usage in convert.rs: PASS
- No `edge_iter_mut()` usage in convert.rs: PASS
- All 3 endpoint_snap tests pass: PASS
- ShellCondition::Closed preserved in round-trip test: PASS
