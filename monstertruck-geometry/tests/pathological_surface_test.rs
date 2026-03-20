//! Integration tests exercising pathological fixtures through surface constructors.

use monstertruck_geometry::nurbs::surface_options::{Birail1Options, SweepRailOptions};
use monstertruck_geometry::nurbs::test_fixtures;
use monstertruck_geometry::prelude::*;

/// A simple straight profile for sweep operations.
fn straight_profile() -> BsplineCurve<Point3> {
    BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    )
}

/// A simple straight rail for sweep operations.
fn straight_rail() -> BsplineCurve<Point3> {
    BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    )
}

/// Helper: asserts that a point has all-finite coordinates.
fn assert_finite(p: &Point3, label: &str) {
    assert!(
        p.x.is_finite() && p.y.is_finite() && p.z.is_finite(),
        "{label}: expected finite point, got {p:?}",
    );
}

#[test]
fn sweep_inflection_rail_produces_surface() {
    let rail = test_fixtures::fixture_inflection_rail();
    let profile = straight_profile();
    let mut opts = SweepRailOptions::default();
    opts.n_sections = 5;
    let result = BsplineSurface::try_sweep_rail(profile, &rail, &opts);
    assert!(
        result.is_ok(),
        "sweep with inflection rail failed: {result:?}"
    );
    let surface = result.unwrap();
    // Evaluate at corners and midpoint.
    [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.5, 0.5)]
        .iter()
        .for_each(|&(u, v)| {
            let p = surface.subs(u, v);
            assert_finite(&p, &format!("subs({u}, {v})"));
        });
}

#[test]
fn birail1_converging_rails_handles_convergence() {
    let (rail1, rail2) = test_fixtures::fixture_converging_rails();
    // Profile connecting rail1 start to rail2 start.
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![
            rail1.subs(rail1.range_tuple().0),
            rail2.subs(rail2.range_tuple().0),
        ],
    );
    let mut opts = Birail1Options::default();
    opts.n_sections = 5;
    // Must not panic.
    let result = BsplineSurface::try_birail1(profile, &rail1, &rail2, &opts);
    if let Ok(ref surface) = result {
        [(0.0, 0.0), (1.0, 1.0), (0.5, 0.5)]
            .iter()
            .for_each(|&(u, v)| {
                let p = surface.subs(u, v);
                assert_finite(&p, &format!("subs({u}, {v})"));
            });
    }
    // Ok or typed error -- either is acceptable. No panic.
}

#[test]
fn sweep_degenerate_section_handles_gracefully() {
    let profile = test_fixtures::fixture_degenerate_section();
    let rail = straight_rail();
    let mut opts = SweepRailOptions::default();
    opts.n_sections = 3;
    // Must not panic.
    let result = BsplineSurface::try_sweep_rail(profile, &rail, &opts);
    if let Ok(ref surface) = result {
        [(0.0, 0.0), (1.0, 1.0), (0.5, 0.5)]
            .iter()
            .for_each(|&(u, v)| {
                let p = surface.subs(u, v);
                assert_finite(&p, &format!("subs({u}, {v})"));
            });
    }
    // Ok or typed error -- either is acceptable. No panic.
}

#[test]
fn sweep_cusped_rail_handles_gracefully() {
    let rail = test_fixtures::fixture_cusped_rail();
    let profile = straight_profile();
    let mut opts = SweepRailOptions::default();
    opts.n_sections = 5;
    // Must not panic.
    let result = BsplineSurface::try_sweep_rail(profile, &rail, &opts);
    if let Ok(ref surface) = result {
        [(0.0, 0.0), (1.0, 1.0), (0.5, 0.5)]
            .iter()
            .for_each(|&(u, v)| {
                let p = surface.subs(u, v);
                assert_finite(&p, &format!("subs({u}, {v})"));
            });
    }
    // Ok or typed error -- either is acceptable. No panic.
}

#[test]
fn near_zero_jacobian_surface_evaluates() {
    let surface = test_fixtures::fixture_near_zero_jacobian_surface();
    // Evaluate at several parameter values including the degenerate boundary (u=0).
    [
        (0.0, 0.0),
        (0.0, 0.5),
        (0.0, 1.0),
        (0.5, 0.0),
        (0.5, 0.5),
        (1.0, 0.0),
        (1.0, 1.0),
    ]
    .iter()
    .for_each(|&(u, v)| {
        let p = surface.subs(u, v);
        assert_finite(&p, &format!("subs({u}, {v})"));
    });
}

#[test]
fn collapsed_control_polygon_surface_evaluates() {
    let surface = test_fixtures::fixture_collapsed_control_polygon_surface();
    // Evaluate at several parameter values including the degenerate edge (v=0).
    [
        (0.0, 0.0),
        (0.0, 0.5),
        (0.0, 1.0),
        (0.5, 0.0),
        (0.5, 0.5),
        (1.0, 0.0),
        (1.0, 1.0),
    ]
    .iter()
    .for_each(|&(u, v)| {
        let p = surface.subs(u, v);
        assert_finite(&p, &format!("subs({u}, {v})"));
    });
}
