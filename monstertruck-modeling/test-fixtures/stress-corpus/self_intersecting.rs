//! Self-intersecting contour fixtures for the stress corpus.

use monstertruck_modeling::*;

/// A figure-8 contour where cubic Bezier segments cross each other.
pub fn self_intersecting_cubic() -> Vec<Wire> {
    vec![]
}

/// Two triangular loops sharing a single vertex, creating a bow-tie.
pub fn bow_tie_contour() -> Vec<Wire> {
    vec![]
}

/// Two separate closed wires whose bounding boxes overlap significantly.
pub fn overlapping_contours() -> Vec<Wire> {
    vec![]
}
