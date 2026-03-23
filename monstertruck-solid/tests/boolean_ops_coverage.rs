//! Coverage tests for boolean operations: union, intersection, difference,
//! and symmetric difference with geometric validation.

use monstertruck_modeling::*;
use monstertruck_topology::shell::ShellCondition;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Tolerance for boolean operations.
const TOL: f64 = 0.05;

/// Builds a cube at `origin` with the given `side` length.
fn make_cube(origin: Point3, side: f64) -> Solid {
    let v = builder::vertex(origin);
    let e = builder::extrude(&v, Vector3::new(side, 0.0, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, side, 0.0));
    builder::extrude(&f, Vector3::new(0.0, 0.0, side))
}

/// Computes the axis-aligned bounding box of a solid by iterating over
/// all boundary vertices.
fn solid_bounding_box(solid: &Solid) -> (Point3, Point3) {
    let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);
    solid.boundaries().iter().for_each(|shell| {
        shell.vertex_iter().for_each(|v| {
            let p = v.point();
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        });
    });
    (min, max)
}

/// Two unit cubes offset by (0.5, 0.5, 0.5). AND should produce a solid
/// whose bounding box is contained within both inputs.
#[test]
fn boolean_and_overlapping_cubes() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::new(0.5, 0.5, 0.5), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&cube_a, &cube_b, TOL)
    }));
    let inner = result.expect("boolean AND must not panic");
    assert!(inner.is_ok(), "boolean AND must return Ok: {inner:?}");
    let solid = inner.unwrap();

    assert!(
        !solid.boundaries().is_empty(),
        "AND result must have boundaries."
    );

    // Verify shell condition is Closed.
    solid.boundaries().iter().for_each(|shell| {
        assert_eq!(
            shell.shell_condition(),
            ShellCondition::Closed,
            "AND result shell must be Closed."
        );
    });

    // The intersection bounding box should be contained within both inputs.
    let (and_min, and_max) = solid_bounding_box(&solid);
    // Intersection should start around (0.5, 0.5, 0.5) and end around (1.0, 1.0, 1.0).
    assert!(
        and_min.x >= 0.5 - TOL && and_min.y >= 0.5 - TOL && and_min.z >= 0.5 - TOL,
        "AND min should be near (0.5, 0.5, 0.5), got ({}, {}, {})",
        and_min.x,
        and_min.y,
        and_min.z
    );
    assert!(
        and_max.x <= 1.0 + TOL && and_max.y <= 1.0 + TOL && and_max.z <= 1.0 + TOL,
        "AND max should be near (1.0, 1.0, 1.0), got ({}, {}, {})",
        and_max.x,
        and_max.y,
        and_max.z
    );
}

/// Two unit cubes offset by (0.5, 0.5, 0.5). OR should produce a solid
/// whose bounding box encompasses both inputs.
#[test]
fn boolean_or_overlapping_cubes() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::new(0.5, 0.5, 0.5), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::or(&cube_a, &cube_b, TOL)
    }));
    let inner = result.expect("boolean OR must not panic");
    assert!(inner.is_ok(), "boolean OR must return Ok: {inner:?}");
    let solid = inner.unwrap();

    assert!(
        !solid.boundaries().is_empty(),
        "OR result must have boundaries."
    );

    // Verify shell condition is Closed.
    solid.boundaries().iter().for_each(|shell| {
        assert_eq!(
            shell.shell_condition(),
            ShellCondition::Closed,
            "OR result shell must be Closed."
        );
    });

    // The union bounding box should encompass both inputs.
    let (or_min, or_max) = solid_bounding_box(&solid);
    assert!(
        or_min.x <= TOL && or_min.y <= TOL && or_min.z <= TOL,
        "OR min should be near origin, got ({}, {}, {})",
        or_min.x,
        or_min.y,
        or_min.z
    );
    assert!(
        or_max.x >= 1.5 - TOL && or_max.y >= 1.5 - TOL && or_max.z >= 1.5 - TOL,
        "OR max should be near (1.5, 1.5, 1.5), got ({}, {}, {})",
        or_max.x,
        or_max.y,
        or_max.z
    );
}

/// Cube A at origin, cube B offset by (0.5, 0.5, 0.5). difference(A, B)
/// should produce a non-empty solid whose bounding box matches A's extent
/// on the non-cut sides.
#[test]
fn boolean_difference_cubes() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::new(0.5, 0.5, 0.5), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::difference(&cube_a, &cube_b, TOL)
    }));
    let inner = result.expect("boolean difference must not panic");
    assert!(
        inner.is_ok(),
        "boolean difference must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();

    assert!(
        !solid.boundaries().is_empty(),
        "Difference result must have boundaries."
    );

    // Bounding box of the result should still span from origin on the
    // non-cut corner.
    let (diff_min, _diff_max) = solid_bounding_box(&solid);
    assert!(
        diff_min.x <= TOL && diff_min.y <= TOL && diff_min.z <= TOL,
        "Difference min should be near origin, got ({}, {}, {})",
        diff_min.x,
        diff_min.y,
        diff_min.z
    );
}

/// symmetric_difference of two overlapping cubes produces a non-empty solid
/// with non-empty boundaries.
#[test]
fn boolean_symmetric_difference_cubes() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::new(0.5, 0.5, 0.5), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::symmetric_difference(&cube_a, &cube_b, TOL)
    }));
    let inner = result.expect("symmetric_difference must not panic");
    assert!(
        inner.is_ok(),
        "symmetric_difference must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();

    assert!(
        !solid.boundaries().is_empty(),
        "Symmetric difference result must have non-empty boundaries."
    );
}

/// Two disjoint cubes. AND result should either be an error or produce a
/// solid with empty/trivial boundaries.
#[test]
fn boolean_and_disjoint_returns_valid() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::new(5.0, 5.0, 5.0), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&cube_a, &cube_b, TOL)
    }));
    // Must not panic regardless of outcome.
    let inner = result.expect("disjoint AND must not panic");
    // Either Err (no intersection) or Ok with empty/valid boundaries are acceptable.
    if let Ok(solid) = &inner {
        // If it succeeds, that is fine; we just verify no crash.
        eprintln!(
            "disjoint AND returned Ok with {} boundaries",
            solid.boundaries().len()
        );
    } else {
        eprintln!("disjoint AND returned Err (expected for no-overlap case)");
    }
}

/// Two fully disjoint cubes. OR should produce a valid result.
#[test]
fn boolean_or_disjoint_cubes() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::new(5.0, 5.0, 5.0), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::or(&cube_a, &cube_b, TOL)
    }));
    let inner = result.expect("disjoint OR must not panic");
    // Disjoint OR may error or produce valid output; no crash is the requirement.
    if let Ok(solid) = &inner {
        assert!(
            !solid.boundaries().is_empty(),
            "Disjoint OR, if Ok, must have boundaries."
        );
    }
}

/// Small cube fully inside a large cube. difference(large, small) should
/// produce a valid solid without panicking. The boolean engine handles
/// full containment by producing a result with non-empty boundaries.
#[test]
fn boolean_difference_contained() {
    let large = make_cube(Point3::origin(), 2.0);
    let small = make_cube(Point3::new(0.5, 0.5, 0.5), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::difference(&large, &small, TOL)
    }));
    let inner = result.expect("contained difference must not panic");
    assert!(
        inner.is_ok(),
        "contained difference must return Ok: {inner:?}"
    );
    let solid = inner.unwrap();

    assert!(
        !solid.boundaries().is_empty(),
        "Contained difference result must have boundaries."
    );

    // Verify each boundary shell has valid topology.
    solid
        .boundaries()
        .iter()
        .enumerate()
        .for_each(|(i, shell)| {
            let cond = shell.shell_condition();
            assert!(
                matches!(
                    cond,
                    ShellCondition::Closed | ShellCondition::Oriented | ShellCondition::Regular
                ),
                "Contained difference shell {i} condition is {cond:?}, expected at least Regular."
            );
        });
}

/// Two identical cubes. AND should produce a solid equivalent to either
/// input (bounding box should match).
#[test]
fn boolean_and_identical_cubes() {
    let cube_a = make_cube(Point3::origin(), 1.0);
    let cube_b = make_cube(Point3::origin(), 1.0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        monstertruck_solid::and(&cube_a, &cube_b, TOL)
    }));
    let inner = result.expect("identical AND must not panic");
    assert!(inner.is_ok(), "identical AND must return Ok: {inner:?}");
    let solid = inner.unwrap();

    assert!(
        !solid.boundaries().is_empty(),
        "Identical AND result must have boundaries."
    );

    // Bounding box should match the input cube.
    let (and_min, and_max) = solid_bounding_box(&solid);
    assert!(
        and_min.x <= TOL && and_min.y <= TOL && and_min.z <= TOL,
        "Identical AND min should be near origin, got ({}, {}, {})",
        and_min.x,
        and_min.y,
        and_min.z
    );
    assert!(
        and_max.x >= 1.0 - TOL && and_max.y >= 1.0 - TOL && and_max.z >= 1.0 - TOL,
        "Identical AND max should be near (1,1,1), got ({}, {}, {})",
        and_max.x,
        and_max.y,
        and_max.z
    );
}
