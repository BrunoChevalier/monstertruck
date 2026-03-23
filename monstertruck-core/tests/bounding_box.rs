#![allow(clippy::op_ref)]

use monstertruck_core::bounding_box::*;
use monstertruck_core::cgmath64::*;

// ---- Construction and emptiness ----

#[test]
fn new_creates_empty() {
    let bb = BoundingBox::<Point3>::new();
    assert!(bb.is_empty());
}

#[test]
fn default_is_empty() {
    let bb = BoundingBox::<Vector2>::default();
    assert!(bb.is_empty());
}

#[test]
fn not_empty_after_single_push() {
    let mut bb = BoundingBox::new();
    bb.push(Vector2::new(1.0, 2.0));
    assert!(!bb.is_empty());
}

#[test]
fn from_iter_refs() {
    let pts = [Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)];
    let bb = BoundingBox::from_iter(&pts);
    assert_eq!(bb.min(), Point2::new(0.0, 0.0));
    assert_eq!(bb.max(), Point2::new(1.0, 1.0));
}

#[test]
fn from_iter_owned() {
    let pts = vec![Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0)];
    let bb: BoundingBox<Vector3> = pts.into_iter().collect();
    assert_eq!(bb.min(), Vector3::new(1.0, 2.0, 3.0));
    assert_eq!(bb.max(), Vector3::new(4.0, 5.0, 6.0));
}

// ---- Push and NaN handling ----

#[test]
fn push_multiple_points() {
    let mut bb = BoundingBox::new();
    bb.push(Vector2::new(3.0, -1.0));
    bb.push(Vector2::new(-2.0, 5.0));
    bb.push(Vector2::new(1.0, 2.0));
    assert_eq!(bb.min(), Vector2::new(-2.0, -1.0));
    assert_eq!(bb.max(), Vector2::new(3.0, 5.0));
}

#[test]
fn push_nan_component_ignored() {
    // NaN components are individually ignored by the comparison-based min/max.
    // Non-NaN components of the same point can still update the bounding box.
    let mut bb = BoundingBox::new();
    bb.push(Vector2::new(1.0, 2.0));
    bb.push(Vector2::new(f64::NAN, 3.0));
    // x = NAN is ignored (comparison with NaN is false), but y = 3.0 updates max.y.
    assert_eq!(bb.min(), Vector2::new(1.0, 2.0));
    assert_eq!(bb.max(), Vector2::new(1.0, 3.0));

    // All-NaN point: nothing changes.
    let mut bb2 = BoundingBox::new();
    bb2.push(Vector2::new(1.0, 2.0));
    bb2.push(Vector2::new(f64::NAN, f64::NAN));
    assert_eq!(bb2.min(), Vector2::new(1.0, 2.0));
    assert_eq!(bb2.max(), Vector2::new(1.0, 2.0));
}

#[test]
fn push_all_nan_then_valid() {
    let mut bb = BoundingBox::new();
    bb.push(Vector2::new(f64::NAN, f64::NAN));
    bb.push(Vector2::new(5.0, 6.0));
    assert_eq!(bb.min(), Vector2::new(5.0, 6.0));
    assert_eq!(bb.max(), Vector2::new(5.0, 6.0));
}

// ---- Geometric properties ----

#[test]
fn diagonal_known_box() {
    let bb = BoundingBox::from_iter(&[Vector2::new(-2.0, -3.0), Vector2::new(6.0, 4.0)]);
    assert_eq!(bb.diagonal(), Vector2::new(8.0, 7.0));
}

#[test]
fn diameter_2d() {
    let bb = BoundingBox::from_iter(&[Vector2::new(-1.0, -3.0), Vector2::new(2.0, 1.0)]);
    assert_eq!(bb.diameter(), 5.0);
}

#[test]
fn diameter_3d() {
    let bb = BoundingBox::from_iter(&[Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 2.0, 2.0)]);
    assert_eq!(bb.diameter(), 3.0);
}

#[test]
fn size_3d() {
    let bb = BoundingBox::from_iter(&[Vector3::new(-1.0, -3.0, 2.0), Vector3::new(2.0, 1.0, 10.0)]);
    assert_eq!(bb.size(), 8.0);
}

#[test]
fn center_known_box() {
    let bb = BoundingBox::from_iter(&[Vector2::new(-1.0, -3.0), Vector2::new(5.0, 1.0)]);
    assert_eq!(bb.center(), Vector2::new(2.0, -1.0));
}

#[test]
fn center_empty_is_nan() {
    let bb = BoundingBox::<Vector3>::new();
    let c = bb.center();
    assert!(c[0].is_nan());
    assert!(c[1].is_nan());
    assert!(c[2].is_nan());
}

#[test]
fn diagonal_empty_is_neg_infinity() {
    let bb = BoundingBox::<Vector2>::new();
    assert_eq!(
        bb.diagonal(),
        Vector2::new(f64::NEG_INFINITY, f64::NEG_INFINITY)
    );
}

#[test]
fn diameter_empty_is_neg_infinity() {
    let bb = BoundingBox::<Vector3>::new();
    assert_eq!(bb.diameter(), f64::NEG_INFINITY);
}

#[test]
fn size_empty_is_neg_infinity() {
    let bb = BoundingBox::<Vector3>::new();
    assert_eq!(bb.size(), f64::NEG_INFINITY);
}

// ---- Containment ----

#[test]
fn contains_interior_point() {
    let bb = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)]);
    assert!(bb.contains(Point2::new(0.5, 0.5)));
}

#[test]
fn contains_boundary_point() {
    let bb = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)]);
    assert!(bb.contains(Point2::new(0.0, 0.5)));
    assert!(bb.contains(Point2::new(1.0, 1.0)));
}

#[test]
fn does_not_contain_exterior_point() {
    let bb = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)]);
    assert!(!bb.contains(Point2::new(-0.1, 0.5)));
    assert!(!bb.contains(Point2::new(0.5, 1.1)));
}

// ---- Union operators ----

#[test]
fn add_assign_ref() {
    let mut bb = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0)]);
    let other = BoundingBox::from_iter(&[Vector2::new(2.0, 2.0), Vector2::new(3.0, 3.0)]);
    bb += &other;
    assert_eq!(bb.min(), Vector2::new(0.0, 0.0));
    assert_eq!(bb.max(), Vector2::new(3.0, 3.0));
}

#[test]
fn add_assign_owned() {
    let mut bb = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0)]);
    bb += BoundingBox::from_iter(&[Vector2::new(-1.0, -1.0)]);
    assert_eq!(bb.min(), Vector2::new(-1.0, -1.0));
}

#[test]
fn add_ref_ref() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0)]);
    let c = &a + &b;
    assert_eq!(c.min(), Vector2::new(0.0, 0.0));
    assert_eq!(c.max(), Vector2::new(1.0, 1.0));
}

#[test]
fn add_owned_ref() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0)]);
    let c = a + &b;
    assert_eq!(c.max(), Vector2::new(1.0, 1.0));
}

#[test]
fn add_ref_owned() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0)]);
    let c = &a + b;
    assert_eq!(c.max(), Vector2::new(1.0, 1.0));
}

#[test]
fn add_owned_owned() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0)]);
    let c = a + b;
    assert_eq!(c.max(), Vector2::new(1.0, 1.0));
}

#[test]
fn union_with_empty_preserves() {
    let bb = BoundingBox::from_iter(&[Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)]);
    let result = bb + BoundingBox::new();
    assert_eq!(result.min(), Vector2::new(1.0, 2.0));
    assert_eq!(result.max(), Vector2::new(3.0, 4.0));
}

// ---- Intersection operators ----

#[test]
fn bitxor_assign_ref() {
    let mut bb = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)]);
    let other = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0), Vector2::new(3.0, 3.0)]);
    bb ^= &other;
    assert_eq!(bb.min(), Vector2::new(1.0, 1.0));
    assert_eq!(bb.max(), Vector2::new(2.0, 2.0));
}

#[test]
fn bitxor_assign_owned() {
    let mut bb = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)]);
    bb ^= BoundingBox::from_iter(&[Vector2::new(1.0, 1.0), Vector2::new(3.0, 3.0)]);
    assert_eq!(bb.min(), Vector2::new(1.0, 1.0));
    assert_eq!(bb.max(), Vector2::new(2.0, 2.0));
}

#[test]
fn bitxor_ref_ref() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0), Vector2::new(3.0, 3.0)]);
    let c = &a ^ &b;
    assert_eq!(c.min(), Vector2::new(1.0, 1.0));
    assert_eq!(c.max(), Vector2::new(2.0, 2.0));
}

#[test]
fn bitxor_owned_ref() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0), Vector2::new(3.0, 3.0)]);
    let c = a ^ &b;
    assert_eq!(c.min(), Vector2::new(1.0, 1.0));
}

#[test]
fn bitxor_ref_owned() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0), Vector2::new(3.0, 3.0)]);
    let c = &a ^ b;
    assert_eq!(c.min(), Vector2::new(1.0, 1.0));
}

#[test]
fn bitxor_owned_owned() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(2.0, 2.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(1.0, 1.0), Vector2::new(3.0, 3.0)]);
    let c = a ^ b;
    assert_eq!(c.min(), Vector2::new(1.0, 1.0));
    assert_eq!(c.max(), Vector2::new(2.0, 2.0));
}

#[test]
fn intersection_non_overlapping_is_empty() {
    let a = BoundingBox::from_iter(&[Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0)]);
    let b = BoundingBox::from_iter(&[Vector2::new(2.0, 2.0), Vector2::new(3.0, 3.0)]);
    let c = a ^ b;
    assert!(c.is_empty());
}

// ---- PartialOrd (inclusion) ----

#[test]
fn partial_ord_greater() {
    let outer = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(4.0, 4.0)]);
    let inner = BoundingBox::from_iter(&[Point2::new(1.0, 1.0), Point2::new(3.0, 3.0)]);
    assert!(outer > inner);
}

#[test]
fn partial_ord_less() {
    let outer = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(4.0, 4.0)]);
    let inner = BoundingBox::from_iter(&[Point2::new(1.0, 1.0), Point2::new(3.0, 3.0)]);
    assert!(inner < outer);
}

#[test]
fn partial_ord_equal() {
    let a = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)]);
    let b = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)]);
    assert_eq!(a.partial_cmp(&b), Some(std::cmp::Ordering::Equal));
}

#[test]
fn partial_ord_none_for_overlapping() {
    let a = BoundingBox::from_iter(&[Point2::new(0.0, 0.0), Point2::new(2.0, 2.0)]);
    let b = BoundingBox::from_iter(&[Point2::new(1.0, 1.0), Point2::new(3.0, 3.0)]);
    assert_eq!(a.partial_cmp(&b), None);
}

// ---- Type coverage: Point2, Point3, Vector2, Vector3 ----

#[test]
fn point2_bounding_box() {
    let bb = BoundingBox::from_iter(&[Point2::new(1.0, 2.0), Point2::new(3.0, 4.0)]);
    assert_eq!(bb.diagonal(), Vector2::new(2.0, 2.0));
}

#[test]
fn point3_bounding_box() {
    let bb = BoundingBox::from_iter(&[Point3::new(1.0, 2.0, 3.0), Point3::new(4.0, 5.0, 6.0)]);
    assert_eq!(bb.diagonal(), Vector3::new(3.0, 3.0, 3.0));
    assert_eq!(bb.center(), Point3::new(2.5, 3.5, 4.5));
}

#[test]
fn vector3_bounding_box_diameter() {
    let bb = BoundingBox::from_iter(&[Vector3::new(0.0, 0.0, 0.0), Vector3::new(3.0, 4.0, 0.0)]);
    assert_eq!(bb.diameter(), 5.0);
}
