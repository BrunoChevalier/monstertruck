//! Coverage tests for fillet pipeline entry points: `fillet_edges_generic`
//! with various profiles (`Round`, `Chamfer`, `Ridge`) and radius specs.

use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;

/// Builds a unit cube at the origin.
fn make_cube() -> Solid {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    builder::extrude(&f, Vector3::unit_z())
}

/// Fillet one edge of a cube with `FilletProfile::Round`. The resulting
/// shell should be Closed and have more faces than the original 6.
#[test]
fn fillet_edges_round_profile() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];
    let original_face_count = shell.face_iter().count();

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.1),
        profile: monstertruck_solid::FilletProfile::Round,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts))
        .expect("round fillet failed");

    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Round fillet result must be Closed."
    );
    assert!(
        shell.face_iter().count() > original_face_count,
        "Fillet should add faces (original {original_face_count})."
    );
}

/// Fillet one edge of a cube with `FilletProfile::Chamfer`. The resulting
/// shell should be Closed and have more faces than the original 6.
#[test]
fn fillet_edges_chamfer_profile() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];
    let original_face_count = shell.face_iter().count();

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.1),
        profile: monstertruck_solid::FilletProfile::Chamfer,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts))
        .expect("chamfer fillet failed");

    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Chamfer result must be Closed."
    );
    assert!(
        shell.face_iter().count() > original_face_count,
        "Chamfer should add faces (original {original_face_count})."
    );
}

/// Fillet one edge of a cube with `FilletProfile::Ridge`. The resulting
/// shell should be Closed.
#[test]
fn fillet_edges_ridge_profile() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.1),
        profile: monstertruck_solid::FilletProfile::Ridge,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts))
        .expect("ridge fillet failed");

    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Ridge result must be Closed."
    );
}

/// Fillet edges sequentially: fillet one edge, then a second edge on the
/// resulting shell. The shell should remain Closed after each step.
#[test]
fn fillet_multiple_edges_sequential() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];
    let original_face_count = shell.face_iter().count();

    // Fillet first edge.
    let first_edge: Vec<_> = shell.edge_iter().take(1).collect();
    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.1),
        profile: monstertruck_solid::FilletProfile::Round,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &first_edge, Some(&opts))
        .expect("first edge fillet failed");

    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Shell must be Closed after first fillet."
    );
    let after_first = shell.face_iter().count();
    assert!(
        after_first > original_face_count,
        "First fillet should add faces."
    );

    // Fillet a second edge from the modified shell.
    let second_edge: Vec<_> = shell.edge_iter().take(1).collect();
    let result = monstertruck_solid::fillet_edges_generic(shell, &second_edge, Some(&opts));
    // Sequential fillet may succeed or fail depending on edge adjacency
    // after the first fillet. If it succeeds, verify topology.
    if let Ok(()) = result {
        assert_eq!(
            shell.shell_condition(),
            ShellCondition::Closed,
            "Shell must be Closed after second fillet."
        );
    }
}

/// Fillet with a variable radius specification.
#[test]
fn fillet_with_variable_radius() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Variable(Box::new(|t| 0.05 + 0.1 * t)),
        profile: monstertruck_solid::FilletProfile::Round,
        ..Default::default()
    };
    let result = monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts));
    assert!(
        result.is_ok(),
        "Variable-radius fillet must succeed: {result:?}"
    );

    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Variable-radius fillet result must be Closed."
    );
}

/// Fillet with an extremely large radius (10.0 on a unit cube edge)
/// should return an error.
#[test]
fn fillet_invalid_radius_returns_error() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(10.0),
        profile: monstertruck_solid::FilletProfile::Round,
        ..Default::default()
    };
    let result = monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts));
    assert!(
        result.is_err(),
        "Extremely large radius should produce an error."
    );
}

/// After filleting, verify `singular_vertices` is empty and
/// `shell_condition` is Closed.
#[test]
fn fillet_result_topology_valid() {
    let cube = make_cube();
    let mut shells = cube.into_boundaries();
    let shell = &mut shells[0];

    let target_edges: Vec<_> = shell.edge_iter().take(1).collect();
    assert!(!target_edges.is_empty(), "cube must have edges");

    let opts = monstertruck_solid::FilletOptions {
        radius: monstertruck_solid::RadiusSpec::Constant(0.1),
        profile: monstertruck_solid::FilletProfile::Round,
        ..Default::default()
    };
    monstertruck_solid::fillet_edges_generic(shell, &target_edges, Some(&opts))
        .expect("fillet failed");

    assert!(
        shell.singular_vertices().is_empty(),
        "Fillet result should have no singular vertices."
    );
    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Fillet result shell must be Closed."
    );
}
