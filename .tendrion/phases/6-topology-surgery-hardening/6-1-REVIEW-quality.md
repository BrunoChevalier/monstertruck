---
target: "6-1"
type: "implementation"
round: 1
max_rounds: 3
reviewer: "codex"
stage: "code-quality"
date: "2026-03-16"
verdict: "FAIL"
confidence_threshold: 80
---

# Review: Implementation - 6-1

**Reviewer:** codex  
**Round:** 1 of 3  
**Stage:** code-quality  
**Date:** 2026-03-16

## Verdict

**FAIL**

**Rationale:** `B1` blocks approval. The production fix in `ops.rs` is small and readable, but the new tests do not exercise the unequal-weight regression through the real production path that changed.

## Findings

### Blockers

#### B1: Weighted seam regression is only covered as stand-alone math [confidence: 93]
- **Confidence:** 93
- **File:** `monstertruck-solid/src/fillet/tests.rs:2801` and `monstertruck-solid/src/fillet/tests.rs:2893`
- **Issue:** `seam_averaging_dehomogenizes` reimplements the midpoint arithmetic in test code and never calls `dehomogenized_average` or `fillet_along_wire`. `fillet_wire_seam_continuity` does call `fillet_along_wire`, but it builds its faces from `Point3` control grids, so all seam weights are effectively uniform. That means the changed production path is never exercised under the non-uniform-weight condition this fix is meant to protect.
- **Impact:** The suite can still pass while the actual weighted regression comes back in production code, leaving the core behavior effectively unverified.
- **Suggested fix:** Add a production-path regression that drives `fillet_along_wire` with rational `NurbsSurface<Vector4>` faces carrying different seam weights, or expose/directly test `dehomogenized_average` instead of only reproducing the math in the test.

### Suggestions

#### S1: The open-wire semi-cube fixture is duplicated across multiple long tests [confidence: 91]
- **Confidence:** 91
- **File:** `monstertruck-solid/src/fillet/tests.rs:1852`, `monstertruck-solid/src/fillet/tests.rs:2242`, and `monstertruck-solid/src/fillet/tests.rs:2893`
- **Issue:** `variable_radius_open_wire`, `continuity_at_wire_joins`, and `fillet_wire_seam_continuity` all inline the same semi-cube points, edge construction, plane builder, and `fillet_with_side` preparation.
- **Impact:** This is harder to maintain than necessary and makes future topology changes error-prone because the same setup must stay synchronized in several large tests.
- **Suggested fix:** Extract a helper that returns the prepared open-wire shell and boundary wire, then reuse it across the open-wire fillet tests.

#### S2: `fillet_wire_seam_continuity` samples only the first appended face pair [confidence: 88]
- **Confidence:** 88
- **File:** `monstertruck-solid/src/fillet/tests.rs:3023`
- **Issue:** The test asserts an exact total of five appended fillet faces, but the continuity check only compares `shell[pre_wire_count]` and `shell[pre_wire_count + 1]`. Later adjacent seams are not sampled.
- **Impact:** A regression in a later seam join can slip through while this test still passes, so the coverage is narrower than the test name and comments imply.
- **Suggested fix:** Collect the appended fillet faces and iterate over all adjacent pairs with `windows(2)`, reusing the broader seam-coverage pattern already present in `continuity_at_wire_joins`.

### Nits

None

## Summary

The production change in `monstertruck-solid/src/fillet/ops.rs` is localized and maintainable: extracting the homogeneous midpoint logic into a helper keeps the fillet code readable. The review fails on coverage because the final test set does not prove the unequal-weight regression through the real production path, and the new seam test also introduces avoidable duplication and narrower seam coverage than it claims.

---

## Review Context

- **Plan ID:** 6-1
- **Review Type:** code-quality
- **Round:** 1 of 3
- **Commit Range:** aba7974c6ce9f2178cad57dd3dd2e7199b2ce6bf..f90064c4dbf206ced053a2b63def8eeffee0d5fa

### Commits in Range
```text
f90064c4 test(fillet): enforce exact face count and full seam coverage in fillet_wire_seam_continuity
757ae8ce test(fillet): strengthen seam tests per review findings B1+S1
354f3c8a docs(6-1): complete plan 6-1
1ba98d69 docs(phase-6): plan 6-1 execution summary
ad3884d1 refactor(fillet): extract dehomogenized_average helper for seam control point averaging
87754e76 feat(fillet): dehomogenize seam control points before averaging in fillet_along_wire
```