use monstertruck_core::tolerance::*;

#[test]
fn new_creates_zero_accumulated_error() {
    let op_tol = OperationTolerance::new(1e-6);
    assert_eq!(op_tol.base(), 1e-6);
    assert_eq!(op_tol.accumulated_error(), 0.0);
    assert_eq!(op_tol.operation_count(), 0);
    assert!(op_tol.last_operation().is_none());
}

#[test]
fn after_operation_increments_error_and_count() {
    let op_tol = OperationTolerance::new(1e-6);
    let next = op_tol.after_operation("boolean", 1e-7);
    assert_eq!(next.accumulated_error(), 1e-7);
    assert_eq!(next.operation_count(), 1);
    assert_eq!(next.last_operation(), Some("boolean"));
}

#[test]
fn chained_operations_accumulate_correctly() {
    let op_tol = OperationTolerance::new(1e-6)
        .after_operation("boolean", 1e-7)
        .after_operation("fillet", 2e-7)
        .after_operation("tessellation", 5e-7);
    assert!((op_tol.accumulated_error() - 8e-7).abs() < 1e-15);
    assert_eq!(op_tol.operation_count(), 3);
    assert_eq!(op_tol.last_operation(), Some("tessellation"));
}

#[test]
fn effective_tolerance_is_base_plus_accumulated() {
    let op_tol = OperationTolerance::new(1e-6).after_operation("boolean", 3e-7);
    let expected = 1e-6 + 3e-7;
    assert!((op_tol.effective_tolerance() - expected).abs() < 1e-15);
}

#[test]
fn within_budget_true_when_accumulated_less_than_base() {
    let op_tol = OperationTolerance::new(1e-6).after_operation("boolean", 5e-7);
    assert!(op_tol.within_budget());
}

#[test]
fn within_budget_false_when_accumulated_exceeds_base() {
    let op_tol = OperationTolerance::new(1e-6).after_operation("boolean", 2e-6);
    assert!(!op_tol.within_budget());
}

#[test]
fn from_global_uses_tolerance_constant() {
    let op_tol = OperationTolerance::from_global();
    assert_eq!(op_tol.base(), TOLERANCE);
    assert_eq!(op_tol.accumulated_error(), 0.0);
}

#[test]
fn backward_compat_tolerance_trait_and_constant() {
    // The global `TOLERANCE` constant remains available and unchanged.
    assert_eq!(TOLERANCE, 1.0e-6);
    assert_eq!(TOLERANCE2, TOLERANCE * TOLERANCE);
}

#[test]
fn backward_compat_near_and_near2() {
    // `near()` and `near2()` still work with the global constant.
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE * 0.5;
    assert!(Tolerance::near(&a, &b));

    let c: f64 = 1.0;
    let d: f64 = 1.0 + TOLERANCE2 * 0.5;
    assert!(Tolerance::near2(&c, &d));
}
