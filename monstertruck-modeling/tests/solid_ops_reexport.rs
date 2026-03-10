//! Verify that solid-ops feature re-exports are accessible from monstertruck-modeling.
#![cfg(feature = "solid-ops")]

use monstertruck_modeling::*;

/// Verify that shell_solid, offset_shell, draft_faces, DraftOptions, and DraftError
/// are accessible via monstertruck_modeling re-exports.
#[test]
fn solid_ops_reexports_accessible() {
    // Verify DraftOptions is constructible through the re-export.
    let _opts = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: Vector3::unit_z(),
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    // Verify DraftError is the right type (pattern match).
    let err: Result<(), DraftError> = Err(DraftError::InvalidAngle);
    assert!(err.is_err());

    // Verify draft_faces is callable (will fail at runtime, but compiles).
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    // Identify side faces.
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

    let opts = DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: pull,
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    // Call draft_faces through the re-export.
    let drafted = draft_faces(&cube, &side_indices, &opts);
    assert!(drafted.is_ok(), "draft_faces should succeed on a cube");
}

/// Verify that shell_solid is accessible (even if we can't fully call it
/// from here due to trait bounds, the symbol should resolve).
#[test]
fn shell_solid_reexport_resolves() {
    // The function signature requires OffsetCurve + OffsetSurface bounds,
    // so we just verify the name is in scope by taking a reference.
    let _fn_ref: fn(&Solid, f64, usize) -> Option<Solid> = shell_solid;
    // Don't actually call it -- the trait impls aren't available here.
    let _ = _fn_ref;
}

/// Verify offset_shell is accessible.
#[test]
fn offset_shell_reexport_resolves() {
    let _fn_ref: fn(&Shell, f64, usize) -> Option<Shell> = offset_shell;
    let _ = _fn_ref;
}
