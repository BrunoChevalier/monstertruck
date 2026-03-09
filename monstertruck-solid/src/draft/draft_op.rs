//! Draft/taper operation implementation.

use monstertruck_core::cgmath64::*;
use monstertruck_geometry::prelude::*;
use monstertruck_topology::compress::*;
use monstertruck_topology::*;
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

/// Apply draft angle to specified faces of a solid.
///
/// Each selected face is tilted by `angle` relative to `pull_direction`,
/// pivoting around the intersection with `neutral_plane`.
///
/// Returns a new solid with drafted faces.
pub fn draft_faces<C, S>(
    _solid: &Solid<Point3, C, S>,
    _face_indices: &[usize],
    _options: &DraftOptions,
) -> std::result::Result<Solid<Point3, C, S>, DraftError> {
    todo!()
}
