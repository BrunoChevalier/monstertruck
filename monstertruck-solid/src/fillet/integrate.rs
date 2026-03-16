use std::collections::HashMap;

use monstertruck_geometry::prelude::*;

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

/// Classifies the geometric continuity between a fillet surface and a host
/// surface along a shared edge by sampling normals and curvatures.
pub(super) fn classify_edge_continuity(
    _fillet_surface: &NurbsSurface<Vector4>,
    _host_surface: &NurbsSurface<Vector4>,
    _edge: &Edge,
    _sample_count: usize,
) -> ContinuityAnnotation {
    // Stub: always returns G0 (will be implemented in GREEN phase)
    ContinuityAnnotation::G0
}

/// Given the three faces returned by a fillet operation, identify shared edges
/// and classify each edge's continuity.
pub(super) fn annotate_fillet_edges(
    _new_face0: &Face,
    _new_face1: &Face,
    _fillet_face: &Face,
) -> HashMap<EdgeId, ContinuityAnnotation> {
    // Stub: returns empty map (will be implemented in GREEN phase)
    HashMap::new()
}

/// Ensures that shared edge vertices between fillet and host faces use
/// bitwise-identical 3D positions, preventing tessellation cracks.
pub(super) fn ensure_seamless_vertices(
    _fillet_face: &mut Face,
    _host_face0: &Face,
    _host_face1: &Face,
) {
    // Stub: no-op (will be implemented in GREEN phase)
}
