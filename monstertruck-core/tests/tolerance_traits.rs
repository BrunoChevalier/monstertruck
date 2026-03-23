use monstertruck_core::cgmath64::*;
use monstertruck_core::tolerance::*;

// ---- Tolerance trait on f64 ----

#[test]
fn f64_near_within_tolerance() {
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE * 0.5;
    assert!(a.near(&b));
}

#[test]
fn f64_near_outside_tolerance() {
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE * 2.0;
    assert!(!a.near(&b));
}

#[test]
fn f64_near2_within_tolerance2() {
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE2 * 0.5;
    assert!(a.near2(&b));
}

#[test]
fn f64_near2_outside_tolerance2() {
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE2 * 2.0;
    assert!(!a.near2(&b));
}

#[test]
fn f64_near_boundary_exact_tolerance() {
    // At exactly TOLERANCE, abs_diff_eq uses <=, so should be true.
    let a: f64 = 0.0;
    let b: f64 = TOLERANCE;
    assert!(a.near(&b));
}

// ---- Tolerance trait on Vector2 ----

#[test]
fn vector2_near_within() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(1.0 + TOLERANCE * 0.5, 2.0 - TOLERANCE * 0.5);
    assert!(a.near(&b));
}

#[test]
fn vector2_near_outside() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(1.0 + TOLERANCE * 2.0, 2.0);
    assert!(!a.near(&b));
}

#[test]
fn vector2_near2_within() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(1.0 + TOLERANCE2 * 0.5, 2.0);
    assert!(a.near2(&b));
}

// ---- Tolerance trait on Vector3 ----

#[test]
fn vector3_near_within() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(1.0 + TOLERANCE * 0.5, 2.0, 3.0 - TOLERANCE * 0.5);
    assert!(a.near(&b));
}

#[test]
fn vector3_near_outside() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(1.0 + TOLERANCE * 2.0, 2.0, 3.0);
    assert!(!a.near(&b));
}

#[test]
fn vector3_near2_within() {
    let a = Vector3::new(0.0, 0.0, 0.0);
    let b = Vector3::new(TOLERANCE2 * 0.5, 0.0, 0.0);
    assert!(a.near2(&b));
}

// ---- Tolerance trait on Point3 ----

#[test]
fn point3_near_within() {
    let a = Point3::new(1.0, 2.0, 3.0);
    let b = Point3::new(1.0, 2.0 + TOLERANCE * 0.5, 3.0);
    assert!(a.near(&b));
}

#[test]
fn point3_near_outside() {
    let a = Point3::new(1.0, 2.0, 3.0);
    let b = Point3::new(1.0, 2.0 + TOLERANCE * 2.0, 3.0);
    assert!(!a.near(&b));
}

#[test]
fn point3_near2_within() {
    let a = Point3::new(0.0, 0.0, 0.0);
    let b = Point3::new(TOLERANCE2 * 0.5, 0.0, 0.0);
    assert!(a.near2(&b));
}

// ---- Origin trait ----

#[test]
fn f64_so_small_near_zero() {
    let v: f64 = TOLERANCE * 0.5;
    assert!(v.so_small());
}

#[test]
fn f64_so_small_not_near_zero() {
    let v: f64 = TOLERANCE * 2.0;
    assert!(!v.so_small());
}

#[test]
fn f64_so_small2_near_zero() {
    let v: f64 = TOLERANCE2 * 0.5;
    assert!(v.so_small2());
}

#[test]
fn vector2_zero_so_small() {
    let v = Vector2::new(0.0, 0.0);
    assert!(v.so_small());
}

#[test]
fn vector2_small_so_small() {
    let v = Vector2::new(TOLERANCE * 0.5, TOLERANCE * 0.5);
    assert!(v.so_small());
}

#[test]
fn vector3_zero_so_small() {
    let v = Vector3::new(0.0, 0.0, 0.0);
    assert!(v.so_small());
}

#[test]
fn vector3_not_so_small() {
    let v = Vector3::new(1.0, 0.0, 0.0);
    assert!(!v.so_small());
}

// ---- OperationTolerance ----

#[test]
fn operation_tolerance_new_custom_base() {
    let ot = OperationTolerance::new(1.0e-4);
    assert_eq!(ot.base(), 1.0e-4);
    assert_eq!(ot.accumulated_error(), 0.0);
    assert_eq!(ot.operation_count(), 0);
    assert!(ot.last_operation().is_none());
}

#[test]
fn operation_tolerance_from_global() {
    let ot = OperationTolerance::from_global();
    assert_eq!(ot.base(), TOLERANCE);
    assert_eq!(ot.accumulated_error(), 0.0);
}

#[test]
fn operation_tolerance_chain_5_operations() {
    let ot = OperationTolerance::new(1.0e-4);
    let ot = ot.after_operation("bool", 1.0e-6);
    let ot = ot.after_operation("fillet", 2.0e-6);
    let ot = ot.after_operation("tessellate", 3.0e-6);
    let ot = ot.after_operation("smooth", 1.5e-6);
    let ot = ot.after_operation("export", 0.5e-6);

    assert_eq!(ot.operation_count(), 5);
    let expected_error = 1.0e-6 + 2.0e-6 + 3.0e-6 + 1.5e-6 + 0.5e-6;
    assert!((ot.accumulated_error() - expected_error).abs() < f64::EPSILON);
    assert_eq!(ot.effective_tolerance(), 1.0e-4 + expected_error);
}

#[test]
fn operation_tolerance_within_budget_transitions() {
    let ot = OperationTolerance::new(1.0e-4);
    assert!(ot.within_budget());

    // Accumulate error close to but below base.
    let ot = ot.after_operation("step1", 5.0e-5);
    assert!(ot.within_budget());

    let ot = ot.after_operation("step2", 4.9e-5);
    assert!(ot.within_budget());

    // Push over budget.
    let ot = ot.after_operation("step3", 1.0e-5);
    assert!(!ot.within_budget());
}

#[test]
fn operation_tolerance_within_budget_boundary() {
    // accumulated == base => should return false (not strictly less than).
    let ot = OperationTolerance::new(1.0e-4);
    let ot = ot.after_operation("exact", 1.0e-4);
    assert!(!ot.within_budget());
}

#[test]
fn operation_tolerance_last_operation() {
    let ot = OperationTolerance::new(1.0e-4);
    assert_eq!(ot.last_operation(), None);

    let ot = ot.after_operation("boolean", 1.0e-6);
    assert_eq!(ot.last_operation(), Some("boolean"));

    let ot = ot.after_operation("fillet", 2.0e-6);
    assert_eq!(ot.last_operation(), Some("fillet"));
}

#[test]
fn operation_tolerance_clone_independent() {
    let ot = OperationTolerance::new(1.0e-4);
    let ot = ot.after_operation("step", 1.0e-6);
    let cloned = ot.clone();

    // Mutate via after_operation on the original path.
    let ot2 = ot.after_operation("step2", 2.0e-6);

    // Cloned should still reflect original state.
    assert_eq!(cloned.operation_count(), 1);
    assert_eq!(ot2.operation_count(), 2);
}

// ---- Macro tests ----

#[test]
fn assert_near_macro_succeeds() {
    let a = 1.0_f64;
    let b = 1.0 + TOLERANCE * 0.5;
    monstertruck_core::assert_near!(a, b);
}

#[test]
fn assert_near2_macro_succeeds() {
    let a = 1.0_f64;
    let b = 1.0 + TOLERANCE2 * 0.5;
    monstertruck_core::assert_near2!(a, b);
}
