//! Integration tests for planar profile normalization and solid construction.

use monstertruck_modeling::*;

/// Helper: builds a CCW rectangular wire in the XY plane at z=0.
fn rect_wire(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, y0, 0.0));
    let v1 = builder::vertex(Point3::new(x1, y0, 0.0));
    let v2 = builder::vertex(Point3::new(x1, y1, 0.0));
    let v3 = builder::vertex(Point3::new(x0, y1, 0.0));
    vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into()
}

/// Helper: builds a CW rectangular wire.
fn rect_wire_cw(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    rect_wire(x0, y0, x1, y1).inverse()
}

/// Helper: builds a triangular wire in the XY plane.
fn triangle_wire(p0: Point3, p1: Point3, p2: Point3) -> Wire {
    let v0 = builder::vertex(p0);
    let v1 = builder::vertex(p1);
    let v2 = builder::vertex(p2);
    vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v0),
    ]
    .into()
}

// -- Phase 0 fixture tests: nested holes --

#[test]
fn nested_holes_two_level() {
    let outer = rect_wire(-5.0, -5.0, 5.0, 5.0);
    let hole = rect_wire(-2.0, -2.0, 2.0, 2.0);
    let face: Face = profile::attach_plane_normalized(vec![outer, hole]).unwrap();
    assert_eq!(face.boundaries().len(), 2);
}

#[test]
fn nested_holes_four_corners() {
    let outer = rect_wire(-10.0, -10.0, 10.0, 10.0);
    let h1 = rect_wire(-9.0, -9.0, -6.0, -6.0);
    let h2 = rect_wire(6.0, -9.0, 9.0, -6.0);
    let h3 = rect_wire(-9.0, 6.0, -6.0, 9.0);
    let h4 = rect_wire(6.0, 6.0, 9.0, 9.0);
    let face: Face = profile::attach_plane_normalized(vec![outer, h1, h2, h3, h4]).unwrap();
    assert_eq!(face.boundaries().len(), 5);
}

// -- Phase 0 fixture tests: mixed CW/CCW inputs --

#[test]
fn all_ccw_auto_normalized() {
    // All wires given as CCW; holes should be flipped automatically.
    let outer = rect_wire(-3.0, -3.0, 3.0, 3.0);
    let hole = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let face: Face = profile::attach_plane_normalized(vec![outer, hole]).unwrap();
    assert_eq!(face.boundaries().len(), 2);
}

#[test]
fn all_cw_auto_normalized() {
    // All wires given as CW; outer should be flipped, holes kept.
    let outer = rect_wire_cw(-3.0, -3.0, 3.0, 3.0);
    let hole = rect_wire_cw(-1.0, -1.0, 1.0, 1.0);
    let face: Face = profile::attach_plane_normalized(vec![outer, hole]).unwrap();
    assert_eq!(face.boundaries().len(), 2);
}

// -- Phase 0 fixture: near-degenerate tiny holes --

#[test]
fn tiny_hole_survives() {
    let outer = rect_wire(-100.0, -100.0, 100.0, 100.0);
    let tiny = rect_wire(-0.01, -0.01, 0.01, 0.01);
    let face: Face = profile::attach_plane_normalized(vec![outer, tiny]).unwrap();
    assert_eq!(face.boundaries().len(), 2);
}

// -- Phase 1: validation error tests --

#[test]
fn open_wire_error() {
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let wire: Wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
    let result = profile::attach_plane_normalized::<Curve, Surface>(vec![wire]);
    assert!(result.is_err());
}

// -- Phase 5: solid from profile integration --

#[test]
fn solid_box_from_profile() {
    let outer = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(0.0, 0.0, 2.0),
    )
    .unwrap();
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.len(), 6);
    assert!(solid.is_geometric_consistent());
}

#[test]
fn solid_tube_from_profile_with_hole() {
    let outer = rect_wire(-2.0, -2.0, 2.0, 2.0);
    let hole = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer, hole],
        Vector3::new(0.0, 0.0, 3.0),
    )
    .unwrap();
    let shell = &solid.boundaries()[0];
    // 2 caps + 4 outer sides + 4 inner sides = 10 faces.
    assert_eq!(shell.len(), 10);
    assert!(solid.is_geometric_consistent());
}

#[test]
fn solid_with_triangle_profile() {
    let tri = triangle_wire(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
    );
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![tri],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    let shell = &solid.boundaries()[0];
    // Triangular prism: 2 triangles + 3 quads = 5 faces.
    assert_eq!(shell.len(), 5);
    assert!(solid.is_geometric_consistent());
}

#[test]
fn solid_diagonal_extrusion() {
    let outer = rect_wire(0.0, 0.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(1.0, 1.0, 1.0),
    )
    .unwrap();
    assert!(solid.is_geometric_consistent());
}

// -- Phase 14: revolve from planar profile --

/// Helper: builds a CCW rectangular wire in the XZ plane at y=0.
fn rect_wire_xz(x0: f64, z0: f64, x1: f64, z1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, 0.0, z0));
    let v1 = builder::vertex(Point3::new(x1, 0.0, z0));
    let v2 = builder::vertex(Point3::new(x1, 0.0, z1));
    let v3 = builder::vertex(Point3::new(x0, 0.0, z1));
    vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into()
}

#[test]
fn revolve_simple_rect() {
    // Revolve a rectangular profile (XZ plane) 360 degrees around the Y-axis.
    let wire = rect_wire_xz(2.0, -1.0, 4.0, 1.0);
    let solid = profile::revolve_from_planar_profile::<Curve, Surface, _>(
        vec![wire],
        Point3::origin(),
        Vector3::unit_y(),
        Rad(2.0 * std::f64::consts::PI),
        4,
    )
    .unwrap();
    assert!(solid.is_geometric_consistent());
}

#[test]
fn revolve_torus_topology() {
    // Revolve a small square offset from the axis to produce a torus-like solid.
    let wire = rect_wire_xz(3.0, -0.5, 4.0, 0.5);
    let solid = profile::revolve_from_planar_profile::<Curve, Surface, _>(
        vec![wire],
        Point3::origin(),
        Vector3::unit_y(),
        Rad(2.0 * std::f64::consts::PI),
        4,
    )
    .unwrap();
    assert!(solid.is_geometric_consistent());
}

#[test]
fn revolve_partial_angle() {
    // Revolve a rectangular profile 90 degrees (PI/2).
    let wire = rect_wire_xz(2.0, -1.0, 4.0, 1.0);
    let solid = profile::revolve_from_planar_profile::<Curve, Surface, _>(
        vec![wire],
        Point3::origin(),
        Vector3::unit_y(),
        Rad(std::f64::consts::FRAC_PI_2),
        2,
    )
    .unwrap();
    assert!(solid.is_geometric_consistent());
}

#[test]
fn revolve_with_hole() {
    // Revolve a profile with an outer rectangle and inner hole around an axis.
    let outer = rect_wire_xz(2.0, -2.0, 6.0, 2.0);
    let hole = rect_wire_xz(3.0, -1.0, 5.0, 1.0);
    let solid_with_hole = profile::revolve_from_planar_profile::<Curve, Surface, _>(
        vec![outer.clone(), hole],
        Point3::origin(),
        Vector3::unit_y(),
        Rad(2.0 * std::f64::consts::PI),
        4,
    )
    .unwrap();

    // With a hole, the solid has two boundary shells (outer + inner).
    assert!(solid_with_hole.boundaries().len() > 1);
    assert!(solid_with_hole.is_geometric_consistent());
}

#[test]
fn revolve_open_wire_rejected() {
    // Passing an open wire should return an error.
    let v0 = builder::vertex(Point3::new(2.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(4.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(4.0, 0.0, 2.0));
    let wire: Wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
    let result = profile::revolve_from_planar_profile::<Curve, Surface, _>(
        vec![wire],
        Point3::origin(),
        Vector3::unit_y(),
        Rad(2.0 * std::f64::consts::PI),
        4,
    );
    assert!(result.is_err());
}

// -- Phase 14: sweep from planar profile --

#[test]
fn sweep_rect_along_line() {
    // Sweep a rectangular profile along a straight-line guide.
    let wire = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let guide = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    );
    let solid = profile::sweep_from_planar_profile(vec![wire], &guide, 4).unwrap();
    assert!(solid.is_geometric_consistent());
}

#[test]
fn sweep_rect_along_curve() {
    // Sweep a rectangular profile along a curved guide (quarter arc approximation).
    let wire = rect_wire(-0.5, -0.5, 0.5, 0.5);
    let guide = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 5.0),
            Point3::new(5.0, 0.0, 5.0),
        ],
    );
    let solid = profile::sweep_from_planar_profile(vec![wire], &guide, 6).unwrap();
    assert!(solid.is_geometric_consistent());
}

#[test]
fn sweep_open_wire_rejected() {
    // Passing an open wire for sweep should return an error.
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let wire: Wire = vec![builder::line(&v0, &v1), builder::line(&v1, &v2)].into();
    let guide = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    );
    let result = profile::sweep_from_planar_profile(vec![wire], &guide, 4);
    assert!(result.is_err());
}

// -- Phase 14-3: validate_solid tests --

#[test]
fn validate_extruded_box() {
    let outer = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    let result = profile::validate_solid(&solid);
    assert!(result.is_ok(), "extruded box should validate: {result:?}");
}

#[test]
fn validate_extruded_with_hole() {
    let outer = rect_wire(-2.0, -2.0, 2.0, 2.0);
    let hole = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer, hole],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    let result = profile::validate_solid(&solid);
    assert!(
        result.is_ok(),
        "extruded tube should validate: {result:?}"
    );
}

#[test]
fn validate_revolved_solid() {
    let wire = rect_wire_xz(3.0, -0.5, 4.0, 0.5);
    let solid = profile::revolve_from_planar_profile::<Curve, Surface, _>(
        vec![wire],
        Point3::origin(),
        Vector3::unit_y(),
        Rad(2.0 * std::f64::consts::PI),
        4,
    )
    .unwrap();
    let result = profile::validate_solid(&solid);
    assert!(
        result.is_ok(),
        "revolved solid should validate: {result:?}"
    );
}

#[test]
fn validate_swept_solid() {
    let wire = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let guide = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    );
    let solid = profile::sweep_from_planar_profile(vec![wire], &guide, 4).unwrap();
    let result = profile::validate_solid(&solid);
    assert!(
        result.is_ok(),
        "swept solid should validate: {result:?}"
    );
}

#[test]
fn validate_triangle_extrusion() {
    let tri = triangle_wire(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
    );
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![tri],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    let result = profile::validate_solid(&solid);
    assert!(
        result.is_ok(),
        "triangular extrusion should validate: {result:?}"
    );
}

#[test]
fn validate_diagonal_extrusion() {
    let outer = rect_wire(0.0, 0.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(1.0, 1.0, 1.0),
    )
    .unwrap();
    let result = profile::validate_solid(&solid);
    assert!(
        result.is_ok(),
        "diagonal extrusion should validate: {result:?}"
    );
}

#[test]
fn validate_reports_details() {
    let outer = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    let report = profile::validate_solid(&solid).unwrap();
    assert!(report.vertices > 0, "vertices must be > 0");
    assert!(report.edges > 0, "edges must be > 0");
    assert!(report.faces > 0, "faces must be > 0");
    assert_eq!(
        report.euler_characteristic, 2,
        "euler characteristic must be 2 for closed shell"
    );
}

#[test]
fn validate_broken_solid_returns_error() {
    let outer = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    let shell = &solid.boundaries()[0];
    // Create a new shell with one face removed (making it non-closed).
    let mut faces: Vec<_> = shell.iter().cloned().collect();
    faces.pop();
    let broken_shell: Shell = faces.into_iter().collect();
    let broken_solid = monstertruck_topology::Solid::new_unchecked(vec![broken_shell]);
    let result = profile::validate_solid(&broken_solid);
    assert!(result.is_err(), "broken solid should fail validation");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("euler") || err_msg.contains("orientation") || err_msg.contains("closed"),
        "error message should describe the topology violation: {err_msg}"
    );
}

#[test]
fn validate_tessellation_smoke() {
    let outer = rect_wire(-1.0, -1.0, 1.0, 1.0);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        vec![outer],
        Vector3::new(0.0, 0.0, 1.0),
    )
    .unwrap();
    // First validate the solid.
    profile::validate_solid(&solid).unwrap();
    // Then confirm geometric data is intact: sample each face's surface.
    for boundary in solid.boundaries() {
        for face in boundary.iter() {
            let surface = face.oriented_surface();
            // Sampling the surface at (0.5, 0.5) should not panic.
            let _pt: Point3 = surface.subs(0.5, 0.5);
        }
    }
}

// -- Non-XY planes --

#[test]
fn profile_on_yz_plane() {
    let v0 = builder::vertex(Point3::new(0.0, -1.0, -1.0));
    let v1 = builder::vertex(Point3::new(0.0, 1.0, -1.0));
    let v2 = builder::vertex(Point3::new(0.0, 1.0, 1.0));
    let v3 = builder::vertex(Point3::new(0.0, -1.0, 1.0));
    let wire: Wire = vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into();
    let face: Face = profile::attach_plane_normalized(vec![wire]).unwrap();
    assert_eq!(face.boundaries().len(), 1);
}
