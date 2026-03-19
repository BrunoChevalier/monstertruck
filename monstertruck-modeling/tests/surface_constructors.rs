use monstertruck_modeling::errors::Error;

/// Verify that the new error variants exist and display correct messages.
#[test]
fn test_error_variants_exist() {
    let e1 = Error::InsufficientRails {
        required: 2,
        got: 1,
    };
    assert_eq!(
        e1.to_string(),
        "multi-rail sweep requires at least 2 rails, got 1."
    );

    let e2 = Error::InsufficientSections {
        required: 2,
        got: 1,
    };
    assert_eq!(
        e2.to_string(),
        "surface construction requires at least 2 sections, got 1."
    );

    let e3 = Error::SurfaceConstructionFailed {
        reason: "degenerate".into(),
    };
    assert_eq!(
        e3.to_string(),
        "surface construction failed: degenerate"
    );

    let e4 = Error::GridDimensionMismatch {
        expected_rows: 2,
        expected_cols: 3,
        actual_rows: 1,
        actual_cols: 2,
    };
    assert_eq!(
        e4.to_string(),
        "gordon surface requires matching grid dimensions: expected 2x3, got 1x2."
    );
}

/// Verify PartialEq still works with the new variants.
#[test]
fn test_error_variants_eq() {
    assert_eq!(
        Error::InsufficientRails {
            required: 2,
            got: 1
        },
        Error::InsufficientRails {
            required: 2,
            got: 1
        },
    );
    assert_ne!(
        Error::InsufficientRails {
            required: 2,
            got: 1
        },
        Error::InsufficientRails {
            required: 3,
            got: 1
        },
    );
}
