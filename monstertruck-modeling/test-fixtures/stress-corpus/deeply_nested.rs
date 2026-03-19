//! Deeply nested hole fixtures for the stress corpus.

use super::make_rectangle;
use monstertruck_modeling::Wire;

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
            make_rectangle(-half, -half, half, half)
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
    let outer = std::iter::once(make_rectangle(0.0, 0.0, 20.0, 25.0));

    // 4x5 grid of small rectangles inside the outer one.
    let inner = (0..5).flat_map(|row| {
        (0..4).map(move |col| {
            let x0 = 1.0 + col as f64 * 5.0;
            let y0 = 1.0 + row as f64 * 5.0;
            make_rectangle(x0, y0, x0 + 3.0, y0 + 3.0)
        })
    });

    outer.chain(inner).collect()
}
