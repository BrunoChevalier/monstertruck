//! Curve-curve intersection via subdivision and Newton-Raphson refinement.
//!
//! The algorithm recursively subdivides both curves, testing [`BoundingBox`]
//! overlap at each level. When both parameter sub-ranges shrink below
//! [`SNAP_TOLERANCE`], Newton-Raphson iteration refines the candidate
//! to full precision.

use super::*;
use monstertruck_core::bounding_box::BoundingBox;
use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;

/// Maximum recursion depth for subdivision.
const MAX_DEPTH: usize = 50;

/// Maximum Newton-Raphson iterations.
const MAX_NEWTON_ITERS: usize = 20;

/// Threshold for detecting near-parallel tangents (cosine of angle).
const PARALLEL_DOT_THRESHOLD: f64 = 0.999;

/// A single intersection result between two curves.
#[derive(Clone, Debug, PartialEq)]
pub struct CurveIntersection {
    /// Parameter on the first curve.
    pub t0: f64,
    /// Parameter on the second curve.
    pub t1: f64,
    /// The intersection point in space.
    pub point: Point3,
}

/// Bundled state for recursive subdivision to keep argument counts low.
struct SubdivisionContext<'a> {
    /// First curve.
    curve0: &'a BsplineCurve<Point3>,
    /// Second curve.
    curve1: &'a BsplineCurve<Point3>,
    /// Accumulated intersection results.
    results: Vec<CurveIntersection>,
}

/// Finds all intersection points between two B-spline curves.
///
/// Uses a subdivision approach: recursively splits curves via bounding-box
/// overlap tests, then refines candidate pairs with Newton-Raphson iteration.
/// Returns intersection parameters accurate within [`SNAP_TOLERANCE`].
///
/// # Arguments
/// * `curve0` - First B-spline curve.
/// * `curve1` - Second B-spline curve.
///
/// # Returns
/// A vector of [`CurveIntersection`] values, one per intersection point,
/// sorted by `t0`.
pub fn find_intersections(
    curve0: &BsplineCurve<Point3>,
    curve1: &BsplineCurve<Point3>,
) -> Vec<CurveIntersection> {
    let (a0, a1) = curve0.range_tuple();
    let (b0, b1) = curve1.range_tuple();
    let mut ctx = SubdivisionContext {
        curve0,
        curve1,
        results: Vec::new(),
    };
    subdivide_and_collect(&mut ctx, a0, a1, b0, b1, 0);
    deduplicate_intersections(&mut ctx.results);
    ctx.results.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap_or(std::cmp::Ordering::Equal));
    ctx.results
}

/// Finds self-intersection points of a single B-spline curve.
///
/// Subdivides the curve into non-overlapping sub-arcs and tests each
/// pair for intersections, excluding adjacent/identical segments.
pub fn find_self_intersections(
    curve: &BsplineCurve<Point3>,
) -> Vec<CurveIntersection> {
    let (t_start, t_end) = curve.range_tuple();
    let range = t_end - t_start;
    if range < SNAP_TOLERANCE {
        return Vec::new();
    }

    // Determine number of sub-arcs: use non-trivial knot spans, minimum 4.
    let knots = curve.knot_vec().as_slice();
    let degree = curve.degree();
    let n_interior = knots[degree..=(knots.len() - 1 - degree)]
        .windows(2)
        .filter(|w| (w[1] - w[0]).abs() > SNAP_TOLERANCE)
        .count();
    let n_arcs = n_interior.max(4);
    let dt = range / n_arcs as f64;

    // Build sub-arc boundaries.
    let boundaries: Vec<f64> = (0..=n_arcs)
        .map(|i| t_start + dt * i as f64)
        .collect();

    let mut results = Vec::new();

    // Test all non-adjacent pairs of sub-arcs.
    (0..n_arcs).for_each(|i| {
        ((i + 2)..n_arcs).for_each(|j| {
            let sub_arc0 = extract_subarc(curve, boundaries[i], boundaries[i + 1]);
            let sub_arc1 = extract_subarc(curve, boundaries[j], boundaries[j + 1]);
            results.extend(find_intersections(&sub_arc0, &sub_arc1));
        });
    });

    deduplicate_intersections(&mut results);
    results.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap_or(std::cmp::Ordering::Equal));
    results
}

/// Recursively subdivides curves and collects intersection candidates.
fn subdivide_and_collect(
    ctx: &mut SubdivisionContext<'_>,
    t0_start: f64,
    t0_end: f64,
    t1_start: f64,
    t1_end: f64,
    depth: usize,
) {
    if depth > MAX_DEPTH {
        return;
    }

    let dt0 = t0_end - t0_start;
    let dt1 = t1_end - t1_start;

    // Skip degenerate ranges.
    if dt0 < 0.0 || dt1 < 0.0 {
        return;
    }

    // Compute bounding boxes over the sub-ranges by cutting sub-arcs.
    let sub0 = extract_subarc(ctx.curve0, t0_start, t0_end);
    let sub1 = extract_subarc(ctx.curve1, t1_start, t1_end);

    let bb0: BoundingBox<Point3> = sub0.roughly_bounding_box();
    let bb1: BoundingBox<Point3> = sub1.roughly_bounding_box();

    // Check bounding box overlap.
    let intersection = bb0 ^ bb1;
    if intersection.is_empty() {
        return;
    }

    // If both sub-ranges are small enough, try Newton refinement.
    if dt0 < SNAP_TOLERANCE && dt1 < SNAP_TOLERANCE {
        let t0_mid = (t0_start + t0_end) * 0.5;
        let t1_mid = (t1_start + t1_end) * 0.5;
        if let Some(hit) = newton_refine(ctx.curve0, ctx.curve1, t0_mid, t1_mid)
            .or_else(|| direct_check(ctx.curve0, ctx.curve1, t0_mid, t1_mid))
        {
            ctx.results.push(hit);
        }
        return;
    }

    // Convergence check: if ranges are small enough for Newton, try it.
    if dt0 < SNAP_TOLERANCE * 10.0 && dt1 < SNAP_TOLERANCE * 10.0 {
        let t0_mid = (t0_start + t0_end) * 0.5;
        let t1_mid = (t1_start + t1_end) * 0.5;

        // Check for parallel/overlapping case.
        if is_parallel_overlap(ctx.curve0, ctx.curve1, t0_mid, t1_mid) {
            return;
        }

        if let Some(hit) = newton_refine(ctx.curve0, ctx.curve1, t0_mid, t1_mid)
            .or_else(|| direct_check(ctx.curve0, ctx.curve1, t0_mid, t1_mid))
        {
            ctx.results.push(hit);
            return;
        }
    }

    // Subdivide the curve with the larger parameter range.
    if dt0 >= dt1 {
        let t0_mid = (t0_start + t0_end) * 0.5;
        subdivide_and_collect(ctx, t0_start, t0_mid, t1_start, t1_end, depth + 1);
        subdivide_and_collect(ctx, t0_mid, t0_end, t1_start, t1_end, depth + 1);
    } else {
        let t1_mid = (t1_start + t1_end) * 0.5;
        subdivide_and_collect(ctx, t0_start, t0_end, t1_start, t1_mid, depth + 1);
        subdivide_and_collect(ctx, t0_start, t0_end, t1_mid, t1_end, depth + 1);
    }
}

/// Extracts a sub-arc of a curve between parameter values `t_start` and `t_end`.
fn extract_subarc(
    curve: &BsplineCurve<Point3>,
    t_start: f64,
    t_end: f64,
) -> BsplineCurve<Point3> {
    let (range_start, range_end) = curve.range_tuple();
    let t_start = t_start.max(range_start);
    let t_end = t_end.min(range_end);
    if (t_end - t_start).abs() < f64::EPSILON * 10.0 {
        // Degenerate sub-arc: return a single-point curve.
        let pt = curve.evaluate(t_start);
        return BsplineCurve::new(
            KnotVector::from(vec![t_start, t_end]),
            vec![pt],
        );
    }
    let mut c = curve.clone();
    // cut at t_end first, keeping [range_start, t_end].
    let _ = c.cut(t_end);
    // Then cut at t_start, getting [t_start, t_end] as the returned part.
    c.cut(t_start)
}

/// Checks whether two curves are nearly parallel and overlapping at given parameters.
///
/// Distinguishes true overlap (curves coincide over a range) from tangential
/// touch (curves meet at a point with parallel tangents but diverge).
/// Returns `true` only for overlap, allowing tangent intersections through.
fn is_parallel_overlap(
    curve0: &BsplineCurve<Point3>,
    curve1: &BsplineCurve<Point3>,
    t0: f64,
    t1: f64,
) -> bool {
    let p0 = curve0.evaluate(t0);
    let p1 = curve1.evaluate(t1);
    let dist = p0.distance(p1);
    if dist > SNAP_TOLERANCE * 100.0 {
        return false;
    }

    let d0 = curve0.derivative(t0);
    let d1 = curve1.derivative(t1);
    let len0 = d0.magnitude();
    let len1 = d1.magnitude();
    if len0 < SNAP_TOLERANCE || len1 < SNAP_TOLERANCE {
        return false;
    }

    let dot = d0.dot(d1) / (len0 * len1);
    if dot.abs() <= PARALLEL_DOT_THRESHOLD {
        return false;
    }

    // Tangents are parallel. Check second derivatives to distinguish
    // true overlap from tangential contact.
    // For overlap, the second derivatives projected onto the normal plane
    // should match. For tangential contact, they differ.
    let d2_0 = curve0.derivative_2(t0);
    let d2_1 = curve1.derivative_2(t1);

    // Compute the component of d2 perpendicular to the tangent direction.
    let tangent_dir = d0 / len0;
    let d2_0_perp = d2_0 - tangent_dir * d2_0.dot(tangent_dir);
    let d2_1_perp = d2_1 - tangent_dir * d2_1.dot(tangent_dir);

    // For true overlap, perpendicular curvatures match.
    let d2_diff = (d2_0_perp - d2_1_perp).magnitude();
    let d2_scale = d2_0_perp.magnitude().max(d2_1_perp.magnitude());

    // If both perpendicular curvatures are near zero (both straight lines),
    // this is a genuine overlap.
    if d2_scale < SNAP_TOLERANCE {
        return true;
    }

    // If the curvatures differ significantly, it is a tangential touch.
    d2_diff / d2_scale < 0.1
}

/// Newton-Raphson refinement to find precise intersection parameters.
///
/// Solves `C0(t0) - C1(t1) = 0` using the Jacobian `[C0'(t0), -C1'(t1)]`.
/// For 3D curves with 2 unknowns, uses the pseudo-inverse `(J^T J)^{-1} J^T`.
fn newton_refine(
    curve0: &BsplineCurve<Point3>,
    curve1: &BsplineCurve<Point3>,
    mut t0: f64,
    mut t1: f64,
) -> Option<CurveIntersection> {
    let (r0_start, r0_end) = curve0.range_tuple();
    let (r1_start, r1_end) = curve1.range_tuple();

    for _ in 0..MAX_NEWTON_ITERS {
        let p0 = curve0.evaluate(t0);
        let p1 = curve1.evaluate(t1);
        let diff = p0 - p1;
        let dist = diff.magnitude();

        if dist < SNAP_TOLERANCE {
            return Some(CurveIntersection {
                t0,
                t1,
                point: p0,
            });
        }

        let d0 = curve0.derivative(t0);
        let d1 = curve1.derivative(t1);

        // J = [d0 | -d1] is a 3x2 matrix.
        // J^T * diff = [d0 . diff, -d1 . diff].
        // J^T * J = [[d0.d0, -d0.d1], [-d0.d1, d1.d1]].
        let a = d0.dot(d0);
        let b = -d0.dot(d1);
        let c = d1.dot(d1);
        let det = a * c - b * b;

        // Guard against singular Jacobian.
        if det.abs() < SNAP_TOLERANCE * SNAP_TOLERANCE {
            return None;
        }

        let rhs0 = d0.dot(diff);
        let rhs1 = -d1.dot(diff);

        let inv_det = 1.0 / det;
        let dt0 = inv_det * (c * rhs0 - b * rhs1);
        let dt1 = inv_det * (-b * rhs0 + a * rhs1);

        t0 -= dt0;
        t1 -= dt1;

        // Clamp to valid ranges.
        t0 = t0.clamp(r0_start, r0_end);
        t1 = t1.clamp(r1_start, r1_end);
    }

    // Check convergence after max iterations.
    let p0 = curve0.evaluate(t0);
    let p1 = curve1.evaluate(t1);
    if p0.distance(p1) < SNAP_TOLERANCE {
        Some(CurveIntersection {
            t0,
            t1,
            point: p0,
        })
    } else {
        None
    }
}

/// Direct distance check for when Newton-Raphson fails (e.g. tangent cases).
///
/// Simply evaluates both curves at the given parameters and accepts the
/// intersection if the points are close enough.
fn direct_check(
    curve0: &BsplineCurve<Point3>,
    curve1: &BsplineCurve<Point3>,
    t0: f64,
    t1: f64,
) -> Option<CurveIntersection> {
    let p0 = curve0.evaluate(t0);
    let p1 = curve1.evaluate(t1);
    if p0.distance(p1) < SNAP_TOLERANCE {
        Some(CurveIntersection {
            t0,
            t1,
            point: Point3::new(
                (p0[0] + p1[0]) * 0.5,
                (p0[1] + p1[1]) * 0.5,
                (p0[2] + p1[2]) * 0.5,
            ),
        })
    } else {
        None
    }
}

/// Merges intersections where both parameters are within [`SNAP_TOLERANCE`].
fn deduplicate_intersections(results: &mut Vec<CurveIntersection>) {
    if results.len() < 2 {
        return;
    }
    results.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap_or(std::cmp::Ordering::Equal));
    let mut i = 0;
    while i < results.len() {
        let mut j = i + 1;
        while j < results.len() {
            if (results[i].t0 - results[j].t0).abs() < SNAP_TOLERANCE * 10.0
                && (results[i].t1 - results[j].t1).abs() < SNAP_TOLERANCE * 10.0
            {
                results.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newton_refine_converges() {
        // Two crossing lines: should converge.
        let c0 = BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 2.0, 0.0)],
        );
        let c1 = BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![Point3::new(2.0, 0.0, 0.0), Point3::new(0.0, 2.0, 0.0)],
        );
        let result = newton_refine(&c0, &c1, 0.4, 0.6);
        assert!(result.is_some(), "Newton should converge for crossing lines");
        // SAFETY: We just checked `is_some` above.
        let hit = result.unwrap();
        assert!((hit.t0 - 0.5).abs() < SNAP_TOLERANCE);
        assert!((hit.t1 - 0.5).abs() < SNAP_TOLERANCE);
    }

    #[test]
    fn test_deduplicate() {
        let mut results = vec![
            CurveIntersection {
                t0: 0.5,
                t1: 0.5,
                point: Point3::new(1.0, 1.0, 0.0),
            },
            CurveIntersection {
                t0: 0.5 + 1e-7,
                t1: 0.5 + 1e-7,
                point: Point3::new(1.0, 1.0, 0.0),
            },
            CurveIntersection {
                t0: 0.8,
                t1: 0.2,
                point: Point3::new(1.5, 0.5, 0.0),
            },
        ];
        deduplicate_intersections(&mut results);
        assert_eq!(results.len(), 2);
    }
}
