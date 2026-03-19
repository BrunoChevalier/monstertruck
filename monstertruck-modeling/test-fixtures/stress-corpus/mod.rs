//! Stress corpus of pathological font geometry fixtures.

use monstertruck_modeling::Wire;

pub mod self_intersecting;
pub mod near_zero_area;
pub mod deeply_nested;
pub mod degenerate;

/// Returns all named fixtures in the corpus.
pub fn all_fixtures() -> Vec<(&'static str, Vec<Wire>)> {
    vec![
        ("self_intersecting_cubic", self_intersecting::self_intersecting_cubic()),
        ("bow_tie_contour", self_intersecting::bow_tie_contour()),
        ("overlapping_contours", self_intersecting::overlapping_contours()),
        ("near_zero_area_sliver", near_zero_area::near_zero_area_sliver()),
        ("collapsed_quad_bezier", near_zero_area::collapsed_quad_bezier()),
        ("micro_feature_loop", near_zero_area::micro_feature_loop()),
        ("deeply_nested_holes", deeply_nested::deeply_nested_holes()),
        ("high_loop_count", deeply_nested::high_loop_count()),
        ("coincident_control_points", degenerate::coincident_control_points()),
        ("reverse_wound_hole", degenerate::reverse_wound_hole()),
        ("single_point_degeneracy", degenerate::single_point_degeneracy()),
    ]
}
