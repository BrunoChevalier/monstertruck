//! Type aliases and wrapper types mapping cgmath names to nalgebra equivalents.

use nalgebra as na;
use serde::{Deserialize, Serialize};

// ---- Vector type aliases (direct nalgebra aliases) ----

/// Generic 1-dimensional vector.
pub type Vector1<S> = na::SVector<S, 1>;
/// Generic 2-dimensional vector.
pub type Vector2<S> = na::Vector2<S>;
/// Generic 3-dimensional vector.
pub type Vector3<S> = na::Vector3<S>;
/// Generic 4-dimensional vector.
pub type Vector4<S> = na::Vector4<S>;

/// Generic 1-dimensional point.
pub type Point1<S> = na::Point1<S>;
/// Generic 2-dimensional point.
pub type Point2<S> = na::Point2<S>;
/// Generic 3-dimensional point.
pub type Point3<S> = na::Point3<S>;

// ---- Matrix wrapper types ----
//
// nalgebra's `Matrix2::new(a, b, c, d)` takes row-major args (row 0: a b, row 1: c d).
// cgmath's `Matrix2::new(a, b, c, d)` takes column-major args (col 0: a b, col 1: c d).
// These wrappers preserve cgmath's column-major convention for `new`.

/// 2x2 matrix with cgmath-compatible column-major `new` constructor.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Matrix2<S: na::Scalar>(
    /// The underlying nalgebra matrix.
    pub na::Matrix2<S>,
);

/// 3x3 matrix with cgmath-compatible column-major `new` constructor.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Matrix3<S: na::Scalar>(
    /// The underlying nalgebra matrix.
    pub na::Matrix3<S>,
);

/// 4x4 matrix with cgmath-compatible column-major `new` constructor.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Matrix4<S: na::Scalar>(
    /// The underlying nalgebra matrix.
    pub na::Matrix4<S>,
);

// ---- From conversions between wrapper and nalgebra ----

impl<S: na::Scalar> From<na::Matrix2<S>> for Matrix2<S> {
    #[inline(always)]
    fn from(m: na::Matrix2<S>) -> Self {
        Matrix2(m)
    }
}
impl<S: na::Scalar> From<Matrix2<S>> for na::Matrix2<S> {
    #[inline(always)]
    fn from(m: Matrix2<S>) -> Self {
        m.0
    }
}
impl<S: na::Scalar> From<na::Matrix3<S>> for Matrix3<S> {
    #[inline(always)]
    fn from(m: na::Matrix3<S>) -> Self {
        Matrix3(m)
    }
}
impl<S: na::Scalar> From<Matrix3<S>> for na::Matrix3<S> {
    #[inline(always)]
    fn from(m: Matrix3<S>) -> Self {
        m.0
    }
}
impl<S: na::Scalar> From<na::Matrix4<S>> for Matrix4<S> {
    #[inline(always)]
    fn from(m: na::Matrix4<S>) -> Self {
        Matrix4(m)
    }
}
impl<S: na::Scalar> From<Matrix4<S>> for na::Matrix4<S> {
    #[inline(always)]
    fn from(m: Matrix4<S>) -> Self {
        m.0
    }
}

// ---- cgmath-compatible constructors (column-major arg order) ----

impl<S: na::Scalar + Copy> Matrix2<S> {
    /// Creates a 2x2 matrix from column-major arguments (cgmath convention).
    ///
    /// Arguments: `c0r0, c0r1, c1r0, c1r1` where `cXrY` = column X, row Y.
    #[inline]
    pub fn new(c0r0: S, c0r1: S, c1r0: S, c1r1: S) -> Self {
        // nalgebra::Matrix2::new takes row-major: (r0c0, r0c1, r1c0, r1c1).
        // Transpose: (c0r0, c1r0, c0r1, c1r1).
        Matrix2(na::Matrix2::new(c0r0, c1r0, c0r1, c1r1))
    }

    /// Creates a 2x2 matrix from two column vectors (cgmath convention).
    #[inline]
    pub fn from_cols(c0: Vector2<S>, c1: Vector2<S>) -> Self {
        Matrix2(na::Matrix2::from_columns(&[c0, c1]))
    }

    /// Returns the identity matrix.
    #[inline]
    pub fn identity() -> Self
    where
        S: na::RealField,
    {
        Matrix2(na::Matrix2::identity())
    }

    /// Creates a 2D rotation matrix from an angle.
    #[inline]
    pub fn from_angle(angle: Rad<S>) -> Self
    where
        S: na::RealField,
    {
        let (sin, cos) = angle.0.sin_cos();
        // cgmath convention: columns are [cos, sin] and [-sin, cos].
        Matrix2::new(cos, sin, -sin.clone(), cos)
    }

    /// Creates a diagonal matrix (from_value in cgmath).
    #[inline]
    pub fn from_value(value: S) -> Self
    where
        S: na::RealField,
    {
        Matrix2(na::Matrix2::identity() * value)
    }
}

impl<S: na::Scalar + Copy> Matrix3<S> {
    /// Creates a 3x3 matrix from column-major arguments (cgmath convention).
    ///
    /// Arguments: `c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2`.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        c0r0: S, c0r1: S, c0r2: S,
        c1r0: S, c1r1: S, c1r2: S,
        c2r0: S, c2r1: S, c2r2: S,
    ) -> Self {
        // Transpose to nalgebra's row-major constructor order.
        Matrix3(na::Matrix3::new(
            c0r0, c1r0, c2r0,
            c0r1, c1r1, c2r1,
            c0r2, c1r2, c2r2,
        ))
    }

    /// Creates a 3x3 matrix from three column vectors.
    #[inline]
    pub fn from_cols(c0: Vector3<S>, c1: Vector3<S>, c2: Vector3<S>) -> Self {
        Matrix3(na::Matrix3::from_columns(&[c0, c1, c2]))
    }

    /// Returns the identity matrix.
    #[inline]
    pub fn identity() -> Self
    where
        S: na::RealField,
    {
        Matrix3(na::Matrix3::identity())
    }

    /// Creates a diagonal matrix.
    #[inline]
    pub fn from_value(value: S) -> Self
    where
        S: na::RealField,
    {
        Matrix3(na::Matrix3::identity() * value)
    }

    /// Creates a Matrix3 from a Matrix2 rotation matrix (cgmath's `From`).
    #[inline]
    pub fn from_matrix2(m: Matrix2<S>) -> Self
    where
        S: na::RealField,
    {
        Matrix3::new(
            m[(0, 0)], m[(1, 0)], S::zero(),
            m[(0, 1)], m[(1, 1)], S::zero(),
            S::zero(), S::zero(), S::one(),
        )
    }
}

impl<S: na::Scalar + Copy> Matrix4<S> {
    /// Creates a 4x4 matrix from column-major arguments (cgmath convention).
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        c0r0: S, c0r1: S, c0r2: S, c0r3: S,
        c1r0: S, c1r1: S, c1r2: S, c1r3: S,
        c2r0: S, c2r1: S, c2r2: S, c2r3: S,
        c3r0: S, c3r1: S, c3r2: S, c3r3: S,
    ) -> Self {
        // Transpose to nalgebra's row-major constructor order.
        Matrix4(na::Matrix4::new(
            c0r0, c1r0, c2r0, c3r0,
            c0r1, c1r1, c2r1, c3r1,
            c0r2, c1r2, c2r2, c3r2,
            c0r3, c1r3, c2r3, c3r3,
        ))
    }

    /// Creates a 4x4 matrix from four column vectors.
    #[inline]
    pub fn from_cols(
        c0: Vector4<S>,
        c1: Vector4<S>,
        c2: Vector4<S>,
        c3: Vector4<S>,
    ) -> Self {
        Matrix4(na::Matrix4::from_columns(&[c0, c1, c2, c3]))
    }

    /// Returns the identity matrix.
    #[inline]
    pub fn identity() -> Self
    where
        S: na::RealField,
    {
        Matrix4(na::Matrix4::identity())
    }

    /// Creates a diagonal matrix.
    #[inline]
    pub fn from_value(value: S) -> Self
    where
        S: na::RealField,
    {
        Matrix4(na::Matrix4::identity() * value)
    }

    /// Creates a right-handed look-at view matrix.
    #[inline]
    pub fn look_at_rh(eye: Point3<S>, center: Point3<S>, up: Vector3<S>) -> Self
    where
        S: na::RealField,
    {
        let m = na::Isometry3::look_at_rh(&eye, &center, &up);
        Matrix4(m.to_homogeneous())
    }

    /// Creates a Matrix4 from a rotation angle around the Y axis.
    #[inline]
    pub fn from_angle_y(angle: Rad<S>) -> Self
    where
        S: na::RealField,
    {
        let rot = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), angle.0);
        Matrix4(rot.to_homogeneous())
    }

    /// Creates a Matrix4 from a rotation angle around the X axis.
    #[inline]
    pub fn from_angle_x(angle: Rad<S>) -> Self
    where
        S: na::RealField,
    {
        let rot = na::Rotation3::from_axis_angle(&na::Vector3::x_axis(), angle.0);
        Matrix4(rot.to_homogeneous())
    }

    /// Creates a Matrix4 from a rotation angle around the Z axis.
    #[inline]
    pub fn from_angle_z(angle: Rad<S>) -> Self
    where
        S: na::RealField,
    {
        let rot = na::Rotation3::from_axis_angle(&na::Vector3::z_axis(), angle.0);
        Matrix4(rot.to_homogeneous())
    }

    /// Creates a non-uniform scaling matrix.
    #[inline]
    pub fn from_nonuniform_scale(x: S, y: S, z: S) -> Self
    where
        S: na::RealField,
    {
        let zero = S::zero();
        let one = S::one();
        Matrix4::new(
            x, zero, zero, zero,
            zero, y, zero, zero,
            zero, zero, z, zero,
            zero, zero, zero, one,
        )
    }

    /// Creates a translation matrix.
    #[inline]
    pub fn from_translation(v: Vector3<S>) -> Self
    where
        S: na::RealField,
    {
        let zero = S::zero();
        let one = S::one();
        Matrix4::new(
            one, zero, zero, v[0],
            zero, one, zero, v[1],
            zero, zero, one, v[2],
            zero, zero, zero, one,
        )
    }
}

// ---- Eq for matrices (when the scalar supports it) ----

impl<S: na::Scalar + Eq> Eq for Matrix2<S> {}
impl<S: na::Scalar + Eq> Eq for Matrix3<S> {}
impl<S: na::Scalar + Eq> Eq for Matrix4<S> {}

// ---- Index access (row, col) ----

macro_rules! impl_matrix_index {
    ($mtype:ident) => {
        impl<S: na::Scalar> std::ops::Index<(usize, usize)> for $mtype<S> {
            type Output = S;
            #[inline(always)]
            fn index(&self, (row, col): (usize, usize)) -> &S {
                &self.0[(row, col)]
            }
        }
        impl<S: na::Scalar> std::ops::IndexMut<(usize, usize)> for $mtype<S> {
            #[inline(always)]
            fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut S {
                &mut self.0[(row, col)]
            }
        }
    };
}

impl_matrix_index!(Matrix2);
impl_matrix_index!(Matrix3);
impl_matrix_index!(Matrix4);

// ---- Arithmetic operator delegation ----

macro_rules! impl_matrix_ops {
    ($mtype:ident, $na_type:ty, $vtype:ty) => {
        // Matrix + Matrix
        impl<S: na::Scalar + na::ClosedAddAssign> std::ops::Add for $mtype<S> {
            type Output = Self;
            #[inline]
            fn add(self, rhs: Self) -> Self {
                $mtype(self.0 + rhs.0)
            }
        }
        // Matrix - Matrix
        impl<S: na::Scalar + na::ClosedSubAssign> std::ops::Sub for $mtype<S> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                $mtype(self.0 - rhs.0)
            }
        }
        // Matrix += Matrix
        impl<S: na::Scalar + na::ClosedAddAssign> std::ops::AddAssign for $mtype<S> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }
        // Matrix -= Matrix
        impl<S: na::Scalar + na::ClosedSubAssign> std::ops::SubAssign for $mtype<S> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }
        // Matrix * scalar
        impl<S: na::Scalar + na::ClosedMulAssign> std::ops::Mul<S> for $mtype<S> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: S) -> Self {
                $mtype(self.0 * rhs)
            }
        }
        // Matrix *= scalar
        impl<S: na::Scalar + na::ClosedMulAssign> std::ops::MulAssign<S> for $mtype<S> {
            #[inline]
            fn mul_assign(&mut self, rhs: S) {
                self.0 *= rhs;
            }
        }
        // Matrix / scalar
        impl<S: na::Scalar + na::ClosedDivAssign> std::ops::Div<S> for $mtype<S> {
            type Output = Self;
            #[inline]
            fn div(self, rhs: S) -> Self {
                $mtype(self.0 / rhs)
            }
        }
        // Matrix /= scalar
        impl<S: na::Scalar + na::ClosedDivAssign> std::ops::DivAssign<S> for $mtype<S> {
            #[inline]
            fn div_assign(&mut self, rhs: S) {
                self.0 /= rhs;
            }
        }
        // Matrix * vector
        impl<S: na::RealField> std::ops::Mul<$vtype> for $mtype<S> {
            type Output = $vtype;
            #[inline]
            fn mul(self, rhs: $vtype) -> $vtype {
                self.0 * rhs
            }
        }
        // Matrix * Matrix
        impl<S: na::RealField> std::ops::Mul<$mtype<S>> for $mtype<S> {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                $mtype(self.0 * rhs.0)
            }
        }
        // Neg
        impl<S: na::Scalar + std::ops::Neg<Output = S> + Copy> std::ops::Neg for $mtype<S> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                $mtype(-self.0)
            }
        }
        // Default (zero matrix)
        impl<S: na::Scalar + num_traits::Zero> Default for $mtype<S> {
            #[inline]
            fn default() -> Self {
                $mtype(<$na_type>::zeros())
            }
        }
        // Zero
        impl<S: na::Scalar + na::ClosedAddAssign + num_traits::Zero> num_traits::Zero
            for $mtype<S>
        {
            #[inline]
            fn zero() -> Self {
                $mtype(<$na_type>::zeros())
            }
            #[inline]
            fn is_zero(&self) -> bool {
                self.0.iter().all(num_traits::Zero::is_zero)
            }
        }
        // One
        impl<S: na::RealField> num_traits::One for $mtype<S>
        {
            #[inline]
            fn one() -> Self {
                $mtype(<$na_type>::identity())
            }
        }
        // AbsDiffEq
        impl<S: na::Scalar + approx::AbsDiffEq> approx::AbsDiffEq for $mtype<S>
        where
            S::Epsilon: Copy,
        {
            type Epsilon = S::Epsilon;
            fn default_epsilon() -> S::Epsilon {
                S::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
                self.0.abs_diff_eq(&other.0, epsilon)
            }
        }
    };
}

impl_matrix_ops!(Matrix2, na::Matrix2<S>, Vector2<S>);
impl_matrix_ops!(Matrix3, na::Matrix3<S>, Vector3<S>);
impl_matrix_ops!(Matrix4, na::Matrix4<S>, Vector4<S>);

// Matrix4 * Point3 (Transform).
impl<S: na::RealField> std::ops::Mul<Point3<S>> for Matrix4<S> {
    type Output = Point3<S>;
    #[inline]
    fn mul(self, rhs: Point3<S>) -> Point3<S> {
        let h = self.0 * rhs.to_homogeneous();
        let w = h[3].clone();
        Point3::new(h[0].clone() / w.clone(), h[1].clone() / w.clone(), h[2].clone() / w)
    }
}

// ---- Angle wrapper types ----

/// Radians wrapper (cgmath-compatible).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rad<S>(
    /// The angle value in radians.
    pub S,
);

/// Degrees wrapper (cgmath-compatible).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Deg<S>(
    /// The angle value in degrees.
    pub S,
);

impl<S: num_traits::Float + num_traits::FloatConst> From<Rad<S>> for Deg<S> {
    fn from(r: Rad<S>) -> Self {
        Deg(r.0 * (S::from(180.0).unwrap() / S::PI()))
    }
}

impl<S: num_traits::Float + num_traits::FloatConst> From<Deg<S>> for Rad<S> {
    fn from(d: Deg<S>) -> Self {
        Rad(d.0 * (S::PI() / S::from(180.0).unwrap()))
    }
}

impl<S: std::ops::Neg<Output = S>> std::ops::Neg for Rad<S> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Rad(-self.0)
    }
}

impl<S: std::ops::Neg<Output = S>> std::ops::Neg for Deg<S> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Deg(-self.0)
    }
}

impl<S: std::ops::Add<Output = S>> std::ops::Add for Rad<S> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Rad(self.0 + rhs.0)
    }
}

impl<S: std::ops::Sub<Output = S>> std::ops::Sub for Rad<S> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Rad(self.0 - rhs.0)
    }
}

impl<S: std::ops::Mul<Output = S>> std::ops::Mul<S> for Rad<S> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self::Output {
        Rad(self.0 * rhs)
    }
}

impl<S: std::ops::Div<Output = S>> std::ops::Div<S> for Rad<S> {
    type Output = Self;
    fn div(self, rhs: S) -> Self::Output {
        Rad(self.0 / rhs)
    }
}
