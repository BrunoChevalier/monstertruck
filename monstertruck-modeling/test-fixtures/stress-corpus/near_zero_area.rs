//! Near-zero-area loop fixtures for the stress corpus.

use monstertruck_modeling::*;

/// A long, extremely thin rectangular wire with near-zero enclosed area.
pub fn near_zero_area_sliver() -> Vec<Wire> {
    vec![]
}

/// A contour built from quadratic Beziers where control points nearly
/// coincide with the endpoints.
pub fn collapsed_quad_bezier() -> Vec<Wire> {
    vec![]
}

/// A tiny closed contour simulating a sub-pixel glyph feature.
pub fn micro_feature_loop() -> Vec<Wire> {
    vec![]
}
