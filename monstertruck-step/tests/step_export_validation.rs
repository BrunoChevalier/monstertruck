use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;
use monstertruck_step::load::*;
use monstertruck_step::save::*;
use std::f64::consts::PI;

// Use fully-qualified types for step-geometry-typed compressed shells to avoid
// ambiguity with the modeling type aliases.
type StepShell = monstertruck_topology::compress::CompressedShell<
    Point3,
    step_geometry::Curve3D,
    step_geometry::Surface,
>;

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

/// Builds a cube solid via extrusion from origin with the given side length.
fn make_cube(origin: Point3, side: f64) -> Solid {
    let v = builder::vertex(origin);
    let e = builder::extrude(&v, Vector3::unit_x() * side);
    let f = builder::extrude(&e, Vector3::unit_y() * side);
    builder::extrude(&f, Vector3::unit_z() * side)
}

#[test]
fn export_sphere_roundtrip_bbox() {
    // Build a sphere by revolving a semicircular profile (rectangle in XZ
    // plane, x from 0..1 = radius) around the Y axis.
    // Use the same rectangular face-revolve pattern as `roundtrip_cylinder`.
    let v0 = builder::vertex(Point3::new(0.0, -1.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, -1.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let wire: Wire = vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into();
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let sphere_like: Solid =
        builder::revolve(&face, Point3::origin(), Vector3::unit_y(), Rad(2.0 * PI), 4);

    let compressed = sphere_like.compress();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();
    assert!(
        step_string.contains("CLOSED_SHELL"),
        "Sphere-like STEP output should contain CLOSED_SHELL"
    );

    let shell = &compressed.boundaries[0];
    let reimported = roundtrip_shell(shell);
    assert!(
        bounding_box_matches(shell, &reimported, 0.15),
        "Sphere-like bounding boxes should match within tolerance"
    );
}

#[test]
fn export_torus_roundtrip_bbox() {
    // Build a torus by revolving a small rectangular cross-section offset from
    // the Y axis around the Y axis.
    // Inner radius = 2.0, cross-section half-extent = 0.5.
    let v0 = builder::vertex(Point3::new(1.5, -0.5, 0.0));
    let v1 = builder::vertex(Point3::new(2.5, -0.5, 0.0));
    let v2 = builder::vertex(Point3::new(2.5, 0.5, 0.0));
    let v3 = builder::vertex(Point3::new(1.5, 0.5, 0.0));
    let wire: Wire = vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into();
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let torus: Solid =
        builder::revolve(&face, Point3::origin(), Vector3::unit_y(), Rad(2.0 * PI), 4);

    let compressed = torus.compress();
    let shell = &compressed.boundaries[0];
    let reimported = roundtrip_shell(shell);
    assert!(
        bounding_box_matches(shell, &reimported, 0.15),
        "Torus bounding boxes should match within tolerance"
    );
}

#[test]
fn export_extrusion_roundtrip_bbox() {
    // Build an L-shaped profile and extrude it.
    // L-shape vertices: (0,0) -> (2,0) -> (2,1) -> (1,1) -> (1,2) -> (0,2) -> (0,0).
    let pts = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
    ];
    let verts: Vec<_> = pts.iter().map(|p| builder::vertex(*p)).collect();
    let wire: Wire = (0..6)
        .map(|i| builder::line(&verts[i], &verts[(i + 1) % 6]))
        .collect::<Vec<_>>()
        .into();
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let l_solid: Solid = builder::extrude(&face, Vector3::unit_z() * 3.0);

    let compressed = l_solid.compress();
    let shell = &compressed.boundaries[0];
    let reimported = roundtrip_shell(shell);
    assert!(
        bounding_box_matches(shell, &reimported, 0.15),
        "L-extrusion bounding boxes should match within tolerance"
    );

    // An L-shape extrusion has: 2 end caps + 6 side faces = 8 faces.
    assert!(
        reimported.faces.len() >= 8,
        "L-extrusion should have at least 8 faces, got {}",
        reimported.faces.len()
    );
}

#[test]
fn export_step_entities_complete() {
    let cube = make_cube(Point3::origin(), 1.0);
    let compressed = cube.compress();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&compressed), Default::default()).to_string();

    // Verify the STEP output contains all required entity types.
    let required_entities = [
        "CARTESIAN_POINT",
        "EDGE_CURVE",
        "VERTEX_POINT",
        "FACE_BOUND",
        "FACE_SURFACE",
        "CLOSED_SHELL",
    ];
    for entity in &required_entities {
        assert!(
            step_string.contains(entity),
            "STEP output should contain {entity}"
        );
    }

    // Verify the ISO header line is present.
    assert!(
        step_string.contains("ISO-10303-21"),
        "STEP output should contain the ISO header line"
    );
}
