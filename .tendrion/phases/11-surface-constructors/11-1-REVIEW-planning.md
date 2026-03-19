---
target: "11-1"
type: planning
round: 2
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-19
verdict: PASS
---

# Planning Review: Plan 11-1

**Reviewer:** claude-opus-4-6
**Round:** 2 of 3
**Stage:** Planning
**Date:** 2026-03-19

## Verdict

**PASS** -- Both round 1 blockers have been resolved. No new blockers found.

- **B1 (SweepBuilder naming):** Resolved. The plan now includes an explicit NOTE in the objective (line 42) explaining the deviation from `SweepBuilder::*` naming and mapping it to the actual API names (`BsplineSurface::sweep_multi_rail` at geometry level, `builder::try_sweep_multi_rail` / `builder::try_sweep_periodic` in Plan 11-2). Plan 11-2 carries a matching note. This is a clear, justified deviation from the roadmap's literal naming.
- **B2 (Euler-Poincare unassigned):** Resolved. The plan now includes a NOTE (line 44) explicitly deferring Euler-Poincare topology checks to Plan 11-2. Plan 11-2's must_haves include `is_geometric_consistent` validation, and its Task 4 (line 312) calls `shell.is_geometric_consistent()` with an explicit comment that this verifies Euler-Poincare checks.
- **S1 (Affine fitting degeneracy):** Resolved. The plan now specifies singular matrix detection (lines 89-92) with a determinant check against TOLERANCE and returns `Err` for degenerate configurations. Must_haves truths #5 and #6 explicitly require this. Test #4 covers the collinear case.
- **S3 (sweep_periodic algorithm ambiguity):** Resolved. The plan commits to a single approach -- the duplicated-endpoint method (line 119: "this is the committed design"). The algorithm is clearly specified in 5 steps with no alternative approaches muddying the description.

## Findings

### Blockers

None

### Suggestions

#### S1: birail1 equivalence test needs tighter geometry specification [confidence: 78]
- **Confidence:** 78
- **File:** 11-1-PLAN.md, Task 3, test_sweep_multi_rail_matches_birail1_for_two_rails (line 151)
- **Issue:** The test compares `sweep_multi_rail` with 2 rails against `birail1`. These use different algorithms (affine fitting vs. chord-based scale+rotate). Equivalence only holds when profile endpoints coincide with rail start points. The plan does not specify the exact geometry that ensures this algebraic equivalence, leaving the implementer to figure out a valid configuration.
- **Impact:** Implementer may waste time debugging a test that fails due to geometry setup rather than code logic. Alternatively, the test may pass with a generous tolerance but not actually verify algorithmic correctness.
- **Suggested fix:** Add a note specifying: "Use a profile whose start point equals rail1.start and end point equals rail2.start, with parallel rails." This ensures the two algorithms produce identical results. Carried forward from round 1 S2.

#### S2: Duplicate closing tag in plan footer [confidence: 96]
- **Confidence:** 96
- **File:** 11-1-PLAN.md, lines 189-190
- **Issue:** The plan has nested `</output>` tags at the end. Line 189 has the intended `</output>` closing the output section, but line 190 has a stray duplicate `</output>`. This is a minor XML/template error that does not affect parsing but could confuse tooling.
- **Impact:** Minimal. Carried forward from round 1 N1, upgraded to suggestion since it persisted through revision.

### Nits

#### N1: Error type uses &'static str instead of typed enum [confidence: 68]
- **Confidence:** 68
- **File:** 11-1-PLAN.md, Task 1 and Task 2 signatures
- **Issue:** The geometry-level methods return `Result<BsplineSurface<Point3>, &'static str>`. Plan 11-2 wraps these with typed error variants (`SurfaceConstructionFailed { reason: String }`). Using `&'static str` at the geometry level means the builder wrapper must do string-based error conversion. A small error enum at the geometry level would allow Plan 11-2 to pattern-match on failure reasons rather than wrapping opaque strings. However, this matches the existing codebase pattern where geometry-level methods use simpler error types and the modeling layer adds structure. Carried forward from round 1 N2.

## Summary

Plan 11-1 has addressed all round 1 blockers and suggestions effectively. The naming deviation from the roadmap is now explicitly documented with clear justification. The Euler-Poincare responsibility is explicitly deferred to Plan 11-2 with verifiable coverage. The sweep_periodic algorithm is clearly specified as the duplicated-endpoint approach. The affine fitting includes degenerate-case handling. The plan is ready for execution. Two minor suggestions remain: tightening the birail1 equivalence test geometry and fixing the duplicate closing tag.
