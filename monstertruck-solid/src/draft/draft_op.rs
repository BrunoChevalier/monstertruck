//! Draft/taper operation implementation.

use monstertruck_core::cgmath64::*;
use monstertruck_geometry::prelude::*;
use monstertruck_topology::compress::*;
use monstertruck_topology::*;
use std::f64::consts::FRAC_PI_2;
use thiserror::Error;

/// Errors for draft operations.
#[derive(Debug, Error)]
pub enum DraftError {
    /// The draft angle is out of valid range.
    #[error("draft angle must be between 0 and 90 degrees")]
    InvalidAngle,
    /// The pull direction is zero-length.
    #[error("pull direction must be non-zero")]
    InvalidPullDirection,
    /// A face could not be drafted.
    #[error("failed to draft face at index {index}")]
    FaceDraftFailed {
        /// The index of the face that failed.
        index: usize,
    },
    /// The resulting shell is topologically invalid.
    #[error("draft produced invalid topology: {message}")]
    InvalidTopology {
        /// Description of the topology error.
        message: String,
    },
}

/// Options for draft operations.
#[derive(Debug, Clone)]
pub struct DraftOptions {
    /// Draft angle in radians. Must be positive and less than PI/2.
    pub angle: f64,
    /// Pull direction (mold opening direction). Must be non-zero.
    pub pull_direction: Vector3,
    /// Neutral plane: faces are tilted relative to this plane.
    /// Points on the neutral plane remain fixed during drafting.
    pub neutral_plane: Plane,
}

/// Returns the oriented normal of a surface, flipped if `orientation` is false.
fn oriented_normal<S>(surface: &S, orientation: bool, u: f64, v: f64) -> Vector3
where
    S: ParametricSurface3D<Point = Point3, Vector = Vector3>,
{
    let n = ParametricSurface3D::normal(surface, u, v);
    if orientation { n } else { -n }
}

/// Apply draft angle to specified faces of a solid.
///
/// Each selected face is tilted by `angle` relative to `pull_direction`,
/// pivoting around the intersection with `neutral_plane`.
///
/// Returns a new solid with drafted faces.
pub fn draft_faces<C, S>(
    solid: &Solid<Point3, C, S>,
    face_indices: &[usize],
    options: &DraftOptions,
) -> std::result::Result<Solid<Point3, C, S>, DraftError>
where
    C: Clone + Transformed<Matrix4>,
    S: Clone + ParametricSurface3D<Point = Point3, Vector = Vector3> + Transformed<Matrix4>,
{
    // Validate inputs.
    if options.angle < 0.0 || options.angle >= FRAC_PI_2 {
        return Err(DraftError::InvalidAngle);
    }
    if options.pull_direction.magnitude().so_small() {
        return Err(DraftError::InvalidPullDirection);
    }

    let shell = solid
        .boundaries()
        .first()
        .ok_or_else(|| DraftError::InvalidTopology {
            message: "solid has no boundary shells".to_string(),
        })?;

    // Zero angle: clone the original solid.
    if options.angle.so_small() {
        return Ok(Solid::new_unchecked(solid.boundaries().clone()));
    }

    let pull = options.pull_direction.normalize();
    let neutral_origin = options.neutral_plane.origin();
    let neutral_normal = options.neutral_plane.normal();

    let compressed = shell.compress();
    let n_verts = compressed.vertices.len();
    let n_faces = compressed.faces.len();

    // Build adjacency maps: vertex-to-face and edge-to-face.
    let mut vertex_faces: Vec<Vec<usize>> = vec![Vec::new(); n_verts];
    let mut edge_face_map: Vec<Vec<usize>> = vec![Vec::new(); compressed.edges.len()];
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
                if !edge_face_map[edge_idx.index].contains(&fi) {
                    edge_face_map[edge_idx.index].push(fi);
                }
            });
        });
    });

    // For each face, compute the transform matrix (rotation about hinge).
    // Non-selected faces get the identity transform.
    let face_transforms: Vec<Matrix4> = (0..n_faces)
        .map(|fi| {
            if !face_indices.contains(&fi) {
                return Matrix4::identity();
            }
            let cface = &compressed.faces[fi];
            let face_normal = oriented_normal(&cface.surface, cface.orientation, 0.5, 0.5);

            compute_draft_transform(
                &cface.surface,
                &face_normal,
                &pull,
                &neutral_origin,
                &neutral_normal,
                options.angle,
            )
        })
        .collect();

    // Apply transforms to surfaces.
    let drafted_surfaces: Vec<S> = compressed
        .faces
        .iter()
        .zip(&face_transforms)
        .map(|(face, transform)| face.surface.transformed(*transform))
        .collect();

    // Apply transforms to edge curves.
    // Each edge is shared by exactly 2 faces. Pick a non-identity transform
    // when available. For Lines the curve is fully determined by endpoints,
    // so the specific transform choice does not matter at boundaries between
    // drafted and undrafted regions.
    let identity = Matrix4::identity();
    let drafted_edges: Vec<CompressedEdge<C>> = compressed
        .edges
        .iter()
        .enumerate()
        .map(|(ei, edge)| {
            let transform = edge_face_map[ei]
                .iter()
                .map(|&fi| &face_transforms[fi])
                .find(|t| **t != identity)
                .unwrap_or(&identity);
            CompressedEdge {
                vertices: edge.vertices,
                curve: edge.curve.transformed(*transform),
            }
        })
        .collect();

    // Collect (normal, point-on-surface) for each drafted surface for
    // 3-plane vertex intersection.
    let plane_data: Vec<(Vector3, Point3)> = (0..n_faces)
        .map(|fi| {
            let cface = &compressed.faces[fi];
            let face_normal = oriented_normal(&drafted_surfaces[fi], cface.orientation, 0.0, 0.0);
            let pt_on_plane = ParametricSurface::evaluate(&drafted_surfaces[fi], 0.0, 0.0);
            (face_normal, pt_on_plane)
        })
        .collect();

    // Compute new vertex positions as 3-plane intersections.
    let new_vertices: Vec<Point3> = (0..n_verts)
        .map(|vi| {
            let adj = &vertex_faces[vi];
            if adj.len() < 3 {
                return compressed.vertices[vi];
            }

            let n0 = plane_data[adj[0]].0;
            let n1 = plane_data[adj[1]].0;
            let n2 = plane_data[adj[2]].0;
            let d0 = n0.dot(plane_data[adj[0]].1.to_vec());
            let d1 = n1.dot(plane_data[adj[1]].1.to_vec());
            let d2 = n2.dot(plane_data[adj[2]].1.to_vec());

            // Column-major: row i is n_i, so n_i . x = d_i.
            let mat = Matrix3::new(n0.x, n1.x, n2.x, n0.y, n1.y, n2.y, n0.z, n1.z, n2.z);

            match mat.invert() {
                Some(inv) => {
                    let rhs = Vector3::new(d0, d1, d2);
                    let result = inv * rhs;
                    Point3::new(result.x, result.y, result.z)
                }
                None => compressed.vertices[vi],
            }
        })
        .collect();

    // Rebuild the compressed shell with new geometry.
    let new_faces: Vec<CompressedFace<S>> = compressed
        .faces
        .into_iter()
        .zip(drafted_surfaces)
        .map(|(f, surface)| CompressedFace {
            boundaries: f.boundaries,
            orientation: f.orientation,
            surface,
        })
        .collect();

    let new_compressed = CompressedShell {
        vertices: new_vertices,
        edges: drafted_edges,
        faces: new_faces,
    };

    let new_shell = Shell::extract(new_compressed).map_err(|e| DraftError::InvalidTopology {
        message: format!("{e}"),
    })?;

    Ok(Solid::new_unchecked(vec![new_shell]))
}

/// Computes the [`Matrix4`] transform that drafts a planar surface by rotating
/// its plane around the hinge axis (intersection of face plane and neutral plane).
fn compute_draft_transform<S>(
    surface: &S,
    face_normal: &Vector3,
    pull: &Vector3,
    neutral_origin: &Point3,
    neutral_normal: &Vector3,
    angle: f64,
) -> Matrix4
where
    S: ParametricSurface3D<Point = Point3, Vector = Vector3>,
{
    // Hinge axis = face_normal x neutral_normal.
    let hinge = face_normal.cross(neutral_normal);
    let hinge_mag = hinge.magnitude();

    if hinge_mag.so_small() {
        // Face is parallel to neutral plane (top/bottom). No draft.
        return Matrix4::identity();
    }
    let hinge = hinge / hinge_mag;

    // Determine rotation sign: face normal should tilt toward pull direction.
    let cross_test = face_normal.cross(pull);
    let sign = if cross_test.dot(hinge) >= 0.0 {
        1.0
    } else {
        -1.0
    };
    let rotation_angle = sign * angle;

    // Find hinge point: any point on the intersection line of the face
    // plane and the neutral plane. Solve the 2-plane intersection by
    // finding a point that satisfies both plane equations.
    let face_origin = ParametricSurface::evaluate(surface, 0.0, 0.0);
    let d_face = face_normal.dot(&face_origin.to_vec());
    let d_neutral = neutral_normal.dot(&neutral_origin.to_vec());

    // Build a 3x3 system: two plane equations plus the hinge direction
    // to pick the unique closest point to the origin.
    let mat = Matrix3::new(
        face_normal.x,
        neutral_normal.x,
        hinge.x,
        face_normal.y,
        neutral_normal.y,
        hinge.y,
        face_normal.z,
        neutral_normal.z,
        hinge.z,
    );
    let hinge_point = match mat.invert() {
        Some(inv) => {
            let rhs = Vector3::new(d_face, d_neutral, 0.0);
            let result = inv * rhs;
            Point3::new(result.x, result.y, result.z)
        }
        None => face_origin,
    };

    // Build the transform: translate to hinge, rotate, translate back.
    let rotation = Matrix3::from_axis_angle(hinge, Rad(rotation_angle));
    Matrix4::from_translation(hinge_point.to_vec())
        * Matrix4::from(rotation)
        * Matrix4::from_translation(-hinge_point.to_vec())
}
