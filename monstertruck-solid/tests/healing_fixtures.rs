//! Integration tests exercising the fixture corpus through the healing pipeline.
//!
//! Each test loads a degenerate-geometry fixture, passes it through
//! [`heal_surface_shell`], and verifies that healing code paths are exercised
//! (gap welding, degenerate edge removal) without panics or timeouts.
//!
//! Single-face open shells legitimately produce `NonManifoldEdges` because
//! boundary edges appear only once. The tests accept this as a valid healing
//! outcome (proving welding and degenerate removal ran). Multi-face shells
//! with shared edges (like the gordon fixture) are expected to pass the
//! manifold check and produce `Ok` with valid [`ShellCondition`].

mod fixture_helpers;

use fixture_helpers::{FIXTURE_NAMES, load_fixture_shell};
use monstertruck_modeling::{Curve, Surface};
use monstertruck_solid::{SurfaceHealingError, heal_surface_shell};
use monstertruck_topology::compress::CompressedShell;
use monstertruck_topology::shell::ShellCondition;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc;
use std::time::Duration;

/// Healing tolerance used across all tests.
const TOL: f64 = 0.05;

/// Extracts a human-readable message from a `catch_unwind` panic payload.
fn panic_message(payload: &Box<dyn std::any::Any + Send>) -> &str {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap_or("<non-string panic>")
}

/// Helper: run `heal_surface_shell` on a single-face open shell. Accepts
/// `NonManifoldEdges` (expected for open shells) or `Ok`. Panics on
/// unexpected errors (`TooManyGaps`).
fn heal_open_shell_and_assert_ran(
    shell: CompressedShell<monstertruck_geometry::prelude::Point3, Curve, Surface>,
    label: &str,
) {
    let result = heal_surface_shell(shell, TOL);
    match result {
        Ok(healed) => {
            let condition = healed.shell_condition();
            eprintln!("[{label}] healed Ok, condition = {condition:?}");
        }
        // Single-face shells have boundary edges appearing only once,
        // which is non-manifold by the strict check. This still proves
        // welding + degenerate removal ran successfully.
        Err(SurfaceHealingError::NonManifoldEdges { edge_indices }) => {
            eprintln!(
                "[{label}] NonManifoldEdges (boundary edges, expected for open shell): {} edges",
                edge_indices.len()
            );
        }
        Err(e) => panic!("{label}: unexpected error: {e}"),
    }
}

// -----------------------------------------------------------------------
// Test 1: Sweep rail with kinked rail (triggers gap welding).
// -----------------------------------------------------------------------

#[test]
fn heal_sweep_rail_kinked() {
    let shell = load_fixture_shell("sweep_rail_kinked");
    // Single-face open shell -- welding runs, then boundary edges trigger
    // `NonManifoldEdges`. This is expected and still exercises the healing path.
    heal_open_shell_and_assert_ran(shell, "sweep_rail_kinked");
}

// -----------------------------------------------------------------------
// Test 2: Birail with diverging rails (triggers gap welding +
//         degenerate removal).
// -----------------------------------------------------------------------

#[test]
fn heal_birail_diverging() {
    let shell = load_fixture_shell("birail_diverging");
    heal_open_shell_and_assert_ran(shell, "birail_diverging");
}

// -----------------------------------------------------------------------
// Test 3: Gordon with degenerate curves (triggers all healing stages).
//         Two-face shell with a shared edge: the shared edge becomes manifold
//         after welding, but the 6 boundary edges remain non-manifold (open).
// -----------------------------------------------------------------------

#[test]
fn heal_gordon_degenerate() {
    let shell = load_fixture_shell("gordon_degenerate");
    let result = heal_surface_shell(shell, TOL);
    match result {
        Ok(healed) => {
            let condition = healed.shell_condition();
            assert!(
                matches!(
                    condition,
                    ShellCondition::Regular | ShellCondition::Oriented | ShellCondition::Closed
                ),
                "gordon_degenerate: shell condition is {condition:?}, expected at least Regular"
            );
        }
        Err(SurfaceHealingError::NonManifoldEdges { edge_indices }) => {
            // Edge 1 (the shared edge) should NOT be in the non-manifold list.
            assert!(
                !edge_indices.contains(&1),
                "gordon_degenerate: shared edge 1 should be manifold after welding, \
                 but found in non-manifold list: {edge_indices:?}"
            );
            // The remaining boundary edges are expected to be non-manifold
            // (they appear only once in an open shell).
            eprintln!(
                "[gordon_degenerate] Non-manifold boundary edges (expected for open shell): \
                 {edge_indices:?}"
            );
        }
        Err(e) => panic!("gordon_degenerate: unexpected error: {e}"),
    }
}

// -----------------------------------------------------------------------
// Test 4: Collapsed edge shell (triggers degenerate edge removal).
// -----------------------------------------------------------------------

#[test]
fn heal_collapsed_edge() {
    let shell = load_fixture_shell("collapsed_edge");
    let result = heal_surface_shell(shell, TOL);
    match result {
        Ok(healed) => {
            let condition = healed.shell_condition();
            assert!(
                matches!(
                    condition,
                    ShellCondition::Regular | ShellCondition::Oriented | ShellCondition::Closed
                ),
                "collapsed_edge: shell condition is {condition:?}, expected at least Regular"
            );
        }
        // Open shell with degenerate edge removed -- boundary edges are
        // naturally non-manifold.
        Err(SurfaceHealingError::NonManifoldEdges { .. }) => {
            // Healing ran and detected the remaining boundary -- acceptable.
        }
        Err(e) => panic!("collapsed_edge: unexpected error: {e}"),
    }
}

// -----------------------------------------------------------------------
// Test 5: Glyph profile sweep healing.
// -----------------------------------------------------------------------

#[test]
fn heal_glyph_sweep() {
    use monstertruck_geometry::prelude::*;
    use monstertruck_modeling::Surface;

    // Use the first segment of the glyph sharp-corners fixture as a profile.
    let glyph_segments =
        monstertruck_geometry::nurbs::test_fixtures::fixture_glyph_sharp_corners();
    let profile = glyph_segments[0].clone();

    // Simple rail along Z axis.
    let rail = BsplineCurve::new(
        KnotVector::bezier_knot(1),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 3.0)],
    );

    let surface = BsplineSurface::sweep_rail(profile, &rail, 5);

    // Build a single-face shell from the swept surface using the same helper
    // pattern as `fixture_helpers`.
    let (u_knots, v_knots) = surface.knot_vecs();
    let u0 = u_knots[0];
    let u1 = u_knots[u_knots.len() - 1];
    let v0 = v_knots[0];
    let v1 = v_knots[v_knots.len() - 1];

    let p00 = surface.subs(u0, v0);
    let p10 = surface.subs(u1, v0);
    let p11 = surface.subs(u1, v1);
    let p01 = surface.subs(u0, v1);

    // Add a small gap to trigger welding.
    let p00_gap = Point3::new(p00.x + 1e-8, p00.y + 1e-8, p00.z);

    let vertices = vec![p00, p10, p11, p01, p00_gap];

    use monstertruck_topology::compress::*;

    let make_edge = |va: usize, vb: usize, a: Point3, b: Point3| CompressedEdge {
        vertices: (va, vb),
        curve: Curve::BsplineCurve(BsplineCurve::new(KnotVector::bezier_knot(1), vec![a, b])),
    };

    let edges = vec![
        make_edge(0, 1, p00, p10),
        make_edge(1, 2, p10, p11),
        make_edge(2, 3, p11, p01),
        make_edge(3, 4, p01, p00_gap),
    ];

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

    let cshell: CompressedShell<Point3, Curve, Surface> = CompressedShell {
        vertices,
        edges,
        faces: vec![face],
    };

    // Single-face open shell -- `NonManifoldEdges` is expected.
    heal_open_shell_and_assert_ran(cshell, "glyph_sweep");
}

// -----------------------------------------------------------------------
// Test 6: Panic safety -- no fixture causes a panic.
// -----------------------------------------------------------------------

#[test]
fn all_fixtures_no_panic() {
    let mut ok_count = 0usize;
    let mut err_count = 0usize;

    FIXTURE_NAMES.iter().for_each(|&name| {
        let shell = load_fixture_shell(name);
        let result = catch_unwind(AssertUnwindSafe(|| heal_surface_shell(shell, TOL)));
        match result {
            Ok(Ok(_)) => {
                ok_count += 1;
            }
            Ok(Err(e)) => {
                eprintln!("[all_fixtures_no_panic] {name}: Err({e})");
                err_count += 1;
            }
            Err(panic_payload) => {
                panic!(
                    "[all_fixtures_no_panic] {name}: PANICKED: {}",
                    panic_message(&panic_payload)
                );
            }
        }
    });

    eprintln!(
        "[all_fixtures_no_panic] summary: {ok_count} Ok, {err_count} Err, {} total",
        ok_count + err_count
    );
    assert!(
        ok_count + err_count == FIXTURE_NAMES.len(),
        "all fixtures should have been processed"
    );
}

// -----------------------------------------------------------------------
// Test 7: Timeout safety -- each fixture heals within 10 seconds.
// -----------------------------------------------------------------------

#[test]
fn all_fixtures_within_timeout() {
    let timeout = Duration::from_secs(10);

    FIXTURE_NAMES.iter().for_each(|&name| {
        let (tx, rx) = mpsc::channel();
        let shell = load_fixture_shell(name);

        let handle = std::thread::spawn(move || {
            let result = catch_unwind(AssertUnwindSafe(|| heal_surface_shell(shell, TOL)));
            let _ = tx.send(());
            result
        });

        match rx.recv_timeout(timeout) {
            Ok(()) => {
                // Thread completed within timeout. Join to get the result.
                let thread_result = handle.join().expect("thread should not panic");
                match thread_result {
                    Ok(_heal_result) => {
                        // Ok or Err from healing -- both acceptable for timeout test.
                    }
                    Err(panic_payload) => {
                        panic!(
                            "[timeout] {name}: panicked: {}",
                            panic_message(&panic_payload)
                        );
                    }
                }
            }
            Err(_) => {
                panic!("[timeout] {name}: healing took longer than {timeout:?}");
            }
        }
    });
}
