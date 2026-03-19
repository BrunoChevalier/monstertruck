---
target: "17-1"
type: "planning"
round: 1
max_rounds: 3
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-20"
verdict: "PASS"
confidence_threshold: 80
---

# Review: planning - 17-1

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-20

## Verdict

**PASS**

**Rationale:** No blockers found. The plan comprehensively covers the single requirement CURVINT-01 and all three roadmap success criteria. The algorithm design (subdivision + Newton-Raphson refinement) is sound and well-specified. The plan correctly references existing codebase APIs (BoundingBox `^` operator, `cut`, `roughly_bounding_box`, `range_tuple`, SNAP_TOLERANCE from Phase 16). Task structure is logical with appropriate verification steps. Three suggestions and two nits are noted for improvement.

## Findings

### Blockers

None

### Suggestions

#### S1: CurveIntersection hardcodes Point3 but find_intersections is generic over P [confidence: 82]
- **Confidence:** 82
- **File:** 17-1-PLAN.md, Task 1 action (lines 75-112)
- **Issue:** The `CurveIntersection` struct uses a concrete `Point3` field for `point`, but `find_intersections` is generic over `P` with broad trait bounds. If `P` is not `Point3` (e.g., `Point2` for 2D curves), the function cannot construct a `CurveIntersection` without a conversion mechanism. The plan mentions a NurbsCurve convenience wrapper that would need `Vector4` -> `Point3` conversion, but does not specify how the generic `P` -> `Point3` mapping works in the main function.
- **Impact:** The implementer may encounter compile errors or need to add trait bounds or make the struct generic, which deviates from the plan.
- **Suggested fix:** Either make `CurveIntersection` generic over the point type (`CurveIntersection<P>`) or add a trait bound like `Into<Point3>` on `P`, or constrain the function to `P = Point3` and keep the NurbsCurve wrapper separate. Document the chosen approach explicitly.

#### S2: Task 1 is large - combines types, subdivision, Newton refinement, deduplication, and NurbsCurve wrapper [confidence: 81]
- **Confidence:** 81
- **File:** 17-1-PLAN.md, Task 1
- **Issue:** Task 1 encompasses at least 5 distinct concerns: result type definition, module registration, recursive subdivision algorithm, Newton-Raphson refinement with pseudo-inverse, deduplication, and a NurbsCurve convenience wrapper. This likely exceeds the 60-minute guideline for a single task.
- **Impact:** A large task increases risk of partial completion and makes verification less granular.
- **Suggested fix:** Consider splitting Task 1 into two tasks: (a) types + module registration + subdivision skeleton, (b) Newton refinement + deduplication + NurbsCurve wrapper.

#### S3: Plan does not note that `cut` is `&mut self` and requires cloning [confidence: 86]
- **Confidence:** 86
- **File:** 17-1-PLAN.md, Task 1 action (lines 120-127)
- **Issue:** The plan instructs the implementer to "use `cut` to extract sub-arcs" for bounding box computation during subdivision. However, `BsplineCurve::cut` is a `&mut self` method that splits the curve in place. The implementer must clone the curve before cutting, which the plan does not mention. The note about caching sub-curves partially addresses this but does not make the cloning requirement explicit.
- **Impact:** Without this guidance, the implementer may write code that inadvertently mutates shared curve references, leading to subtle bugs in the recursive subdivision.
- **Suggested fix:** Add a note that `cut` mutates the curve and sub-arcs must be obtained by cloning first, e.g., `let mut sub = curve.clone(); let right = sub.cut(mid); // sub is now left half`.

### Nits

#### N1: Duplicate closing output tag [confidence: 95]
- **Confidence:** 95
- **File:** 17-1-PLAN.md, line 261
- **Issue:** There is a stray `</output>` tag after the output section's closing tag on line 260.

#### N2: TDD type but implementation precedes tests [confidence: 72]
- **Confidence:** 72
- **File:** 17-1-PLAN.md, task ordering
- **Issue:** The plan has `type: tdd` but Task 1 creates the implementation and Task 2 writes the integration tests. Strict TDD writes tests first. For this algorithmic module, implementation-first is pragmatically reasonable since the API shape emerges during algorithm design, but the type label is slightly misleading.

## Summary

This is a well-structured single-plan phase that fully addresses CURVINT-01 and all three roadmap success criteria. The algorithm choice (bounding-box subdivision + Newton-Raphson) is standard and appropriate for curve-curve intersection. The plan correctly leverages existing codebase infrastructure (BoundingBox operations, SNAP_TOLERANCE, curve evaluation/derivative APIs). The generic type mismatch between the concrete `CurveIntersection::point` field and the generic function signature is the most notable concern but is unlikely to block a competent implementer. Task 1 is on the large side but manageable for an autonomous agent.
