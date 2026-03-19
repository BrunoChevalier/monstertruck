---
target: "11-2"
type: planning
round: 2
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-19
verdict: PASS
confidence_threshold: 80
---

# Review: planning - 11-2

**Reviewer:** claude-opus-4-6
**Round:** 2 of 3
**Stage:** planning
**Date:** 2026-03-19

## Verdict

**PASS**

**Rationale:** All three blockers from round 1 have been resolved. B1 (return type mismatch): `try_sweep_periodic` now consistently returns `Result<Shell>` in both must_haves and Task 3 signature. B2 (SweepBuilder missing): the objective section now has an explicit NOTE explaining that the free-function pattern matches codebase conventions and satisfies the roadmap criterion's intent. B3 (oversized task): the original 5-wrapper task has been split into Task 2 (three wrappers) and Task 3 (two wrappers). Round 1 suggestions S1 and S2 were also addressed: `face_from_bspline_surface` is now an explicit shared helper in Task 2, and the stream-of-consciousness text has been replaced with a single clear approach. No new blockers introduced.

## Findings

### Blockers

None

### Suggestions

#### S1: Boundary curve extraction in face_from_bspline_surface has swapped column_curve/row_curve semantics [confidence: 88]
- **Confidence:** 88
- **File:** 11-2-PLAN.md, Task 2 lines 150-154
- **Issue:** The plan's code assigns `column_curve(0)` as "bottom" (v=0, u varies) and `row_curve(0)` as "left" (u=0, v varies). Based on the actual BsplineSurface implementation, `column_curve(row_idx)` returns a curve using `knot_vector_v` at a fixed u-position (not fixed v), while `row_curve(col_idx)` returns a curve using `knot_vector_u` at a fixed v-position. For surfaces produced by `skin()`, where sections become columns in the control point grid: `row_curve(0)` is the first section (v=0 boundary, u varies) and `column_curve(0)` is the u=0 boundary (v varies). The assignments in the plan are backwards.
- **Impact:** An implementer copying the code verbatim would create edges with wrong curve assignments, leading to a topologically inconsistent face where boundary curves do not match vertex positions. The plan does include a mitigating NOTE (line 168) telling the implementer to verify the actual layout, which reduces the risk of this causing a runtime failure.
- **Suggested fix:** Swap the assignments: `row_curve(0)` should be `bottom`, `row_curve(n_cols-1)` should be `top`, `column_curve(0)` should be `left`, `column_curve(n_rows-1)` should be `right`. Or remove the inline code and describe the intent, letting the implementer derive correct assignments from the API docs.

### Nits

#### N1: Duplicate closing output tag [confidence: 96]
- **Confidence:** 96
- **File:** 11-2-PLAN.md, line 346
- **Issue:** The file ends with `</output>` twice (lines 345 and 346). The second one is spurious. Carried over from round 1 (N1).

## Summary

Plan 11-2 is in good shape after round 1 revisions. All three previous blockers are resolved: the return type for `try_sweep_periodic` is consistent, the SweepBuilder naming gap is explicitly addressed, and the oversized task has been properly split. The `face_from_bspline_surface` helper is now a clear, shared utility. The one remaining suggestion (S1) concerns swapped `column_curve`/`row_curve` semantics in the helper code -- the plan's mitigating NOTE partially addresses this, but the inline code as written is incorrect. This is a suggestion rather than a blocker because the NOTE instructs the implementer to verify boundary layout against the actual API, and the `is_geometric_consistent` tests will catch any topology errors.
