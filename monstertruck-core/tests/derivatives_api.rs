use monstertruck_core::cgmath64::*;

// ---- CurveDerivatives ----

#[test]
fn curve_derivatives_new_zeroed() {
    let cd = CurveDerivatives::<f64>::new(3);
    assert_eq!(cd.max_order(), 3);
    assert_eq!(cd.len(), 4);
    assert!(cd.iter().all(|&v| v == 0.0));
}

#[test]
fn curve_derivatives_push() {
    let mut cd = CurveDerivatives::<f64>::new(0);
    cd[0] = 1.0;
    cd.push(2.0);
    cd.push(3.0);
    assert_eq!(cd.max_order(), 2);
    assert_eq!(&*cd, &[1.0, 2.0, 3.0]);
}

#[test]
fn curve_derivatives_derivative() {
    let cd = CurveDerivatives::try_from([0.0, 1.0, 2.0]).unwrap();
    let der = cd.derivative();
    assert_eq!(&*der, &[1.0, 2.0]);
    assert_eq!(der.max_order(), 1);
}

#[test]
fn curve_derivatives_try_from_array() {
    let cd = CurveDerivatives::try_from([10.0, 20.0, 30.0]).unwrap();
    assert_eq!(cd.max_order(), 2);
    assert_eq!(cd[0], 10.0);
    assert_eq!(cd[1], 20.0);
    assert_eq!(cd[2], 30.0);
}

#[test]
fn curve_derivatives_try_from_slice() {
    let data = [5.0, 6.0, 7.0, 8.0];
    let cd = CurveDerivatives::try_from(data.as_slice()).unwrap();
    assert_eq!(cd.max_order(), 3);
    assert_eq!(&*cd, &[5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn curve_derivatives_from_iterator() {
    let cd: CurveDerivatives<f64> = vec![1.0, 2.0, 3.0].into_iter().collect();
    assert_eq!(cd.max_order(), 2);
    assert_eq!(&*cd, &[1.0, 2.0, 3.0]);
}

#[test]
fn curve_derivatives_max_order() {
    let cd = CurveDerivatives::try_from([1.0, 2.0]).unwrap();
    assert_eq!(cd.max_order(), 1);
}

#[test]
fn curve_derivatives_deref_to_slice() {
    let cd = CurveDerivatives::try_from([1.0, 2.0, 3.0]).unwrap();
    let slice: &[f64] = &cd;
    assert_eq!(slice, &[1.0, 2.0, 3.0]);
}

#[test]
fn curve_derivatives_mul_scalar() {
    let cd = CurveDerivatives::try_from([Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)]).unwrap();
    let scaled = cd * 2.0;
    assert_eq!(scaled[0], Vector2::new(2.0, 4.0));
    assert_eq!(scaled[1], Vector2::new(6.0, 8.0));
}

#[test]
fn curve_derivatives_div_scalar() {
    let cd = CurveDerivatives::try_from([Vector2::new(4.0, 6.0), Vector2::new(8.0, 10.0)]).unwrap();
    let divided = cd / 2.0;
    assert_eq!(divided[0], Vector2::new(2.0, 3.0));
    assert_eq!(divided[1], Vector2::new(4.0, 5.0));
}

// ---- SurfaceDerivatives ----

#[test]
fn surface_derivatives_new_zeroed() {
    let sd = SurfaceDerivatives::<f64>::new(2);
    assert_eq!(sd.max_order(), 2);
    // Row 0 has 3 elements, row 1 has 2, row 2 has 1.
    assert_eq!(sd[0].len(), 3);
    assert_eq!(sd[1].len(), 2);
    assert_eq!(sd[2].len(), 1);
    assert!(sd.slice_iter().flat_map(|s| s.iter()).all(|&v| v == 0.0));
}

#[test]
fn surface_derivatives_derivative_u() {
    let sd =
        SurfaceDerivatives::try_from([[0.0, 1.0, 2.0].as_slice(), &[3.0, 4.0], &[5.0]].as_slice())
            .unwrap();
    let du = sd.derivative_u();
    assert_eq!(du.max_order(), 1);
    assert_eq!(du[0], [3.0_f64, 4.0]);
    assert_eq!(du[1], [5.0_f64]);
}

#[test]
fn surface_derivatives_derivative_v() {
    let sd =
        SurfaceDerivatives::try_from([[0.0, 1.0, 2.0].as_slice(), &[3.0, 4.0], &[5.0]].as_slice())
            .unwrap();
    let dv = sd.derivative_v();
    assert_eq!(dv.max_order(), 1);
    assert_eq!(dv[0], [1.0_f64, 2.0]);
    assert_eq!(dv[1], [4.0_f64]);
}

#[test]
fn surface_derivatives_index_and_index_mut() {
    let mut sd = SurfaceDerivatives::<f64>::new(2);
    sd[0][0] = 10.0;
    sd[1][0] = 20.0;
    sd[2][0] = 30.0;
    assert_eq!(sd[0][0], 10.0);
    assert_eq!(sd[1][0], 20.0);
    assert_eq!(sd[2][0], 30.0);
}

#[test]
fn curve_derivatives_to_array() {
    let cd = CurveDerivatives::try_from([1.0, 2.0, 3.0]).unwrap();
    let arr: [f64; 2] = cd.to_array::<2>();
    assert_eq!(arr, [1.0, 2.0]);
}

#[test]
fn curve_derivatives_abs_diff_eq() {
    let a = CurveDerivatives::try_from([1.0, 2.0, 3.0]).unwrap();
    let b = CurveDerivatives::try_from([1.0 + 1e-10, 2.0, 3.0 - 1e-10]).unwrap();
    assert!(monstertruck_core::tolerance::Tolerance::near(&a, &b));
}

#[test]
fn surface_derivatives_mul_scalar() {
    let sd = SurfaceDerivatives::try_from(
        [
            [Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0)].as_slice(),
            &[Vector2::new(5.0, 6.0)],
        ]
        .as_slice(),
    )
    .unwrap();
    let scaled = sd * 2.0;
    assert_eq!(scaled[0][0], Vector2::new(2.0, 4.0));
    assert_eq!(scaled[0][1], Vector2::new(6.0, 8.0));
    assert_eq!(scaled[1][0], Vector2::new(10.0, 12.0));
}

#[test]
fn surface_derivatives_div_scalar() {
    let sd = SurfaceDerivatives::try_from(
        [
            [Vector2::new(4.0, 6.0), Vector2::new(8.0, 10.0)].as_slice(),
            &[Vector2::new(12.0, 14.0)],
        ]
        .as_slice(),
    )
    .unwrap();
    let divided = sd / 2.0;
    assert_eq!(divided[0][0], Vector2::new(2.0, 3.0));
    assert_eq!(divided[0][1], Vector2::new(4.0, 5.0));
    assert_eq!(divided[1][0], Vector2::new(6.0, 7.0));
}
