//! Draft (taper) operations for solid bodies.
//!
//! Applies a draft angle to selected faces relative to a pull direction
//! and neutral plane. Used in injection mold design to ensure parts
//! can be released from molds.

mod draft_op;

#[cfg(test)]
mod tests;

pub use draft_op::{DraftError, DraftOptions, draft_faces};
