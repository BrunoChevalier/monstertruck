//! Cross-feature integration tests for Phase 3.
//!
//! Each test creates a shape, applies an operation, verifies topology,
//! compresses to `CompressedSolid`/`CompressedShell`, exports to STEP via
//! `CompleteStepDisplay`, and parses the STEP string with `ruststep`.
//!
//! Note: `shell_solid` cannot be tested from integration tests because
//! the `OffsetCurve`/`OffsetSurface` impls for modeling types are only
//! available in the crate's `#[cfg(test)]` module (circular dependency
//! prevents making monstertruck-modeling a regular dep). The shell STEP
//! export test constructs a multi-boundary solid manually.

use monstertruck_modeling::*;
use monstertruck_step::save::*;
use monstertruck_topology::compress::CompressedSolid;
use monstertruck_topology::shell::ShellCondition;

/// Helper: build a unit cube solid using the modeling builder.
fn make_cube() -> Solid {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    builder::extrude(&f, Vector3::unit_z())
}

/// Helper: build a cube at a given origin with a given side length.
fn make_cube_at(origin: Point3, side: f64) -> Solid {
    let v = builder::vertex(origin);
    let e = builder::extrude(&v, Vector3::new(side, 0.0, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, side, 0.0));
    builder::extrude(&f, Vector3::new(0.0, 0.0, side))
}

macro_rules! resource_dir {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../resources/shape/")
    };
}

/// Load a compressed solid from a JSON resource file and extract it.
fn load_solid(filename: &str) -> Solid {
    let path = format!("{}{}", resource_dir!(), filename);
    let json = std::fs::read(&path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));
    let compressed: CompressedSolid<Point3, Curve, Surface> =
        serde_json::from_slice(&json).unwrap();
    Solid::extract(compressed).expect("failed to extract solid")
}

/// Helper: export a compressed solid to STEP and verify the result parses.
fn export_and_parse_solid(solid: &Solid) {
    let compressed = solid.compress();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();
    assert!(
        step_string.contains("MANIFOLD_SOLID_BREP") || step_string.contains("CLOSED_SHELL"),
        "STEP output must contain solid/shell entities"
    );
    ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
        panic!("Failed to parse STEP output\n[Error]\n{e}\n[STEP]\n{step_string}")
    });
}

/// Helper: export a compressed shell to STEP and verify the result parses.
fn export_and_parse_shell(shell: &Shell) {
    let compressed = shell.compress();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();
    assert!(
        step_string.contains("CLOSED_SHELL") || step_string.contains("OPEN_SHELL"),
        "STEP output must contain shell entities"
    );
    ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
        panic!("Failed to parse STEP output\n[Error]\n{e}\n[STEP]\n{step_string}")
    });
}

/// Load a boolean-result solid (punched cube from shapeops), chamfer one
/// edge of the result, compress, export to STEP, and parse.
#[test]
fn boolean_then_chamfer_step_export() {
    // Load a known-good boolean AND result.
    let boolean_result = load_solid("punched-cube-shapeops.json");

    // Grab the outer shell and chamfer one edge.
    let mut shells = boolean_result.into_boundaries();
    let shell = &mut shells[0];
    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "boolean result must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.05),
        profile: monstertruck_solid::FilletProfile::Chamfer,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts))
        .expect("chamfer after boolean failed");

    // Topological validity.
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.singular_vertices().is_empty());

    // Export chamfered shell to STEP and parse.
    export_and_parse_shell(shell);
}

/// Construct a hollow (multi-boundary) solid and export to STEP.
///
/// Since `shell_solid` requires `OffsetCurve`/`OffsetSurface` trait impls
/// that are only available in the crate's `#[cfg(test)]` module, this test
/// manually constructs a solid with two boundary shells (outer and inner)
/// to verify that multi-boundary solids export to valid STEP files.
#[test]
fn shell_then_step_export() {
    // Build an outer cube [0,1]^3 and an inner cube [0.2, 0.8]^3.
    let outer = make_cube();
    let inner_cube = make_cube_at(Point3::new(0.2, 0.2, 0.2), 0.6);

    // Extract shells and invert the inner one to create inward-facing normals.
    let outer_shell = outer.boundaries()[0].clone();
    let mut inner_shell = inner_cube.boundaries()[0].clone();
    inner_shell.face_iter_mut().for_each(|f| {
        f.invert();
    });

    // Create a hollow solid with two boundary shells.
    let hollow = Solid::new_unchecked(vec![outer_shell, inner_shell]);
    assert_eq!(hollow.boundaries().len(), 2);

    // Verify outer shell is closed.
    assert_eq!(
        hollow.boundaries()[0].shell_condition(),
        ShellCondition::Closed
    );

    // Export to STEP and parse.
    export_and_parse_solid(&hollow);
}

/// Draft a cube's side faces by 5 degrees, compress, export to STEP, and parse.
#[test]
fn draft_then_step_export() {
    let cube = make_cube();

    // Identify side faces (normals perpendicular to pull direction z).
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
    assert_eq!(side_indices.len(), 4, "Expected 4 side faces");

    let options = monstertruck_solid::DraftOptions {
        angle: 5.0_f64.to_radians(),
        pull_direction: pull,
        neutral_plane: Plane::new(
            Point3::new(0.0, 0.0, 0.5),
            Point3::new(1.0, 0.0, 0.5),
            Point3::new(0.0, 1.0, 0.5),
        ),
    };

    let drafted = monstertruck_solid::draft_faces(&cube, &side_indices, &options)
        .expect("draft_faces failed");

    // Topological validity.
    assert_eq!(drafted.boundaries().len(), 1);
    let drafted_shell = &drafted.boundaries()[0];
    assert_eq!(drafted_shell.shell_condition(), ShellCondition::Closed);
    assert!(drafted_shell.singular_vertices().is_empty());

    // Export to STEP and parse.
    export_and_parse_solid(&drafted);
}

/// Chamfer a cube edge, compress, export to STEP, and parse.
#[test]
fn chamfer_cube_step_export() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.15),
        profile: monstertruck_solid::FilletProfile::Chamfer,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts))
        .expect("chamfer failed");

    // Topological validity.
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.singular_vertices().is_empty());

    // Export chamfered shell to STEP and parse.
    export_and_parse_shell(shell);
}
