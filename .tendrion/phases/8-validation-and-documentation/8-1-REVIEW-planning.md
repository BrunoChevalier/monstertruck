---
target: "8-1"
type: "planning"
round: 3
max_rounds: 3
reviewer: "codex"
stage: "planning"
date: "2026-03-16"
verdict: "FAIL"
confidence_threshold: 80
---

# Review: Planning - Phase 8

**Reviewer:** codex
**Round:** 3 of 3
**Stage:** planning
**Date:** 2026-03-16

## Verdict

**FAIL**

**Rationale:** `B1` must be fixed before execution. The plan is structurally valid (`td-tools` passed), its wave/dependency alignment with sibling plan `8-2` is coherent, and Phase 8 requirement coverage is complete across the sibling pair (`8-1` for `TOPO-03`, `8-2` for `DOC-01`), but `8-1` still contains an internal acceptance-criteria conflict around the direct `euler_poincare_check` test case.

## Findings

### Blockers

#### B1: Must-have truth conflicts with the task's own test design [confidence: 93]
- **Confidence:** 93
- **File:** `.tendrion/phases/8-validation-and-documentation/8-1-PLAN.md:20`
- **Issue:** The frontmatter still declares a must-have truth that "`euler_poincare_check` returns true for a valid closed shell and false for an invalid one," but Task 3's fourth test explicitly says a realizable closed-shell wrong-chi case is not constructible here and then instructs the executor to assert `true` on a non-closed shell because the check intentionally skips non-`Closed` shells.
- **Impact:** The plan cannot satisfy both its own acceptance truth and its task instructions. That leaves the executor without a consistent completion target and makes the plan internally inconsistent.
- **Suggested fix:** Align the frontmatter, Task 3 test description, test name, and done criteria to the same realizable outcome. If the real intent is to verify guard behavior, change the must-have to match that. If a `false` case is truly required, specify an actually constructible shell that makes `euler_poincare_check` return `false`.

### Suggestions

#### S1: Name both `apply_single_edge_fillet` assertion sites explicitly [confidence: 88]
- **Confidence:** 88
- **File:** `.tendrion/phases/8-validation-and-documentation/8-1-PLAN.md:114`
- **Issue:** Task 2 says to add `debug_assert_euler` "after each successful single-edge fillet in the chain loop," but `edge_select.rs` has two separate success sites for `apply_single_edge_fillet`: the single-edge chain branch and the fallback per-edge loop after wire-filleting fails. The current wording reads like the fallback mid-chain path only.
- **Impact:** An executor could instrument only one path and still believe the task is complete, leaving the "after every fillet topology modification" coverage less deterministic than intended.
- **Suggested fix:** Call out both `apply_single_edge_fillet` success sites explicitly and state which one should use `debug_assert_euler` versus which is covered by the final `debug_assert_topology`.

### Nits

None

## Summary

This plan is close to execution-ready. The prior corrupted-orientation blocker is resolved, the structure validates cleanly, and the sibling split for Phase 8 is coherent. The remaining blocker is an internal mismatch between the plan's declared must-have and the test Task 3 actually describes for `euler_poincare_check`.
