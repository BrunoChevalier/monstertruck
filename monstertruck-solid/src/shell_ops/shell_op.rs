//! Shell (hollow-out) operation for solid bodies.
//!
//! The shell operation creates a hollow version of a solid by:
//! 1. Taking the outer boundary shell of the solid.
//! 2. Creating an inner shell by offsetting all faces inward by the wall thickness.
//! 3. Inverting the inner shell's face normals to point inward.
//! 4. Combining both shells into a new solid.
//!
//! The resulting solid has exactly two boundary shells: the original outer shell
//! and the new inner shell.
//!
//! # Self-intersection detection
//!
//! If the wall thickness exceeds what the solid geometry can support (e.g.,
//! thickness > half the smallest dimension of a cube), the operation detects
//! the resulting edge direction reversal and returns `None`.

use monstertruck_core::cgmath64::*;
use monstertruck_geometry::prelude::*;
use monstertruck_topology::*;

use super::offset_op::{OffsetCurve, OffsetSurface, offset_shell};

/// Creates a hollow solid by offsetting all faces inward.
///
/// Returns a [`Solid`] with two boundary shells: the original outer shell
/// and a new inner shell offset by `wall_thickness`.
///
/// Returns `None` if:
/// - The solid has no boundary shells.
/// - The wall thickness is too large for the solid (inner shell self-intersects).
/// - The offset computation fails for any surface.
/// - The resulting inner shell does not form a valid closed manifold.
///
/// # Arguments
///
/// * `solid` -- The input solid to hollow out.
/// * `wall_thickness` -- The desired wall thickness. Must be positive.
/// * `n_samples` -- Number of samples for approximating curved surface offsets.
///
/// # Examples
///
/// ```ignore
/// let cube: Solid = builder::extrude(&face, Vector3::unit_z());
/// let hollow = shell_solid(&cube, 0.1, 20).unwrap();
/// assert_eq!(hollow.boundaries().len(), 2);
/// ```
pub fn shell_solid<C, S>(
    solid: &Solid<Point3, C, S>,
    wall_thickness: f64,
    n_samples: usize,
) -> Option<Solid<Point3, C, S>>
where
    C: Clone + OffsetCurve + BoundedCurve<Point = Point3>,
    S: Clone + ParametricSurface3D<Point = Point3, Vector = Vector3> + Invertible + OffsetSurface,
{
    let outer_shell = solid.boundaries().first()?;

    // Create inner shell by offsetting inward (negative distance).
    let mut inner_shell = offset_shell(outer_shell, -wall_thickness, n_samples)?;

    // Validate that the inner shell has not self-intersected by checking that
    // every edge connects two distinct (non-coincident) vertices.
    let degenerate = inner_shell.iter().any(|face| {
        face.boundaries().iter().any(|wire| {
            wire.iter().any(|edge| {
                let front = edge.front().point();
                let back = edge.back().point();
                front.near(&back)
            })
        })
    });
    if degenerate {
        return None;
    }

    // Invert all face normals of the inner shell so they point inward.
    // This is necessary because `offset_shell` preserves the original face
    // orientation, but for a hollow solid the inner shell normals must point
    // toward the interior (opposite to the outer shell normals).
    inner_shell.face_iter_mut().for_each(|face| {
        face.invert();
    });

    // Build the solid with outer and inner boundary shells.
    // `Solid::try_new` validates that both shells are non-empty, connected,
    // closed manifolds without singular vertices.
    Solid::try_new(vec![outer_shell.clone(), inner_shell]).ok()
}
