use monstertruck_geometry::prelude::*;
use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;
use std::f64::consts::PI;

use super::{DraftError, DraftOptions, draft_faces};

/// Helper: create a unit cube solid.
fn unit_cube() -> Solid {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    builder::extrude(&f, Vector3::unit_z())
}

/// Helper: indices of the 4 side faces of an extruded cube (not top/bottom).
/// For a cube built by extruding along z, face indices 2..6 are the side faces.
fn side_face_indices(solid: &Solid) -> Vec<usize> {
    let shell = &solid.boundaries()[0];
    let pull = Vector3::unit_z();
    shell
        .iter()
        .enumerate()
        .filter_map(|(i, face)| {
            let normal = ParametricSurface3D::normal(&face.surface(), 0.0, 0.0);
            let dot = normal.dot(pull).abs();
            // Side faces have normals perpendicular to the pull direction.
            if dot < 0.1 { Some(i) } else { None }
        })
        .collect()
}

#[test]
fn draft_cube_faces_valid_topology() {
    let cube = unit_cube();
    let side_indices = side_face_indices(&cube);
    assert_eq!(side_indices.len(), 4, "Expected 4 side faces");

    let options = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let drafted = draft_faces(&cube, &side_indices, &options).expect("draft should succeed");

    // Drafted solid should have 1 boundary shell with 6 faces.
    assert_eq!(drafted.boundaries().len(), 1);
    let shell = &drafted.boundaries()[0];
    assert_eq!(shell.len(), 6);

    // Shell should be closed with no singular vertices.
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(
        shell.singular_vertices().is_empty(),
        "Drafted shell has singular vertices"
    );
}

#[test]
fn draft_zero_angle_returns_original() {
    let cube = unit_cube();
    let side_indices = side_face_indices(&cube);

    let options = DraftOptions {
        angle: 0.0,
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let drafted = draft_faces(&cube, &side_indices, &options).expect("zero-angle draft should succeed");
    assert_eq!(drafted.boundaries().len(), 1);
    assert_eq!(drafted.boundaries()[0].len(), 6);
}

#[test]
fn draft_invalid_angle_error() {
    let cube = unit_cube();
    let options = DraftOptions {
        angle: PI,
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let result = draft_faces(&cube, &[0, 1, 2, 3], &options);
    assert!(result.is_err());
    assert!(
        matches!(result, Err(DraftError::InvalidAngle)),
        "Expected InvalidAngle error"
    );
}

#[test]
fn draft_serialization_round_trip() {
    let cube = unit_cube();
    let side_indices = side_face_indices(&cube);

    let options = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let drafted = draft_faces(&cube, &side_indices, &options).expect("draft should succeed");
    let compressed = drafted.compress();
    let json = serde_json::to_vec(&compressed).unwrap();
    let restored: monstertruck_topology::compress::CompressedSolid<Point3, Curve, Surface> =
        serde_json::from_slice(&json).unwrap();
    let restored_solid = Solid::extract(restored).unwrap();
    assert_eq!(
        restored_solid.boundaries()[0].len(),
        drafted.boundaries()[0].len()
    );
}
