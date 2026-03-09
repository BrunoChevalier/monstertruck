//! Surface offset operation for shells.

use monstertruck_core::cgmath64::*;
use monstertruck_geometry::prelude::*;
use monstertruck_topology::compress::*;
use monstertruck_topology::*;

/// Trait for surfaces that can be offset along their normals.
pub trait OffsetSurface: Sized {
    /// Offsets the surface by `distance` along its normal direction.
    /// `n_samples` controls approximation quality for non-planar surfaces.
    fn offset(&self, distance: f64, n_samples: usize) -> Option<Self>;

    /// Computes a point on the surface closest to `pt` and returns the normal there.
    /// Returns `None` if the surface is degenerate.
    fn normal_at_closest(&self, pt: Point3) -> Option<(Vector3, (f64, f64))>;

    /// Evaluates the surface at parameters `(u, v)`.
    fn evaluate_at(&self, u: f64, v: f64) -> Point3;
}

/// Trait for curves that can be re-created for offset edges.
pub trait OffsetCurve: Sized {
    /// Creates a new curve connecting two offset endpoints.
    /// `original` is the original curve for reference geometry.
    fn offset_curve(original: &Self, new_front: Point3, new_back: Point3) -> Self;
}

/// Offsets all surfaces of a shell by `distance` along their normals.
///
/// Positive distance offsets outward (along surface normal direction).
/// Negative distance offsets inward.
///
/// `n_samples` controls the approximation quality for non-planar surfaces.
pub fn offset_shell<C, S>(
    shell: &Shell<Point3, C, S>,
    distance: f64,
    n_samples: usize,
) -> Option<Shell<Point3, C, S>>
where
    C: Clone + OffsetCurve,
    S: Clone + ParametricSurface3D<Point = Point3, Vector = Vector3> + OffsetSurface,
{
    let compressed = shell.compress();
    let n_verts = compressed.vertices.len();

    // Build vertex-to-face adjacency: for each vertex, which faces reference it.
    let mut vertex_faces: Vec<Vec<usize>> = vec![Vec::new(); n_verts];
    compressed.faces.iter().enumerate().for_each(|(fi, face)| {
        face.boundaries.iter().for_each(|boundary| {
            boundary.iter().for_each(|edge_idx| {
                let edge = &compressed.edges[edge_idx.index];
                if !vertex_faces[edge.vertices.0].contains(&fi) {
                    vertex_faces[edge.vertices.0].push(fi);
                }
                if !vertex_faces[edge.vertices.1].contains(&fi) {
                    vertex_faces[edge.vertices.1].push(fi);
                }
            });
        });
    });

    // Offset each face surface.
    // The offset trait works in the geometric normal direction. When a face has
    // `orientation == false`, the outward normal is `-geometric_normal`, so the
    // effective offset distance must be negated to produce the correct result.
    let offset_surfaces: Vec<S> = compressed
        .faces
        .iter()
        .map(|f| {
            let effective_distance = if f.orientation { distance } else { -distance };
            f.surface.offset(effective_distance, n_samples)
        })
        .collect::<Option<Vec<_>>>()?;

    // Compute offset vertex positions.
    // For planar faces, find the intersection of the three offset planes that meet
    // at each vertex. For curved faces, use the average-normal approach.
    let offset_vertices: Vec<Point3> = (0..n_verts)
        .map(|vi| {
            let pt = compressed.vertices[vi];
            let adj_faces = &vertex_faces[vi];

            if adj_faces.is_empty() {
                return pt;
            }

            // Try to compute the vertex as intersection of offset planes.
            // Collect (normal, point-on-offset-surface) pairs for adjacent faces.
            let plane_data: Vec<(Vector3, Point3)> = adj_faces
                .iter()
                .filter_map(|&fi| {
                    let offset_surf = &offset_surfaces[fi];
                    let orig_surf = &compressed.faces[fi].surface;
                    let orientation = compressed.faces[fi].orientation;

                    // Get normal at the vertex location on the original surface.
                    let (normal, (u, v)) = orig_surf.normal_at_closest(pt)?;
                    let oriented_normal = if orientation { normal } else { -normal };

                    // Point on the offset surface at same parameters.
                    let offset_pt = offset_surf.evaluate_at(u, v);

                    Some((oriented_normal, offset_pt))
                })
                .collect();

            if plane_data.len() >= 3 {
                // Solve the linear system: for each plane i, n_i . x = n_i . p_i.
                // Use the first 3 planes (for a cube vertex, exactly 3 planes meet).
                let n0 = plane_data[0].0;
                let n1 = plane_data[1].0;
                let n2 = plane_data[2].0;
                let d0 = n0.dot(plane_data[0].1.to_vec());
                let d1 = n1.dot(plane_data[1].1.to_vec());
                let d2 = n2.dot(plane_data[2].1.to_vec());

                // Each row i is normal n_i, so n_i · x = d_i.
                // cgmath `Matrix3::new` takes column-major arguments.
                let mat = Matrix3::new(n0.x, n1.x, n2.x, n0.y, n1.y, n2.y, n0.z, n1.z, n2.z);

                match mat.invert() {
                    Some(inv) => {
                        let rhs = Vector3::new(d0, d1, d2);
                        let result = inv * rhs;
                        Point3::new(result.x, result.y, result.z)
                    }
                    // Planes are parallel or degenerate; fall back to average normal offset.
                    None => offset_by_average_normal(pt, &plane_data, distance),
                }
            } else {
                // Not enough adjacent faces for plane intersection; use average normal.
                offset_by_average_normal(pt, &plane_data, distance)
            }
        })
        .collect();

    // Validate that the offset hasn't caused self-intersection by checking that
    // each edge direction is consistent: the original edge vector and offset edge
    // vector should point in the same general direction (positive dot product).
    let self_intersecting = compressed.edges.iter().any(|e| {
        let orig_dir = compressed.vertices[e.vertices.1] - compressed.vertices[e.vertices.0];
        let offset_dir = offset_vertices[e.vertices.1] - offset_vertices[e.vertices.0];
        orig_dir.dot(offset_dir) < 0.0
    });
    if self_intersecting {
        return None;
    }

    // Compute offset edges: create curves between offset vertices.
    let offset_edges: Vec<CompressedEdge<C>> = compressed
        .edges
        .iter()
        .map(|e| {
            let p0 = offset_vertices[e.vertices.0];
            let p1 = offset_vertices[e.vertices.1];
            CompressedEdge {
                vertices: e.vertices,
                curve: C::offset_curve(&e.curve, p0, p1),
            }
        })
        .collect();

    // Reconstruct the offset faces with offset surfaces.
    let offset_faces: Vec<CompressedFace<S>> = compressed
        .faces
        .into_iter()
        .zip(offset_surfaces)
        .map(|(f, surface)| CompressedFace {
            boundaries: f.boundaries,
            orientation: f.orientation,
            surface,
        })
        .collect();

    let offset_compressed = CompressedShell {
        vertices: offset_vertices,
        edges: offset_edges,
        faces: offset_faces,
    };

    Shell::extract(offset_compressed).ok()
}

/// Fallback: offset a vertex by averaging normals of adjacent faces.
fn offset_by_average_normal(pt: Point3, plane_data: &[(Vector3, Point3)], distance: f64) -> Point3 {
    if plane_data.is_empty() {
        return pt;
    }
    let avg_normal = plane_data
        .iter()
        .fold(Vector3::zero(), |acc, &(n, _)| acc + n);
    let len = avg_normal.magnitude();
    if len.so_small() {
        pt
    } else {
        pt + (avg_normal / len) * distance
    }
}
