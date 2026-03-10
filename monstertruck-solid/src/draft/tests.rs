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

/// Helper: create a 2x3x4 box solid.
fn non_unit_box() -> Solid {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::new(2.0, 0.0, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, 3.0, 0.0));
    let solid: Solid = builder::extrude(&f, Vector3::new(0.0, 0.0, 4.0));
    solid
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

    let drafted =
        draft_faces(&cube, &side_indices, &options).expect("zero-angle draft should succeed");
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

#[test]
fn draft_cube_angle_verification() {
    let cube = unit_cube();
    let side_indices = side_face_indices(&cube);
    let draft_angle = 5.0_f64.to_radians();

    let options = DraftOptions {
        angle: draft_angle,
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let drafted = draft_faces(&cube, &side_indices, &options).expect("draft should succeed");
    let shell = &drafted.boundaries()[0];
    let pull = Vector3::unit_z();

    // Identify the drafted side faces (those not parallel to pull) and verify
    // each has its normal tilted by ~5 degrees from the original perpendicular.
    let mut drafted_side_count = 0;
    for (i, face) in shell.iter().enumerate() {
        let normal = ParametricSurface3D::normal(&face.surface(), 0.5, 0.5);
        let dot = normal.dot(pull).abs();
        // Skip top/bottom faces (normal nearly parallel to pull direction).
        if dot > 0.5 {
            continue;
        }
        drafted_side_count += 1;
        let angle_to_pull = dot.acos();
        // Originally PI/2, now should differ by ~draft_angle.
        let deviation = (angle_to_pull - std::f64::consts::FRAC_PI_2).abs();
        assert!(
            (deviation - draft_angle).abs() < 0.02,
            "Face {i} angle deviation {deviation:.4} should be ~{draft_angle:.4}"
        );
    }
    assert_eq!(drafted_side_count, 4, "Expected 4 drafted side faces");
}

#[test]
fn draft_cube_neutral_plane_fixed() {
    let cube = unit_cube();
    let side_indices = side_face_indices(&cube);

    let neutral_z = 0.5;
    let options = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, neutral_z),
            Point3::new(1.0, 0.0, neutral_z),
            Point3::new(0.0, 1.0, neutral_z),
        ),
    };

    let drafted = draft_faces(&cube, &side_indices, &options).expect("draft should succeed");
    let compressed = drafted.compress();

    // Vertices at the original z = 0 and z = 1 should remain at those heights since
    // the draft pivots around the neutral plane and the top/bottom faces are not drafted.
    // Access vertices through the compressed solid's boundary shell.
    let shell_compressed = &compressed.boundaries[0];
    for v in &shell_compressed.vertices {
        let z = v.z;
        // Vertices should still be near the original z = 0 or z = 1 planes.
        assert!(
            (z - 0.0).abs() < 0.01 || (z - 1.0).abs() < 0.01,
            "Vertex z = {z:.4} should be near 0.0 or 1.0"
        );
    }
}

#[test]
fn draft_10_degree_larger_angle() {
    let cube = unit_cube();
    let side_indices = side_face_indices(&cube);
    let draft_angle = 10.0_f64.to_radians();

    let options = DraftOptions {
        angle: draft_angle,
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let drafted = draft_faces(&cube, &side_indices, &options).expect("draft should succeed");
    let shell = &drafted.boundaries()[0];
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.singular_vertices().is_empty());

    // Verify face angles.
    let pull = Vector3::unit_z();
    let mut drafted_side_count = 0;
    for (i, face) in shell.iter().enumerate() {
        let normal = ParametricSurface3D::normal(&face.surface(), 0.5, 0.5);
        let dot = normal.dot(pull).abs();
        if dot > 0.5 {
            continue;
        }
        drafted_side_count += 1;
        let angle_to_pull = dot.acos();
        let deviation = (angle_to_pull - std::f64::consts::FRAC_PI_2).abs();
        assert!(
            (deviation - draft_angle).abs() < 0.02,
            "Face {i} angle deviation {deviation:.4} should be ~{draft_angle:.4}"
        );
    }
    assert_eq!(drafted_side_count, 4, "Expected 4 drafted side faces");
}

#[test]
fn draft_box_non_unit() {
    // Build a 2x3x4 box.
    let box_solid = non_unit_box();

    // Identify side faces (normals perpendicular to z-axis).
    let pull = Vector3::unit_z();
    let shell = &box_solid.boundaries()[0];
    let side_indices: Vec<usize> = shell
        .iter()
        .enumerate()
        .filter_map(|(i, face)| {
            let normal = ParametricSurface3D::normal(&face.surface(), 0.0, 0.0);
            if normal.dot(pull).abs() < 0.1 {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(side_indices.len(), 4, "Expected 4 side faces on box");

    let options = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: pull,
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(1.0, 0.0, 2.0),
            Point3::new(0.0, 1.0, 2.0),
        ),
    };

    let drafted = draft_faces(&box_solid, &side_indices, &options).expect("draft should succeed");
    let drafted_shell = &drafted.boundaries()[0];

    // Topological validity.
    assert_eq!(drafted_shell.shell_condition(), ShellCondition::Closed);
    assert!(
        drafted_shell.singular_vertices().is_empty(),
        "Non-unit box draft has singular vertices"
    );
    assert_eq!(drafted_shell.len(), 6);
}
