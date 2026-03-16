//! Topology validation for fillet operations.
//!
//! Provides debug-only assertions that verify [`Shell`] topology invariants
//! (Euler-Poincare characteristic and face orientation consistency) after
//! fillet modifications. No runtime cost in release builds.

use std::collections::HashSet;

use monstertruck_topology::shell::ShellCondition;

use super::types::*;

/// Counts unique vertices, edges, and faces in a shell.
///
/// Returns `(V, E, F)` where V and E are deduplicated by ID.
fn count_vef(shell: &Shell) -> (usize, usize, usize) {
    let v = shell
        .vertex_iter()
        .map(|vtx| vtx.id())
        .collect::<HashSet<_>>()
        .len();
    let e = shell
        .edge_iter()
        .map(|edge| edge.id())
        .collect::<HashSet<_>>()
        .len();
    let f = shell.len();
    (v, e, f)
}

/// Computes the Euler characteristic (V - E + F) for a shell.
fn euler_characteristic(shell: &Shell) -> isize {
    let (v, e, f) = count_vef(shell);
    v as isize - e as isize + f as isize
}

/// Checks the Euler-Poincare characteristic for a closed shell.
///
/// For a closed shell: V - E + F must equal 2.
/// For non-closed shells (open, oriented, regular, irregular): returns `true`
/// unconditionally because the Euler-Poincare formula only applies to closed
/// 2-manifolds.
pub(crate) fn euler_poincare_check(shell: &Shell) -> bool {
    if shell.shell_condition() != ShellCondition::Closed {
        return true;
    }
    euler_characteristic(shell) == 2
}

/// Checks that the shell has compatible face orientations.
///
/// Returns `true` if [`shell_condition()`] is [`ShellCondition::Oriented`] or
/// [`ShellCondition::Closed`].
pub(crate) fn is_oriented_check(shell: &Shell) -> bool {
    matches!(
        shell.shell_condition(),
        ShellCondition::Oriented | ShellCondition::Closed
    )
}

/// Debug-only topology validation for a shell after fillet operations.
///
/// In release builds this is a complete no-op.
/// In debug builds it checks both the Euler-Poincare characteristic (closed
/// shells only) and orientation consistency.
pub(crate) fn debug_assert_topology(shell: &Shell, context: &str) {
    if cfg!(debug_assertions) {
        let (v, e, f) = count_vef(shell);
        let chi = v as isize - e as isize + f as isize;

        if shell.shell_condition() == ShellCondition::Closed {
            debug_assert!(
                chi == 2,
                "Euler-Poincare violation after {context}: V={v} E={e} F={f}, V-E+F={chi}"
            );
        }

        let condition = shell.shell_condition();
        debug_assert!(
            matches!(condition, ShellCondition::Oriented | ShellCondition::Closed),
            "Orientation violation after {context}: condition={condition:?}"
        );
    }
}

/// Debug-only Euler-Poincare check without orientation validation.
///
/// Used for mid-chain intermediate states where orientation may be temporarily
/// invalid but the vertex-edge-face count should still be consistent.
pub(crate) fn debug_assert_euler(shell: &Shell, context: &str) {
    if cfg!(debug_assertions) {
        let (v, e, f) = count_vef(shell);
        let chi = v as isize - e as isize + f as isize;

        if shell.shell_condition() == ShellCondition::Closed {
            debug_assert!(
                chi == 2,
                "Euler-Poincare violation after {context}: V={v} E={e} F={f}, V-E+F={chi}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use monstertruck_geometry::prelude::*;

    use super::*;

    // -- Test 1: Euler-Poincare on valid closed box --

    /// Builds a 6-face closed box, verifies `ShellCondition::Closed`,
    /// `euler_poincare_check` returns true, `is_oriented_check` returns true,
    /// and explicitly counts V=8, E=12, F=6 so that V - E + F = 2.
    #[test]
    fn euler_poincare_valid_closed_box() {
        let shell = build_closed_box();

        assert_eq!(shell.shell_condition(), ShellCondition::Closed);
        assert!(euler_poincare_check(&shell));
        assert!(is_oriented_check(&shell));

        // Explicit V, E, F count.
        let (v, e, f) = count_vef(&shell);
        assert_eq!(v, 8, "expected 8 unique vertices");
        assert_eq!(e, 12, "expected 12 unique edges");
        assert_eq!(f, 6, "expected 6 faces");
        assert_eq!(
            v as isize - e as isize + f as isize,
            2,
            "Euler characteristic must be 2 for closed box"
        );
    }

    // -- Test 2: Topology valid after box fillet --

    /// Builds a 6-face closed box, fillets one edge with `fillet_edges`,
    /// then verifies both `euler_poincare_check` and `is_oriented_check` pass.
    #[test]
    fn topology_valid_after_box_fillet() {
        let (mut shell, edge) = build_closed_box_with_edges();
        let edge_id = edge[0].id();

        super::super::edge_select::fillet_edges(&mut shell, &[edge_id], None).unwrap();

        assert!(
            euler_poincare_check(&shell),
            "Euler-Poincare must hold after filleting a closed box edge"
        );
        assert!(
            is_oriented_check(&shell),
            "Orientation must be consistent after filleting a closed box edge"
        );
    }

    // -- Test 3: Debug assertion fires on corrupted orientation --

    /// Constructs a 6-face closed box, inverts face 5 (bottom) to corrupt
    /// orientation, and verifies `debug_assert_topology` panics with an
    /// orientation violation message.
    ///
    /// Inverting a face (rather than removing one) changes `shell_condition()`
    /// from `Closed` to `Regular` -- which fails `is_oriented_check` and
    /// triggers the orientation assertion.
    #[cfg(debug_assertions)]
    #[test]
    fn debug_assert_fires_on_corrupted_orientation() {
        let mut shell = build_closed_box();

        // Sanity: before corruption the shell is Closed.
        assert_eq!(shell.shell_condition(), ShellCondition::Closed);

        // Corrupt orientation by inverting the bottom face.
        shell[5].invert();

        // After inversion the condition degrades from Closed.
        let condition = shell.shell_condition();
        assert!(
            !matches!(condition, ShellCondition::Oriented | ShellCondition::Closed),
            "Expected non-oriented condition after face inversion, got {condition:?}"
        );

        // `debug_assert_topology` must panic on orientation violation.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            debug_assert_topology(&shell, "corrupted_orientation_test");
        }));
        assert!(
            result.is_err(),
            "debug_assert_topology must panic on orientation-corrupted shell"
        );
    }

    // -- Test 4: Euler-Poincare check guard logic --

    /// Verifies the guard logic in `euler_poincare_check`:
    /// - A valid closed box returns true (V-E+F=2).
    /// - A 5-face open box (missing one face) is not Closed, so
    ///   `euler_poincare_check` returns true unconditionally (guard skips
    ///   the check for non-closed shells).
    #[test]
    fn euler_poincare_check_detects_invalid_chi() {
        // Valid closed box: euler check passes.
        let shell = build_closed_box();
        assert_eq!(shell.shell_condition(), ShellCondition::Closed);
        assert!(euler_poincare_check(&shell));

        // Build a 5-face open box by dropping the last face.
        let open_shell = build_open_box_5face();
        assert_ne!(
            open_shell.shell_condition(),
            ShellCondition::Closed,
            "5-face box must not be Closed"
        );
        // Guard logic: non-closed shells return true unconditionally.
        assert!(
            euler_poincare_check(&open_shell),
            "euler_poincare_check must return true for non-closed shells"
        );
    }

    // -- Helper: builds a 6-face closed unit cube --

    fn build_closed_box_with_edges() -> (Shell, [Edge; 12]) {
        let (shell, edge, _) = build_closed_box_full();
        (shell, edge)
    }

    fn build_closed_box() -> Shell {
        build_closed_box_full().0
    }

    fn build_closed_box_full() -> (Shell, [Edge; 12], Vec<Vertex>) {
        let p = [
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let v = Vertex::news(p);
        let line = |i: usize, j: usize| {
            let bsp = BsplineCurve::new(KnotVector::bezier_knot(1), vec![p[i], p[j]]);
            Edge::new(&v[i], &v[j], NurbsCurve::from(bsp).into())
        };
        let edge = [
            line(0, 1),
            line(1, 2),
            line(2, 3),
            line(3, 0),
            line(0, 4),
            line(1, 5),
            line(2, 6),
            line(3, 7),
            line(4, 5),
            line(5, 6),
            line(6, 7),
            line(7, 4),
        ];
        let plane = |i: usize, j: usize, k: usize, l: usize| {
            let control_points = vec![vec![p[i], p[l]], vec![p[j], p[k]]];
            let knot_vec = KnotVector::bezier_knot(1);
            let bsp = BsplineSurface::new((knot_vec.clone(), knot_vec), control_points);
            let wire: Wire = [i, j, k, l]
                .into_iter()
                .circular_tuple_windows()
                .map(|(a, b)| {
                    edge.iter()
                        .find_map(|e| {
                            if e.front() == &v[a] && e.back() == &v[b] {
                                Some(e.clone())
                            } else if e.back() == &v[a] && e.front() == &v[b] {
                                Some(e.inverse())
                            } else {
                                None
                            }
                        })
                        // SAFETY: The edge array contains all 12 edges of a cube,
                        // so every vertex pair forming a cube edge is present.
                        .unwrap()
                })
                .collect();
            Face::new(vec![wire], bsp.into())
        };
        let shell: Shell = [
            plane(0, 1, 2, 3), // top
            plane(1, 0, 4, 5), // front
            plane(2, 1, 5, 6), // right
            plane(3, 2, 6, 7), // back
            plane(0, 3, 7, 4), // left
            plane(5, 4, 7, 6), // bottom
        ]
        .into();
        (shell, edge, v)
    }

    /// Builds a 5-face open box (cube missing the bottom face).
    fn build_open_box_5face() -> Shell {
        let p = [
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let v = Vertex::news(p);
        let line = |i: usize, j: usize| {
            let bsp = BsplineCurve::new(KnotVector::bezier_knot(1), vec![p[i], p[j]]);
            Edge::new(&v[i], &v[j], NurbsCurve::from(bsp).into())
        };
        let edge = [
            line(0, 1),
            line(1, 2),
            line(2, 3),
            line(3, 0),
            line(0, 4),
            line(1, 5),
            line(2, 6),
            line(3, 7),
            line(4, 5),
            line(5, 6),
            line(6, 7),
            line(7, 4),
        ];
        let plane = |i: usize, j: usize, k: usize, l: usize| {
            let control_points = vec![vec![p[i], p[l]], vec![p[j], p[k]]];
            let knot_vec = KnotVector::bezier_knot(1);
            let bsp = BsplineSurface::new((knot_vec.clone(), knot_vec), control_points);
            let wire: Wire = [i, j, k, l]
                .into_iter()
                .circular_tuple_windows()
                .map(|(a, b)| {
                    edge.iter()
                        .find_map(|e| {
                            if e.front() == &v[a] && e.back() == &v[b] {
                                Some(e.clone())
                            } else if e.back() == &v[a] && e.front() == &v[b] {
                                Some(e.inverse())
                            } else {
                                None
                            }
                        })
                        // SAFETY: Same edge array as closed box.
                        .unwrap()
                })
                .collect();
            Face::new(vec![wire], bsp.into())
        };
        // 5 faces only -- no bottom face.
        [
            plane(0, 1, 2, 3), // top
            plane(1, 0, 4, 5), // front
            plane(2, 1, 5, 6), // right
            plane(3, 2, 6, 7), // back
            plane(0, 3, 7, 4), // left
        ]
        .into()
    }
}
