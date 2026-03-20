---
phase: 18-gordon-surface-variants
plan: 1
tags: [gordon, surface, curve-intersection, validation]
key-files:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/surface_diagnostics.rs
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/tests/gordon_variants_test.rs
decisions: []
metrics:
  tests_added: 13
  tests_passed: 262
  deviations: 0
---

## What was built

### GordonOptions.grid_tolerance (surface_options.rs)
- Added `grid_tolerance: f64` field to `GordonOptions` with default value of `SNAP_TOLERANCE` from monstertruck-core.
- Replaced `#[derive(Default)]` with manual `Default` impl to set the tolerance value.

### New diagnostic variants (surface_diagnostics.rs)
- `IntersectionCountMismatch { u_curve_index, v_curve_index, found, expected }` -- reports when curve intersection finds wrong number of hits.
- `GridPointNotOnCurve { row, col, u_distance, v_distance, tolerance }` -- reports when a caller-supplied grid point is too far from its curves.
- Both have `Display` implementations with descriptive messages.

### try_gordon_from_network (bspline_surface.rs)
- Auto-computes intersection grid points by intersecting each u-curve with each v-curve using `curve_intersect::find_intersections`.
- Intersections happen **before** compatibility normalization for numerical accuracy.
- Expects exactly 1 intersection per (u, v) pair; returns `IntersectionCountMismatch` otherwise.
- Delegates to `try_gordon` with the computed grid.

### try_gordon_verified (bspline_surface.rs)
- Validates caller-supplied grid points against both curve families.
- Uses `SearchNearestParameter` to find closest point on each curve.
- Points within `grid_tolerance` are snapped to the midpoint of the two nearest curve positions.
- Points exceeding tolerance return `GridPointNotOnCurve` with distances and tolerance.
- Delegates to `try_gordon` with snapped grid.

## Task commits

| SHA | Message |
|-----|---------|
| 83554f3e | test(gordon): add failing tests for gordon surface variant APIs |
| 6c9f65fc | feat(gordon): implement try_gordon_from_network and try_gordon_verified with diagnostics |

## Self-check

- [x] `cargo nextest run -p monstertruck-geometry` -- 262 passed, 1 skipped
- [x] `cargo clippy -p monstertruck-geometry --lib` -- clean
- [x] All 13 new tests pass
- [x] No regressions in existing tests
- [x] All artifact min_lines met (3352/2600, 160/120, 121/95)
- [x] Key links verified: bspline_surface -> curve_intersect, surface_diagnostics, SNAP_TOLERANCE
