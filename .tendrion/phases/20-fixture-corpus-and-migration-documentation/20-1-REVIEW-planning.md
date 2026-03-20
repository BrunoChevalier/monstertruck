---
target: "20-1"
type: planning
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-20
verdict: PASS
---

# Planning Review: 20-1

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** Planning
**Date:** 2026-03-20

## Verdict

**PASS**

No blockers found. The plan covers FIXTURE-01 and FIXTURE-02 requirements thoroughly with well-structured tasks, correct fixture types, and realistic integration tests. Requirement coverage is complete for this plan's scope (FIXTURE-03 and DOC-01 are correctly handled by sibling plans 20-3 and 20-2 respectively).

## Findings

### Blockers

None

### Suggestions

#### S1: No integration test exercises near-zero weight NURBS curve [confidence: 74]
- **Confidence:** 74
- **File:** 20-1-PLAN.md, Task 3 action
- **Issue:** Task 2 creates `fixture_near_zero_weight_nurbs()` returning a `NurbsCurve<Vector4>`, but Task 3 lists no integration test that exercises this fixture through any operation. The smoke tests will validate structure, but no test verifies graceful handling under evaluation or use in a surface constructor.
- **Impact:** The FIXTURE-02 success criterion says "tests verifying graceful handling" for near-degenerate cases. A rational curve with near-zero weight is one of the most pathological cases and would benefit from an explicit evaluation test (e.g., verifying `subs()` returns finite values at the near-zero weight parameter).
- **Suggested fix:** Add a 7th test in Task 3 that evaluates the near-zero weight NURBS curve at several parameter values (including near the near-zero weight control point) and asserts finite results.

#### S2: Existing fixture overlap with collapsed control polygon surface [confidence: 68]
- **Confidence:** 68
- **File:** 20-1-PLAN.md, Task 2 action item 3
- **Issue:** The existing `degenerate_surface_collapsed_edge()` in test_fixtures.rs already creates a bi-quadratic surface where an entire row of control points collapses to the same location. The planned `fixture_collapsed_control_polygon_surface()` collapses a column instead of a row. While these are geometrically distinct (different parametric direction degeneracy), the plan should acknowledge the existing fixture to avoid confusion.
- **Impact:** Minor -- the implementer may wonder if this duplicates existing work. Clarifying the distinction (row vs column, bi-quadratic vs bi-cubic) would help.
- **Suggested fix:** Add a note in Task 2 action item 3 acknowledging `degenerate_surface_collapsed_edge()` and clarifying that the new fixture tests column-collapse (orthogonal parametric direction) at bi-cubic degree.

### Nits

#### N1: Duplicate closing output tag [confidence: 91]
- **Confidence:** 91
- **File:** 20-1-PLAN.md:152-153
- **Issue:** The plan ends with `</output>\n</output>` -- a duplicated closing tag. The plan-structure validator passes but this is malformed XML.

#### N2: must_haves missing collapsed control points truth [confidence: 83]
- **Confidence:** 83
- **File:** 20-1-PLAN.md, frontmatter must_haves.truths
- **Issue:** The truths list mentions inflection rail, converging rails, degenerate section, near-zero Jacobian surface, and near-zero weight NURBS curve, but omits `fixture_collapsed_control_polygon_surface` and `fixture_cusped_rail`. These are created in the tasks but not captured as must_have truths.

## Summary

Plan 20-1 is well-constructed with clear task decomposition, correct wave ordering, and appropriate requirement coverage. The fixture types are well-chosen and address genuine pathological geometry cases relevant to the NURBS surface constructor pipeline. Task sizing is appropriate (3 tasks, each creating related fixtures + tests). The existing codebase patterns (import paths, test structure, type usage) are correctly followed. Two suggestions note minor completeness gaps in test coverage and existing fixture acknowledgment, but neither rises to blocker severity.
