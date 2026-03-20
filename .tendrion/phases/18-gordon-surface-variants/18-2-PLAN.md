---
phase: 18-gordon-surface-variants
plan: 2
type: execute
wave: 2
depends_on: ["18-1"]
files_modified:
  - monstertruck-modeling/src/builder.rs
  - monstertruck-geometry/tests/gordon_variants_test.rs
  - monstertruck-modeling/tests/surface_constructors.rs
autonomous: true
must_haves:
  truths:
    - "User calls builder::try_gordon_from_network with u/v curves and gets a Face without manually computing intersection points"
    - "User calls builder::try_gordon_verified with pre-computed grid points and gets validation feedback before surface construction"
    - "Gordon-specific test fixtures with crossing line networks exercise try_gordon_from_network end-to-end"
    - "Gordon-specific test fixtures with near-miss grid points exercise try_gordon_verified snapping behavior"
    - "Gordon-specific test fixtures with nonuniform spacing verify both variants handle irregular curve networks"
    - "Error cases (missing intersections, out-of-tolerance points) are tested and produce correct diagnostics"
  artifacts:
    - path: "monstertruck-modeling/src/builder.rs"
      provides: "builder::try_gordon_from_network and builder::try_gordon_verified wrapper functions"
      min_lines: 1400
      contains: "try_gordon_from_network"
    - path: "monstertruck-geometry/tests/gordon_variants_test.rs"
      provides: "Geometry-level tests for both Gordon variants including edge cases"
      min_lines: 100
      contains: "try_gordon_from_network"
    - path: "monstertruck-modeling/tests/surface_constructors.rs"
      provides: "Builder-level tests for both Gordon variants"
      min_lines: 200
      contains: "try_gordon_from_network"
  key_links:
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "BsplineSurface::try_gordon_from_network call"
      pattern: "BsplineSurface::try_gordon_from_network"
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "BsplineSurface::try_gordon_verified call"
      pattern: "BsplineSurface::try_gordon_verified"
    - from: "monstertruck-geometry/tests/gordon_variants_test.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "test exercises of geometry-level API"
      pattern: "try_gordon_from_network"
---

<objective>
Add builder-level wrappers for both Gordon variants and comprehensive test coverage including Gordon-specific network fixtures (near-miss grid points, nonuniform spacing, crossing curves) that exercise both new variants end-to-end.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/builder.rs
@monstertruck-geometry/tests/try_gordon_skin_test.rs
@monstertruck-modeling/tests/surface_constructors.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add builder wrappers for both Gordon variants</name>
  <files>monstertruck-modeling/src/builder.rs</files>
  <action>
Add two new public functions to `monstertruck-modeling/src/builder.rs`, following the pattern of the existing `try_gordon_with_options`:

**1. `try_gordon_from_network`:**
```rust
/// Constructs a Gordon surface by auto-computing intersection grid points
/// from the curve network, returning a [`Face`].
///
/// Intersects each u-curve with each v-curve using the curve intersection
/// engine before compatibility normalization.
///
/// # Errors
///
/// Returns [`Error::FromGeometry`] wrapping geometry-level diagnostics.
pub fn try_gordon_from_network(
    u_curves: Vec<BsplineCurve<Point3>>,
    v_curves: Vec<BsplineCurve<Point3>>,
    options: &GordonOptions,
) -> Result<Face<Curve, Surface>> {
    let surface = BsplineSurface::try_gordon_from_network(u_curves, v_curves, options)?;
    Ok(face_from_bspline_surface(surface))
}
```

**2. `try_gordon_verified`:**
```rust
/// Constructs a Gordon surface from caller-supplied grid points after
/// validating that each point lies on both corresponding curves, returning
/// a [`Face`].
///
/// Points within the tolerance are snapped. Points outside tolerance
/// cause an error with diagnostic information.
///
/// # Errors
///
/// Returns [`Error::FromGeometry`] wrapping geometry-level diagnostics.
pub fn try_gordon_verified(
    u_curves: Vec<BsplineCurve<Point3>>,
    v_curves: Vec<BsplineCurve<Point3>>,
    points: &[Vec<Point3>],
    options: &GordonOptions,
) -> Result<Face<Curve, Surface>> {
    let surface = BsplineSurface::try_gordon_verified(u_curves, v_curves, points, options)?;
    Ok(face_from_bspline_surface(surface))
}
```

Place them immediately after the existing `try_gordon_with_options` function. Make sure `GordonOptions` is already imported (it is -- see the existing use statement at line 5-8).
  </action>
  <verify>Run `cargo check -p monstertruck-modeling` to confirm compilation.</verify>
  <done>Builder wrappers try_gordon_from_network and try_gordon_verified added to builder.rs.</done>
</task>

<task type="auto">
  <name>Task 2: Create geometry-level tests for both Gordon variants</name>
  <files>monstertruck-geometry/tests/gordon_variants_test.rs</files>
  <action>
Create a new test file `monstertruck-geometry/tests/gordon_variants_test.rs` with comprehensive tests:

```rust
use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::GordonOptions;
use monstertruck_geometry::prelude::*;
```

**Test 1: `try_gordon_from_network_crossing_lines`**
- Create a 2x2 grid of crossing lines in 3D:
  - u0: (0,0,0) -> (2,0,0) (along x-axis at z=0)
  - u1: (0,0,1) -> (2,0,1) (along x-axis at z=1)
  - v0: (0,0,0) -> (0,0,1) (along z-axis at x=0)  -- Wait, these don't cross. We need crossing curves.

  Actually for a Gordon network, u-curves and v-curves must intersect transversally. Use:
  - u0: (0,0,0) -> (1,0,0) (horizontal line at z=0)
  - u1: (0,0,1) -> (1,0,1) (horizontal line at z=1)
  - v0: (0,0,0) -> (0,0,1) (vertical line at x=0)
  - v1: (1,0,0) -> (1,0,1) (vertical line at x=1)

  Expected intersections: u0∩v0=(0,0,0), u0∩v1=(1,0,0), u1∩v0=(0,0,1), u1∩v1=(1,0,1)
- Call `BsplineSurface::try_gordon_from_network(vec![u0,u1], vec![v0,v1], &GordonOptions::default())`
- Assert success
- Verify corner evaluations match expected points

**Test 2: `try_gordon_from_network_nonuniform_spacing`**
- Create a 3x2 network with nonuniform curve spacing:
  - u0: (0,0,0) -> (1,0,0)
  - u1: (0,0,0.3) -> (1,0,0.3) (closer to u0)
  - u2: (0,0,1) -> (1,0,1)
  - v0: (0,0,0) -> (0,0,1)
  - v1: (1,0,0) -> (1,0,1)
- Assert success

**Test 3: `try_gordon_from_network_no_intersection_error`**
- Create parallel curves that don't intersect:
  - u0: (0,0,0) -> (1,0,0)
  - v0: (0,1,0) -> (0,1,1) (offset in y, no intersection)
- Assert error is `CurveNetworkIncompatible(IntersectionCountMismatch { found: 0, expected: 1, .. })`

**Test 4: `try_gordon_verified_exact_points`**
- Use the same 2x2 network as test 1
- Supply exact intersection points: [(0,0,0),(1,0,0)],[(0,0,1),(1,0,1)]
- Assert success and equivalence with try_gordon_from_network result

**Test 5: `try_gordon_verified_near_miss_snapping`**
- Use the 2x2 network
- Supply grid points that are slightly off (within SNAP_TOLERANCE):
  - e.g., (0,1e-7,0) instead of (0,0,0)
- Assert success (snapping should fix them)

**Test 6: `try_gordon_verified_out_of_tolerance`**
- Use the 2x2 network
- Supply a grid point that's clearly wrong: (0.5, 0.5, 0.5) for position [0][0]
- Assert error is `CurveNetworkIncompatible(GridPointNotOnCurve { row: 0, col: 0, .. })`

**Test 7: `try_gordon_from_network_empty_curves`**
- Empty u_curves or v_curves
- Assert InsufficientCurves error

**Test 8: `try_gordon_verified_equivalence`**
- Construct surface via try_gordon_from_network and try_gordon_verified with the auto-computed points
- Verify both surfaces evaluate to the same values at a grid of sample parameters
  </action>
  <verify>Run `cargo test -p monstertruck-geometry --test gordon_variants_test` to confirm all tests pass.</verify>
  <done>Comprehensive geometry-level tests created covering both Gordon variants including crossing lines, nonuniform spacing, near-miss snapping, out-of-tolerance rejection, and equivalence verification.</done>
</task>

<task type="auto">
  <name>Task 3: Add builder-level tests for both Gordon variants</name>
  <files>monstertruck-modeling/tests/surface_constructors.rs</files>
  <action>
Add new test functions to the existing `monstertruck-modeling/tests/surface_constructors.rs` file (or add a new test module within builder.rs tests). If surface_constructors.rs exists and has the right imports, add tests there. Otherwise add to the existing test module in builder.rs.

Add these tests in the builder test module (the existing `mod options_api` or similar):

**Test 1: `gordon_from_network_builder_success`**
- Create 2x2 crossing-line network in Point3
- Call `builder::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default())`
- Assert success, verify Face has 4 boundary edges

**Test 2: `gordon_verified_builder_success`**
- Create 2x2 crossing-line network with exact points
- Call `builder::try_gordon_verified(u_curves, v_curves, &points, &GordonOptions::default())`
- Assert success, verify Face has 4 boundary edges

**Test 3: `gordon_from_network_error_propagates`**
- Use parallel non-intersecting curves
- Verify error propagates as `Error::FromGeometry(_)`

**Test 4: `gordon_verified_error_propagates`**
- Use valid curves but bad grid points
- Verify error propagates as `Error::FromGeometry(_)`
  </action>
  <verify>Run `cargo test -p monstertruck-modeling` to confirm all tests pass.</verify>
  <done>Builder-level tests added for both Gordon variant wrappers covering success and error propagation cases.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-geometry --test gordon_variants_test` -- all 8 tests pass
2. `cargo test -p monstertruck-modeling` -- all existing and new tests pass
3. `cargo test --workspace` -- no regressions anywhere
4. Builder wrappers return Face objects with correct boundary topology
5. Near-miss snapping test confirms points within SNAP_TOLERANCE are accepted
6. Out-of-tolerance test confirms points beyond tolerance are rejected with GridPointNotOnCurve diagnostic
7. Nonuniform spacing fixture exercises non-trivial curve networks
</verification>

<success_criteria>
- Builder-level API exposes try_gordon_from_network and try_gordon_verified (both requirements surfaced at modeling layer)
- Gordon-specific network fixtures exercise near-miss grid points, nonuniform spacing, and crossing curves
- Both variants produce surfaces equivalent to manual try_gordon calls with correctly computed grid points
- Error diagnostics propagate correctly through the FromGeometry wrapper
</success_criteria>

<output>
After completion, create `.tendrion/phases/18-gordon-surface-variants/18-2-SUMMARY.md`
</output>
