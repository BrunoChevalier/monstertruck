//! Degenerate geometry fixtures for the stress corpus.

use monstertruck_modeling::*;

/// A closed contour of cubic Bezier edges where all control points
/// coincide with one endpoint.
///
/// Each edge is a cubic Bezier where both interior control points
/// are placed at the start vertex position. This makes each Bezier
/// collapse to near-degenerate geometry (the curve hugs the start
/// point then jumps to the end).
///
/// **Failure mode:** Tests whether the pipeline handles Bezier edges
/// that effectively collapse to points. Tangent computation at the
/// start of such curves produces zero vectors, which can cause
/// division-by-zero in normalization.
///
/// **Real-world analog:** Font editor artifacts where control points
/// are accidentally placed on top of anchor points.
pub fn coincident_control_points() -> Vec<Wire> {
    let corners = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(2.0, 2.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
    ];

    let vertices: Vec<Vertex> = corners.iter().map(|&p| builder::vertex(p)).collect();

    let edges: Vec<Edge> = (0..4)
        .map(|i| {
            let j = (i + 1) % 4;
            // Both control points coincide with the start point.
            let cp = corners[i];
            builder::bezier(&vertices[i], &vertices[j], vec![cp, cp])
        })
        .collect();

    vec![edges.into()]
}

/// An outer CCW rectangle with an inner CW rectangle (pre-wound hole).
///
/// The inner rectangle is explicitly wound clockwise using
/// [`Wire::inverse()`]. This tests that the normalization pipeline
/// correctly handles holes that are already wound in the expected
/// direction without double-inverting them.
///
/// **Failure mode:** Tests whether `classify_and_normalize` detects
/// that a hole is already CW and leaves it unchanged. A naive
/// implementation might invert all inner wires unconditionally,
/// producing a CCW hole (wrong orientation).
///
/// **Real-world analog:** Font outlines from editors that pre-normalize
/// winding order before export (e.g., FontForge with "correct direction"
/// applied).
pub fn reverse_wound_hole() -> Vec<Wire> {
    // Outer rectangle: (0,0) to (4,4), wound CCW.
    let outer = make_rectangle_ccw(0.0, 0.0, 4.0, 4.0);

    // Inner rectangle: (1,1) to (3,3), wound CW via inverse.
    let inner_ccw = make_rectangle_ccw(1.0, 1.0, 3.0, 3.0);
    let inner_cw = inner_ccw.inverse();

    vec![outer, inner_cw]
}

/// A wire where two consecutive vertices are at the same point,
/// creating a zero-length edge.
///
/// Constructs a rectangular contour with an extra vertex inserted
/// at the same position as one corner, producing a degenerate
/// zero-length edge between two coincident vertices.
///
/// **Failure mode:** Tests whether the pipeline gracefully handles
/// zero-length edges. Edge direction vectors are zero, causing
/// issues in tangent and normal computation.
///
/// **Real-world analog:** Font outlines with redundant on-curve
/// points at the same position (common in fonts with complex
/// hinting instructions).
pub fn single_point_degeneracy() -> Vec<Wire> {
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    // v2 is at the exact same position as v1, creating a zero-length edge.
    let v2 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v3 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v4 = builder::vertex(Point3::new(0.0, 1.0, 0.0));

    let e0: Edge = builder::line(&v0, &v1);
    // Zero-length edge.
    let e1: Edge = builder::line(&v1, &v2);
    let e2: Edge = builder::line(&v2, &v3);
    let e3: Edge = builder::line(&v3, &v4);
    let e4: Edge = builder::line(&v4, &v0);

    vec![vec![e0, e1, e2, e3, e4].into()]
}

/// Helper: constructs a closed rectangular wire wound CCW.
fn make_rectangle_ccw(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, y0, 0.0));
    let v1 = builder::vertex(Point3::new(x1, y0, 0.0));
    let v2 = builder::vertex(Point3::new(x1, y1, 0.0));
    let v3 = builder::vertex(Point3::new(x0, y1, 0.0));

    let e0: Edge = builder::line(&v0, &v1);
    let e1: Edge = builder::line(&v1, &v2);
    let e2: Edge = builder::line(&v2, &v3);
    let e3: Edge = builder::line(&v3, &v0);

    vec![e0, e1, e2, e3].into()
}
