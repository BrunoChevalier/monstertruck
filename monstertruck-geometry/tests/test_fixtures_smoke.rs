//! Smoke tests for the fixture corpus in [`monstertruck_geometry::nurbs::test_fixtures`].

use monstertruck_geometry::nurbs::test_fixtures;

#[test]
fn degenerate_collapsed_control_points_valid() {
    let curve = test_fixtures::degenerate_collapsed_control_points();
    assert_eq!(curve.degree(), 3);
    assert_eq!(curve.control_points().len(), 4);
}

#[test]
fn degenerate_near_zero_knot_span_valid() {
    let curve = test_fixtures::degenerate_near_zero_knot_span();
    assert_eq!(curve.degree(), 3);
    assert_eq!(curve.control_points().len(), 6);
}

#[test]
fn degenerate_high_curvature_pole_valid() {
    let curve = test_fixtures::degenerate_high_curvature_pole();
    assert_eq!(curve.degree(), 3);
    assert_eq!(curve.control_points().len(), 4);
}

#[test]
fn degenerate_surface_collapsed_edge_valid() {
    let surface = test_fixtures::degenerate_surface_collapsed_edge();
    assert_eq!(surface.degrees(), (2, 2));
    assert_eq!(surface.control_points().len(), 3);
    assert_eq!(surface.control_points()[0].len(), 3);
}

#[test]
fn kinked_rail_valid() {
    let curve = test_fixtures::fixture_kinked_rail();
    assert_eq!(curve.degree(), 3);
    assert_eq!(curve.control_points().len(), 5);
}

#[test]
fn diverging_rails_valid() {
    let (rail1, rail2) = test_fixtures::fixture_diverging_rails();
    assert_eq!(rail1.degree(), 3);
    assert_eq!(rail2.degree(), 3);
    assert_eq!(rail1.control_points().len(), 4);
    assert_eq!(rail2.control_points().len(), 4);
}

#[test]
fn self_intersecting_profile_valid() {
    let curve = test_fixtures::fixture_self_intersecting_profile();
    assert_eq!(curve.degree(), 3);
    assert_eq!(curve.control_points().len(), 8);
}

#[test]
fn closed_rail_valid() {
    let curve = test_fixtures::fixture_closed_rail();
    assert_eq!(curve.degree(), 3);
    assert_eq!(curve.control_points().len(), 7);
}

#[test]
fn glyph_sharp_corners_valid() {
    let curves = test_fixtures::fixture_glyph_sharp_corners();
    assert_eq!(curves.len(), 6);
    curves.iter().for_each(|c| {
        assert_eq!(c.degree(), 1);
        assert_eq!(c.control_points().len(), 2);
    });
}

#[test]
fn glyph_nested_contours_valid() {
    let contours = test_fixtures::fixture_glyph_nested_contours();
    assert_eq!(contours.len(), 2);
    assert_eq!(contours[0].len(), 4);
    assert_eq!(contours[1].len(), 4);
    contours.iter().flatten().for_each(|c| {
        assert_eq!(c.degree(), 1);
        assert_eq!(c.control_points().len(), 2);
    });
}
