---
target: "18-2"
type: planning
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-20
verdict: PASS
---

# Review: Plan 18-2 (Builder Wrappers and Test Coverage)

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** Planning
**Date:** 2026-03-20

## Verdict

**PASS**

No blockers found. The plan correctly adds builder-level wrappers following the existing `try_gordon_with_options` pattern and provides comprehensive test coverage for both Gordon variants at geometry and builder levels. The wave-2 dependency on 18-1 is correct since 18-1 implements the geometry-level methods that these wrappers delegate to.

## Findings

### Blockers

None

### Suggestions

#### S1: Test 3 error pattern may not match actual error structure [confidence: 72]
- **Confidence:** 72
- **File:** 18-2-PLAN.md, Task 2 Test 3
- **Issue:** The plan asserts the error is `CurveNetworkIncompatible(IntersectionCountMismatch { found: 0, expected: 1, .. })` but does not account for the possibility that parallel non-intersecting curves in 3D (u0 along x-axis, v0 offset in y) may still have overlapping bounding boxes, potentially causing the intersection engine to converge on a near-miss rather than returning zero intersections. The exact error variant returned depends on the intersection engine's behavior with non-intersecting but bounding-box-adjacent curves.
- **Impact:** Test may need adjustment based on actual intersection engine behavior with the specific curve geometry chosen.
- **Suggested fix:** Add a note in the task that the implementer should verify the actual error variant returned and adjust the assertion accordingly, or choose curves that are more clearly separated (e.g., large y-offset).

#### S2: Task 2 action text contains visible "Wait, these don't cross" self-correction [confidence: 91]
- **Confidence:** 91
- **File:** 18-2-PLAN.md, Task 2 lines 138-140
- **Issue:** The action text includes the planner's stream-of-consciousness self-correction: "-- Wait, these don't cross. We need crossing curves." followed by "Actually for a Gordon network...". This drafting artifact may confuse the implementer about which curve setup to use.
- **Impact:** Minor confusion risk. The final curve definitions are correct, but the crossed-out first attempt is still present.
- **Suggested fix:** Clean up the action text to only show the final correct curve definitions.

#### S3: Task 3 action has ambiguous file placement instructions [confidence: 84]
- **Confidence:** 84
- **File:** 18-2-PLAN.md, Task 3 action
- **Issue:** The action says "Add new test functions to the existing `monstertruck-modeling/tests/surface_constructors.rs` file (or add a new test module within builder.rs tests). If surface_constructors.rs exists and has the right imports, add tests there. Otherwise add to the existing test module in builder.rs." The file `surface_constructors.rs` exists (confirmed), and `files_modified` lists it, but the hedging language creates unnecessary ambiguity. The implementer should be told definitively where to put the tests.
- **Impact:** Could cause the implementer to waste time deciding placement or put tests in the wrong file.
- **Suggested fix:** Remove the conditional language. State directly: "Add test functions to `monstertruck-modeling/tests/surface_constructors.rs`."

### Nits

#### N1: Duplicate closing output tag [confidence: 95]
- **Confidence:** 95
- **File:** 18-2-PLAN.md, line 246
- **Issue:** There are two `</output>` closing tags at the end of the file (lines 245-246). The second one is extraneous.

#### N2: Test 8 partially overlaps with Test 4 [confidence: 82]
- **Confidence:** 82
- **File:** 18-2-PLAN.md, Task 2 Tests 4 and 8
- **Issue:** Test 4 (`try_gordon_verified_exact_points`) already asserts "equivalence with try_gordon_from_network result" and Test 8 (`try_gordon_verified_equivalence`) verifies "both surfaces evaluate to the same values at a grid of sample parameters." These cover similar ground. Test 8 is more rigorous (sample-point comparison vs. simple success assertion) but the overlap could be consolidated.

## Summary

Plan 18-2 is well-structured with correct wave ordering (depends on 18-1 which implements the geometry-level methods). The builder wrappers follow the established `try_gordon_with_options` pattern exactly. Test coverage is comprehensive, spanning geometry-level and builder-level tests with good variety (success paths, snapping, error cases, equivalence checks). The three suggestions are about clarity improvements that would help the implementer but do not affect correctness. The plan adequately covers both GORDON-01 and GORDON-02 requirements at the builder and test layers, complementing 18-1's geometry-level implementation.
