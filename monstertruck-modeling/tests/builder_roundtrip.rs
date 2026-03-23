use monstertruck_modeling::errors::Error;
use monstertruck_modeling::*;
use std::f64::consts::PI;

fn rect_wire(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, y0, 0.0));
    let v1 = builder::vertex(Point3::new(x1, y0, 0.0));
    let v2 = builder::vertex(Point3::new(x1, y1, 0.0));
    let v3 = builder::vertex(Point3::new(x0, y1, 0.0));
    wire![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
}

fn rect_wire_xz(x0: f64, z0: f64, x1: f64, z1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, 0.0, z0));
    let v1 = builder::vertex(Point3::new(x1, 0.0, z0));
    let v2 = builder::vertex(Point3::new(x1, 0.0, z1));
    let v3 = builder::vertex(Point3::new(x0, 0.0, z1));
    wire![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
}

#[test]
fn extrude_vertex_to_edge() {
    let v = builder::vertex(Point3::new(1.0, 2.0, 3.0));
    let edge: Edge = builder::extrude(&v, Vector3::new(1.0, 0.0, 0.0));
    let front = edge.front().point();
    let back = edge.back().point();
    assert!(front.near(&Point3::new(1.0, 2.0, 3.0)));
    assert!(back.near(&Point3::new(2.0, 2.0, 3.0)));
}

#[test]
fn extrude_edge_to_face() {
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let edge: Edge = builder::line(&v0, &v1);
    let face: Face = builder::extrude(&edge, Vector3::new(0.0, 1.0, 0.0));
    assert_eq!(face.boundaries()[0].len(), 4);
}

#[test]
fn extrude_face_to_solid() {
    let wire = rect_wire(0.0, 0.0, 1.0, 1.0);
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let solid: Solid = builder::extrude(&face, Vector3::new(0.0, 0.0, 1.0));
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.len(), 6);
    // Count unique edges by ID.
    let unique_edges: std::collections::HashSet<_> = solid.edge_iter().map(|e| e.id()).collect();
    assert_eq!(unique_edges.len(), 12);
    // Count unique vertices by ID.
    let unique_verts: std::collections::HashSet<_> = solid.vertex_iter().map(|v| v.id()).collect();
    assert_eq!(unique_verts.len(), 8);
    assert!(solid.is_geometric_consistent());
}

#[test]
fn revolve_vertex_to_circle() {
    let v = builder::vertex(Point3::new(2.0, 0.0, 0.0));
    let wire: Wire = builder::revolve(&v, Point3::origin(), Vector3::unit_y(), Rad(2.0 * PI), 4);
    assert_eq!(wire.len(), 4);
}

#[test]
fn revolve_edge_to_face() {
    let v0 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let edge: Edge = builder::line(&v0, &v1);
    // Partial revolve (90 degrees, division=1) produces a shell with one face.
    let shell: Shell =
        builder::revolve(&edge, Point3::origin(), Vector3::unit_y(), Rad(PI / 2.0), 1);
    assert_eq!(shell.len(), 1);
    assert_eq!(shell[0].boundaries()[0].len(), 4);
}

#[test]
fn revolve_face_to_solid() {
    // Rectangle in XZ plane offset from Y axis, revolve 360 degrees.
    let wire = rect_wire_xz(2.0, -0.5, 3.0, 0.5);
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let solid: Solid =
        builder::revolve(&face, Point3::origin(), Vector3::unit_y(), Rad(2.0 * PI), 4);
    assert!(solid.is_geometric_consistent());
    assert_eq!(
        solid.boundaries()[0].shell_condition(),
        ShellCondition::Closed,
    );
}

#[test]
fn revolve_wire_degenerate_axis() {
    // One endpoint on the Y axis.
    let v0 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v1 = builder::vertex(Point3::new(0.0, 0.0, 1.0));
    let v2 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let wire: Wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
    let shell: Shell =
        builder::revolve_wire(&wire, Point3::origin(), Vector3::unit_y(), Rad(2.0 * PI), 4);
    // Degenerate edges collapsed: first face has 3 boundary edges, not 4.
    assert_eq!(shell[0].boundaries()[0].len(), 3);
}

#[test]
fn homotopy_two_lines() {
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 1.0, 1.0));
    let line0: Edge = builder::line(&v0, &v1);
    let line1: Edge = builder::line(&v2, &v3);
    let face: Face = builder::homotopy(&line0, &line1);
    assert_eq!(face.boundaries()[0].len(), 4);
}

#[test]
fn wire_homotopy_matching_edges() {
    let v00 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v01 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v02 = builder::vertex(Point3::new(2.0, 0.0, 0.0));
    let v10 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v11 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v12 = builder::vertex(Point3::new(2.0, 1.0, 0.0));
    let wire0: Wire = wire![builder::line(&v00, &v01), builder::line(&v01, &v02),];
    let wire1: Wire = wire![builder::line(&v10, &v11), builder::line(&v11, &v12),];
    let shell: Shell = builder::try_wire_homotopy(&wire0, &wire1).unwrap();
    // Face count equals edge count per wire.
    assert_eq!(shell.len(), 2);
}

#[test]
fn wire_homotopy_mismatch_error() {
    let v00 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v01 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v10 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v11 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v12 = builder::vertex(Point3::new(2.0, 1.0, 0.0));
    let wire0: Wire = wire![builder::line(&v00, &v01),];
    let wire1: Wire = wire![builder::line(&v10, &v11), builder::line(&v11, &v12),];
    assert!(matches!(
        builder::try_wire_homotopy::<Curve, Surface>(&wire0, &wire1),
        Err(Error::NotSameNumberOfEdges),
    ));
}

#[test]
fn skin_wires_three_strips() {
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v4 = builder::vertex(Point3::new(0.0, 2.0, 0.0));
    let v5 = builder::vertex(Point3::new(1.0, 2.0, 0.0));
    let w0: Wire = wire![builder::line(&v0, &v1)];
    let w1: Wire = wire![builder::line(&v2, &v3)];
    let w2: Wire = wire![builder::line(&v4, &v5)];
    let shell: Shell = builder::try_skin_wires(&[w0, w1, w2]).unwrap();
    // 3 wires, 1 edge each => (3-1)*1 = 2 faces.
    assert_eq!(shell.len(), 2);
}

#[test]
fn skin_wires_too_few_error() {
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let w0: Wire = wire![builder::line(&v0, &v1)];
    assert!(builder::try_skin_wires::<Curve, Surface>(&[w0]).is_err());
}

#[test]
fn transformed_vertex_position() {
    let v = builder::vertex(Point3::new(1.0, 2.0, 3.0));
    let moved: Vertex = builder::translated(&v, Vector3::new(10.0, 20.0, 30.0));
    assert!(moved.point().near(&Point3::new(11.0, 22.0, 33.0)));
}

#[test]
fn rotated_edge_consistency() {
    let v0 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(0.0, 0.0, 1.0));
    let edge: Edge = builder::line(&v0, &v1);
    let rotated: Edge = builder::rotated(&edge, Point3::origin(), Vector3::unit_z(), Rad(PI / 2.0));
    assert!(rotated.front().point().near(&Point3::new(0.0, 1.0, 0.0)));
    assert!(rotated.back().point().near(&Point3::new(0.0, 0.0, 1.0)));
}

#[test]
fn scaled_face_bounding_box() {
    let wire = rect_wire(0.0, 0.0, 1.0, 1.0);
    let face: Face = builder::try_attach_plane(vec![wire]).unwrap();
    let scaled: Face = builder::scaled(&face, Point3::origin(), Vector3::new(2.0, 3.0, 1.0));
    let pts: Vec<_> = scaled.vertex_iter().map(|v| v.point()).collect();
    // After scaling by (2, 3, 1) from origin, unit square corners become
    // (0,0), (2,0), (2,3), (0,3).
    assert!(pts.iter().any(|p| p.near(&Point3::new(0.0, 0.0, 0.0))));
    assert!(pts.iter().any(|p| p.near(&Point3::new(2.0, 0.0, 0.0))));
    assert!(pts.iter().any(|p| p.near(&Point3::new(2.0, 3.0, 0.0))));
    assert!(pts.iter().any(|p| p.near(&Point3::new(0.0, 3.0, 0.0))));
}

#[test]
fn sweep_rail_face() {
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    let rail = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    );
    let face: Face = builder::try_sweep_rail(&profile, &rail, 3).unwrap();
    assert_eq!(face.boundaries()[0].len(), 4);
}
