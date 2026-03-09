---
phase: 3-feature-completeness
plan: 3
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/shell_ops/mod.rs
  - monstertruck-solid/src/shell_ops/shell_op.rs
  - monstertruck-solid/src/shell_ops/offset_op.rs
  - monstertruck-solid/src/shell_ops/tests.rs
  - monstertruck-solid/src/lib.rs
autonomous: true
must_haves:
  truths:
    - "User calls shell_solid on a cube with wall thickness 0.1 and gets a hollow box with 12 faces (6 outer + 6 inner)"
    - "User calls offset_surface on a BsplineSurface and gets a valid offset surface at the specified distance"
    - "Shell operation produces a valid Solid with two boundary shells (outer and inner)"
    - "Offset operation preserves surface tangent continuity within approximation tolerance"
    - "Shell and offset results pass topological validity checks (closed shells, no singular vertices)"
  artifacts:
    - path: "monstertruck-solid/src/shell_ops/mod.rs"
      provides: "Shell/offset module with public API"
      min_lines: 20
      contains: "pub fn shell_solid"
    - path: "monstertruck-solid/src/shell_ops/shell_op.rs"
      provides: "Shell (hollow-out) operation implementation"
      min_lines: 80
      contains: "shell_solid"
    - path: "monstertruck-solid/src/shell_ops/offset_op.rs"
      provides: "Surface offset operation for shells"
      min_lines: 60
      contains: "offset_shell"
    - path: "monstertruck-solid/src/shell_ops/tests.rs"
      provides: "Tests for shell and offset operations"
      min_lines: 100
      contains: "shell_cube"
  key_links:
    - from: "monstertruck-solid/src/shell_ops/shell_op.rs"
      to: "monstertruck-geometry/src/nurbs/offset.rs"
      via: "Uses surface_offset for creating inner offset surfaces"
      pattern: "surface_offset"
    - from: "monstertruck-solid/src/shell_ops/mod.rs"
      to: "monstertruck-solid/src/lib.rs"
      via: "Module re-exported from crate root"
      pattern: "shell_ops"
---

<objective>
Implement shell (hollow-out) and offset operations for solid bodies. Shell creates a hollow solid by offsetting all faces inward by a wall thickness. Offset creates an offset surface at a specified distance from the original.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/lib.rs
@monstertruck-geometry/src/nurbs/offset.rs
@monstertruck-topology/src/compress.rs
@monstertruck-modeling/src/geometry.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for shell and offset operations</name>
  <files>monstertruck-solid/src/shell_ops/tests.rs, monstertruck-solid/src/shell_ops/mod.rs, monstertruck-solid/src/lib.rs</files>
  <action>
**TDD Red phase**: Create the module structure and write tests first.

1. Create `monstertruck-solid/src/shell_ops/mod.rs`:
```rust
//! Shell (hollow-out) and surface offset operations for solid bodies.

mod shell_op;
mod offset_op;

#[cfg(test)]
mod tests;

pub use shell_op::shell_solid;
pub use offset_op::offset_shell;
```

2. Create stub implementations in `shell_op.rs` and `offset_op.rs` that return `todo!()` or `Err`.

3. Add `pub mod shell_ops;` and re-exports to `monstertruck-solid/src/lib.rs`:
```rust
pub mod shell_ops;
pub use shell_ops::{shell_solid, offset_shell};
```

4. Create `monstertruck-solid/src/shell_ops/tests.rs` with tests:

```rust
use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;

#[test]
fn shell_cube_valid_topology() {
    // Create unit cube
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    // Shell with 0.1 wall thickness
    let shelled = super::shell_solid(&cube, 0.1, 20).unwrap();

    // Should have 2 boundary shells (outer + inner)
    assert_eq!(shelled.boundaries().len(), 2);

    // Both shells should be closed
    for shell in shelled.boundaries() {
        assert_eq!(shell.shell_condition(), ShellCondition::Closed);
        assert!(shell.singular_vertices().is_empty());
        assert!(shell.extract_boundaries().is_empty());
    }

    // Outer shell: 6 faces (original), inner shell: 6 faces (offset)
    assert_eq!(shelled.boundaries()[0].len(), 6);
    assert_eq!(shelled.boundaries()[1].len(), 6);
}

#[test]
fn shell_cube_wall_thickness_geometric() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let thickness = 0.1;
    let shelled = super::shell_solid(&cube, thickness, 20).unwrap();

    // Check that inner shell vertices are offset inward
    let inner_shell = &shelled.boundaries()[1];
    for face in inner_shell.iter() {
        // Each inner face's surface should be approximately `thickness` away
        // from the corresponding outer face
    }
}

#[test]
fn offset_shell_flat_surface() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let outer_shell = &cube.boundaries()[0];
    let offset = super::offset_shell(outer_shell, -0.1, 20).unwrap();

    // Offset shell should have same number of faces
    assert_eq!(offset.len(), outer_shell.len());

    // Offset shell should be closed
    assert_eq!(offset.shell_condition(), ShellCondition::Closed);
}

#[test]
fn shell_serialization_round_trip() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let shelled = super::shell_solid(&cube, 0.1, 20).unwrap();
    let compressed = shelled.compress();
    let json = serde_json::to_vec(&compressed).unwrap();
    let restored: monstertruck_topology::compress::CompressedSolid<Point3, Curve, Surface> =
        serde_json::from_slice(&json).unwrap();
    let restored_solid = Solid::extract(restored).unwrap();
    assert_eq!(restored_solid.boundaries().len(), 2);
}
```

Run tests -- they should fail since implementations are stubs.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- shell_ops` and confirm tests exist but fail (compile or runtime).
  </verify>
  <done>Shell/offset test suite created with topological validity assertions. Tests fail because implementations are stubs.</done>
</task>

<task type="auto">
  <name>Task 2: Implement offset_shell and shell_solid operations</name>
  <files>monstertruck-solid/src/shell_ops/offset_op.rs, monstertruck-solid/src/shell_ops/shell_op.rs</files>
  <action>
**TDD Green phase**: Implement the operations to make the tests pass.

**offset_op.rs** - `offset_shell`:
```rust
use monstertruck_topology::prelude;
// Use the crate's concrete type aliases (Point3, Curve, Surface)
// following the pattern from the fillet module.

/// Offset all surfaces of a shell by `distance` along their normals.
///
/// Positive distance offsets outward (along surface normal direction).
/// Negative distance offsets inward.
///
/// `n_samples` controls the approximation quality for non-planar surfaces.
pub fn offset_shell(
    shell: &Shell<Point3, Curve, Surface>,
    distance: f64,
    n_samples: usize,
) -> Option<Shell<Point3, Curve, Surface>>
```

Use concrete `Surface` enum type so variant dispatch (Plane vs BsplineSurface vs NurbsSurface) is possible. Follow the fillet module pattern: `monstertruck_topology::prelude!(Point3, Curve, Surface, pub(super))`.

The implementation should:
1. Compress the shell to `CompressedShell`
2. For each face's surface, compute the offset surface:
   - For `Surface::Plane`: offset the plane by translating along its normal by `distance`
   - For `Surface::BsplineSurface`: use `monstertruck_geometry::nurbs::offset::surface_offset`
   - For `Surface::NurbsSurface`: convert to BsplineSurface, offset, convert back
   - For `Surface::RevolutedCurve`: offset the entity curve
   - For `Surface::TSplineSurface`: convert to BsplineSurface, offset
3. For each vertex, compute offset position by averaging normals of adjacent faces
4. For each edge curve, refit along the offset surface
5. Reconstruct the shell from offset data
6. Validate and return

**Alternative simpler approach** (recommended for robustness):
1. For a shell composed of Plane surfaces (like a cube), offset each plane
2. Compute new vertex positions as intersections of offset planes
3. Recompute edge curves (Lines between new vertices)
4. Build the offset shell from scratch

**shell_op.rs** - `shell_solid`:
```rust
/// Create a hollow solid by offsetting all faces inward.
///
/// Returns a Solid with two boundary shells: the original outer shell
/// and a new inner shell offset by `wall_thickness`.
pub fn shell_solid(
    solid: &Solid<Point3, Curve, Surface>,
    wall_thickness: f64,
    n_samples: usize,
) -> Option<Solid<Point3, Curve, Surface>>
```

Implementation:
1. Take the outer shell (boundaries()[0])
2. Call `offset_shell` with negative `wall_thickness`
3. Invert the inner shell orientation (all face normals point inward)
4. Build a new Solid with [outer_shell, inner_shell]

For a first implementation, focus on solids with planar faces (cubes, boxes, prisms). Surface offset for curved faces is already available via the geometry crate.

Make the tests pass.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- shell_ops` and confirm all tests pass.
Run `cargo test -p monstertruck-solid` to confirm no regressions.
  </verify>
  <done>Shell and offset operations implemented. Shell creates valid hollow solids with inner offset shells. Offset creates valid offset shells preserving topology.</done>
</task>

<task type="auto">
  <name>Task 3: Add edge case tests and curved surface support</name>
  <files>monstertruck-solid/src/shell_ops/tests.rs, monstertruck-solid/src/shell_ops/offset_op.rs</files>
  <action>
Add additional tests for robustness:

1. `offset_shell_curved_surface`: Create a cylinder or sphere and offset its shell
2. `shell_with_open_faces`: Test that shell_solid handles the case where the user wants to leave some faces open (e.g., an open box) -- this is a common "shell" operation variant where specific faces are removed. Add an optional `open_faces` parameter or note it as future work.
3. `shell_negative_thickness_error`: Verify that providing a wall thickness larger than the smallest dimension fails gracefully rather than producing a self-intersecting result.
4. `offset_shell_preserves_face_count`: Verify offset doesn't add or remove faces.

Ensure all offset operations for planar shells are exact (Plane offset is trivial: translate origin along normal). For curved surfaces, use the `surface_offset` function from monstertruck-geometry with appropriate sample counts.

Fix any failures that arise during testing.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- shell_ops` and confirm all tests pass.
  </verify>
  <done>Shell/offset operations handle edge cases robustly. Curved surface offset uses geometry crate's surface_offset. Error cases are handled gracefully.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-solid -- shell_ops` -- all shell/offset tests pass
2. `cargo test -p monstertruck-solid` -- no regressions in existing tests
3. Shell operation produces a valid Solid with two closed boundary shells
4. Offset preserves face count and shell topology
5. Operations work for planar-face solids (cubes, boxes)
</verification>

<success_criteria>
- Shell operations hollow out a solid body to a specified wall thickness
- Offset operations produce a valid offset surface
- Results pass topological validity checks (closed shells, no singular vertices)
- Operations handle both planar and curved surfaces
</success_criteria>

<output>
After completion, create `.tendrion/phases/3-feature-completeness/3-3-SUMMARY.md`
</output>
