use monstertruck_modeling::errors::Error;
use monstertruck_modeling::*;
use monstertruck_geometry::nurbs::surface_options::GordonOptions;

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
    assert_eq!(e3.to_string(), "surface construction failed: degenerate");

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

#[test]
fn test_try_sweep_multi_rail_three_rails() {
    // Profile is a quadratic Bezier arc spanning the three rail start points.
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
    );
    // Three non-collinear rails (triangle configuration in XY, sweeping along Z).
    let rail0 = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 5.0));
    let rail1 = line_bspline(Point3::new(0.0, 1.0, 0.0), Point3::new(0.0, 1.0, 5.0));
    let rail2 = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 5.0));
    let face = builder::try_sweep_multi_rail(&profile, &[rail0, rail1, rail2], 5).unwrap();
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn test_try_sweep_multi_rail_insufficient_rails() {
    let profile = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let rail0 = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 5.0));
    let result = builder::try_sweep_multi_rail(&profile, &[rail0], 5);
    assert_eq!(
        result.unwrap_err(),
        Error::InsufficientRails {
            required: 2,
            got: 1,
        },
    );
}

#[test]
fn test_try_sweep_periodic_closed_seam() {
    // Circular-ish rail.
    let rail = BsplineCurve::new(
        KnotVector::from(vec![0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0]),
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(0.0, -1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
    );
    let profile = line_bspline(Point3::new(1.0, 0.0, -0.2), Point3::new(1.0, 0.0, 0.2));
    let shell = builder::try_sweep_periodic(&profile, &rail, 8).unwrap();
    assert_eq!(shell.len(), 1);

    // Verify C0 seam continuity: subs(u, 0) == subs(u, 1).
    let face = &shell[0];
    let surface = face.surface();
    for i in 0..=10 {
        let u = i as f64 / 10.0;
        let p0 = surface.subs(u, 0.0);
        let p1 = surface.subs(u, 1.0);
        assert!(p0.near(&p1), "seam mismatch at u={u}: {p0:?} vs {p1:?}",);
    }
}

/// Verify that a periodic sweep shell passes the Euler-Poincare topology check.
#[test]
fn test_try_sweep_periodic_euler_poincare() {
    let rail = BsplineCurve::new(
        KnotVector::from(vec![0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0]),
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(0.0, -1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
    );
    let profile = line_bspline(Point3::new(1.0, 0.0, -0.2), Point3::new(1.0, 0.0, 0.2));
    let shell = builder::try_sweep_periodic(&profile, &rail, 8).unwrap();
    assert!(shell.is_geometric_consistent());
}

/// Verify corner vertex positions for a swept-rail face.
#[test]
fn test_try_sweep_rail_vertex_positions() {
    let profile = line_bspline(Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let rail = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0));
    let face = builder::try_sweep_rail(&profile, &rail, 3).unwrap();
    let wire = &face.boundaries()[0];
    // Collect all vertex positions around the boundary.
    let pts: Vec<Point3> = wire.vertex_iter().map(|v| v.point()).collect();
    // The four corners should be approximately at the expected positions.
    assert!(pts.iter().any(|p| p.near(&Point3::new(-1.0, 0.0, 0.0))));
    assert!(pts.iter().any(|p| p.near(&Point3::new(1.0, 0.0, 0.0))));
    assert!(pts.iter().any(|p| p.near(&Point3::new(-1.0, 0.0, 5.0))));
    assert!(pts.iter().any(|p| p.near(&Point3::new(1.0, 0.0, 5.0))));
}

#[test]
fn test_try_sweep_periodic_insufficient_sections() {
    let profile = line_bspline(Point3::new(1.0, 0.0, -0.2), Point3::new(1.0, 0.0, 0.2));
    let rail = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
    let result = builder::try_sweep_periodic(&profile, &rail, 2);
    assert_eq!(
        result.unwrap_err(),
        Error::InsufficientSections {
            required: 3,
            got: 2,
        },
    );
}

// --- Gordon variant builder wrappers ---

#[test]
fn gordon_from_network_builder_success() {
    let u0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let u1 = line_bspline(Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    let v0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
    let v1 = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    let face =
        builder::try_gordon_from_network(vec![u0, u1], vec![v0, v1], &GordonOptions::default())
            .unwrap();
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn gordon_verified_builder_success() {
    let u0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let u1 = line_bspline(Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    let v0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
    let v1 = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    let points = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
        vec![Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    ];
    let face = builder::try_gordon_verified(
        vec![u0, u1],
        vec![v0, v1],
        &points,
        &GordonOptions::default(),
    )
    .unwrap();
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn gordon_from_network_error_propagates() {
    // Parallel non-intersecting curves.
    let u0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let v0 = line_bspline(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0));
    let result =
        builder::try_gordon_from_network(vec![u0], vec![v0], &GordonOptions::default());
    assert!(
        matches!(result, Err(Error::FromGeometry(_))),
        "Expected FromGeometry error, got {:?}",
        result,
    );
}

#[test]
fn gordon_verified_error_propagates() {
    let u0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let u1 = line_bspline(Point3::new(0.0, 1.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    let v0 = line_bspline(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0));
    let v1 = line_bspline(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    // Deliberately wrong grid point.
    let points = vec![
        vec![
            Point3::new(0.5, 0.5, 0.5),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ],
    ];
    let result = builder::try_gordon_verified(
        vec![u0, u1],
        vec![v0, v1],
        &points,
        &GordonOptions::default(),
    );
    assert!(
        matches!(result, Err(Error::FromGeometry(_))),
        "Expected FromGeometry error, got {:?}",
        result,
    );
}
