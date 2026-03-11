//! Compatibility tests verifying the cgmath-like API surface provided by monstertruck-math.

use monstertruck_math::prelude::*;
use monstertruck_math::*;

// ---------- Vector construction and basic operations ----------

#[test]
fn vector3_new() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(v[0], 1.0);
    assert_eq!(v[1], 2.0);
    assert_eq!(v[2], 3.0);
}

#[test]
fn vector3_dot_product() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(4.0, 5.0, 6.0);
    assert_eq!(InnerSpace::dot(a, b), 32.0);
}

#[test]
fn vector3_cross_product() {
    let a = Vector3::new(1.0, 0.0, 0.0);
    let b = Vector3::new(0.0, 1.0, 0.0);
    let c = a.cross(&b);
    assert_eq!(c, Vector3::new(0.0, 0.0, 1.0));
}

#[test]
fn vector3_magnitude() {
    let v: Vector3<f64> = Vector3::new(3.0, 4.0, 0.0);
    assert!((InnerSpace::magnitude(v) - 5.0).abs() < 1e-10);
}

#[test]
fn vector3_normalize() {
    let v: Vector3<f64> = Vector3::new(3.0, 0.0, 0.0);
    let n = InnerSpace::normalize(v);
    assert!((InnerSpace::magnitude(n) - 1.0).abs() < 1e-10);
    assert!((n[0] - 1.0).abs() < 1e-10);
}

// ---------- Point operations ----------

#[test]
fn point3_origin() {
    let p: Point3<f64> = EuclideanSpace::origin();
    assert_eq!(p[0], 0.0);
    assert_eq!(p[1], 0.0);
    assert_eq!(p[2], 0.0);
}

#[test]
fn point3_to_vec_from_vec() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let v: Vector3<f64> = EuclideanSpace::to_vec(p);
    assert_eq!(v, Vector3::new(1.0, 2.0, 3.0));
    let p2: Point3<f64> = EuclideanSpace::from_vec(v);
    assert_eq!(p2, p);
}

// ---------- Index/IndexMut for vectors and points ----------

#[test]
fn vector_index_access() {
    let mut v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(v[0], 1.0);
    v[1] = 5.0;
    assert_eq!(v[1], 5.0);
}

#[test]
fn point_index_access() {
    let mut p = Point3::new(1.0, 2.0, 3.0);
    assert_eq!(p[0], 1.0);
    p[2] = 7.0;
    assert_eq!(p[2], 7.0);
}

// ---------- Rad/Deg conversions ----------

#[test]
fn rad_deg_conversion() {
    let r = Rad(std::f64::consts::PI);
    let d: Deg<f64> = r.into();
    assert!((d.0 - 180.0).abs() < 1e-10);

    let d2 = Deg(90.0);
    let r2: Rad<f64> = d2.into();
    assert!((r2.0 - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
}

// ---------- Zero and VectorSpace ----------

#[test]
fn vector_zero() {
    let z: Vector3<f64> = Zero::zero();
    assert_eq!(z, Vector3::new(0.0, 0.0, 0.0));
}

#[test]
fn vector_scalar_mul_div() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let scaled = v * 2.0;
    assert_eq!(scaled, Vector3::new(2.0, 4.0, 6.0));
    let halved = v / 2.0;
    assert_eq!(halved, Vector3::new(0.5, 1.0, 1.5));
}

// ---------- MetricSpace ----------

#[test]
fn metric_space_distance() {
    let a: Point3<f64> = Point3::new(0.0, 0.0, 0.0);
    let b: Point3<f64> = Point3::new(3.0, 4.0, 0.0);
    assert!((MetricSpace::distance(a, b) - 5.0).abs() < 1e-10);
}

// ---------- From array conversions ----------

#[test]
fn vector_from_array() {
    let v = Vector3::from([1.0, 2.0, 3.0]);
    assert_eq!(v, Vector3::new(1.0, 2.0, 3.0));
}

// ---------- Truncate / extend / to_homogeneous ----------

#[test]
fn vector4_truncate() {
    let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let t: Vector3<f64> = v.truncate();
    assert_eq!(t, Vector3::new(1.0, 2.0, 3.0));
}

#[test]
fn vector3_truncate() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let t: Vector2<f64> = v.truncate();
    assert_eq!(t, Vector2::new(1.0, 2.0));
}

#[test]
fn point3_to_homogeneous() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let h: Vector4<f64> = p.to_homogeneous();
    assert_eq!(h, Vector4::new(1.0, 2.0, 3.0, 1.0));
}

// ---------- Serde round-trip ----------

#[test]
fn serde_vector3_roundtrip() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let json = serde_json::to_string(&v).unwrap();
    let v2: Vector3<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(v, v2);
}

#[test]
fn serde_point3_roundtrip() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let json = serde_json::to_string(&p).unwrap();
    let p2: Point3<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(p, p2);
}

// ---------- SquareMatrix invert ----------

#[test]
fn matrix2_invert() {
    // cgmath column-major convention: new(c0r0, c0r1, c1r0, c1r1)
    // This creates matrix: | 1  2 |
    //                      | 3  4 |
    // Wait, cgmath: col0=[c0r0, c0r1]=[1,2], col1=[c1r0, c1r1]=[3,4]
    // So matrix is: | 1  3 |  (row0: c0r0, c1r0)
    //               | 2  4 |  (row1: c0r1, c1r1)
    let m: Matrix2<f64> = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    // Verify column-major construction: m[(row,col)]
    assert_eq!(m[(0, 0)], 1.0); // c0r0
    assert_eq!(m[(1, 0)], 2.0); // c0r1
    assert_eq!(m[(0, 1)], 3.0); // c1r0
    assert_eq!(m[(1, 1)], 4.0); // c1r1

    let inv = SquareMatrix::invert(&m).unwrap();
    let prod = m * inv;
    let id: Matrix2<f64> = Matrix2::identity();
    for i in 0..2 {
        for j in 0..2 {
            assert!(
                (prod[(i, j)] - id[(i, j)]).abs() < 1e-10,
                "prod[({i},{j})] = {}, id = {}",
                prod[(i, j)],
                id[(i, j)]
            );
        }
    }
}

// ---------- BaseFloat ----------

#[test]
fn base_float_basics() {
    fn check<S: BaseFloat>() {
        let _ = S::infinity();
        let _ = S::neg_infinity();
        let _ = S::one();
        let _ = S::zero();
    }
    check::<f32>();
    check::<f64>();
}

// ---------- mul_element_wise ----------

#[test]
fn vector_mul_element_wise() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(4.0, 5.0, 6.0);
    let c = a.mul_element_wise(b);
    assert_eq!(c, Vector3::new(4.0, 10.0, 18.0));
}

// ---------- cast ----------

#[test]
fn vector_cast() {
    let v: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
    // nalgebra's native cast() returns the value directly (not Option)
    let v32: Vector3<f32> = v.cast();
    assert_eq!(v32, Vector3::new(1.0f32, 2.0f32, 3.0f32));
    // Also test our Cast trait (returns Option)
    let v32b: Vector3<f32> = Cast::<f32>::cast(v).unwrap();
    assert_eq!(v32b, Vector3::new(1.0f32, 2.0f32, 3.0f32));
}

// ---------- Named fields (x, y, z) ----------

#[test]
fn vector2_named_fields() {
    let v = Vector2::new(1.0, 2.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
}

#[test]
fn vector3_named_fields() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

// ---------- AbsDiffEq ----------

#[test]
fn abs_diff_eq_for_vectors() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(1.0 + 1e-12, 2.0, 3.0);
    assert!(a.abs_diff_eq(&b, 1e-10));
}

// ---------- Matrix from_value ----------

#[test]
fn matrix_from_value() {
    let m: Matrix2<f64> = Matrix2::from_value(5.0);
    // from_value fills diagonal with the value and zeros elsewhere.
    assert_eq!(m[(0, 0)], 5.0);
    assert_eq!(m[(1, 1)], 5.0);
    assert_eq!(m[(0, 1)], 0.0);
    assert_eq!(m[(1, 0)], 0.0);
}

// ---------- Matrix from_angle ----------

#[test]
fn matrix2_from_angle() {
    let m = Matrix2::from_angle(Rad(std::f64::consts::FRAC_PI_2));
    // 90-degree rotation: cos=0, sin=1.
    // cgmath convention: col0=[cos, sin], col1=[-sin, cos].
    assert!((m[(0, 0)] - 0.0).abs() < 1e-10); // cos
    assert!((m[(1, 0)] - 1.0).abs() < 1e-10); // sin
    assert!((m[(0, 1)] - (-1.0)).abs() < 1e-10); // -sin
    assert!((m[(1, 1)] - 0.0).abs() < 1e-10); // cos
}

// ---------- Matrix multiplication ----------

#[test]
fn matrix_vector_mul() {
    // Identity * v = v.
    let id = Matrix2::identity();
    let v = Vector2::new(3.0, 7.0);
    let result = id * v;
    assert_eq!(result, v);
}

#[test]
fn matrix_matrix_mul() {
    let a = Matrix2::identity();
    let b = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    let c = a * b;
    assert_eq!(c, b);
}

// ---------- Serde for matrices ----------

#[test]
fn serde_matrix2_roundtrip() {
    let m = Matrix2::new(1.0, 2.0, 3.0, 4.0);
    let json = serde_json::to_string(&m).unwrap();
    let m2: Matrix2<f64> = serde_json::from_str(&json).unwrap();
    assert_eq!(m, m2);
}
