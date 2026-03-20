use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::GordonOptions;
use monstertruck_geometry::prelude::*;

// --- Task 1: GordonOptions and CurveNetworkDiagnostic extensions ---

#[test]
fn gordon_options_default_grid_tolerance_equals_snap_tolerance() {
    let opts = GordonOptions::default();
    assert_eq!(opts.grid_tolerance, SNAP_TOLERANCE);
}

#[test]
fn gordon_options_custom_grid_tolerance() {
    let mut opts = GordonOptions::default();
    opts.grid_tolerance = 0.01;
    assert_eq!(opts.grid_tolerance, 0.01);
}

#[test]
fn intersection_count_mismatch_display() {
    let diag = CurveNetworkDiagnostic::IntersectionCountMismatch {
        u_curve_index: 2,
        v_curve_index: 3,
        found: 0,
        expected: 1,
    };
    let msg = format!("{}", diag);
    assert_eq!(
        msg,
        "intersection count mismatch at u[2] x v[3]: found 0, expected 1"
    );
}

#[test]
fn grid_point_not_on_curve_display() {
    let diag = CurveNetworkDiagnostic::GridPointNotOnCurve {
        row: 1,
        col: 2,
        u_distance: 0.001234,
        v_distance: 0.005678,
        tolerance: 0.00001,
    };
    let msg = format!("{}", diag);
    assert_eq!(
        msg,
        "grid point [1][2] not on curves: u-distance=0.001234, v-distance=0.005678, tolerance=0.000010"
    );
}

// --- Tests for try_gordon_from_network ---

/// Helper: build a simple perpendicular grid of linear curves in 3D.
/// u-curves go along X at different Y values.
/// v-curves go along Y at different X values.
fn make_simple_grid_curves() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>) {
    // Two u-curves: horizontal lines at y=0 and y=1.
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    let u1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );

    // Two v-curves: vertical lines at x=0 and x=1.
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
    );
    let v1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );

    (vec![u0, u1], vec![v0, v1])
}

#[test]
fn try_gordon_from_network_success() {
    let (u_curves, v_curves) = make_simple_grid_curves();
    let result =
        BsplineSurface::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default());
    assert!(result.is_ok(), "Expected success, got {:?}", result.err());
    let surface = result.unwrap();
    // Check corner points.
    assert_near2!(surface.subs(0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
    assert_near2!(surface.subs(1.0, 1.0), Point3::new(1.0, 1.0, 0.0));
}

#[test]
fn try_gordon_from_network_empty_u_curves() {
    let (_, v_curves) = make_simple_grid_curves();
    let result =
        BsplineSurface::try_gordon_from_network(vec![], v_curves, &GordonOptions::default());
    assert!(matches!(
        result,
        Err(Error::CurveNetworkIncompatible(
            CurveNetworkDiagnostic::InsufficientCurves { .. }
        ))
    ));
}

#[test]
fn try_gordon_from_network_empty_v_curves() {
    let (u_curves, _) = make_simple_grid_curves();
    let result =
        BsplineSurface::try_gordon_from_network(u_curves, vec![], &GordonOptions::default());
    assert!(matches!(
        result,
        Err(Error::CurveNetworkIncompatible(
            CurveNetworkDiagnostic::InsufficientCurves { .. }
        ))
    ));
}

#[test]
fn try_gordon_from_network_parallel_curves_no_intersection() {
    // Two parallel u-curves that never intersect with the v-curve.
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    // v-curve also runs along X (parallel to u0, no intersection).
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0)],
    );
    let result =
        BsplineSurface::try_gordon_from_network(vec![u0], vec![v0], &GordonOptions::default());
    assert!(matches!(
        result,
        Err(Error::CurveNetworkIncompatible(
            CurveNetworkDiagnostic::IntersectionCountMismatch { found: 0, .. }
        ))
    ));
}

// --- Nonuniform spacing ---

#[test]
fn try_gordon_from_network_nonuniform_spacing() {
    // 3x3 network with nonuniform spacing: u-curves at y = 0.0, 0.3, 1.0.
    // (Asymmetric grids like 3x2 trigger a pre-existing bug in try_gordon.)
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    let u1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.3, 0.0), Point3::new(1.0, 0.3, 0.0)],
    );
    let u2 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
    );
    let v1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.5, 0.0, 0.0), Point3::new(0.5, 1.0, 0.0)],
    );
    let v2 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    );
    let result = BsplineSurface::try_gordon_from_network(
        vec![u0, u1, u2],
        vec![v0, v1, v2],
        &GordonOptions::default(),
    );
    assert!(
        result.is_ok(),
        "Nonuniform spacing should succeed, got {:?}",
        result.err()
    );
}

// --- Equivalence between variants ---

#[test]
fn try_gordon_verified_equivalence_with_from_network() {
    let points = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    ];
    let (u2, v2) = make_simple_grid_curves();
    let (u3, v3) = make_simple_grid_curves();
    let from_network =
        BsplineSurface::try_gordon_from_network(u2, v2, &GordonOptions::default()).unwrap();
    let verified =
        BsplineSurface::try_gordon_verified(u3, v3, &points, &GordonOptions::default()).unwrap();

    // Verify both surfaces evaluate to the same values at a grid of sample parameters.
    for i in 0..=10 {
        for j in 0..=10 {
            let u = i as f64 / 10.0;
            let v = j as f64 / 10.0;
            assert_near2!(from_network.subs(u, v), verified.subs(u, v));
        }
    }
}

// --- Tests for try_gordon_verified ---

#[test]
fn try_gordon_verified_success_with_exact_points() {
    let (u_curves, v_curves) = make_simple_grid_curves();
    let points = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    ];
    let result = BsplineSurface::try_gordon_verified(
        u_curves,
        v_curves,
        &points,
        &GordonOptions::default(),
    );
    assert!(result.is_ok(), "Expected success, got {:?}", result.err());
    let surface = result.unwrap();
    assert_near2!(surface.subs(0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
    assert_near2!(surface.subs(1.0, 1.0), Point3::new(1.0, 1.0, 0.0));
}

#[test]
fn try_gordon_verified_snaps_near_miss_points() {
    let (u_curves, v_curves) = make_simple_grid_curves();
    // Slightly perturbed points (within SNAP_TOLERANCE = 1e-5).
    let eps = SNAP_TOLERANCE * 0.5;
    let points = vec![
        vec![
            Point3::new(eps, eps, 0.0),
            Point3::new(1.0 - eps, eps, 0.0),
        ],
        vec![
            Point3::new(eps, 1.0 - eps, 0.0),
            Point3::new(1.0 - eps, 1.0 - eps, 0.0),
        ],
    ];
    let result = BsplineSurface::try_gordon_verified(
        u_curves,
        v_curves,
        &points,
        &GordonOptions::default(),
    );
    assert!(
        result.is_ok(),
        "Near-miss points within tolerance should be snapped, got {:?}",
        result.err()
    );
}

#[test]
fn try_gordon_verified_rejects_far_points() {
    let (u_curves, v_curves) = make_simple_grid_curves();
    // Point far from the curves.
    let points = vec![
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 5.0), // z=5.0, way off the curve
        ],
    ];
    let result = BsplineSurface::try_gordon_verified(
        u_curves,
        v_curves,
        &points,
        &GordonOptions::default(),
    );
    assert!(
        matches!(
            result,
            Err(Error::CurveNetworkIncompatible(
                CurveNetworkDiagnostic::GridPointNotOnCurve { row: 1, col: 1, .. }
            ))
        ),
        "Expected GridPointNotOnCurve error for far point, got {:?}",
        result
    );
}

#[test]
fn try_gordon_verified_dimension_mismatch() {
    let (u_curves, v_curves) = make_simple_grid_curves();
    // Wrong number of rows.
    let points = vec![vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ]];
    let result = BsplineSurface::try_gordon_verified(
        u_curves,
        v_curves,
        &points,
        &GordonOptions::default(),
    );
    assert!(
        matches!(
            result,
            Err(Error::CurveNetworkIncompatible(
                CurveNetworkDiagnostic::GridDimensionMismatch { .. }
            ))
        ),
        "Expected GridDimensionMismatch error, got {:?}",
        result
    );
}

#[test]
fn try_gordon_verified_custom_tolerance() {
    let (u_curves, v_curves) = make_simple_grid_curves();
    // Points with z offset of 0.005 (within custom tolerance of 0.01).
    let points = vec![
        vec![
            Point3::new(0.0, 0.0, 0.005),
            Point3::new(1.0, 0.0, 0.005),
        ],
        vec![
            Point3::new(0.0, 1.0, 0.005),
            Point3::new(1.0, 1.0, 0.005),
        ],
    ];
    let mut opts = GordonOptions::default();
    opts.grid_tolerance = 0.01;
    let result = BsplineSurface::try_gordon_verified(u_curves, v_curves, &points, &opts);
    assert!(
        result.is_ok(),
        "Points within custom tolerance should succeed, got {:?}",
        result.err()
    );
}
