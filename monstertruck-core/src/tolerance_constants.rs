//! Centralized tolerance constants for the monstertruck workspace.
//!
//! These constants consolidate magic numbers that were previously scattered
//! across multiple crates. Changing a value here affects all consumers.
//!
//! # Derivation
//!
//! - `SNAP_TOLERANCE`: Used in boolean/intersection operations to snap nearby
//!   points onto boundary curves. Historically `10.0 * TOLERANCE` = `1.0e-5`.
//! - `VERTEX_MERGE_TOLERANCE`: Grid-based vertex merging in transversal loop
//!   construction. Historically `100.0 * TOLERANCE` = `1.0e-4`.
//! - `TESSELLATION_TOLERANCE`: Default geometric tolerance for tessellation of
//!   curves and surfaces. Historically `0.01`.
//! - `PERIODIC_CLOSURE_RATIO`: Ratio threshold for detecting periodic curve
//!   closure. Historically `0.01`.
//! - `G1_ANGLE_TOLERANCE`: Angle in radians for G1 tangent continuity
//!   classification (~1 degree). Historically `0.0175`.
//! - `G2_CURVATURE_TOLERANCE`: Relative tolerance for mean curvature matching
//!   in G2 continuity classification (10%). Historically `0.10`.

use super::tolerance::TOLERANCE;

/// Snap tolerance for boolean/intersection vertex snapping.
///
/// Points closer than this distance are considered coincident during
/// intersection curve construction and loop stitching.
///
/// Default: `10.0 * TOLERANCE` = `1.0e-5`
pub const SNAP_TOLERANCE: f64 = 10.0 * TOLERANCE;

/// Grid-based vertex merge tolerance for transversal loop construction.
///
/// Used to quantize 3D points into an integer grid for O(1) duplicate detection
/// during boolean operation loop assembly.
///
/// Default: `100.0 * TOLERANCE` = `1.0e-4`
pub const VERTEX_MERGE_TOLERANCE: f64 = 100.0 * TOLERANCE;

/// Default geometric tolerance for curve and surface tessellation.
///
/// Controls the maximum deviation between the tessellated polygon/mesh
/// and the underlying exact geometry.
///
/// Default: `0.01`
pub const TESSELLATION_TOLERANCE: f64 = 0.01;

/// Ratio threshold for detecting periodic curve closure.
///
/// A curve is considered closed when the gap between its endpoints
/// is less than `PERIODIC_CLOSURE_RATIO * total_arc_length`.
///
/// Default: `0.01` (1% of arc length)
pub const PERIODIC_CLOSURE_RATIO: f64 = 0.01;

/// UV-space closure threshold for boundary piece loop detection.
///
/// When constructing a [`PolyBoundary`], each [`PolyBoundaryPiece`] is classified
/// as closed or open based on whether its first and last UV-space points are
/// within this distance. A piece is closed when
/// `start.uv.distance(end.uv) < UV_CLOSURE_TOLERANCE`.
///
/// Derived as `TESSELLATION_TOLERANCE / 10.0` to be one order of magnitude
/// tighter than the geometric tessellation tolerance, since UV-space distances
/// are typically smaller than 3D distances.
///
/// Default: `0.001` (= [`TESSELLATION_TOLERANCE`] / 10.0)
pub const UV_CLOSURE_TOLERANCE: f64 = TESSELLATION_TOLERANCE / 10.0;

/// Angle tolerance in radians for G1 tangent continuity classification.
///
/// Two adjacent surfaces are G1-continuous along a shared edge if the
/// angle between their normals is below this threshold at every sample point.
///
/// Default: `0.0175` (~1 degree)
pub const G1_ANGLE_TOLERANCE: f64 = 0.0175;

/// Relative tolerance for mean curvature matching in G2 continuity classification.
///
/// Two adjacent surfaces are G2-continuous along a shared edge if,
/// in addition to G1 continuity, the relative difference in mean curvature
/// `|kappa_1 - kappa_2| / max(|kappa_1|, |kappa_2|)` is below this threshold.
///
/// Default: `0.10` (10%)
pub const G2_CURVATURE_TOLERANCE: f64 = 0.10;
