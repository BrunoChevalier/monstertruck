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

// ---------------------------------------------------------------------------
// Helpers for mixed glyph + custom profile tests
// ---------------------------------------------------------------------------

/// Builds a rectangular wire in the XY plane using the same pattern as
/// `profile_test.rs`.
fn rect_wire(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, y0, 0.0));
    let v1 = builder::vertex(Point3::new(x1, y0, 0.0));
    let v2 = builder::vertex(Point3::new(x1, y1, 0.0));
    let v3 = builder::vertex(Point3::new(x0, y1, 0.0));
    vec![
        builder::line(&v0, &v1),
        builder::line(&v1, &v2),
        builder::line(&v2, &v3),
        builder::line(&v3, &v0),
    ]
    .into()
}

/// Large rectangle wire sized to contain font glyphs at `scale = 1.0`
/// (font-unit coordinates, typically 0..~2000 range).
fn large_rect_wire() -> Wire {
    rect_wire(-500.0, -1500.0, 2500.0, 2500.0)
}

/// Options that disable the per-em normalization so glyph coordinates stay in
/// raw font units (typically 0..~2048).
fn font_unit_opts() -> text::TextOptions {
    text::TextOptions {
        scale: Some(1.0),
        ..text::TextOptions::default()
    }
}

// ---------------------------------------------------------------------------
// Mixed glyph + custom profile integration tests
// ---------------------------------------------------------------------------

/// A custom large outer rectangle with glyph 'O' hole contours merged in.
/// The face boundary count should equal the total wire count.
#[test]
fn mixed_glyph_custom_outer_with_glyph_holes() {
    let f = face();
    let glyph_id = f.glyph_index('O').expect("glyph for 'O'");
    let opts = font_unit_opts();
    let glyph_wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile");

    // Take only the hole contours from the glyph (skip the outer).
    // For 'O', wires[0] is typically the outer, wires[1..] are holes.
    let glyph_holes: Vec<Wire> = glyph_wires.into_iter().skip(1).collect();
    assert!(
        !glyph_holes.is_empty(),
        "'O' must have at least one hole contour"
    );

    let outer = large_rect_wire();
    let custom_set = vec![outer];
    let merged = profile::merge_profiles(vec![custom_set, glyph_holes.clone()]);
    let expected_count = 1 + glyph_holes.len();
    assert_eq!(merged.len(), expected_count);

    let face: Face =
        profile::attach_plane_normalized(merged).expect("attach_plane_normalized mixed");
    assert_eq!(
        face.boundaries().len(),
        expected_count,
        "Face boundary count must match merged wire count"
    );
}

/// A custom outer rectangle with glyph 'l' (single contour, no holes) merged.
/// The glyph wire should be classified as a hole inside the larger rectangle.
#[test]
fn mixed_glyph_custom_face_construction() {
    let f = face();
    let glyph_id = f.glyph_index('l').expect("glyph for 'l'");
    let opts = font_unit_opts();
    let glyph_wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'l'");
    assert_eq!(glyph_wires.len(), 1);

    let outer = large_rect_wire();
    let merged = profile::merge_profiles(vec![vec![outer], glyph_wires]);
    let face: Face = profile::attach_plane_normalized(merged).expect("attach_plane_normalized");
    assert_eq!(
        face.boundaries().len(),
        2,
        "Expected 2 boundaries: outer rectangle + glyph 'l' hole"
    );
}

/// A large custom outer rectangle with multiple glyph outlines ('I' and 'l')
/// positioned via `text_profile("Il")`. All glyph wires should be classified
/// as holes inside the outer rectangle. Expect 3 boundaries (1 outer + 2 holes).
#[test]
fn mixed_multiple_glyphs_as_holes() {
    let f = face();
    let opts = font_unit_opts();
    let glyph_wires = text::text_profile(&f, "Il", &opts).expect("text_profile for 'Il'");
    // 'I' and 'l' each have 1 contour.
    assert_eq!(
        glyph_wires.len(),
        2,
        "Expected 2 wires for 'Il', got {}",
        glyph_wires.len()
    );

    let outer = large_rect_wire();
    let merged = profile::merge_profiles(vec![vec![outer], glyph_wires]);
    let face: Face = profile::attach_plane_normalized(merged).expect("attach_plane_normalized");
    assert_eq!(
        face.boundaries().len(),
        3,
        "Expected 3 boundaries: 1 outer + 2 glyph holes"
    );
}

/// Combine a custom outer rectangle with glyph 'O' holes, create a face,
/// then extrude to a solid. The solid must be geometrically consistent.
#[test]
fn mixed_glyph_custom_solid_extrusion() {
    let f = face();
    let glyph_id = f.glyph_index('O').expect("glyph for 'O'");
    let opts = font_unit_opts();
    let glyph_wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for 'O'");

    // Take only the hole contours.
    let glyph_holes: Vec<Wire> = glyph_wires.into_iter().skip(1).collect();

    let outer = large_rect_wire();
    let merged = profile::merge_profiles(vec![vec![outer], glyph_holes]);
    let solid = profile::solid_from_planar_profile::<Curve, Surface>(
        merged,
        Vector3::new(0.0, 0.0, 1.0),
    )
    .expect("solid_from_planar_profile mixed");
    assert!(
        solid.is_geometric_consistent(),
        "Mixed glyph+custom solid must be geometrically consistent"
    );
}

/// `merge_profiles` with two non-empty sets returns the combined count.
#[test]
fn merge_profiles_basic() {
    let w1 = rect_wire(0.0, 0.0, 1.0, 1.0);
    let w2 = rect_wire(2.0, 0.0, 3.0, 1.0);
    let merged = profile::merge_profiles(vec![vec![w1], vec![w2]]);
    assert_eq!(merged.len(), 2);
}

/// `merge_profiles` with a non-empty set and an empty set returns the first
/// set's count.
#[test]
fn merge_profiles_empty_second() {
    let w1 = rect_wire(0.0, 0.0, 1.0, 1.0);
    let merged = profile::merge_profiles(vec![vec![w1], vec![]]);
    assert_eq!(merged.len(), 1);
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
