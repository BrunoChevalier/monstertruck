use monstertruck_geometry::prelude::*;
use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;

use super::{OffsetCurve, OffsetSurface};

// Implement `OffsetSurface` for the modeling `Surface` enum.
impl OffsetSurface for Surface {
    fn offset(&self, distance: f64, n_samples: usize) -> Option<Self> {
        match self {
            Surface::Plane(plane) => {
                // Offset a plane by translating its origin along its normal.
                let normal = plane.normal();
                let offset = normal * distance;
                let new_origin = plane.origin() + offset;
                let new_p = new_origin + plane.u_axis();
                let new_q = new_origin + plane.v_axis();
                Some(Surface::Plane(Plane::new(new_origin, new_p, new_q)))
            }
            Surface::BsplineSurface(bsp) => {
                let n = n_samples.max(4);
                offset::surface_offset(bsp, distance, (n, n))
                    .ok()
                    .map(Surface::BsplineSurface)
            }
            // Other surface types are not yet supported.
            _ => None,
        }
    }

    fn normal_at_closest(&self, pt: Point3) -> Option<(Vector3, (f64, f64))> {
        match self {
            Surface::Plane(plane) => {
                let params = plane.get_parameter(pt);
                let normal = plane.normal();
                Some((normal, (params.x, params.y)))
            }
            Surface::BsplineSurface(bsp) => {
                let (u, v) = bsp.search_nearest_parameter(pt, None, 10)?;
                let normal = ParametricSurface3D::normal(bsp, u, v);
                Some((normal, (u, v)))
            }
            _ => None,
        }
    }

    fn evaluate_at(&self, u: f64, v: f64) -> Point3 {
        match self {
            Surface::Plane(plane) => ParametricSurface::evaluate(plane, u, v),
            Surface::BsplineSurface(bsp) => ParametricSurface::evaluate(bsp, u, v),
            Surface::NurbsSurface(nbs) => ParametricSurface::evaluate(nbs, u, v),
            Surface::RevolutedCurve(rc) => ParametricSurface::evaluate(rc, u, v),
            Surface::TSplineSurface(ts) => ParametricSurface::evaluate(ts, u, v),
        }
    }
}

// Implement `OffsetCurve` for the modeling `Curve` enum.
impl OffsetCurve for Curve {
    fn offset_curve(_original: &Self, new_front: Point3, new_back: Point3) -> Self {
        // Create a straight line between the two offset endpoints.
        Curve::Line(Line(new_front, new_back))
    }
}

#[test]
fn shell_cube_valid_topology() {
    // Create unit cube.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    // Shell with 0.1 wall thickness.
    let shelled = super::shell_solid(&cube, 0.1, 20).unwrap();

    // Should have 2 boundary shells (outer + inner).
    assert_eq!(shelled.boundaries().len(), 2);

    // Both shells should be closed.
    for shell in shelled.boundaries() {
        assert_eq!(shell.shell_condition(), ShellCondition::Closed);
        assert!(shell.singular_vertices().is_empty());
        assert!(shell.extract_boundaries().is_empty());
    }

    // Outer shell: 6 faces (original), inner shell: 6 faces (offset).
    assert_eq!(shelled.boundaries()[0].len(), 6);
    assert_eq!(shelled.boundaries()[1].len(), 6);
}

#[test]
fn shell_cube_wall_thickness_geometric() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let thickness = 0.1;
    let shelled = super::shell_solid(&cube, thickness, 20).unwrap();

    // The inner shell should exist and have 6 faces.
    let inner_shell = &shelled.boundaries()[1];
    assert_eq!(inner_shell.len(), 6);

    // All inner face vertices should be offset inward from the outer face vertices.
    // For a unit cube with wall_thickness 0.1, the inner vertices should be in
    // the range [0.1, 0.9] along each axis.
    for face in inner_shell.iter() {
        for edge in face.boundaries().iter().flatten() {
            let pt = edge.front().point();
            // Each coordinate should be within [0.1 - tol, 0.9 + tol].
            assert!(
                pt.x >= 0.1 - 0.01 && pt.x <= 0.9 + 0.01,
                "Inner vertex x out of range: {pt:?}"
            );
            assert!(
                pt.y >= 0.1 - 0.01 && pt.y <= 0.9 + 0.01,
                "Inner vertex y out of range: {pt:?}"
            );
            assert!(
                pt.z >= 0.1 - 0.01 && pt.z <= 0.9 + 0.01,
                "Inner vertex z out of range: {pt:?}"
            );
        }
    }
}

#[test]
fn offset_shell_flat_surface() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let outer_shell = &cube.boundaries()[0];
    let offset = super::offset_shell(outer_shell, -0.1, 20).unwrap();

    // Offset shell should have same number of faces.
    assert_eq!(offset.len(), outer_shell.len());

    // Offset shell should be closed.
    assert_eq!(offset.shell_condition(), ShellCondition::Closed);
}

#[test]
fn shell_serialization_round_trip() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let shelled = super::shell_solid(&cube, 0.1, 20).unwrap();
    let compressed = shelled.compress();
    let json = serde_json::to_vec(&compressed).unwrap();
    let restored: monstertruck_topology::compress::CompressedSolid<Point3, Curve, Surface> =
        serde_json::from_slice(&json).unwrap();
    let restored_solid = Solid::extract(restored).unwrap();
    assert_eq!(restored_solid.boundaries().len(), 2);
}

#[test]
fn shell_negative_thickness_error() {
    // Create a small cube (side 0.3) and try to shell with a thickness (0.2) that
    // exceeds half the smallest dimension. The result should be None because the
    // inner shell would self-intersect.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::new(0.3, 0.0, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, 0.3, 0.0));
    let small_cube: Solid = builder::extrude(&f, Vector3::new(0.0, 0.0, 0.3));

    // Wall thickness 0.2 exceeds half the smallest dimension (0.15).
    // The inner shell vertices would collapse past each other.
    let result = super::shell_solid(&small_cube, 0.2, 20);
    assert!(
        result.is_none(),
        "Shell with excessive thickness should return None"
    );
}

#[test]
fn offset_shell_preserves_face_count() {
    // Verify that offset does not add or remove faces.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());
    let shell = &cube.boundaries()[0];

    // Test multiple offset distances.
    for &dist in &[-0.05, -0.1, -0.2, 0.1, 0.3] {
        let offset = super::offset_shell(shell, dist, 20).unwrap();
        assert_eq!(
            offset.len(),
            shell.len(),
            "Face count changed for distance {dist}"
        );
    }
}

#[test]
fn shell_different_thicknesses() {
    // Verify shell works with various wall thicknesses.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    for &thickness in &[0.05, 0.1, 0.2, 0.3, 0.4] {
        let shelled = super::shell_solid(&cube, thickness, 20).unwrap();
        assert_eq!(shelled.boundaries().len(), 2);

        // Inner shell should be closed.
        let inner = &shelled.boundaries()[1];
        assert_eq!(inner.shell_condition(), ShellCondition::Closed);
        assert_eq!(inner.len(), 6);
    }
}
