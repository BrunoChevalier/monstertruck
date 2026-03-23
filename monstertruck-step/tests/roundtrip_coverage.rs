use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;
use monstertruck_step::load::*;
use monstertruck_step::save::*;
use monstertruck_topology::shell::ShellCondition;
use std::f64::consts::PI;

// Use fully-qualified types for step-geometry-typed compressed shells to avoid
// ambiguity with the modeling type aliases.
type StepShell = monstertruck_topology::compress::CompressedShell<
    Point3,
    step_geometry::Curve3D,
    step_geometry::Surface,
>;

/// Builds a cube solid via extrusion from origin with the given side length.
fn make_cube(origin: Point3, side: f64) -> Solid {
    let v = builder::vertex(origin);
    let e = builder::extrude(&v, Vector3::unit_x() * side);
    let f = builder::extrude(&e, Vector3::unit_y() * side);
    builder::extrude(&f, Vector3::unit_z() * side)
}

/// Round-trips a single compressed shell through STEP export/import.
fn roundtrip_shell(shell: &CompressedShell) -> StepShell {
    let step_string =
        CompleteStepDisplay::new(StepModel::from(shell), Default::default()).to_string();
    let table = Table::from_step(&step_string).unwrap();
    let step_shell = table.shell.values().next().unwrap().clone();
    table.to_compressed_shell(&step_shell).unwrap()
}

/// Checks that bounding boxes of original and reimported shells match within tolerance.
fn bounding_box_matches(
    original: &CompressedShell,
    reimported: &StepShell,
    tolerance: f64,
) -> bool {
    let orig_bbox = original.triangulation(0.05).to_polygon().bounding_box();
    let reimp_bbox = reimported.triangulation(0.05).to_polygon().bounding_box();
    let orig_min = orig_bbox.min();
    let orig_max = orig_bbox.max();
    let reimp_min = reimp_bbox.min();
    let reimp_max = reimp_bbox.max();
    (orig_min.x - reimp_min.x).abs() < tolerance
        && (orig_min.y - reimp_min.y).abs() < tolerance
        && (orig_min.z - reimp_min.z).abs() < tolerance
        && (orig_max.x - reimp_max.x).abs() < tolerance
        && (orig_max.y - reimp_max.y).abs() < tolerance
        && (orig_max.z - reimp_max.z).abs() < tolerance
}

#[test]
fn roundtrip_cube() {
    let cube = make_cube(Point3::origin(), 1.0);
    let compressed = cube.compress();
    let shell = &compressed.boundaries[0];

    // Export as solid to get CLOSED_SHELL in the STEP output.
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();
    assert!(
        step_string.contains("CLOSED_SHELL"),
        "STEP output should contain CLOSED_SHELL"
    );

    let reimported = roundtrip_shell(shell);
    assert_eq!(
        reimported.faces.len(),
        6,
        "Re-imported cube should have 6 faces"
    );
    assert!(
        bounding_box_matches(shell, &reimported, 0.1),
        "Bounding boxes should match within tolerance"
    );
}

#[test]
fn roundtrip_cube_offset() {
    let cube = make_cube(Point3::new(1.0, 2.0, 3.0), 2.0);
    let compressed = cube.compress();
    let shell = &compressed.boundaries[0];

    let reimported = roundtrip_shell(shell);
    let bbox = reimported.triangulation(0.05).to_polygon().bounding_box();
    let min = bbox.min();
    let max = bbox.max();

    let tol = 0.1;
    assert!(
        (min.x - 1.0).abs() < tol,
        "min.x should be ~1.0, got {}",
        min.x
    );
    assert!(
        (min.y - 2.0).abs() < tol,
        "min.y should be ~2.0, got {}",
        min.y
    );
    assert!(
        (min.z - 3.0).abs() < tol,
        "min.z should be ~3.0, got {}",
        min.z
    );
    assert!(
        (max.x - 3.0).abs() < tol,
        "max.x should be ~3.0, got {}",
        max.x
    );
    assert!(
        (max.y - 4.0).abs() < tol,
        "max.y should be ~4.0, got {}",
        max.y
    );
    assert!(
        (max.z - 5.0).abs() < tol,
        "max.z should be ~5.0, got {}",
        max.z
    );
}

#[test]
fn roundtrip_compressed_solid() {
    let cube = make_cube(Point3::origin(), 1.0);
    let compressed = cube.compress();
    let original_face_count: usize = compressed.boundaries.iter().map(|s| s.faces.len()).sum();

    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();
    let table = Table::from_step(&step_string).unwrap();
    let reimported_face_count: usize = table
        .shell
        .values()
        .map(|step_shell| table.to_compressed_shell(step_shell).unwrap().faces.len())
        .sum();

    assert_eq!(
        original_face_count, reimported_face_count,
        "Face count should be preserved through round-trip"
    );
}

#[test]
fn roundtrip_boolean_result() {
    let cube1 = make_cube(Point3::origin(), 1.0);
    let cube2 = make_cube(Point3::new(0.5, 0.5, 0.5), 1.0);
    let result = monstertruck_solid::or(&cube1, &cube2, 0.05).unwrap();
    let compressed = result.compress();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();

    // Verify the boolean result exports to a valid STEP string.
    assert!(
        step_string.contains("CLOSED_SHELL"),
        "Boolean result STEP should contain CLOSED_SHELL"
    );
    ruststep::parser::parse(&step_string)
        .unwrap_or_else(|e| panic!("Boolean STEP string should parse: {e}"));

    // Re-import and verify at least one shell comes back.
    let table = Table::from_step(&step_string).unwrap();
    assert!(
        !table.shell.is_empty(),
        "Boolean result should produce at least one shell on reimport"
    );
}

#[test]
fn roundtrip_step_string_valid() {
    let cube = make_cube(Point3::origin(), 1.0);
    let compressed = cube.compress();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();

    // Verify the STEP string contains expected entities.
    assert!(
        step_string.contains("CLOSED_SHELL") || step_string.contains("MANIFOLD_SOLID_BREP"),
        "STEP string should contain CLOSED_SHELL or MANIFOLD_SOLID_BREP"
    );
    assert!(
        step_string.contains("CARTESIAN_POINT"),
        "STEP string should contain CARTESIAN_POINT"
    );
    assert!(
        step_string.contains("EDGE_CURVE"),
        "STEP string should contain EDGE_CURVE"
    );

    // Verify the STEP string parses correctly.
    ruststep::parser::parse(&step_string)
        .unwrap_or_else(|e| panic!("STEP string should parse successfully: {e}"));
}

#[test]
fn roundtrip_preserves_closedness() {
    let cube = make_cube(Point3::origin(), 1.0);
    let compressed = cube.compress();
    let shell = &compressed.boundaries[0];

    let reimported = roundtrip_shell(shell);
    let bdb = reimported.triangulation(0.01).to_polygon().bounding_box();
    let diag = bdb.max() - bdb.min();
    let r = diag.x.min(diag.y).min(diag.z);
    let mut poly = reimported.triangulation(0.01 * r).to_polygon();
    poly.put_together_same_attrs(TOLERANCE * 50.0)
        .remove_degenerate_faces();
    assert_eq!(
        poly.shell_condition(),
        ShellCondition::Closed,
        "Re-imported cube shell should be closed"
    );
}

#[test]
fn roundtrip_multiple_shapes() {
    let cube1 = make_cube(Point3::origin(), 1.0).compress();
    let cube2 = make_cube(Point3::new(5.0, 0.0, 0.0), 2.0).compress();
    let solids = [&cube1, &cube2];
    let models: StepModels<Point3, Curve, Surface> = solids.into_iter().collect();
    let step_string = CompleteStepDisplay::new(models, Default::default()).to_string();

    let table = Table::from_step(&step_string).unwrap();
    let shell_count = table.shell.len();
    assert!(
        shell_count >= 2,
        "Multiple shapes should produce at least 2 shells, got {shell_count}"
    );
}

#[test]
fn roundtrip_from_resource_file() {
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../resources/shape/cube.json");
    let json = std::fs::read(json_path).unwrap();
    let compressed: CompressedSolid = serde_json::from_reader(json.as_slice()).unwrap();

    let original_face_count: usize = compressed.boundaries.iter().map(|s| s.faces.len()).sum();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();

    let table = Table::from_step(&step_string).unwrap();
    let reimported_face_count: usize = table
        .shell
        .values()
        .map(|step_shell| table.to_compressed_shell(step_shell).unwrap().faces.len())
        .sum();

    assert_eq!(
        original_face_count, reimported_face_count,
        "Face count from resource file should be preserved"
    );

    // Verify bounding box is reasonable for a cube.
    let shell = &compressed.boundaries[0];
    let reimported_shell = table
        .to_compressed_shell(table.shell.values().next().unwrap())
        .unwrap();
    assert!(
        bounding_box_matches(shell, &reimported_shell, 0.1),
        "Bounding box from resource cube should match"
    );
}

#[test]
fn roundtrip_cylinder() {
    // Build a cylinder by revolving a rectangular face around the Y axis.
    // Rectangle in XY plane: x from 0 to 1 (radius), y from 0 to 2 (height).
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 2.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 2.0, 0.0));
    let wire: Wire = vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into();
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let cylinder: Solid =
        builder::revolve(&face, Point3::origin(), Vector3::unit_y(), Rad(2.0 * PI), 4);

    let compressed = cylinder.compress();

    // Export as solid to STEP and verify it contains a closed shell.
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();
    assert!(
        step_string.contains("CLOSED_SHELL"),
        "Cylinder STEP output should contain CLOSED_SHELL"
    );

    // Round-trip the full solid through STEP and verify geometry preservation.
    let table = Table::from_step(&step_string).unwrap();
    let reimported: Vec<StepShell> = table
        .shell
        .values()
        .map(|step_shell| table.to_compressed_shell(step_shell).unwrap())
        .collect();
    assert!(
        !reimported.is_empty(),
        "Cylinder should produce at least one shell on reimport"
    );

    // Verify bounding box preservation on the first shell.
    let shell = &compressed.boundaries[0];
    assert!(
        bounding_box_matches(shell, &reimported[0], 0.15),
        "Cylinder bounding boxes should match within tolerance"
    );

    // Verify the reimported face count is non-trivial (cylinder has lateral + cap faces).
    let reimported_face_count: usize = reimported.iter().map(|s| s.faces.len()).sum();
    assert!(
        reimported_face_count >= 4,
        "Cylinder should have at least 4 faces, got {reimported_face_count}"
    );
}
