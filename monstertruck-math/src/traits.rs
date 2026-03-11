//! Trait bridges providing cgmath-compatible trait names backed by nalgebra.

use crate::types;
use nalgebra as na;
use num_traits::{Float, NumCast};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign};

// Re-export core numeric traits under cgmath-style names.
pub use num_traits::{One, Zero};

/// Marker trait matching cgmath's `BaseFloat`.
///
/// Implemented for `f32` and `f64`. Combines standard float operations
/// with common numeric-cast and approx-comparison requirements.
pub trait BaseFloat:
    Float
    + NumCast
    + num_traits::FromPrimitive
    + num_traits::FloatConst
    + Debug
    + Copy
    + Default
    + Send
    + Sync
    + 'static
    + approx::AbsDiffEq<Epsilon = Self>
    + approx::RelativeEq<Epsilon = Self>
    + approx::UlpsEq<Epsilon = Self>
    + std::fmt::Display
    + na::RealField
{
}

impl BaseFloat for f32 {}
impl BaseFloat for f64 {}

/// Trait matching cgmath's `VectorSpace`.
///
/// Requires addition, subtraction, scalar multiplication/division and a zero element.
pub trait VectorSpace:
    Copy
    + Clone
    + Debug
    + PartialEq
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self::Scalar, Output = Self>
    + Div<Self::Scalar, Output = Self>
    + AddAssign
    + SubAssign
    + Neg<Output = Self>
    + Zero
{
    /// The scalar type.
    type Scalar: BaseFloat;
}

/// Trait matching cgmath's `InnerSpace`.
pub trait InnerSpace: VectorSpace {
    /// Dot product.
    fn dot(self, other: Self) -> Self::Scalar;
    /// Euclidean norm (magnitude).
    fn magnitude(self) -> Self::Scalar;
    /// Squared magnitude.
    fn magnitude2(self) -> Self::Scalar;
    /// Returns a unit vector in the same direction.
    fn normalize(self) -> Self;
}

/// Trait matching cgmath's `EuclideanSpace`.
pub trait EuclideanSpace:
    Copy
    + Clone
    + Debug
    + PartialEq
    + Add<<Self as EuclideanSpace>::Diff, Output = Self>
    + Sub<<Self as EuclideanSpace>::Diff, Output = Self>
    + Sub<Self, Output = <Self as EuclideanSpace>::Diff>
    + Index<usize, Output = Self::Scalar>
    + IndexMut<usize>
{
    /// The scalar type.
    type Scalar: BaseFloat;
    /// The difference (vector) type.
    type Diff: VectorSpace<Scalar = Self::Scalar>
        + InnerSpace
        + Index<usize, Output = Self::Scalar>
        + IndexMut<usize>;
    /// Returns the origin.
    fn origin() -> Self;
    /// Converts the point to a vector from the origin.
    fn to_vec(self) -> Self::Diff;
    /// Creates a point from a vector from the origin.
    fn from_vec(v: Self::Diff) -> Self;
    /// Returns the midpoint between two points.
    fn midpoint(self, other: Self) -> Self {
        let two = Self::Scalar::one() + Self::Scalar::one();
        Self::from_vec((self.to_vec() + other.to_vec()) / two)
    }
}

/// Trait matching cgmath's `MetricSpace`.
pub trait MetricSpace {
    /// The metric type.
    type Metric;
    /// Distance between two points.
    fn distance(self, other: Self) -> Self::Metric;
    /// Squared distance.
    fn distance2(self, other: Self) -> Self::Metric;
}

/// Trait matching cgmath's `SquareMatrix`.
pub trait SquareMatrix: Sized {
    /// The scalar type.
    type Scalar: BaseFloat;
    /// Returns the identity matrix.
    fn identity() -> Self;
    /// Inverts the matrix. Returns `None` if singular.
    fn invert(&self) -> Option<Self>;
    /// Creates a diagonal matrix (cgmath's `from_value`).
    fn from_value(value: Self::Scalar) -> Self;
}

/// Trait matching cgmath's `Transform`.
pub trait Transform<P> {
    /// Transforms a point.
    fn transform_point(&self, point: P) -> P;
}

// ---- VectorSpace impls for nalgebra vectors ----

macro_rules! impl_vector_space {
    ($vtype:ty) => {
        impl<S: BaseFloat> VectorSpace for $vtype {
            type Scalar = S;
        }
    };
}

impl_vector_space!(na::SVector<S, 1>);
impl_vector_space!(na::Vector2<S>);
impl_vector_space!(na::Vector3<S>);
impl_vector_space!(na::Vector4<S>);

// ---- InnerSpace impls ----

macro_rules! impl_inner_space {
    ($vtype:ty) => {
        impl<S: BaseFloat> InnerSpace for $vtype {
            #[inline]
            fn dot(self, other: Self) -> S {
                na::Matrix::dot(&self, &other)
            }
            #[inline]
            fn magnitude(self) -> S {
                na::Matrix::norm(&self)
            }
            #[inline]
            fn magnitude2(self) -> S {
                na::Matrix::norm_squared(&self)
            }
            #[inline]
            fn normalize(self) -> Self {
                na::Matrix::normalize(&self)
            }
        }
    };
}

impl_inner_space!(na::SVector<S, 1>);
impl_inner_space!(na::Vector2<S>);
impl_inner_space!(na::Vector3<S>);
impl_inner_space!(na::Vector4<S>);

// ---- EuclideanSpace impls for nalgebra points ----

macro_rules! impl_euclidean_space {
    ($ptype:ty, $vtype:ty) => {
        impl<S: BaseFloat> EuclideanSpace for $ptype {
            type Scalar = S;
            type Diff = $vtype;
            #[inline]
            fn origin() -> Self {
                na::Point::origin()
            }
            #[inline]
            fn to_vec(self) -> Self::Diff {
                self.coords
            }
            #[inline]
            fn from_vec(v: Self::Diff) -> Self {
                na::Point::from(v)
            }
        }
    };
}

impl_euclidean_space!(na::Point1<S>, na::SVector<S, 1>);
impl_euclidean_space!(na::Point2<S>, na::Vector2<S>);
impl_euclidean_space!(na::Point3<S>, na::Vector3<S>);

// ---- MetricSpace impls ----

macro_rules! impl_metric_space_point {
    ($ptype:ty) => {
        impl<S: BaseFloat> MetricSpace for $ptype {
            type Metric = S;
            #[inline]
            fn distance(self, other: Self) -> S {
                na::distance(&self, &other)
            }
            #[inline]
            fn distance2(self, other: Self) -> S {
                na::distance_squared(&self, &other)
            }
        }
    };
}

impl_metric_space_point!(na::Point1<S>);
impl_metric_space_point!(na::Point2<S>);
impl_metric_space_point!(na::Point3<S>);

macro_rules! impl_metric_space_vector {
    ($vtype:ty) => {
        impl<S: BaseFloat> MetricSpace for $vtype {
            type Metric = S;
            #[inline]
            fn distance(self, other: Self) -> S {
                (self - other).norm()
            }
            #[inline]
            fn distance2(self, other: Self) -> S {
                (self - other).norm_squared()
            }
        }
    };
}

impl_metric_space_vector!(na::SVector<S, 1>);
impl_metric_space_vector!(na::Vector2<S>);
impl_metric_space_vector!(na::Vector3<S>);
impl_metric_space_vector!(na::Vector4<S>);

// ---- SquareMatrix impls for wrapper types ----

impl<S: BaseFloat> SquareMatrix for types::Matrix2<S> {
    type Scalar = S;
    #[inline]
    fn identity() -> Self {
        types::Matrix2::identity()
    }
    #[inline]
    fn invert(&self) -> Option<Self> {
        self.0.try_inverse().map(types::Matrix2)
    }
    #[inline]
    fn from_value(value: S) -> Self {
        types::Matrix2::from_value(value)
    }
}

impl<S: BaseFloat> SquareMatrix for types::Matrix3<S> {
    type Scalar = S;
    #[inline]
    fn identity() -> Self {
        types::Matrix3::identity()
    }
    #[inline]
    fn invert(&self) -> Option<Self> {
        self.0.try_inverse().map(types::Matrix3)
    }
    #[inline]
    fn from_value(value: S) -> Self {
        types::Matrix3::from_value(value)
    }
}

impl<S: BaseFloat> SquareMatrix for types::Matrix4<S> {
    type Scalar = S;
    #[inline]
    fn identity() -> Self {
        types::Matrix4::identity()
    }
    #[inline]
    fn invert(&self) -> Option<Self> {
        self.0.try_inverse().map(types::Matrix4)
    }
    #[inline]
    fn from_value(value: S) -> Self {
        types::Matrix4::from_value(value)
    }
}

// ---- VectorSpace impls for matrix wrapper types ----

impl<S: BaseFloat> VectorSpace for types::Matrix2<S> {
    type Scalar = S;
}

impl<S: BaseFloat> VectorSpace for types::Matrix3<S> {
    type Scalar = S;
}

impl<S: BaseFloat> VectorSpace for types::Matrix4<S> {
    type Scalar = S;
}

// ---- Transform impls ----

impl<S: BaseFloat> Transform<na::Point3<S>> for types::Matrix4<S> {
    fn transform_point(&self, point: na::Point3<S>) -> na::Point3<S> {
        let h = self.0 * point.to_homogeneous();
        na::Point3::new(h[0] / h[3], h[1] / h[3], h[2] / h[3])
    }
}

impl<S: BaseFloat> Transform<na::Point2<S>> for types::Matrix3<S> {
    fn transform_point(&self, point: na::Point2<S>) -> na::Point2<S> {
        let h = self.0 * na::Vector3::new(point[0], point[1], S::one());
        na::Point2::new(h[0] / h[2], h[1] / h[2])
    }
}
