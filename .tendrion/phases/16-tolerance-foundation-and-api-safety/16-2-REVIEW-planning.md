---
target: "16-2"
type: "planning"
round: 1
max_rounds: 3
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-19"
verdict: "PASS"
confidence_threshold: 80
---

# Review: planning - 16-2

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-19

## Verdict

**PASS**

**Rationale:** No blockers found. The plan correctly addresses TOLAPI-02 by adding `#[non_exhaustive]` to all five option structs and updating the single external-crate struct literal construction site in `builder.rs`. The plan demonstrates accurate understanding of Rust's `#[non_exhaustive]` semantics (same-crate vs cross-crate behavior). Task sizing is appropriate, verification steps are concrete, and wave/dependency ordering is correct. Two suggestions noted below for accuracy improvement.

## Findings

### Blockers

None

### Suggestions

#### S1: Misleading objective text about `..` syntax [confidence: 91]
- **Confidence:** 91
- **File:** 16-2-PLAN.md, objective tag (line 39)
- **Issue:** The objective states "update all downstream struct literal construction sites to use Default::default() with field overrides via the `..` syntax." However, `#[non_exhaustive]` prevents ALL struct literal syntax from external crates, including `.. Default::default()`. Task 2 correctly describes the `let mut opts = X::default(); opts.field = val;` pattern, but the objective contradicts it.
- **Impact:** An implementer reading only the objective could attempt the wrong fix pattern and waste time debugging compiler errors.
- **Suggested fix:** Change the objective to: "...update all downstream struct literal construction sites to use Default::default() with field mutation."

#### S2: Unnecessary geometry test files in files_modified [confidence: 88]
- **Confidence:** 88
- **File:** 16-2-PLAN.md, frontmatter files_modified (lines 9-11)
- **Issue:** `surface_types_test.rs`, `try_surface_constructors_test.rs`, and `try_gordon_skin_test.rs` are listed in `files_modified`, but these are same-crate tests within `monstertruck-geometry`. The plan itself correctly notes in Task 1 that "`#[non_exhaustive]` prevents struct literal construction from *outside* the defining crate. Within monstertruck-geometry itself, struct literals still work." These test files will compile without any changes.
- **Impact:** Including files that don't need modification in `files_modified` could lead the implementer to make unnecessary changes to working test code, or cause confusion about what actually needs updating. If the intent is to update them for consistency (using Default + mutation style everywhere), this should be stated explicitly as a non-breaking style change.
- **Suggested fix:** Either remove the three geometry test files from `files_modified`, or add an explicit note in Task 1 or a new task explaining that these same-crate tests are being updated for style consistency (not compilation necessity).

### Nits

#### N1: key_links pattern uses Default::default but tests currently use struct literals [confidence: 72]
- **Confidence:** 72
- **File:** 16-2-PLAN.md, must_haves.key_links (line 34-35)
- **Issue:** The key_link from `surface_options.rs` to `try_surface_constructors_test.rs` says the test should use `Default::default` pattern, but since same-crate struct literal construction still works, this link's "via" description is aspirational rather than required.

## Summary

Plan 16-2 is well-structured and technically sound for implementing TOLAPI-02. It correctly identifies the single external-crate breakage point (`builder.rs` line 1441-1444), demonstrates accurate understanding of `#[non_exhaustive]` same-crate vs cross-crate semantics, and includes comprehensive verification steps. The two suggestions address an objective text inconsistency and unnecessary files in `files_modified` -- neither blocks execution. Requirement coverage across the three phase plans is complete (16-1 covers TOLAPI-01, 16-2 covers TOLAPI-02, 16-3 covers TOLAPI-03).
