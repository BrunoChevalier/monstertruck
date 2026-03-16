# Review Context: Code Quality - 6-1

## Review Metadata
- **Plan ID:** 6-1
- **Review Type:** code-quality
- **Round:** 3 of 3
- **Commit Range:** `aba7974c6ce9f2178cad57dd3dd2e7199b2ce6bf..513f6275d25577ab3ff9f93dad6031cf2398232b`
- **Embedded Mode:** false

## Commits in Range
```text
87754e76 feat(fillet): dehomogenize seam control points before averaging in fillet_along_wire
ad3884d1 refactor(fillet): extract dehomogenized_average helper for seam control point averaging
757ae8ce test(fillet): strengthen seam tests per review findings B1+S1
f90064c4 test(fillet): enforce exact face count and full seam coverage in fillet_wire_seam_continuity
9001693b test(fillet): add failing test for dehomogenized_average production call and extract open-wire helper
513f6275 feat(fillet): expose dehomogenized_average as pub(super) for direct testing
```

## Plan Content

---
phase: 6-topology-surgery-hardening
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "Seam control points in fillet_along_wire are dehomogenized before averaging, producing correct 3D midpoints"
    - "Averaging two Vector4 control points with different weights no longer produces weight-biased positions"
    - "Fillet along a wire with non-uniform-weight control points produces geometrically correct seam transitions"
    - "All existing fillet tests continue to pass unchanged"
  artifacts:
    - path: "monstertruck-solid/src/fillet/ops.rs"
      provides: "Fixed seam averaging logic using dehomogenize-average-rehomogenize pattern"
      min_lines: 600
      contains: "to_point"
    - path: "monstertruck-solid/src/fillet/tests.rs"
      provides: "Test verifying dehomogenized seam averaging produces correct 3D midpoints"
      min_lines: 1800
      contains: "seam_averaging_dehomogenizes"
  key_links:
    - from: "monstertruck-solid/src/fillet/ops.rs"
      to: "monstertruck-core/src/cgmath_extend_traits.rs"
      via: "Homogeneous::to_point() and Homogeneous::from_point_weight()"
      pattern: "to_point"
---

<objective>
Fix the homogeneous coordinate seam averaging bug in `fillet_along_wire` so that Vector4 control points are dehomogenized before averaging, producing correct 3D midpoints instead of weight-biased positions. This addresses TOPO-02.
</objective>

## Summary Content

---
phase: 6-topology-surgery-hardening
plan: 1
tags: [fillet, seam-averaging, homogeneous-coordinates, TOPO-02]
key-files:
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/tests.rs
decisions: []
metrics:
  tests_added: 2
  tests_passing: 37
  tests_failing_preexisting: 7
  tdd_compliance: strict
---

### What Was Built

- **monstertruck-solid/src/fillet/ops.rs**: Fixed seam averaging in `fillet_along_wire` to dehomogenize Vector4 control points before averaging, then rehomogenize with the average weight. Extracted `dehomogenized_average` helper function to reduce duplication between interior seam and wrap-around seam blocks. Addresses TOPO-02.

- **monstertruck-solid/src/fillet/tests.rs**: Fixed `seam_averaging_dehomogenizes` test to properly assert that naive homogeneous averaging produces incorrect 3D midpoints (negative assertion) and that the dehomogenize-average-rehomogenize pattern produces correct results. Added `fillet_wire_seam_continuity` integration test that builds a 4-face open box, applies `fillet_along_wire`, and verifies the resulting shell geometry is well-formed.

### Task Commits

| SHA | Message |
|-----|---------|
| aba7974c | test(fillet): add failing test for dehomogenized seam averaging |
| 87754e76 | feat(fillet): dehomogenize seam control points before averaging in fillet_along_wire |
| ad3884d1 | refactor(fillet): extract dehomogenized_average helper for seam control point averaging |

## Must-Haves
- Seam control points in fillet_along_wire are dehomogenized before averaging, producing correct 3D midpoints
- Averaging two Vector4 control points with different weights no longer produces weight-biased positions
- Fillet along a wire with non-uniform-weight control points produces geometrically correct seam transitions
- All existing fillet tests continue to pass unchanged

## Confidence Rules
- Blockers SHOULD have confidence >= 85
- Confidence threshold for surfacing: 80
- Findings below 80 are preserved but filtered from verdict

## Previous Review (Round 2)

---
target: "6-1"
type: "implementation"
round: 2
max_rounds: 3
reviewer: "codex"
stage: "code-quality"
date: "2026-03-16"
verdict: "FAIL"
confidence_threshold: 80
---

# Review: Implementation - 6-1

**Reviewer:** codex
**Round:** 2 of 3
**Stage:** code-quality
**Date:** 2026-03-16

## Verdict

**FAIL**

**Rationale:** `B1` remains. The production change in monstertruck-solid/src/fillet/ops.rs is small and readable, but the final test set still does not exercise the weighted regression through the changed production path.

## Findings

### Blockers

#### B1: Weighted seam regression is still only covered as stand-alone math [confidence: 93]
- **Confidence:** 93
- **File:** monstertruck-solid/src/fillet/tests.rs at `f90064c4:2801` and `f90064c4:2893`.
- **Issue:** `seam_averaging_dehomogenizes` proves the midpoint math by constructing synthetic `Vector4` control points, but it never calls the production helper or `fillet_along_wire`. The only end-to-end test, `fillet_wire_seam_continuity`, still builds its faces from `Point3` control points, so it does not force the non-uniform seam weights that differentiate the new averaging logic from the old `(p + q) / 2` path.
- **Impact:** The suite can still pass while the actual regression scenario regresses. That leaves the core production behavior effectively unverified under the condition this fix is meant to handle.
- **Suggested fix:** Add a production-path test that drives `fillet_along_wire` with rational `NurbsSurface<Vector4>` faces carrying different seam weights and then checks seam continuity. At minimum, restore a direct test of the production averaging helper instead of only re-implementing the arithmetic in test code.

### Suggestions

#### S1: The open-wire semi-cube fixture is duplicated across multiple long tests [confidence: 91]
- **Confidence:** 91
- **File:** monstertruck-solid/src/fillet/tests.rs at `f90064c4:1852`, `f90064c4:2242`, and `f90064c4:2893`.
- **Issue:** `variable_radius_open_wire`, `continuity_at_wire_joins`, and `fillet_wire_seam_continuity` all inline the same semi-cube point set, edge construction, plane builder, and `fillet_with_side` preparation.
- **Impact:** This is hard to maintain and makes future topology updates error-prone because the same scaffold has to stay in sync across several 80-100 line tests.
- **Suggested fix:** Extract a helper that returns the prepared open-wire shell and boundary wire, then reuse it across the open-wire fillet tests.

#### S2: `fillet_wire_seam_continuity` only samples the first appended face pair [confidence: 88]
- **Confidence:** 88
- **File:** monstertruck-solid/src/fillet/tests.rs at `f90064c4:3023`.
- **Issue:** The test asserts an exact total of 5 appended fillet faces, but the continuity check only compares `shell[pre_wire_count]` and `shell[pre_wire_count + 1]`. Later adjacent seams are not sampled.
- **Impact:** The test name and comment overstate the coverage, and a regression in a later seam could slip through while this test still passes.
- **Suggested fix:** Collect the appended fillet faces and iterate over all adjacent pairs with `windows(2)`, reusing the broader seam-coverage pattern already present in `continuity_at_wire_joins`.

### Nits

None

## Summary

The implementation change itself is clean: extracting the homogeneous midpoint logic into a helper keeps `fillet_along_wire` readable and localizes the fix. The review still fails on coverage because the final test set does not prove the weighted production path, and the last commit also regressed test maintainability by reintroducing large duplicated fixtures.

## Important Note for Round 3 Reviewer

The last two commits in the range (9001693b, 513f6275) were added AFTER Round 2 to address the Round 2 findings:
1. Commit 9001693b adds a `dehomogenized_average_production` test that directly calls the production `dehomogenized_average` helper with non-uniform weights, and extracts a `build_open_wire_semi_cube()` helper to eliminate fixture duplication.
2. Commit 513f6275 exposes `dehomogenized_average` as `pub(super)` to enable direct testing from the test module.

These commits specifically address:
- B1 (weighted seam regression only covered as stand-alone math) -> now has production-path test
- S1 (open-wire semi-cube fixture duplication) -> now extracted to helper
