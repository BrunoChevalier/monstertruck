---
phase: 22-conversion-fidelity
plan: 1
tags: [fillet, nurbs, interpolation, degree-3]
key-files:
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-modeling/src/fillet_impl.rs
decisions: []
metrics:
  tests_added: 3
  tests_passing: true
  pre_existing_failures: 6
---

## What was built

Upgraded all degree-1 piecewise-linear NURBS sampling to degree-3 cubic interpolation across the fillet conversion pipeline:

- **monstertruck-solid/src/fillet/convert.rs**: `sample_curve_to_nurbs` now uses `BsplineCurve::try_interpolate` with `KnotVector::uniform_knot(3, n_points - 3)` to produce degree-3 curves. `sample_surface_to_nurbs` uses two-pass row/column tensor product interpolation with Greville abscissae sampling (following the t_mesh.rs pattern). A `greville_abscissae` helper and `try_degree3_surface` helper were added. Both functions fall back to degree-1 if interpolation fails.

- **monstertruck-modeling/src/fillet_impl.rs**: `sample_to_nurbs` upgraded from degree-1 manual knot construction to degree-3 `try_interpolate` with fallback. Sample count increased from 16 to 24 in all three call sites (`From<ParameterCurveLinear>`, `From<FilletIntersectionCurve>`, `TryFrom<Curve>` for `IntersectionCurve`).

## Task commits

| SHA | Message |
|-----|---------|
| bef492c3 | test(fillet): add failing tests for degree-3 sample_curve_to_nurbs and sample_surface_to_nurbs |
| e26a79f3 | feat(fillet): upgrade sample_curve_to_nurbs and sample_surface_to_nurbs to degree-3 cubic interpolation |
| 29ab141a | refactor(fillet): extract try_degree3_surface helper and add degree-1 fallback for surface interpolation |
| 317b5f2b | test(fillet): add failing test for degree-3 sample_to_nurbs in fillet_impl |
| fba76beb | feat(fillet): upgrade sample_to_nurbs in fillet_impl.rs to degree-3 cubic interpolation with 24-point sampling |

## Deviations from plan

- 6 pre-existing test failures in monstertruck-solid (generic_fillet_identity, generic_fillet_mixed_surfaces, generic_fillet_modeling_types, generic_fillet_multi_chain, generic_fillet_unsupported, test_unit_circle) confirmed to fail on master without any changes. Not caused by this plan.

## Self-check

- `try_interpolate` used in convert.rs (3 call sites) and fillet_impl.rs (1 call site): CONFIRMED
- `n_points - 3` knot vector sizing in convert.rs (3 instances) and fillet_impl.rs (1 instance): CONFIRMED
- No `uniform_knot(1,` in primary paths (only in fallback code and test fixtures): CONFIRMED
- All 112/118 non-pre-existing tests pass in monstertruck-solid: CONFIRMED
- All 31/31 tests pass in monstertruck-modeling: CONFIRMED
