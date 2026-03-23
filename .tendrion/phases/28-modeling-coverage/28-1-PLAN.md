---
phase: 28-modeling-coverage
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-modeling/tests/builder_roundtrip.rs
  - monstertruck-modeling/tests/primitive_test.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo nextest run -p monstertruck-modeling -E 'test(builder_roundtrip)' and all tests pass"
    - "User runs cargo nextest run -p monstertruck-modeling -E 'test(primitive_)' and all tests pass"
    - "Extrude test creates a solid from a face, verifies 6 faces and geometric consistency"
    - "Revolve test creates a torus-like solid, verifies closed shell and geometric consistency"
    - "Sweep test creates a swept solid along a guide curve, verifies geometric consistency"
    - "Primitive cuboid and circle tests verify topology and point positions"
    - "Transformation tests verify translated/rotated/scaled produce correct geometry"
  artifacts:
    - path: "monstertruck-modeling/tests/builder_roundtrip.rs"
      provides: "Round-trip construction tests for builder extrude, revolve, sweep, homotopy, wire_homotopy, skin_wires, transformations"
      min_lines: 180
      contains: "builder::extrude"
    - path: "monstertruck-modeling/tests/primitive_test.rs"
      provides: "Tests for primitive::rect, primitive::circle, primitive::cuboid"
      min_lines: 60
      contains: "primitive::cuboid"
  key_links:
    - from: "monstertruck-modeling/tests/builder_roundtrip.rs"
      to: "monstertruck-modeling/src/builder.rs"
      via: "direct builder API calls with topology verification"
      pattern: "builder::extrude"
    - from: "monstertruck-modeling/tests/primitive_test.rs"
      to: "monstertruck-modeling/src/primitive.rs"
      via: "direct primitive API calls with vertex/edge verification"
      pattern: "primitive::cuboid"
---

<objective>
Add comprehensive round-trip construction tests for the builder module (extrude, revolve, sweep, homotopy, transformations) and primitive module (rect, circle, cuboid) to significantly increase monstertruck-modeling test coverage toward the 45% target.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/builder.rs
@monstertruck-modeling/src/primitive.rs
@monstertruck-modeling/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: Builder round-trip construction tests</name>
  <files>monstertruck-modeling/tests/builder_roundtrip.rs</files>
  <action>
Create a new integration test file `monstertruck-modeling/tests/builder_roundtrip.rs` with comprehensive round-trip tests for builder API methods. Each test constructs geometry and verifies the resulting topology.

Tests to implement:

1. **extrude_vertex_to_edge**: Extrude a vertex by a vector, verify the result is an edge with correct endpoints.

2. **extrude_edge_to_face**: Extrude a line edge to produce a face, verify 4 boundary edges.

3. **extrude_face_to_solid**: Create a square face (via try_attach_plane on a rectangular wire), extrude it, verify the resulting solid has 6 faces (box), 12 edges, 8 vertices, and is_geometric_consistent.

4. **revolve_vertex_to_circle**: Revolve a vertex 360 degrees around an axis, verify the result is a closed wire with the specified division count of edges.

5. **revolve_edge_to_face**: Revolve a line edge 90 degrees, verify the face has 4 boundary edges.

6. **revolve_face_to_solid**: Create a rectangular face in the XZ plane offset from the Y axis, revolve 360 degrees to create a torus-like solid. Verify is_geometric_consistent and that the shell is closed (ShellCondition::Closed).

7. **revolve_wire_degenerate_axis**: Use `builder::revolve_wire` to revolve a wire where one endpoint is on the axis. Verify it produces 3-sided faces (not degenerate 4-sided).

8. **homotopy_two_lines**: Create two line edges, call `builder::homotopy`, verify the face has 4 boundary edges.

9. **wire_homotopy_matching_edges**: Create two matching wires (same edge count), call `try_wire_homotopy`, verify shell face count equals edge count.

10. **wire_homotopy_mismatch_error**: Create two wires with different edge counts, verify `try_wire_homotopy` returns `Err(Error::NotSameNumberOfEdges)`.

11. **skin_wires_three_strips**: Create 3 wires, call `try_skin_wires`, verify shell has 2 * edges_per_wire faces.

12. **skin_wires_too_few_error**: Call `try_skin_wires` with 1 wire, verify it returns an error.

13. **transformed_vertex_position**: Create a vertex, apply `builder::translated`, verify point moved correctly.

14. **rotated_edge_consistency**: Create a line edge, apply `builder::rotated` by 90 degrees around Z axis, verify front/back points are rotated.

15. **scaled_face_bounding_box**: Create a rectangular face, apply `builder::scaled` by (2, 3, 1), verify vertex positions scaled correctly.

16. **sweep_rail_face**: Create a line edge as profile and a curve as rail. Call `builder::try_sweep_rail(&profile, &rail, 3)`. Verify the result is a face with correct topology (4 boundary edges for a quadrilateral sweep surface).

Use these imports:
```rust
use monstertruck_modeling::*;
use monstertruck_modeling::errors::Error;
use monstertruck_topology::shell::ShellCondition;
use std::f64::consts::PI;
```

Helper function:
```rust
fn rect_wire(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, y0, 0.0));
    let v1 = builder::vertex(Point3::new(x1, y0, 0.0));
    let v2 = builder::vertex(Point3::new(x1, y1, 0.0));
    let v3 = builder::vertex(Point3::new(x0, y1, 0.0));
    vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into()
}

fn rect_wire_xz(x0: f64, z0: f64, x1: f64, z1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, 0.0, z0));
    let v1 = builder::vertex(Point3::new(x1, 0.0, z0));
    let v2 = builder::vertex(Point3::new(x1, 0.0, z1));
    let v3 = builder::vertex(Point3::new(x0, 0.0, z1));
    vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into()
}
```

Each test should construct the object, then assert on topology counts (vertices, edges, faces) and geometric consistency where applicable. Use `assert_near!` or `Tolerance::near` for floating-point comparisons where needed.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling -E 'test(builder_roundtrip)' --no-fail-fast` and verify all tests pass.</verify>
  <done>Builder round-trip tests for extrude, revolve, sweep, homotopy, wire_homotopy, skin_wires, and transformations created and passing.</done>
</task>

<task type="auto">
  <name>Task 2: Primitive module tests</name>
  <files>monstertruck-modeling/tests/primitive_test.rs</files>
  <action>
Create a new integration test file `monstertruck-modeling/tests/primitive_test.rs` testing the primitive module functions:

1. **rect_xy_plane**: Create a rectangle on the XY plane using `primitive::rect` with a BoundingBox. Verify 4 edges, wire is closed, and vertex positions match expected corners.

2. **rect_zx_plane**: Create a rectangle on the ZX plane. Verify vertex positions match the plane's coordinate mapping.

3. **circle_division_2**: Create a circle with division=2. Verify 2 edges, wire is closed, all points equidistant from origin.

4. **circle_division_4**: Create a circle with division=4. Verify 4 edges, wire is closed, all edge midpoints equidistant from origin.

5. **cuboid_topology**: Create a cuboid from a BoundingBox. Verify 6 faces, 12 edges, 8 vertices in the solid.

6. **cuboid_vertex_positions**: Create a cuboid, collect all vertex points. Verify each coordinate is near one of the two extremes of the bounding box.

7. **cuboid_geometric_consistency**: Create a cuboid, verify `is_geometric_consistent()`.

Use these imports:
```rust
use monstertruck_modeling::*;
```

Sample vertex positions using `Tolerance::near` for floating-point assertions.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling -E 'test(primitive_)' --no-fail-fast` and verify all tests pass.</verify>
  <done>Primitive module tests for rect, circle, and cuboid created and passing.</done>
</task>

<task type="auto">
  <name>Task 3: Verify coverage improvement</name>
  <files>monstertruck-modeling/tests/builder_roundtrip.rs, monstertruck-modeling/tests/primitive_test.rs</files>
  <action>
Run clippy on the new test files to ensure no warnings:
```bash
cargo clippy -p monstertruck-modeling --tests -- -W warnings
```

Fix any clippy warnings in the new test files.

Then run the full monstertruck-modeling test suite to verify nothing is broken:
```bash
cargo nextest run -p monstertruck-modeling --no-fail-fast
```

Confirm that all existing tests still pass alongside the new ones.
  </action>
  <verify>Clippy reports no warnings and all tests pass including the new test files.</verify>
  <done>All new and existing tests pass, clippy clean, coverage contribution verified.</done>
</task>

</tasks>

<verification>
1. All tests in builder_roundtrip.rs pass with `cargo nextest run -p monstertruck-modeling -E 'test(builder_roundtrip)'`
2. All tests in primitive_test.rs pass with `cargo nextest run -p monstertruck-modeling -E 'test(primitive_)'`
3. No clippy warnings on `cargo clippy -p monstertruck-modeling --tests -- -W warnings`
4. Full test suite passes: `cargo nextest run -p monstertruck-modeling --no-fail-fast`
5. Builder extrude/revolve/sweep each have at least one round-trip construction test
</verification>

<success_criteria>
- Builder API methods (extrude, revolve, sweep) each have at least one round-trip construction test
- Primitive module functions (rect, circle, cuboid) each have at least one test
- Transformation functions (translated, rotated, scaled) have tests
- All tests are deterministic and pass consistently
- No clippy warnings introduced
</success_criteria>

<output>
After completion, create `.tendrion/phases/28-modeling-coverage/28-1-SUMMARY.md`
</output>
