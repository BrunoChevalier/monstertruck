//! Deeply nested hole fixtures for the stress corpus.

use monstertruck_modeling::*;

/// Five concentric rectangular wires simulating deeply nested containment.
///
/// Returns 5 wires representing the hierarchy:
///   outer (10x10) > hole (8x8) > island (6x6) > inner-hole (4x4) > innermost-island (2x2).
///
/// All rectangles are centered at the origin with decreasing sizes.
/// The profile normalization must correctly classify each level as
/// either outer or hole based on nesting depth parity.
///
/// **Failure mode:** Tests whether the containment classifier handles
/// 3+ levels of nesting. Most simple classifiers only handle outer+hole
/// (2 levels) and may fail on deeper hierarchies.
///
/// **Real-world analog:** The `@` symbol, which has multiple nested
/// contours. Also relevant for CJK characters with complex radical
/// nesting.
pub fn deeply_nested_holes() -> Vec<Wire> {
    [10.0, 8.0, 6.0, 4.0, 2.0]
        .iter()
        .map(|&size| {
            let half = size / 2.0;
            make_centered_rectangle(half)
        })
        .collect()
}

/// Twenty-plus small non-overlapping rectangular wires in a grid
/// inside a large outer rectangle.
///
/// Returns 1 large outer rectangle plus 20 small rectangles arranged
/// in a 4x5 grid pattern. Tests scalability of containment
/// classification with many contours.
///
/// **Failure mode:** Tests whether the O(n^2) containment checks
/// remain tractable with many wires. Also tests that the classifier
/// correctly identifies all small rectangles as holes within the
/// outer rectangle.
///
/// **Real-world analog:** Perforated or dotted patterns in decorative
/// fonts, or bitmap-style fonts with many small features.
pub fn high_loop_count() -> Vec<Wire> {
    let mut wires = Vec::with_capacity(21);

    // Outer rectangle: 0..20 x 0..25.
    wires.push(make_rectangle(0.0, 0.0, 20.0, 25.0));

    // 4x5 grid of small rectangles inside the outer one.
    for row in 0..5 {
        for col in 0..4 {
            let x0 = 1.0 + col as f64 * 5.0;
            let y0 = 1.0 + row as f64 * 5.0;
            wires.push(make_rectangle(x0, y0, x0 + 3.0, y0 + 3.0));
        }
    }

    wires
}

/// Helper: constructs a closed rectangular wire centered at origin.
fn make_centered_rectangle(half: f64) -> Wire {
    make_rectangle(-half, -half, half, half)
}

/// Helper: constructs a closed rectangular wire from corner coordinates.
fn make_rectangle(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    let v0 = builder::vertex(Point3::new(x0, y0, 0.0));
    let v1 = builder::vertex(Point3::new(x1, y0, 0.0));
    let v2 = builder::vertex(Point3::new(x1, y1, 0.0));
    let v3 = builder::vertex(Point3::new(x0, y1, 0.0));

    let e0: Edge = builder::line(&v0, &v1);
    let e1: Edge = builder::line(&v1, &v2);
    let e2: Edge = builder::line(&v2, &v3);
    let e3: Edge = builder::line(&v3, &v0);

    vec![e0, e1, e2, e3].into()
}
