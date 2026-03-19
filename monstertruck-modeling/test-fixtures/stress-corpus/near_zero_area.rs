//! Near-zero-area loop fixtures for the stress corpus.

use monstertruck_modeling::*;

/// A long, extremely thin rectangular wire with near-zero enclosed area.
///
/// Constructs a rectangle of width ~1e-8 and height 1.0, producing
/// a contour whose enclosed area is approximately 1e-8. The extreme
/// aspect ratio stresses signed-area calculations.
///
/// **Failure mode:** Tests whether the pipeline handles near-degenerate
/// planes where the normal vector may be poorly conditioned. The tiny
/// width approaches floating-point tolerance thresholds.
///
/// **Real-world analog:** Hairline strokes in thin-weight fonts, or
/// font hinting artifacts that produce near-zero-width features.
pub fn near_zero_area_sliver() -> Vec<Wire> {
    let w = 1e-8;
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(w, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(w, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 1.0, 0.0));

    let e0: Edge = builder::line(&v0, &v1);
    let e1: Edge = builder::line(&v1, &v2);
    let e2: Edge = builder::line(&v2, &v3);
    let e3: Edge = builder::line(&v3, &v0);

    vec![vec![e0, e1, e2, e3].into()]
}

/// A contour built from quadratic Beziers where control points nearly
/// coincide with the endpoints.
///
/// Each edge is a quadratic Bezier (one interior control point) where
/// the control point is offset from the midpoint of the endpoints by
/// only ~1e-10. This produces micro-curves that are nearly straight
/// lines.
///
/// **Failure mode:** Tests whether the pipeline handles Bezier curves
/// where the curvature is below tolerance thresholds. The near-zero
/// deviation may cause numerical instability in tangent and normal
/// computations.
///
/// **Real-world analog:** Font outlines that use quadratic Beziers
/// to approximate straight lines (common in TrueType fonts for grid
/// fitting).
pub fn collapsed_quad_bezier() -> Vec<Wire> {
    let eps = 1e-10;
    let corners = [
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ];

    let vertices: Vec<Vertex> = corners
        .iter()
        .map(|&(x, y)| builder::vertex(Point3::new(x, y, 0.0)))
        .collect();

    let edges: Vec<Edge> = (0..4)
        .map(|i| {
            let j = (i + 1) % 4;
            let (x0, y0) = corners[i];
            let (x1, y1) = corners[j];
            // Control point is the midpoint plus a tiny offset.
            let mid = Point3::new(
                (x0 + x1) / 2.0 + eps,
                (y0 + y1) / 2.0 + eps,
                0.0,
            );
            builder::bezier(&vertices[i], &vertices[j], vec![mid])
        })
        .collect();

    vec![edges.into()]
}

/// A tiny closed contour simulating a sub-pixel glyph feature.
///
/// All coordinates lie within a 1e-6 x 1e-6 bounding box, producing
/// a contour that is smaller than a typical pixel. Tests whether the
/// pipeline preserves or filters micro-features.
///
/// **Failure mode:** Tests whether extremely small geometry is handled
/// without numerical breakdown. Cross products and area calculations
/// may underflow to zero.
///
/// **Real-world analog:** Sub-pixel diacritical marks in very small
/// font sizes, or decorative micro-features in ornamental fonts.
pub fn micro_feature_loop() -> Vec<Wire> {
    let s = 1e-6;
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(s, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(s, s, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, s, 0.0));

    let e0: Edge = builder::line(&v0, &v1);
    let e1: Edge = builder::line(&v1, &v2);
    let e2: Edge = builder::line(&v2, &v3);
    let e3: Edge = builder::line(&v3, &v0);

    vec![vec![e0, e1, e2, e3].into()]
}
