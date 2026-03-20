---
phase: 20-fixture-corpus-and-migration-documentation
plan: 3
type: execute
wave: 2
depends_on: ["20-1"]
files_modified:
  - monstertruck-geometry/src/nurbs/test_fixtures.rs
  - monstertruck-geometry/tests/gordon_network_fixtures_test.rs
  - monstertruck-geometry/tests/test_fixtures_smoke.rs
autonomous: true
must_haves:
  truths:
    - "User runs `cargo nextest run -p monstertruck-geometry gordon_network_fixtures` and all tests pass"
    - "Near-miss grid point fixture has points offset from exact intersections by an amount within SNAP_TOLERANCE"
    - "Nonuniform spacing fixture has u-curves and v-curves at irregular intervals"
    - "High-degree curve family fixture uses degree-4 or higher curves"
    - "try_gordon_from_network successfully builds a surface from the nonuniform spacing fixture"
    - "try_gordon_verified correctly handles near-miss grid points within tolerance"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/test_fixtures.rs"
      provides: "Gordon-specific network fixtures for near-miss, nonuniform, and high-degree cases"
      min_lines: 500
      contains: "fixture_gordon_near_miss"
    - path: "monstertruck-geometry/tests/gordon_network_fixtures_test.rs"
      provides: "Integration tests for Gordon-specific network fixtures"
      min_lines: 100
      contains: "try_gordon_from_network"
  key_links:
    - from: "monstertruck-geometry/tests/gordon_network_fixtures_test.rs"
      to: "monstertruck-geometry/src/nurbs/test_fixtures.rs"
      via: "import of Gordon fixture functions"
      pattern: "use monstertruck_geometry::nurbs::test_fixtures"
    - from: "monstertruck-geometry/tests/gordon_network_fixtures_test.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "exercising try_gordon_from_network and try_gordon_verified"
      pattern: "try_gordon_from_network"
---

<objective>
Add Gordon-specific network fixtures (near-miss grid points, nonuniform spacing, high-degree curve families) to the test fixture corpus and create integration tests that exercise them through try_gordon_from_network and try_gordon_verified.
</objective>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/test_fixtures.rs
@monstertruck-geometry/tests/gordon_variants_test.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add Gordon-specific network fixtures to test_fixtures.rs</name>
  <files>monstertruck-geometry/src/nurbs/test_fixtures.rs</files>
  <action>
Add the following fixture functions to `monstertruck-geometry/src/nurbs/test_fixtures.rs` under a new section header:

```
// ---------------------------------------------------------------------------
// Gordon-specific network fixtures (FIXTURE-03)
// ---------------------------------------------------------------------------
```

1. `fixture_gordon_near_miss_grid() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>, Vec<Vec<Point3>>)`: Returns u-curves, v-curves, and grid points where the grid points are perturbed from exact intersection positions by a small amount (half of SNAP_TOLERANCE = 5e-6). Use `monstertruck_core::tolerance_constants::SNAP_TOLERANCE`. Build a 3x3 network of linear curves forming a planar grid, then offset each grid point by `(eps, eps, 0)` where `eps = SNAP_TOLERANCE * 0.5`. This tests the snapping behavior of try_gordon_verified.

2. `fixture_gordon_nonuniform_spacing() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>)`: Returns a 4x3 network (4 u-curves, 3 v-curves) with nonuniform spacing. U-curves at y = 0.0, 0.1, 0.7, 1.0 (clustered near y=0). V-curves at x = 0.0, 0.5, 1.0. All linear curves on a planar grid. This tests try_gordon_from_network with asymmetric curve distributions.

3. `fixture_gordon_high_degree_family() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>)`: Returns a 3x3 network where all curves are degree 4 (quartic). U-curves go along X at different Y values with 5 control points each (providing some curvature). V-curves go along Y at different X values, also degree 4. Use `KnotVector::bezier_knot(4)` for each. This tests Gordon surface construction with high-degree input curves that require compatibility normalization.

4. `fixture_gordon_curved_network() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>)`: Returns a 2x2 network of cubic curves that are genuinely curved (not linear), forming a curved patch. U-curves are parabolic arcs at y=0 and y=1 (with control points creating a bulge in Z). V-curves are parabolic arcs at x=0 and x=1. This tests Gordon surface construction with non-trivial curve geometry.

Add unit tests in the `#[cfg(test)] mod tests` block verifying curve counts, degrees, and control point counts for each new fixture.

Import `SNAP_TOLERANCE` at the top of the file:
```rust
use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry test_fixtures` and verify all new unit tests pass.</verify>
  <done>Four Gordon-specific network fixtures added with passing unit tests.</done>
</task>

<task type="auto">
  <name>Task 2: Add integration tests for Gordon network fixtures</name>
  <files>monstertruck-geometry/tests/gordon_network_fixtures_test.rs, monstertruck-geometry/tests/test_fixtures_smoke.rs</files>
  <action>
Create `monstertruck-geometry/tests/gordon_network_fixtures_test.rs` with integration tests:

```rust
use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
use monstertruck_geometry::nurbs::surface_options::GordonOptions;
use monstertruck_geometry::nurbs::test_fixtures;
use monstertruck_geometry::prelude::*;
```

Tests to add:

1. `gordon_near_miss_grid_snaps_successfully`: Call `fixture_gordon_near_miss_grid()` to get u-curves, v-curves, and perturbed grid points. Call `try_gordon_verified` with default options. Assert `is_ok()` since points are within SNAP_TOLERANCE. Evaluate the resulting surface at corners and verify finite results.

2. `gordon_near_miss_grid_rejects_with_tight_tolerance`: Same fixture but create `GordonOptions` with `grid_tolerance = 1e-10` (much tighter than SNAP_TOLERANCE * 0.5 offset). Assert the result is `Err` with `GridPointNotOnCurve`.

3. `gordon_nonuniform_spacing_from_network`: Call `fixture_gordon_nonuniform_spacing()` to get u-curves and v-curves. Call `try_gordon_from_network` with default options. Assert `is_ok()`. Evaluate at corners (0,0), (1,0), (0,1), (1,1) and verify the surface interpolates the expected points using `assert_near2!`.

4. `gordon_high_degree_family_from_network`: Call `fixture_gordon_high_degree_family()`. Call `try_gordon_from_network` with default options. Assert `is_ok()` (compatibility normalization should handle degree elevation). Evaluate at corners.

5. `gordon_curved_network_from_network`: Call `fixture_gordon_curved_network()`. Call `try_gordon_from_network` with default options. Assert `is_ok()`. Evaluate at corners and verify the surface passes through the curve endpoints.

6. `gordon_curved_network_verified_with_computed_points`: Use `fixture_gordon_curved_network()`, manually compute intersection points (curve endpoints at network corners), and pass to `try_gordon_verified`. Assert `is_ok()` and that the result matches the `try_gordon_from_network` result at sampled parameter values.

Also update `monstertruck-geometry/tests/test_fixtures_smoke.rs` with smoke tests for the new Gordon fixtures (curve counts, degrees, point counts).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry gordon_network_fixtures` and `cargo nextest run -p monstertruck-geometry test_fixtures_smoke` and verify all pass.</verify>
  <done>Integration tests exercise Gordon-specific fixtures through try_gordon_from_network and try_gordon_verified with both success and error cases.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-geometry test_fixtures` passes (unit tests for Gordon fixtures)
2. `cargo nextest run -p monstertruck-geometry gordon_network_fixtures` passes (integration tests)
3. `cargo nextest run -p monstertruck-geometry test_fixtures_smoke` passes (smoke tests for new fixtures)
4. Near-miss grid point fixture exercises snapping behavior in try_gordon_verified
5. Nonuniform spacing and high-degree fixtures exercise try_gordon_from_network
6. All tests pass without panics
</verification>

<success_criteria>
- Gordon-specific network fixtures present: near-miss grid points, nonuniform spacing, high-degree curve families (FIXTURE-03)
- try_gordon_from_network exercised with nonuniform and high-degree fixtures
- try_gordon_verified exercised with near-miss grid point fixtures (both accept and reject cases)
- All tests pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/20-fixture-corpus-and-migration-documentation/20-3-SUMMARY.md`
</output>
