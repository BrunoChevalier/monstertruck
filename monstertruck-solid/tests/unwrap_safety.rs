use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;

/// Verifies that `and()` returns an error (not a panic) when given a
/// `Solid` with zero boundary shells.
#[test]
fn and_empty_boundary_returns_error() {
    let empty_solid: Solid = Solid::new_unchecked(vec![]);

    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    // This must return Err, not panic.
    let result = monstertruck_solid::and(&empty_solid, &cube, 0.05);
    assert!(
        result.is_err(),
        "and() with empty-boundary solid must return Err"
    );
}

/// Verifies that `or()` returns an error (not a panic) when given a
/// `Solid` with zero boundary shells.
#[test]
fn or_empty_boundary_returns_error() {
    let empty_solid: Solid = Solid::new_unchecked(vec![]);

    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    // This must return Err, not panic.
    let result = monstertruck_solid::or(&empty_solid, &cube, 0.05);
    assert!(
        result.is_err(),
        "or() with empty-boundary solid must return Err"
    );
}
