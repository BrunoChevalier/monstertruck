---
phase: 31-deferred-ayam-port-completion
plan: 1
tags: [gordon-surface, intersection-grid, brep-validation, tdd]
key-files:
  - monstertruck-geometry/tests/gordon_intersection_grid_test.rs
  - monstertruck-modeling/tests/gordon_brep_validation_test.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
decisions:
  - Replaced tessellation test with surface evaluation test (monstertruck-meshing not a dependency of monstertruck-modeling)
  - Fixed tensor product control point knot assignment bug for asymmetric grids
metrics:
  tests_added: 7
  tests_passed: 462
  tests_skipped: 1
  regressions: 0
  deviations: 2
---

## What was built

### New test files

- **monstertruck-geometry/tests/gordon_intersection_grid_test.rs** (4 tests):
  - `curved_3x3_network_interpolates_intersections` -- verifies Gordon surface interpolates all curve intersection points within SNAP_TOLERANCE.
  - `curved_network_with_near_tangent_curves_error` -- verifies near-tangent curves produce `IntersectionCountMismatch` error, not a panic.
  - `large_5x4_curved_grid_success` -- verifies asymmetric 5x4 curved grid succeeds and has expected control point dimensions.
  - `gordon_from_network_surface_corners_match_curve_endpoints` -- verifies 2x2 Gordon surface corner evaluations match curve intersection points.

- **monstertruck-modeling/tests/gordon_brep_validation_test.rs** (3 tests):
  - `gordon_face_has_valid_boundary_topology` -- verifies Gordon face has 1 boundary wire with 4 edges.
  - `gordon_shell_passes_shell_condition` -- verifies shell of two Gordon faces has condition >= Regular (Oriented).
  - `gordon_face_surface_evaluates_correctly` -- verifies surface evaluates to finite coordinates and correct corner points.

### Bug fix

- **monstertruck-geometry/src/nurbs/bspline_surface.rs**: Fixed tensor product surface knot assignment in `try_gordon`. The control point grid has `n_v` rows and `n_u` columns, so the knots must be `(knot_v, knot_u)` not `(knot_u, knot_v)`. The previous assignment worked only for symmetric grids where `n_u == n_v` and caused a panic during degree elevation on asymmetric grids.

## Deviations

1. **Design**: Replaced `gordon_face_tessellation_produces_valid_mesh` with `gordon_face_surface_evaluates_correctly` because `monstertruck-meshing` is not a dependency of `monstertruck-modeling`.

2. **Bug fix**: Fixed tensor product knot assignment for asymmetric Gordon grids. The bug caused panics in `elevate_vdegree` when `n_u != n_v`.

## Verification

- `cargo nextest run -p monstertruck-geometry --test gordon_intersection_grid_test` -- 4/4 passed.
- `cargo nextest run -p monstertruck-modeling --test gordon_brep_validation_test` -- 3/3 passed.
- `cargo nextest run -p monstertruck-geometry -p monstertruck-modeling` -- 462 passed, 1 skipped, 0 failed.
- `cargo clippy -p monstertruck-geometry -p monstertruck-modeling --all-targets -- -W warnings` -- no new warnings (all pre-existing).
