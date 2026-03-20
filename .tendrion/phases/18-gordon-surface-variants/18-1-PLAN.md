---
phase: 18-gordon-surface-variants
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/surface_diagnostics.rs
  - monstertruck-geometry/src/nurbs/surface_options.rs
autonomous: true
must_haves:
  truths:
    - "User calls try_gordon_from_network with u/v curve families and gets a Gordon surface without needing to manually compute intersection grid points"
    - "try_gordon_from_network intersects each u-curve with each v-curve using find_intersections before compatibility normalization"
    - "User calls try_gordon_verified with pre-computed grid points and gets validation that each point lies on both corresponding curves within tolerance"
    - "try_gordon_verified snaps near-miss points that are within SNAP_TOLERANCE of the curves"
    - "Both variants produce surfaces equivalent to calling try_gordon with correctly computed grid points"
    - "Missing or extra intersections in try_gordon_from_network produce descriptive CurveNetworkDiagnostic errors"
    - "Grid points that fail validation in try_gordon_verified produce descriptive errors with point index and distance"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "try_gordon_from_network and try_gordon_verified methods on BsplineSurface<Point3>"
      min_lines: 2600
      contains: "try_gordon_from_network"
    - path: "monstertruck-geometry/src/nurbs/surface_diagnostics.rs"
      provides: "New diagnostic variants for intersection count mismatch and grid point validation failure"
      min_lines: 120
      contains: "IntersectionCountMismatch"
    - path: "monstertruck-geometry/src/nurbs/surface_options.rs"
      provides: "GordonOptions with tolerance field for grid point validation"
      min_lines: 95
      contains: "grid_tolerance"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/src/nurbs/curve_intersect.rs"
      via: "find_intersections function call"
      pattern: "curve_intersect::find_intersections"
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/src/nurbs/surface_diagnostics.rs"
      via: "error diagnostic variants"
      pattern: "IntersectionCountMismatch"
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-core/src/tolerance_constants.rs"
      via: "SNAP_TOLERANCE for grid point validation"
      pattern: "SNAP_TOLERANCE"
---

<objective>
Implement two new Gordon surface construction variants at the geometry level: `try_gordon_from_network` (auto-intersects curve families to compute grid points) and `try_gordon_verified` (validates caller-supplied grid points against both curve families with snapping). Both delegate to the existing `try_gordon` after computing/validating grid points.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-geometry/src/nurbs/curve_intersect.rs
@monstertruck-geometry/src/nurbs/surface_options.rs
@monstertruck-geometry/src/nurbs/surface_diagnostics.rs
@monstertruck-core/src/tolerance_constants.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Extend GordonOptions and CurveNetworkDiagnostic for new variants</name>
  <files>monstertruck-geometry/src/nurbs/surface_options.rs, monstertruck-geometry/src/nurbs/surface_diagnostics.rs</files>
  <action>
**surface_options.rs** -- Add a `grid_tolerance` field to `GordonOptions`:

```rust
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GordonOptions {
    /// Tolerance for grid point validation in `try_gordon_verified`.
    /// Points within this distance of the expected curve position are snapped.
    /// Defaults to `SNAP_TOLERANCE` from monstertruck-core.
    pub grid_tolerance: f64,
}

impl Default for GordonOptions {
    fn default() -> Self {
        Self {
            grid_tolerance: monstertruck_core::tolerance_constants::SNAP_TOLERANCE,
        }
    }
}
```

**surface_diagnostics.rs** -- Add two new `CurveNetworkDiagnostic` variants:

1. `IntersectionCountMismatch` -- for when the intersection engine finds too few or too many intersections between curve families:
```rust
IntersectionCountMismatch {
    /// Index of the u-curve.
    u_curve_index: usize,
    /// Index of the v-curve.
    v_curve_index: usize,
    /// Number of intersections found.
    found: usize,
    /// Number expected (1 for a well-formed Gordon network).
    expected: usize,
},
```

2. `GridPointNotOnCurve` -- for when a caller-supplied grid point fails validation:
```rust
GridPointNotOnCurve {
    /// Row index (u-curve index) of the failing point.
    row: usize,
    /// Column index (v-curve index) of the failing point.
    col: usize,
    /// Distance from the point to the nearest position on the u-curve.
    u_distance: f64,
    /// Distance from the point to the nearest position on the v-curve.
    v_distance: f64,
    /// Tolerance that was exceeded.
    tolerance: f64,
},
```

Add corresponding `Display` implementations in the existing `fmt::Display for CurveNetworkDiagnostic` match block:
- `IntersectionCountMismatch`: `"intersection count mismatch at u[{u_curve_index}] x v[{v_curve_index}]: found {found}, expected {expected}"`
- `GridPointNotOnCurve`: `"grid point [{row}][{col}] not on curves: u-distance={u_distance:.6}, v-distance={v_distance:.6}, tolerance={tolerance:.6}"`
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` to confirm compilation. Verify both new variants exist in the enum and have Display implementations.</verify>
  <done>GordonOptions has grid_tolerance field with SNAP_TOLERANCE default. CurveNetworkDiagnostic has IntersectionCountMismatch and GridPointNotOnCurve variants with Display implementations.</done>
</task>

<task type="auto">
  <name>Task 2: Implement try_gordon_from_network on BsplineSurface&lt;Point3&gt;</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add `try_gordon_from_network` as a method on `impl BsplineSurface<Point3>` (the existing Point3-specific impl block starting around line 1648). This method:

1. Takes `u_curves: Vec<BsplineCurve<Point3>>`, `v_curves: Vec<BsplineCurve<Point3>>`, `options: &GordonOptions`.
2. Validates that both families are non-empty (reuse existing InsufficientCurves diagnostic).
3. **Before any compatibility normalization**, intersects each u-curve with each v-curve using `curve_intersect::find_intersections`. The numerical ordering constraint is critical: intersection must happen on the original curves, not after reparameterization.
4. For each (i, j) pair, expects exactly 1 intersection. If `find_intersections` returns 0 or >1, return `Error::CurveNetworkIncompatible(IntersectionCountMismatch { u_curve_index: i, v_curve_index: j, found: count, expected: 1 })`.
5. Collects the intersection points into a `Vec<Vec<Point3>>` grid where `points[i][j]` = intersection of u_curves[i] with v_curves[j].
6. Delegates to `Self::try_gordon(u_curves, v_curves, &points, options)`.

Add necessary import at the top of the file:
```rust
use super::curve_intersect;
```

The import for `curve_intersect` may need to go into the existing import block. Check if `use super::*;` is already present -- if so the module is already accessible. If not, add `use super::curve_intersect;`.

Signature:
```rust
/// Constructs a Gordon surface by auto-computing intersection grid points
/// from the curve network.
///
/// Intersects each u-curve with each v-curve using the curve intersection
/// engine **before** compatibility normalization, ensuring numerical
/// accuracy on the original curve parameterizations.
///
/// # Errors
///
/// Returns [`Error::CurveNetworkIncompatible`] with:
/// - [`InsufficientCurves`] if either family is empty
/// - [`IntersectionCountMismatch`] if any curve pair has zero or multiple intersections
/// - Any error from the underlying [`try_gordon`] call
pub fn try_gordon_from_network(
    u_curves: Vec<BsplineCurve<Point3>>,
    v_curves: Vec<BsplineCurve<Point3>>,
    options: &GordonOptions,
) -> Result<BsplineSurface<Point3>> {
```
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` to confirm compilation. Verify the method is accessible on `BsplineSurface<Point3>`.</verify>
  <done>try_gordon_from_network implemented on BsplineSurface&lt;Point3&gt;, auto-computing intersection grid points via curve_intersect::find_intersections before compatibility normalization.</done>
</task>

<task type="auto">
  <name>Task 3: Implement try_gordon_verified on BsplineSurface&lt;Point3&gt;</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add `try_gordon_verified` as a method on `impl BsplineSurface<Point3>` (same Point3-specific impl block). This method:

1. Takes `u_curves: Vec<BsplineCurve<Point3>>`, `v_curves: Vec<BsplineCurve<Point3>>`, `points: &[Vec<Point3>]`, `options: &GordonOptions`.
2. Validates dimensions (same checks as try_gordon).
3. For each grid point `points[i][j]`:
   a. Find the nearest parameter on `u_curves[i]` to the point (use `search_nearest_parameter` or evaluate at a dense set of parameters to find the closest point on the curve). The `SearchNearestParameter` trait is available -- use `u_curves[i].search_nearest_parameter(points[i][j], hint, trials)` where hint can be derived from the curve's parameter range.
   b. Compute `u_distance` = distance from point to nearest point on u_curves[i].
   c. Find the nearest parameter on `v_curves[j]` similarly.
   d. Compute `v_distance` = distance from point to nearest point on v_curves[j].
   e. If both distances are within `options.grid_tolerance`: **snap** -- replace the point with the average of the two nearest curve points (for better numerical consistency).
   f. If either distance exceeds `options.grid_tolerance`: return `Error::CurveNetworkIncompatible(GridPointNotOnCurve { row: i, col: j, u_distance, v_distance, tolerance: options.grid_tolerance })`.
4. Delegates to `Self::try_gordon(u_curves, v_curves, &snapped_points, options)` with the snapped grid.

For nearest-parameter search, use the `SearchNearestParameter<D1>` trait which is implemented for `BsplineCurve`. The method signature is:
```rust
fn search_nearest_parameter<H: Into<SPHint1D>>(&self, pt: Point3, hint: H, trial: usize) -> Option<f64>
```
Use `None` as the hint (triggers presearch) and `100` as the trial count.

Then evaluate `u_curves[i].subs(t)` to get the nearest point and compute the distance.

Signature:
```rust
/// Constructs a Gordon surface from caller-supplied grid points after
/// validating that each point lies on both corresponding curves.
///
/// Points within `options.grid_tolerance` of both curves are snapped to
/// the average of the nearest curve positions for numerical consistency.
///
/// # Errors
///
/// Returns [`Error::CurveNetworkIncompatible`] with:
/// - [`GridDimensionMismatch`] if points dimensions don't match curve counts
/// - [`GridPointNotOnCurve`] if any point exceeds the tolerance on either curve
/// - Any error from the underlying [`try_gordon`] call
pub fn try_gordon_verified(
    u_curves: Vec<BsplineCurve<Point3>>,
    v_curves: Vec<BsplineCurve<Point3>>,
    points: &[Vec<Point3>],
    options: &GordonOptions,
) -> Result<BsplineSurface<Point3>> {
```
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` to confirm compilation. Verify the method is accessible on `BsplineSurface<Point3>`.</verify>
  <done>try_gordon_verified implemented on BsplineSurface&lt;Point3&gt;, validating caller-supplied grid points against both curve families with snapping support.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-geometry` compiles without errors
2. `cargo test -p monstertruck-geometry` passes all existing tests (no regressions)
3. GordonOptions::default().grid_tolerance equals SNAP_TOLERANCE
4. CurveNetworkDiagnostic has IntersectionCountMismatch and GridPointNotOnCurve variants
5. Both new methods exist on BsplineSurface<Point3> impl block
6. try_gordon_from_network calls find_intersections before any compatibility normalization
7. try_gordon_verified snaps points within tolerance and rejects points outside tolerance
</verification>

<success_criteria>
- try_gordon_from_network computes intersection grid points from curve families using the curve intersection engine before compatibility normalization (GORDON-01)
- try_gordon_verified validates caller-supplied grid points lie on both curve families within tolerance and snaps near-miss points (GORDON-02)
- Both methods delegate to try_gordon with correctly computed/validated grid points
- Error diagnostics provide actionable information (curve indices, distances, tolerances)
</success_criteria>

<output>
After completion, create `.tendrion/phases/18-gordon-surface-variants/18-1-SUMMARY.md`
</output>
