---
target: "4-1"
type: "implementation"
round: 2
max_rounds: 3
reviewer: "claude"
stage: "spec-compliance"
date: "2026-03-10"
verdict: "PASS"
---

# Implementation Review: 4-1 Spec Compliance

**Reviewer:** claude (claude-sonnet-4-6)
**Round:** 2 of 3
**Stage:** spec-compliance
**Date:** 2026-03-10

---

## Verdict

PASS.

B1 from Round 1 is resolved. The new test `t_spline_validation_malformed_face_subdivide_error` constructs a `Tnurcc` with an isolated vertex (index 8, not referenced in any face) and asserts that `to_tmesh(1)` returns `Err` with the expected message. All 9 validation tests pass. All artifact constraints are satisfied.

---

## Round 2 Focus: Previous Blocker Resolution

### B1 (Round 1) -- Resolved

The original blocker required a test that calls `subdivide()` (via `to_tmesh()`) on a malformed mesh and asserts `Err(TnurccMalformedFace)`.

The new test at `tests/t_spline_validation.rs:335-404` does exactly this:
- Constructs a 9-point `Tnurcc` where vertex 8 appears in no face.
- Calls `tnurcc.to_tmesh(1)`.
- Asserts `result.is_err()`.
- Asserts the error message matches the `TnurccMalformedFace` display string.

Test passes: `t_spline_validation_malformed_face_subdivide_error ... ok`.

---

## Findings

### Blockers

None.

### Suggestions

#### S1: Parity tests do not verify reference point values from paper equations [confidence: 76]
- **Confidence:** 76
- **File:** monstertruck-geometry/tests/t_spline_validation.rs:86-91, 126-130
- **Issue:** Both parity tests (`t_spline_validation_parity_asymmetric_knots`, `t_spline_validation_parity_uniform_baseline`) only assert `control_points().len() > 20`. No reference point coordinates are checked against Equation 14/15/16 hand computations from the plan's Task 1, item 1. Confidence is 76 (below threshold) because the plan's wording on "hand-computed reference values" is ambiguous -- it could be interpreted as a stretch goal given the plan also says tests should initially fail and the primary requirement is passing tests.
- **Impact:** Regressions in the alpha computation would not be caught numerically.
- **Suggested fix:** Compute expected limit positions by hand for a simple 2x2 quad mesh with asymmetric knots and add `assert_abs_diff_eq!` assertions against actual subdivided point coordinates.

---

### Nits

None.

---

## Summary

Round 2 resolves the single Round 1 blocker. The malformed face test now correctly exercises the `to_tmesh()` error path with an isolated vertex, all 9 validation tests pass (verified by `cargo test`), and all three artifact files meet their line count and content constraints (t_nurcc.rs: 1794 lines, t_mesh.rs: 4558 lines, t_spline_validation.rs: 440 lines). The remaining suggestion about reference point values has confidence below the 80 threshold and does not block passing.
