---
phase: 3-feature-completeness
plan: 4
type: tdd
wave: 2
depends_on: ["3-3"]
files_modified:
  - monstertruck-solid/src/draft/mod.rs
  - monstertruck-solid/src/draft/draft_op.rs
  - monstertruck-solid/src/draft/tests.rs
  - monstertruck-solid/src/lib.rs
autonomous: true
must_haves:
  truths:
    - "User applies a 5-degree draft angle to faces of a cube and gets a valid solid with tapered faces"
    - "User specifies a pull direction and neutral plane, and the draft operation tilts faces relative to that plane"
    - "Drafted solid passes topological validity checks (closed shell, no singular vertices)"
    - "Draft produces valid B-rep output that can be serialized and deserialized"
    - "Draft angle of 0 degrees returns the original solid unchanged"
  artifacts:
    - path: "monstertruck-solid/src/draft/mod.rs"
      provides: "Draft/taper module with public API"
      min_lines: 15
      contains: "pub fn draft_faces"
    - path: "monstertruck-solid/src/draft/draft_op.rs"
      provides: "Draft/taper operation implementation"
      min_lines: 100
      contains: "draft_faces"
    - path: "monstertruck-solid/src/draft/tests.rs"
      provides: "Tests for draft/taper operations"
      min_lines: 100
      contains: "draft_cube_faces"
  key_links:
    - from: "monstertruck-solid/src/draft/mod.rs"
      to: "monstertruck-solid/src/lib.rs"
      via: "Module re-exported from crate root"
      pattern: "pub mod draft"
    - from: "monstertruck-solid/src/draft/draft_op.rs"
      to: "monstertruck-topology/src/face.rs"
      via: "Draft modifies face surfaces and edge curves"
      pattern: "Face"
---

<objective>
Implement draft/taper operations for solid bodies. Draft applies a specified angle to selected faces relative to a pull direction and neutral plane, tilting faces for injection mold release. This is required for injection-molded part design workflows.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/lib.rs
@monstertruck-topology/src/face.rs
@monstertruck-topology/src/shell.rs
@monstertruck-modeling/src/geometry.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for draft/taper operations</name>
  <files>monstertruck-solid/src/draft/tests.rs, monstertruck-solid/src/draft/mod.rs, monstertruck-solid/src/draft/draft_op.rs, monstertruck-solid/src/lib.rs</files>
  <action>
**TDD Red phase**: Create the module structure and write tests.

1. Create `monstertruck-solid/src/draft/mod.rs`:
```rust
//! Draft (taper) operations for solid bodies.
//!
//! Applies a draft angle to selected faces relative to a pull direction
//! and neutral plane. Used in injection mold design to ensure parts
//! can be released from molds.

mod draft_op;

#[cfg(test)]
mod tests;

pub use draft_op::{draft_faces, DraftOptions, DraftError};
```

2. Create `monstertruck-solid/src/draft/draft_op.rs` with stub:
```rust
use monstertruck_geometry::prelude::*;
use monstertruck_topology::*;
use thiserror::Error;

/// Errors for draft operations.
#[derive(Debug, Error)]
pub enum DraftError {
    /// The draft angle is out of valid range.
    #[error("draft angle must be between 0 and 90 degrees")]
    InvalidAngle,
    /// The pull direction is zero-length.
    #[error("pull direction must be non-zero")]
    InvalidPullDirection,
    /// A face could not be drafted.
    #[error("failed to draft face at index {index}")]
    FaceDraftFailed { index: usize },
    /// The resulting shell is topologically invalid.
    #[error("draft produced invalid topology: {message}")]
    InvalidTopology { message: String },
}

/// Options for draft operations.
#[derive(Debug, Clone)]
pub struct DraftOptions {
    /// Draft angle in radians. Must be positive and less than PI/2.
    pub angle: f64,
    /// Pull direction (mold opening direction). Must be non-zero.
    pub pull_direction: Vector3,
    /// Neutral plane: faces are tilted relative to this plane.
    /// Points on the neutral plane remain fixed during drafting.
    pub neutral_plane: Plane,
}

/// Apply draft angle to specified faces of a solid.
///
/// Each selected face is tilted by `angle` relative to `pull_direction`,
/// pivoting around the intersection with `neutral_plane`.
///
/// Returns a new solid with drafted faces.
pub fn draft_faces<C, S>(
    solid: &Solid<Point3, C, S>,
    face_indices: &[usize],
    options: &DraftOptions,
) -> Result<Solid<Point3, C, S>, DraftError> {
    todo!()
}
```

3. Add `pub mod draft;` and re-exports to `monstertruck-solid/src/lib.rs` (after the existing shell_ops module added by plan 3-3):
```rust
pub mod draft;
pub use draft::{draft_faces, DraftOptions, DraftError};
```

4. Create `monstertruck-solid/src/draft/tests.rs` with these tests:

- `draft_cube_faces_valid_topology`: Create unit cube, draft 4 side faces by 5 degrees around z-axis pull direction and neutral plane at z=0.5. Assert closed shell, no singular vertices.
- `draft_zero_angle_returns_original`: Draft with angle=0.0 returns solid with same face count.
- `draft_invalid_angle_error`: Draft with angle=PI returns `DraftError::InvalidAngle`.
- `draft_serialization_round_trip`: Draft cube, compress, serialize to JSON, deserialize, extract, verify boundary count matches.

Run tests -- they should fail since `draft_faces` is a stub.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- draft::tests` and confirm tests exist but fail.
  </verify>
  <done>Draft/taper test suite created with topological validity, zero-angle, error handling, and serialization tests. Tests fail because implementation is a stub.</done>
</task>

<task type="auto">
  <name>Task 2: Implement draft_faces operation</name>
  <files>monstertruck-solid/src/draft/draft_op.rs</files>
  <action>
**TDD Green phase**: Implement `draft_faces` to make the tests pass.

The draft operation algorithm for planar faces:

1. **Validate inputs**:
   - `angle` must be in [0, PI/2)
   - `pull_direction` must be non-zero

2. **Handle zero angle**: return a clone of the original solid.

3. **For each selected face** (planar faces case):
   - Find the intersection line between the face's plane and the neutral plane
   - This intersection line is the "hinge" around which the face rotates
   - Rotate the face's plane by `angle` around the hinge axis
   - The rotation direction is determined by: the face normal should tilt away from the pull direction
   - For a face with normal perpendicular to pull direction:
     - The draft rotates the face so that above the neutral plane it moves outward (positive draft)
     - Below the neutral plane it also moves outward

4. **Rebuild topology**:
   - Compute new vertex positions as intersections of the modified planes with neighboring planes
   - Recompute edge curves (lines between new vertices)
   - Build new faces with updated surfaces
   - Construct the shell and solid

**Implementation approach for planar faces** (simpler, covers cube/box):
Work with compressed representation. For each face to draft, compute the tilted plane. Then recompute all vertex positions as 3-plane intersections. Update edge curves as Lines between new vertex positions. Rebuild the shell via `Shell::extract`.

The key challenge is recomputing vertex positions after tilting faces. For a cube:
- Each vertex is the intersection of 3 planes
- When some planes are tilted, the vertex positions change
- Edge curves (lines) must be updated to connect the new vertex positions

Focus on making the planar-face case work first. Use the `Plane` type's ability to compute ray-plane intersections or solve 3-plane intersection systems.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- draft::tests` and confirm all tests pass.
Run `cargo test -p monstertruck-solid` to confirm no regressions.
  </verify>
  <done>Draft operation implemented for planar-face solids. Tests pass including topological validity, zero-angle identity, error handling, and serialization round-trip.</done>
</task>

<task type="auto">
  <name>Task 3: Add geometric verification tests for draft angle</name>
  <files>monstertruck-solid/src/draft/tests.rs</files>
  <action>
Add tests that verify the geometric correctness of the draft:

1. `draft_cube_angle_verification`:
   - Create a cube, draft the 4 side faces by 5 degrees
   - Measure the angle between each drafted face and the pull direction
   - Verify the angle changed by approximately the draft amount
   - Use: `normal.dot(pull_direction).acos()` should differ from PI/2 by ~5 degrees

2. `draft_cube_neutral_plane_fixed`:
   - Create a cube, draft side faces
   - Verify that points on the neutral plane remain (approximately) on the neutral plane after drafting
   - Sample the face surface at the neutral plane height and check distance to neutral plane is near zero

3. `draft_10_degree_larger_angle`:
   - Draft with a larger angle (10 degrees) and verify the solid is still valid
   - Check face angles are correct

4. `draft_box_non_unit`:
   - Create a non-unit box (e.g., 2x3x4) and draft its side faces
   - Verify topological validity

Ensure geometric assertions use appropriate tolerance (`TOLERANCE` or `0.01` for angle checks in radians).
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- draft::tests` and confirm all tests pass.
  </verify>
  <done>Draft operations verified geometrically: draft angles are correct, neutral plane points are preserved, and the operation works on non-unit boxes.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-solid -- draft::tests` -- all draft tests pass
2. `cargo test -p monstertruck-solid` -- no regressions
3. Drafted solids have closed shells with no singular vertices
4. Draft angle is geometrically correct (face normals tilted by specified angle)
5. Zero draft angle returns the original solid topology
6. Invalid inputs produce appropriate errors
</verification>

<success_criteria>
- Draft/taper operations apply a specified draft angle to faces of a solid body
- Results produce valid B-rep output that passes topological validity checks
- Draft is geometrically correct relative to pull direction and neutral plane
- Error handling covers invalid angles and directions
</success_criteria>

<output>
After completion, create `.tendrion/phases/3-feature-completeness/3-4-SUMMARY.md`
</output>
