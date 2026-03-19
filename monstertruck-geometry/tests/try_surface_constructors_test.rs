use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::{Birail1Options, Birail2Options, SweepRailOptions};
use monstertruck_geometry::prelude::*;

fn straight_rail() -> BsplineCurve<Point3> {
    BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0)],
    )
}

fn straight_profile() -> BsplineCurve<Point3> {
    BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    )
}

// --- try_sweep_rail ---

#[test]
fn try_sweep_rail_success_default_options() {
    let profile = straight_profile();
    let rail = straight_rail();
    let opts = SweepRailOptions { n_sections: 3, ..Default::default() };
    let result = BsplineSurface::try_sweep_rail(profile, &rail, &opts);
    assert!(result.is_ok());
    let surface = result.unwrap();
    assert_near2!(surface.subs(0.0, 0.0), Point3::new(-1.0, 0.0, 0.0));
    assert_near2!(surface.subs(1.0, 1.0), Point3::new(1.0, 0.0, 5.0));
}

#[test]
fn try_sweep_rail_insufficient_sections() {
    let profile = straight_profile();
    let rail = straight_rail();
    let opts = SweepRailOptions { n_sections: 1, ..Default::default() };
    let result = BsplineSurface::try_sweep_rail(profile, &rail, &opts);
    assert!(matches!(
        result,
        Err(Error::InsufficientSections { required: 2, got: 1 })
    ));
}

// --- try_birail1 ---

#[test]
fn try_birail1_success() {
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
    let opts = Birail1Options { n_sections: 3 };
    let result = BsplineSurface::try_birail1(profile, &rail1, &rail2, &opts);
    assert!(result.is_ok());
}

#[test]
fn try_birail1_insufficient_sections() {
    let rail1 = straight_rail();
    let rail2 = straight_rail();
    let profile = straight_profile();
    let opts = Birail1Options { n_sections: 0 };
    let result = BsplineSurface::try_birail1(profile, &rail1, &rail2, &opts);
    assert!(matches!(
        result,
        Err(Error::InsufficientSections { required: 2, got: 0 })
    ));
}

// --- try_birail2 ---

#[test]
fn try_birail2_success() {
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
    let opts = Birail2Options { n_sections: 3 };
    let result = BsplineSurface::try_birail2(profile1, profile2, &rail1, &rail2, &opts);
    assert!(result.is_ok());
}

#[test]
fn try_birail2_insufficient_sections() {
    let rail1 = straight_rail();
    let rail2 = straight_rail();
    let profile1 = straight_profile();
    let profile2 = straight_profile();
    let opts = Birail2Options { n_sections: 1 };
    let result = BsplineSurface::try_birail2(profile1, profile2, &rail1, &rail2, &opts);
    assert!(matches!(
        result,
        Err(Error::InsufficientSections { required: 2, got: 1 })
    ));
}

// --- Deprecated old APIs still work ---

#[test]
#[allow(deprecated)]
fn deprecated_sweep_rail_still_works() {
    let profile = straight_profile();
    let rail = straight_rail();
    let _surface = BsplineSurface::sweep_rail(profile, &rail, 3);
}

#[test]
#[allow(deprecated)]
fn deprecated_birail1_still_works() {
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
    let _surface = BsplineSurface::birail1(profile, &rail1, &rail2, 3);
}
