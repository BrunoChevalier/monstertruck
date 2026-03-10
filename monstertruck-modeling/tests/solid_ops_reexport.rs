//! Verify that solid-ops feature re-exports are accessible from monstertruck-modeling.
#![cfg(feature = "solid-ops")]
#![allow(unused_imports)]

// These use-imports verify the re-exports compile. If any symbol is missing,
// this file will fail to compile.
use monstertruck_modeling::DraftError;
use monstertruck_modeling::DraftOptions;
use monstertruck_modeling::draft_faces;
use monstertruck_modeling::offset_shell;
use monstertruck_modeling::shell_solid;

use monstertruck_modeling::*;

/// Verify that DraftOptions, DraftError, and draft_faces are accessible
/// via monstertruck_modeling re-exports and work correctly.
#[test]
fn draft_reexports_accessible() {
    // Verify DraftOptions is constructible through the re-export.
    let opts = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    // Verify DraftError is the right type (pattern match).
    let err: std::result::Result<(), DraftError> = Err(DraftError::InvalidAngle);
    assert!(err.is_err());
    assert!(matches!(err, Err(DraftError::InvalidAngle)));

    // Build a cube and draft its side faces through the re-export.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let pull = Vector3::unit_z();
    let shell = &cube.boundaries()[0];
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

    // Call draft_faces through the re-export.
    let drafted = draft_faces(&cube, &side_indices, &opts);
    assert!(drafted.is_ok(), "draft_faces should succeed on a cube");
}

/// Verify that shell_solid and offset_shell are importable.
/// These are generic functions requiring OffsetCurve/OffsetSurface impls.
/// The `use` statements at file top verify the names exist in scope.
#[test]
fn shell_offset_names_importable() {
    // The use-imports at the top of this file are the actual verification.
    // If the re-exports are missing, this file won't compile.
    // Here we just assert true to have a named test.
    assert!(true, "shell_solid and offset_shell are importable");
}
