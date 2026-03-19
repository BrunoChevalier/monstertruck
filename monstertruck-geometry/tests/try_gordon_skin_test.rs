use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::{GordonOptions, SkinOptions};
use monstertruck_geometry::prelude::*;

// --- try_skin ---

#[test]
fn try_skin_single_curve() {
    let curve = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let result = BsplineSurface::try_skin(vec![curve], &SkinOptions::default());
    assert!(result.is_ok());
}

#[test]
fn try_skin_two_curves_homotopy() {
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    );
    let result = BsplineSurface::try_skin(vec![c0, c1], &SkinOptions::default());
    assert!(result.is_ok());
    let surface = result.unwrap();
    assert_near2!(surface.subs(0.0, 0.0), Vector2::new(0.0, 0.0));
    assert_near2!(surface.subs(1.0, 1.0), Vector2::new(1.0, 1.0));
}

#[test]
fn try_skin_three_curves() {
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.5), Vector2::new(1.0, 0.5)],
    );
    let c2 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    );
    let result = BsplineSurface::try_skin(vec![c0, c1, c2], &SkinOptions::default());
    assert!(result.is_ok());
}

#[test]
fn try_skin_empty_returns_error() {
    let result =
        BsplineSurface::<Vector2>::try_skin(vec![], &SkinOptions::default());
    assert!(matches!(
        result,
        Err(Error::CurveNetworkIncompatible(
            CurveNetworkDiagnostic::InsufficientCurves {
                required: 1,
                got: 0,
            }
        ))
    ));
}

// --- try_gordon ---

#[test]
fn try_gordon_success() {
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let u1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    );
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)],
    );
    let v1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(1.0, 0.0), Vector2::new(1.0, 1.0)],
    );
    let points = vec![
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    ];
    let result = BsplineSurface::try_gordon(
        vec![u0, u1],
        vec![v0, v1],
        &points,
        &GordonOptions::default(),
    );
    assert!(result.is_ok());
    let gordon = result.unwrap();
    assert_near2!(gordon.subs(0.0, 0.0), Vector2::new(0.0, 0.0));
    assert_near2!(gordon.subs(1.0, 1.0), Vector2::new(1.0, 1.0));
}

#[test]
fn try_gordon_grid_dimension_mismatch() {
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)],
    );
    // Points grid has wrong dimensions: 1 u_curve but 2 rows.
    let points = vec![
        vec![Vector2::new(0.0, 0.0)],
        vec![Vector2::new(0.0, 1.0)],
    ];
    let result = BsplineSurface::try_gordon(
        vec![u0],
        vec![v0],
        &points,
        &GordonOptions::default(),
    );
    assert!(matches!(
        result,
        Err(Error::CurveNetworkIncompatible(
            CurveNetworkDiagnostic::GridDimensionMismatch { .. }
        ))
    ));
}

#[test]
fn try_gordon_empty_u_curves() {
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)],
    );
    let points: Vec<Vec<Vector2>> = vec![];
    let result = BsplineSurface::try_gordon(
        vec![],
        vec![v0],
        &points,
        &GordonOptions::default(),
    );
    assert!(matches!(
        result,
        Err(Error::CurveNetworkIncompatible(
            CurveNetworkDiagnostic::InsufficientCurves { .. }
        ))
    ));
}

// --- Deprecated old APIs still work ---

#[test]
#[allow(deprecated)]
fn deprecated_skin_still_works() {
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    );
    let _surface = BsplineSurface::skin(vec![c0, c1]);
}

#[test]
#[allow(deprecated)]
fn deprecated_gordon_still_works() {
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let u1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    );
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(0.0, 1.0)],
    );
    let v1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(1.0, 0.0), Vector2::new(1.0, 1.0)],
    );
    let points = vec![
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    ];
    let _gordon = BsplineSurface::gordon(vec![u0, u1], vec![v0, v1], &points);
}
