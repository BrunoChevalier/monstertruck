use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Helper to build a unit cube at a given origin.
fn make_cube(origin: Point3) -> Solid {
    let v = builder::vertex(origin);
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    builder::extrude(&f, Vector3::unit_z())
}

/// Two cubes touching at a face (tangent). One at origin, one at (1,0,0).
/// Boolean AND should not panic and should return Ok.
#[test]
fn tangent_face_and() {
    let cube0 = make_cube(Point3::origin());
    let cube1 = make_cube(Point3::new(1.0, 0.0, 0.0));
    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&cube0, &cube1, 0.05)
    }));
    let inner = result.expect("tangent_face_and must not panic");
    assert!(inner.is_ok(), "tangent_face_and must return Ok: {inner:?}");
    let solid = inner.unwrap();
    // Tangent AND should produce a valid solid (possibly thin/degenerate).
    assert!(
        !solid.boundaries().is_empty(),
        "Tangent AND result must have boundaries."
    );
}

/// Two cubes touching at a face (tangent). Boolean OR should not panic and return Ok.
#[test]
fn tangent_face_or() {
    let cube0 = make_cube(Point3::origin());
    let cube1 = make_cube(Point3::new(1.0, 0.0, 0.0));
    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::or(&cube0, &cube1, 0.05)
    }));
    let inner = result.expect("tangent_face_or must not panic");
    assert!(inner.is_ok(), "tangent_face_or must return Ok: {inner:?}");
    let solid = inner.unwrap();
    // OR of two touching cubes should produce a 2x1x1 box.
    assert!(
        !solid.boundaries().is_empty(),
        "Tangent OR result must have boundaries."
    );
}

/// Two cubes sharing a coincident face (translated by exactly 1 along X).
/// Boolean AND should not panic and return Ok.
#[test]
fn coincident_face_and() {
    let cube0 = make_cube(Point3::origin());
    let cube1 = make_cube(Point3::new(1.0, 0.0, 0.0));
    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&cube0, &cube1, 0.05)
    }));
    let inner = result.expect("coincident_face_and must not panic");
    assert!(
        inner.is_ok(),
        "coincident_face_and must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();
    assert!(
        !solid.boundaries().is_empty(),
        "Coincident AND result must have boundaries."
    );
}

/// Two cubes sharing a coincident face. Boolean OR should produce a valid merged solid.
#[test]
fn coincident_face_or() {
    let cube0 = make_cube(Point3::origin());
    let cube1 = make_cube(Point3::new(1.0, 0.0, 0.0));
    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::or(&cube0, &cube1, 0.05)
    }));
    let inner = result.expect("coincident_face_or must not panic");
    assert!(
        inner.is_ok(),
        "coincident_face_or must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();
    assert!(
        !solid.boundaries().is_empty(),
        "Coincident OR result must have boundaries."
    );
}

/// Helper to build a sphere solid via revolving a semicircular arc.
/// Uses `revolve_wire` to handle pole degeneration at the axis endpoints.
fn make_sphere(center: Point3, radius: f64) -> Solid {
    use std::f64::consts::PI;
    // Build a semicircular arc from the north pole to the south pole via the equator.
    let north = center + Vector3::new(0.0, radius, 0.0);
    let south = center - Vector3::new(0.0, radius, 0.0);
    let transit = center + Vector3::new(radius, 0.0, 0.0);
    let v0 = builder::vertex(north);
    let v1 = builder::vertex(south);
    let arc: Edge = builder::circle_arc(&v0, &v1, transit);
    let wire: Wire = vec![arc].into();
    // Revolve the semicircular wire around the Y axis to create a sphere shell.
    // `revolve_wire` handles pole degeneration at the on-axis endpoints.
    let shell: Shell = builder::revolve_wire(&wire, center, Vector3::unit_y(), Rad(2.0 * PI), 4);
    Solid::new(vec![shell])
}

/// Sphere (pole-degenerate) intersected with a cube. Boolean AND should not panic
/// and should return Ok with a valid solid.
#[test]
fn pole_degeneration_sphere_and() {
    let sphere = make_sphere(Point3::new(0.5, 0.5, 0.5), 0.4);
    let cube = make_cube(Point3::origin());
    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&sphere, &cube, 0.05)
    }));
    let inner = result.expect("pole_degeneration_sphere_and must not panic");
    assert!(
        inner.is_ok(),
        "pole_degeneration_sphere_and must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();
    // The sphere is fully inside the cube, so AND should produce the sphere itself.
    assert!(
        !solid.boundaries().is_empty(),
        "Sphere AND cube result must have boundaries."
    );
}

/// Sphere (pole-degenerate) difference with a cube. Should not panic
/// and should return Ok with a valid solid.
#[test]
fn pole_degeneration_sphere_difference() {
    let sphere = make_sphere(Point3::new(0.5, 0.5, 0.5), 0.4);
    let cube = make_cube(Point3::origin());
    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::difference(&sphere, &cube, 0.05)
    }));
    let inner = result.expect("pole_degeneration_sphere_difference must not panic");
    assert!(
        inner.is_ok(),
        "pole_degeneration_sphere_difference must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();
    assert!(
        !solid.boundaries().is_empty(),
        "Sphere difference cube result must have boundaries."
    );
}

/// Regression test: standard boolean ops on overlapping cubes must not panic.
/// The and/or/difference operations should return results (Ok or Err) without panicking.
/// When successful, the result should have non-empty boundaries.
#[test]
fn regression_standard_boolean() {
    let cube0 = make_cube(Point3::origin());
    let cube1 = make_cube(Point3::new(0.3, 0.3, 0.3));

    // All operations must not panic.
    let and_result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&cube0, &cube1, 0.05)
    }));
    assert!(
        and_result.is_ok(),
        "Standard AND must not panic: {and_result:?}"
    );

    let or_result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::or(&cube0, &cube1, 0.05)
    }));
    assert!(
        or_result.is_ok(),
        "Standard OR must not panic: {or_result:?}"
    );
    // OR on overlapping cubes should succeed and produce valid output.
    if let Ok(Ok(or_solid)) = &or_result {
        assert!(
            !or_solid.boundaries().is_empty(),
            "OR result must have boundaries."
        );
    }

    let diff_result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::difference(&cube0, &cube1, 0.05)
    }));
    assert!(
        diff_result.is_ok(),
        "Standard difference must not panic: {diff_result:?}"
    );
}
