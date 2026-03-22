---
target: "22-2"
type: planning
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-22
verdict: PASS
---

# Planning Review: 22-2

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** Planning
**Date:** 2026-03-22

## Verdict

**PASS**

No blockers found. The plan correctly addresses FCONV-03 with a well-structured two-task approach: implementing exact `RevolutedCurve::to_nurbs_surface` via rational circle arc tensor product (Task 1), then wiring it into the `TryFrom<Surface>` conversion path (Task 2). The algorithm description is detailed and mathematically sound. The `Processor` transform/orientation handling in Task 2 is correctly identified as necessary. Code references match the actual codebase.

## Findings

### Blockers

None

### Suggestions

#### S1: Unnecessary dependency on 22-1 prevents wave parallelization [confidence: 88]
- **Confidence:** 88
- **File:** 22-2-PLAN.md, frontmatter `depends_on`
- **Issue:** Plan declares `depends_on: ["22-1"]` but there is no actual code dependency. Plan 22-1 modifies sampling functions (`sample_curve_to_nurbs`, `sample_surface_to_nurbs`, `sample_to_nurbs`), while 22-2 modifies the `TryFrom<Surface>` match arm (line 13 of `fillet_impl.rs`) and adds a new method to `revolved_curve.rs`. These are disjoint code paths. The exact conversion in 22-2 bypasses sampling entirely.
- **Impact:** Forces sequential execution (wave 2 after wave 1) when both plans could execute in parallel as wave 1. This adds unnecessary latency.
- **Suggested fix:** Change `depends_on: []` and `wave: 1`. Both plans modify `fillet_impl.rs` but at completely different locations (sampling functions vs. TryFrom match arm), so there is no merge conflict risk.

#### S2: must_haves claim about "partial arcs" is inaccurate [confidence: 86]
- **Confidence:** 86
- **File:** 22-2-PLAN.md, must_haves truths
- **Issue:** The must_have truth states "Full 2*PI revolution and partial arcs are both handled correctly," but `RevolutedCurve` always has a v-range of `[0, 2*PI)` (confirmed in `revolved_curve.rs:120`). The implementation in Task 1 correctly builds a full-circle 9-point rational representation. There is no partial arc path needed or implemented.
- **Impact:** The must_have truth cannot be verified as stated since no partial arc functionality exists. This could cause confusion during implementation review.
- **Suggested fix:** Change the truth to "Full 2*PI revolution is handled correctly with the standard 9-point rational circle representation" or remove the partial arc claim entirely.

#### S3: Task 2 action uses pseudocode without specifying Curve enum conversion [confidence: 82]
- **Confidence:** 82
- **File:** 22-2-PLAN.md, Task 2 action
- **Issue:** Task 2's action describes the conversion steps in comments but does not explicitly mention that `RevolutedCurve<Curve>` requires extracting the inner `Curve` enum variant and converting it via `TryFrom<Curve> for NurbsCurve<Vector4>` (which exists at `fillet_impl.rs:19`). The inner curve could be `Curve::Line`, `Curve::BsplineCurve`, `Curve::NurbsCurve`, or `Curve::IntersectionCurve`. The `IntersectionCurve` variant also needs consideration since `TryFrom<Curve>` for it uses sampling (may fail or lose fidelity).
- **Impact:** The implementer may miss error handling for the `IntersectionCurve` case where inner curve conversion could fail.
- **Suggested fix:** Add explicit mention that the inner `Curve` should be converted via the existing `TryFrom<Curve> for NurbsCurve<Vector4>` impl, and note that `IntersectionCurve` profiles would still go through sampling (which is acceptable since RevolutedCurve with IntersectionCurve profiles is extremely rare).

### Nits

#### N1: Duplicate closing output tag [confidence: 91]
- **Confidence:** 91
- **File:** 22-2-PLAN.md:189-190
- **Issue:** The file ends with `</output>\n</output>` -- a duplicate closing XML tag.

#### N2: Task sizing slightly unbalanced [confidence: 72]
- **Confidence:** 72
- **File:** 22-2-PLAN.md, tasks section
- **Issue:** Task 1 (implementing the full rational circle arc tensor product algorithm, ~500+ lines with tests) is substantially larger than Task 2 (modifying a single match arm, ~20-30 lines). Task 1 may push toward the upper end of the 15-60 minute range.

## Summary

Plan 22-2 is well-designed for implementing FCONV-03. The rational circle arc tensor product algorithm is described with sufficient mathematical detail for autonomous execution. The two-task decomposition correctly separates the geometry-crate conversion method from the modeling-crate wiring. The primary improvement opportunity is removing the unnecessary dependency on 22-1 to enable parallel execution, but this does not block the plan.
