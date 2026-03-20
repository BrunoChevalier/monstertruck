//! Geometric primitives for CAD modeling: B-spline and NURBS curves/surfaces,
//! knot vectors, and decorator types (revolved, extruded, intersection curves).
//!
//! # Migration Guide (v0.5.0+)
//!
//! Surface constructor functions have been migrated from panicking APIs to
//! fallible `try_*` variants that return [`Result`].  The old functions are
//! deprecated and will be removed in a future version.
//!
//! ## Quick Reference
//!
//! | Deprecated | Replacement | Options type |
//! |-----------|-------------|-------------|
//! | `BsplineSurface::skin` | [`BsplineSurface::try_skin`](nurbs::BsplineSurface::try_skin) | [`SkinOptions`](nurbs::surface_options::SkinOptions) |
//! | `BsplineSurface::sweep_rail` | [`BsplineSurface::try_sweep_rail`](nurbs::BsplineSurface::try_sweep_rail) | [`SweepRailOptions`](nurbs::surface_options::SweepRailOptions) |
//! | `BsplineSurface::birail1` | [`BsplineSurface::try_birail1`](nurbs::BsplineSurface::try_birail1) | [`Birail1Options`](nurbs::surface_options::Birail1Options) |
//! | `BsplineSurface::birail2` | [`BsplineSurface::try_birail2`](nurbs::BsplineSurface::try_birail2) | [`Birail2Options`](nurbs::surface_options::Birail2Options) |
//! | `BsplineSurface::gordon` | [`BsplineSurface::try_gordon`](nurbs::BsplineSurface::try_gordon) | [`GordonOptions`](nurbs::surface_options::GordonOptions) |
//!
//! ## New Functions (no deprecated counterpart)
//!
//! - [`BsplineSurface::try_gordon_from_network`](nurbs::BsplineSurface::try_gordon_from_network):
//!   Builds a Gordon surface by auto-computing intersection grid points from the
//!   curve network.
//! - [`BsplineSurface::try_gordon_verified`](nurbs::BsplineSurface::try_gordon_verified):
//!   Builds a Gordon surface from caller-supplied grid points after validating
//!   each lies on both curves.
//!
//! ## Before / After Example
//!
//! ```ignore
//! // BEFORE (panics on failure):
//! let surface = BsplineSurface::skin(curves);
//!
//! // AFTER (returns Result):
//! use monstertruck_geometry::nurbs::surface_options::SkinOptions;
//! let surface = BsplineSurface::try_skin(curves, &SkinOptions::default())?;
//! ```
//!
//! All `try_*` functions return [`errors::Error`] with detailed diagnostics
//! via [`nurbs::surface_diagnostics::CurveNetworkDiagnostic`].

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

use monstertruck_core::bounding_box::Bounded;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Bound};

const INCLUDE_CURVE_TRIALS: usize = 100;
const PRESEARCH_DIVISION: usize = 50;

/// re-export `monstertruck_core`
pub mod base {
    pub use monstertruck_core::{
        assert_near, assert_near2, bounding_box::BoundingBox, cgmath64::*, hash, hash::HashGen,
        prop_assert_near, prop_assert_near2, tolerance::*,
    };
    pub use monstertruck_traits::*;
    // Explicit re-export disambiguates the polynomial solver module from
    // `monstertruck_traits::polynomial` (which provides `PolynomialCurve`/`PolynomialSurface`).
    pub use monstertruck_core::cgmath64::polynomial;
}
/// NURBS and B-spline curves, surfaces, and knot vectors.
pub mod nurbs;

/// Error types for geometry operations.
pub mod errors;

/// Concrete geometric primitives: [`Plane`], [`Sphere`], [`Line`], etc.
pub mod specifieds;

/// Composite geometry: revolved curves, intersection curves, processor wrappers.
pub mod decorators;

/// T-Spline and T-NURCC surface types.
pub mod t_spline;

/// re-export all modules.
pub mod prelude {
    use crate::*;
    pub use base::*;
    pub use decorators::*;
    pub use errors::*;
    pub use nurbs::*;
    pub use specifieds::*;
    pub use t_spline::*;
}
