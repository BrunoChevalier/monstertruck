//! Tests for the text module's public API: [`TextOptions`] configuration,
//! edge cases (empty string, space glyph), and direct `glyph_profile`
//! behavior. These tests are distinct from `font_pipeline.rs` which covers
//! the full text-to-solid pipeline.
#![cfg(feature = "font")]

use monstertruck_modeling::*;

const FONT_BYTES: &[u8] = include_bytes!("../test-fixtures/DejaVuSans.ttf");

fn face() -> ttf_parser::Face<'static> {
    ttf_parser::Face::parse(FONT_BYTES, 0).expect("valid TTF")
}

/// `TextOptions::default()` produces y_flip=true, z=0.0, scale=None,
/// closure_tolerance=1e-7.
#[test]
fn text_module_options_default() {
    let opts = text::TextOptions::default();
    assert!(opts.y_flip, "default y_flip must be true");
    assert!(
        (opts.z - 0.0).abs() < f64::EPSILON,
        "default z must be 0.0"
    );
    assert!(opts.scale.is_none(), "default scale must be None");
    assert!(
        (opts.closure_tolerance - 1e-7).abs() < 1e-15,
        "default closure_tolerance must be 1e-7"
    );
}

/// Custom scale=Some(0.01) produces vertex coordinates in a small range
/// compared to the default (1/units_per_em) scale.
#[test]
fn text_module_options_custom_scale() {
    let f = face();
    let glyph_id = f.glyph_index('H').expect("glyph for 'H'");

    let opts_small = text::TextOptions {
        scale: Some(0.01),
        ..text::TextOptions::default()
    };
    let wires_small =
        text::glyph_profile(&f, glyph_id, &opts_small).expect("glyph_profile with small scale");

    // With scale=0.01, all coordinates should be small (< 30.0 absolute,
    // since font units are typically in the 0..2048 range).
    for wire in &wires_small {
        let v = wire.front_vertex().expect("front vertex");
        let pt = v.point();
        assert!(
            pt.x.abs() < 30.0 && pt.y.abs() < 30.0,
            "scaled vertex must have small coordinates, got ({}, {})",
            pt.x,
            pt.y
        );
    }

    // Default scale (1/units_per_em ~ 1/2048) produces even smaller values.
    let opts_default = text::TextOptions::default();
    let wires_default =
        text::glyph_profile(&f, glyph_id, &opts_default).expect("glyph_profile default scale");

    let default_x = wires_default[0]
        .front_vertex()
        .expect("front vertex")
        .point()
        .x
        .abs();
    let small_x = wires_small[0]
        .front_vertex()
        .expect("front vertex")
        .point()
        .x
        .abs();

    // With scale=0.01 coordinates will be larger than default scale (~0.0005).
    assert!(
        small_x > default_x * 5.0,
        "scale=0.01 should produce larger coords than default; small_x={}, default_x={}",
        small_x,
        default_x
    );
}

/// Custom z=5.0 puts all vertex Z coordinates near 5.0.
#[test]
fn text_module_options_custom_z() {
    let f = face();
    let glyph_id = f.glyph_index('H').expect("glyph for 'H'");

    let opts = text::TextOptions {
        z: 5.0,
        ..text::TextOptions::default()
    };
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile z=5.0");

    for wire in &wires {
        let v = wire.front_vertex().expect("front vertex");
        let pt = v.point();
        assert!(
            (pt.z - 5.0).abs() < 1e-10,
            "vertex Z must be near 5.0, got {}",
            pt.z
        );
    }
}

/// Looser closure_tolerance=1e-3 still produces valid wires.
#[test]
fn text_module_options_closure_tolerance() {
    let f = face();
    // Use 'S' which has complex curves.
    let glyph_id = f.glyph_index('S').expect("glyph for 'S'");

    let opts = text::TextOptions {
        closure_tolerance: 1e-3,
        ..text::TextOptions::default()
    };
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile loose tolerance");
    assert!(
        !wires.is_empty(),
        "glyph 'S' with loose tolerance must produce wires"
    );
    for w in &wires {
        assert!(w.is_closed(), "wire must be closed with loose tolerance");
    }
}

/// `text_profile("")` returns Ok with an empty vec.
#[test]
fn text_module_text_empty_string() {
    let f = face();
    let opts = text::TextOptions::default();
    let wires = text::text_profile(&f, "", &opts).expect("text_profile for empty string");
    assert!(
        wires.is_empty(),
        "empty string must produce 0 wires, got {}",
        wires.len()
    );
}

/// Space character has no outline, so `glyph_profile` returns an error.
#[test]
fn text_module_glyph_no_outline() {
    let f = face();
    let glyph_id = f.glyph_index(' ').expect("glyph for space");

    let opts = text::TextOptions::default();
    let result = text::glyph_profile(&f, glyph_id, &opts);
    assert!(
        result.is_err(),
        "space glyph has no outline, glyph_profile must return Err"
    );
}

/// `TextOptions` implements `Debug`.
#[test]
fn text_module_options_debug_display() {
    let opts = text::TextOptions::default();
    let debug_str = format!("{:?}", opts);
    assert!(
        !debug_str.is_empty(),
        "Debug output for TextOptions must not be empty"
    );
    assert!(
        debug_str.contains("TextOptions"),
        "Debug output must contain 'TextOptions', got: {}",
        debug_str
    );
}
