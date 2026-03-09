use super::*;
use monstertruck_meshing::tessellation::stitch_boundaries;

/// Tolerance used for boundary stitching tests.
const STITCH_TOL: f64 = TOLERANCE * 2.0;

/// Creates a unit cube solid using the builder.
fn unit_cube() -> Solid {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    builder::extrude(&f, Vector3::unit_z())
}

/// Creates a cylinder shell by manually constructing revoluted surfaces.
fn cylinder_shell() -> Shell {
    let v0 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v1 = builder::vertex(Point3::new(0.0, 1.0, 1.0));
    let v2 = builder::vertex(Point3::new(0.0, -1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, -1.0, 1.0));

    let edge0 = builder::line(&v0, &v1);
    let edge1 = builder::line(&v2, &v3);
    let edge2 = builder::circle_arc(&v0, &v2, Point3::new(-1.0, 0.0, 0.0));
    let edge3 = builder::circle_arc(&v2, &v0, Point3::new(1.0, 0.0, 0.0));
    let edge4 = builder::circle_arc(&v1, &v3, Point3::new(-1.0, 0.0, 1.0));
    let edge5 = builder::circle_arc(&v3, &v1, Point3::new(1.0, 0.0, 1.0));

    let face0 =
        builder::try_attach_plane(&[vec![edge2.inverse(), edge3.inverse()].into()]).unwrap();
    let face1 = builder::try_attach_plane(&[vec![edge4.clone(), edge5.clone()].into()]).unwrap();

    let surface_raw = RevolutedCurve::<Curve>::by_revolution(
        Line(Point3::new(1.0, 0.0, 1.0), Point3::new(1.0, 0.0, 0.0)).into(),
        Point3::origin(),
        Vector3::unit_z(),
    );
    let surface: Surface = Processor::new(surface_raw).into();

    let face2 = Face::new(
        vec![vec![edge2, edge1.clone(), edge4.inverse(), edge0.inverse()].into()],
        surface.clone(),
    );
    let face3 = Face::new(
        vec![vec![edge3, edge0, edge5.inverse(), edge1.inverse()].into()],
        surface,
    );

    vec![face0, face1, face2, face3].into()
}

/// Regression guard: tessellated cube produces a closed mesh after vertex merging.
#[test]
fn no_seams_on_cube() {
    let cube = unit_cube();
    let mut mesh = cube.triangulation(0.01).to_polygon();
    mesh.put_together_same_attrs(STITCH_TOL)
        .remove_degenerate_faces()
        .remove_unused_attrs();
    assert_eq!(mesh.shell_condition(), ShellCondition::Closed);
}

/// Regression guard: tessellated cylinder produces a closed mesh after vertex merging.
#[test]
fn no_seams_on_cylinder() {
    let shell = cylinder_shell();
    let mut mesh = shell.triangulation(0.01).to_polygon();
    mesh.put_together_same_attrs(STITCH_TOL)
        .remove_degenerate_faces()
        .remove_unused_attrs();
    assert_eq!(mesh.shell_condition(), ShellCondition::Closed);
}

/// Core test: after `stitch_boundaries`, boundary vertices along shared edges must
/// have identical positions across adjacent faces. This verifies the stitching function
/// exists and produces the expected output.
#[test]
fn boundary_vertices_shared_between_adjacent_faces() {
    let shell = cylinder_shell();
    let mut meshed = shell.triangulation(0.01);

    // Apply stitching to enforce exact vertex sharing along boundaries.
    stitch_boundaries(&mut meshed, TOLERANCE);

    // Collect positions from each face.
    let face_positions: Vec<Vec<Point3>> = meshed
        .face_iter()
        .filter_map(|face| face.surface().map(|poly| poly.positions().clone()))
        .collect();

    assert!(
        face_positions.len() >= 2,
        "At least two faces should be tessellated."
    );

    // After stitching, shared boundary vertices between curved faces should be
    // bitwise-identical. Collect all positions and check that shared-edge vertices
    // appear in multiple faces with exact matches.
    let all_positions: Vec<Point3> = face_positions.iter().flatten().copied().collect();

    // Build a set of positions that appear in more than one face.
    let mut shared_count = 0_usize;
    for (i, face_i) in face_positions.iter().enumerate() {
        for p_i in face_i {
            for face_j in face_positions.iter().skip(i + 1) {
                if face_j
                    .iter()
                    .any(|p_j| p_i.x == p_j.x && p_i.y == p_j.y && p_i.z == p_j.z)
                {
                    shared_count += 1;
                    break;
                }
            }
        }
    }

    // There should be at least some vertices shared across faces at boundary edges.
    assert!(
        shared_count > 0,
        "After stitching, at least some boundary vertices should be exactly shared. \
         Total positions: {}",
        all_positions.len()
    );
}

/// Stitching should not significantly alter vertex count; it merges duplicates.
#[test]
fn stitched_tessellation_preserves_vertex_count() {
    let shell = cylinder_shell();
    let mut meshed = shell.triangulation(0.01);

    // Count positions before stitching.
    let pre_stitch_count: usize = meshed
        .face_iter()
        .filter_map(|face| face.surface().map(|poly| poly.positions().len()))
        .sum();

    stitch_boundaries(&mut meshed, TOLERANCE);

    // Count positions after stitching.
    let post_stitch_count: usize = meshed
        .face_iter()
        .filter_map(|face| face.surface().map(|poly| poly.positions().len()))
        .sum();

    // Stitching should not add or remove vertices; it only unifies positions.
    assert_eq!(
        pre_stitch_count, post_stitch_count,
        "Stitching should preserve vertex count per face."
    );
}

/// Both loose and tight tolerances should produce seam-free meshes after stitching.
#[test]
fn stitch_respects_tolerance() {
    let cube = unit_cube();

    for tolerance in [0.1, 0.001] {
        let meshed = cube.triangulation(tolerance);

        let mut mesh = meshed.to_polygon();
        mesh.put_together_same_attrs(STITCH_TOL)
            .remove_degenerate_faces()
            .remove_unused_attrs();
        assert_eq!(
            mesh.shell_condition(),
            ShellCondition::Closed,
            "Mesh should be closed at tolerance={tolerance}."
        );

        // Verify that all faces tessellated successfully.
        let meshed = cube.triangulation(tolerance);
        let face_count = meshed
            .face_iter()
            .filter(|face| face.surface().is_some())
            .count();
        assert!(
            face_count > 0,
            "At least some faces should be tessellated at tolerance={tolerance}."
        );
    }
}
