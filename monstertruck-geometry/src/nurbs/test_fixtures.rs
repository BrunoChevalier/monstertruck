//! Programmatic fixture generators for problematic NURBS curves and surfaces.
//!
//! This module is **unconditionally compiled** (no `#[cfg(test)]` gate) so that
//! other crates' test code can import it via
//! `use monstertruck_geometry::nurbs::test_fixtures::*`.
//!
//! Each fixture function returns a well-formed [`BsplineCurve`] or
//! [`BsplineSurface`] that exercises a specific numerical edge case.

use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;

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

// ---------------------------------------------------------------------------
// Pathological rail/section combinations (FIXTURE-01)
// ---------------------------------------------------------------------------

/// A cubic rail with an inflection point (S-curve shape).
///
/// Control points create a curve that changes curvature sign, going from
/// concave-up to concave-down. Tests [`BsplineSurface::try_sweep_rail`]
/// framing stability through inflection points.
pub fn fixture_inflection_rail() -> BsplineCurve<Point3> {
    let knot_vec = KnotVector::bezier_knot(3);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, -2.0, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// Two cubic rails that start apart and converge to the same endpoint
/// (within 1e-8).
///
/// Tests [`BsplineSurface::try_birail1`] behavior when the profile must
/// shrink to near-zero width.
pub fn fixture_converging_rails() -> (BsplineCurve<Point3>, BsplineCurve<Point3>) {
    let knot = KnotVector::bezier_knot(3);
    let rail1 = BsplineCurve::new(
        knot.clone(),
        vec![
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(-0.7, 0.0, 1.0),
            Point3::new(-0.3, 0.0, 2.0),
            Point3::new(0.0, 0.0, 3.0),
        ],
    );
    let rail2 = BsplineCurve::new(
        knot,
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.7, 0.0, 1.0),
            Point3::new(0.3, 0.0, 2.0),
            // Converge to (nearly) the same endpoint as rail1.
            Point3::new(1e-9, 0.0, 3.0),
        ],
    );
    (rail1, rail2)
}

/// A cubic section curve where all control points are nearly collinear
/// (within 1e-10 of a line), creating an effectively 1D profile.
///
/// Tests [`BsplineSurface::try_sweep_rail`] with a section that has
/// near-zero cross-sectional area.
pub fn fixture_degenerate_section() -> BsplineCurve<Point3> {
    let knot_vec = KnotVector::bezier_knot(3);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1e-11, 0.0),
        Point3::new(2.0, -1e-11, 0.0),
        Point3::new(3.0, 0.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

/// A cubic rail with a cusp (tangent goes to zero at a point).
///
/// Control points are arranged so the curve has zero tangent magnitude
/// at an interior parameter value.
pub fn fixture_cusped_rail() -> BsplineCurve<Point3> {
    // A cusp occurs when two adjacent control points coincide, causing
    // the derivative to vanish at that parameter.
    let knot_vec = KnotVector::bezier_knot(3);
    let control_points = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        // Duplicate of the previous point creates a cusp.
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    ];
    BsplineCurve::new(knot_vec, control_points)
}

// ---------------------------------------------------------------------------
// Near-degenerate NURBS cases (FIXTURE-02)
// ---------------------------------------------------------------------------

/// A bi-quadratic surface where the surface Jacobian (cross product of partial
/// derivatives) is near-zero along the u=0 boundary.
///
/// The first row of control points are nearly coincident (within 1e-10),
/// creating a pole-like degeneracy.
pub fn fixture_near_zero_jacobian_surface() -> BsplineSurface<Point3> {
    let knot_u = KnotVector::bezier_knot(2);
    let knot_v = KnotVector::bezier_knot(2);
    // First row: all points nearly coincident at the origin.
    let control_points = vec![
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1e-11, 0.0, 0.0),
            Point3::new(2e-11, 0.0, 0.0),
        ],
        vec![
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(0.5, 0.5, 0.5),
            Point3::new(1.0, 0.5, 0.0),
        ],
        vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.5, 1.0, 0.5),
            Point3::new(1.0, 1.0, 0.0),
        ],
    ];
    BsplineSurface::new((knot_u, knot_v), control_points)
}

/// A degree-3 NURBS curve (using [`NurbsCurve<Vector4>`]) where one control
/// point has a weight approaching zero (1e-12), creating a near-singularity
/// in rational evaluation.
pub fn fixture_near_zero_weight_nurbs() -> NurbsCurve<Vector4> {
    let knot_vec = KnotVector::bezier_knot(3);
    // Homogeneous coordinates: (w*x, w*y, w*z, w).
    let control_points = vec![
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        Vector4::new(1.0, 1.0, 0.0, 1.0),
        // Near-zero weight: w = 1e-12, so (w*x, w*y, w*z, w).
        Vector4::new(2.0e-12, 0.0, 0.0, 1e-12),
        Vector4::new(3.0, 0.0, 0.0, 1.0),
    ];
    NurbsCurve::new(BsplineCurve::new(knot_vec, control_points))
}

/// A bi-cubic surface where an entire column of control points collapses
/// to the same location, creating a degenerate edge.
pub fn fixture_collapsed_control_polygon_surface() -> BsplineSurface<Point3> {
    let knot_u = KnotVector::bezier_knot(3);
    let knot_v = KnotVector::bezier_knot(3);
    // Column 0 (v=0) has all control points at (0,0,0).
    let control_points = vec![
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, 3.0),
        ],
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 2.0),
            Point3::new(1.0, 0.0, 3.0),
        ],
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 1.0),
            Point3::new(2.0, 0.0, 2.0),
            Point3::new(2.0, 0.0, 3.0),
        ],
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(3.0, 0.0, 1.0),
            Point3::new(3.0, 0.0, 2.0),
            Point3::new(3.0, 0.0, 3.0),
        ],
    ];
    BsplineSurface::new((knot_u, knot_v), control_points)
}

// ---------------------------------------------------------------------------
// Gordon-specific network fixtures (FIXTURE-03)
// ---------------------------------------------------------------------------

/// A 3x3 network of linear curves forming a planar grid, with grid points
/// perturbed from exact intersections by half of [`SNAP_TOLERANCE`].
///
/// Returns `(u_curves, v_curves, perturbed_grid_points)`. The perturbation
/// tests the snapping behavior of
/// [`BsplineSurface::try_gordon_verified`](crate::nurbs::BsplineSurface::try_gordon_verified).
#[allow(clippy::type_complexity)]
pub fn fixture_gordon_near_miss_grid() -> (
    Vec<BsplineCurve<Point3>>,
    Vec<BsplineCurve<Point3>>,
    Vec<Vec<Point3>>,
) {
    let eps = SNAP_TOLERANCE * 0.5;
    let y_values = [0.0, 0.5, 1.0];
    let x_values = [0.0, 0.5, 1.0];

    // U-curves: horizontal lines at each y-value, from x=0 to x=1.
    let u_curves = y_values
        .iter()
        .map(|&y| {
            BsplineCurve::new(
                KnotVector::bezier_knot(1),
                vec![Point3::new(0.0, y, 0.0), Point3::new(1.0, y, 0.0)],
            )
        })
        .collect();

    // V-curves: vertical lines at each x-value, from y=0 to y=1.
    let v_curves = x_values
        .iter()
        .map(|&x| {
            BsplineCurve::new(
                KnotVector::bezier_knot(1),
                vec![Point3::new(x, 0.0, 0.0), Point3::new(x, 1.0, 0.0)],
            )
        })
        .collect();

    // Grid points offset from exact intersections by (eps, eps, 0).
    let grid_points = y_values
        .iter()
        .map(|&y| {
            x_values
                .iter()
                .map(|&x| Point3::new(x + eps, y + eps, 0.0))
                .collect()
        })
        .collect();

    (u_curves, v_curves, grid_points)
}

/// A 4x4 network with nonuniform spacing in both directions.
///
/// U-curves at y = 0.0, 0.1, 0.7, 1.0 (clustered near y=0).
/// V-curves at x = 0.0, 0.2, 0.8, 1.0 (clustered near both ends).
/// All linear curves on a planar grid. Tests
/// [`BsplineSurface::try_gordon_from_network`](crate::nurbs::BsplineSurface::try_gordon_from_network)
/// with nonuniform curve distributions.
pub fn fixture_gordon_nonuniform_spacing() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>)
{
    let y_values = [0.0, 0.1, 0.7, 1.0];
    let x_values = [0.0, 0.2, 0.8, 1.0];

    let u_curves = y_values
        .iter()
        .map(|&y| {
            BsplineCurve::new(
                KnotVector::bezier_knot(1),
                vec![Point3::new(0.0, y, 0.0), Point3::new(1.0, y, 0.0)],
            )
        })
        .collect();

    let v_curves = x_values
        .iter()
        .map(|&x| {
            BsplineCurve::new(
                KnotVector::bezier_knot(1),
                vec![Point3::new(x, 0.0, 0.0), Point3::new(x, 1.0, 0.0)],
            )
        })
        .collect();

    (u_curves, v_curves)
}

/// A 3x3 network where all curves are degree 4 (quartic).
///
/// U-curves go along X at different Y values with 5 control points each.
/// V-curves go along Y at different X values, also degree 4. Uses
/// [`KnotVector::bezier_knot(4)`] for each curve. All curves are planar
/// (z=0), but the high degree exercises compatibility normalization in
/// the Gordon surface construction.
pub fn fixture_gordon_high_degree_family() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>)
{
    let y_values = [0.0, 0.5, 1.0];
    let x_values = [0.0, 0.5, 1.0];
    let knot = KnotVector::bezier_knot(4);

    // U-curves: degree-4 curves along X for each y-value.
    // 5 control points evenly spaced along X, planar in the XY plane.
    let u_curves = y_values
        .iter()
        .map(|&y| {
            let pts = (0..5)
                .map(|k| {
                    let x = k as f64 / 4.0;
                    Point3::new(x, y, 0.0)
                })
                .collect();
            BsplineCurve::new(knot.clone(), pts)
        })
        .collect();

    // V-curves: degree-4 curves along Y for each x-value.
    let v_curves = x_values
        .iter()
        .map(|&x| {
            let pts = (0..5)
                .map(|k| {
                    let y = k as f64 / 4.0;
                    Point3::new(x, y, 0.0)
                })
                .collect();
            BsplineCurve::new(knot.clone(), pts)
        })
        .collect();

    (u_curves, v_curves)
}

/// A 2x2 network of cubic curves that are genuinely curved (not linear),
/// forming a curved patch.
///
/// U-curves are parabolic arcs at y=0 and y=1 (with control points creating
/// a bulge in Z). V-curves are parabolic arcs at x=0 and x=1. Tests Gordon
/// surface construction with non-trivial curve geometry.
pub fn fixture_gordon_curved_network() -> (Vec<BsplineCurve<Point3>>, Vec<BsplineCurve<Point3>>) {
    let knot = KnotVector::bezier_knot(3);

    // U-curves: cubic arcs along X at y=0 and y=1.
    // Control points create a Z-bulge (parabolic arc shape).
    let u0 = BsplineCurve::new(
        knot.clone(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.33, 0.0, 0.3),
            Point3::new(0.67, 0.0, 0.3),
            Point3::new(1.0, 0.0, 0.0),
        ],
    );
    let u1 = BsplineCurve::new(
        knot.clone(),
        vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.33, 1.0, 0.3),
            Point3::new(0.67, 1.0, 0.3),
            Point3::new(1.0, 1.0, 0.0),
        ],
    );

    // V-curves: cubic arcs along Y at x=0 and x=1.
    let v0 = BsplineCurve::new(
        knot.clone(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.33, 0.2),
            Point3::new(0.0, 0.67, 0.2),
            Point3::new(0.0, 1.0, 0.0),
        ],
    );
    let v1 = BsplineCurve::new(
        knot,
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.33, 0.2),
            Point3::new(1.0, 0.67, 0.2),
            Point3::new(1.0, 1.0, 0.0),
        ],
    );

    (vec![u0, u1], vec![v0, v1])
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

    // FIXTURE-01: Pathological rail/section combinations

    #[test]
    fn inflection_rail_valid() {
        let curve = fixture_inflection_rail();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 4);
        // Verify S-curve shape: middle control points have opposite y signs.
        assert!(curve.control_points()[1].y > 0.0);
        assert!(curve.control_points()[2].y < 0.0);
    }

    #[test]
    fn converging_rails_valid() {
        let (rail1, rail2) = fixture_converging_rails();
        assert_eq!(rail1.degree(), 3);
        assert_eq!(rail2.degree(), 3);
        assert_eq!(rail1.control_points().len(), 4);
        assert_eq!(rail2.control_points().len(), 4);
        // Verify convergence: endpoints are within 1e-8.
        let end1 = rail1.control_points().last().unwrap();
        let end2 = rail2.control_points().last().unwrap();
        let dist =
            ((end1.x - end2.x).powi(2) + (end1.y - end2.y).powi(2) + (end1.z - end2.z).powi(2))
                .sqrt();
        assert!(dist < 1e-8, "endpoints should converge, dist = {dist}");
    }

    #[test]
    fn degenerate_section_valid() {
        let curve = fixture_degenerate_section();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 4);
        // Verify near-collinearity: all y-coordinates within 1e-10 of zero.
        curve
            .control_points()
            .iter()
            .for_each(|p| assert!(p.y.abs() < 1e-10));
    }

    #[test]
    fn cusped_rail_valid() {
        let curve = fixture_cusped_rail();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 4);
        // Verify cusp: two adjacent control points coincide.
        let p1 = &curve.control_points()[1];
        let p2 = &curve.control_points()[2];
        let dist = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt();
        assert!(
            dist < 1e-14,
            "adjacent points should coincide, dist = {dist}"
        );
    }

    // FIXTURE-02: Near-degenerate NURBS cases

    #[test]
    fn near_zero_jacobian_surface_valid() {
        let surface = fixture_near_zero_jacobian_surface();
        assert_eq!(surface.degrees(), (2, 2));
        assert_eq!(surface.control_points().len(), 3);
        assert_eq!(surface.control_points()[0].len(), 3);
        // Verify first row is nearly coincident.
        let row0 = &surface.control_points()[0];
        row0.windows(2).for_each(|w| {
            let dist =
                ((w[0].x - w[1].x).powi(2) + (w[0].y - w[1].y).powi(2) + (w[0].z - w[1].z).powi(2))
                    .sqrt();
            assert!(
                dist < 1e-10,
                "first-row points should be nearly coincident, dist = {dist}"
            );
        });
    }

    #[test]
    fn near_zero_weight_nurbs_valid() {
        let nurbs = fixture_near_zero_weight_nurbs();
        let bspline = nurbs.non_rationalized();
        assert_eq!(bspline.degree(), 3);
        assert_eq!(bspline.control_points().len(), 4);
        // Verify near-zero weight on third control point (index 3 = weight).
        let w2 = bspline.control_points()[2][3];
        assert!(w2.abs() < 1e-11, "weight should be near-zero, w = {w2}");
    }

    #[test]
    fn collapsed_control_polygon_surface_valid() {
        let surface = fixture_collapsed_control_polygon_surface();
        assert_eq!(surface.degrees(), (3, 3));
        assert_eq!(surface.control_points().len(), 4);
        assert_eq!(surface.control_points()[0].len(), 4);
        // Verify column 0 (v=0) has all points at (0,0,0).
        surface.control_points().iter().for_each(|row| {
            let p = &row[0];
            let dist = (p.x.powi(2) + p.y.powi(2) + p.z.powi(2)).sqrt();
            assert!(
                dist < 1e-14,
                "column-0 point should be at origin, dist = {dist}"
            );
        });
    }

    // FIXTURE-03: Gordon-specific network fixtures

    #[test]
    fn gordon_near_miss_grid_valid() {
        let (u_curves, v_curves, grid_points) = fixture_gordon_near_miss_grid();
        assert_eq!(u_curves.len(), 3);
        assert_eq!(v_curves.len(), 3);
        assert_eq!(grid_points.len(), 3);
        grid_points.iter().for_each(|row| assert_eq!(row.len(), 3));
        // All curves should be linear (degree 1).
        u_curves
            .iter()
            .chain(v_curves.iter())
            .for_each(|c| assert_eq!(c.degree(), 1));
    }

    #[test]
    fn gordon_near_miss_grid_perturbation_within_snap_tolerance() {
        use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
        let (_u_curves, _v_curves, grid_points) = fixture_gordon_near_miss_grid();
        // Each grid point should be offset from the exact intersection by less than SNAP_TOLERANCE.
        let y_values = [0.0, 0.5, 1.0];
        let x_values = [0.0, 0.5, 1.0];
        (0..3).for_each(|i| {
            (0..3).for_each(|j| {
                let exact = Point3::new(x_values[j], y_values[i], 0.0);
                let perturbed = &grid_points[i][j];
                let dist = ((perturbed.x - exact.x).powi(2)
                    + (perturbed.y - exact.y).powi(2)
                    + (perturbed.z - exact.z).powi(2))
                .sqrt();
                assert!(
                    dist < SNAP_TOLERANCE,
                    "grid point [{i}][{j}] offset {dist} exceeds SNAP_TOLERANCE"
                );
                assert!(dist > 0.0, "grid point [{i}][{j}] should be perturbed");
            });
        });
    }

    #[test]
    fn gordon_nonuniform_spacing_valid() {
        let (u_curves, v_curves) = fixture_gordon_nonuniform_spacing();
        assert_eq!(u_curves.len(), 4);
        assert_eq!(v_curves.len(), 4);
        // All curves should be linear.
        u_curves
            .iter()
            .chain(v_curves.iter())
            .for_each(|c| assert_eq!(c.degree(), 1));
    }

    #[test]
    fn gordon_high_degree_family_valid() {
        let (u_curves, v_curves) = fixture_gordon_high_degree_family();
        assert_eq!(u_curves.len(), 3);
        assert_eq!(v_curves.len(), 3);
        // All curves should be degree 4 (quartic).
        u_curves.iter().chain(v_curves.iter()).for_each(|c| {
            assert_eq!(c.degree(), 4, "expected degree 4, got {}", c.degree());
            assert_eq!(c.control_points().len(), 5);
        });
    }

    #[test]
    fn gordon_curved_network_valid() {
        let (u_curves, v_curves) = fixture_gordon_curved_network();
        assert_eq!(u_curves.len(), 2);
        assert_eq!(v_curves.len(), 2);
        // All curves should be cubic.
        u_curves.iter().chain(v_curves.iter()).for_each(|c| {
            assert_eq!(c.degree(), 3);
            assert_eq!(c.control_points().len(), 4);
        });
    }
}
