---
target: 13-1
type: impl-review
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: code-quality
date: 2026-03-19
verdict: FAIL
---

## Verdict

**FAIL** due to S1 (code duplication creating maintainability risk).

While the implementation is clean, well-documented, and all 36 tests pass with clean clippy, the deprecated methods duplicate the entire body of the try_* methods rather than delegating to them. This creates a maintenance burden where bug fixes or algorithm improvements must be applied in two places. This is exactly the kind of issue that leads to divergent behavior over time.

## Findings

### Blockers

None

### Suggestions

#### S1: Deprecated methods should delegate to try_* methods instead of duplicating logic [confidence: 93]
- **Confidence:** 93
- **File:** monstertruck-geometry/src/nurbs/bspline_surface.rs (lines 1623-1659 vs 1884-1934, lines 1705-1752 vs 1946-2005, lines 1801-1876 vs 2020-2115, lines 2513-2577 vs 2592-2690, lines 1482-1518 vs 1530-1587)
- **Issue:** Each deprecated method (`sweep_rail`, `birail1`, `birail2`, `gordon`, `skin`) contains a full copy of the algorithm logic rather than delegating to the corresponding `try_*` method with `.unwrap()` or `.expect()`. This is approximately 300 lines of duplicated logic across 5 method pairs.
- **Impact:** Any future bug fix or algorithm improvement must be applied in two places. The deprecated `birail1` has subtly different degenerate-chord behavior (fallback to identity) compared to `try_birail1` (returns error), demonstrating that the implementations have already diverged in behavior. This divergence will compound over time.
- **Suggested fix:** Rewrite each deprecated method to delegate:
  ```rust
  #[deprecated(since = "0.5.0", note = "use try_sweep_rail with SweepRailOptions")]
  pub fn sweep_rail(profile: BsplineCurve<Point3>, rail: &BsplineCurve<Point3>, n_sections: usize) -> BsplineSurface<Point3> {
      Self::try_sweep_rail(profile, rail, &SweepRailOptions { n_sections, ..Default::default() })
          .expect("sweep_rail failed")
  }
  ```
  Note: `birail1` requires care since `try_birail1` has stricter validation (endpoint mismatch, degenerate chord). The deprecated wrapper could either use `.unwrap()` to match the new stricter behavior, or the additional checks in `try_birail1` could be separated so the deprecated method preserves its old permissive behavior while sharing the core transform logic.

#### S2: GridDimensionMismatch actual_cols reports first row length only [confidence: 86]
- **Confidence:** 86
- **File:** monstertruck-geometry/src/nurbs/bspline_surface.rs:2623
- **Issue:** When `points.iter().any(|row| row.len() != m)` triggers because a non-first row has the wrong length, `actual_cols: points.first().map_or(0, |r| r.len())` reports the first row's column count, which may be correct. The diagnostic is misleading -- it would be more useful to report the index and length of the first offending row.
- **Impact:** Debugging grid dimension issues is harder when the error message shows correct dimensions for `actual_cols` but the check still fails because an interior row has wrong length.
- **Suggested fix:** Find the first row with incorrect length and report its index and length, or report `actual_cols` as the length of the row that actually failed the check.

#### S3: FrameRule field is accepted but silently ignored [confidence: 88]
- **Confidence:** 88
- **File:** monstertruck-geometry/src/nurbs/bspline_surface.rs:1884-1934, monstertruck-geometry/src/nurbs/surface_options.rs:1-9
- **Issue:** `SweepRailOptions.frame_rule` is a public field that users can set, but `try_sweep_rail` never reads it. The method always uses tangent-aligned framing regardless of the `frame_rule` value. There is no documentation indicating this field is not yet implemented.
- **Impact:** Users setting `frame_rule: FrameRule::FixedUp` will get tangent-aligned framing with no indication their setting was ignored. This is a silent correctness issue from the user's perspective.
- **Suggested fix:** Either (a) add a `// TODO` comment and document in the struct/method doc that `FrameRule::FixedUp` is not yet implemented and will behave as `TangentAligned`, or (b) return an error when `FrameRule::FixedUp` is requested, or (c) remove `FrameRule` from the struct until it is implemented. Option (a) is the least disruptive.

### Nits

#### N1: Birail1Options and Birail2Options are structurally identical [confidence: 71]
- **Confidence:** 71
- **File:** monstertruck-geometry/src/nurbs/surface_options.rs:29-53
- **Issue:** Both structs contain only `pub n_sections: usize` with identical Default impls. They could share a common `BirailOptions` type alias or be unified. However, keeping them separate provides forward-compatibility for divergent fields.

#### N2: Doc comment has broken intra-doc link syntax [confidence: 82]
- **Confidence:** 82
- **File:** monstertruck-geometry/src/nurbs/bspline_surface.rs:1527, 2589
- **Issue:** Lines 1527 and 2589 have `/ [` instead of `/// [` -- the doc comment continuation is missing a `/`. While this compiles (it's treated as a regular comment), the doc link won't render in rustdoc output.

## Summary

The implementation is well-structured with good documentation, comprehensive test coverage (36 tests covering success paths, error paths, and backward compatibility), clean clippy output, and consistent API patterns across all five surface constructors. The main quality concern is the full duplication of algorithm logic between deprecated methods and their try_* replacements (~300 lines), which creates maintenance risk. The silently ignored `FrameRule` field is also a user-facing quality issue that should be documented or addressed. The misleading `GridDimensionMismatch` diagnostic is a minor but real usability issue.
