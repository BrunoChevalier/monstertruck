use monstertruck_modeling::*;

/// Helper: build a `Curve::IntersectionCurve` from two planes intersecting
/// along the x-axis. The leader is a B-spline approximation of the line
/// segment from (0,0,0) to (1,0,0).
fn plane_plane_intersection_curve() -> Curve {
    // Two planes whose intersection is the x-axis.
    let plane0 = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let plane1 = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
    );
    let leader = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::origin(), Point3::new(1.0, 0.0, 0.0)],
    );
    Curve::IntersectionCurve(IntersectionCurve::new(
        Box::new(Surface::Plane(plane0)),
        Box::new(Surface::Plane(plane1)),
        Box::new(Curve::BsplineCurve(leader)),
    ))
}

#[test]
fn intersection_curve_lift_up_returns_bspline() {
    let ic = plane_plane_intersection_curve();
    // This should NOT panic -- it must delegate through the leader curve.
    let lifted = ic.lift_up();
    // The lifted curve evaluated at endpoints should match the leader.
    let start = lifted.subs(0.0);
    let end = lifted.subs(1.0);
    // Start should be near (0,0,0,1) and end near (1,0,0,1).
    assert!(
        (start.truncate() - Vector3::new(0.0, 0.0, 0.0)).magnitude() < 1.0e-6,
        "start = {start:?}"
    );
    assert!(
        (end.truncate() - Vector3::new(1.0, 0.0, 0.0)).magnitude() < 1.0e-6,
        "end = {end:?}"
    );
}

#[test]
fn include_curve_bspline_surface_with_intersection_curve() {
    let ic = plane_plane_intersection_curve();
    // A BsplineSurface that contains the x-axis line (the xy-plane patch).
    let surface = BsplineSurface::new(
        (KnotVector::bezier_knot(1), KnotVector::bezier_knot(1)),
        vec![
            vec![Point3::new(-1.0, -1.0, 0.0), Point3::new(-1.0, 1.0, 0.0)],
            vec![Point3::new(2.0, -1.0, 0.0), Point3::new(2.0, 1.0, 0.0)],
        ],
    );
    let surface = Surface::BsplineSurface(surface);
    // The intersection curve lies on the x-axis which is on this surface.
    assert!(surface.include(&ic));
}

#[test]
fn include_curve_nurbs_surface_with_intersection_curve() {
    let ic = plane_plane_intersection_curve();
    // Create a NurbsSurface from a BsplineSurface.
    let bsp_surface = BsplineSurface::new(
        (KnotVector::bezier_knot(1), KnotVector::bezier_knot(1)),
        vec![
            vec![
                Vector4::new(-1.0, -1.0, 0.0, 1.0),
                Vector4::new(-1.0, 1.0, 0.0, 1.0),
            ],
            vec![
                Vector4::new(2.0, -1.0, 0.0, 1.0),
                Vector4::new(2.0, 1.0, 0.0, 1.0),
            ],
        ],
    );
    let surface = Surface::NurbsSurface(NurbsSurface::new(bsp_surface));
    assert!(surface.include(&ic));
}

#[test]
fn include_curve_plane_with_intersection_curve() {
    let ic = plane_plane_intersection_curve();
    // The x-axis lies on the xy-plane.
    let plane = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let surface = Surface::Plane(plane);
    assert!(surface.include(&ic));
}

#[test]
fn include_curve_revolved_bspline_with_intersection_curve() {
    let ic = plane_plane_intersection_curve();
    // A cylinder of radius 0 (degenerate) won't work. Instead, build a
    // revolved surface that contains the x-axis. Use a line in the xz-plane
    // revolved around the x-axis.
    // Actually use a simpler approach: check that the include call does not panic.
    // We use a cylinder that does NOT contain the line, so result should be false
    // (but no panic).
    let entity = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let surface = Surface::RevolutedCurve(Processor::new(RevolutedCurve::by_revolution(
        Curve::BsplineCurve(entity),
        Point3::origin(),
        Vector3::unit_x(),
    )));
    // The intersection curve is on the x-axis, but the revolved surface is a
    // cylinder of radius 1 around the x-axis, so inclusion should be false.
    // The key test is that it does NOT panic.
    let result = surface.include(&ic);
    assert!(!result, "x-axis is not on a cylinder of radius 1");
}

#[test]
fn include_curve_revolved_nurbs_with_intersection_curve() {
    let ic = plane_plane_intersection_curve();
    let bsp = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let nurbs_curve = NurbsCurve::from(bsp);
    let surface = Surface::RevolutedCurve(Processor::new(RevolutedCurve::by_revolution(
        Curve::NurbsCurve(nurbs_curve),
        Point3::origin(),
        Vector3::unit_x(),
    )));
    // Should not panic. The x-axis is not on the cylinder of radius 1.
    let result = surface.include(&ic);
    assert!(!result);
}

#[test]
fn include_curve_revolved_intersection_entity_with_intersection_curve() {
    // When the entity curve of the RevolutedCurve is itself an IntersectionCurve.
    let ic = plane_plane_intersection_curve();
    let entity_ic = plane_plane_intersection_curve();
    let surface = Surface::RevolutedCurve(Processor::new(RevolutedCurve::by_revolution(
        entity_ic,
        Point3::origin(),
        Vector3::unit_x(),
    )));
    // Should not panic. We just test that it doesn't crash.
    let _result = surface.include(&ic);
}

#[test]
fn extruded_curve_to_same_geometry_with_intersection_curve() {
    let ic = plane_plane_intersection_curve();
    let extrusion = Vector3::new(0.0, 0.0, 1.0);
    let extruded = ExtrudedCurve::by_extrusion(ic, extrusion);
    // Should not panic.
    let surface: Surface = extruded.to_same_geometry();
    // The resulting surface at (0,0) should be near (0,0,0).
    let pt = surface.subs(0.0, 0.0);
    assert!(
        pt.distance(Point3::origin()) < 1.0e-3,
        "pt at (0,0) = {pt:?}"
    );
}
