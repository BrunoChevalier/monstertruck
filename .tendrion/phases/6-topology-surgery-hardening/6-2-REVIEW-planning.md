---
target: "6-2"
type: "planning"
round: 1
max_rounds: 3
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-16"
verdict: "FAIL"
confidence_threshold: 80
---

# Review: planning - 6-2

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-16

## Verdict

**FAIL**

**Rationale:** FAIL due to B1. The plan's must_haves state that "parameter-space projection is attempted first for IntersectionCurve edges, with NURBS approximation as fallback," but Task 2's implementation description and code example go straight to NURBS conversion without any projection-first logic. This is a spec-implementation mismatch within the plan itself. Additionally, the plan's technical narrative is partially misleading about when IntersectionCurve edges appear (S1), and convert.rs is listed in files_modified without any task modifying it (S2).

## Findings

### Blockers

#### B1: Must-have truth contradicts task implementation [confidence: 91]
- **Confidence:** 91
- **File:** 6-2-PLAN.md, must_haves.truths[3] vs Task 2 action
- **Issue:** The must_haves truth states: "Parameter-space projection is attempted first for IntersectionCurve edges, with NURBS approximation as fallback." The roadmap success criterion 1 also says "by projecting splitting curves into parameter space, with NURBS approximation fallback." However, Task 2's implementation description and the `ensure_cuttable_edge` code example perform unconditional NURBS conversion -- there is no "try projection first, fall back to NURBS" logic. The key_links reference `search_parameter` as a "parameter-space projection pattern" but this is never wired into the task action.
- **Impact:** An implementer following the task description will produce code that satisfies the NURBS fallback behavior but NOT the "projection first" contract. Either the must_have truth should be updated to match the simpler approach (always convert to NURBS), or Task 2 should describe the two-step projection-then-fallback logic.
- **Suggested fix:** Choose one approach and make both the must_haves and the task description consistent. If projection-first is desired, add explicit steps in Task 2 describing how `search_parameter` on the IntersectionCurve's underlying surface is tried before falling back to NURBS sampling. If unconditional NURBS conversion is the intended approach, update must_haves.truths[3] to remove the "attempted first" language.

### Suggestions

#### S1: Misleading technical narrative about when IntersectionCurve edges appear [confidence: 86]
- **Confidence:** 86
- **File:** 6-2-PLAN.md, Task 2 action and objective
- **Issue:** The plan states IntersectionCurve edges come "from boolean operations" and that `cut_face_by_bezier` encounters them on "boolean-result faces." However, `convert_shell_in` (convert.rs:140) converts ALL curves to `Curve::NurbsCurve` before any fillet processing begins. The IntersectionCurve edges that `cut_face_by_bezier` might actually encounter are created during fillet processing itself by `create_new_side` (topology.rs:144), not from the original boolean result. The plan's approach (converting IntersectionCurve to NURBS in `cut_face_by_bezier`) will work regardless, but the misleading explanation could confuse the implementer about the actual data flow.
- **Impact:** An implementer may spend time investigating boolean-result edges when the real source is internal fillet construction. The test design in Task 1 may not exercise the actual failure path if the IntersectionCurve edges from booleans are already converted to NURBS before `cut_face_by_bezier` runs.
- **Suggested fix:** Add a note in Task 2 clarifying that `convert_shell_in` already converts external IntersectionCurve edges to NURBS, and that the hardening in `cut_face_by_bezier` primarily protects against IntersectionCurve edges created by `create_new_side` during multi-edge fillet operations. Adjust Test 3 in Task 1 to construct a scenario where `create_new_side` has introduced IntersectionCurve edges before `cut_face_by_bezier` is called.

#### S2: convert.rs listed in files_modified but no task modifies it [confidence: 88]
- **Confidence:** 88
- **File:** 6-2-PLAN.md, frontmatter files_modified vs tasks
- **Issue:** `monstertruck-solid/src/fillet/convert.rs` is listed in `files_modified` and appears in `key_links`, but no task action describes any modification to this file. The key_link says `FilletableCurve::to_nurbs_curve()` is used FROM convert.rs, but this is a read dependency, not a write.
- **Impact:** The `files_modified` field should accurately reflect which files are changed. Listing unmodified files may cause confusion about plan scope or trigger unnecessary file locking in parallel workflows.
- **Suggested fix:** Either remove `convert.rs` from `files_modified` (if it truly won't be changed), or add a task step describing what modification is needed in convert.rs.

#### S3: error.rs artifact specified but no task describes changes [confidence: 85]
- **Confidence:** 85
- **File:** 6-2-PLAN.md, must_haves.artifacts[2] vs tasks
- **Issue:** The must_haves specify an artifact for `error.rs` with `min_lines: 50` and `contains: "FilletError"`. The file already has 50 lines and contains `FilletError`. No task describes adding new error variants. Task 3 vaguely mentions "add appropriate error handling" but does not specify new `FilletError` variants for IntersectionCurve handling failures.
- **Impact:** If new error variants are intended, they should be explicitly described. If no changes to error.rs are needed, it should be removed from files_modified and must_haves artifacts to avoid confusion.
- **Suggested fix:** Either specify the exact new error variants needed (e.g., `IntersectionCurveConversionFailed`) in a task, or remove error.rs from files_modified and must_haves artifacts if the existing variants suffice.

### Nits

#### N1: Duplicate closing tag [confidence: 95]
- **Confidence:** 95
- **File:** 6-2-PLAN.md:196-197
- **Issue:** Two `</output>` closing tags at end of file. The structural validator passed but this is a formatting error.

#### N2: Code sample uses degree-1 NURBS approximation [confidence: 72]
- **Confidence:** 72
- **File:** 6-2-PLAN.md, Task 2 code example
- **Issue:** The `ensure_cuttable_edge` code uses `KnotVector::uniform_knot(1, sample_count)` which creates a degree-1 (piecewise linear) approximation. This matches the existing `sample_curve_to_nurbs` in convert.rs, so it's consistent, but a degree-3 approximation would be geometrically smoother. Since this is consistent with existing code, this is purely a nit.

## Summary

Plan 6-2 addresses the right problem space (TOPO-01) with a reasonable approach, and together with plan 6-1 covers both Phase 6 requirements. The structural validation passes and task sizing is appropriate. However, there is an internal contradiction between the must_haves truth about "parameter-space projection first" and the task description which only implements unconditional NURBS conversion. This must be reconciled before the plan can proceed. The technical narrative about the source of IntersectionCurve edges is also partially misleading, and two files (convert.rs, error.rs) are listed in files_modified without corresponding task modifications.
