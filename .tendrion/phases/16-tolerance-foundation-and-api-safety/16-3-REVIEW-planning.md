---
target: "16-3"
type: "planning"
round: 1
max_rounds: 3
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-19"
verdict: "PASS"
confidence_threshold: 80
---

# Review: planning - 16-3

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-19

## Verdict

**PASS**

**Rationale:** No blockers found. The plan is well-structured, feasible, and correctly addresses TOLAPI-03. All five deprecated methods are covered with detailed delegation patterns. Behavioral differences between old and new implementations are identified and documented. The dependency on plan 16-2 is conservative but defensible.

## Findings

### Blockers

None

### Suggestions

#### S1: Missing test for deprecated_birail2_still_works [confidence: 88]
- **Confidence:** 88
- **File:** 16-3-PLAN.md, must_haves.key_links
- **Issue:** The must_haves truths state "User calls deprecated birail2() and gets identical results to try_birail2() with matching options" but no `deprecated_birail2_still_works` test exists. Tests exist for `deprecated_gordon_still_works`, `deprecated_skin_still_works`, `deprecated_sweep_rail_still_works`, and `deprecated_birail1_still_works` in the test files, but birail2 is missing. The key_links section only references gordon/skin tests. Task 2's verify step uses `cargo test -- sweep_rail birail` which would catch birail1 but there is no specific deprecated-birail2 regression test.
- **Impact:** After refactoring, there is no automated verification that the deprecated birail2() produces identical results to try_birail2(). If the delegation is subtly wrong, it would go undetected.
- **Suggested fix:** Add a `deprecated_birail2_still_works` test to the test file (similar to the existing deprecated_birail1_still_works test), or add it as an explicit sub-action in Task 2. Also update key_links to reference it.

#### S2: Dependency on 16-2 is overly conservative [confidence: 82]
- **Confidence:** 82
- **File:** 16-3-PLAN.md, frontmatter depends_on
- **Issue:** Plan 16-3 depends on 16-2 (non_exhaustive), but the refactoring work is functionally independent. The delegation code uses `::default()` and field mutation, both of which work identically with or without `#[non_exhaustive]`. The dependency exists only to avoid concurrent modification of related files, but 16-2 modifies `surface_options.rs` and test files while 16-3 only modifies `bspline_surface.rs`.
- **Impact:** Forcing wave 2 means this plan cannot run in parallel with 16-1 and 16-2, adding unnecessary serialization. The files_modified sets do not overlap.
- **Suggested fix:** Consider moving to wave 1 with `depends_on: []` since there is no actual file overlap. However, the current ordering is safe and conservative, so this is not a blocker.

### Nits

#### N1: Duplicate closing output tag [confidence: 95]
- **Confidence:** 95
- **File:** 16-3-PLAN.md:202
- **Issue:** Line 202 has a stray `</output>` closing tag after the `</output>` block that was already properly closed on line 201.

## Summary

Plan 16-3 is well-crafted with detailed code snippets, accurate line references, and thorough documentation of behavioral differences between deprecated and try_* variants. The plan correctly addresses TOLAPI-03 and goes beyond the minimum requirement (gordon only) to cover all five deprecated constructors. The main improvement opportunity is adding a missing regression test for deprecated birail2.
