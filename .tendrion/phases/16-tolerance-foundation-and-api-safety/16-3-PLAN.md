---
phase: 16-tolerance-foundation-and-api-safety
plan: 3
type: execute
wave: 2
depends_on: ["16-2"]
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
autonomous: true
must_haves:
  truths:
    - "User calls deprecated gordon() and gets identical results to try_gordon() with default options"
    - "User calls deprecated skin() and gets identical results to try_skin() with default options"
    - "User calls deprecated sweep_rail() and gets identical results to try_sweep_rail() with matching options"
    - "User calls deprecated birail1() and gets identical results to try_birail1() with matching options"
    - "User calls deprecated birail2() and gets identical results to try_birail2() with matching options"
    - "Each deprecated method body is under 10 lines (delegation only)"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "Deprecated methods refactored to delegate to try_* variants"
      min_lines: 2500
      contains: "try_gordon"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/tests/try_gordon_skin_test.rs"
      via: "deprecated_gordon_still_works and deprecated_skin_still_works tests"
      pattern: "deprecated_gordon_still_works"
---

<objective>
Refactor all deprecated surface constructor methods (gordon, skin, sweep_rail, birail1, birail2) to delegate to their try_* counterparts instead of maintaining independent algorithm implementations, eliminating ~300 lines of duplicated logic.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-geometry/src/nurbs/surface_options.rs
@monstertruck-geometry/tests/try_gordon_skin_test.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Refactor gordon() and skin() to delegate</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Replace the full implementations of the deprecated `gordon()` and `skin()` methods with thin delegation wrappers.

**skin()** (currently lines ~1582-1620): Replace the full body with:
```rust
#[deprecated(since = "0.5.0", note = "use try_skin with SkinOptions")]
pub fn skin(curves: Vec<BsplineCurve<P>>) -> BsplineSurface<P> {
    Self::try_skin(curves, &SkinOptions::default())
        .expect("skin: construction failed (use try_skin for error handling)")
}
```

**gordon()** (currently lines ~2613-2678): Replace the full body with:
```rust
#[deprecated(since = "0.5.0", note = "use try_gordon with GordonOptions")]
pub fn gordon(
    u_curves: Vec<BsplineCurve<P>>,
    v_curves: Vec<BsplineCurve<P>>,
    points: &[Vec<P>],
) -> BsplineSurface<P> {
    Self::try_gordon(u_curves, v_curves, points, &GordonOptions::default())
        .expect("gordon: construction failed (use try_gordon for error handling)")
}
```

Key considerations:
- The `#[allow(deprecated)]` attribute on `gordon()` can be removed since it was there because `gordon()` called `skin()` (also deprecated). Now it calls `try_gordon()` which is not deprecated.
- The `#[allow(deprecated)]` on `skin()` was there for the same reason -- it can be removed since `try_skin()` is not deprecated.
- Keep the `#[deprecated(...)]` attribute.
- The `.expect()` message should guide users to the try_* variant.
- Verify the old `gordon()` called `skin()` internally (it did, at line 2629 and 2633). The `try_gordon()` calls `try_skin()` internally, so behavior is identical.
- The old `skin()` contained the same algorithm as `try_skin()` but with assert! instead of Result. Delegation via `.expect()` preserves the panicking behavior.

Note that `SkinOptions` and `GordonOptions` are imported at the top of this impl block already (line 5):
```rust
use crate::nurbs::surface_options::{
    Birail1Options, Birail2Options, GordonOptions, SkinOptions, SweepRailOptions,
};
```
  </action>
  <verify>Run `cargo test -p monstertruck-geometry -- deprecated_gordon_still_works deprecated_skin_still_works` to verify the delegation produces identical results.</verify>
  <done>gordon() and skin() refactored to delegate to try_gordon() and try_skin() respectively.</done>
</task>

<task type="auto">
  <name>Task 2: Refactor sweep_rail(), birail1(), birail2() to delegate</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Replace the full implementations of the deprecated `sweep_rail()`, `birail1()`, and `birail2()` methods with thin delegation wrappers.

**sweep_rail()** (currently lines ~1723-1760): Replace the full body with:
```rust
#[deprecated(since = "0.5.0", note = "use try_sweep_rail with SweepRailOptions")]
pub fn sweep_rail(
    profile: BsplineCurve<Point3>,
    rail: &BsplineCurve<Point3>,
    n_sections: usize,
) -> BsplineSurface<Point3> {
    let mut opts = SweepRailOptions::default();
    opts.n_sections = n_sections;
    Self::try_sweep_rail(profile, rail, &opts)
        .expect("sweep_rail: construction failed (use try_sweep_rail for error handling)")
}
```

**birail1()** (currently lines ~1805-1853): Replace the full body with:
```rust
#[deprecated(since = "0.5.0", note = "use try_birail1 with Birail1Options")]
pub fn birail1(
    profile: BsplineCurve<Point3>,
    rail1: &BsplineCurve<Point3>,
    rail2: &BsplineCurve<Point3>,
    n_sections: usize,
) -> BsplineSurface<Point3> {
    let mut opts = Birail1Options::default();
    opts.n_sections = n_sections;
    Self::try_birail1(profile, rail1, rail2, &opts)
        .expect("birail1: construction failed (use try_birail1 for error handling)")
}
```

**birail2()** (currently lines ~1901-1977): Replace the full body with:
```rust
#[deprecated(since = "0.5.0", note = "use try_birail2 with Birail2Options")]
pub fn birail2(
    profile1: BsplineCurve<Point3>,
    profile2: BsplineCurve<Point3>,
    rail1: &BsplineCurve<Point3>,
    rail2: &BsplineCurve<Point3>,
    n_sections: usize,
) -> BsplineSurface<Point3> {
    let mut opts = Birail2Options::default();
    opts.n_sections = n_sections;
    Self::try_birail2(profile1, profile2, rail1, rail2, &opts)
        .expect("birail2: construction failed (use try_birail2 for error handling)")
}
```

Key considerations:
- Remove `#[allow(deprecated)]` from all three methods since they no longer call `skin()` (deprecated). They now call `try_sweep_rail`, `try_birail1`, `try_birail2` respectively (not deprecated).
- Keep the `#[deprecated(...)]` attribute.
- The `n_sections` parameter is forwarded to the options struct.
- **Behavioral difference for sweep_rail**: The old `sweep_rail()` had `assert!(n_sections >= 2)` but did NOT check for zero-length start tangent (it just used identity matrix). The new `try_sweep_rail()` returns `Err` for zero-length start tangent. The `.expect()` will now panic in that case where the old code silently produced a degenerate surface. This is an improvement (fail-fast), and the doc comment on the old method already documented `n_sections >= 2` as a requirement.
- **Behavioral difference for birail1**: The old `birail1()` didn't validate endpoint coincidence or chord degeneracy. `try_birail1()` does. Again, `.expect()` makes this fail-fast, which is acceptable for a deprecated method.
- Preserve the doc comments and examples on the deprecated methods (they serve as documentation for the signatures). Only replace the function body.
  </action>
  <verify>Run `cargo test -p monstertruck-geometry -- sweep_rail birail` to verify existing tests still pass. Run `cargo test -p monstertruck-modeling -- sweep_rail birail` to verify builder-level tests still pass.</verify>
  <done>sweep_rail(), birail1(), birail2() refactored to delegate to try_* variants.</done>
</task>

<task type="auto">
  <name>Task 3: Verify full test suite and check line reduction</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
1. Run the full test suite to verify no regressions:
   - `cargo test -p monstertruck-geometry`
   - `cargo test -p monstertruck-modeling`
   - `cargo test -p monstertruck-solid` (in case any code path exercises deprecated methods)

2. Verify that the deprecated methods are now thin wrappers (each under 10 lines of function body). The refactoring should have eliminated approximately 250-300 lines of duplicated algorithm logic.

3. Verify that `#[allow(deprecated)]` attributes are only present where still needed (e.g., test functions that deliberately call deprecated methods to verify they still work).

4. Check that no other files in the workspace call deprecated `skin()` or `gordon()` directly (other than test files). If found, they should be using `try_skin`/`try_gordon` already. The one known call site is `monstertruck-geometry/src/nurbs/offset.rs` line 246 which already uses `try_skin`.
  </action>
  <verify>All three test commands pass. Grep for `BsplineSurface::skin(` and `BsplineSurface::gordon(` should only find occurrences in deprecated method definitions and test functions.</verify>
  <done>Full test suite passes. Deprecated methods are thin delegation wrappers. No duplicated algorithm logic remains.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-geometry` passes -- all surface constructor tests
2. `cargo test -p monstertruck-modeling` passes -- all builder tests
3. `cargo test -p monstertruck-solid` passes -- no regressions
4. deprecated_gordon_still_works test passes
5. deprecated_skin_still_works test passes
6. Each deprecated method body is under 10 lines (delegation only)
7. No duplicated algorithm logic between deprecated and try_* methods
8. #[allow(deprecated)] only appears on test functions, not on the deprecated methods themselves
</verification>

<success_criteria>
- TOLAPI-03 is fully satisfied: gordon() delegates to try_gordon(), and same pattern applied to all other deprecated constructors
- ~250-300 lines of duplicated algorithm code removed
- All existing tests pass with identical behavior
- Deprecated methods panic with informative messages on error cases (rather than silently producing degenerate geometry)
</success_criteria>

<output>
After completion, create `16-tolerance-foundation-and-api-safety/16-3-SUMMARY.md`
</output>
