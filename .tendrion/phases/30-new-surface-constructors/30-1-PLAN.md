---
phase: 30-new-surface-constructors
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/lib.rs
  - monstertruck-modeling/tests/surface_constructors.rs
autonomous: true
must_haves:
  truths:
    - "User calls builder::try_ruled_surface with two BsplineCurve boundaries and receives a Face with 4-edge boundary wire"
    - "User evaluates the ruled surface at v=0 and v=1 and gets points matching the two input boundary curves"
    - "User evaluates the ruled surface at v=0.5 and gets the linear midpoint between the two boundary curves"
    - "User passes empty or zero-control-point curves and receives a descriptive error instead of a panic"
    - "User passes curves with different degrees and the ruled surface constructor normalizes them via syncro_degree/syncro_knots"
    - "RuledSurfaceOptions is re-exported from monstertruck-modeling for user convenience"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/surface_options.rs"
      provides: "RuledSurfaceOptions struct with direction control"
      min_lines: 130
      contains: "RuledSurfaceOptions"
    - path: "monstertruck-modeling/src/builder.rs"
      provides: "try_ruled_surface builder function with input validation"
      min_lines: 1100
      contains: "try_ruled_surface"
    - path: "monstertruck-modeling/tests/surface_constructors.rs"
      provides: "Tests for ruled surface construction including error cases"
      min_lines: 60
      contains: "try_ruled_surface"
  key_links:
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "BsplineSurface::try_ruled calls homotopy after validation"
      pattern: "try_ruled"
    - from: "monstertruck-modeling/src/lib.rs"
      to: "monstertruck-geometry/src/nurbs/surface_options.rs"
      via: "Re-export of RuledSurfaceOptions"
      pattern: "RuledSurfaceOptions"
---

<objective>
Implement a ruled surface constructor that creates a linearly-interpolated surface between two boundary curves, with proper input validation to prevent panics on degenerate inputs.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs (homotopy method, lines 1518-1535)
@monstertruck-geometry/src/nurbs/compat.rs (make_curves_compatible)
@monstertruck-geometry/src/nurbs/surface_options.rs (existing options pattern)
@monstertruck-modeling/src/builder.rs (existing try_ builder functions)
@monstertruck-modeling/src/errors.rs (Error variants including FromGeometry, SurfaceConstructionFailed)
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add RuledSurfaceOptions and try_ruled geometry method with input validation</name>
  <files>monstertruck-geometry/src/nurbs/surface_options.rs, monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
1. In `surface_options.rs`, add a `RuledSurfaceOptions` struct following the existing pattern (with `#[non_exhaustive]`, `Default` impl):

```rust
/// Options for ruled surface construction between two boundary curves.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::RuledSurfaceOptions;
/// let opts = RuledSurfaceOptions::default();
/// ```
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct RuledSurfaceOptions {}
```

2. In `bspline_surface.rs`, add a `try_ruled` method that wraps the existing `homotopy` with input validation. Place it near the `homotopy` method (around line 1518). Critical: validate BEFORE calling syncro_degree/syncro_knots because those methods panic on empty/degenerate curves.

```rust
/// Fallible ruled surface construction between two boundary curves.
///
/// Creates a linearly-interpolated surface where v=0 matches `curve0`
/// and v=1 matches `curve1`. The two curves are degree-synchronized
/// and knot-synchronized before interpolation.
///
/// # Errors
///
/// Returns [`Error::EmptyControlPoints`] if either curve has no control points.
/// Returns [`Error::CurveNetworkIncompatible`] if compatibility normalization fails.
#[allow(unused_variables)]
pub fn try_ruled(
    mut curve0: BsplineCurve<P>,
    mut curve1: BsplineCurve<P>,
    options: &RuledSurfaceOptions,
) -> Result<BsplineSurface<P>> {
    // Input validation BEFORE syncro calls to prevent panics.
    if curve0.control_points().is_empty() || curve1.control_points().is_empty() {
        return Err(Error::EmptyControlPoints);
    }
    // syncro_degree and syncro_knots are infallible once curves have control points.
    curve0.syncro_degree(&mut curve1);
    curve0.syncro_knots(&mut curve1);

    let knot_vector_u = curve0.knot_vec().clone();
    let knot_vector_v = KnotVector::from(vec![0.0, 0.0, 1.0, 1.0]);
    let control_points: Vec<Vec<_>> = (0..curve0.control_points().len())
        .map(|i| vec![*curve0.control_point(i), *curve1.control_point(i)])
        .collect();
    Ok(BsplineSurface::new_unchecked((knot_vector_u, knot_vector_v), control_points))
}
```

3. Add the import for `RuledSurfaceOptions` in the `use` block at the top of `bspline_surface.rs` where `SkinOptions` is imported from `surface_options`.
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` to confirm the new method and struct compile without errors.</verify>
  <done>RuledSurfaceOptions struct and BsplineSurface::try_ruled method with input validation added to geometry crate.</done>
</task>

<task type="auto">
  <name>Task 2: Add try_ruled_surface builder function and re-exports</name>
  <files>monstertruck-modeling/src/builder.rs, monstertruck-modeling/src/lib.rs</files>
  <action>
1. In `builder.rs`, add `try_ruled_surface` following the pattern of `try_sweep_rail_with_options`. Place it near the other surface construction functions (after `try_skin_wires`).

```rust
/// Constructs a ruled (linearly-interpolated) surface between two boundary curves.
///
/// The resulting face has a 4-edge boundary wire: the two input curves plus
/// two linear edges connecting their endpoints.
///
/// Returns [`Error::FromGeometry`] if the curves are empty or incompatible.
///
/// # Examples
///
/// ```
/// use monstertruck_modeling::*;
/// use monstertruck_modeling::RuledSurfaceOptions;
///
/// let c0 = BsplineCurve::new(
///     KnotVector::bezier_knot(1),
///     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
/// );
/// let c1 = BsplineCurve::new(
///     KnotVector::bezier_knot(1),
///     vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
/// );
/// let face = builder::try_ruled_surface(&c0, &c1, &RuledSurfaceOptions::default()).unwrap();
/// assert_eq!(face.boundaries()[0].len(), 4);
/// ```
pub fn try_ruled_surface(
    curve0: &BsplineCurve<Point3>,
    curve1: &BsplineCurve<Point3>,
    options: &RuledSurfaceOptions,
) -> Result<Face<Curve, Surface>> {
    let surface = BsplineSurface::try_ruled(curve0.clone(), curve1.clone(), options)?;
    let bnd = surface.splitted_boundary();
    let wire = wire_from_bspline_boundary(bnd);
    let surface: Surface = surface.into();
    Ok(Face::new(vec![wire], surface))
}
```

The helper `wire_from_bspline_boundary` follows the same pattern used in `try_sweep_rail_with_options` and `try_gordon_with_options` to create a Wire from the surface boundary curves. Look at how `try_sweep_rail_with_options` builds its Face -- it calls `surface.splitted_boundary()` to get the 4 boundary BsplineCurves, then builds edges from them. Replicate that exact pattern.

If `wire_from_bspline_boundary` doesn't exist as a helper, inline the boundary extraction following the existing pattern in one of the `try_*_with_options` functions. Look specifically at how `try_birail_with_options` or `try_gordon_with_options` constructs `Face` from a `BsplineSurface`.

2. In `lib.rs`, add `RuledSurfaceOptions` to the existing `pub use monstertruck_geometry::nurbs::surface_options::{...}` re-export line (line ~102-104).
  </action>
  <verify>Run `cargo check -p monstertruck-modeling` and `cargo doc -p monstertruck-modeling --no-deps` to verify compilation and doc examples.</verify>
  <done>try_ruled_surface builder function added to monstertruck-modeling with RuledSurfaceOptions re-exported.</done>
</task>

<task type="auto">
  <name>Task 3: Write TDD tests for ruled surface including error cases</name>
  <files>monstertruck-modeling/tests/surface_constructors.rs</files>
  <action>
Create a new test file `monstertruck-modeling/tests/surface_constructors.rs` with tests covering:

1. **Happy path**: Two parallel linear curves produce a planar ruled surface. Verify:
   - Face has 4-edge boundary wire
   - Surface at v=0 matches curve0 (sample at u=0, u=0.5, u=1)
   - Surface at v=1 matches curve1
   - Surface at v=0.5 is midpoint between curves

2. **Different-degree curves**: One linear, one quadratic curve. Verify:
   - Construction succeeds (syncro_degree normalizes)
   - Face has 4-edge boundary wire
   - Boundary interpolation is correct at endpoints

3. **Empty curve error**: Pass a curve with empty control points. Verify:
   - Returns `Err` (match against `Error::FromGeometry(...)`)
   - Does NOT panic

4. **Single-point degenerate curve**: Pass a curve with a single control point (degree 0). Verify:
   - Either returns `Err` or produces a degenerate surface (no panic)

Use the test imports pattern from `builder_roundtrip.rs`:
```rust
use monstertruck_modeling::errors::Error;
use monstertruck_modeling::*;
```

Use `assert_near!` macro for geometric comparisons. The crate re-exports `assert_near` from `monstertruck_core`.
  </action>
  <verify>Run `cargo test -p monstertruck-modeling --test surface_constructors` to verify all tests pass.</verify>
  <done>TDD tests for ruled surface construction written and passing, covering happy paths and error cases.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-geometry` passes
2. `cargo check -p monstertruck-modeling` passes
3. `cargo test -p monstertruck-modeling --test surface_constructors` passes with all ruled surface tests green
4. `cargo doc -p monstertruck-modeling --no-deps` succeeds with no warnings on new items
5. Empty/degenerate curve inputs return errors, not panics
6. RuledSurfaceOptions is accessible from `monstertruck_modeling::RuledSurfaceOptions`
</verification>

<success_criteria>
- CAD-01 is fully satisfied: ruled surface constructor exists between two boundary curves
- Input validation prevents panics from empty/degenerate curves (review blocker B1 fixed)
- syncro_degree/syncro_knots API usage is correct (review suggestion S1 addressed)
- Error propagation uses FromGeometry variant (review suggestion S2 addressed)
- Constructor follows existing builder pattern (try_ prefix, options struct, Face return)
</success_criteria>

<output>
After completion, create `.tendrion/phases/30-new-surface-constructors/30-1-SUMMARY.md`
</output>
