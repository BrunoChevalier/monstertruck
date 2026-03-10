//! Draft (taper) operations for solid bodies.
//!
//! Applies a draft angle to selected faces relative to a pull direction
//! and neutral plane. Used in injection mold design to ensure parts
//! can be released from molds.
//!
//! The primary entry point is [`draft_faces`], which accepts a [`Solid`],
//! a list of face indices to draft, and [`DraftOptions`] specifying the
//! draft angle, pull direction, and neutral plane. Faces are tilted by
//! rotating around the hinge line where each face plane intersects the
//! neutral plane. Vertex positions are recomputed as 3-plane intersections.
//!
//! [`Solid`]: monstertruck_topology::Solid

mod draft_op;

#[cfg(test)]
mod tests;

pub use draft_op::{DraftError, DraftOptions, draft_faces};
