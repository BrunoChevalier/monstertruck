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

/// A sub-arc paired with its parameter range.
struct SubArcRange {
    /// Pre-extracted sub-curve for bounding-box tests.
    arc: BsplineCurve<Point3>,
    /// Start of the parameter range on the original curve.
    t_start: f64,
    /// End of the parameter range on the original curve.
    t_end: f64,
}

/// Finds all intersection points between two [`BsplineCurve<Point3>`] values.
///
/// Uses a subdivision approach: recursively splits curves via bounding-box
/// overlap tests, then refines candidate pairs with Newton-Raphson iteration.
/// Returns intersection parameters accurate within [`SNAP_TOLERANCE`].
///
/// # Arguments
/// * `curve0` -- first B-spline curve.
/// * `curve1` -- second B-spline curve.
///
/// # Returns
/// A vector of [`CurveIntersection`] values, one per intersection point,
/// sorted by `t0`.
#[must_use]
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
    // Pass initial full-range sub-arcs to avoid re-extraction at the first level.
    let r0 = SubArcRange {
        arc: extract_subarc(curve0, a0, a1),
        t_start: a0,
        t_end: a1,
    };
    let r1 = SubArcRange {
        arc: extract_subarc(curve1, b0, b1),
        t_start: b0,
        t_end: b1,
    };
    subdivide_and_collect(&mut ctx, &r0, &r1, 0);
    finalize_results(&mut ctx.results);
    ctx.results
}

/// Finds self-intersection points of a single [`BsplineCurve<Point3>`].
///
/// Subdivides the curve into non-overlapping sub-arcs and tests each
/// pair for intersections, excluding adjacent/identical segments.
#[must_use]
pub fn find_self_intersections(curve: &BsplineCurve<Point3>) -> Vec<CurveIntersection> {
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

    let boundaries: Vec<f64> = (0..=n_arcs).map(|i| t_start + dt * i as f64).collect();

    // Build non-adjacent arc pairs, then test each pair for intersections.
    let mut results: Vec<CurveIntersection> = (0..n_arcs)
        .flat_map(|i| ((i + 2)..n_arcs).map(move |j| (i, j)))
        .flat_map(|(i, j)| {
            let sub_arc0 = extract_subarc(curve, boundaries[i], boundaries[i + 1]);
            let sub_arc1 = extract_subarc(curve, boundaries[j], boundaries[j + 1]);
            find_intersections(&sub_arc0, &sub_arc1)
        })
        .collect();

    finalize_results(&mut results);
    results
}

/// Deduplicates and sorts results by `t0`.
fn finalize_results(results: &mut Vec<CurveIntersection>) {
    deduplicate_intersections(results);
    results.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap_or(std::cmp::Ordering::Equal));
}

/// Recursively subdivides curves and collects intersection candidates.
///
/// Accepts pre-extracted [`SubArcRange`] values to avoid redundant cloning
/// and cutting. Each recursion level splits one sub-arc at the midpoint
/// and passes both halves down.
fn subdivide_and_collect(
    ctx: &mut SubdivisionContext<'_>,
    r0: &SubArcRange,
    r1: &SubArcRange,
    depth: usize,
) {
    if depth > MAX_DEPTH {
        return;
    }

    let dt0 = r0.t_end - r0.t_start;
    let dt1 = r1.t_end - r1.t_start;

    // Skip degenerate ranges.
    if dt0 < 0.0 || dt1 < 0.0 {
        return;
    }

    // Test bounding-box overlap of the pre-extracted sub-arcs.
    let bb0: BoundingBox<Point3> = r0.arc.roughly_bounding_box();
    let bb1: BoundingBox<Point3> = r1.arc.roughly_bounding_box();
    if (bb0 ^ bb1).is_empty() {
        return;
    }

    // If both sub-ranges are small enough, try Newton refinement.
    if dt0 < SNAP_TOLERANCE && dt1 < SNAP_TOLERANCE {
        let t0_mid = (r0.t_start + r0.t_end) * 0.5;
        let t1_mid = (r1.t_start + r1.t_end) * 0.5;
        if let Some(hit) = try_refine(ctx.curve0, ctx.curve1, t0_mid, t1_mid) {
            ctx.results.push(hit);
        }
        return;
    }

    // Convergence check: if ranges are small enough for Newton, try it.
    if dt0 < SNAP_TOLERANCE * 10.0 && dt1 < SNAP_TOLERANCE * 10.0 {
        let t0_mid = (r0.t_start + r0.t_end) * 0.5;
        let t1_mid = (r1.t_start + r1.t_end) * 0.5;

        // Skip parallel/overlapping segments (not point intersections).
        if is_parallel_overlap(ctx.curve0, ctx.curve1, t0_mid, t1_mid) {
            return;
        }

        if let Some(hit) = try_refine(ctx.curve0, ctx.curve1, t0_mid, t1_mid) {
            ctx.results.push(hit);
            return;
        }
    }

    // Subdivide the curve with the larger parameter range.
    // Cut once at the midpoint and pass both halves to recursive calls.
    if dt0 >= dt1 {
        let t0_mid = (r0.t_start + r0.t_end) * 0.5;
        let left = SubArcRange {
            arc: extract_subarc(ctx.curve0, r0.t_start, t0_mid),
            t_start: r0.t_start,
            t_end: t0_mid,
        };
        let right = SubArcRange {
            arc: extract_subarc(ctx.curve0, t0_mid, r0.t_end),
            t_start: t0_mid,
            t_end: r0.t_end,
        };
        subdivide_and_collect(ctx, &left, r1, depth + 1);
        subdivide_and_collect(ctx, &right, r1, depth + 1);
    } else {
        let t1_mid = (r1.t_start + r1.t_end) * 0.5;
        let left = SubArcRange {
            arc: extract_subarc(ctx.curve1, r1.t_start, t1_mid),
            t_start: r1.t_start,
            t_end: t1_mid,
        };
        let right = SubArcRange {
            arc: extract_subarc(ctx.curve1, t1_mid, r1.t_end),
            t_start: t1_mid,
            t_end: r1.t_end,
        };
        subdivide_and_collect(ctx, r0, &left, depth + 1);
        subdivide_and_collect(ctx, r0, &right, depth + 1);
    }
}

/// Attempts Newton refinement, falling back to direct distance check.
fn try_refine(
    curve0: &BsplineCurve<Point3>,
    curve1: &BsplineCurve<Point3>,
    t0: f64,
    t1: f64,
) -> Option<CurveIntersection> {
    newton_refine(curve0, curve1, t0, t1).or_else(|| direct_check(curve0, curve1, t0, t1))
}

/// Extracts a sub-arc of a curve between parameters `t_start` and `t_end`.
fn extract_subarc(curve: &BsplineCurve<Point3>, t_start: f64, t_end: f64) -> BsplineCurve<Point3> {
    let (range_start, range_end) = curve.range_tuple();
    let t_start = t_start.max(range_start);
    let t_end = t_end.min(range_end);
    if (t_end - t_start).abs() < f64::EPSILON * 10.0 {
        // Degenerate sub-arc: return a single-point curve.
        let pt = curve.evaluate(t_start);
        return BsplineCurve::new(KnotVector::from(vec![t_start, t_end]), vec![pt]);
    }
    let mut c = curve.clone();
    let _ = c.cut(t_end);
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
    if p0.distance(p1) > SNAP_TOLERANCE * 100.0 {
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

    // Tangents are parallel. Compare second derivatives projected onto the
    // plane perpendicular to the tangent to distinguish overlap from
    // tangential contact.
    let d2_0 = curve0.derivative_2(t0);
    let d2_1 = curve1.derivative_2(t1);
    let tangent_dir = d0 / len0;
    let d2_0_perp = d2_0 - tangent_dir * d2_0.dot(tangent_dir);
    let d2_1_perp = d2_1 - tangent_dir * d2_1.dot(tangent_dir);
    let d2_diff = (d2_0_perp - d2_1_perp).magnitude();
    let d2_scale = d2_0_perp.magnitude().max(d2_1_perp.magnitude());

    // Both straight lines: genuine overlap.
    if d2_scale < SNAP_TOLERANCE {
        return true;
    }

    // Curvatures differ significantly: tangential touch, not overlap.
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

        if diff.magnitude() < SNAP_TOLERANCE {
            return Some(CurveIntersection { t0, t1, point: p0 });
        }

        let d0 = curve0.derivative(t0);
        let d1 = curve1.derivative(t1);

        // Pseudo-inverse of 3x2 Jacobian J = [d0 | -d1].
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

        t0 -= inv_det * (c * rhs0 - b * rhs1);
        t1 -= inv_det * (-b * rhs0 + a * rhs1);
        t0 = t0.clamp(r0_start, r0_end);
        t1 = t1.clamp(r1_start, r1_end);
    }

    // Check convergence after max iterations.
    let p0 = curve0.evaluate(t0);
    let p1 = curve1.evaluate(t1);
    if p0.distance(p1) < SNAP_TOLERANCE {
        Some(CurveIntersection { t0, t1, point: p0 })
    } else {
        None
    }
}

/// Direct distance check for when Newton-Raphson fails (e.g. tangent cases).
///
/// Evaluates both curves at the given parameters and accepts the
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
            point: p0.mid(p1),
        })
    } else {
        None
    }
}

/// Merges intersections where both parameters are within [`SNAP_TOLERANCE`].
fn deduplicate_intersections(results: &mut Vec<CurveIntersection>) {
    results.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap_or(std::cmp::Ordering::Equal));
    results.dedup_by(|a, b| {
        (a.t0 - b.t0).abs() < SNAP_TOLERANCE * 10.0 && (a.t1 - b.t1).abs() < SNAP_TOLERANCE * 10.0
    });
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
        assert!(
            result.is_some(),
            "Newton should converge for crossing lines"
        );
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

    #[test]
    fn test_deduplicate_preserves_distinct() {
        // Three distinct intersections well separated -- all should survive dedup.
        let mut results = vec![
            CurveIntersection {
                t0: 0.1,
                t1: 0.9,
                point: Point3::new(0.0, 0.0, 0.0),
            },
            CurveIntersection {
                t0: 0.5,
                t1: 0.5,
                point: Point3::new(1.0, 1.0, 0.0),
            },
            CurveIntersection {
                t0: 0.9,
                t1: 0.1,
                point: Point3::new(2.0, 0.0, 0.0),
            },
        ];
        deduplicate_intersections(&mut results);
        assert_eq!(
            results.len(),
            3,
            "all distinct intersections must be preserved"
        );
    }

    #[test]
    fn test_deduplicate_multiple_clusters() {
        // Two clusters of near-duplicates should collapse to two results.
        let mut results = vec![
            CurveIntersection {
                t0: 0.1,
                t1: 0.1,
                point: Point3::new(0.0, 0.0, 0.0),
            },
            CurveIntersection {
                t0: 0.1 + 1e-7,
                t1: 0.1 + 1e-7,
                point: Point3::new(0.0, 0.0, 0.0),
            },
            CurveIntersection {
                t0: 0.9,
                t1: 0.9,
                point: Point3::new(1.0, 1.0, 0.0),
            },
            CurveIntersection {
                t0: 0.9 + 1e-7,
                t1: 0.9 + 1e-7,
                point: Point3::new(1.0, 1.0, 0.0),
            },
        ];
        deduplicate_intersections(&mut results);
        assert_eq!(
            results.len(),
            2,
            "each cluster should collapse to one result"
        );
    }

    #[test]
    fn test_crossing_lines_tight_tolerance() {
        // Verify Newton refinement achieves accuracy well within `SNAP_TOLERANCE * 10.0`.
        let c0 = BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 2.0, 0.0)],
        );
        let c1 = BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![Point3::new(2.0, 0.0, 0.0), Point3::new(0.0, 2.0, 0.0)],
        );
        let results = find_intersections(&c0, &c1);
        assert_eq!(results.len(), 1);
        assert!(
            (results[0].t0 - 0.5).abs() < SNAP_TOLERANCE * 10.0,
            "t0 accuracy should be within SNAP_TOLERANCE * 10.0, got diff = {}",
            (results[0].t0 - 0.5).abs()
        );
        assert!(
            (results[0].t1 - 0.5).abs() < SNAP_TOLERANCE * 10.0,
            "t1 accuracy should be within SNAP_TOLERANCE * 10.0, got diff = {}",
            (results[0].t1 - 0.5).abs()
        );
        assert!(
            results[0].point.distance(Point3::new(1.0, 1.0, 0.0)) < SNAP_TOLERANCE * 10.0,
            "point accuracy should be within SNAP_TOLERANCE * 10.0",
        );
    }
}
