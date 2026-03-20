---
phase: 17-curve-intersection-engine
verified: true
verified_date: 2026-03-20
tdd_compliance: 100%
plans_total: 1
plans_complete: 1
---

## Phase 17: Curve Intersection Engine

### What Was Built

A reusable curve-curve intersection module (`monstertruck-geometry/src/nurbs/curve_intersect.rs`, 411 lines) added to monstertruck-geometry. Public API: `find_intersections()` for curve-curve intersections using bounding-box subdivision with Newton-Raphson refinement, and `find_self_intersections()` for single-curve self-intersection. Result type `CurveIntersection` exposes `t0`, `t1`, and `point` fields. Module is accessible as `monstertruck_geometry::nurbs::curve_intersect`. Integration tests in `monstertruck-geometry/tests/curve_intersect.rs` (214 lines, 15 tests covering crossing, non-intersecting, tangent, multiple, shared-endpoint, parallel, self-intersection, and no-panic degenerate cases).

### Requirement Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| CURVINT-01 | PASS | curve_intersect.rs exports find_intersections/find_self_intersections; 15 tests pass |

### Test Results

- 15 tests added, 15 passed (0 failures)
- TDD compliance: 100% (strict mode, 1/1 cycles compliant, 0 violations)
- All degenerate cases handled without panics

### Decisions Made

1. Used `BsplineCurve<Point3>` concrete type instead of fully generic bounds -- simpler, covers immediate use cases, generalizable later.
2. Added `direct_check()` fallback for tangent/singular Jacobian cases where Newton-Raphson fails.
3. Used curvature-based overlap detection to distinguish tangent intersections from parallel overlaps.
4. Tightened test tolerances from `SNAP_TOLERANCE * 100.0` to `SNAP_TOLERANCE * 10.0` after S3 review finding.
5. NurbsCurve convenience wrapper not implemented (described as optional in plan); deferred to follow-up.

### Deviations

- 43 auto-fix deviations (project-wide total, not phase-specific)
- 0 approval-needed deviations
- No significant plan deviations; optional NurbsCurve wrapper omitted by design.

### TDD Compliance

Strict mode. Tests written before implementation (commit `098ecbc6` adds failing tests; `ccbf5c81` adds implementation). 0 TDD violations reported.
