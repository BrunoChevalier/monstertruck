//! Stress corpus of pathological font geometry fixtures.
//!
//! Provides synthetic pathological font geometry for regression testing.
//! Each fixture constructs [`Wire`] objects that simulate problematic
//! font outlines -- self-intersections, near-zero areas, deep nesting,
//! and degenerate geometry.
//!
//! Run the tests with:
//! ```bash
//! cargo nextest run -p monstertruck-modeling --features font font_stress
//! ```

use monstertruck_modeling::Wire;

pub mod degenerate;
pub mod deeply_nested;
pub mod near_zero_area;
pub mod self_intersecting;

/// Returns all named fixtures in the corpus as `(name, wires)` pairs.
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
