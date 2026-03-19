---
phase: 10-test-infrastructure-and-healing-hooks
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/test_fixtures.rs
  - monstertruck-geometry/src/nurbs/mod.rs
  - monstertruck-solid/tests/fixture_helpers.rs
autonomous: true
must_haves:
  truths:
    - "User can load any fixture by name via a test helper and get a valid BsplineCurve/BsplineSurface"
    - "Fixture corpus includes at least 3 near-degenerate NURBS cases (collapsed control points, zero-length spans, near-singular knots)"
    - "Fixture corpus includes at least 3 problematic rail/section combos (kinked rail, diverging rails, self-intersecting profile)"
    - "Fixture corpus includes representative glyph-like wire profiles (sharp corners, nested contours)"
    - "Running cargo nextest run on monstertruck-geometry with fixture module compiles and loads all fixtures without panic"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/test_fixtures.rs"
      provides: "Programmatic fixture generators for problematic NURBS curves and surfaces"
      min_lines: 150
      contains: "fn degenerate_"
    - path: "monstertruck-solid/tests/fixture_helpers.rs"
      provides: "Integration test helper module that builds CompressedShells from fixture geometries and provides loading/validation utilities"
      min_lines: 100
      contains: "pub fn load_fixture"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/test_fixtures.rs"
      to: "monstertruck-solid/tests/fixture_helpers.rs"
      via: "Test helpers call geometry fixture functions to construct surfaces, then build CompressedShells"
      pattern: "BsplineSurface::sweep_rail"
---

<objective>
Create a comprehensive fixture corpus of problematic geometries for regression testing (TEST-01). This includes near-degenerate NURBS curves/surfaces, problematic rail/section combinations for sweep_rail/birail/gordon, and representative glyph-like profiles. Geometry fixtures live in monstertruck-geometry as an unconditionally compiled module (no #[cfg(test)] gate) so they are visible cross-crate. Topology-level fixture builders and test helpers live in monstertruck-solid/tests/.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-solid/src/healing/mod.rs
@monstertruck-solid/tests/feature_integration.rs
@monstertruck-modeling/src/text.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create geometry-level NURBS fixture generators</name>
  <files>monstertruck-geometry/src/nurbs/test_fixtures.rs, monstertruck-geometry/src/nurbs/mod.rs</files>
  <action>
Create `monstertruck-geometry/src/nurbs/test_fixtures.rs` as an **unconditionally compiled** module (NOT `#[cfg(test)]`). Add `pub mod test_fixtures;` to `monstertruck-geometry/src/nurbs/mod.rs` (no cfg gate). This ensures the module is visible to other crates' test code via `use monstertruck_geometry::nurbs::test_fixtures::*`.

The module should contain these fixture categories:

**Near-degenerate NURBS cases (at least 3):**
1. `degenerate_collapsed_control_points()` -> `BsplineCurve<Point3>`: A curve where 2+ adjacent control points are nearly coincident (distance < 1e-10), causing numerical issues in parameter searches.
2. `degenerate_near_zero_knot_span()` -> `BsplineCurve<Point3>`: A curve with a knot span of ~1e-12, creating a near-C0 discontinuity.
3. `degenerate_high_curvature_pole()` -> `BsplineCurve<Point3>`: A cubic curve with control points arranged to create extreme curvature at one end (curvature radius < 1e-6).
4. `degenerate_surface_collapsed_edge()` -> `BsplineSurface<Point3>`: A surface where one edge collapses to a point (like a cone tip or sphere pole), making the boundary degenerate.

**Problematic rail/section combos (at least 3):**
5. `fixture_kinked_rail()` -> `BsplineCurve<Point3>`: A rail with a sharp kink (tangent reversal) that causes sweep_rail framing to flip.
6. `fixture_diverging_rails()` -> `(BsplineCurve<Point3>, BsplineCurve<Point3>)`: Two rails that diverge wildly, causing birail profile stretching beyond reasonable bounds.
7. `fixture_self_intersecting_profile()` -> `BsplineCurve<Point3>`: A figure-8 like profile that, when swept, creates self-intersecting surfaces.
8. `fixture_closed_rail()` -> `BsplineCurve<Point3>`: A closed (periodic-like) rail where start and end tangents should match but the curve is only C0 at the join.

**Glyph-like profiles (at least 2):**
9. `fixture_glyph_sharp_corners()` -> `Vec<BsplineCurve<Point3>>`: Multiple curves forming a letter-like outline with sharp corners (mimicking a sans-serif 'L' or 'T').
10. `fixture_glyph_nested_contours()` -> `Vec<Vec<BsplineCurve<Point3>>>`: Nested contour profiles (outer boundary + inner hole), mimicking an 'O' or 'B' glyph.

Each function should:
- Return well-formed BsplineCurve/BsplineSurface objects (valid knot vectors, correct control point counts).
- Be declared `pub` so they are visible to other crates' test code.
- Include a doc comment explaining what makes this fixture problematic.
- Be deterministic (no random values).

Use `KnotVector::bezier_knot`, `KnotVector::uniform_knot`, and direct `KnotVector::from` for the various cases. Reference existing patterns in `bspline_surface.rs` tests and examples.

Add a simple `#[cfg(test)] mod tests` block at the bottom of the file that calls each fixture function and asserts the returned objects have the expected degree and control point count.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --lib -E 'test(nurbs::test_fixtures)'` and confirm the module compiles and the sanity test passes.</verify>
  <done>Geometry-level fixture module created as an unconditionally compiled module with 10+ fixture generators covering degenerate NURBS, problematic rail/section combos, and glyph-like profiles, all compiling and passing basic sanity tests.</done>
</task>

<task type="auto">
  <name>Task 2: Create topology-level fixture builders and integration test helpers</name>
  <files>monstertruck-solid/tests/fixture_helpers.rs</files>
  <action>
Create `monstertruck-solid/tests/fixture_helpers.rs` as an integration test helper module. This file is imported via `mod fixture_helpers;` from integration test files (like `healing_fixtures.rs` in plan 10-3).

This module builds `CompressedShell` instances from problematic surfaces by constructing topology around them. It does NOT require any changes to `monstertruck-solid/src/healing/mod.rs`.

The module should provide:

**Fixture shell builders (each returns `CompressedShell<Point3, Curve, Surface>` using monstertruck-modeling types):**

1. `pub fn fixture_sweep_rail_kinked_shell() -> CompressedShell<...>`: Calls `BsplineSurface::sweep_rail` with a kinked rail (construct the kinked rail inline -- a cubic B-spline with control points that cause a sharp tangent reversal at the midpoint). Wraps the surface in a single-face CompressedShell with 4 boundary edges from the surface parameter boundaries (u=0, u=1, v=0, v=1 iso-curves). Deliberately introduces a small vertex gap (~1e-8) at one corner to simulate the topology gaps that surface constructors produce.

2. `pub fn fixture_birail_diverging_shell() -> CompressedShell<...>`: Calls `BsplineSurface::birail1` with two rails that diverge significantly. Builds a CompressedShell with deliberate vertex offsets between adjacent edge endpoints.

3. `pub fn fixture_gordon_degenerate_shell() -> CompressedShell<...>`: Builds a gordon surface from curves that include near-collapsed control points. Creates a 2-face shell where the shared edge has slightly mismatched vertex positions.

4. `pub fn fixture_collapsed_edge_shell() -> CompressedShell<...>`: Creates a shell where one face boundary contains an edge whose two vertices are the same point (collapsed/degenerate edge from a surface pole).

For building CompressedShells, follow the pattern in existing healing tests: create `CompressedEdge` entries with `(vertex_index, vertex_index)` pairs, `CompressedFace` entries with boundary wires referencing `CompressedEdgeIndex`, and vertex coordinates sampled from the surface boundaries using `surface.subs(u, v)`.

Use `monstertruck_modeling::{Curve, Surface}` types since monstertruck-modeling is a dev-dependency of monstertruck-solid.

**Helper utilities:**
- `pub fn load_fixture_shell(name: &str) -> CompressedShell<...>` -- dispatches to the appropriate builder by name string.
- `pub fn assert_shell_valid(shell: &monstertruck_topology::Shell<Point3, Curve, Surface>)` -- checks that `shell.shell_condition()` is at least `Regular`.
- `pub const FIXTURE_NAMES: &[&str]` -- lists all available fixture names: `["sweep_rail_kinked", "birail_diverging", "gordon_degenerate", "collapsed_edge"]`.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` to confirm the crate still compiles. Then create a minimal standalone test file `monstertruck-solid/tests/fixture_smoke.rs` containing `mod fixture_helpers; #[test] fn smoke() { for name in fixture_helpers::FIXTURE_NAMES { let _ = fixture_helpers::load_fixture_shell(name); } }` and run `cargo nextest run -p monstertruck-solid -E 'test(fixture_smoke)'` to verify all fixture builders compile and execute without panic. Remove the smoke test file after verification.</verify>
  <done>Topology-level fixture builders created for sweep_rail, birail, gordon, and collapsed-edge shells in the integration test helpers module, plus load/validation utility functions, all verified to compile and execute.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-geometry --lib -E 'test(nurbs::test_fixtures)'` passes with no panics
2. Each fixture generator produces geometrically valid objects (correct degree, proper knot vector length)
3. At least 3 degenerate NURBS fixtures, 3 problematic rail/section fixtures, and 2 glyph-like fixtures exist
4. Topology-level fixture builders create valid CompressedShell structures
5. Fixture module is unconditionally compiled (no `#[cfg(test)]` gate) and visible cross-crate
6. No changes to `monstertruck-solid/src/healing/mod.rs` are required (all fixture code is in tests/)
7. `cargo clippy --all-targets -- -W warnings` introduces no new warnings
</verification>

<success_criteria>
- TEST-01 is satisfied: fixture corpus covers problematic rail/section combos, near-degenerate NURBS cases, and representative glyph sets
- All fixtures are loadable via test helpers without panics
- Fixture corpus is programmatic and deterministic (no external data files needed)
- Fixture module is unconditionally compiled and accessible from other crates' test code
</success_criteria>

<output>
After completion, create `.tendrion/phases/10-test-infrastructure-and-healing-hooks/10-1-SUMMARY.md`
</output>
