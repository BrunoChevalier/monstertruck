---
phase: 6-topology-surgery-hardening
plan: 2
tags: [fillet, intersection-curve, nurbs-conversion, boolean-result, topology]
key-files:
  - monstertruck-solid/src/fillet/topology.rs
  - monstertruck-solid/src/fillet/tests.rs
  - monstertruck-solid/src/fillet/convert.rs
decisions:
  - "Used FilletableCurve::to_nurbs_curve() for IC edge conversion instead of manual sampling"
  - "Boolean fillet tests (fillet_boolean_union, fillet_boolean_subtraction_multi_wire) marked #[ignore] due to pre-existing boolean operation failures"
metrics:
  tests_added: 3
  tests_passing: 1
  tests_ignored: 2
  regressions: 0
  pre_existing_failures: 7
---

## What was built

### Files modified

- **monstertruck-solid/src/fillet/topology.rs**: Added `ensure_cuttable_edge()` function that converts `IntersectionCurve` boundary edges to NURBS approximations via `FilletableCurve::to_nurbs_curve()` before `cut_face_by_bezier` performs parameter search and edge splitting. Applied to both front and back adjacent edges.

- **monstertruck-solid/src/fillet/tests.rs**: Added three tests:
  - `cut_face_by_bezier_intersection_curve_edge` -- unit test constructing a face with IntersectionCurve adjacent edges and verifying `cut_face_by_bezier` succeeds (PASSES)
  - `fillet_boolean_union` -- end-to-end boolean union + fillet (#[ignore], blocked by boolean op bug)
  - `fillet_boolean_subtraction_multi_wire` -- boolean subtraction + fillet (#[ignore], blocked by boolean op bug)
  - Helper: `build_face_with_intersection_curve_edge()` constructs faces with highly curved IntersectionCurve boundary edges

- **monstertruck-solid/src/fillet/convert.rs**: No functional changes (reverted temporary pub(super) on sample_curve_to_nurbs)

## Task commits

| SHA | Message |
|-----|---------|
| 615f83ce | test(fillet): add failing test for cut_face_by_bezier with IntersectionCurve edges |
| bdbbb9b0 | feat(fillet): convert IntersectionCurve edges to NURBS before cutting |
| 92100b62 | refactor(fillet): reuse sample_curve_to_nurbs in ensure_cuttable_edge |
| d216df38 | feat(fillet): use FilletableCurve::to_nurbs_curve in ensure_cuttable_edge |

## Decisions made

1. **NURBS conversion via FilletableCurve trait**: Instead of inline sampling, used the existing `to_nurbs_curve()` trait method which already handles the sampling pattern. This avoids code duplication and stays consistent with the convert module.

2. **Boolean fillet tests marked #[ignore]**: Both `crate::or()` and `crate::and()` have pre-existing failures (`CreateLoopsStoreFailed`, `WireNotInOnePlane`). The boolean fillet tests are structurally correct but cannot execute until boolean operations are fixed.

## Deviations from plan

1. **Boolean operations pre-existing failure** (auto-fix, dependency): `crate::or()` fails with `CreateLoopsStoreFailed` and `crate::and()` panics at `try_attach_plane` with `WireNotInOnePlane`. Both `fillet_boolean_union` and `fillet_boolean_subtraction_multi_wire` tests are marked `#[ignore]` pending boolean op fixes.

## Self-check

- `cut_face_by_bezier_intersection_curve_edge` PASSES (1.4s, was 11.6s with raw IC edges)
- `ensure_cuttable_edge` contains `to_nurbs_curve` call in topology.rs
- All 39 previously-passing fillet tests continue to pass (no regressions)
- 7 pre-existing failures unchanged
- topology.rs: 371 lines, contains "to_nurbs_curve"
- tests.rs: 3307 lines, contains "fillet_boolean_union"
- error.rs: 50 lines, contains "FilletError"
