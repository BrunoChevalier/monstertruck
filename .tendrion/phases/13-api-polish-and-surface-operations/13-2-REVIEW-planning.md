---
target: "13-2"
type: planning
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-19
verdict: PASS
---

# Planning Review: 13-2-PLAN.md

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** Planning
**Date:** 2026-03-19

## Verdict

**PASS**

No blockers found. The plan is well-structured, technically sound, and correctly implements SURF-03. The `split_at_u`/`split_at_v`/`sub_patch` API design correctly builds on the existing `cut_u`/`cut_v` infrastructure. The TDD approach is appropriate for this kind of geometric correctness work. The delegation pattern for NurbsSurface matches the established codebase conventions exactly (cf. existing `cut_u`/`cut_v` delegation at nurbs_surface.rs:431-438). The sub_patch implementation strategy of successive splits is mathematically correct since `cut_u`/`cut_v` preserve absolute parameter domains.

## Findings

### Blockers

None

### Suggestions

#### S1: Missing NurbsSurface test file from files_modified [confidence: 82]
- **Confidence:** 82
- **File:** 13-2-PLAN.md, frontmatter `files_modified`
- **Issue:** Task 3 mentions adding "inline tests or doc-tests for the NurbsSurface versions" but `monstertruck-geometry/tests/nurbssurface.rs` (which exists in the codebase) is not listed in `files_modified`. If the implementer adds integration tests for NurbsSurface split/sub_patch, they would likely go in that file.
- **Impact:** The `files_modified` list is used for dependency tracking across plans. An unlisted file modification could cause tracking gaps.
- **Suggested fix:** Either add `monstertruck-geometry/tests/nurbssurface.rs` to `files_modified`, or clarify in Task 3 that NurbsSurface tests should be doc-tests only (which are inline in `nurbs_surface.rs`, already listed).

#### S2: Error handling for out-of-domain parameters in sub_patch [confidence: 78]
- **Confidence:** 78
- **File:** 13-2-PLAN.md, Task 2 action
- **Issue:** The `sub_patch` doc-comment says "Panics if `u0 >= u1` or `v0 >= v1`, or if the range is outside the surface domain." However, the implementation relies on `split_at_u`/`split_at_v` which delegate to `cut_u`/`cut_v`. Looking at `cut_u` (bspline_surface.rs:1202-1258), out-of-domain parameters are handled by returning degenerate surfaces rather than panicking. The plan's doc-comment promises panics that the implementation may not actually produce. No explicit validation is shown before the splits.
- **Impact:** The documented contract (panics on invalid input) would not match actual behavior (silently produces degenerate/incorrect surfaces).
- **Suggested fix:** Add explicit domain validation at the top of `sub_patch` (check `u0 < u1`, `v0 < v1`, and ranges within `[u_start, u_end]` / `[v_start, v_end]`), or adjust the doc-comment to describe actual behavior.

### Nits

#### N1: Duplicate closing tag in output section [confidence: 95]
- **Confidence:** 95
- **File:** 13-2-PLAN.md:236
- **Issue:** There is a stray `</output>` tag at the end of the file, creating a malformed XML structure.

#### N2: Test file min_lines threshold is trivially satisfied [confidence: 88]
- **Confidence:** 88
- **File:** 13-2-PLAN.md, must_haves.artifacts
- **Issue:** The `bspsurface.rs` test file artifact has `min_lines: 30` but the file already has 641 lines. This threshold provides no verification value for the new work.
- **Suggested fix:** Set `min_lines` to something like 680 to verify new tests were actually added.

## Summary

Plan 13-2 is a focused, well-scoped plan that correctly addresses SURF-03. The three-task TDD structure is appropriate, the implementation approach of clone+cut for non-mutating split is sound, and the NurbsSurface delegation follows established codebase patterns. The sub_patch algorithm of successive parameter splits is mathematically correct. Two suggestions warrant attention: ensuring `files_modified` is complete and adding explicit parameter validation for `sub_patch`. Neither blocks execution.
