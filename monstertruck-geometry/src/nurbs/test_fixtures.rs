//! Programmatic fixture generators for problematic NURBS curves and surfaces.
//!
//! This module is **unconditionally compiled** (no `#[cfg(test)]` gate) so that
//! other crates' test code can import it via
//! `use monstertruck_geometry::nurbs::test_fixtures::*`.
//!
//! Each fixture function returns a well-formed [`BsplineCurve`] or
//! [`BsplineSurface`] that exercises a specific numerical edge case.

use super::*;

// ---------------------------------------------------------------------------
// Near-degenerate NURBS cases
// ---------------------------------------------------------------------------

/// A cubic curve where two adjacent control points are nearly coincident
/// (distance < 1e-10), causing numerical issues in parameter searches.
pub fn degenerate_collapsed_control_points() -> BsplineCurve<Point3> {
    let knot_vec = KnotVector::bezier_knot(3);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0 + 1e-11, 0.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// A cubic curve with a knot span of ~1e-12, creating a near-C0 discontinuity.
pub fn degenerate_near_zero_knot_span() -> BsplineCurve<Point3> {
    let knot_vec = KnotVector::from(vec![
        0.0,
        0.0,
        0.0,
        0.0,
        0.5,
        0.5 + 1e-12,
        1.0,
        1.0,
        1.0,
        1.0,
    ]);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.3, 1.0, 0.0),
        Point3::new(0.5, 0.0, 0.0),
        Point3::new(0.7, 1.0, 0.0),
        Point3::new(0.8, -1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// A cubic curve with control points arranged to create extreme curvature
/// at one end (curvature radius < 1e-6).
pub fn degenerate_high_curvature_pole() -> BsplineCurve<Point3> {
    let knot_vec = KnotVector::bezier_knot(3);
    // The first three control points are extremely close, forcing a tight turn.
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1e-7, 1e-7, 0.0),
        Point3::new(2e-7, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// A bi-quadratic surface where one edge (v=0) collapses to a single point,
/// like a cone tip or sphere pole. The boundary along v=0 is degenerate.
pub fn degenerate_surface_collapsed_edge() -> BsplineSurface<Point3> {
    let knot_vec_u = KnotVector::bezier_knot(2);
    let knot_vec_v = KnotVector::bezier_knot(2);
    // All control points in the first row collapse to the origin.
    let control_points = vec![
        vec![
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 1.0),
        ],
        vec![
            Point3::new(-0.5, 0.0, 0.5),
            Point3::new(0.0, 0.5, 0.5),
            Point3::new(0.5, 0.0, 0.5),
        ],
        vec![
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
    ];
    BsplineSurface::new((knot_vec_u, knot_vec_v), control_points)
}

// ---------------------------------------------------------------------------
// Problematic rail/section combos
// ---------------------------------------------------------------------------

/// A cubic rail with a sharp kink (tangent reversal) at the midpoint,
/// causing sweep_rail framing to flip.
pub fn fixture_kinked_rail() -> BsplineCurve<Point3> {
    let knot_vec = KnotVector::from(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0]);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 2.0),
        // Reversal: tangent flips back.
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.5),
        Point3::new(0.0, 1.0, 4.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// Two rails that diverge wildly, causing birail profile stretching
/// beyond reasonable bounds.
pub fn fixture_diverging_rails() -> (BsplineCurve<Point3>, BsplineCurve<Point3>) {
    let knot = KnotVector::bezier_knot(3);
    let rail1 = BsplineCurve::new(
        knot.clone(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, 3.0),
        ],
    );
    let rail2 = BsplineCurve::new(
        knot,
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(10.0, 0.0, 1.0),
            Point3::new(100.0, 0.0, 2.0),
            Point3::new(1000.0, 0.0, 3.0),
        ],
    );
    (rail1, rail2)
}

/// A figure-8 like profile that, when swept, creates self-intersecting surfaces.
pub fn fixture_self_intersecting_profile() -> BsplineCurve<Point3> {
    // A cubic B-spline that crosses itself, forming a figure-8 in the XY plane.
    let knot_vec = KnotVector::uniform_knot(3, 5);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, -1.0, 0.0),
        Point3::new(3.0, 1.0, 0.0),
        Point3::new(2.0, -1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(-1.0, -1.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// A closed (periodic-like) rail where start and end tangents should match
/// but the curve is only C0 at the join.
pub fn fixture_closed_rail() -> BsplineCurve<Point3> {
    // A cubic B-spline that forms a closed loop but with only C0 continuity
    // at the join point (knot multiplicity = degree at the endpoints).
    let knot_vec = KnotVector::from(vec![
        0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0,
    ]);
    let control_points = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(-1.0, 1.0, 0.0),
        Point3::new(-1.0, -1.0, 0.0),
        Point3::new(1.0, -1.0, 0.0),
        Point3::new(1.0, -0.5, 0.0),
        // End at start to close the loop, but tangent differs.
        Point3::new(1.0, 0.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

// ---------------------------------------------------------------------------
// Glyph-like profiles
// ---------------------------------------------------------------------------

/// Multiple curves forming a letter-like outline with sharp corners,
/// mimicking a sans-serif 'L'. Each segment is a degree-1 (linear) B-spline.
pub fn fixture_glyph_sharp_corners() -> Vec<BsplineCurve<Point3>> {
    let make_line = |a: Point3, b: Point3| -> BsplineCurve<Point3> {
        BsplineCurve::new(KnotVector::bezier_knot(1), vec![a, b])
    };
    // 'L' shape outline (counter-clockwise).
    let p0 = Point3::new(0.0, 0.0, 0.0);
    let p1 = Point3::new(0.0, 2.0, 0.0);
    let p2 = Point3::new(0.3, 2.0, 0.0);
    let p3 = Point3::new(0.3, 0.3, 0.0);
    let p4 = Point3::new(1.5, 0.3, 0.0);
    let p5 = Point3::new(1.5, 0.0, 0.0);
    vec![
        make_line(p0, p1),
        make_line(p1, p2),
        make_line(p2, p3),
        make_line(p3, p4),
        make_line(p4, p5),
        make_line(p5, p0),
    ]
}

/// Nested contour profiles (outer boundary + inner hole),
/// mimicking an 'O' glyph. Each contour is a vector of curves.
pub fn fixture_glyph_nested_contours() -> Vec<Vec<BsplineCurve<Point3>>> {
    let make_rect = |x0: f64, y0: f64, x1: f64, y1: f64| -> Vec<BsplineCurve<Point3>> {
        let make_line = |a: Point3, b: Point3| -> BsplineCurve<Point3> {
            BsplineCurve::new(KnotVector::bezier_knot(1), vec![a, b])
        };
        let p0 = Point3::new(x0, y0, 0.0);
        let p1 = Point3::new(x1, y0, 0.0);
        let p2 = Point3::new(x1, y1, 0.0);
        let p3 = Point3::new(x0, y1, 0.0);
        vec![
            make_line(p0, p1),
            make_line(p1, p2),
            make_line(p2, p3),
            make_line(p3, p0),
        ]
    };
    // Outer contour.
    let outer = make_rect(0.0, 0.0, 2.0, 3.0);
    // Inner hole.
    let inner = make_rect(0.3, 0.3, 1.7, 2.7);
    vec![outer, inner]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degenerate_collapsed_control_points_valid() {
        let curve = degenerate_collapsed_control_points();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 4);
    }

    #[test]
    fn degenerate_near_zero_knot_span_valid() {
        let curve = degenerate_near_zero_knot_span();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 6);
    }

    #[test]
    fn degenerate_high_curvature_pole_valid() {
        let curve = degenerate_high_curvature_pole();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 4);
    }

    #[test]
    fn degenerate_surface_collapsed_edge_valid() {
        let surface = degenerate_surface_collapsed_edge();
        assert_eq!(surface.degrees(), (2, 2));
        assert_eq!(surface.control_points().len(), 3);
        assert_eq!(surface.control_points()[0].len(), 3);
    }

    #[test]
    fn kinked_rail_valid() {
        let curve = fixture_kinked_rail();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 5);
    }

    #[test]
    fn diverging_rails_valid() {
        let (rail1, rail2) = fixture_diverging_rails();
        assert_eq!(rail1.degree(), 3);
        assert_eq!(rail2.degree(), 3);
        assert_eq!(rail1.control_points().len(), 4);
        assert_eq!(rail2.control_points().len(), 4);
    }

    #[test]
    fn self_intersecting_profile_valid() {
        let curve = fixture_self_intersecting_profile();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 8);
    }

    #[test]
    fn closed_rail_valid() {
        let curve = fixture_closed_rail();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 7);
    }

    #[test]
    fn glyph_sharp_corners_valid() {
        let curves = fixture_glyph_sharp_corners();
        assert_eq!(curves.len(), 6);
        curves.iter().for_each(|c| {
            assert_eq!(c.degree(), 1);
            assert_eq!(c.control_points().len(), 2);
        });
    }

    #[test]
    fn glyph_nested_contours_valid() {
        let contours = fixture_glyph_nested_contours();
        assert_eq!(contours.len(), 2);
        assert_eq!(contours[0].len(), 4);
        assert_eq!(contours[1].len(), 4);
        contours.iter().flatten().for_each(|c| {
            assert_eq!(c.degree(), 1);
            assert_eq!(c.control_points().len(), 2);
        });
    }
}
