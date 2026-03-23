#![cfg(feature = "polynomial")]

use monstertruck_core::assert_near2;
use monstertruck_core::cgmath64::*;
use monstertruck_traits::{polynomial::PolynomialCurve, *};
use std::ops::Bound;

// P(t) = (t^2 + t + 1, 2t - 1)
fn make_curve() -> PolynomialCurve<Point2> {
    PolynomialCurve::<Point2>(vec![
        Vector2::new(1.0, -1.0),
        Vector2::new(1.0, 2.0),
        Vector2::new(1.0, 0.0),
    ])
}

// -- ParametricCurve trait methods --

#[test]
fn evaluate_returns_correct_point() {
    let c = make_curve();
    // P(0) = (1, -1)
    let p = c.evaluate(0.0);
    assert_near2!(p, Point2::new(1.0, -1.0));
    // P(1) = (1+1+1, 2-1) = (3, 1)
    let p = c.evaluate(1.0);
    assert_near2!(p, Point2::new(3.0, 1.0));
    // P(2) = (4+2+1, 4-1) = (7, 3)
    let p = c.evaluate(2.0);
    assert_near2!(p, Point2::new(7.0, 3.0));
}

#[test]
fn derivative_returns_first_derivative() {
    let c = make_curve();
    // P'(t) = (2t + 1, 2)
    let d = c.derivative(0.0);
    assert_near2!(d, Vector2::new(1.0, 2.0));
    let d = c.derivative(1.0);
    assert_near2!(d, Vector2::new(3.0, 2.0));
}

#[test]
fn derivative_2_returns_second_derivative() {
    let c = make_curve();
    // P''(t) = (2, 0)
    let d = c.derivative_2(0.0);
    assert_near2!(d, Vector2::new(2.0, 0.0));
    let d = c.derivative_2(5.0);
    assert_near2!(d, Vector2::new(2.0, 0.0));
}

#[test]
fn derivative_n_for_various_orders() {
    let c = make_curve();
    // n=0 is evaluate as vector
    let d0 = c.derivative_n(0, 1.0);
    assert_near2!(d0, Vector2::new(3.0, 1.0));
    // n=1 is first derivative
    let d1 = c.derivative_n(1, 1.0);
    assert_near2!(d1, Vector2::new(3.0, 2.0));
    // n=2 is second derivative
    let d2 = c.derivative_n(2, 1.0);
    assert_near2!(d2, Vector2::new(2.0, 0.0));
    // n=3 for quadratic polynomial is zero
    let d3 = c.derivative_n(3, 1.0);
    assert_near2!(d3, Vector2::new(0.0, 0.0));
}

#[test]
fn derivatives_returns_curve_derivatives() {
    let c = make_curve();
    let ders = c.derivatives(2, 1.0);
    assert_eq!(ders.max_order(), 2);
    assert_near2!(ders[0], Vector2::new(3.0, 1.0));
    assert_near2!(ders[1], Vector2::new(3.0, 2.0));
    assert_near2!(ders[2], Vector2::new(2.0, 0.0));
}

#[test]
fn subs_is_alias_for_evaluate() {
    let c = make_curve();
    assert_near2!(c.subs(1.5), c.evaluate(1.5));
}

#[test]
fn der_is_alias_for_derivative() {
    let c = make_curve();
    assert_near2!(c.der(1.5), c.derivative(1.5));
}

#[test]
fn der2_is_alias_for_derivative_2() {
    let c = make_curve();
    assert_near2!(c.der2(1.5), c.derivative_2(1.5));
}

#[test]
fn der_n_is_alias_for_derivative_n() {
    let c = make_curve();
    assert_near2!(c.der_n(1, 1.5), c.derivative_n(1, 1.5));
    assert_near2!(c.der_n(2, 1.5), c.derivative_n(2, 1.5));
}

#[test]
fn ders_is_alias_for_derivatives() {
    let c = make_curve();
    let a = c.ders(2, 1.5);
    let b = c.derivatives(2, 1.5);
    assert_eq!(a.max_order(), b.max_order());
    (0..=2).for_each(|i| assert_near2!(a[i], b[i]));
}

#[test]
fn parameter_range_is_bounded() {
    let c = make_curve();
    let (lo, hi) = c.parameter_range();
    assert_eq!(lo, Bound::Included(-100.0));
    assert_eq!(hi, Bound::Included(100.0));
}

#[test]
fn try_range_tuple_returns_some() {
    let c = make_curve();
    assert_eq!(c.try_range_tuple(), Some((-100.0, 100.0)));
}

#[test]
fn period_returns_none() {
    let c = make_curve();
    assert_eq!(c.period(), None);
}

// -- BoundedCurve methods --

#[test]
fn range_tuple_returns_bounds() {
    let c = make_curve();
    assert_eq!(c.range_tuple(), (-100.0, 100.0));
}

#[test]
fn front_evaluates_at_range_start() {
    let c = make_curve();
    let front = c.front();
    let expected = c.evaluate(-100.0);
    assert_near2!(front, expected);
}

#[test]
fn back_evaluates_at_range_end() {
    let c = make_curve();
    let back = c.back();
    let expected = c.evaluate(100.0);
    assert_near2!(back, expected);
}

// -- Reference delegation --

#[test]
fn reference_delegation_works() {
    let c = make_curve();
    let r = &c;
    assert_near2!(r.evaluate(1.0), c.evaluate(1.0));
    assert_near2!(r.derivative(1.0), c.derivative(1.0));
    assert_near2!(r.derivative_2(1.0), c.derivative_2(1.0));
    assert_near2!(r.derivative_n(1, 1.0), c.derivative_n(1, 1.0));
    assert_eq!(r.parameter_range(), c.parameter_range());
    assert_eq!(r.period(), c.period());
    assert_eq!(r.range_tuple(), c.range_tuple());
    assert_near2!(r.front(), c.front());
    assert_near2!(r.back(), c.back());
}

// -- Box delegation --

#[test]
fn box_delegation_works() {
    let c = make_curve();
    let b = Box::new(c.clone());
    assert_near2!(b.evaluate(1.0), c.evaluate(1.0));
    assert_near2!(b.derivative(1.0), c.derivative(1.0));
    assert_near2!(b.derivative_2(1.0), c.derivative_2(1.0));
    assert_eq!(b.parameter_range(), c.parameter_range());
    assert_eq!(b.period(), c.period());
    assert_eq!(b.range_tuple(), c.range_tuple());
    assert_near2!(b.front(), c.front());
    assert_near2!(b.back(), c.back());
}

// -- CurveCollector tests --

#[test]
fn curve_collector_singleton_is_singleton() {
    let collector: CurveCollector<(usize, usize)> = CurveCollector::Singleton;
    assert!(collector.is_singleton());
}

#[test]
fn curve_collector_curve_is_not_singleton() {
    let collector = CurveCollector::Curve((1usize, 2usize));
    assert!(!collector.is_singleton());
}

#[test]
fn curve_collector_into_option() {
    let singleton: CurveCollector<(usize, usize)> = CurveCollector::Singleton;
    let opt: Option<(usize, usize)> = singleton.into();
    assert_eq!(opt, None);

    let curve = CurveCollector::Curve((1usize, 2usize));
    let opt: Option<(usize, usize)> = curve.into();
    assert_eq!(opt, Some((1, 2)));
}

// -- ConcatError tests --

#[test]
fn concat_error_disconnected_parameters_display() {
    let err: ConcatError<usize> = ConcatError::DisconnectedParameters(1.0, 2.0);
    let msg = format!("{}", err);
    assert!(msg.contains("end parameter"));
    assert!(msg.contains("start parameter"));
}

#[test]
fn concat_error_disconnected_points_display() {
    let err = ConcatError::DisconnectedPoints(1usize, 2usize);
    let msg = format!("{}", err);
    assert!(msg.contains("end point"));
    assert!(msg.contains("start point"));
}

#[test]
fn concat_error_point_map_transforms_correctly() {
    let err = ConcatError::DisconnectedPoints(1usize, 2usize);
    let mapped = err.point_map(|x| x as f64 * 10.0);
    match mapped {
        ConcatError::DisconnectedPoints(a, b) => {
            assert!((a - 10.0).abs() < f64::EPSILON);
            assert!((b - 20.0).abs() < f64::EPSILON);
        }
        _ => panic!("Expected DisconnectedPoints"),
    }

    // Test that DisconnectedParameters passes through unchanged.
    let err2: ConcatError<usize> = ConcatError::DisconnectedParameters(3.0, 4.0);
    let mapped2 = err2.point_map(|x| x * 100);
    match mapped2 {
        ConcatError::DisconnectedParameters(a, b) => {
            assert!((a - 3.0).abs() < f64::EPSILON);
            assert!((b - 4.0).abs() < f64::EPSILON);
        }
        _ => panic!("Expected DisconnectedParameters"),
    }
}

// -- Cut trait (using (usize, usize) impl doesn't exist, test via PolynomialCurve if Cut is impl'd) --
// Note: Cut is not implemented for PolynomialCurve, and (usize, usize) doesn't implement Cut.
// We test the Cut trait for Box<C> delegation indirectly.
