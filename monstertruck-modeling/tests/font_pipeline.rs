//! End-to-end integration tests for the font outline -> wire -> face -> solid pipeline.
//!
//! These tests require the `font` feature flag:
//!   cargo nextest run -p monstertruck-modeling --features font
#![cfg(feature = "font")]

use monstertruck_modeling::*;

/// Path to the bundled DejaVu Sans font fixture.
const FONT_BYTES: &[u8] = include_bytes!("../test-fixtures/DejaVuSans.ttf");

fn face() -> ttf_parser::Face<'static> {
    ttf_parser::Face::parse(FONT_BYTES, 0).expect("valid TTF")
}

fn default_opts() -> text::TextOptions {
    text::TextOptions::default()
}

/// Glyph 'O' has an outer contour and at least one inner hole.
#[test]
fn glyph_o_has_hole() {
    let f = face();
    let glyph_id = f.glyph_index('O').expect("glyph for 'O'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'O'");
    assert!(
        wires.len() >= 2,
        "Expected >= 2 wires for 'O' (outer + hole), got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "Wire in 'O' must be closed");
        assert!(w.len() >= 3, "Wire in 'O' must have >= 3 edges");
    }
}

/// Glyph 'B' has an outer contour and two inner holes.
#[test]
fn glyph_b_has_two_holes() {
    let f = face();
    let glyph_id = f.glyph_index('B').expect("glyph for 'B'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'B'");
    assert!(
        wires.len() >= 3,
        "Expected >= 3 wires for 'B' (outer + 2 holes), got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "Wire in 'B' must be closed");
    }
}

/// Glyph '8' has an outer contour and two inner holes.
#[test]
fn glyph_8_has_two_holes() {
    let f = face();
    let glyph_id = f.glyph_index('8').expect("glyph for '8'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for '8'");
    assert!(
        wires.len() >= 3,
        "Expected >= 3 wires for '8' (outer + 2 holes), got {}",
        wires.len()
    );
}

/// Glyph 'l' (lowercase L) has exactly 1 contour and no holes.
#[test]
fn glyph_l_has_no_holes() {
    let f = face();
    let glyph_id = f.glyph_index('l').expect("glyph for 'l'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'l'");
    assert_eq!(wires.len(), 1, "Expected exactly 1 wire for 'l'");
    assert!(wires[0].is_closed(), "Wire for 'l' must be closed");
}

/// Wires from glyph 'O' produce a valid face with boundaries matching wire count.
#[test]
fn glyph_profile_face_with_holes() {
    let f = face();
    let glyph_id = f.glyph_index('O').expect("glyph for 'O'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'O'");
    let wire_count = wires.len();
    let face: Face = profile::attach_plane_normalized(wires).expect("attach_plane_normalized");
    assert_eq!(
        face.boundaries().len(),
        wire_count,
        "Face boundary count must match wire count"
    );
}

/// Wires from glyph 'O' can be extruded into a geometrically consistent solid.
#[test]
fn glyph_profile_solid_extrusion() {
    let f = face();
    let glyph_id = f.glyph_index('O').expect("glyph for 'O'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'O'");
    let wire_count = wires.len();
    let solid =
        profile::solid_from_planar_profile::<Curve, Surface>(wires, Vector3::new(0.0, 0.0, 1.0))
            .expect("solid_from_planar_profile for 'O'");
    assert!(
        solid.is_geometric_consistent(),
        "Solid from 'O' must be geometrically consistent"
    );
    let shell = &solid.boundaries()[0];
    // 2 caps + outer_edges sides + hole_edges sides.
    // For 'O' with 2 wires, shell face count > 2.
    assert!(
        shell.len() > 2,
        "Solid shell must have more than 2 faces (caps + sides), got {}",
        shell.len()
    );
    // Verify face count: 2 caps + edges from each wire as side faces.
    if wire_count == 2 {
        // Expected: 2 caps + outer_sides + inner_sides.
        assert!(
            shell.len() >= 4,
            "Expected at least 4 faces for 'O' solid, got {}",
            shell.len()
        );
    }
}

/// Wires from glyph 'B' can be extruded into a geometrically consistent solid.
#[test]
fn glyph_b_solid_extrusion() {
    let f = face();
    let glyph_id = f.glyph_index('B').expect("glyph for 'B'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'B'");
    let solid =
        profile::solid_from_planar_profile::<Curve, Surface>(wires, Vector3::new(0.0, 0.0, 1.0))
            .expect("solid_from_planar_profile for 'B'");
    assert!(
        solid.is_geometric_consistent(),
        "Solid from 'B' must be geometrically consistent"
    );
}

/// Multi-character text profile for "HO" produces wires from both glyphs.
#[test]
fn text_profile_hello() {
    let f = face();
    let opts = default_opts();
    let wires = text::text_profile(&f, "HO", &opts).expect("text_profile for 'HO'");
    // H has outlines, O has outer + hole => total wire count > 2.
    assert!(
        wires.len() > 2,
        "Expected > 2 wires for 'HO', got {}",
        wires.len()
    );
    for w in &wires {
        assert!(
            w.is_closed(),
            "All wires in 'HO' text profile must be closed"
        );
    }
}

/// Two identical characters have horizontally offset wires.
#[test]
fn text_profile_spacing() {
    let f = face();
    let opts = default_opts();
    let wires = text::text_profile(&f, "II", &opts).expect("text_profile for 'II'");
    assert!(
        wires.len() >= 2,
        "Expected at least 2 wires for 'II', got {}",
        wires.len()
    );
    // Sample the front vertex X coordinate from first and second wire.
    let x0 = wires[0]
        .front_vertex()
        .expect("first wire has front vertex")
        .point()
        .x;
    let x1 = wires[1]
        .front_vertex()
        .expect("second wire has front vertex")
        .point()
        .x;
    assert!(
        x1 > x0,
        "Second 'I' wire must have greater X coordinate than first (x0={}, x1={})",
        x0,
        x1
    );
}

/// Space character adds horizontal advance but no wires.
#[test]
fn text_profile_space_skipped() {
    let f = face();
    let opts = default_opts();
    let wires_no_space = text::text_profile(&f, "II", &opts).expect("text_profile for 'II'");
    let wires_with_space = text::text_profile(&f, "I I", &opts).expect("text_profile for 'I I'");
    // Same wire count: space has no outline.
    assert_eq!(
        wires_no_space.len(),
        wires_with_space.len(),
        "Space should not add wires"
    );
    // But the second character's wires should be further right with the space.
    let x_no_space = wires_no_space
        .last()
        .unwrap()
        .front_vertex()
        .unwrap()
        .point()
        .x;
    let x_with_space = wires_with_space
        .last()
        .unwrap()
        .front_vertex()
        .unwrap()
        .point()
        .x;
    assert!(
        x_with_space > x_no_space,
        "With space, second 'I' must be further right (no_space={}, with_space={})",
        x_no_space,
        x_with_space
    );
}

/// Y-flip option inverts Y coordinates.
#[test]
fn glyph_profile_y_flip() {
    let f = face();
    let glyph_id = f.glyph_index('O').expect("glyph for 'O'");

    let opts_flip = text::TextOptions {
        y_flip: true,
        ..text::TextOptions::default()
    };
    let opts_no_flip = text::TextOptions {
        y_flip: false,
        ..text::TextOptions::default()
    };

    let wires_flip = text::glyph_profile(&f, glyph_id, &opts_flip).expect("y_flip=true");
    let wires_no_flip = text::glyph_profile(&f, glyph_id, &opts_no_flip).expect("y_flip=false");

    // Sample Y coordinates from the first vertex of the first wire.
    let y_flip = wires_flip[0]
        .front_vertex()
        .expect("front vertex")
        .point()
        .y;
    let y_no_flip = wires_no_flip[0]
        .front_vertex()
        .expect("front vertex")
        .point()
        .y;

    // With y_flip, Y should be negated relative to no-flip.
    assert!(
        (y_flip + y_no_flip).abs() < 1e-10,
        "Y-flip should negate Y coordinate: y_flip={}, y_no_flip={}",
        y_flip,
        y_no_flip
    );
}
