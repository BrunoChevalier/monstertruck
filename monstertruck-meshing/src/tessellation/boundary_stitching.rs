//! Boundary-aware vertex stitching for tessellated shells.
//!
//! After per-face tessellation, adjacent faces may have near-identical but
//! not bitwise-equal positions along shared topological edges. This module
//! unifies those positions so that downstream merging via
//! [`put_together_same_attrs`](crate::filters::OptimizingFilter::put_together_same_attrs)
//! works reliably.
//!
//! The core algorithm:
//! 1. Build a map from each topological edge ID to its canonical polyline
//!    3D points.
//! 2. For each face in the shell, collect the canonical boundary points
//!    from its edges.
//! 3. Replace polygon mesh positions that are within tolerance of a
//!    canonical boundary point with the exact canonical value.
//!
//! This ensures that adjacent faces sharing a topological edge use
//! bitwise-identical position values along that edge.

use super::*;
use rustc_hash::FxHashMap as HashMap;

/// Stitches boundary vertices of a tessellated [`Shell`] so that adjacent
/// faces share exactly identical positions along shared topological edges.
///
/// After per-face tessellation, each face independently evaluates surface
/// positions for its boundary vertices. Even though the underlying polyline
/// edge is shared, the resulting polygon mesh positions can differ by
/// floating-point rounding. This function collects canonical positions from
/// the shared polyline edges and overwrites each face's polygon mesh
/// positions along those edges to be exactly equal.
///
/// The function modifies the shell in place. Vertex counts per face are
/// preserved; only position values are updated.
pub fn stitch_boundaries(
    shell: &mut Shell<Point3, PolylineCurve, Option<PolygonMesh>>,
    tolerance: f64,
) {
    // Build a map from edge ID to the canonical polyline 3D points.
    let canonical_positions: HashMap<_, Vec<Point3>> = shell
        .edge_iter()
        .map(|edge| {
            let points: Vec<Point3> = edge.curve().0.clone();
            (edge.id(), points)
        })
        .collect();

    // For each face, find boundary vertices in its polygon mesh that lie on
    // a shared edge and replace them with the canonical positions.
    shell.face_iter().for_each(|face| {
        let Some(mut poly) = face.surface() else {
            return;
        };

        // Collect canonical boundary points from all edges of this face.
        let boundary_points: Vec<Point3> =
            collect_face_boundary_points(face.absolute_boundaries(), &canonical_positions);

        // Replace polygon mesh positions that are within tolerance of a
        // canonical boundary point with the exact canonical value.
        replace_near_positions(poly.positions_mut(), &boundary_points, tolerance);

        face.set_surface(Some(poly));
    });
}

/// Stitches boundary vertices of a tessellated [`CompressedShell`] so that
/// adjacent faces share exactly identical positions along shared edges.
///
/// This is the [`CompressedShell`] analogue of [`stitch_boundaries`]. It
/// operates on the indexed edge/face representation, matching polygon mesh
/// positions in each face against the canonical polyline positions stored
/// in the compressed edge array.
pub fn stitch_compressed_boundaries(
    shell: &mut CompressedShell<Point3, PolylineCurve, Option<PolygonMesh>>,
    tolerance: f64,
) {
    // Collect canonical positions from all compressed edges.
    let edge_positions: Vec<Vec<Point3>> = shell
        .edges
        .iter()
        .map(|edge| edge.curve.0.clone())
        .collect();

    // For each face, resolve its boundary edges and collect canonical points.
    shell.faces.iter_mut().for_each(|face| {
        let Some(ref mut poly) = face.surface else {
            return;
        };

        // Collect canonical boundary points from the face's edge indices.
        let boundary_points: Vec<Point3> = face
            .boundaries
            .iter()
            .flat_map(|wire| {
                wire.iter().flat_map(|edge_idx| {
                    let Some(edge) = edge_positions.get(edge_idx.index) else {
                        return Vec::new();
                    };
                    // Use oriented points: forward or reversed.
                    let points: Vec<Point3> = match edge_idx.orientation {
                        true => edge.clone(),
                        false => edge.iter().rev().copied().collect(),
                    };
                    // Drop the last point of each polyline segment.
                    let n = points.len().saturating_sub(1);
                    points.into_iter().take(n).collect::<Vec<_>>()
                })
            })
            .collect();

        replace_near_positions(poly.positions_mut(), &boundary_points, tolerance);
    });
}

/// Collects canonical boundary points for a face from its wire boundaries.
///
/// For each edge in the wires, looks up the canonical polyline positions and
/// orients them according to the edge direction. Drops the last point of
/// each polyline segment to avoid duplication at segment junctions.
fn collect_face_boundary_points<C>(
    boundaries: &[Wire<Point3, C>],
    canonical_positions: &HashMap<EdgeId<C>, Vec<Point3>>,
) -> Vec<Point3>
where
    C: Clone,
{
    boundaries
        .iter()
        .flat_map(|wire| {
            wire.edge_iter().flat_map(|edge| {
                let id = edge.id();
                // SAFETY: every edge in the shell was inserted into canonical_positions.
                let canonical = canonical_positions
                    .get(&id)
                    .expect("edge must be in canonical map");
                // Use oriented points: if the edge is inverted, reverse order.
                let points: Vec<Point3> = match edge.orientation() {
                    true => canonical.clone(),
                    false => canonical.iter().rev().copied().collect(),
                };
                // Drop the last point of each polyline segment.
                let n = points.len().saturating_sub(1);
                points.into_iter().take(n)
            })
        })
        .collect()
}

/// Replaces positions in `positions` that are within `tolerance` of any
/// point in `canonical` with the exact canonical value.
///
/// Uses squared-distance comparison for efficiency.
fn replace_near_positions(positions: &mut [Point3], canonical: &[Point3], tolerance: f64) {
    let tol_sq = tolerance * tolerance;
    positions.iter_mut().for_each(|pos| {
        if let Some(bp) = canonical
            .iter()
            .find(|bp| (*pos - **bp).magnitude2() < tol_sq)
        {
            *pos = *bp;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `replace_near_positions` updates positions within tolerance.
    #[test]
    fn replace_near_positions_basic() {
        let mut positions = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.5, 0.5, 0.0),
        ];
        let canonical = vec![Point3::new(1e-15, 0.0, 0.0)];
        replace_near_positions(&mut positions, &canonical, TOLERANCE);
        // The first position should be replaced with the canonical value.
        assert_eq!(positions[0], Point3::new(1e-15, 0.0, 0.0));
        // The second position is too far from canonical; unchanged.
        assert_eq!(positions[1], Point3::new(1.0, 0.0, 0.0));
    }

    /// Verifies that positions far from any canonical point are not modified.
    #[test]
    fn replace_near_positions_no_match() {
        let mut positions = vec![Point3::new(5.0, 5.0, 5.0)];
        let canonical = vec![Point3::new(0.0, 0.0, 0.0)];
        replace_near_positions(&mut positions, &canonical, TOLERANCE);
        assert_eq!(positions[0], Point3::new(5.0, 5.0, 5.0));
    }
}
