//! Tests verifying that deprecated surface constructors delegate correctly
//! to their `try_*` counterparts, producing identical results.

use monstertruck_geometry::nurbs::surface_options::{
    Birail1Options, Birail2Options, GordonOptions, SkinOptions, SweepRailOptions,
};
use monstertruck_geometry::prelude::*;

// --- skin delegation ---

#[test]
#[allow(deprecated)]
fn deprecated_skin_matches_try_skin_two_curves() {
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 0.0), Vector2::new(1.0, 0.0)],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Vector2::new(0.0, 1.0), Vector2::new(1.0, 1.0)],
    );
    let deprecated_surface = BsplineSurface::skin(vec![c0.clone(), c1.clone()]);
    let try_surface =
        BsplineSurface::try_skin(vec![c0, c1], &SkinOptions::default()).unwrap();

    // Both surfaces must evaluate identically at sample points.
    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_near2!(deprecated_surface.subs(u, v), try_surface.subs(u, v));
        }
    }
}

#[test]
#[allow(deprecated)]
fn deprecated_skin_matches_try_skin_three_curves() {
    let c0 = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector2::new(0.0, 0.0),
            Vector2::new(0.5, 0.2),
            Vector2::new(1.0, 0.0),
        ],
    );
    let c1 = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector2::new(0.0, 1.0),
            Vector2::new(0.5, 0.8),
            Vector2::new(1.0, 1.0),
        ],
    );
    let c2 = BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector2::new(0.0, 2.0),
            Vector2::new(0.5, 1.5),
            Vector2::new(1.0, 2.0),
        ],
    );
    let deprecated_surface =
        BsplineSurface::skin(vec![c0.clone(), c1.clone(), c2.clone()]);
    let try_surface =
        BsplineSurface::try_skin(vec![c0, c1, c2], &SkinOptions::default()).unwrap();

    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_near2!(deprecated_surface.subs(u, v), try_surface.subs(u, v));
        }
    }
}

// --- gordon delegation ---

#[test]
#[allow(deprecated)]
fn deprecated_gordon_matches_try_gordon() {
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

    let deprecated_gordon = BsplineSurface::gordon(
        vec![u0.clone(), u1.clone()],
        vec![v0.clone(), v1.clone()],
        &points,
    );
    let try_gordon = BsplineSurface::try_gordon(
        vec![u0, u1],
        vec![v0, v1],
        &points,
        &GordonOptions::default(),
    )
    .unwrap();

    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_near2!(deprecated_gordon.subs(u, v), try_gordon.subs(u, v));
        }
    }
}

// --- sweep_rail delegation ---

#[test]
#[allow(deprecated)]
fn deprecated_sweep_rail_matches_try_sweep_rail() {
    let rail = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    );
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );

    let deprecated_surface =
        BsplineSurface::sweep_rail(profile.clone(), &rail, 3);
    let mut opts = SweepRailOptions::default();
    opts.n_sections = 3;
    let try_surface =
        BsplineSurface::try_sweep_rail(profile, &rail, &opts).unwrap();

    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_near2!(deprecated_surface.subs(u, v), try_surface.subs(u, v));
        }
    }
}

// --- birail1 delegation ---

#[test]
#[allow(deprecated)]
fn deprecated_birail1_matches_try_birail1() {
    let rail1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 5.0)],
    );
    let rail2 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 5.0)],
    );
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );

    let deprecated_surface =
        BsplineSurface::birail1(profile.clone(), &rail1, &rail2, 3);
    let mut opts = Birail1Options::default();
    opts.n_sections = 3;
    let try_surface =
        BsplineSurface::try_birail1(profile, &rail1, &rail2, &opts).unwrap();

    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_near2!(deprecated_surface.subs(u, v), try_surface.subs(u, v));
        }
    }
}

// --- birail2 delegation ---

#[test]
#[allow(deprecated)]
fn deprecated_birail2_matches_try_birail2() {
    let rail1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 4.0)],
    );
    let rail2 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(2.0, 0.0, 0.0), Point3::new(2.0, 0.0, 4.0)],
    );
    let profile1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0)],
    );
    let profile2 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 4.0), Point3::new(2.0, 0.0, 4.0)],
    );

    let deprecated_surface = BsplineSurface::birail2(
        profile1.clone(),
        profile2.clone(),
        &rail1,
        &rail2,
        3,
    );
    let mut opts = Birail2Options::default();
    opts.n_sections = 3;
    let try_surface = BsplineSurface::try_birail2(
        profile1, profile2, &rail1, &rail2, &opts,
    )
    .unwrap();

    for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_near2!(deprecated_surface.subs(u, v), try_surface.subs(u, v));
        }
    }
}
