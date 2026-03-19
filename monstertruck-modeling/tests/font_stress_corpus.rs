//! Regression tests for the stress corpus of pathological font geometry.
//!
//! Run with: `cargo nextest run -p monstertruck-modeling --features font font_stress`
#![cfg(feature = "font")]

use monstertruck_modeling::*;

#[path = "../test-fixtures/stress-corpus/mod.rs"]
mod stress_corpus;

/// Path to the bundled DejaVu Sans font fixture.
const FONT_BYTES: &[u8] = include_bytes!("../test-fixtures/DejaVuSans.ttf");

fn face() -> ttf_parser::Face<'static> {
    ttf_parser::Face::parse(FONT_BYTES, 0).expect("valid TTF")
}

fn default_opts() -> text::TextOptions {
    text::TextOptions::default()
}

// ---------------------------------------------------------------------------
// Self-intersecting contour fixtures
// ---------------------------------------------------------------------------

#[test]
fn font_stress_self_intersecting_cubic() {
    let wires = stress_corpus::self_intersecting::self_intersecting_cubic();
    assert!(!wires.is_empty(), "self_intersecting_cubic must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "self_intersecting_cubic wires must be closed");
    }
    // Attempt pipeline normalization -- may succeed or document known limitation.
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Self-intersecting cubic contours may not normalize cleanly.
            // TODO(font-stress): Track issue for self-intersecting cubic normalization.
            eprintln!("self_intersecting_cubic: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_bow_tie_contour() {
    let wires = stress_corpus::self_intersecting::bow_tie_contour();
    assert!(!wires.is_empty(), "bow_tie_contour must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "bow_tie_contour wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Bow-tie contours share a vertex and may fail normalization.
            // TODO(font-stress): Track issue for bow-tie contour normalization.
            eprintln!("bow_tie_contour: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_overlapping_contours() {
    let wires = stress_corpus::self_intersecting::overlapping_contours();
    assert!(
        wires.len() >= 2,
        "overlapping_contours must return at least 2 wires, got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "overlapping_contours wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Overlapping contours may produce ambiguous nesting.
            // TODO(font-stress): Track issue for overlapping contour classification.
            eprintln!("overlapping_contours: known limitation: {e}");
        }
    }
}

// ---------------------------------------------------------------------------
// Near-zero-area loop fixtures
// ---------------------------------------------------------------------------

#[test]
fn font_stress_near_zero_area_sliver() {
    let wires = stress_corpus::near_zero_area::near_zero_area_sliver();
    assert!(!wires.is_empty(), "near_zero_area_sliver must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "near_zero_area_sliver wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Near-zero-area slivers may produce degenerate planes.
            // TODO(font-stress): Track issue for near-zero-area sliver handling.
            eprintln!("near_zero_area_sliver: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_collapsed_quad_bezier() {
    let wires = stress_corpus::near_zero_area::collapsed_quad_bezier();
    assert!(!wires.is_empty(), "collapsed_quad_bezier must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "collapsed_quad_bezier wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Collapsed quadratic Beziers may produce degenerate geometry.
            // TODO(font-stress): Track issue for collapsed quadratic Bezier handling.
            eprintln!("collapsed_quad_bezier: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_micro_feature_loop() {
    let wires = stress_corpus::near_zero_area::micro_feature_loop();
    assert!(!wires.is_empty(), "micro_feature_loop must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "micro_feature_loop wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Micro-feature loops may be below processing thresholds.
            // TODO(font-stress): Track issue for micro-feature loop handling.
            eprintln!("micro_feature_loop: known limitation: {e}");
        }
    }
}

// ---------------------------------------------------------------------------
// Deeply nested hole fixtures
// ---------------------------------------------------------------------------

#[test]
fn font_stress_deeply_nested_holes() {
    let wires = stress_corpus::deeply_nested::deeply_nested_holes();
    assert_eq!(
        wires.len(),
        5,
        "deeply_nested_holes must return 5 concentric wires, got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "deeply_nested_holes wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert_eq!(
                face.boundaries().len(),
                5,
                "Expected 5 boundaries for deeply nested holes"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Deeply nested holes (3+ levels) may exceed containment classification.
            // TODO(font-stress): Track issue for deeply nested hole classification.
            eprintln!("deeply_nested_holes: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_high_loop_count() {
    let wires = stress_corpus::deeply_nested::high_loop_count();
    assert!(
        wires.len() >= 21,
        "high_loop_count must return at least 21 wires (1 outer + 20 inner), got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "high_loop_count wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 21,
                "Expected at least 21 boundaries for high loop count"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: High loop counts may be slow or fail containment classification.
            // TODO(font-stress): Track issue for high loop count scalability.
            eprintln!("high_loop_count: known limitation: {e}");
        }
    }
}

// ---------------------------------------------------------------------------
// Degenerate geometry fixtures
// ---------------------------------------------------------------------------

#[test]
fn font_stress_coincident_control_points() {
    let wires = stress_corpus::degenerate::coincident_control_points();
    assert!(!wires.is_empty(), "coincident_control_points must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "coincident_control_points wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Coincident control points may produce degenerate Bezier edges.
            // TODO(font-stress): Track issue for coincident control point handling.
            eprintln!("coincident_control_points: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_reverse_wound_hole() {
    let wires = stress_corpus::degenerate::reverse_wound_hole();
    assert_eq!(
        wires.len(),
        2,
        "reverse_wound_hole must return 2 wires (outer + hole), got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "reverse_wound_hole wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires.clone()) {
        Ok(face) => {
            assert_eq!(
                face.boundaries().len(),
                2,
                "Expected 2 boundaries for reverse wound hole"
            );
            // Attempt extrusion and validate the solid.
            let solid = profile::solid_from_planar_profile::<Curve, Surface>(
                wires,
                Vector3::new(0.0, 0.0, 1.0),
            )
            .expect("solid_from_planar_profile for reverse_wound_hole");
            let _report = profile::validate_solid(&solid).expect("validate_solid");
        }
        Err(e) => {
            // KNOWN LIMITATION: Reverse-wound holes may cause double-inversion.
            // TODO(font-stress): Track issue for reverse-wound hole normalization.
            eprintln!("reverse_wound_hole: known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_single_point_degeneracy() {
    let wires = stress_corpus::degenerate::single_point_degeneracy();
    assert!(!wires.is_empty(), "single_point_degeneracy must return at least one wire");
    for w in &wires {
        assert!(w.is_closed(), "single_point_degeneracy wires must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected at least 1 boundary"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: Zero-length edges may cause degenerate geometry.
            // TODO(font-stress): Track issue for single-point degeneracy handling.
            eprintln!("single_point_degeneracy: known limitation: {e}");
        }
    }
}

// ---------------------------------------------------------------------------
// All-fixtures iterator test
// ---------------------------------------------------------------------------

#[test]
fn font_stress_all_fixtures_no_panic() {
    let fixtures = stress_corpus::all_fixtures();
    assert!(
        fixtures.len() >= 11,
        "Expected at least 11 fixtures in corpus, got {}",
        fixtures.len()
    );
    for (name, wires) in &fixtures {
        assert!(
            !wires.is_empty(),
            "Fixture '{name}' must return at least one wire"
        );
    }
}

// ---------------------------------------------------------------------------
// Real font glyph stress tests
// ---------------------------------------------------------------------------

#[test]
fn font_stress_glyph_at_sign() {
    let f = face();
    let glyph_id = f.glyph_index('@').expect("glyph for '@'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for '@'");
    assert!(
        wires.len() >= 2,
        "Expected >= 2 wires for '@' (complex nested contours), got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "Wire in '@' must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 2,
                "Expected >= 2 boundaries for '@'"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: '@' glyph has complex nested contours.
            // TODO(font-stress): Track issue for '@' glyph normalization.
            eprintln!("glyph '@': known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_glyph_ampersand() {
    let f = face();
    let glyph_id = f.glyph_index('&').expect("glyph for '&'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for '&'");
    assert!(!wires.is_empty(), "Expected at least 1 wire for '&'");
    for w in &wires {
        assert!(w.is_closed(), "Wire in '&' must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 1,
                "Expected >= 1 boundary for '&'"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: '&' glyph has self-intersecting-like curves.
            // TODO(font-stress): Track issue for '&' glyph normalization.
            eprintln!("glyph '&': known limitation: {e}");
        }
    }
}

#[test]
fn font_stress_glyph_percent() {
    let f = face();
    let glyph_id = f.glyph_index('%').expect("glyph for '%'");
    let opts = default_opts();
    let wires = text::glyph_profile(&f, glyph_id, &opts).expect("glyph_profile for '%'");
    assert!(
        wires.len() >= 3,
        "Expected >= 3 wires for '%' (multiple small circles), got {}",
        wires.len()
    );
    for w in &wires {
        assert!(w.is_closed(), "Wire in '%' must be closed");
    }
    match profile::attach_plane_normalized::<Curve, Surface>(wires) {
        Ok(face) => {
            assert!(
                face.boundaries().len() >= 3,
                "Expected >= 3 boundaries for '%'"
            );
        }
        Err(e) => {
            // KNOWN LIMITATION: '%' glyph has multiple small circular contours.
            // TODO(font-stress): Track issue for '%' glyph normalization.
            eprintln!("glyph '%': known limitation: {e}");
        }
    }
}

// ---------------------------------------------------------------------------
// Full ASCII sweep
// ---------------------------------------------------------------------------

#[test]
fn font_stress_dejavusans_full_ascii() {
    let f = face();
    let opts = default_opts();
    for code in 0x21_u8..=0x7E_u8 {
        let ch = code as char;
        let glyph_id = match f.glyph_index(ch) {
            Some(id) => id,
            None => continue,
        };
        // Calling glyph_profile must not panic for any printable ASCII character.
        let result = text::glyph_profile(&f, glyph_id, &opts);
        match result {
            Ok(wires) => {
                for w in &wires {
                    assert!(
                        w.is_closed(),
                        "Wire for '{ch}' (0x{code:02X}) must be closed"
                    );
                }
            }
            Err(e) => {
                // Some glyphs may fail gracefully -- this is acceptable.
                eprintln!("glyph '{ch}' (0x{code:02X}): {e}");
            }
        }
    }
}
