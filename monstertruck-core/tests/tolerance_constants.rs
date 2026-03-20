use monstertruck_core::tolerance::TOLERANCE;
use monstertruck_core::tolerance_constants::{
    G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE, PERIODIC_CLOSURE_RATIO, SNAP_TOLERANCE,
    TESSELLATION_TOLERANCE, UV_CLOSURE_TOLERANCE, VERTEX_MERGE_TOLERANCE,
};

#[test]
fn snap_tolerance_equals_10x_tolerance() {
    assert_eq!(SNAP_TOLERANCE, 10.0 * TOLERANCE);
}

#[test]
fn vertex_merge_tolerance_equals_100x_tolerance() {
    assert_eq!(VERTEX_MERGE_TOLERANCE, 100.0 * TOLERANCE);
}

#[test]
fn tessellation_tolerance_value() {
    assert_eq!(TESSELLATION_TOLERANCE, 0.01);
}

#[test]
fn periodic_closure_ratio_value() {
    assert_eq!(PERIODIC_CLOSURE_RATIO, 0.01);
}

#[test]
fn g1_angle_tolerance_value() {
    assert_eq!(G1_ANGLE_TOLERANCE, 0.0175);
}

#[test]
fn g2_curvature_tolerance_value() {
    assert_eq!(G2_CURVATURE_TOLERANCE, 0.10);
}

#[test]
fn snap_tolerance_numeric_value() {
    assert!((SNAP_TOLERANCE - 1.0e-5).abs() < 1.0e-15);
}

#[test]
fn vertex_merge_tolerance_numeric_value() {
    assert!((VERTEX_MERGE_TOLERANCE - 1.0e-4).abs() < 1.0e-14);
}

#[test]
fn uv_closure_tolerance_value() {
    // UV_CLOSURE_TOLERANCE should be TESSELLATION_TOLERANCE / 10.
    assert!((UV_CLOSURE_TOLERANCE - TESSELLATION_TOLERANCE / 10.0).abs() < f64::EPSILON);
    // Must be positive.
    const { assert!(UV_CLOSURE_TOLERANCE > 0.0) }
    // Must be smaller than TESSELLATION_TOLERANCE.
    const { assert!(UV_CLOSURE_TOLERANCE < TESSELLATION_TOLERANCE) }
}
