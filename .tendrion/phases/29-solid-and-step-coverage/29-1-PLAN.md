---
phase: 29-solid-and-step-coverage
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/tests/boolean_ops_coverage.rs
  - monstertruck-solid/tests/fillet_coverage.rs
  - monstertruck-solid/tests/healing_coverage.rs
autonomous: true
must_haves:
  truths:
    - "cargo nextest run -p monstertruck-solid passes with all new tests green"
    - "Boolean union of two overlapping cubes produces a solid whose bounding box encompasses both inputs"
    - "Boolean intersection of two overlapping cubes produces a solid smaller than either input"
    - "Boolean difference of A minus B produces a solid that excludes B's interior"
    - "Symmetric difference of overlapping cubes produces a valid solid with non-empty boundaries"
    - "Fillet pipeline entry points (fillet_edges, fillet_edges_generic) succeed on a cube edge"
    - "Healing functions (heal_surface_shell, extract_healed) produce valid shells from known-degenerate inputs"
  artifacts:
    - path: "monstertruck-solid/tests/boolean_ops_coverage.rs"
      provides: "Unit tests for boolean union, intersection, difference, and symmetric_difference"
      min_lines: 100
      contains: "monstertruck_solid::and"
    - path: "monstertruck-solid/tests/fillet_coverage.rs"
      provides: "Unit tests for fillet pipeline entry points (fillet_edges, fillet_edges_generic)"
      min_lines: 80
      contains: "fillet_edges"
    - path: "monstertruck-solid/tests/healing_coverage.rs"
      provides: "Unit tests for healing module functions (heal_surface_shell, extract_healed, SplitClosedEdgesAndFaces)"
      min_lines: 80
      contains: "heal_surface_shell"
  key_links:
    - from: "monstertruck-solid/tests/boolean_ops_coverage.rs"
      to: "monstertruck-solid/src/transversal/mod.rs"
      via: "Public API calls to and/or/difference/symmetric_difference"
      pattern: "monstertruck_solid::and"
    - from: "monstertruck-solid/tests/fillet_coverage.rs"
      to: "monstertruck-solid/src/fillet/mod.rs"
      via: "Public API calls to fillet_edges and fillet_edges_generic"
      pattern: "monstertruck_solid::fillet_edges"
    - from: "monstertruck-solid/tests/healing_coverage.rs"
      to: "monstertruck-solid/src/healing/mod.rs"
      via: "Public API calls to heal_surface_shell and extract_healed"
      pattern: "monstertruck_solid::heal_surface_shell"
---

<objective>
Add meaningful test coverage to monstertruck-solid by creating integration tests for boolean operations (union, intersection, difference, symmetric_difference), fillet pipeline entry points (fillet_edges, fillet_edges_generic with various options), and healing module functions (heal_surface_shell, extract_healed, SplitClosedEdgesAndFaces trait).
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/lib.rs
@monstertruck-solid/src/transversal/mod.rs
@monstertruck-solid/src/fillet/mod.rs
@monstertruck-solid/src/healing/mod.rs
@monstertruck-solid/tests/boolean_edge_cases.rs
@monstertruck-solid/tests/feature_integration.rs
@monstertruck-solid/tests/healing_fixtures.rs
@monstertruck-solid/tests/fixture_helpers.rs
@monstertruck-solid/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: Boolean operations coverage tests</name>
  <files>monstertruck-solid/tests/boolean_ops_coverage.rs</files>
  <action>
Create a new integration test file `boolean_ops_coverage.rs` that tests boolean operations with geometric validation. The existing `boolean_edge_cases.rs` primarily tests panic safety and edge cases (tangent faces, empty boundaries, pole degeneration). This new file should test correctness of the boolean results.

Structure:
```rust
use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;
```

Helper functions:
- `make_cube(origin: Point3, side: f64) -> Solid` -- builds a cube using builder::vertex -> extrude chain
- `solid_bounding_box(solid: &Solid) -> (Point3, Point3)` -- computes bounding box via triangulation and `to_polygon().bounding_box()`
- `approximate_volume(solid: &Solid) -> f64` -- rough volume estimate from bounding box diagonal

Tests to create:

1. `boolean_and_overlapping_cubes` -- Two unit cubes offset by (0.5, 0.5, 0.5). AND should produce a solid whose bounding box is contained within both inputs. Verify shell condition is Closed. Verify face count is 6 (intersection of two cubes is a cube).

2. `boolean_or_overlapping_cubes` -- Same two cubes. OR should produce a solid whose bounding box encompasses both inputs. Verify Closed shell condition.

3. `boolean_difference_cubes` -- Cube A at origin, cube B offset by (0.5,0.5,0.5). difference(A,B) should produce a non-empty solid whose bounding box matches A's extent on the non-cut sides.

4. `boolean_symmetric_difference_cubes` -- Same cubes. symmetric_difference should produce a non-empty solid. Verify boundaries are non-empty.

5. `boolean_and_disjoint_returns_valid` -- Two cubes with no overlap. AND result should either be an error or produce a solid with empty/trivial boundaries (valid behavior for disjoint inputs).

6. `boolean_or_disjoint_cubes` -- Two fully disjoint cubes. OR should produce a valid result (possibly two separate shells).

7. `boolean_difference_contained` -- Small cube fully inside a large cube. difference(large, small) should produce a hollow solid with 2 boundary shells.

8. `boolean_and_identical_cubes` -- Two identical cubes. AND should produce a solid equivalent to either input (bounding box should match).

For each test:
- Use `catch_unwind` wrapper for panic safety
- Assert `.is_ok()` on the result
- Verify `!solid.boundaries().is_empty()`
- Verify `ShellCondition::Closed` on each boundary shell
- Use bounding box checks for geometric correctness
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid boolean_ops_coverage` and verify all tests pass.</verify>
  <done>Boolean operations coverage tests created and passing.</done>
</task>

<task type="auto">
  <name>Task 2: Fillet pipeline coverage tests</name>
  <files>monstertruck-solid/tests/fillet_coverage.rs</files>
  <action>
Create a new integration test file `fillet_coverage.rs` that tests fillet pipeline entry points. The existing inline tests in `src/fillet/tests.rs` are extensive (58 tests) but use internal types. This file tests the public API from the consumer perspective.

Structure:
```rust
use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;
```

Helper:
- `make_cube() -> Solid` -- builds a unit cube

Tests to create:

1. `fillet_edges_round_profile` -- Build a cube, select one edge via `shell.edge_iter().take(1)`, call `fillet_edges_generic` with `FilletProfile::Round` and `RadiusSpec::Constant(0.1)`. Verify shell condition is Closed, verify the shell has more faces than the original 6 (fillet adds faces).

2. `fillet_edges_chamfer_profile` -- Same setup but with `FilletProfile::Chamfer`. Verify Closed shell, more faces than 6.

3. `fillet_edges_ridge_profile` -- Same setup with `FilletProfile::Ridge`. Verify Closed shell.

4. `fillet_multiple_edges` -- Select 2-3 edges from the cube, call `fillet_edges_generic`. Verify Closed shell, face count increased by at least the number of filleted edges.

5. `fillet_with_variable_radius` -- Use `RadiusSpec::Variable` with a closure `|t| 0.05 + 0.1 * t`. Verify the result is Ok and shell is Closed.

6. `fillet_invalid_radius_returns_error` -- Use an extremely large radius (e.g., 10.0 on a unit cube edge). Verify the result is Err (fillet should fail gracefully).

7. `fillet_result_topology_valid` -- After filleting, verify `shell.singular_vertices().is_empty()` and `shell.shell_condition() == ShellCondition::Closed`.

Use `FilletOptions { radius, profile, ..Default::default() }` for option construction. Refer to `feature_integration.rs` for the pattern.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid fillet_coverage` and verify all tests pass.</verify>
  <done>Fillet pipeline coverage tests created and passing.</done>
</task>

<task type="auto">
  <name>Task 3: Healing module coverage tests</name>
  <files>monstertruck-solid/tests/healing_coverage.rs</files>
  <action>
Create a new integration test file `healing_coverage.rs` that tests the healing module's public API beyond what `healing_fixtures.rs` already covers. The existing tests focus on panic safety and fixture-specific behavior. This file adds tests for:

Structure:
```rust
use monstertruck_geometry::prelude::*;
use monstertruck_modeling::*;
use monstertruck_topology::compress::*;
use monstertruck_topology::shell::ShellCondition;
```

Tests to create:

1. `extract_healed_well_formed_shell` -- Build a cube solid, compress it to `CompressedShell`, call `extract_healed(cshell, 0.05)`. Verify the result is Ok and the extracted shell condition is at least Regular.

2. `heal_surface_shell_well_formed` -- Build a cube solid, compress to CompressedShell, call `heal_surface_shell(cshell, 0.05)`. Verify the shell condition of the result is Closed.

3. `split_closed_edges_and_faces_noop_on_cube` -- Build a cube, compress it, call `split_closed_edges_and_faces(0.05)` on the CompressedShell. Verify the shell still has the same number of faces, edges, and vertices (a cube has no closed edges to split).

4. `robust_split_closed_edges_and_faces_noop_on_cube` -- Same as above but with `robust_split_closed_edges_and_faces`.

5. `heal_surface_shell_cylinder` -- Build a cylinder solid (extrude a circle), compress it, call `heal_surface_shell`. Verify the result is Ok or returns a known error type (NonManifoldEdges for open shells).

6. `extract_healed_preserves_face_count` -- Build a cube, compress, call `extract_healed`. Verify the resulting shell has 6 faces (same as the original cube).

7. `heal_surface_shell_with_gap` -- Construct a CompressedShell with a small vertex gap (~1e-8) using the same pattern as `fixture_helpers.rs::single_face_shell_from_surface`. Call `heal_surface_shell` and verify healing runs without panic. Accept NonManifoldEdges for single-face open shells.

Build cube solid for compression using:
```rust
let v = builder::vertex(Point3::origin());
let e = builder::extrude(&v, Vector3::unit_x());
let f = builder::extrude(&e, Vector3::unit_y());
let solid: Solid = builder::extrude(&f, Vector3::unit_z());
let compressed_shell = solid.boundaries()[0].compress();
```

For cylinder:
```rust
use std::f64::consts::PI;
let v = builder::vertex(Point3::origin());
let circle_edge = builder::circle_arc(&v, &v, Point3::new(1.0, 0.0, 0.0));
// Or use builder::rsweep for a full revolution
```

Note: The `SplitClosedEdgesAndFaces` and `RobustSplitClosedEdgesAndFaces` traits need to be imported from `monstertruck_solid`.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid healing_coverage` and verify all tests pass.</verify>
  <done>Healing module coverage tests created and passing.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid boolean_ops_coverage` -- all boolean tests pass
2. `cargo nextest run -p monstertruck-solid fillet_coverage` -- all fillet tests pass
3. `cargo nextest run -p monstertruck-solid healing_coverage` -- all healing tests pass
4. `cargo nextest run -p monstertruck-solid` -- all existing + new tests pass (no regressions)
5. New tests exercise the public API functions: `and`, `or`, `difference`, `symmetric_difference`, `fillet_edges_generic`, `heal_surface_shell`, `extract_healed`, `SplitClosedEdgesAndFaces`
</verification>

<success_criteria>
- monstertruck-solid has new unit tests covering boolean union/intersection/difference/symmetric_difference operations with geometric validation
- monstertruck-solid has new unit tests covering fillet pipeline entry points (fillet_edges_generic with Round, Chamfer, Ridge profiles)
- monstertruck-solid has new unit tests covering healing module functions (heal_surface_shell, extract_healed, SplitClosedEdgesAndFaces trait)
- All new tests pass via `cargo nextest run -p monstertruck-solid`
- No regressions in existing tests
</success_criteria>

<output>
After completion, create `.tendrion/phases/29-solid-and-step-coverage/29-1-SUMMARY.md`
</output>
