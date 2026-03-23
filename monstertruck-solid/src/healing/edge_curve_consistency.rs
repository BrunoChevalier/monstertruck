//! Edge-curve consistency validation for compressed shells.
//!
//! Checks that each edge's curve endpoints match its vertex positions
//! within a specified tolerance. This is an opt-in validation step
//! that can be called independently of [`heal_surface_shell`](super::heal_surface_shell).

use monstertruck_geometry::prelude::*;
use monstertruck_topology::compress::*;

/// A single edge-curve deviation report.
#[derive(Debug, Clone)]
pub struct EdgeCurveDeviation {
    /// Index of the edge in the compressed shell.
    pub edge_index: usize,
    /// Distance between the curve start point and the front vertex.
    pub front_deviation: f64,
    /// Distance between the curve end point and the back vertex.
    pub back_deviation: f64,
}

/// Checks edge-curve consistency for all edges in a compressed shell.
///
/// For each edge, evaluates the edge's curve at its parameter bounds
/// and measures the distance to the corresponding vertex positions.
/// Returns deviations that exceed `tol`.
///
/// This is a non-destructive validation step -- it does not modify
/// the shell. Use it to diagnose geometry issues before or after
/// calling [`heal_surface_shell`](super::heal_surface_shell).
///
/// # Examples
///
/// ```ignore
/// use monstertruck_solid::check_edge_curve_consistency;
/// let deviations = check_edge_curve_consistency(&cshell, 1e-6);
/// for dev in &deviations {
///     eprintln!("Edge {}: front={:.2e}, back={:.2e}",
///         dev.edge_index, dev.front_deviation, dev.back_deviation);
/// }
/// ```
pub fn check_edge_curve_consistency<C, S>(
    shell: &CompressedShell<Point3, C, S>,
    tol: f64,
) -> Vec<EdgeCurveDeviation>
where
    C: BoundedCurve<Point = Point3>,
{
    shell
        .edges
        .iter()
        .enumerate()
        .filter_map(|(i, edge)| {
            let (t0, t1) = edge.curve.range_tuple();
            let curve_start = edge.curve.subs(t0);
            let curve_end = edge.curve.subs(t1);

            let front_vertex = shell.vertices[edge.vertices.0];
            let back_vertex = shell.vertices[edge.vertices.1];

            let front_dev = curve_start.distance(front_vertex);
            let back_dev = curve_end.distance(back_vertex);

            if front_dev > tol || back_dev > tol {
                Some(EdgeCurveDeviation {
                    edge_index: i,
                    front_deviation: front_dev,
                    back_deviation: back_dev,
                })
            } else {
                None
            }
        })
        .collect()
}
