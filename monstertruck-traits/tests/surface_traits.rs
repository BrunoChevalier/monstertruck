#![cfg(feature = "polynomial")]

use monstertruck_core::assert_near;
use monstertruck_core::cgmath64::*;
use monstertruck_traits::{polynomial::PolynomialSurface, *};
use std::ops::Bound;

// Surface with non-trivial u and v dependence, defined directly.
// S(u,v) = (u+v, u*v+1, u^2+v^2)
fn make_nondegenerate_surface() -> PolynomialSurface<Point3> {
    // Use a surface with non-trivial u and v dependence.
    // coefs[i][j] for i=0..2, j=0..2
    // S(u,v) = sum coefs[i][j] * u^i * v^j
    // Let's set it directly rather than via tensor product:
    // S(u,v) = (u + v, u*v + 1, u^2 + v^2)
    // coefs[0][0] = (0, 1, 0)
    // coefs[1][0] = (1, 0, 0)
    // coefs[0][1] = (1, 0, 0)
    // coefs[1][1] = (0, 1, 0)
    // coefs[2][0] = (0, 0, 1)
    // coefs[0][2] = (0, 0, 1)
    PolynomialSurface::<Point3>(vec![
        vec![
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ],
        vec![Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0)],
        vec![Vector3::new(0.0, 0.0, 1.0)],
    ])
}

// S(u,v) = (u+v, u*v+1, u^2+v^2)
// dS/du = (1, v, 2u)
// dS/dv = (1, u, 2v)
// d2S/du2 = (0, 0, 2)
// d2S/dudv = (0, 1, 0)
// d2S/dv2 = (0, 0, 2)

// -- ParametricSurface trait methods --

#[test]
fn evaluate_returns_correct_point() {
    let s = make_nondegenerate_surface();
    // S(1,2) = (1+2, 1*2+1, 1+4) = (3, 3, 5)
    let p = s.evaluate(1.0, 2.0);
    assert_near!(p, Point3::new(3.0, 3.0, 5.0));
    // S(0,0) = (0, 1, 0)
    let p = s.evaluate(0.0, 0.0);
    assert_near!(p, Point3::new(0.0, 1.0, 0.0));
}

#[test]
fn derivative_u_is_correct() {
    let s = make_nondegenerate_surface();
    // dS/du at (1,2) = (1, 2, 2)
    let d = s.derivative_u(1.0, 2.0);
    assert_near!(d, Vector3::new(1.0, 2.0, 2.0));
}

#[test]
fn derivative_v_is_correct() {
    let s = make_nondegenerate_surface();
    // dS/dv at (1,2) = (1, 1, 4)
    let d = s.derivative_v(1.0, 2.0);
    assert_near!(d, Vector3::new(1.0, 1.0, 4.0));
}

#[test]
fn derivative_uu_is_correct() {
    let s = make_nondegenerate_surface();
    // d2S/du2 = (0, 0, 2)
    let d = s.derivative_uu(1.0, 2.0);
    assert_near!(d, Vector3::new(0.0, 0.0, 2.0));
}

#[test]
fn derivative_uv_is_correct() {
    let s = make_nondegenerate_surface();
    // d2S/dudv = (0, 1, 0)
    let d = s.derivative_uv(1.0, 2.0);
    assert_near!(d, Vector3::new(0.0, 1.0, 0.0));
}

#[test]
fn derivative_vv_is_correct() {
    let s = make_nondegenerate_surface();
    // d2S/dv2 = (0, 0, 2)
    let d = s.derivative_vv(1.0, 2.0);
    assert_near!(d, Vector3::new(0.0, 0.0, 2.0));
}

#[test]
fn derivative_mn_various_orders() {
    let s = make_nondegenerate_surface();
    // (0,0) = evaluate as vector
    let d00 = s.derivative_mn(0, 0, 1.0, 2.0);
    assert_near!(d00, Vector3::new(3.0, 3.0, 5.0));
    // (1,0) = du
    let d10 = s.derivative_mn(1, 0, 1.0, 2.0);
    assert_near!(d10, Vector3::new(1.0, 2.0, 2.0));
    // (0,1) = dv
    let d01 = s.derivative_mn(0, 1, 1.0, 2.0);
    assert_near!(d01, Vector3::new(1.0, 1.0, 4.0));
    // (1,1) = duv
    let d11 = s.derivative_mn(1, 1, 1.0, 2.0);
    assert_near!(d11, Vector3::new(0.0, 1.0, 0.0));
    // (2,0) = duu
    let d20 = s.derivative_mn(2, 0, 1.0, 2.0);
    assert_near!(d20, Vector3::new(0.0, 0.0, 2.0));
    // (0,2) = dvv
    let d02 = s.derivative_mn(0, 2, 1.0, 2.0);
    assert_near!(d02, Vector3::new(0.0, 0.0, 2.0));
}

#[test]
fn derivatives_returns_surface_derivatives() {
    let s = make_nondegenerate_surface();
    let ders = s.derivatives(2, 1.0, 2.0);
    assert_eq!(ders.max_order(), 2);
    // [0][0] = S
    assert_near!(ders[0][0], Vector3::new(3.0, 3.0, 5.0));
    // [1][0] = du
    assert_near!(ders[1][0], Vector3::new(1.0, 2.0, 2.0));
    // [0][1] = dv
    assert_near!(ders[0][1], Vector3::new(1.0, 1.0, 4.0));
    // [1][1] = duv
    assert_near!(ders[1][1], Vector3::new(0.0, 1.0, 0.0));
    // [2][0] = duu
    assert_near!(ders[2][0], Vector3::new(0.0, 0.0, 2.0));
    // [0][2] = dvv
    assert_near!(ders[0][2], Vector3::new(0.0, 0.0, 2.0));
}

// -- Deprecated aliases --

#[test]
fn subs_is_alias_for_evaluate() {
    let s = make_nondegenerate_surface();
    assert_near!(s.subs(1.0, 2.0), s.evaluate(1.0, 2.0));
}

#[test]
fn uder_is_alias_for_derivative_u() {
    let s = make_nondegenerate_surface();
    assert_near!(s.uder(1.0, 2.0), s.derivative_u(1.0, 2.0));
}

#[test]
fn vder_is_alias_for_derivative_v() {
    let s = make_nondegenerate_surface();
    assert_near!(s.vder(1.0, 2.0), s.derivative_v(1.0, 2.0));
}

#[test]
fn uuder_is_alias_for_derivative_uu() {
    let s = make_nondegenerate_surface();
    assert_near!(s.uuder(1.0, 2.0), s.derivative_uu(1.0, 2.0));
}

#[test]
fn uvder_is_alias_for_derivative_uv() {
    let s = make_nondegenerate_surface();
    assert_near!(s.uvder(1.0, 2.0), s.derivative_uv(1.0, 2.0));
}

#[test]
fn vvder_is_alias_for_derivative_vv() {
    let s = make_nondegenerate_surface();
    assert_near!(s.vvder(1.0, 2.0), s.derivative_vv(1.0, 2.0));
}

#[test]
fn der_mn_is_alias_for_derivative_mn() {
    let s = make_nondegenerate_surface();
    assert_near!(s.der_mn(1, 1, 1.0, 2.0), s.derivative_mn(1, 1, 1.0, 2.0));
}

#[test]
fn ders_is_alias_for_derivatives() {
    let s = make_nondegenerate_surface();
    let a = s.ders(2, 1.0, 2.0);
    let b = s.derivatives(2, 1.0, 2.0);
    assert_eq!(a.max_order(), b.max_order());
    (0..=2).for_each(|m| {
        (0..=2 - m).for_each(|n| assert_near!(a[m][n], b[m][n]));
    });
}

// -- parameter_range, try_range_tuple, periods --

#[test]
fn parameter_range_returns_bounded_ranges() {
    let s = make_nondegenerate_surface();
    let (urange, vrange) = s.parameter_range();
    assert_eq!(urange, (Bound::Included(-100.0), Bound::Included(100.0)));
    assert_eq!(vrange, (Bound::Included(-50.0), Bound::Included(50.0)));
}

#[test]
fn try_range_tuple_returns_some() {
    let s = make_nondegenerate_surface();
    let (ur, vr) = s.try_range_tuple();
    assert_eq!(ur, Some((-100.0, 100.0)));
    assert_eq!(vr, Some((-50.0, 50.0)));
}

#[test]
fn u_period_returns_none() {
    let s = make_nondegenerate_surface();
    assert_eq!(s.u_period(), None);
}

#[test]
fn v_period_returns_none() {
    let s = make_nondegenerate_surface();
    assert_eq!(s.v_period(), None);
}

// -- ParametricSurface3D methods --

#[test]
fn normal_is_perpendicular_to_tangents() {
    let s = make_nondegenerate_surface();
    let u = 1.0;
    let v = 2.0;
    let n = s.normal(u, v);
    let du = s.derivative_u(u, v);
    let dv = s.derivative_v(u, v);
    assert!(n.dot(du).abs() < 1.0e-10);
    assert!(n.dot(dv).abs() < 1.0e-10);
}

#[test]
fn normal_is_unit_length() {
    let s = make_nondegenerate_surface();
    let n = s.normal(1.0, 2.0);
    assert!((n.magnitude() - 1.0).abs() < 1.0e-10);
}

#[test]
fn normal_uder_matches_finite_difference() {
    let s = make_nondegenerate_surface();
    let u = 1.0;
    let v = 2.0;
    let eps = 1.0e-4;
    let approx = (s.normal(u + eps, v) - s.normal(u - eps, v)) / (2.0 * eps);
    let exact = s.normal_uder(u, v);
    assert!((exact - approx).magnitude() < eps);
}

#[test]
fn normal_vder_matches_finite_difference() {
    let s = make_nondegenerate_surface();
    let u = 1.0;
    let v = 2.0;
    let eps = 1.0e-4;
    let approx = (s.normal(u, v + eps) - s.normal(u, v - eps)) / (2.0 * eps);
    let exact = s.normal_vder(u, v);
    assert!((exact - approx).magnitude() < eps);
}

// -- BoundedSurface methods --

#[test]
fn bounded_surface_range_tuple() {
    let s = make_nondegenerate_surface();
    let (ur, vr) = s.range_tuple();
    assert_eq!(ur, (-100.0, 100.0));
    assert_eq!(vr, (-50.0, 50.0));
}

// -- Delegation tests --

#[test]
fn reference_delegation_works() {
    let s = make_nondegenerate_surface();
    let r = &s;
    assert_near!(r.evaluate(1.0, 2.0), s.evaluate(1.0, 2.0));
    assert_near!(r.derivative_u(1.0, 2.0), s.derivative_u(1.0, 2.0));
    assert_near!(r.derivative_v(1.0, 2.0), s.derivative_v(1.0, 2.0));
    assert_eq!(r.parameter_range(), s.parameter_range());
    assert_eq!(r.u_period(), s.u_period());
    assert_eq!(r.v_period(), s.v_period());
    // ParametricSurface3D delegation
    let n1 = r.normal(1.0, 2.0);
    let n2 = s.normal(1.0, 2.0);
    assert_near!(n1, n2);
}

#[test]
fn box_delegation_works() {
    let s = make_nondegenerate_surface();
    let b = Box::new(s.clone());
    assert_near!(b.evaluate(1.0, 2.0), s.evaluate(1.0, 2.0));
    assert_near!(b.derivative_u(1.0, 2.0), s.derivative_u(1.0, 2.0));
    assert_eq!(b.parameter_range(), s.parameter_range());
    assert_eq!(b.u_period(), s.u_period());
    // ParametricSurface3D for Box
    let n1 = b.normal(1.0, 2.0);
    let n2 = s.normal(1.0, 2.0);
    assert_near!(n1, n2);
}
