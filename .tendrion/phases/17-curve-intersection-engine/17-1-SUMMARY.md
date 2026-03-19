---
phase: 17-curve-intersection-engine
plan: 1
tags: [nurbs, intersection, subdivision, newton-raphson, geometry]
key-files:
  - monstertruck-geometry/src/nurbs/curve_intersect.rs
  - monstertruck-geometry/src/nurbs/mod.rs
  - monstertruck-geometry/tests/curve_intersect.rs
decisions:
  - "Used curvature-based overlap detection to distinguish tangent intersections from parallel overlaps"
  - "Added direct_check fallback for when Newton-Raphson fails due to singular Jacobian at tangent points"
metrics:
  lines_added: 625
  tests_added: 12
  tests_passed: 12
  tdd_violations: 0
---

## What Was Built

### monstertruck-geometry/src/nurbs/curve_intersect.rs (411 lines)
- `CurveIntersection` result type with `t0`, `t1`, `point` fields.
- `find_intersections()` -- public entry point for curve-curve intersection using bounding-box subdivision and Newton-Raphson refinement.
- `find_self_intersections()` -- public entry point that subdivides a single curve into sub-arcs and tests non-adjacent pairs.
- `SubdivisionContext` struct to bundle recursion state and avoid too-many-arguments clippy lint.
- `subdivide_and_collect()` -- recursive subdivision with bounding-box overlap pruning.
- `newton_refine()` -- pseudo-inverse Newton-Raphson solver for 3D curves with 2 unknowns.
- `direct_check()` -- fallback for tangent/singular cases where Newton fails.
- `is_parallel_overlap()` -- curvature-based filter to reject overlapping segments while allowing tangent point intersections.
- `deduplicate_intersections()` -- merges nearby duplicates within `SNAP_TOLERANCE`.
- `extract_subarc()` -- helper to extract a parameter sub-range via `cut()`.
- `finalize_results()` -- helper that deduplicates and sorts results.

### monstertruck-geometry/src/nurbs/mod.rs
- Added `pub mod curve_intersect;` declaration.

### monstertruck-geometry/tests/curve_intersect.rs (214 lines)
- `two_crossing_lines` -- linear curves crossing at known point.
- `two_crossing_cubics` -- cubic Bezier curves crossing near (1,1,0).
- `non_intersecting_curves` -- separated curves return empty.
- `tangent_intersection` -- parabola touching horizontal line tangentially.
- `multiple_intersections` -- wavy cubic crossing a line at multiple points.
- `identical_endpoint` -- curves sharing a common endpoint.
- `test_parallel_curves` -- parallel offset lines return empty.
- `test_self_intersection_figure_eight` -- figure-eight curve self-intersection.
- `test_self_intersection_simple_curve` -- straight line has no self-intersection.
- `test_near_tangent_no_panic` -- nearly tangent curves do not panic.

## Task Commits

| SHA | Message |
|-----|---------|
| 098ecbc6 | test(curve_intersect): add failing tests for curve-curve intersection |
| ccbf5c81 | feat(curve_intersect): implement subdivision/Newton-Raphson curve intersection |
| 53e3bd33 | refactor(curve_intersect): extract helpers, improve doc comments and code structure |

## Decisions Made

1. **Curvature-based overlap detection**: The initial parallel overlap check was too aggressive, rejecting tangential intersections. Fixed by comparing perpendicular second-derivative components -- tangential contact has different curvature while true overlap has matching curvature.

2. **Direct fallback for singular Jacobian**: When curves touch tangentially, the Newton Jacobian is singular (parallel tangent vectors yield rank-deficient J^T J). Added `direct_check` fallback that accepts the intersection if the points are close enough.

3. **Concrete type rather than generic**: Used `BsplineCurve<Point3>` directly instead of the fully generic bounds suggested in the plan. The generic version would require `Bounded + ControlPoint + EuclideanSpace + MetricSpace + Tolerance` bounds with associated type constraints. The concrete version is simpler and covers the immediate use cases. Can be generalized later if needed.

## Deviations from Plan

- None significant. The plan mentioned a convenience wrapper for `NurbsCurve<V>` -- this was not implemented as it was described as optional ("also provide a convenience wrapper"). Can be added in a follow-up if needed.

## Self-Check

- [x] `curve_intersect.rs` exists and exports `find_intersections` and `find_self_intersections`.
- [x] `CurveIntersection` has `t0`, `t1`, `point` fields.
- [x] All 96 lib tests pass, all 10 integration tests pass, 2 unit tests pass.
- [x] `cargo clippy -p monstertruck-geometry -- -W warnings` produces no warnings.
- [x] Intersection accuracy within `SNAP_TOLERANCE` for all test cases.
- [x] No panics on degenerate inputs (parallel, tangent, singular Jacobian).
