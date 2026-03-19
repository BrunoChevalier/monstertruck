//! Degenerate geometry fixtures for the stress corpus.

use monstertruck_modeling::*;

/// A closed contour of cubic Bezier edges where all control points
/// coincide with one endpoint.
pub fn coincident_control_points() -> Vec<Wire> {
    vec![]
}

/// An outer CCW rectangle with an inner CW rectangle (pre-wound hole).
pub fn reverse_wound_hole() -> Vec<Wire> {
    vec![]
}

/// A wire where two consecutive vertices are at the same point.
pub fn single_point_degeneracy() -> Vec<Wire> {
    vec![]
}
