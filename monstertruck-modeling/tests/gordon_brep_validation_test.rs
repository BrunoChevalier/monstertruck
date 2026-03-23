//! B-rep topology validation tests for Gordon surfaces.
//!
//! Proves that Gordon surfaces built from curved curve networks produce
//! valid [`Face`] topology with correct boundary structure and pass
//! [`ShellCondition`] validation.

use monstertruck_geometry::nurbs::surface_options::GordonOptions;
use monstertruck_modeling::*;

/// Builds a quadratic B-spline curve from three control points.
fn quadratic_arc(start: Point3, mid_ctrl: Point3, end: Point3) -> BsplineCurve<Point3> {
    BsplineCurve::new(KnotVector::bezier_knot(2), vec![start, mid_ctrl, end])
}

/// Builds a 2x2 curved network for Gordon surface construction.
fn make_curved_2x2_network() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>) {
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
    (vec![u0, u1], vec![v0, v1])
}

#[test]
fn gordon_face_has_valid_boundary_topology() {
    let (u_curves, v_curves) = make_curved_2x2_network();
    let face: Face =
        builder::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default())
            .expect("Gordon face construction should succeed");

    // A Gordon face from a 4-sided surface patch should have exactly 1 boundary
    // wire with 4 edges (quad boundary: bottom, right, top, left).
    assert_eq!(
        face.boundaries().len(),
        1,
        "Gordon face should have exactly 1 boundary wire"
    );
    assert_eq!(
        face.boundaries()[0].len(),
        4,
        "Gordon face boundary wire should have 4 edges"
    );
}

#[test]
fn gordon_shell_passes_shell_condition() {
    // Build two adjacent Gordon faces side-by-side.
    // Face A covers y=[0,1], Face B covers y=[1,2].
    // Since they are independently constructed, they don't share edges,
    // but each face's boundary is well-formed. The shell condition for
    // independently-constructed faces should be at least Oriented.
    let (u_a, v_a) = make_curved_2x2_network();
    let face_a: Face = builder::try_gordon_from_network(u_a, v_a, &GordonOptions::default())
        .expect("Face A construction should succeed");

    // Face B: shifted to y=[1,2].
    let u0_b = quadratic_arc(
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.5, 1.0, 0.3),
        Point3::new(1.0, 1.0, 0.0),
    );
    let u1_b = quadratic_arc(
        Point3::new(0.0, 2.0, 0.0),
        Point3::new(0.5, 2.0, 0.3),
        Point3::new(1.0, 2.0, 0.0),
    );
    let v0_b = quadratic_arc(
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.5, 0.2),
        Point3::new(0.0, 2.0, 0.0),
    );
    let v1_b = quadratic_arc(
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.5, 0.2),
        Point3::new(1.0, 2.0, 0.0),
    );
    let face_b: Face = builder::try_gordon_from_network(
        vec![u0_b, u1_b],
        vec![v0_b, v1_b],
        &GordonOptions::default(),
    )
    .expect("Face B construction should succeed");

    let shell: Shell = vec![face_a, face_b].into_iter().collect();
    let condition = shell.shell_condition();

    // Each face has well-formed boundary edges. The shell with independent faces
    // has condition Oriented (each edge appears once, consistently oriented).
    // Oriented >= Regular, satisfying the B-rep validity requirement.
    assert!(
        condition == ShellCondition::Oriented || condition == ShellCondition::Closed,
        "Shell condition should be at least Regular, got {:?}",
        condition
    );
}

#[test]
fn gordon_face_surface_evaluates_correctly() {
    // Build a Gordon face and verify the underlying surface evaluates correctly
    // at sample points, producing finite 3D coordinates.
    let (u_curves, v_curves) = make_curved_2x2_network();
    let face: Face =
        builder::try_gordon_from_network(u_curves, v_curves, &GordonOptions::default())
            .expect("Gordon face construction should succeed");

    // Extract the surface and evaluate at a grid of parameter values.
    let surface = face.surface();
    // The surface should evaluate to finite points everywhere in its domain.
    (0..=10).for_each(|i| {
        (0..=10).for_each(|j| {
            let u = i as f64 / 10.0;
            let v = j as f64 / 10.0;
            let pt = surface.subs(u, v);
            assert!(
                pt.x.is_finite() && pt.y.is_finite() && pt.z.is_finite(),
                "Surface evaluation at ({u}, {v}) should produce finite coordinates, got {pt:?}"
            );
        });
    });

    // Corner evaluations should match curve intersection points.
    let p00 = surface.subs(0.0, 0.0);
    let p11 = surface.subs(1.0, 1.0);
    assert_near2!(p00, Point3::new(0.0, 0.0, 0.0));
    assert_near2!(p11, Point3::new(1.0, 1.0, 0.0));
}
