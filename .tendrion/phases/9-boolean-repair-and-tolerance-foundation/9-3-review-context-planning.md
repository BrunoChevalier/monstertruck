# Review Context: Plan 9-3 (Planning Review, Round 3 of 3 -- FINAL ROUND)

## Plan Under Review

**Plan ID:** 9-3
**Plan Path:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-PLAN.md`
**Round:** 3 of 3 (FINAL)

---

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 9-1 | 1 | Establish a documented numeric tolerance policy in monstertruck-core and eliminate the hardcoded 1.0e-6 in monstertruck-solid's fillet edge_select. Scope is limited to monstertruck-core and monstertruck-solid fillet files. |
| 9-2 | 1 | Fix boolean operation bugs by hardening the face classification pipeline, making the unknown-face classification loop resilient to single-face failures, wiring coincident-face detection as diagnostic-only logging, and improving shell healing robustness. Tangent face handling is out of scope. |

Full sibling plans can be read from `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/{sibling_plan_id}-PLAN.md` if cross-plan analysis is needed.

---

## Roadmap (Phase 9 Section)

### Phase 9: Boolean Repair and Tolerance Foundation
**Goal**: Boolean operations on complex faces produce correct topology and all crates share a consistent numeric tolerance policy
**Depends on**: None
**Requirements**: BOOL-01, TEST-02
**Success Criteria** (what must be TRUE):
  1. The v0.3.0 criteria 2 and 4 gaps (boolean result face handling) pass their original verification checks without manual workarounds
  2. A shared tolerance constants module exists and is imported by truck-shapeops, truck-modeling, and truck-meshalgo
  3. Running `cargo test -p truck-shapeops` passes with no boolean-related test failures
  4. Tolerance constants are documented with rationale for each value choice

---

## Previous Review (Round 2 -- FAIL)

```markdown
---
target: "9-3"
type: "planning"
round: 2
max_rounds: 3
reviewer: "codex"
stage: "planning"
date: "2026-03-18"
verdict: "FAIL"
confidence_threshold: 80
---

# Review: Planning - Phase 9

**Reviewer:** codex
**Round:** 2 of 3
**Stage:** planning
**Date:** 2026-03-18

## Verdict

**FAIL**

**Rationale:** FAIL due to B1. The round-1 blockers around declared file scope and chained-boolean coverage are resolved, but the plan still does not cover the phase's end-to-end boolean validation because every `monstertruck-solid` verification command is `--lib` only and excludes the existing integration regression suite for tangent, coincident, and pole-degenerate cases.

## Findings

### Blockers

#### B1: End-to-end boolean validation still excludes the integration regression suite [confidence: 94]
- **Confidence:** 94
- **File:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-PLAN.md:45`, `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-PLAN.md:218`, `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-PLAN.md:243`
- **Issue:** The objective promises end-to-end integration tests, but Task 1 only adds unit tests under `monstertruck-solid/src/transversal/integrate/tests.rs`, and both verification steps run `cargo nextest run -p monstertruck-solid --lib ...`. That excludes `monstertruck-solid/tests/boolean_edge_cases.rs`, which is the existing public-API regression suite for tangent-face, coincident-face, and pole-degeneration scenarios that are closest to Phase 9's "complex faces" goal. Sibling plan `9-2` also verifies `monstertruck-solid` with `--lib`, so this gap is not covered elsewhere in the phase.
- **Impact:** The plan can pass while the boolean repairs still fail the most relevant end-to-end scenarios, so `BOOL-01` and the roadmap's `truck-shapeops` verification equivalent are not actually proven.
- **Suggested fix:** Add an explicit package-level verification that includes integration tests, such as dropping `--lib` from the relevant `monstertruck-solid` runs or adding `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast`. If `9-3` is meant to own end-to-end validation, make those integration tests part of the acceptance criteria.

### Suggestions

#### S1: Limit post-verification fixes to the declared edit surface [confidence: 84]
- **Confidence:** 84
- **File:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-PLAN.md:235-241`
- **Issue:** Task 2 says to "Fix any issues found" after running workspace-level verification, but the plan only declares edits in `monstertruck-solid/src/transversal/integrate/mod.rs`, `monstertruck-solid/src/transversal/integrate/tests.rs`, and `monstertruck-solid/src/transversal/loops_store/mod.rs`.
- **Impact:** If the new validation exposes a bug in another production file, the executor has to choose between violating `files_modified` or stopping mid-plan for replanning.
- **Suggested fix:** Either constrain Task 2 to issues in the declared files or expand `files_modified` and task scope to the additional production modules this plan expects an executor to touch.

### Nits

#### N1: Task 1 still contains a stale self-edit instruction [confidence: 99]
- **Confidence:** 99
- **File:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-3-PLAN.md:68`
- **Issue:** The action text says to "Also add to the file's `<files>` field" even though Task 1's `<files>` field already includes `monstertruck-solid/src/transversal/loops_store/mod.rs`.

## Summary

This revision fixed the two round-1 blockers: the declared file set now matches the task body, and the chained-boolean must-have is planned explicitly. The remaining problem is validation coverage: the plan still proves only lib/unit scenarios while Phase 9 needs the public-API boolean regression suite to be part of the acceptance path.
```

---

## Note for Round 3 Reviewer

This is the FINAL review round (3 of 3). The round 2 review (included above) found one blocker: the plan's verification steps used `--lib` only and excluded the `boolean_edge_cases` integration regression suite. The plan has since been updated -- please check whether this blocker has been resolved by examining the current plan at the path above.
