//! Shell (hollow-out) and surface offset operations for solid bodies.
//!
//! This module provides two main operations:
//!
//! - [`shell_solid`]: Hollows out a solid body by creating an inner offset shell,
//!   producing a solid with the specified wall thickness.
//! - [`offset_shell`]: Offsets all surfaces of a shell by a given distance along
//!   their normals.
//!
//! Both operations are generic over the curve and surface types, requiring
//! implementations of [`OffsetCurve`] and [`OffsetSurface`] traits.
//!
//! # Planar surfaces
//!
//! For shells composed entirely of [`Plane`](monstertruck_geometry::specifieds::Plane)
//! surfaces (cubes, boxes, prisms), vertex positions are computed exactly as the
//! intersection of three offset planes. This avoids approximation errors inherent
//! in the average-normal approach.
//!
//! # Curved surfaces
//!
//! For shells with curved surfaces, the [`OffsetSurface`] trait delegates to the
//! geometry crate's `surface_offset` function, which uses a sample-and-refit
//! strategy.

mod offset_op;
mod shell_op;

#[cfg(test)]
mod tests;

pub use offset_op::{OffsetCurve, OffsetSurface, offset_shell};
pub use shell_op::shell_solid;
