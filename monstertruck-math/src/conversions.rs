//! Extension traits adding cgmath-style methods to nalgebra types.

use crate::traits::BaseFloat;
use crate::types::*;
use nalgebra as na;

/// Extension trait providing `truncate()` on vectors, matching cgmath's API.
pub trait Truncate {
    /// The truncated vector type.
    type Output;
    /// Returns the vector with the last component removed.
    fn truncate(self) -> Self::Output;
}

impl<S: BaseFloat> Truncate for Vector4<S> {
    type Output = Vector3<S>;
    #[inline]
    fn truncate(self) -> Vector3<S> {
        Vector3::new(self[0], self[1], self[2])
    }
}

impl<S: BaseFloat> Truncate for Vector3<S> {
    type Output = Vector2<S>;
    #[inline]
    fn truncate(self) -> Vector2<S> {
        Vector2::new(self[0], self[1])
    }
}

impl<S: BaseFloat> Truncate for Vector2<S> {
    type Output = Vector1<S>;
    #[inline]
    fn truncate(self) -> Vector1<S> {
        Vector1::new(self[0])
    }
}

/// Extension trait providing `to_homogeneous()` on points, matching cgmath's API.
pub trait ToHomogeneous {
    /// The homogeneous vector type.
    type Output;
    /// Converts the point to a homogeneous vector (appends `1.0` as last component).
    fn to_homogeneous(self) -> Self::Output;
}

impl<S: BaseFloat> ToHomogeneous for na::Point3<S> {
    type Output = Vector4<S>;
    #[inline]
    fn to_homogeneous(self) -> Vector4<S> {
        Vector4::new(self[0], self[1], self[2], S::one())
    }
}

impl<S: BaseFloat> ToHomogeneous for na::Point2<S> {
    type Output = Vector3<S>;
    #[inline]
    fn to_homogeneous(self) -> Vector3<S> {
        Vector3::new(self[0], self[1], S::one())
    }
}

impl<S: BaseFloat> ToHomogeneous for na::Point1<S> {
    type Output = Vector2<S>;
    #[inline]
    fn to_homogeneous(self) -> Vector2<S> {
        Vector2::new(self[0], S::one())
    }
}

/// Extension trait providing `extend()` on vectors.
pub trait Extend<S> {
    /// The extended vector type.
    type Output;
    /// Returns the vector with an extra component appended.
    fn extend(self, value: S) -> Self::Output;
}

impl<S: BaseFloat> Extend<S> for Vector3<S> {
    type Output = Vector4<S>;
    #[inline]
    fn extend(self, w: S) -> Vector4<S> {
        Vector4::new(self[0], self[1], self[2], w)
    }
}

impl<S: BaseFloat> Extend<S> for Vector2<S> {
    type Output = Vector3<S>;
    #[inline]
    fn extend(self, z: S) -> Vector3<S> {
        Vector3::new(self[0], self[1], z)
    }
}

impl<S: BaseFloat> Extend<S> for Vector1<S> {
    type Output = Vector2<S>;
    #[inline]
    fn extend(self, y: S) -> Vector2<S> {
        Vector2::new(self[0], y)
    }
}

/// Extension trait providing `mul_element_wise()` matching cgmath's `ElementWise`.
pub trait MulElementWise {
    /// Element-wise multiply.
    fn mul_element_wise(self, other: Self) -> Self;
}

macro_rules! impl_mul_element_wise {
    ($vtype:ty) => {
        impl<S: BaseFloat> MulElementWise for $vtype {
            #[inline]
            fn mul_element_wise(self, other: Self) -> Self {
                self.component_mul(&other)
            }
        }
    };
}

impl_mul_element_wise!(na::SVector<S, 1>);
impl_mul_element_wise!(na::Vector2<S>);
impl_mul_element_wise!(na::Vector3<S>);
impl_mul_element_wise!(na::Vector4<S>);

/// Extension trait providing `cast()` matching cgmath's `cast()` on vectors.
pub trait Cast<T> {
    /// The target type with a different scalar.
    type Output;
    /// Casts the scalar components to a different float type.
    fn cast(self) -> Option<Self::Output>;
}

macro_rules! impl_cast_vector {
    ($dim:expr) => {
        impl<S: BaseFloat, T: BaseFloat> Cast<T> for na::SVector<S, $dim>
        where
            T: num_traits::NumCast,
            S: num_traits::NumCast,
        {
            type Output = na::SVector<T, $dim>;
            fn cast(self) -> Option<Self::Output> {
                let mut out = na::SVector::<T, $dim>::zeros();
                for i in 0..$dim {
                    out[i] = T::from(self[i])?;
                }
                Some(out)
            }
        }
    };
}

impl_cast_vector!(1);
impl_cast_vector!(2);
impl_cast_vector!(3);
impl_cast_vector!(4);

macro_rules! impl_cast_point {
    ($dim:expr) => {
        impl<S: BaseFloat, T: BaseFloat> Cast<T> for na::Point<S, $dim>
        where
            T: num_traits::NumCast,
            S: num_traits::NumCast,
        {
            type Output = na::Point<T, $dim>;
            fn cast(self) -> Option<Self::Output> {
                let mut out = na::Point::<T, $dim>::origin();
                for i in 0..$dim {
                    out[i] = T::from(self[i])?;
                }
                Some(out)
            }
        }
    };
}

impl_cast_point!(1);
impl_cast_point!(2);
impl_cast_point!(3);

// ---- Convenience constructors ----
// cgmath supports `(x, y, z).into()` for vectors.
// nalgebra does NOT provide these due to orphan rules,
// so we provide convenience free functions instead.

/// Creates a `Vector2<f64>` from a tuple.
pub fn vec2(x: f64, y: f64) -> Vector2<f64> {
    Vector2::new(x, y)
}

/// Creates a `Vector3<f64>` from a tuple.
pub fn vec3(x: f64, y: f64, z: f64) -> Vector3<f64> {
    Vector3::new(x, y, z)
}

/// Creates a `Vector4<f64>` from a tuple.
pub fn vec4(x: f64, y: f64, z: f64, w: f64) -> Vector4<f64> {
    Vector4::new(x, y, z, w)
}
