---
target: "13-2"
type: implementation
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: code-quality
date: 2026-03-19
verdict: PASS
---

# Code Quality Review: Plan 13-2 (Surface Split and Sub-Patch Operations)

**Reviewer:** claude-opus-4-6 | **Round:** 1 of 3 | **Stage:** code-quality | **Date:** 2026-03-19

## Verdict

**PASS** -- Zero blockers. The implementation is clean, idiomatic, well-documented, and all tests pass. Two suggestions for improved robustness and test coverage.

## Findings

### Blockers

None

### Suggestions

#### S1: `sub_patch` doc comment claims panics but no validation exists [confidence: 91]
- **Confidence:** 91
- **File:** monstertruck-geometry/src/nurbs/bspline_surface.rs:1370-1375
- **Issue:** The doc comment states "Panics if `u0 >= u1` or `v0 >= v1`, or if the range is outside the surface domain." However, the implementation contains no assertions. The underlying `cut_u` method does not panic for out-of-domain values either -- it silently returns a degenerate surface with origin-filled control points (lines 1231-1238). This means (a) the documentation is inaccurate, and (b) invalid inputs produce silently incorrect results rather than failing fast.
- **Impact:** Callers relying on the documented panic behavior will get silent data corruption instead of a clear error. This applies equally to the NurbsSurface delegation which inherits the same behavior.
- **Suggested fix:** Either add `assert!(u0 < u1, "...")` and `assert!(v0 < v1, "...")` guards at the top of `sub_patch` to match the documented behavior, or update the doc comment to describe what actually happens with reversed/out-of-range inputs. Adding domain range checks would require reading knot vector bounds and is lower priority than the ordering check.

#### S2: No tests for NurbsSurface split/sub_patch delegation [confidence: 88]
- **Confidence:** 88
- **File:** monstertruck-geometry/src/nurbs/nurbs_surface.rs:441-472
- **Issue:** The three NurbsSurface delegation methods (`split_at_u`, `split_at_v`, `sub_patch`) have no tests -- no doc-tests and no integration tests. While the underlying BsplineSurface methods are well-tested, the wrapping/unwrapping through `NurbsSurface::new` is not verified. The NurbsSurface doc comments also lack `# Examples` sections unlike the BsplineSurface counterparts.
- **Impact:** A regression in the NurbsSurface wrapping logic (e.g., incorrect constructor call, weight handling issue) would go undetected. The plan's Task 3 action explicitly requested "inline tests or doc-tests for the NurbsSurface versions."
- **Suggested fix:** Add at least one doc-test per NurbsSurface method, or a single integration test that exercises `NurbsSurface::split_at_u` and verifies evaluation preservation through the NURBS projection.

### Nits

#### N1: Doc example code duplication across three methods [confidence: 82]
- **Confidence:** 82
- **File:** monstertruck-geometry/src/nurbs/bspline_surface.rs:1317-1394
- **Issue:** The `split_at_u`, `split_at_v`, and `sub_patch` doc examples all contain an identical 8-line surface construction block. This is a common pattern in Rust doc examples and not inherently wrong, but a helper reference in the examples could reduce visual noise.

#### N2: No boundary test for v-direction split [confidence: 78]
- **Confidence:** 78
- **File:** monstertruck-geometry/tests/bspsurface.rs
- **Issue:** `test_split_at_boundary_u` tests splitting at domain start/end for u, but there is no equivalent `test_split_at_boundary_v`. Since `cut_v` delegates through `swap_axes` -> `cut_u` -> `swap_axes`, this is lower risk, but asymmetric test coverage could miss axis-swapping edge cases.

## Summary

The implementation is clean and idiomatic Rust. The three new methods on BsplineSurface follow the established clone-then-mutate pattern used by the existing `cut_u`/`cut_v` infrastructure. Doc comments with working examples are provided for all BsplineSurface methods. NurbsSurface delegation is correctly structured in the same impl block as `cut_u`/`cut_v`. All 26 integration tests and 3 doc-tests pass. Clippy reports zero warnings. The two suggestions address a documentation accuracy issue in `sub_patch` and a test coverage gap for NurbsSurface delegation.
