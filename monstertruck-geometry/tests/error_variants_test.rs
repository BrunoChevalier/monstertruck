use monstertruck_geometry::errors::Error;
use monstertruck_geometry::nurbs::surface_diagnostics::CurveNetworkDiagnostic;

#[test]
fn error_curve_network_incompatible_display() {
    let e = Error::CurveNetworkIncompatible(CurveNetworkDiagnostic::InsufficientCurves {
        required: 2,
        got: 0,
    });
    let msg = e.to_string();
    assert!(msg.contains("curve network is incompatible"));
    assert!(msg.contains("insufficient curves"));
}

#[test]
fn error_insufficient_sections_display() {
    let e = Error::InsufficientSections {
        required: 2,
        got: 1,
    };
    let msg = e.to_string();
    assert!(msg.contains("at least 2 sections"));
    assert!(msg.contains("got 1"));
}

#[test]
fn error_surface_construction_failed_display() {
    let e = Error::SurfaceConstructionFailed {
        reason: "test reason".into(),
    };
    let msg = e.to_string();
    assert!(msg.contains("surface construction failed"));
    assert!(msg.contains("test reason"));
}

#[test]
fn error_from_curve_network_diagnostic() {
    let d = CurveNetworkDiagnostic::DegenerateGeometry {
        description: "zero chord".into(),
    };
    let e: Error = d.into();
    assert!(matches!(e, Error::CurveNetworkIncompatible(_)));
}

#[test]
fn error_variants_partial_eq() {
    let e1 = Error::CurveNetworkIncompatible(CurveNetworkDiagnostic::InsufficientCurves {
        required: 2,
        got: 0,
    });
    let e2 = Error::CurveNetworkIncompatible(CurveNetworkDiagnostic::InsufficientCurves {
        required: 2,
        got: 0,
    });
    assert_eq!(e1, e2);
}
