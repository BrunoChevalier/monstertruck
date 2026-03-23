//! Tests for the geometry module's [`Curve`] and [`Surface`] enum variants,
//! construction, trait implementations, and conversions.

use monstertruck_modeling::*;

// ---------------------------------------------------------------------------
// Curve tests
// ---------------------------------------------------------------------------

/// Construct a `Curve::Line` and verify `subs` at t=0 and t=1 gives endpoints.
#[test]
fn geometry_curve_line_construction() {
    let p0 = Point3::new(1.0, 2.0, 3.0);
    let p1 = Point3::new(4.0, 5.0, 6.0);
    let curve = Curve::Line(Line(p0, p1));
    assert_near!(curve.subs(0.0), p0);
    assert_near!(curve.subs(1.0), p1);
}

/// Construct a `Curve::BsplineCurve` (degree-1, two control points) and
/// verify `subs` at t=0 and t=1 gives the endpoints.
#[test]
fn geometry_curve_bspline_construction() {
    let p0 = Point3::new(0.0, 0.0, 0.0);
    let p1 = Point3::new(10.0, 0.0, 0.0);
    let bsp = BsplineCurve::new(KnotVector::bezier_knot(1), vec![p0, p1]);
    let curve = Curve::BsplineCurve(bsp);
    assert_near!(curve.subs(0.0), p0);
    assert_near!(curve.subs(1.0), p1);
}

/// `range_tuple()` returns valid (t0, t1) with t0 < t1 for various `Curve`
/// variants.
#[test]
fn geometry_curve_range() {
    let line_curve = Curve::Line(Line(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ));
    let (t0, t1) = line_curve.range_tuple();
    assert!(t0 < t1, "Line range_tuple must have t0 < t1: ({}, {})", t0, t1);

    let bsp = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let bsp_curve = Curve::BsplineCurve(bsp);
    let (t0, t1) = bsp_curve.range_tuple();
    assert!(
        t0 < t1,
        "BsplineCurve range_tuple must have t0 < t1: ({}, {})",
        t0,
        t1
    );
}

/// For a Line curve, `der(0.5)` returns a non-zero vector in the expected
/// direction.
#[test]
fn geometry_curve_der_finite() {
    let p0 = Point3::new(0.0, 0.0, 0.0);
    let p1 = Point3::new(3.0, 4.0, 0.0);
    let curve = Curve::Line(Line(p0, p1));
    let d = curve.der(0.5);
    // Derivative of a line is constant: p1 - p0.
    assert_near!(d, p1 - p0);
    assert!(
        d.magnitude() > 1e-10,
        "derivative must be non-zero, got magnitude {}",
        d.magnitude()
    );
}

/// Construct a `Surface::Plane` and verify `subs(0.0, 0.0)` gives the origin.
#[test]
fn geometry_surface_plane_construction() {
    let origin = Point3::new(1.0, 2.0, 3.0);
    let p1 = Point3::new(2.0, 2.0, 3.0);
    let p2 = Point3::new(1.0, 3.0, 3.0);
    let plane = Plane::new(origin, p1, p2);
    let surface = Surface::Plane(plane);
    assert_near!(surface.subs(0.0, 0.0), origin);
}

/// Construct a `Surface::BsplineSurface` from a simple bilinear surface and
/// verify `subs` at corners.
#[test]
fn geometry_surface_bspline_construction() {
    let p00 = Point3::new(0.0, 0.0, 0.0);
    let p01 = Point3::new(0.0, 1.0, 0.0);
    let p10 = Point3::new(1.0, 0.0, 0.0);
    let p11 = Point3::new(1.0, 1.0, 0.0);
    let bsp = BsplineSurface::new(
        (KnotVector::bezier_knot(1), KnotVector::bezier_knot(1)),
        vec![vec![p00, p01], vec![p10, p11]],
    );
    let surface = Surface::BsplineSurface(bsp);
    assert_near!(surface.subs(0.0, 0.0), p00);
    assert_near!(surface.subs(1.0, 0.0), p10);
    assert_near!(surface.subs(0.0, 1.0), p01);
    assert_near!(surface.subs(1.0, 1.0), p11);
}

/// For a `Surface::Plane`, the normal at any parameter is perpendicular to
/// both the u and v directions.
#[test]
fn geometry_surface_normal() {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let p1 = Point3::new(1.0, 0.0, 0.0);
    let p2 = Point3::new(0.0, 1.0, 0.0);
    let plane = Plane::new(origin, p1, p2);
    let surface = Surface::Plane(plane);
    let n: Vector3 = ParametricSurface3D::normal(&surface, 0.0, 0.0);
    let u_dir = p1 - origin;
    let v_dir = p2 - origin;
    assert!(
        n.dot(u_dir).abs() < 1e-10,
        "normal must be perpendicular to u direction"
    );
    assert!(
        n.dot(v_dir).abs() < 1e-10,
        "normal must be perpendicular to v direction"
    );
    // Should be unit Z.
    assert_near!(n, Vector3::new(0.0, 0.0, 1.0));
}

/// Clone a `Curve::Line` and verify both produce the same `subs` values.
#[test]
fn geometry_curve_clone_and_eq() {
    let curve = Curve::Line(Line(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    ));
    let cloned = curve.clone();
    for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        assert_near!(curve.subs(t), cloned.subs(t));
    }
}

/// For a Plane surface, `search_parameter` finds the (u, v) parameters for a
/// known point on the plane.
#[test]
fn geometry_surface_search_parameter() {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let p1 = Point3::new(1.0, 0.0, 0.0);
    let p2 = Point3::new(0.0, 1.0, 0.0);
    let plane = Plane::new(origin, p1, p2);
    let surface = Surface::Plane(plane);
    // Point at (0.3, 0.7, 0.0) should have parameters (0.3, 0.7).
    let target = Point3::new(0.3, 0.7, 0.0);
    let params = SearchParameter::<D2>::search_parameter(&surface, target, None, 1);
    assert!(params.is_some(), "search_parameter must find the point");
    let (u, v) = params.unwrap();
    assert!(
        (u - 0.3).abs() < 1e-10,
        "u parameter must be near 0.3, got {}",
        u
    );
    assert!(
        (v - 0.7).abs() < 1e-10,
        "v parameter must be near 0.7, got {}",
        v
    );
}

/// For a `Curve::Line`, `inverse()` swaps start and end points.
#[test]
fn geometry_curve_inverse() {
    let p0 = Point3::new(1.0, 2.0, 3.0);
    let p1 = Point3::new(4.0, 5.0, 6.0);
    let curve = Curve::Line(Line(p0, p1));
    let inv = curve.inverse();
    // subs(0.0) of inverted should be subs(1.0) of original.
    assert_near!(inv.subs(0.0), curve.subs(1.0));
    assert_near!(inv.subs(1.0), curve.subs(0.0));
}
