use monstertruck_traits::*;

// -- D1 and D2 dimension types --

#[test]
fn d1_dim_equals_1() {
    assert_eq!(<D1 as SearchParameterDimension>::DIM, 1);
}

#[test]
fn d2_dim_equals_2() {
    assert_eq!(<D2 as SearchParameterDimension>::DIM, 2);
}

// -- SearchParameterHint1D --

#[test]
fn hint1d_parameter_round_trips() {
    let hint = SearchParameterHint1D::Parameter(3.15);
    match hint {
        SearchParameterHint1D::Parameter(x) => assert!((x - 3.15).abs() < f64::EPSILON),
        _ => panic!("Expected Parameter variant"),
    }
}

#[test]
fn hint1d_range_round_trips() {
    let hint = SearchParameterHint1D::Range(1.0, 2.0);
    match hint {
        SearchParameterHint1D::Range(a, b) => {
            assert!((a - 1.0).abs() < f64::EPSILON);
            assert!((b - 2.0).abs() < f64::EPSILON);
        }
        _ => panic!("Expected Range variant"),
    }
}

#[test]
fn hint1d_none_from_option_none() {
    let hint: SearchParameterHint1D = Option::<f64>::None.into();
    assert_eq!(hint, SearchParameterHint1D::None);
}

#[test]
fn from_f64_creates_parameter_variant() {
    let hint: SearchParameterHint1D = 5.0.into();
    assert_eq!(hint, SearchParameterHint1D::Parameter(5.0));
}

#[test]
fn from_tuple_f64_creates_range_variant() {
    let hint: SearchParameterHint1D = (1.0, 2.0).into();
    assert_eq!(hint, SearchParameterHint1D::Range(1.0, 2.0));
}

#[test]
fn from_option_some_creates_parameter_variant() {
    let hint: SearchParameterHint1D = Some(7.0).into();
    assert_eq!(hint, SearchParameterHint1D::Parameter(7.0));
}

// -- SearchParameterHint2D --

#[test]
fn hint2d_parameter_round_trips() {
    let hint = SearchParameterHint2D::Parameter(1.0, 2.0);
    match hint {
        SearchParameterHint2D::Parameter(u, v) => {
            assert!((u - 1.0).abs() < f64::EPSILON);
            assert!((v - 2.0).abs() < f64::EPSILON);
        }
        _ => panic!("Expected Parameter variant"),
    }
}

#[test]
fn hint2d_range_round_trips() {
    let hint = SearchParameterHint2D::Range((0.0, 1.0), (2.0, 3.0));
    match hint {
        SearchParameterHint2D::Range(ur, vr) => {
            assert_eq!(ur, (0.0, 1.0));
            assert_eq!(vr, (2.0, 3.0));
        }
        _ => panic!("Expected Range variant"),
    }
}

#[test]
fn hint2d_none_from_option_none() {
    let hint: SearchParameterHint2D = Option::<(f64, f64)>::None.into();
    assert_eq!(hint, SearchParameterHint2D::None);
}

#[test]
fn from_tuple_creates_2d_parameter_variant() {
    let hint: SearchParameterHint2D = (1.0, 2.0).into();
    assert_eq!(hint, SearchParameterHint2D::Parameter(1.0, 2.0));
}

#[test]
fn from_nested_tuple_creates_2d_range_variant() {
    let hint: SearchParameterHint2D = ((0.0, 1.0), (2.0, 3.0)).into();
    assert_eq!(hint, SearchParameterHint2D::Range((0.0, 1.0), (2.0, 3.0)));
}

#[test]
fn from_option_some_creates_2d_parameter_variant() {
    let hint: SearchParameterHint2D = Some((4.0, 5.0)).into();
    assert_eq!(hint, SearchParameterHint2D::Parameter(4.0, 5.0));
}

// -- PartialEq and Debug --

#[test]
fn partial_eq_works_for_hints() {
    assert_eq!(
        SearchParameterHint1D::Parameter(1.0),
        SearchParameterHint1D::Parameter(1.0)
    );
    assert_ne!(
        SearchParameterHint1D::Parameter(1.0),
        SearchParameterHint1D::Parameter(2.0)
    );
    assert_eq!(SearchParameterHint1D::None, SearchParameterHint1D::None);
    assert_eq!(
        SearchParameterHint2D::Parameter(1.0, 2.0),
        SearchParameterHint2D::Parameter(1.0, 2.0)
    );
    assert_eq!(SearchParameterHint2D::None, SearchParameterHint2D::None);
}

#[test]
fn debug_is_implemented() {
    let hint1 = SearchParameterHint1D::Parameter(1.0);
    let s1 = format!("{:?}", hint1);
    assert!(s1.contains("Parameter"));
    let hint2 = SearchParameterHint2D::Range((0.0, 1.0), (2.0, 3.0));
    let s2 = format!("{:?}", hint2);
    assert!(s2.contains("Range"));
    // D1 and D2 are uninhabited enums -- they exist as type-level markers only.
    // Verify they implement Clone + Copy + Debug at the type level.
    fn _assert_debug<T: std::fmt::Debug + Clone + Copy>() {}
    _assert_debug::<D1>();
    _assert_debug::<D2>();
}
