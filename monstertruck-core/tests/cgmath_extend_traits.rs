use monstertruck_core::cgmath64::*;

// ---- Homogeneous trait ----

#[test]
fn vector2_homogeneous_truncate() {
    let v = Vector2::new(3.0, 2.0);
    assert_eq!(Homogeneous::truncate(v), Vector1::new(3.0));
}

#[test]
fn vector2_homogeneous_weight() {
    let v = Vector2::new(3.0, 2.0);
    assert_eq!(Homogeneous::weight(v), 2.0);
}

#[test]
fn vector2_from_point() {
    let p = Point1::new(5.0);
    assert_eq!(Vector2::from_point(p), Vector2::new(5.0, 1.0));
}

#[test]
fn vector2_to_point() {
    let v = Vector2::new(6.0, 2.0);
    assert_eq!(Homogeneous::to_point(v), Point1::new(3.0));
}

#[test]
fn vector2_from_point_weight() {
    let p = Point1::new(4.0);
    let v = Vector2::from_point_weight(p, 3.0);
    assert_eq!(v, Vector2::new(12.0, 3.0));
}

#[test]
fn vector3_homogeneous_truncate() {
    let v = Vector3::new(8.0, 6.0, 4.0);
    assert_eq!(Homogeneous::truncate(v), Vector2::new(8.0, 6.0));
}

#[test]
fn vector3_homogeneous_weight() {
    let v = Vector3::new(8.0, 6.0, 4.0);
    assert_eq!(Homogeneous::weight(v), 4.0);
}

#[test]
fn vector3_from_point() {
    let p = Point2::new(3.0, 7.0);
    assert_eq!(Vector3::from_point(p), Vector3::new(3.0, 7.0, 1.0));
}

#[test]
fn vector3_to_point() {
    let v = Vector3::new(6.0, 8.0, 2.0);
    assert_eq!(Homogeneous::to_point(v), Point2::new(3.0, 4.0));
}

#[test]
fn vector4_homogeneous_truncate() {
    let v = Vector4::new(8.0, 6.0, 4.0, 2.0);
    assert_eq!(Homogeneous::truncate(v), Vector3::new(8.0, 6.0, 4.0));
}

#[test]
fn vector4_homogeneous_weight() {
    let v = Vector4::new(8.0, 6.0, 4.0, 2.0);
    assert_eq!(Homogeneous::weight(v), 2.0);
}

#[test]
fn vector4_to_point() {
    let v = Vector4::new(8.0, 6.0, 4.0, 2.0);
    assert_eq!(Homogeneous::to_point(v), Point3::new(4.0, 3.0, 2.0));
}

#[test]
fn vector4_from_point() {
    assert_eq!(
        Vector4::from_point(Point3::new(4.0, 3.0, 2.0)),
        Vector4::new(4.0, 3.0, 2.0, 1.0),
    );
}

#[test]
fn vector4_from_point_weight() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let v = Vector4::from_point_weight(p, 5.0);
    assert_eq!(v, Vector4::new(5.0, 10.0, 15.0, 5.0));
}

// ---- rat_der function ----

#[test]
fn rat_der_first_order() {
    let t = 1.5;
    let pt = Vector4::new(t * t, t * t * t, t * t * t * t, t);
    let der = Vector4::new(2.0 * t, 3.0 * t * t, 4.0 * t * t * t, 1.0);
    let result = rat_der(&[pt, der]);
    let expected = Vector3::new(1.0, 2.0 * t, 3.0 * t * t);
    monstertruck_core::assert_near!(result, expected);
}

#[test]
fn rat_der_second_order() {
    let t = 1.5;
    let pt = Vector4::new(t * t, t * t * t, t * t * t * t, t);
    let der = Vector4::new(2.0 * t, 3.0 * t * t, 4.0 * t * t * t, 1.0);
    let der2 = Vector4::new(2.0, 6.0 * t, 12.0 * t * t, 0.0);
    let result = rat_der(&[pt, der, der2]);
    let expected = Vector3::new(0.0, 2.0, 6.0 * t);
    monstertruck_core::assert_near!(result, expected);
}

#[test]
fn rat_der_third_order() {
    let t = 1.5;
    let pt = Vector4::new(t * t, t * t * t, t * t * t * t, t);
    let der = Vector4::new(2.0 * t, 3.0 * t * t, 4.0 * t * t * t, 1.0);
    let der2 = Vector4::new(2.0, 6.0 * t, 12.0 * t * t, 0.0);
    let der3 = Vector4::new(0.0, 6.0, 24.0 * t, 0.0);
    let result = rat_der(&[pt, der, der2, der3]);
    let expected = Vector3::new(0.0, 0.0, 6.0);
    monstertruck_core::assert_near!(result, expected);
}

// ---- rat_ders function ----

#[test]
fn rat_ders_multi_order() {
    let t = 1.5;
    let ders = [
        Vector4::new(t * t, t * t * t, t * t * t * t, t),
        Vector4::new(2.0 * t, 3.0 * t * t, 4.0 * t * t * t, 1.0),
        Vector4::new(2.0, 6.0 * t, 12.0 * t * t, 0.0),
        Vector4::new(0.0, 6.0, 24.0 * t, 0.0),
    ];
    let mut evals = [Vector3::zero(); 4];
    rat_ders(&ders, &mut evals);

    let ans = [
        Vector3::new(t, t * t, t * t * t),
        Vector3::new(1.0, 2.0 * t, 3.0 * t * t),
        Vector3::new(0.0, 2.0, 6.0 * t),
        Vector3::new(0.0, 0.0, 6.0),
    ];
    assert_eq!(evals, ans);
}

// ---- abs_ders function ----

#[test]
fn abs_ders_magnitude_derivatives() {
    let t = 0.3;
    // c(t) = (2t, 1 - t^2) -> |c(t)| = 1 + t^2.
    let ders = [
        Vector2::new(2.0 * t, 1.0 - t * t),
        Vector2::new(2.0, -2.0 * t),
        Vector2::new(0.0, -2.0),
        Vector2::new(0.0, 0.0),
    ];
    let mut evals = [0.0; 4];
    abs_ders(&ders, &mut evals);

    monstertruck_core::assert_near!(evals[0], 1.0 + t * t);
    monstertruck_core::assert_near!(evals[1], 2.0 * t);
    monstertruck_core::assert_near!(evals[2], 2.0);
    monstertruck_core::assert_near!(evals[3], 0.0);
}

// ---- ControlPoint trait ----

#[test]
fn point1_control_point() {
    use monstertruck_core::cgmath_extend_traits::control_point::ControlPoint;
    let origin = <Point1 as ControlPoint<f64>>::origin();
    assert_eq!(origin, Point1::new(0.0));
    let v = <Point1 as ControlPoint<f64>>::to_vec(Point1::new(3.0));
    assert_eq!(v, Vector1::new(3.0));
    let p = <Point1 as ControlPoint<f64>>::from_vec(Vector1::new(5.0));
    assert_eq!(p, Point1::new(5.0));
}

#[test]
fn point2_control_point() {
    use monstertruck_core::cgmath_extend_traits::control_point::ControlPoint;
    let origin = <Point2 as ControlPoint<f64>>::origin();
    assert_eq!(origin, Point2::new(0.0, 0.0));
    let v = <Point2 as ControlPoint<f64>>::to_vec(Point2::new(1.0, 2.0));
    assert_eq!(v, Vector2::new(1.0, 2.0));
    let p = <Point2 as ControlPoint<f64>>::from_vec(Vector2::new(3.0, 4.0));
    assert_eq!(p, Point2::new(3.0, 4.0));
}

#[test]
fn point3_control_point() {
    use monstertruck_core::cgmath_extend_traits::control_point::ControlPoint;
    let origin = <Point3 as ControlPoint<f64>>::origin();
    assert_eq!(origin, Point3::new(0.0, 0.0, 0.0));
}

#[test]
fn vector2_control_point() {
    use monstertruck_core::cgmath_extend_traits::control_point::ControlPoint;
    let origin = <Vector2 as ControlPoint<f64>>::origin();
    assert_eq!(origin, Vector2::new(0.0, 0.0));
    let v = Vector2::new(1.0, 2.0);
    assert_eq!(<Vector2 as ControlPoint<f64>>::to_vec(v), v);
    assert_eq!(<Vector2 as ControlPoint<f64>>::from_vec(v), v);
}

#[test]
fn vector3_control_point() {
    use monstertruck_core::cgmath_extend_traits::control_point::ControlPoint;
    let origin = <Vector3 as ControlPoint<f64>>::origin();
    assert_eq!(origin, Vector3::new(0.0, 0.0, 0.0));
}

#[test]
fn vector4_control_point() {
    use monstertruck_core::cgmath_extend_traits::control_point::ControlPoint;
    let origin = <Vector4 as ControlPoint<f64>>::origin();
    assert_eq!(origin, Vector4::new(0.0, 0.0, 0.0, 0.0));
}

// ---- multi_rat_der ----

#[test]
fn multi_rat_der_surface() {
    let (u, v) = (1.0, 2.0);
    let ders: [[Vector4; 3]; 3] = [
        [
            Vector4::new(u * u * u * v * v, u * u * v * v * v, u * v, u),
            Vector4::new(2.0 * u * u * u * v, 3.0 * u * u * v * v, u, 0.0),
            Vector4::new(2.0 * u * u * u, 6.0 * u * u * v, 0.0, 0.0),
        ],
        [
            Vector4::new(3.0 * u * u * v * v, 2.0 * u * v * v * v, v, 1.0),
            Vector4::new(6.0 * u * u * v, 6.0 * u * v * v, 1.0, 0.0),
            Vector4::new(6.0 * u * u, 12.0 * u * v, 0.0, 0.0),
        ],
        [
            Vector4::new(6.0 * u * v * v, 2.0 * v * v * v, 0.0, 0.0),
            Vector4::new(12.0 * u * v, 6.0 * v * v, 0.0, 0.0),
            Vector4::new(12.0 * u, 12.0 * v, 0.0, 0.0),
        ],
    ];
    let result = multi_rat_der(&ders);
    monstertruck_core::assert_near!(result, Vector3::new(4.0, 0.0, 0.0));
}
