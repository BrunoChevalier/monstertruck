//! Integration test helper module that builds [`CompressedShell`] instances
//! from fixture geometries and provides loading/validation utilities.
//!
//! Imported via `mod fixture_helpers;` from integration test files.

use monstertruck_geometry::prelude::*;
use monstertruck_modeling::{Curve, Surface};
use monstertruck_topology::compress::*;
use monstertruck_topology::shell::ShellCondition;

/// All available fixture names.
pub const FIXTURE_NAMES: &[&str] = &[
    "sweep_rail_kinked",
    "birail_diverging",
    "gordon_degenerate",
    "collapsed_edge",
];

/// Dispatches to the appropriate builder by name string.
///
/// # Panics
///
/// Panics if `name` is not one of [`FIXTURE_NAMES`].
pub fn load_fixture_shell(name: &str) -> CompressedShell<Point3, Curve, Surface> {
    match name {
        "sweep_rail_kinked" => fixture_sweep_rail_kinked_shell(),
        "birail_diverging" => fixture_birail_diverging_shell(),
        "gordon_degenerate" => fixture_gordon_degenerate_shell(),
        "collapsed_edge" => fixture_collapsed_edge_shell(),
        _ => panic!("Unknown fixture name: {name}"),
    }
}

/// Checks that a shell's condition is at least [`ShellCondition::Regular`].
#[allow(dead_code)]
pub fn assert_shell_valid(shell: &monstertruck_topology::Shell<Point3, Curve, Surface>) {
    let condition = shell.shell_condition();
    assert!(
        matches!(
            condition,
            ShellCondition::Regular | ShellCondition::Oriented | ShellCondition::Closed
        ),
        "shell condition is {condition:?}, expected at least Regular"
    );
}

// ---------------------------------------------------------------------------
// Helper: build a single-face `CompressedShell` from a surface, sampling
// boundary iso-curves and introducing optional vertex gaps.
// ---------------------------------------------------------------------------

/// Builds a single-face [`CompressedShell`] from a [`BsplineSurface`] with
/// 4 boundary edges (parameter-space iso-curves). An optional `gap` offset
/// is applied to one corner vertex to simulate topology gaps.
fn single_face_shell_from_surface(
    surface: BsplineSurface<Point3>,
    gap: f64,
) -> CompressedShell<Point3, Curve, Surface> {
    // Sample the four corners.
    let (u_knots, v_knots) = surface.knot_vecs();
    let u0 = u_knots[0];
    let u1 = u_knots[u_knots.len() - 1];
    let v0 = v_knots[0];
    let v1 = v_knots[v_knots.len() - 1];

    let p00 = surface.subs(u0, v0);
    let p10 = surface.subs(u1, v0);
    let p11 = surface.subs(u1, v1);
    let p01 = surface.subs(u0, v1);

    // Introduce a small gap at vertex 0 for the second usage.
    let p00_gap = Point3::new(p00.x + gap, p00.y + gap, p00.z);

    // Vertices: 0=p00, 1=p10, 2=p11, 3=p01, 4=p00_gap (gap copy of v0).
    let vertices = vec![p00, p10, p11, p01, p00_gap];

    // Build boundary curves as linear segments (iso-curve approximations).
    let edge_bottom = CompressedEdge {
        vertices: (0, 1),
        curve: Curve::BsplineCurve(BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![p00, p10],
        )),
    };
    let edge_right = CompressedEdge {
        vertices: (1, 2),
        curve: Curve::BsplineCurve(BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![p10, p11],
        )),
    };
    let edge_top = CompressedEdge {
        vertices: (2, 3),
        curve: Curve::BsplineCurve(BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![p11, p01],
        )),
    };
    // Close with the gap vertex to simulate a topology gap.
    let edge_left = CompressedEdge {
        vertices: (3, 4),
        curve: Curve::BsplineCurve(BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![p01, p00_gap],
        )),
    };

    let edges = vec![edge_bottom, edge_right, edge_top, edge_left];

    let face = CompressedFace {
        boundaries: vec![vec![
            CompressedEdgeIndex {
                index: 0,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 1,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 2,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 3,
                orientation: true,
            },
        ]],
        orientation: true,
        surface: Surface::BsplineSurface(surface),
    };

    CompressedShell {
        vertices,
        edges,
        faces: vec![face],
    }
}

/// Builds a [`CompressedShell`] by sweeping a profile along a kinked rail.
///
/// The rail has a sharp tangent reversal at the midpoint, causing frame
/// flipping in `sweep_rail`. A small vertex gap (~1e-8) is introduced at
/// one corner.
pub fn fixture_sweep_rail_kinked_shell() -> CompressedShell<Point3, Curve, Surface> {
    // Build a simple profile (line segment in XY plane).
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(-0.5, 0.0, 0.0), Point3::new(0.5, 0.0, 0.0)],
    );
    // Kinked rail: tangent reversal at midpoint.
    let rail = BsplineCurve::new(
        KnotVector::from(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0]),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 1.0, 1.5),
            Point3::new(0.0, 1.0, 4.0),
        ],
    );
    #[allow(deprecated)]
    let surface = BsplineSurface::sweep_rail(profile, &rail, 5);
    single_face_shell_from_surface(surface, 1e-8)
}

/// Builds a [`CompressedShell`] from a birail surface using two diverging rails.
///
/// The rails diverge wildly (1.0 to 1000.0), causing extreme profile stretching.
/// Deliberate vertex offsets are introduced between adjacent edge endpoints.
pub fn fixture_birail_diverging_shell() -> CompressedShell<Point3, Curve, Surface> {
    let profile = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    let rail1 = BsplineCurve::new(
        KnotVector::bezier_knot(3),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 0.0, 3.0),
        ],
    );
    let rail2 = BsplineCurve::new(
        KnotVector::bezier_knot(3),
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(10.0, 0.0, 1.0),
            Point3::new(100.0, 0.0, 2.0),
            Point3::new(1000.0, 0.0, 3.0),
        ],
    );
    #[allow(deprecated)]
    let surface = BsplineSurface::birail1(profile, &rail1, &rail2, 5);
    single_face_shell_from_surface(surface, 1e-8)
}

/// Builds a 2-face [`CompressedShell`] from a gordon surface constructed
/// from curves that include near-collapsed control points.
///
/// The shared edge between the two faces has slightly mismatched vertex
/// positions (~1e-8 offset).
pub fn fixture_gordon_degenerate_shell() -> CompressedShell<Point3, Curve, Surface> {
    // u-curves (cross-sections).
    let u0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
    );
    let u1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![
            Point3::new(0.0, 1.0, 0.0),
            // Near-collapsed: these two points define the curve but the
            // second is almost identical to the value at the first.
            Point3::new(1.0, 1.0 + 1e-11, 0.0),
        ],
    );

    // v-curves (guides).
    let v0 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
    );
    let v1 = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0 + 1e-11, 0.0),
        ],
    );

    // Intersection grid points.
    let points = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
        vec![
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 1.0 + 1e-11, 0.0),
        ],
    ];

    #[allow(deprecated)]
    let surface = BsplineSurface::gordon(vec![u0, u1], vec![v0, v1], &points);

    // Build a 2-face shell by splitting the surface at u=0.5.
    // Sample vertices.
    let (u_knots, v_knots) = surface.knot_vecs();
    let u_start = u_knots[0];
    let u_end = u_knots[u_knots.len() - 1];
    let v_start = v_knots[0];
    let v_end = v_knots[v_knots.len() - 1];
    let u_mid = (u_start + u_end) / 2.0;

    let p00 = surface.subs(u_start, v_start);
    let pm0 = surface.subs(u_mid, v_start);
    let p10 = surface.subs(u_end, v_start);
    let p01 = surface.subs(u_start, v_end);
    let pm1 = surface.subs(u_mid, v_end);
    let p11 = surface.subs(u_end, v_end);

    // Introduce a mismatch on the shared edge midpoint vertices.
    let pm0_b = Point3::new(pm0.x + 1e-8, pm0.y, pm0.z);
    let pm1_b = Point3::new(pm1.x + 1e-8, pm1.y, pm1.z);

    // Vertices: 0=p00, 1=pm0, 2=p10, 3=p01, 4=pm1, 5=p11, 6=pm0_b, 7=pm1_b.
    let vertices = vec![p00, pm0, p10, p01, pm1, p11, pm0_b, pm1_b];

    let make_line_curve = |a: Point3, b: Point3| -> Curve {
        Curve::BsplineCurve(BsplineCurve::new(KnotVector::bezier_knot(1), vec![a, b]))
    };

    // Edge layout:
    //   0: bottom-left  (0 -> 1)
    //   1: shared-mid   (1 -> 4) -- used forward by face 0, reversed by face 1
    //   2: top-left     (4 -> 3)
    //   3: left          (3 -> 0)
    //   4: bottom-right (6 -> 2)  -- vertex 6 ~= vertex 1 after welding
    //   5: right         (2 -> 5)
    //   6: top-right    (5 -> 7)  -- vertex 7 ~= vertex 4 after welding
    //
    // Face 1 references edge 1 in reverse orientation. After welding merges
    // vertices 6->1 and 7->4, edges 4/6 get consistent vertex indices and the
    // shared edge becomes manifold.
    let edges = vec![
        // Face 0 edges.
        CompressedEdge {
            vertices: (0, 1),
            curve: make_line_curve(p00, pm0),
        },
        // Shared edge (forward = face 0 direction: 1 -> 4).
        CompressedEdge {
            vertices: (1, 4),
            curve: make_line_curve(pm0, pm1),
        },
        CompressedEdge {
            vertices: (4, 3),
            curve: make_line_curve(pm1, p01),
        },
        CompressedEdge {
            vertices: (3, 0),
            curve: make_line_curve(p01, p00),
        },
        // Face 1 edges (shared edge is edge 1 reversed).
        CompressedEdge {
            vertices: (6, 2),
            curve: make_line_curve(pm0_b, p10),
        },
        CompressedEdge {
            vertices: (2, 5),
            curve: make_line_curve(p10, p11),
        },
        CompressedEdge {
            vertices: (5, 7),
            curve: make_line_curve(p11, pm1_b),
        },
    ];

    let face0 = CompressedFace {
        boundaries: vec![vec![
            CompressedEdgeIndex {
                index: 0,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 1,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 2,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 3,
                orientation: true,
            },
        ]],
        orientation: true,
        surface: Surface::BsplineSurface(surface.clone()),
    };

    let face1 = CompressedFace {
        boundaries: vec![vec![
            // Shared edge reversed (4 -> 1 direction, i.e. edge 1 with orientation=false).
            CompressedEdgeIndex {
                index: 1,
                orientation: false,
            },
            CompressedEdgeIndex {
                index: 4,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 5,
                orientation: true,
            },
            CompressedEdgeIndex {
                index: 6,
                orientation: true,
            },
        ]],
        orientation: true,
        surface: Surface::BsplineSurface(surface),
    };

    CompressedShell {
        vertices,
        edges,
        faces: vec![face0, face1],
    }
}

/// Builds a [`CompressedShell`] with a collapsed/degenerate edge: one face
/// boundary contains an edge whose two vertices are the same point (from
/// a surface pole).
pub fn fixture_collapsed_edge_shell() -> CompressedShell<Point3, Curve, Surface> {
    // Use the degenerate surface with collapsed edge from the geometry fixtures.
    let surface = monstertruck_geometry::nurbs::test_fixtures::degenerate_surface_collapsed_edge();

    let (u_knots, v_knots) = surface.knot_vecs();
    let u0 = u_knots[0];
    let u1 = u_knots[u_knots.len() - 1];
    let v0 = v_knots[0];
    let v1 = v_knots[v_knots.len() - 1];

    // The surface has a collapsed edge at u=0 (all control points in first
    // row are identical). Sample corners.
    let p_top = surface.subs(u0, v0);
    // SAFETY: u=0 is the collapsed edge, so p_top == surface.subs(u0, v1).
    let p_bottom_left = surface.subs(u1, v0);
    let p_bottom_right = surface.subs(u1, v1);

    // Vertices: 0=tip (collapsed), 1=bottom_left, 2=bottom_right.
    let vertices = vec![p_top, p_bottom_left, p_bottom_right];

    let make_line_curve = |a: Point3, b: Point3| -> Curve {
        Curve::BsplineCurve(BsplineCurve::new(KnotVector::bezier_knot(1), vec![a, b]))
    };

    let edges = vec![
        // Degenerate edge: collapsed (same vertex).
        CompressedEdge {
            vertices: (0, 0),
            curve: make_line_curve(p_top, p_top),
        },
        // Bottom edge.
        CompressedEdge {
            vertices: (1, 2),
            curve: make_line_curve(p_bottom_left, p_bottom_right),
        },
        // Left edge (tip to bottom-left).
        CompressedEdge {
            vertices: (0, 1),
            curve: make_line_curve(p_top, p_bottom_left),
        },
        // Right edge (bottom-right to tip).
        CompressedEdge {
            vertices: (2, 0),
            curve: make_line_curve(p_bottom_right, p_top),
        },
    ];

    let face = CompressedFace {
        boundaries: vec![vec![
            // Degenerate edge at top.
            CompressedEdgeIndex {
                index: 0,
                orientation: true,
            },
            // Left side down.
            CompressedEdgeIndex {
                index: 2,
                orientation: true,
            },
            // Bottom.
            CompressedEdgeIndex {
                index: 1,
                orientation: true,
            },
            // Right side up.
            CompressedEdgeIndex {
                index: 3,
                orientation: true,
            },
        ]],
        orientation: true,
        surface: Surface::BsplineSurface(surface),
    };

    CompressedShell {
        vertices,
        edges,
        faces: vec![face],
    }
}
