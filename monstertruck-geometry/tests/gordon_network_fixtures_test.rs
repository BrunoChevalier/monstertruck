//! Integration tests for Gordon-specific network fixtures.
//!
//! Exercises [`test_fixtures`](monstertruck_geometry::nurbs::test_fixtures)
//! fixtures through
//! [`try_gordon_from_network`](monstertruck_geometry::nurbs::BsplineSurface::try_gordon_from_network)
//! and
//! [`try_gordon_verified`](monstertruck_geometry::nurbs::BsplineSurface::try_gordon_verified).

use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::GordonOptions;
use monstertruck_geometry::nurbs::test_fixtures;
use monstertruck_geometry::prelude::*;

#[test]
fn gordon_near_miss_grid_snaps_successfully() {
    let (u_curves, v_curves, grid_points) = test_fixtures::fixture_gordon_near_miss_grid();
    let result = BsplineSurface::try_gordon_verified(
        u_curves,
        v_curves,
        &grid_points,
        &GordonOptions::default(),
    );
    assert!(
        result.is_ok(),
        "Near-miss grid points within SNAP_TOLERANCE should snap, got {:?}",
        result.err()
    );
    let surface = result.unwrap();
    // Evaluate at corners and verify finite results.
    let p00 = surface.subs(0.0, 0.0);
    let p11 = surface.subs(1.0, 1.0);
    assert!(p00.x.is_finite() && p00.y.is_finite() && p00.z.is_finite());
    assert!(p11.x.is_finite() && p11.y.is_finite() && p11.z.is_finite());
}

#[test]
fn gordon_near_miss_grid_rejects_with_tight_tolerance() {
    let (u_curves, v_curves, grid_points) = test_fixtures::fixture_gordon_near_miss_grid();
    let mut opts = GordonOptions::default();
    // Much tighter than the SNAP_TOLERANCE * 0.5 offset applied to fixture points.
    opts.grid_tolerance = 1e-10;
    let result = BsplineSurface::try_gordon_verified(u_curves, v_curves, &grid_points, &opts);
    assert!(
        matches!(
            result,
            Err(Error::CurveNetworkIncompatible(
                CurveNetworkDiagnostic::GridPointNotOnCurve { .. }
            ))
        ),
        "Tight tolerance should reject near-miss points, got {:?}",
        result
    );
}

#[test]
fn gordon_nonuniform_spacing_from_network() {
    let (u_curves, v_curves) = test_fixtures::fixture_gordon_nonuniform_spacing();
    let result =
        BsplineSurface::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default());
    assert!(
        result.is_ok(),
        "Nonuniform spacing network should succeed, got {:?}",
        result.err()
    );
    let surface = result.unwrap();
    // The surface should interpolate the corner points of the planar grid.
    assert_near2!(surface.subs(0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
    assert_near2!(surface.subs(1.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    assert_near2!(surface.subs(0.0, 1.0), Point3::new(0.0, 1.0, 0.0));
    assert_near2!(surface.subs(1.0, 1.0), Point3::new(1.0, 1.0, 0.0));
}

#[test]
fn gordon_high_degree_family_from_network() {
    let (u_curves, v_curves) = test_fixtures::fixture_gordon_high_degree_family();
    let result =
        BsplineSurface::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default());
    assert!(
        result.is_ok(),
        "High-degree (quartic) network should succeed, got {:?}",
        result.err()
    );
    let surface = result.unwrap();
    // Evaluate at corners and verify finite results.
    let p00 = surface.subs(0.0, 0.0);
    assert!(p00.x.is_finite() && p00.y.is_finite() && p00.z.is_finite());
    let p11 = surface.subs(1.0, 1.0);
    assert!(p11.x.is_finite() && p11.y.is_finite() && p11.z.is_finite());
}

#[test]
fn gordon_curved_network_from_network() {
    let (u_curves, v_curves) = test_fixtures::fixture_gordon_curved_network();
    let result =
        BsplineSurface::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default());
    assert!(
        result.is_ok(),
        "Curved network should succeed, got {:?}",
        result.err()
    );
    let surface = result.unwrap();
    // Surface should pass through the curve endpoints at corners.
    // All four corner curves start/end at z=0.
    assert_near2!(surface.subs(0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
    assert_near2!(surface.subs(1.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    assert_near2!(surface.subs(0.0, 1.0), Point3::new(0.0, 1.0, 0.0));
    assert_near2!(surface.subs(1.0, 1.0), Point3::new(1.0, 1.0, 0.0));
}

#[test]
fn gordon_curved_network_verified_with_computed_points() {
    let (u_curves, v_curves) = test_fixtures::fixture_gordon_curved_network();
    let (u_curves2, v_curves2) = test_fixtures::fixture_gordon_curved_network();

    // Compute intersection points at curve endpoints (network corners).
    // For a 2x2 network, the grid is 2x2.
    // u_curves[i] evaluated at the parameter where v_curves[j] crosses.
    let grid_points = vec![
        vec![
            // (u0 at x=0, v0 at y=0) = (0,0,0).
            Point3::new(0.0, 0.0, 0.0),
            // (u0 at x=1, v1 at y=0) = (1,0,0).
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![
            // (u1 at x=0, v0 at y=1) = (0,1,0).
            Point3::new(0.0, 1.0, 0.0),
            // (u1 at x=1, v1 at y=1) = (1,1,0).
            Point3::new(1.0, 1.0, 0.0),
        ],
    ];

    let result_verified = BsplineSurface::try_gordon_verified(
        u_curves,
        v_curves,
        &grid_points,
        &GordonOptions::default(),
    );
    assert!(
        result_verified.is_ok(),
        "Verified with computed points should succeed, got {:?}",
        result_verified.err()
    );

    let result_from_network =
        BsplineSurface::try_gordon_from_network(u_curves2, v_curves2, &GordonOptions::default());
    assert!(result_from_network.is_ok());

    let verified = result_verified.unwrap();
    let from_network = result_from_network.unwrap();

    // Both surfaces should agree at sampled parameter values.
    (0..=10).for_each(|i| {
        (0..=10).for_each(|j| {
            let u = i as f64 / 10.0;
            let v = j as f64 / 10.0;
            assert_near2!(verified.subs(u, v), from_network.subs(u, v));
        });
    });
}
