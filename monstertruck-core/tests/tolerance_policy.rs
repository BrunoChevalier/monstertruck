//! Tolerance policy regression tests.
//!
//! These tests pin the canonical tolerance values and verify the import path
//! that all workspace crates should use. If TOLERANCE needs to change, update
//! these tests intentionally after assessing downstream impact.

use monstertruck_core::tolerance::{OperationTolerance, TOLERANCE, TOLERANCE2, Tolerance};

#[test]
fn tolerance_value_is_1e_minus_6() {
    assert_eq!(TOLERANCE, 1.0e-6, "TOLERANCE must be 1.0e-6");
}

#[test]
fn tolerance2_is_tolerance_squared() {
    assert_eq!(TOLERANCE2, TOLERANCE * TOLERANCE);
}

#[test]
fn operation_tolerance_from_global_uses_tolerance() {
    let op = OperationTolerance::from_global();
    assert_eq!(op.base(), TOLERANCE);
    assert_eq!(op.accumulated_error(), 0.0);
    assert_eq!(op.operation_count(), 0);
}

#[test]
fn near_trait_uses_tolerance() {
    let a: f64 = 1.0;
    let b: f64 = 1.0 + TOLERANCE * 0.5;
    assert!(a.near(&b), "Values within TOLERANCE should be near");

    let c: f64 = 1.0 + TOLERANCE * 2.0;
    assert!(!a.near(&c), "Values beyond TOLERANCE should not be near");
}
