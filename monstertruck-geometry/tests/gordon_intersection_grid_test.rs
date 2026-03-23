//! Integration tests for intersection-grid Gordon surface with curved networks.
//!
//! Exercises [`try_gordon_from_network`](monstertruck_geometry::nurbs::BsplineSurface::try_gordon_from_network)
//! with non-trivial curved curve families and validates intersection point interpolation.

use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::curve_intersect;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::GordonOptions;
use monstertruck_geometry::prelude::*;

/// Builds a quadratic B-spline curve from `start` to `end` with a control
/// point offset that creates a genuine arc (not a straight line).
fn quadratic_arc(start: Point3, mid_ctrl: Point3, end: Point3) -> BsplineCurve<Point3> {
    BsplineCurve::new(KnotVector::bezier_knot(2), vec![start, mid_ctrl, end])
}

/// Builds an `n x m` network of quadratic curves with consistent Z-bulge so
/// each u-v pair has exactly one intersection in 3D.
///
/// The Z height at crossing point `(x, y)` follows
/// `z(x,y) = 4*x*(1-x) * 4*y*(1-y) * z_scale`, which is zero at all
/// boundary coordinates and maximal at the center. For a quadratic Bezier
/// with endpoints at z=0, setting the control point Z to `2 * midpoint_z`
/// produces the desired midpoint height via `B(0.5) = 0.25*P0 + 0.5*P1 + 0.25*P2`.
fn make_curved_network(
    u_ys: &[f64],
    v_xs: &[f64],
    z_scale: f64,
) -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>) {
    let u_curves = u_ys
        .iter()
        .map(|&y| {
            let ctrl_z = 2.0 * 4.0 * y * (1.0 - y) * z_scale;
            quadratic_arc(
                Point3::new(0.0, y, 0.0),
                Point3::new(0.5, y, ctrl_z),
                Point3::new(1.0, y, 0.0),
            )
        })
        .collect();

    let v_curves = v_xs
        .iter()
        .map(|&x| {
            let ctrl_z = 2.0 * 4.0 * x * (1.0 - x) * z_scale;
            quadratic_arc(
                Point3::new(x, 0.0, 0.0),
                Point3::new(x, 0.5, ctrl_z),
                Point3::new(x, 1.0, 0.0),
            )
        })
        .collect();

    (u_curves, v_curves)
}

/// Builds a 3x3 network of quadratic curves with curved intersections.
fn make_curved_3x3_network() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>) {
    make_curved_network(&[0.0, 0.5, 1.0], &[0.0, 0.5, 1.0], 0.3)
}

#[test]
fn curved_3x3_network_interpolates_intersections() {
    let (u_curves, v_curves) = make_curved_3x3_network();

    // Independently compute intersection points.
    let intersection_points: Vec<Vec<Point3>> = u_curves
        .iter()
        .map(|u| {
            v_curves
                .iter()
                .map(|v| {
                    let hits = curve_intersect::find_intersections(u, v);
                    assert_eq!(
                        hits.len(),
                        1,
                        "Expected exactly 1 intersection per curve pair"
                    );
                    hits[0].point
                })
                .collect()
        })
        .collect();

    // Build the Gordon surface.
    let (u2, v2) = make_curved_3x3_network();
    let surface = BsplineSurface::try_gordon_from_network(u2, v2, &GordonOptions::default())
        .expect("Gordon surface construction should succeed for 3x3 curved network");

    // Verify the surface interpolates all intersection points.
    // We need to find the parameter values where the intersections occur on the surface.
    // The Gordon construction maps curve parameters to surface parameters, so we evaluate
    // at the normalized intersection parameter locations.
    let tol = monstertruck_core::tolerance_constants::SNAP_TOLERANCE;

    // For a well-constructed Gordon surface, the surface should pass through the
    // intersection points. We verify by searching for nearest parameters on the surface.
    intersection_points.iter().flatten().for_each(|pt| {
        let result = surface.search_nearest_parameter(*pt, None, 100);
        assert!(
            result.is_some(),
            "Should find nearest parameter for intersection point {pt:?}"
        );
        let (u, v) = result.unwrap();
        let surf_pt = surface.subs(u, v);
        let dist = pt.distance(surf_pt);
        assert!(
            dist < tol,
            "Surface should interpolate intersection point within tolerance: \
             point={pt:?}, surface({u:.6}, {v:.6})={surf_pt:?}, distance={dist:.2e}, tol={tol:.2e}"
        );
    });
}

#[test]
fn curved_network_with_near_tangent_curves_error() {
    // Create curves that are nearly tangent -- two curves that approach each
    // other closely but do not cleanly cross.
    // A u-curve along X at z=0, and a v-curve also along X but offset slightly in Y and Z.
    // These are nearly parallel/tangent, producing zero or multiple "near" intersections.
    let u0 = quadratic_arc(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 0.0, 0.01),
        Point3::new(1.0, 0.0, 0.0),
    );
    // v-curve runs almost parallel to u0 but slightly offset.
    let v0 = quadratic_arc(
        Point3::new(0.0, 0.001, 0.0),
        Point3::new(0.5, 0.001, 0.01),
        Point3::new(1.0, 0.001, 0.0),
    );

    let result =
        BsplineSurface::try_gordon_from_network(vec![u0], vec![v0], &GordonOptions::default());
    // Should return an error (IntersectionCountMismatch), not panic.
    assert!(
        result.is_err(),
        "Near-tangent curves should produce an error, got Ok"
    );
    match result.unwrap_err() {
        Error::CurveNetworkIncompatible(CurveNetworkDiagnostic::IntersectionCountMismatch {
            ..
        }) => {}
        other => panic!("Expected IntersectionCountMismatch error, got {:?}", other),
    }
}

#[test]
fn large_5x4_curved_grid_success() {
    // Build a 5x4 network of degree-2 curves with genuine curved intersections.
    let (u_curves, v_curves) =
        make_curved_network(&[0.0, 0.25, 0.5, 0.75, 1.0], &[0.0, 0.33, 0.67, 1.0], 0.2);

    let result =
        BsplineSurface::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default());

    assert!(
        result.is_ok(),
        "5x4 curved grid should succeed, got {:?}",
        result.err()
    );

    let surface = result.unwrap();
    let cp = surface.control_points();
    // A 5x4 Gordon surface should have at least 5 rows and 4 columns of control points.
    assert!(
        cp.len() >= 5,
        "Expected at least 5 control point rows, got {}",
        cp.len()
    );
    assert!(
        cp[0].len() >= 4,
        "Expected at least 4 control point columns, got {}",
        cp[0].len()
    );
}

#[test]
fn gordon_from_network_surface_corners_match_curve_endpoints() {
    // For a 2x2 curved grid, verify corners.
    let u0 = quadratic_arc(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 0.0, 0.3),
        Point3::new(1.0, 0.0, 0.0),
    );
    let u1 = quadratic_arc(
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.5, 1.0, 0.3),
        Point3::new(1.0, 1.0, 0.0),
    );
    let v0 = quadratic_arc(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.5, 0.2),
        Point3::new(0.0, 1.0, 0.0),
    );
    let v1 = quadratic_arc(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.5, 0.2),
        Point3::new(1.0, 1.0, 0.0),
    );

    // Compute expected corners from curve intersections.
    let p00 = curve_intersect::find_intersections(&u0, &v0)[0].point;
    let p10 = curve_intersect::find_intersections(&u0, &v1)[0].point;
    let p01 = curve_intersect::find_intersections(&u1, &v0)[0].point;
    let p11 = curve_intersect::find_intersections(&u1, &v1)[0].point;

    let surface = BsplineSurface::try_gordon_from_network(
        vec![u0, u1],
        vec![v0, v1],
        &GordonOptions::default(),
    )
    .expect("2x2 curved grid should succeed");

    let ((u_min, u_max), (v_min, v_max)) = surface.range_tuple();
    assert_near2!(surface.subs(u_min, v_min), p00);
    assert_near2!(surface.subs(u_max, v_min), p10);
    assert_near2!(surface.subs(u_min, v_max), p01);
    assert_near2!(surface.subs(u_max, v_max), p11);
}
