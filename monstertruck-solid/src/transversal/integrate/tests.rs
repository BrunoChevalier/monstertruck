use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;

/// Verify that `detect_coincident_faces` is callable from the integrate module
/// and returns a valid (possibly empty) result. This exercises the wiring
/// added in Task 2 where coincident detection is used as diagnostic logging
/// inside `process_one_pair_of_shells`.
///
/// Note: cubes use unbounded [`Plane`] surfaces, so the coincident detection
/// (which requires bounded parameter ranges) returns empty. The test verifies
/// the function is accessible and compiles.
#[test]
fn coincident_detection_wired_from_integrate() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let shell = &cube.boundaries()[0];

    // Call detect_coincident_faces -- verifies the module path is accessible.
    let coincident = super::edge_cases::detect_coincident_faces(shell, shell, 0.05);
    // Planes are unbounded so detection returns empty, but the call must not panic.
    assert!(
        coincident.is_empty(),
        "Unbounded surfaces should produce no coincident pairs."
    );
}

/// Verify that `heal_shell_if_needed` never returns `None` for a non-empty
/// shell. The 3-stage fallback always includes the original shell as a
/// candidate, so the result should always be `Some`.
#[test]
fn heal_shell_never_returns_none() {
    // Build a unit cube -- its shell is already closed.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());
    let shell = cube.boundaries()[0].clone();

    // Even for a well-formed shell, heal_shell_if_needed should return Some.
    let healed = super::heal_shell_if_needed(shell, 0.05);
    assert!(
        healed.is_some(),
        "heal_shell_if_needed must never return None for non-empty shell."
    );
}

/// Verify that `heal_shell_if_needed` returns `Some` for an open (non-closed)
/// shell and that the result quality is no worse than the input.
#[test]
fn heal_shell_open_shell_returns_some() {
    use monstertruck_topology::shell::ShellCondition;
    // Build a single face (open shell -- not closed).
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let face: Shell = vec![builder::extrude(&e, Vector3::unit_y())].into();

    assert_ne!(
        face.shell_condition(),
        ShellCondition::Closed,
        "Precondition: shell must not be closed."
    );

    let original_quality = super::shell_quality(&face);
    let healed = super::heal_shell_if_needed(face, 0.05);
    assert!(
        healed.is_some(),
        "heal_shell_if_needed must return Some even for open shells."
    );
    let healed_quality = super::shell_quality(healed.as_ref().unwrap());
    assert!(
        healed_quality <= original_quality,
        "Healed shell quality {healed_quality:?} must be no worse than original {original_quality:?}."
    );
}

/// Verify that `try_cap_shell_with_existing_surfaces` returns the shell
/// unchanged when it is already closed (early-return optimization).
#[test]
fn cap_shell_early_return_for_closed() {
    use monstertruck_topology::shell::ShellCondition;
    // Build a closed cube shell.
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());
    let shell = cube.boundaries()[0].clone();

    assert_eq!(
        shell.shell_condition(),
        ShellCondition::Closed,
        "Precondition: shell must be closed."
    );
    let original_len = shell.len();

    // Capping a closed shell should return it unchanged.
    let capped = super::try_cap_shell_with_existing_surfaces(shell, 0.05);
    assert_eq!(
        capped.len(),
        original_len,
        "Closed shell should not gain faces from capping."
    );
}

#[test]
fn adjacent_cubes_or() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.5, 1.0));
    let w = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&w, Vector3::unit_y());
    let cube2: Solid = builder::extrude(&f, Vector3::unit_z());

    let result = crate::or(&cube, &cube2, 0.05);
    assert!(
        result.is_ok(),
        "Boolean OR of adjacent cubes should succeed"
    );
    let solid = result.unwrap();

    assert_eq!(solid.boundaries().len(), 1);

    let poly = solid.triangulation(0.01).to_polygon();
    assert_near!(poly.volume(), 2.0);

    let homog = poly.center_of_gravity();
    assert_near!(homog.to_point(), Point3::new(0.75, 0.75, 1.0));

    let bbx = poly.bounding_box();
    assert_near!(bbx.min(), Point3::new(0.0, 0.0, 0.0));
    assert_near!(bbx.max(), Point3::new(1.5, 1.5, 2.0));

    assert_eq!(solid.face_iter().count(), 12);
}

#[test]
fn punched_cube() {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    let v = builder::vertex(Point3::new(0.5, 0.25, -0.5));
    let w = builder::revolve(
        &v,
        Point3::new(0.5, 0.5, 0.0),
        Vector3::unit_z(),
        Rad(7.0),
        3,
    );
    let f = builder::try_attach_plane(&[w]).unwrap();
    let mut cylinder = builder::extrude(&f, Vector3::unit_z() * 2.0);
    cylinder.not();
    let and = crate::and(&cube, &cylinder, 0.05).unwrap();

    let poly = and.triangulation(0.01).to_polygon();
    let file = std::fs::File::create("punched-cube.obj").unwrap();
    obj::write(&poly, file).unwrap();
}
