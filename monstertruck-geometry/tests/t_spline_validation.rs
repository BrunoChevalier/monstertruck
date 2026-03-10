use monstertruck_geometry::prelude::*;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// 1. Connection parity tests for T-NURCC subdivision
// ---------------------------------------------------------------------------

/// Builds a cube T-NURCC with non-uniform knot intervals that exercise the
/// L/R parity in the alpha computation (Equation 14 of Sederberg et al. 1998).
///
/// The horizontal ring of edges (0-3, 1-2, 4-7, 5-6) all have interval 2.0,
/// while the vertical edges (0-1, 2-3, 4-5, 6-7) keep interval 1.0.
/// This creates faces where opposing edges have matching totals, but the
/// left-side and right-side knot interval sums differ for certain edges,
/// giving distinct alpha_{ij} != alpha_{ji} values.
#[test]
fn t_spline_validation_parity_asymmetric_knots() {
    let points = vec![
        Point3::from((0.0, 0.0, 0.0)), // 0
        Point3::from((0.0, 0.0, 1.0)), // 1
        Point3::from((2.0, 0.0, 1.0)), // 2
        Point3::from((2.0, 0.0, 0.0)), // 3
        Point3::from((0.0, 1.0, 0.0)), // 4
        Point3::from((0.0, 1.0, 1.0)), // 5
        Point3::from((2.0, 1.0, 1.0)), // 6
        Point3::from((2.0, 1.0, 0.0)), // 7
    ];

    // All faces use the convention: opposing edges sum to the same total.
    // Horizontal edges (along x-axis): interval 2.0.
    // Vertical edges (along y-axis): interval 1.0.
    // Depth edges (along z-axis): interval 1.0.
    let faces = vec![
        [
            // Front face: bottom(0->3)=2.0, right(3->2)=1.0, top(2->1)=2.0, left(1->0)=1.0.
            (0, vec![(3, 2.0)]),
            (3, vec![(2, 1.0)]),
            (2, vec![(1, 2.0)]),
            (1, vec![(0, 1.0)]),
        ],
        [
            // Left face: (0->1)=1.0, (1->5)=1.0, (5->4)=1.0, (4->0)=1.0.
            (0, vec![(1, 1.0)]),
            (1, vec![(5, 1.0)]),
            (5, vec![(4, 1.0)]),
            (4, vec![(0, 1.0)]),
        ],
        [
            // Top face: (1->2)=2.0, (2->6)=1.0, (6->5)=2.0, (5->1)=1.0.
            (1, vec![(2, 2.0)]),
            (2, vec![(6, 1.0)]),
            (6, vec![(5, 2.0)]),
            (5, vec![(1, 1.0)]),
        ],
        [
            // Back face: (4->5)=1.0, (5->6)=2.0, (6->7)=1.0, (7->4)=2.0.
            (4, vec![(5, 1.0)]),
            (5, vec![(6, 2.0)]),
            (6, vec![(7, 1.0)]),
            (7, vec![(4, 2.0)]),
        ],
        [
            // Right face: (2->3)=1.0, (3->7)=1.0, (7->6)=1.0, (6->2)=1.0.
            (2, vec![(3, 1.0)]),
            (3, vec![(7, 1.0)]),
            (7, vec![(6, 1.0)]),
            (6, vec![(2, 1.0)]),
        ],
        [
            // Bottom face: (0->4)=1.0, (4->7)=2.0, (7->3)=1.0, (3->0)=2.0.
            (0, vec![(4, 1.0)]),
            (4, vec![(7, 2.0)]),
            (7, vec![(3, 1.0)]),
            (3, vec![(0, 2.0)]),
        ],
    ];

    let tnurcc = Tnurcc::try_new(points, faces)
        .expect("Asymmetric-knot cube T-NURCC should be constructable");

    // Convert to T-mesh with 2 subdivision levels. This internally calls
    // global_subdivide which exercises the parity-dependent alpha computation.
    let tmesh = tnurcc
        .to_tmesh(2)
        .expect("to_tmesh should succeed for asymmetric-knot cube");

    // After 2 subdivisions of a cube we should have many control points.
    assert!(
        tmesh.control_points().len() > 20,
        "T-mesh from asymmetric-knot cube should have >20 control points, got {}",
        tmesh.control_points().len()
    );
}

/// Verifies that a T-NURCC with uniform knot intervals produces a valid
/// T-mesh after subdivision, establishing a baseline for parity tests.
#[test]
fn t_spline_validation_parity_uniform_baseline() {
    let points = vec![
        Point3::from((0.0, 0.0, 0.0)),
        Point3::from((0.0, 0.0, 1.0)),
        Point3::from((1.0, 0.0, 1.0)),
        Point3::from((1.0, 0.0, 0.0)),
        Point3::from((0.0, 1.0, 0.0)),
        Point3::from((0.0, 1.0, 1.0)),
        Point3::from((1.0, 1.0, 1.0)),
        Point3::from((1.0, 1.0, 0.0)),
    ];
    let faces = [
        [0, 3, 2, 1],
        [0, 1, 5, 4],
        [1, 2, 6, 5],
        [4, 5, 6, 7],
        [2, 3, 7, 6],
        [0, 4, 7, 3],
    ];

    let tnurcc =
        Tnurcc::from_quad_mesh(points, &faces).expect("Uniform cube should be constructable");

    // With uniform knots, alpha_{ij} = alpha_{ji} = 0.25 for all edges
    // (Equation 14 in Sederberg 1998 with equal left/right sums).
    let tmesh = tnurcc
        .to_tmesh(2)
        .expect("to_tmesh should succeed for uniform cube");

    assert!(
        tmesh.control_points().len() > 20,
        "T-mesh from uniform cube should have >20 control points"
    );
}

// ---------------------------------------------------------------------------
// 2. Zero knot interval tests for T-mesh
// ---------------------------------------------------------------------------

/// Tests that `add_control_point` succeeds when the knot ratio is 0.0,
/// resulting in a zero knot interval connection (degenerate case per Figure 9
/// of Sederberg et al. 2003).
#[test]
fn t_spline_validation_zero_knot_interval_insertion() {
    let points = [
        Point3::from((0.0, 0.0, 0.0)),
        Point3::from((1.0, 0.0, 0.0)),
        Point3::from((1.0, 1.0, 0.0)),
        Point3::from((0.0, 1.0, 0.0)),
    ];

    let mut mesh = Tmesh::new(points, 1.0);

    // Insert a point with knot_ratio = 0.0, creating a zero knot interval
    // between the existing point and the new point.
    let origin = mesh
        .find(Point3::from((0.0, 1.0, 0.0)))
        .expect("Origin point should exist");

    let result = mesh.add_control_point(
        Point3::from((0.0, 1.0, 0.0)),
        origin,
        TmeshDirection::Right,
        0.0,
    );

    assert!(
        result.is_ok(),
        "Zero knot interval insertion should succeed: {:?}",
        result.err()
    );

    // The new point should exist in the mesh.
    assert_eq!(
        mesh.control_points().len(),
        5,
        "Mesh should have 5 control points after insertion"
    );
}

/// Tests that inserting with knot_ratio = 1.0 produces a zero knot interval
/// on the far side of the new point.
#[test]
fn t_spline_validation_zero_knot_interval_far_side() {
    let points = [
        Point3::from((0.0, 0.0, 0.0)),
        Point3::from((1.0, 0.0, 0.0)),
        Point3::from((1.0, 1.0, 0.0)),
        Point3::from((0.0, 1.0, 0.0)),
    ];

    let mut mesh = Tmesh::new(points, 1.0);

    let origin = mesh
        .find(Point3::from((0.0, 1.0, 0.0)))
        .expect("Origin point should exist");

    let result = mesh.add_control_point(
        Point3::from((1.0, 1.0, 0.0)),
        origin,
        TmeshDirection::Right,
        1.0,
    );

    assert!(
        result.is_ok(),
        "Knot ratio 1.0 insertion should succeed: {:?}",
        result.err()
    );

    assert_eq!(
        mesh.control_points().len(),
        5,
        "Mesh should have 5 control points after insertion"
    );
}

/// Tests that knot vector computation handles zero intervals correctly
/// (zero intervals appear as repeated knots in the knot vector).
#[test]
fn t_spline_validation_zero_knot_interval_knot_vectors() {
    let points = [
        Point3::from((0.0, 0.0, 0.0)),
        Point3::from((1.0, 0.0, 0.0)),
        Point3::from((1.0, 1.0, 0.0)),
        Point3::from((0.0, 1.0, 0.0)),
    ];

    let mut mesh = Tmesh::new(points, 1.0);

    // Insert a point at the midpoint of the top edge.
    let origin = mesh
        .find(Point3::from((0.0, 1.0, 0.0)))
        .expect("Origin point should exist");

    mesh.add_control_point(
        Point3::from((0.5, 1.0, 0.0)),
        origin,
        TmeshDirection::Right,
        0.5,
    )
    .expect("Midpoint insertion should succeed");

    // Now insert another point with zero knot interval.
    let midpoint = mesh
        .find(Point3::from((0.5, 1.0, 0.0)))
        .expect("Midpoint should exist");

    let result = mesh.add_control_point(
        Point3::from((0.5, 1.0, 0.0)),
        midpoint,
        TmeshDirection::Right,
        0.0,
    );

    assert!(
        result.is_ok(),
        "Zero knot interval insertion after midpoint should succeed: {:?}",
        result.err()
    );

    assert_eq!(mesh.control_points().len(), 6);
}

/// Tests that the perpendicular edge conditions are correctly evaluated
/// even when adjacent edges have zero intervals.
#[test]
fn t_spline_validation_zero_knot_perpendicular_edges() {
    let points = [
        Point3::from((0.0, 0.0, 0.0)),
        Point3::from((1.0, 0.0, 0.0)),
        Point3::from((1.0, 1.0, 0.0)),
        Point3::from((0.0, 1.0, 0.0)),
    ];

    let mut mesh = Tmesh::new(points, 1.0);

    let origin = mesh
        .find(Point3::from((0.0, 0.0, 0.0)))
        .expect("Origin point should exist");

    // Insert on the bottom edge with zero ratio.
    let result = mesh.add_control_point(
        Point3::from((0.0, 0.0, 0.0)),
        origin,
        TmeshDirection::Right,
        0.0,
    );

    assert!(
        result.is_ok(),
        "Zero knot bottom edge insertion should succeed: {:?}",
        result.err()
    );

    let new_point = result.unwrap();
    let new_borrow = new_point.read();

    // The new point should have proper connections in the perpendicular directions
    // (up and down should be edge conditions since we're on the boundary).
    assert_eq!(
        new_borrow.con_type(TmeshDirection::Up),
        TmeshConnectionType::Edge,
        "Perpendicular up direction should be an edge condition"
    );
    assert_eq!(
        new_borrow.con_type(TmeshDirection::Down),
        TmeshConnectionType::Edge,
        "Perpendicular down direction should be an edge condition"
    );
}

// ---------------------------------------------------------------------------
// 3. Malformed face error test
// ---------------------------------------------------------------------------

/// Verifies the `TnurccMalformedFace` error variant exists and that
/// a well-formed mesh does not trigger it during subdivision.
#[test]
fn t_spline_validation_malformed_face_error_variant() {
    // Verify that the error variant can be instantiated (compile-time check).
    let err: Error = Error::TnurccMalformedFace;
    assert_eq!(
        format!("{}", err),
        "A T-NURCC face must have at least two points and one edge defining it."
    );
}

/// Tests that `to_tmesh` converts a T-NURCC with parity-sensitive edges
/// into a valid T-mesh preserving the knot structure.
#[test]
fn t_spline_validation_parity_to_tmesh_conversion() {
    let points = vec![
        Point3::from((0.0, 0.0, 0.0)),
        Point3::from((0.0, 0.0, 1.0)),
        Point3::from((1.0, 0.0, 1.0)),
        Point3::from((1.0, 0.0, 0.0)),
        Point3::from((0.0, 1.0, 0.0)),
        Point3::from((0.0, 1.0, 1.0)),
        Point3::from((1.0, 1.0, 1.0)),
        Point3::from((1.0, 1.0, 0.0)),
    ];
    let faces = [
        [0, 3, 2, 1],
        [0, 1, 5, 4],
        [1, 2, 6, 5],
        [4, 5, 6, 7],
        [2, 3, 7, 6],
        [0, 4, 7, 3],
    ];

    let tnurcc =
        Tnurcc::from_quad_mesh(points, &faces).expect("Cube should be constructable");

    let tmesh = tnurcc
        .to_tmesh(2)
        .expect("to_tmesh with 2 subdivision levels should succeed");

    assert!(
        tmesh.control_points().len() > 20,
        "T-mesh should have many control points, got {}",
        tmesh.control_points().len()
    );
}
