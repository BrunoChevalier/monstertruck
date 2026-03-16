//! Math adapter crate providing a cgmath-compatible API over nalgebra.
//!
//! This crate wraps [`nalgebra`] types and re-exports them under names matching
//! the cgmath convention (`Vector3`, `Point3`, `Matrix4`, etc.), along with
//! trait bridges (`BaseFloat`, `VectorSpace`, `InnerSpace`, `EuclideanSpace`,
//! `MetricSpace`) so that downstream code originally written against cgmath can
//! migrate with minimal source changes.

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

/// Conversion extension traits (truncate, extend, to_homogeneous, cast, etc.).
pub mod conversions;
/// Trait bridges for cgmath-compatible trait names.
pub mod traits;
/// Type aliases and wrapper types.
pub mod types;

/// Polynomial equation solvers (quadratic, cubic, quartic).
pub mod polynomial;

// Re-export all types at crate root.
pub use types::{
    Deg, Matrix2, Matrix3, Matrix4, Point1, Point2, Point3, Rad, Vector1, Vector2, Vector3, Vector4,
};

// Re-export all traits at crate root.
pub use traits::{
    BaseFloat, EuclideanSpace, InnerSpace, MetricSpace, One, SquareMatrix, Transform, VectorSpace,
    Zero,
};

// Re-export conversion traits at crate root.
pub use conversions::{Cast, Extend, MulElementWise, ToHomogeneous, Truncate, UnitVectors};

// Re-export approx traits (cgmath re-exports these).
pub use approx::AbsDiffEq;

// Re-export num_traits (cgmath re-exports these).
pub use num_traits;

// Re-export num_complex so downstream crates can use `Complex` without adding their own dep.
pub use num_complex;

// Re-export nalgebra for advanced use.
pub use nalgebra;

/// Prelude module mimicking `cgmath::prelude::*`.
pub mod prelude {
    pub use crate::conversions::{
        Cast, Extend, MulElementWise, ToHomogeneous, Truncate, UnitVectors,
    };
    pub use crate::traits::{
        BaseFloat, EuclideanSpace, InnerSpace, MetricSpace, One, SquareMatrix, Transform,
        VectorSpace, Zero,
    };
    pub use approx::AbsDiffEq;
}

/// Creates a perspective projection matrix (right-handed, zero-to-one depth).
pub fn perspective<S: BaseFloat>(fovy: Rad<S>, aspect: S, near: S, far: S) -> Matrix4<S> {
    let two = S::one() + S::one();
    let f = S::one() / num_traits::Float::tan(fovy.0 / two);
    Matrix4::new(
        f / aspect,
        S::zero(),
        S::zero(),
        S::zero(),
        S::zero(),
        f,
        S::zero(),
        S::zero(),
        S::zero(),
        S::zero(),
        (far + near) / (near - far),
        (two * far * near) / (near - far),
        S::zero(),
        S::zero(),
        -S::one(),
        S::zero(),
    )
}

/// Creates an orthographic projection matrix.
pub fn ortho<S: BaseFloat>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {
    let two = S::one() + S::one();
    let zero = S::zero();
    let one = S::one();
    let rl = right - left;
    let tb = top - bottom;
    let f_n = far - near;
    Matrix4::new(
        two / rl,
        zero,
        zero,
        -(right + left) / rl,
        zero,
        two / tb,
        zero,
        -(top + bottom) / tb,
        zero,
        zero,
        -two / f_n,
        -(far + near) / f_n,
        zero,
        zero,
        zero,
        one,
    )
}

/// Creates a frustum projection matrix.
pub fn frustum<S: BaseFloat>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {
    let two = S::one() + S::one();
    let zero = S::zero();
    let rl = right - left;
    let tb = top - bottom;
    let f_n = far - near;
    Matrix4::new(
        two * near / rl,
        zero,
        (right + left) / rl,
        zero,
        zero,
        two * near / tb,
        (top + bottom) / tb,
        zero,
        zero,
        zero,
        -(far + near) / f_n,
        -(two * far * near) / f_n,
        zero,
        zero,
        -S::one(),
        zero,
    )
}
