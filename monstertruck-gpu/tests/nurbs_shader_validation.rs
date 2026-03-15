//! Tests for WGSL NURBS tessellation shader validation.

use naga::front::wgsl;

const SHADER_SOURCE: &str = include_str!("../shaders/nurbs_tessellation.wgsl");

#[test]
fn shader_parses_without_errors() {
    let module = wgsl::parse_str(SHADER_SOURCE).expect("WGSL shader failed to parse");
    // Verify the module has at least one entry point.
    assert!(
        !module.entry_points.is_empty(),
        "Shader must have at least one entry point."
    );
}

#[test]
fn shader_has_compute_entry_point() {
    let module = wgsl::parse_str(SHADER_SOURCE).expect("WGSL shader failed to parse");
    let has_compute = module
        .entry_points
        .iter()
        .any(|ep| ep.stage == naga::ShaderStage::Compute);
    assert!(has_compute, "Shader must have a @compute entry point.");
}

#[test]
fn shader_declares_max_degree_override() {
    // The shader source must contain the MAX_DEGREE override constant.
    assert!(
        SHADER_SOURCE.contains("override MAX_DEGREE"),
        "Shader must declare an override constant named MAX_DEGREE."
    );
}

#[test]
fn shader_contains_basis_funs() {
    assert!(
        SHADER_SOURCE.contains("fn basis_funs"),
        "Shader must contain a basis_funs function."
    );
}

#[test]
fn shader_contains_find_span() {
    assert!(
        SHADER_SOURCE.contains("fn find_span"),
        "Shader must contain a find_span function."
    );
}

#[test]
fn shader_contains_surface_point() {
    assert!(
        SHADER_SOURCE.contains("fn surface_point"),
        "Shader must contain a surface_point function."
    );
}

#[test]
fn shader_contains_surface_normal() {
    assert!(
        SHADER_SOURCE.contains("fn surface_normal"),
        "Shader must contain a surface_normal function."
    );
}
