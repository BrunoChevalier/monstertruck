//! Curve-curve intersection via subdivision and Newton-Raphson refinement.
//!
//! The algorithm recursively subdivides both curves, testing [`BoundingBox`]
//! overlap at each level. When both parameter sub-ranges shrink below
//! [`SNAP_TOLERANCE`], Newton-Raphson iteration refines the candidate
//! to full precision.

use super::*;
use monstertruck_core::bounding_box::BoundingBox;
use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;

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
    _curve0: &BsplineCurve<Point3>,
    _curve1: &BsplineCurve<Point3>,
) -> Vec<CurveIntersection> {
    todo!("not yet implemented")
}

/// Finds self-intersection points of a single B-spline curve.
///
/// Subdivides the curve into non-overlapping sub-arcs and tests each
/// pair for intersections, excluding adjacent/identical segments.
pub fn find_self_intersections(
    _curve: &BsplineCurve<Point3>,
) -> Vec<CurveIntersection> {
    todo!("not yet implemented")
}
