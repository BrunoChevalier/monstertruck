use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
use monstertruck_geometry::nurbs::curve_intersect::{find_intersections, find_self_intersections};
use monstertruck_geometry::prelude::*;

/// Helper: asserts a parameter is within `SNAP_TOLERANCE * 10.0` of expected.
///
/// The 10x multiplier accounts for accumulated floating-point error in
/// curve evaluation and Newton-Raphson convergence.
fn assert_param_near(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() < SNAP_TOLERANCE * 10.0,
        "{label}: expected {expected}, got {actual}, diff = {}",
        (actual - expected).abs()
    );
}

/// Helper: asserts a point is within `SNAP_TOLERANCE * 10.0` of expected.
///
/// The 10x multiplier accounts for accumulated floating-point error in
/// curve evaluation and Newton-Raphson convergence.
fn assert_point_near(actual: Point3, expected: Point3, label: &str) {
    let dist = actual.distance(expected);
    assert!(
        dist < SNAP_TOLERANCE * 10.0,
        "{label}: expected {expected:?}, got {actual:?}, dist = {dist}"
    );
}

#[test]
fn two_crossing_lines() {
    // Line 0: (0,0,0) -> (2,2,0), degree 1.
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 2.0, 0.0)],
    );
    // Line 1: (2,0,0) -> (0,2,0), degree 1.
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(2.0, 0.0, 0.0), Point3::new(0.0, 2.0, 0.0)],
    );
    let results = find_intersections(&c0, &c1);
    assert_eq!(results.len(), 1, "expected exactly 1 intersection");
    assert_param_near(results[0].t0, 0.5, "t0");
    assert_param_near(results[0].t1, 0.5, "t1");
    assert_point_near(results[0].point, Point3::new(1.0, 1.0, 0.0), "point");
}

#[test]
fn two_crossing_cubics() {
    // Cubic Bezier curve0: roughly from (0,0,0) to (2,2,0).
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(3),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(1.5, 1.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
        ],
    );
    // Cubic Bezier curve1: roughly from (2,0,0) to (0,2,0).
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(3),
        vec![
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(1.5, 1.0, 0.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
        ],
    );
    let results = find_intersections(&c0, &c1);
    assert_eq!(
        results.len(),
        1,
        "expected exactly 1 intersection for crossing cubics"
    );
    // The intersection should be near (1,1,0).
    assert_point_near(
        results[0].point,
        Point3::new(1.0, 1.0, 0.0),
        "crossing cubics point",
    );
}

#[test]
fn non_intersecting_curves() {
    // Two curves in different z-planes.
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 5.0), Point3::new(1.0, 1.0, 5.0)],
    );
    let results = find_intersections(&c0, &c1);
    assert!(
        results.is_empty(),
        "expected no intersections for separated curves"
    );
}

#[test]
fn tangent_intersection() {
    // Parabola-like curve touching a horizontal line tangentially at vertex.
    // Curve 0: quadratic Bezier forming an upward parabola with vertex at (1,0,0).
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, -1.0, 0.0),
            Point3::new(2.0, 1.0, 0.0),
        ],
    );
    // Curve 1: horizontal line at y=0.
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0)],
    );
    let results = find_intersections(&c0, &c1);
    assert_eq!(results.len(), 1, "expected 1 tangent intersection");
    assert_point_near(
        results[0].point,
        Point3::new(1.0, 0.0, 0.0),
        "tangent point",
    );
}

#[test]
fn multiple_intersections() {
    // A wavy cubic and a line crossing it at multiple points.
    // Cubic through (0,0,0),(0.33,1,0),(0.67,-1,0),(1,0,0) vs horizontal line y=0.
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(3),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.33, 2.0, 0.0),
            Point3::new(0.67, -2.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-0.5, 0.0, 0.0), Point3::new(1.5, 0.0, 0.0)],
    );
    let results = find_intersections(&c0, &c1);
    // The cubic starts at y=0, passes through y>0, back through y=0 at midpoint,
    // then y<0, and back to y=0. That's 3 crossings of the x-axis.
    assert!(
        results.len() >= 2,
        "expected at least 2 intersections, got {}",
        results.len()
    );
    // Results should be sorted by t0.
    for w in results.windows(2) {
        assert!(w[0].t0 <= w[1].t0, "results not sorted by t0");
    }
}

#[test]
fn identical_endpoint() {
    // Two curves sharing the endpoint (1,1,0).
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(1.0, 1.0, 0.0), Point3::new(2.0, 0.0, 0.0)],
    );
    let results = find_intersections(&c0, &c1);
    assert!(
        !results.is_empty(),
        "expected at least 1 intersection at shared endpoint"
    );
    // One of the intersections should be at the shared endpoint.
    let has_endpoint = results
        .iter()
        .any(|r| r.point.distance(Point3::new(1.0, 1.0, 0.0)) < SNAP_TOLERANCE * 10.0);
    assert!(has_endpoint, "shared endpoint not found in results");
}

#[test]
fn test_parallel_curves() {
    // Two parallel lines offset in y.
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let results = find_intersections(&c0, &c1);
    assert!(
        results.is_empty(),
        "parallel curves should have no intersections"
    );
}

#[test]
fn test_self_intersection_figure_eight() {
    // A curve that crosses itself: control points forming a figure-eight shape.
    let c = BsplineCurve::new(
        KnotVector::from(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0]),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(2.0, -2.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(-2.0, 2.0, 0.0),
        ],
    );
    let results = find_self_intersections(&c);
    assert!(
        !results.is_empty(),
        "expected self-intersection for figure-eight curve"
    );
}

#[test]
fn test_self_intersection_simple_curve() {
    // A simple straight line -- no self-intersection.
    let c = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let results = find_self_intersections(&c);
    assert!(
        results.is_empty(),
        "straight line should have no self-intersections"
    );
}

#[test]
fn test_near_tangent_no_panic() {
    // Two curves that are nearly tangent but slightly separated.
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Point3::new(0.0, 0.1, 0.0),
            Point3::new(1.0, -0.5, 0.0),
            Point3::new(2.0, 0.1, 0.0),
        ],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0)],
    );
    // Should not panic. May or may not find intersections depending on proximity.
    let _results = find_intersections(&c0, &c1);
}
