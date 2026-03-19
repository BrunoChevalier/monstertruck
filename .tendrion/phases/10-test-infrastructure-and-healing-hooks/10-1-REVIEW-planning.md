---
target: "10-1"
type: planning
round: 2
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-19
verdict: PASS
---

# Planning Review: 10-1

**Reviewer:** claude-opus-4-6
**Round:** 2 of 3
**Stage:** planning
**Date:** 2026-03-19

## Verdict

**PASS** -- All round 1 blockers have been resolved. No new blockers found.

B1 (cfg(test) invisible cross-crate): Resolved. The plan now explicitly specifies "unconditionally compiled module (NOT `#[cfg(test)]`)" in the objective, Task 1 action, must_haves, verification section, and success_criteria. The only `#[cfg(test)]` usage is for the internal `mod tests` block within `test_fixtures.rs`, which is correct standard Rust practice.

B2 (cargo test/check instead of nextest): Resolved. All verification commands now use `cargo nextest run`. Task 1 verify, Task 2 verify, must_haves truths, and the verification section all reference nextest exclusively.

Previous suggestions S1 (vague Task 2 verify) and S2 (BOOL-02 coverage clarity) are also addressed: Task 2 now has a concrete smoke-test verification strategy, and the success_criteria correctly scopes to TEST-01.

## Findings

### Blockers

None

### Suggestions

#### S1: fixture_helpers.rs will compile as a standalone test crate [confidence: 81]
- **Confidence:** 81
- **File:** 10-1-PLAN.md, Task 2 action
- **Issue:** Placing `fixture_helpers.rs` directly in `monstertruck-solid/tests/` means Cargo will compile it as a standalone integration test binary (in addition to its use as a `mod` import from other test files). Since the file contains only `pub fn` definitions and no `#[test]` attributes, it will produce a binary with zero tests. This is harmless but wasteful and produces noise in test output. The idiomatic Rust pattern for shared test helpers is `tests/common/mod.rs` or `tests/fixture_helpers/mod.rs` (subdirectory module).
- **Impact:** Minor: extra compilation unit, possible confusion in test output showing a test binary with 0 tests.
- **Suggested fix:** Move the helper to `monstertruck-solid/tests/fixture_helpers/mod.rs` so it is only compiled when explicitly imported via `mod fixture_helpers;`. This follows Rust convention for shared test utilities.

#### S2: Plan references deprecated surface.subs(u, v) API [confidence: 74]
- **Confidence:** 74
- **File:** 10-1-PLAN.md, Task 2 action (line 111)
- **Issue:** Task 2 instructs sampling surface boundaries using `surface.subs(u, v)`. The `subs` method exists on the `ParametricSurface` trait but is marked deprecated in favor of `surface.evaluate(u, v)`. This is guidance for the implementer, not a compilation error.
- **Impact:** Implementer may use deprecated API; clippy will warn.
- **Suggested fix:** Reference `surface.evaluate(u, v)` instead of `surface.subs(u, v)`.

### Nits

#### N1: Duplicate closing output tag may persist [confidence: 58]
- **Confidence:** 58
- **File:** 10-1-PLAN.md, lines 145-146
- **Issue:** Round 1 noted a duplicate `</output>` tag. The current state is ambiguous due to trailing whitespace/empty lines. If the duplicate persists, it is a minor XML structure issue.

## Summary

The plan is well-structured and feasible. Both round 1 blockers have been convincingly addressed: the fixture module is explicitly unconditionally compiled (no cfg(test) gate on the module itself), and all verification steps use cargo nextest run. The fixture corpus design is comprehensive, covering degenerate NURBS cases, problematic rail/section combos, and glyph-like profiles as required by TEST-01. Task sizing is appropriate (each task is a substantial but bounded unit of work). Cross-plan coherence is correct: 10-1 provides fixtures, 10-2 provides healing hooks, and 10-3 integrates them.
