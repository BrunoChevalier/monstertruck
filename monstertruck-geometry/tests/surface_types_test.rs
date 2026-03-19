use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;
use monstertruck_geometry::nurbs::surface_options::{
    Birail1Options, Birail2Options, FrameRule, GordonOptions, SkinOptions, SweepRailOptions,
};

#[test]
fn sweep_rail_options_default() {
    let opts = SweepRailOptions::default();
    assert_eq!(opts.n_sections, 10);
    assert_eq!(opts.frame_rule, FrameRule::TangentAligned);
}

#[test]
fn birail1_options_default() {
    let opts = Birail1Options::default();
    assert_eq!(opts.n_sections, 10);
}

#[test]
fn birail2_options_default() {
    let opts = Birail2Options::default();
    assert_eq!(opts.n_sections, 10);
}

#[test]
fn gordon_options_default() {
    let _opts = GordonOptions::default();
}

#[test]
fn skin_options_default() {
    let _opts = SkinOptions::default();
}

#[test]
fn frame_rule_default_is_tangent_aligned() {
    assert_eq!(FrameRule::default(), FrameRule::TangentAligned);
}

#[test]
fn curve_network_diagnostic_display_insufficient_curves() {
    let d = CurveNetworkDiagnostic::InsufficientCurves {
        required: 2,
        got: 0,
    };
    assert_eq!(
        d.to_string(),
        "insufficient curves: need at least 2, got 0"
    );
}

#[test]
fn curve_network_diagnostic_display_insufficient_sections() {
    let d = CurveNetworkDiagnostic::InsufficientSections {
        required: 2,
        got: 1,
    };
    assert_eq!(
        d.to_string(),
        "insufficient sections: need at least 2, got 1"
    );
}

#[test]
fn curve_network_diagnostic_display_endpoint_mismatch() {
    let d = CurveNetworkDiagnostic::EndpointMismatch {
        curve_index: 0,
        expected: [1.0, 2.0, 3.0],
        actual: [1.1, 2.1, 3.1],
        distance: 0.173205,
    };
    let s = d.to_string();
    assert!(s.contains("endpoint mismatch on curve 0"));
}

#[test]
fn curve_network_diagnostic_display_grid_dimension_mismatch() {
    let d = CurveNetworkDiagnostic::GridDimensionMismatch {
        expected_rows: 3,
        expected_cols: 4,
        actual_rows: 2,
        actual_cols: 5,
    };
    assert_eq!(
        d.to_string(),
        "grid dimension mismatch: expected 3x4, got 2x5"
    );
}

#[test]
fn curve_network_diagnostic_implements_error_trait() {
    let d = CurveNetworkDiagnostic::DegenerateGeometry {
        description: "zero-length chord".into(),
    };
    let e: &dyn std::error::Error = &d;
    assert!(e.to_string().contains("degenerate geometry"));
}

#[test]
fn curve_network_diagnostic_partial_eq() {
    let a = CurveNetworkDiagnostic::InsufficientCurves {
        required: 2,
        got: 0,
    };
    let b = CurveNetworkDiagnostic::InsufficientCurves {
        required: 2,
        got: 0,
    };
    assert_eq!(a, b);
}
