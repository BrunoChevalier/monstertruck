//! Self-intersecting contour fixtures for the stress corpus.

use monstertruck_modeling::*;

/// A figure-8 contour where cubic Bezier segments cross each other.
///
/// Constructs a single closed wire whose cubic Bezier edges form
/// a self-intersecting figure-8 shape. The control points are arranged
/// so that the curve crosses itself at the origin.
///
/// **Failure mode:** Tests whether profile normalization handles
/// self-intersection without panicking. The signed-area computation
/// may produce unexpected results when the contour crosses itself.
///
/// **Real-world analog:** Similar to certain CJK glyphs with complex
/// stroke crossings, or decorative script fonts with looping strokes.
pub fn self_intersecting_cubic() -> Vec<Wire> {
    // Figure-8: top loop goes right then curves back left,
    // bottom loop goes left then curves back right.
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v2 = builder::vertex(Point3::new(0.0, 2.0, 0.0));
    let v3 = builder::vertex(Point3::new(-1.0, 1.0, 0.0));

    // Top loop: v0 -> v1 with control points that curve right.
    let e0: Edge = builder::bezier(
        &v0,
        &v1,
        vec![
            Point3::new(1.5, 0.0, 0.0),
            Point3::new(1.5, 1.0, 0.0),
        ],
    );
    // Right to top: v1 -> v2.
    let e1: Edge = builder::bezier(
        &v1,
        &v2,
        vec![
            Point3::new(1.5, 1.5, 0.0),
            Point3::new(0.5, 2.0, 0.0),
        ],
    );
    // Top to left: v2 -> v3 -- crosses through center.
    let e2: Edge = builder::bezier(
        &v2,
        &v3,
        vec![
            Point3::new(-0.5, 2.0, 0.0),
            Point3::new(-1.5, 1.5, 0.0),
        ],
    );
    // Left back to origin: v3 -> v0 -- crosses through center.
    let e3: Edge = builder::bezier(
        &v3,
        &v0,
        vec![
            Point3::new(-1.5, 0.5, 0.0),
            Point3::new(-0.5, 0.0, 0.0),
        ],
    );

    let wire: Wire = vec![e0, e1, e2, e3].into();
    vec![wire]
}

/// Two triangular loops sharing a single vertex, creating a bow-tie.
///
/// Constructs a single self-intersecting wire that forms a bow-tie shape:
/// two triangles meeting at a shared vertex. The wire path crosses itself
/// at the shared point.
///
/// **Failure mode:** Tests whether the pipeline handles contours that
/// pass through the same point more than once. The shared vertex creates
/// ambiguity in inside/outside classification.
///
/// **Real-world analog:** Certain decorative fonts have strokes that
/// touch or cross at a single point, particularly in serif terminals.
pub fn bow_tie_contour() -> Vec<Wire> {
    // Two triangles sharing the origin.
    // Triangle 1: (0,0) -> (1,1) -> (-1,1) -> (0,0)
    // Triangle 2: (0,0) -> (1,-1) -> (-1,-1) -> (0,0)
    // Combined into one wire path that crosses at origin.
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v2 = builder::vertex(Point3::new(-1.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(1.0, -1.0, 0.0));
    let v4 = builder::vertex(Point3::new(-1.0, -1.0, 0.0));

    let e0: Edge = builder::line(&v0, &v1);
    let e1: Edge = builder::line(&v1, &v2);
    let e2: Edge = builder::line(&v2, &v0);
    let e3: Edge = builder::line(&v0, &v3);
    let e4: Edge = builder::line(&v3, &v4);
    let e5: Edge = builder::line(&v4, &v0);

    let wire: Wire = vec![e0, e1, e2, e3, e4, e5].into();
    vec![wire]
}

/// Two separate closed wires whose bounding boxes overlap significantly.
///
/// Returns two rectangular wires offset by half their width so that
/// they overlap. Tests containment classification when contours are
/// separate but spatially overlapping.
///
/// **Failure mode:** Tests whether the containment classifier correctly
/// determines that neither rectangle contains the other (they are
/// siblings, not parent/child). Overlap can cause point-in-polygon
/// tests to give misleading results.
///
/// **Real-world analog:** Ligature glyphs or composed characters where
/// separate strokes overlap (e.g., `fi` ligature, `AE` digraph).
pub fn overlapping_contours() -> Vec<Wire> {
    // Rectangle 1: (0,0) to (2,2).
    let r1 = make_rectangle(0.0, 0.0, 2.0, 2.0);
    // Rectangle 2: (1,0) to (3,2) -- overlaps by half width.
    let r2 = make_rectangle(1.0, 0.0, 3.0, 2.0);
    vec![r1, r2]
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
