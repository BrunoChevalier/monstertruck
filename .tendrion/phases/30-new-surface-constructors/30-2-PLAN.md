---
phase: 30-new-surface-constructors
plan: 2
type: tdd
wave: 2
depends_on: ["30-1"]
files_modified:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/lib.rs
  - monstertruck-modeling/tests/surface_constructors.rs
autonomous: true
must_haves:
  truths:
    - "User calls builder::try_loft with 3+ cross-section BsplineCurves and SkinOptions::default() and receives a Face"
    - "User evaluates the loft surface at section v-parameters and the surface interpolates the section curves"
    - "User passes fewer than 2 curves and receives an InsufficientSections error"
    - "User passes empty curve vec and receives an error, not a panic"
    - "SkinOptions is re-exported from monstertruck-modeling (already present at surface_options re-export line)"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/surface_options.rs"
      provides: "SkinOptions with v_degree field for future interpolation control"
      min_lines: 125
      contains: "v_degree"
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "Updated try_skin with v_degree-aware interpolation using existing BsplineCurve fitting"
      min_lines: 2700
      contains: "v_degree"
    - path: "monstertruck-modeling/src/builder.rs"
      provides: "try_loft builder function delegating to try_skin with validation"
      min_lines: 1120
      contains: "try_loft"
    - path: "monstertruck-modeling/tests/surface_constructors.rs"
      provides: "Tests for loft surface including input validation error cases"
      min_lines: 120
      contains: "try_loft"
  key_links:
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "try_loft calls BsplineSurface::try_skin"
      pattern: "try_skin"
    - from: "monstertruck-modeling/src/lib.rs"
      to: "monstertruck-geometry/src/nurbs/surface_options.rs"
      via: "Re-export of SkinOptions (already exists)"
      pattern: "SkinOptions"
---

<objective>
Implement a loft surface constructor that creates a surface through multiple cross-section profiles with configurable v-direction interpolation, using SkinOptions::default() for construction and adding input validation for <2 curves.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs (try_skin method, lines 1613-1667)
@monstertruck-geometry/src/nurbs/surface_options.rs (SkinOptions - currently empty #[non_exhaustive])
@monstertruck-geometry/src/nurbs/compat.rs (make_curves_compatible)
@monstertruck-modeling/src/builder.rs (try_skin_wires and other try_ patterns)
@monstertruck-modeling/src/errors.rs (InsufficientSections variant)
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add v_degree to SkinOptions and update try_skin for higher-order interpolation</name>
  <files>monstertruck-geometry/src/nurbs/surface_options.rs, monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
1. In `surface_options.rs`, add a `v_degree` field to `SkinOptions`. Since `SkinOptions` is `#[non_exhaustive]`, callers MUST use `SkinOptions::default()` or a builder method -- they cannot construct struct literals from external crates. This is the correct pattern.

```rust
/// Options for skin (loft) surface construction.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::SkinOptions;
/// let mut opts = SkinOptions::default();
/// opts.v_degree = 3;
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SkinOptions {
    /// Polynomial degree in the v-direction (loft direction).
    /// - `1` (default): linear interpolation between sections (degree-1 v-knot vector).
    /// - `2` or `3`: higher-order B-spline interpolation through the section curves.
    /// Clamped to `1..=min(n-1, requested)` where `n` = number of sections.
    pub v_degree: usize,
}

impl Default for SkinOptions {
    fn default() -> Self {
        Self { v_degree: 1 }
    }
}
```

Remove the existing `#[derive(Default)]` since we now have a manual `Default` impl.

2. In `bspline_surface.rs`, update `try_skin` to use `options.v_degree`. The current implementation builds a degree-1 v-direction knot vector. When `v_degree > 1`, use BsplineCurve interpolation through control point columns in the v-direction:

For the v-degree > 1 path, after making curves compatible via `make_curves_compatible`:
- For each column `j` of control points (one point per section curve), fit a B-spline curve of degree `v_degree` through those points using the existing `BsplineCurve::interpolate` method (if available) or construct a clamped uniform knot vector of the appropriate degree.
- The approach: build a `BsplineCurve<P>` through the n points `curves[0..n].control_point(j)` with the requested v_degree. Use the clamped uniform knot vector of degree `v_degree` over `n` interpolation points.

IMPORTANT: Check if `BsplineCurve::interpolate` exists. If not, the simplest correct approach for degree > 1 is:
- Build a clamped uniform knot vector of degree `min(v_degree, n-1)` for `n` section points
- Use the control points directly as the v-direction control points (this gives an approximating surface, which is standard for lofting)
- The knot vector construction: for degree `p` and `n` points, use `KnotVector::from` with `p+1` zeros, then `n-p-1` uniform interior knots, then `p+1` ones

Keep the existing behavior as the `v_degree == 1` path (current code is correct for linear).

The actual effective v_degree should be clamped: `let eff_v_degree = options.v_degree.min(curves.len() - 1).max(1);`
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` and `cargo test -p monstertruck-geometry` to verify compilation and existing skin tests still pass.</verify>
  <done>SkinOptions updated with v_degree field; try_skin updated to support higher-order v-direction interpolation.</done>
</task>

<task type="auto">
  <name>Task 2: Add try_loft builder function with input validation</name>
  <files>monstertruck-modeling/src/builder.rs, monstertruck-modeling/src/lib.rs</files>
  <action>
1. In `builder.rs`, add `try_loft` function. This is the user-facing loft constructor that validates inputs before delegating to `BsplineSurface::try_skin`.

CRITICAL FIX from review: `try_skin` accepts 1 curve (degenerate case), but `try_loft` as a modeling-level API should require >= 2 curves. Add explicit validation BEFORE delegating.

```rust
/// Constructs a loft (skinned) surface through multiple cross-section curves.
///
/// Requires at least 2 section curves. For exactly 2 curves, the result is
/// identical to a ruled surface. For 3+ curves, interpolation is controlled
/// by the v_degree field in SkinOptions.
///
/// # Errors
///
/// Returns [`Error::InsufficientSections`] if fewer than 2 curves are provided.
/// Returns [`Error::FromGeometry`] if compatibility normalization fails.
///
/// # Examples
///
/// ```
/// use monstertruck_modeling::*;
///
/// let c0 = BsplineCurve::new(
///     KnotVector::bezier_knot(1),
///     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
/// );
/// let c1 = BsplineCurve::new(
///     KnotVector::bezier_knot(1),
///     vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
/// );
/// let c2 = BsplineCurve::new(
///     KnotVector::bezier_knot(1),
///     vec![Point3::new(0.0, 2.0, 0.5), Point3::new(1.0, 2.0, 0.5)],
/// );
/// let face = builder::try_loft(
///     vec![c0, c1, c2],
///     &SkinOptions::default(),
/// ).unwrap();
/// assert_eq!(face.boundaries()[0].len(), 4);
/// ```
pub fn try_loft(
    curves: Vec<BsplineCurve<Point3>>,
    options: &SkinOptions,
) -> Result<Face<Curve, Surface>> {
    if curves.len() < 2 {
        return Err(Error::InsufficientSections {
            required: 2,
            got: curves.len(),
        });
    }
    let surface = BsplineSurface::try_skin(curves, options)?;
    let bnd = surface.splitted_boundary();
    let wire = wire_from_bspline_boundary(bnd);  // Use same pattern as try_ruled_surface from Plan 30-1
    let surface: Surface = surface.into();
    Ok(Face::new(vec![wire], surface))
}
```

Use the same Face construction pattern established in Plan 30-1's `try_ruled_surface`. The helper function or inline pattern for building the wire from `splitted_boundary()` should already exist from Plan 30-1.

2. In `lib.rs`, ensure `SkinOptions` is in the re-export. Check the existing line:
```rust
pub use monstertruck_geometry::nurbs::surface_options::{
    Birail1Options, Birail2Options, GordonOptions, SweepRailOptions,
};
```
Add `SkinOptions` to this list if not already present (it is NOT currently re-exported).

Also add `RuledSurfaceOptions` if Plan 30-1 hasn't already added it (it should have, but verify).
  </action>
  <verify>Run `cargo check -p monstertruck-modeling` to verify compilation. Run `cargo doc -p monstertruck-modeling --no-deps` to verify doc examples.</verify>
  <done>try_loft builder function added with <2 curve validation; SkinOptions re-exported from monstertruck-modeling.</done>
</task>

<task type="auto">
  <name>Task 3: Write TDD tests for loft surface including validation and interpolation</name>
  <files>monstertruck-modeling/tests/surface_constructors.rs</files>
  <action>
Append tests to the existing `surface_constructors.rs` test file (created in Plan 30-1).

Tests to add:

1. **Loft with 3 linear curves (v_degree=1)**: Three parallel linear curves at y=0, y=1, y=2.
   - Verify Face has 4-edge boundary wire
   - Verify surface at v=0 matches curve0, v=0.5 matches curve1, v=1.0 matches curve2
   - Use `SkinOptions::default()` (v_degree=1)

2. **Loft with 4 curves (v_degree=3)**: Four section curves at different heights.
   - Construct with `let mut opts = SkinOptions::default(); opts.v_degree = 3;`
   - Verify construction succeeds
   - Verify surface interpolates section curves at their v-parameters

3. **Loft with exactly 2 curves**: Should succeed and match ruled surface behavior.
   - Verify Face has 4-edge boundary wire
   - Verify midpoint interpolation at v=0.5

4. **Fewer than 2 curves error**: Pass 1 curve and empty vec separately.
   - `try_loft(vec![single_curve], &SkinOptions::default())` should return `Err(Error::InsufficientSections { required: 2, got: 1 })`
   - `try_loft(vec![], &SkinOptions::default())` should return `Err(Error::InsufficientSections { required: 2, got: 0 })`

5. **Different-degree section curves**: Mix linear and quadratic curves. Verify construction succeeds (compatibility normalization handles degree differences).

Use `assert_near!` for geometric comparisons. Import `SkinOptions` from `monstertruck_modeling`.
  </action>
  <verify>Run `cargo test -p monstertruck-modeling --test surface_constructors` to verify all loft tests pass alongside ruled surface tests from Plan 30-1.</verify>
  <done>TDD tests for loft surface written and passing, covering happy paths, v_degree control, and input validation errors.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-geometry` passes with updated SkinOptions
2. `cargo test -p monstertruck-geometry` passes (existing skin tests still work)
3. `cargo check -p monstertruck-modeling` passes
4. `cargo test -p monstertruck-modeling --test surface_constructors` passes all loft tests
5. Passing <2 curves returns InsufficientSections error (not panic or wrong error)
6. SkinOptions accessible from `monstertruck_modeling::SkinOptions`
7. v_degree=3 with 4+ curves produces a surface with smooth interpolation
</verification>

<success_criteria>
- CAD-02 is fully satisfied: loft surface through multiple cross-sections with interpolation control
- Input validation for <2 curves uses modeling-level InsufficientSections error (review blocker B1 fixed)
- SkinOptions uses ::default() pattern, not struct literal -- compatible with #[non_exhaustive] (review blocker B2 fixed)
- No nalgebra dependency needed -- uses existing BsplineCurve/knot vector facilities (review suggestion S1 addressed)
- SkinOptions re-exported early enough for all tasks (review suggestion S2 addressed)
</success_criteria>

<output>
After completion, create `.tendrion/phases/30-new-surface-constructors/30-2-SUMMARY.md`
</output>
