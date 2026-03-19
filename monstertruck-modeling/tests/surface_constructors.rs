use monstertruck_modeling::errors::Error;
use monstertruck_modeling::*;

/// Verify that the new error variants exist and display correct messages.
#[test]
fn test_error_variants_exist() {
    let e1 = Error::InsufficientRails {
        required: 2,
        got: 1,
    };
    assert_eq!(
        e1.to_string(),
        "multi-rail sweep requires at least 2 rails, got 1."
    );

    let e2 = Error::InsufficientSections {
        required: 2,
        got: 1,
    };
    assert_eq!(
        e2.to_string(),
        "surface construction requires at least 2 sections, got 1."
    );

    let e3 = Error::SurfaceConstructionFailed {
        reason: "degenerate".into(),
    };
    assert_eq!(
        e3.to_string(),
        "surface construction failed: degenerate"
    );

    let e4 = Error::GridDimensionMismatch {
        expected_rows: 2,
        expected_cols: 3,
        actual_rows: 1,
        actual_cols: 2,
    };
    assert_eq!(
        e4.to_string(),
        "gordon surface requires matching grid dimensions: expected 2x3, got 1x2."
    );
}

/// Verify PartialEq still works with the new variants.
#[test]
fn test_error_variants_eq() {
    assert_eq!(
        Error::InsufficientRails {
            required: 2,
            got: 1
        },
        Error::InsufficientRails {
            required: 2,
            got: 1
        },
    );
    assert_ne!(
        Error::InsufficientRails {
            required: 2,
            got: 1
        },
        Error::InsufficientRails {
            required: 3,
            got: 1
        },
    );
}

/// Helper: straight-line B-spline from `a` to `b`.
fn line_bspline(a: Point3, b: Point3) -> BsplineCurve<Point3> {
    BsplineCurve::new(KnotVector::bezier_knot(1), vec![a, b])
}

#[test]
fn test_try_sweep_rail_basic() {
    let profile = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let rail = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0));
    let face = builder::try_sweep_rail(&profile, &rail, 3).unwrap();
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn test_try_sweep_rail_insufficient_sections() {
    let profile = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let rail = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0));
    assert_eq!(
        builder::try_sweep_rail(&profile, &rail, 1).unwrap_err(),
        Error::InsufficientSections {
            required: 2,
            got: 1,
        },
    );
}

#[test]
fn test_try_birail_basic() {
    let rail1 = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 5.0));
    let rail2 = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 5.0));
    let profile = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let face = builder::try_birail(&profile, &rail1, &rail2, 3).unwrap();
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn test_try_gordon_basic() {
    // 2x2 bilinear patch.
    let u0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let u1 = line_bspline(Point3::new(0.0, 0.0, 1.0), Point3::new(1.0, 0.0, 1.0));
    let v0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));
    let v1 = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let points = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
        vec![Point3::new(0.0, 0.0, 1.0), Point3::new(1.0, 0.0, 1.0)],
    ];
    let face = builder::try_gordon(vec![u0, u1], vec![v0, v1], &points).unwrap();
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn test_try_gordon_dimension_mismatch() {
    let u0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let u1 = line_bspline(Point3::new(0.0, 0.0, 1.0), Point3::new(1.0, 0.0, 1.0));
    let v0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));
    // 2 u-curves but points has 3 rows.
    let points = vec![
        vec![Point3::new(0.0, 0.0, 0.0)],
        vec![Point3::new(0.0, 0.0, 0.5)],
        vec![Point3::new(0.0, 0.0, 1.0)],
    ];
    let result = builder::try_gordon(vec![u0, u1], vec![v0], &points);
    assert!(matches!(result, Err(Error::GridDimensionMismatch { .. })));
}
