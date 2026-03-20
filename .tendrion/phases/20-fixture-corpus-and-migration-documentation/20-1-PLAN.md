---
phase: 20-fixture-corpus-and-migration-documentation
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/test_fixtures.rs
  - monstertruck-geometry/tests/pathological_surface_test.rs
  - monstertruck-geometry/tests/test_fixtures_smoke.rs
autonomous: true
must_haves:
  truths:
    - "User runs `cargo nextest run -p monstertruck-geometry pathological_surface` and all tests pass"
    - "Inflection rail fixture produces a curve with an inflection point and can be used with try_sweep_rail"
    - "Converging rails fixture produces two rails that converge to the same endpoint and can be used with try_birail1"
    - "Degenerate section fixture produces a section curve with near-zero extent"
    - "Near-zero Jacobian surface fixture produces a surface where the Jacobian is near-zero at a boundary"
    - "Near-zero weight NURBS curve fixture produces a rational curve with a weight approaching zero"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/test_fixtures.rs"
      provides: "Expanded fixture corpus with inflection rail, converging rails, degenerate section, near-zero Jacobian surface, near-zero weight curve"
      min_lines: 400
      contains: "fixture_inflection_rail"
    - path: "monstertruck-geometry/tests/pathological_surface_test.rs"
      provides: "Integration tests exercising pathological fixtures with surface constructors"
      min_lines: 80
      contains: "try_sweep_rail"
  key_links:
    - from: "monstertruck-geometry/tests/pathological_surface_test.rs"
      to: "monstertruck-geometry/src/nurbs/test_fixtures.rs"
      via: "import of fixture functions"
      pattern: "use monstertruck_geometry::nurbs::test_fixtures"
---

<objective>
Expand the NURBS test fixture corpus with problematic rail/section combinations (inflection rails, converging rails, degenerate sections) and near-degenerate NURBS cases (near-zero Jacobian, near-zero weight, collapsed control points beyond what exists), then add integration tests that exercise these fixtures through surface constructors.
</objective>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/test_fixtures.rs
@monstertruck-geometry/tests/test_fixtures_smoke.rs
@monstertruck-geometry/tests/try_surface_constructors_test.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add pathological rail/section fixtures to test_fixtures.rs</name>
  <files>monstertruck-geometry/src/nurbs/test_fixtures.rs</files>
  <action>
Add the following fixture functions to `monstertruck-geometry/src/nurbs/test_fixtures.rs` under a new section header:

```
// ---------------------------------------------------------------------------
// Pathological rail/section combinations (FIXTURE-01)
// ---------------------------------------------------------------------------
```

1. `fixture_inflection_rail() -> BsplineCurve<Point3>`: A cubic rail with an inflection point (S-curve shape). Control points should create a curve that changes curvature sign, e.g. going from concave-up to concave-down. This tests sweep_rail framing stability through inflection points.

2. `fixture_converging_rails() -> (BsplineCurve<Point3>, BsplineCurve<Point3>)`: Two cubic rails that start apart and converge to the same endpoint (or nearly so, within 1e-8). This tests birail1 behavior when the profile must shrink to near-zero width.

3. `fixture_degenerate_section() -> BsplineCurve<Point3>`: A cubic section curve where all control points are nearly collinear (within 1e-10 of a line), creating an effectively 1D profile. Tests sweep_rail with a section that has near-zero cross-sectional area.

4. `fixture_cusped_rail() -> BsplineCurve<Point3>`: A cubic rail with a cusp (tangent goes to zero at a point). Control points arranged so the curve has zero tangent magnitude at some interior parameter.

Add corresponding unit tests inside the `#[cfg(test)] mod tests` block that verify the degree, control point count, and basic structural validity of each new fixture.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry test_fixtures` and verify all new unit tests pass.</verify>
  <done>Four new pathological rail/section fixtures added to test_fixtures.rs with passing unit tests.</done>
</task>

<task type="auto">
  <name>Task 2: Add near-degenerate NURBS fixtures to test_fixtures.rs</name>
  <files>monstertruck-geometry/src/nurbs/test_fixtures.rs</files>
  <action>
Add the following fixture functions under a new section header:

```
// ---------------------------------------------------------------------------
// Near-degenerate NURBS cases (FIXTURE-02)
// ---------------------------------------------------------------------------
```

1. `fixture_near_zero_jacobian_surface() -> BsplineSurface<Point3>`: A bi-quadratic surface where control points are arranged so that the surface Jacobian (cross product of partial derivatives) is near-zero along one boundary. For example, three rows of 3 control points where the first row has all points nearly coincident (within 1e-10), creating a pole-like degeneracy.

2. `fixture_near_zero_weight_nurbs() -> NurbsCurve<Vector4>`: A degree-3 NURBS curve (using Vector4 for weighted points) where one control point has a weight approaching zero (e.g., 1e-12). This creates a near-singularity in the rational evaluation. Use the existing `NurbsCurve` type if available, otherwise use `BsplineCurve<Vector4>`.

3. `fixture_collapsed_control_polygon_surface() -> BsplineSurface<Point3>`: A bi-cubic surface where an entire column of control points collapses to the same location, creating a degenerate edge.

Add corresponding unit tests in the `#[cfg(test)] mod tests` block verifying degree, control point dimensions, and structural validity.

Note: Check how NurbsCurve/NurbsSurface are represented in the codebase. If they use `BsplineCurve<Vector4>` (homogeneous coordinates), use that type. If there's a dedicated NURBS type, use it instead.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry test_fixtures` and verify all new unit tests pass.</verify>
  <done>Three new near-degenerate NURBS fixtures added with passing unit tests.</done>
</task>

<task type="auto">
  <name>Task 3: Add integration tests exercising fixtures with surface constructors</name>
  <files>monstertruck-geometry/tests/pathological_surface_test.rs, monstertruck-geometry/tests/test_fixtures_smoke.rs</files>
  <action>
Create `monstertruck-geometry/tests/pathological_surface_test.rs` with integration tests that exercise the new fixtures through try_* surface constructors:

```rust
use monstertruck_geometry::nurbs::test_fixtures;
use monstertruck_geometry::nurbs::surface_options::*;
use monstertruck_geometry::prelude::*;
```

Tests to add:

1. `sweep_inflection_rail_produces_surface`: Use `fixture_inflection_rail()` with a simple straight profile via `try_sweep_rail`. Assert the result is `Ok` and the surface evaluates to finite values at corners and midpoint.

2. `birail1_converging_rails_handles_convergence`: Use `fixture_converging_rails()` with a matching profile via `try_birail1`. The result may be Ok or an appropriate error -- assert that it does NOT panic. If Ok, verify corners.

3. `sweep_degenerate_section_handles_gracefully`: Use `fixture_degenerate_section()` as profile with a straight rail via `try_sweep_rail`. Assert it doesn't panic and either succeeds or returns a typed error.

4. `sweep_cusped_rail_handles_gracefully`: Use `fixture_cusped_rail()` with a straight profile via `try_sweep_rail`. Assert no panic, accepts Ok or typed error.

5. `near_zero_jacobian_surface_evaluates`: Evaluate `fixture_near_zero_jacobian_surface()` at several parameter values including the degenerate boundary. Assert that `subs()` returns finite (non-NaN, non-infinite) points.

6. `collapsed_control_polygon_surface_evaluates`: Evaluate `fixture_collapsed_control_polygon_surface()` at several parameter values. Assert finite results.

Also update `monstertruck-geometry/tests/test_fixtures_smoke.rs` with smoke tests for the new fixtures (following the existing pattern of checking degree and control point counts).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry pathological_surface` and `cargo nextest run -p monstertruck-geometry test_fixtures_smoke` to verify all tests pass.</verify>
  <done>Integration tests exercise pathological fixtures through surface constructors, and smoke tests validate all new fixtures.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-geometry test_fixtures` passes (unit tests for all fixtures)
2. `cargo nextest run -p monstertruck-geometry pathological_surface` passes (integration tests)
3. `cargo nextest run -p monstertruck-geometry test_fixtures_smoke` passes (smoke tests for new fixtures)
4. No panics in any test -- all pathological cases handled gracefully
5. test_fixtures.rs contains at least 7 new fixture functions beyond the existing corpus
</verification>

<success_criteria>
- Fixture corpus includes inflection rails, converging rails, degenerate sections (FIXTURE-01)
- Fixture corpus includes near-zero Jacobian surface, near-zero weight NURBS, collapsed control polygon (FIXTURE-02)
- Integration tests exercise fixtures through try_sweep_rail, try_birail1 surface constructors
- All tests pass without panics
</success_criteria>

<output>
After completion, create `.tendrion/phases/20-fixture-corpus-and-migration-documentation/20-1-SUMMARY.md`
</output>
