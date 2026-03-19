---
target: 17-1
type: impl-review
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: spec-compliance
date: 2026-03-20
verdict: PASS
---

# Implementation Review: 17-1 (Spec Compliance)

**Reviewer:** claude-opus-4-6 | **Round:** 1/3 | **Stage:** spec-compliance | **Date:** 2026-03-20

## Verdict

**PASS**

All five must-have truths are satisfied. The implementation provides `find_intersections` and `find_self_intersections` on `BsplineCurve<Point3>`, returns `Vec<CurveIntersection>` with `t0`/`t1`/`point` fields, handles transversal, tangent, non-intersecting, and self-intersection cases, imports `SNAP_TOLERANCE`, and is publicly accessible via `monstertruck_geometry::nurbs::curve_intersect`. Artifact line counts exceed minimums (411 >= 150, 214 >= 80). All plan-specified tests are present. Deviations exist at the task-action level (concrete vs generic signature, missing NurbsCurve wrapper, widened dedup tolerance) but none violate must-have truths.

## Findings

### Blockers

None

### Suggestions

#### S1: Concrete type instead of plan-specified generic signature [confidence: 72]
- **Confidence:** 72
- **File:** monstertruck-geometry/src/nurbs/curve_intersect.rs:55
- **Issue:** Plan Task 1 explicitly prescribes a generic `find_intersections<P>` with `ControlPoint + EuclideanSpace + MetricSpace + Tolerance + Bounded` trait bounds. The implementation uses concrete `BsplineCurve<Point3>`. While the must-have truths mention `BsplineCurve<Point3>` (satisfied), the task action intended a generic API for broader reusability.
- **Impact:** Limits future use with other point types (e.g., `Point2`). However, the summary notes this was a deliberate simplification.
- **Suggested fix:** Can be deferred. If generalization is needed, add generic bounds later.

#### S2: Missing NurbsCurve convenience wrapper [confidence: 68]
- **Confidence:** 68
- **File:** monstertruck-geometry/src/nurbs/curve_intersect.rs
- **Issue:** Plan Task 1 says "also provide a convenience wrapper for NurbsCurve<V> that delegates to the underlying BsplineCurve." The implementation omits this. The summary characterizes it as optional, but the plan's language reads as a task requirement.
- **Impact:** Downstream code using `NurbsCurve` must manually call `non_rationalized()` before intersecting.
- **Suggested fix:** Add a thin wrapper function, or document the omission as intentional scope reduction.

#### S3: Test assertion tolerance 100x wider than plan specification [confidence: 76]
- **Confidence:** 76
- **File:** monstertruck-geometry/tests/curve_intersect.rs:6-11
- **Issue:** The plan states "Parameter values within SNAP_TOLERANCE of expected." Test helpers `assert_param_near` and `assert_point_near` use `SNAP_TOLERANCE * 100.0`, which is 100x looser. This could mask cases where the algorithm converges to ~10x SNAP_TOLERANCE accuracy rather than within SNAP_TOLERANCE as specified.
- **Impact:** Reduced confidence that the must-have truth "accurate within SNAP_TOLERANCE" holds for all cases. The algorithm itself uses SNAP_TOLERANCE as its convergence criterion (line 283), so the actual output should be within tolerance, but the tests do not verify this tightly.
- **Suggested fix:** Tighten test assertions to `SNAP_TOLERANCE` or at most `SNAP_TOLERANCE * 10.0`, adding a small note if wider tolerance is needed for specific cases.

### Nits

#### N1: Deduplication uses widened tolerance [confidence: 63]
- **Confidence:** 63
- **File:** monstertruck-geometry/src/nurbs/curve_intersect.rs:354-355
- **Issue:** Plan specifies deduplication merges where `|t0_a - t0_b| < SNAP_TOLERANCE`. Implementation uses `SNAP_TOLERANCE * 10.0`. This is likely a pragmatic choice to avoid near-duplicate results but differs from the plan's literal specification.

## Summary

The implementation satisfies all five must-have truths and both artifact requirements. The subdivision/Newton-Raphson algorithm structure matches the plan. All plan-specified test cases are present and cover the required scenarios (crossing lines, crossing cubics, non-intersecting, tangent, multiple, endpoint, parallel, self-intersection, near-tangent). The deviations (concrete type vs generic, missing NurbsCurve wrapper, widened tolerances) are task-level scope reductions that do not break must-have contracts. No blockers.
