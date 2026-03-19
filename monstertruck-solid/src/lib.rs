//! Solid modeling operations: boolean, chamfer/fillet, shell/offset, and draft/taper.
//!
//! Provides the following operations on [`Solid`](monstertruck_topology::Solid) bodies:
//!
//! - **Boolean operations** ([`and`], [`or`], [`difference`], [`symmetric_difference`]):
//!   Combine solids via CSG operations.
//! - **Fillet/chamfer** ([`fillet`]): Round or bevel edges of a shell.
//! - **Shell/offset** ([`shell_ops`]): Hollow out a solid or offset shell surfaces.
//! - **Draft/taper** ([`draft`]): Apply mold-release draft angles to selected faces.
//! - **Shape healing** ([`extract_healed`], [`heal_surface_shell`]): Repair topology
//!   for imported shapes and surfaces from sweep/birail/gordon constructors.

#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(clippy::all, rust_2018_idioms)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

mod healing;
pub use healing::{
    RobustSplitClosedEdgesAndFaces, SplitClosedEdgesAndFaces, SurfaceHealingError, extract_healed,
    heal_surface_shell,
};
pub mod shell_ops;
pub use shell_ops::{OffsetCurve, OffsetSurface, offset_shell, shell_solid};
pub mod draft;
pub use draft::{DraftError, DraftOptions, draft_faces};
mod transversal;
pub use transversal::{
    ShapeOpsCurve, ShapeOpsError, ShapeOpsSurface, and, difference, or, symmetric_difference,
};
mod alternative;
pub mod fillet;
pub use fillet::{
    ContinuityAnnotation, CornerMode, ExtendMode, FilletError, FilletIntersectionCurve, FilletMode,
    FilletOptions, FilletProfile, FilletResult, FilletableCurve, FilletableSurface,
    ParameterCurveLinear, RadiusSpec, fillet, fillet_along_wire, fillet_annotated, fillet_edges,
    fillet_edges_generic, fillet_with_side,
};
