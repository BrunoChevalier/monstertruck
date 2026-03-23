use monstertruck_core::assert_near;
use monstertruck_core::assert_near2;
use monstertruck_core::cgmath64::*;
use monstertruck_traits::*;

// -- Invertible for (usize, usize) --

#[test]
fn usize_tuple_invert_swaps_elements() {
    let mut pair = (1usize, 2usize);
    pair.invert();
    assert_eq!(pair, (2, 1));
}

#[test]
fn usize_tuple_inverse_returns_swapped() {
    let pair = (3usize, 7usize);
    let inv = pair.inverse();
    assert_eq!(inv, (7, 3));
    // Original is unchanged.
    assert_eq!(pair, (3, 7));
}

// -- Invertible for Vec<P> --

#[test]
fn vec_invert_reverses() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.invert();
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
}

#[test]
fn vec_inverse_returns_reversed() {
    let v = vec![10, 20, 30];
    let inv = v.inverse();
    assert_eq!(inv, vec![30, 20, 10]);
    // Original unchanged.
    assert_eq!(v, vec![10, 20, 30]);
}

// -- Invertible for Box<T> --

#[test]
fn box_invert_delegates() {
    let mut b = Box::new((1usize, 2usize));
    b.invert();
    assert_eq!(*b, (2, 1));
}

#[test]
fn box_inverse_delegates() {
    let b = Box::new((5usize, 10usize));
    let inv = b.inverse();
    assert_eq!(*inv, (10, 5));
    // Original unchanged.
    assert_eq!(*b, (5, 10));
}

// -- Transformed for Point3 with Matrix4 --

#[test]
fn point3_transform_by_matrix4() {
    // Translation matrix: translate by (1, 2, 3).
    let mat = Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0));
    let mut p = Point3::new(0.0, 0.0, 0.0);
    p.transform_by(mat);
    assert_near!(p, Point3::new(1.0, 2.0, 3.0));
}

#[test]
fn point3_transformed_by_matrix4() {
    let mat = Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0));
    let p = Point3::new(10.0, 20.0, 30.0);
    let q = p.transformed(mat);
    assert_near!(q, Point3::new(11.0, 22.0, 33.0));
    // Original unchanged.
    assert_near!(p, Point3::new(10.0, 20.0, 30.0));
}

// -- Transformed for Point3 with Matrix3 --

#[test]
fn point3_transform_by_matrix3() {
    // Scale x by 2, keep y and z.
    let mat = Matrix3::from_nonuniform_scale(2.0, 1.0);
    let p = Point3::new(5.0, 10.0, 15.0);
    let q = p.transformed(mat);
    assert_near!(q, Point3::new(10.0, 10.0, 15.0));
}

// -- Transformed for Point2 with Matrix3 --

#[test]
fn point2_transform_by_matrix3() {
    // Scale x by 3, y by 2.
    let mat = Matrix3::from_nonuniform_scale(3.0, 2.0);
    let p = Point2::new(1.0, 1.0);
    let q = p.transformed(mat);
    assert_near2!(q, Point2::new(3.0, 2.0));
}

// -- Transformed for Box<Point3> --

#[test]
fn box_point3_transform_by_delegates() {
    let mat = Matrix4::from_translation(Vector3::new(1.0, 0.0, 0.0));
    let mut b = Box::new(Point3::new(0.0, 0.0, 0.0));
    b.transform_by(mat);
    assert_near!(*b, Point3::new(1.0, 0.0, 0.0));
}

#[test]
fn box_point3_transformed_delegates() {
    let mat = Matrix4::from_translation(Vector3::new(0.0, 5.0, 0.0));
    let b = Box::new(Point3::new(1.0, 2.0, 3.0));
    let result = b.transformed(mat);
    assert_near!(*result, Point3::new(1.0, 7.0, 3.0));
}

// -- ToSameGeometry trait existence (compile-time check) --

#[test]
fn to_same_geometry_trait_exists() {
    // Compile-time check: the trait is importable and can be referenced.
    fn _assert_trait_exists<T: ToSameGeometry<T>>() {}
}
