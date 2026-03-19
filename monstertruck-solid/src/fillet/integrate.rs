use std::collections::HashMap;

use monstertruck_core::tolerance_constants::{G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE};
use monstertruck_geometry::prelude::*;
use monstertruck_traits::ParametricSurface;

use super::types::*;

/// Geometric continuity class for an edge shared between a fillet face
/// and a host face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContinuityAnnotation {
    /// Positional continuity only (G0) -- faces share edge positions.
    G0,
    /// Tangent continuity (G1) -- faces share tangent planes along the edge.
    G1,
    /// Curvature continuity (G2) -- faces share curvature along the edge.
    G2,
}

/// A fillet result carrying continuity annotations on shared edges.
///
/// In IntegrateVisual mode, the fillet face is annotated with continuity
/// constraints at its shared edges with host faces. In KeepSeparateFace
/// mode, annotations is empty.
#[derive(Debug)]
pub struct FilletResult {
    /// The trimmed first host face.
    pub new_face0: Face,
    /// The trimmed second host face.
    pub new_face1: Face,
    /// The fillet face.
    pub fillet_face: Face,
    /// Continuity annotations on shared edges (EdgeId -> annotation).
    /// Non-empty only when mode is IntegrateVisual.
    pub annotations: HashMap<EdgeId, ContinuityAnnotation>,
}

/// Default number of sample points along an edge for continuity classification.
const DEFAULT_SAMPLE_COUNT: usize = 8;

/// Classifies the geometric continuity between a fillet surface and a host
/// surface along a shared edge by sampling normals and curvatures.
///
/// Samples `sample_count` points along the edge and compares surface normals.
/// - If all normal angles are within ~1 degree: G1 (tangent continuous)
/// - If additionally mean curvatures match within 10%: G2 (curvature continuous)
/// - Otherwise: G0 (positional only)
pub(super) fn classify_edge_continuity(
    fillet_surface: &NurbsSurface<Vector4>,
    host_surface: &NurbsSurface<Vector4>,
    edge: &Edge,
    sample_count: usize,
) -> ContinuityAnnotation {
    let curve = edge.curve();
    let (t0, t1) = curve.range_tuple();
    let count = sample_count.max(2);

    let mut all_g1 = true;
    let mut all_g2 = true;

    for i in 0..count {
        let t = t0 + (t1 - t0) * (i as f64) / ((count - 1) as f64);
        let pt = curve.evaluate(t);

        // Find the nearest parameters on each surface.
        let (fu, fv) = match fillet_surface.search_parameter(pt, (0.5, 0.5), 100) {
            Some(uv) => uv,
            None => return ContinuityAnnotation::G0,
        };
        let (hu, hv) = match host_surface.search_parameter(pt, (0.5, 0.5), 100) {
            Some(uv) => uv,
            None => return ContinuityAnnotation::G0,
        };

        let fillet_normal = fillet_surface.normal(fu, fv);
        let host_normal = host_surface.normal(hu, hv);

        let fillet_len = fillet_normal.magnitude();
        let host_len = host_normal.magnitude();
        if !fillet_len.is_finite()
            || fillet_len < 1.0e-12
            || !host_len.is_finite()
            || host_len < 1.0e-12
        {
            all_g1 = false;
            all_g2 = false;
            continue;
        }

        // Normals may point in opposite directions depending on face orientation.
        let cos_angle = (fillet_normal.dot(host_normal) / (fillet_len * host_len))
            .abs()
            .min(1.0);
        let angle = cos_angle.acos();

        if angle > G1_ANGLE_TOLERANCE {
            all_g1 = false;
            all_g2 = false;
        } else {
            // Check curvature for G2 using mean curvature as a scalar proxy.
            let fillet_curv = mean_curvature(fillet_surface, fu, fv);
            let host_curv = mean_curvature(host_surface, hu, hv);

            let max_curv = fillet_curv.abs().max(host_curv.abs());
            if max_curv > 1.0e-12 {
                let diff = (fillet_curv - host_curv).abs();
                if diff / max_curv > G2_CURVATURE_TOLERANCE {
                    all_g2 = false;
                }
            }
            // Both curvatures near zero means both flat -> G2 holds.
        }
    }

    if all_g2 {
        ContinuityAnnotation::G2
    } else if all_g1 {
        ContinuityAnnotation::G1
    } else {
        ContinuityAnnotation::G0
    }
}

/// Approximate mean curvature at a surface point using fundamental forms.
///
/// H = (EN - 2FM + GL) / (2(EG - F^2))
fn mean_curvature(surface: &NurbsSurface<Vector4>, u: f64, v: f64) -> f64 {
    let du = surface.derivative_u(u, v);
    let dv = surface.derivative_v(u, v);
    let n = surface.normal(u, v);

    if n.magnitude() < 1.0e-12 {
        return 0.0;
    }

    let duu = surface.derivative_uu(u, v);
    let dvv = surface.derivative_vv(u, v);
    let duv = surface.derivative_uv(u, v);

    // First fundamental form.
    let e_coeff = du.dot(du);
    let f_coeff = du.dot(dv);
    let g_coeff = dv.dot(dv);

    // Second fundamental form (n is already unit-length from normal()).
    let l_coeff = duu.dot(n);
    let m_coeff = duv.dot(n);
    let n_coeff = dvv.dot(n);

    let denom = e_coeff * g_coeff - f_coeff * f_coeff;
    if denom.abs() < 1.0e-20 {
        return 0.0;
    }

    (e_coeff * n_coeff - 2.0 * f_coeff * m_coeff + g_coeff * l_coeff) / (2.0 * denom)
}

/// Given the three faces returned by a fillet operation, identify the shared
/// edges between the fillet face and the two host faces, then classify each
/// shared edge's continuity.
///
/// The fillet face has 4 boundary edges:
///   [0] shared with new_face0  (fillet_edge0.inverse())
///   [1] side edge
///   [2] shared with new_face1  (fillet_edge1.inverse())
///   [3] side edge
pub(super) fn annotate_fillet_edges(
    new_face0: &Face,
    new_face1: &Face,
    fillet_face: &Face,
) -> HashMap<EdgeId, ContinuityAnnotation> {
    let fillet_boundary = &fillet_face.absolute_boundaries()[0];
    if fillet_boundary.len() < 4 {
        return HashMap::new();
    }

    let fillet_surface = fillet_face.oriented_surface();
    let mut annotations = HashMap::with_capacity(2);

    // Shared edge with new_face0 at boundary index 0.
    let shared_edge0 = &fillet_boundary[0];
    let host_surface0 = new_face0.oriented_surface();
    annotations.insert(
        shared_edge0.id(),
        classify_edge_continuity(
            &fillet_surface,
            &host_surface0,
            shared_edge0,
            DEFAULT_SAMPLE_COUNT,
        ),
    );

    // Shared edge with new_face1 at boundary index 2.
    let shared_edge1 = &fillet_boundary[2];
    let host_surface1 = new_face1.oriented_surface();
    annotations.insert(
        shared_edge1.id(),
        classify_edge_continuity(
            &fillet_surface,
            &host_surface1,
            shared_edge1,
            DEFAULT_SAMPLE_COUNT,
        ),
    );

    annotations
}

/// Ensures that shared edge vertices between fillet and host faces use
/// bitwise-identical 3D positions, preventing tessellation cracks.
///
/// The fillet face shares Edge instances (via Arc) with host faces, so
/// vertex positions at shared edges are already bitwise-identical through
/// the topology. No additional snapping is needed.
pub(super) fn ensure_seamless_vertices(
    _fillet_face: &mut Face,
    _host_face0: &Face,
    _host_face1: &Face,
) {
    // The Arc-based edge sharing guarantees positional consistency.
    // Future enhancement: if surface evaluation drift is detected,
    // add explicit control-point snapping here.

    // Debug-mode contract check: verify shared boundary vertices between fillet
    // and host faces have identical positions, confirming the Arc-sharing
    // invariant prevents tessellation cracks.
    #[cfg(debug_assertions)]
    {
        let fillet_boundary = &_fillet_face.absolute_boundaries()[0];
        if fillet_boundary.len() >= 4 {
            let tol = 1.0e-10;
            // Shared edge with host_face0 at boundary index 0.
            // The fillet edge endpoint positions must appear on the host boundary.
            let shared0 = &fillet_boundary[0];
            let f0_front = shared0.front().point();
            let f0_back = shared0.back().point();
            let host0_verts: Vec<_> = _host_face0.absolute_boundaries()[0]
                .iter()
                .map(|e| e.front().point())
                .collect();
            debug_assert!(
                host0_verts
                    .iter()
                    .any(|v| (*v - f0_front).magnitude() < tol),
                "ensure_seamless_vertices: fillet edge 0 front vertex not found on host face 0"
            );
            debug_assert!(
                host0_verts.iter().any(|v| (*v - f0_back).magnitude() < tol),
                "ensure_seamless_vertices: fillet edge 0 back vertex not found on host face 0"
            );

            // Shared edge with host_face1 at boundary index 2.
            let shared2 = &fillet_boundary[2];
            let f2_front = shared2.front().point();
            let f2_back = shared2.back().point();
            let host1_verts: Vec<_> = _host_face1.absolute_boundaries()[0]
                .iter()
                .map(|e| e.front().point())
                .collect();
            debug_assert!(
                host1_verts
                    .iter()
                    .any(|v| (*v - f2_front).magnitude() < tol),
                "ensure_seamless_vertices: fillet edge 2 front vertex not found on host face 1"
            );
            debug_assert!(
                host1_verts.iter().any(|v| (*v - f2_back).magnitude() < tol),
                "ensure_seamless_vertices: fillet edge 2 back vertex not found on host face 1"
            );
        }
    }
}
