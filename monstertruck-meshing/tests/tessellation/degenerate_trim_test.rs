use super::*;

/// Constructs a planar shell whose single face has a near-zero-area
/// outer boundary (an extremely thin sliver in 3D/UV space).
fn near_zero_area_shell() -> Shell {
    let eps = 1e-9;
    let v0 = builder::vertex(Point3::new(0.0, 0.0, 0.0));
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, eps, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, eps, 0.0));

    let e01 = builder::line(&v0, &v1);
    let e12 = builder::line(&v1, &v2);
    let e23 = builder::line(&v2, &v3);
    let e30 = builder::line(&v3, &v0);

    let wire: Wire = vec![e01, e12, e23, e30].into();
    let plane = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let surface: Surface = plane.into();
    let face = Face::new_unchecked(vec![wire], surface);
    vec![face].into()
}

/// Constructs a unit-square face with a near-zero-area triangular inner hole.
/// The inner trim loop has vertices within 1e-10 of each other, making its
/// signed UV area essentially zero. After the fix, this loop should be detected
/// as degenerate and skipped rather than corrupting the CDT.
fn face_with_degenerate_hole() -> Shell {
    // Outer boundary: unit square.
    let v0 = builder::vertex(Point3::origin());
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 1.0, 0.0));

    let e01 = builder::line(&v0, &v1);
    let e12 = builder::line(&v1, &v2);
    let e23 = builder::line(&v2, &v3);
    let e30 = builder::line(&v3, &v0);
    let outer_wire: Wire = vec![e01, e12, e23, e30].into();

    // Inner hole: a tiny triangle at center with near-zero area.
    let eps = 1e-10;
    let center = Point3::new(0.5, 0.5, 0.0);
    let h0 = builder::vertex(center);
    let h1 = builder::vertex(center + Vector3::new(eps, 0.0, 0.0));
    let h2 = builder::vertex(center + Vector3::new(0.0, eps, 0.0));

    let he01 = builder::line(&h0, &h1);
    let he12 = builder::line(&h1, &h2);
    let he20 = builder::line(&h2, &h0);
    let hole_wire: Wire = vec![he01, he12, he20].into();

    let plane = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let surface: Surface = plane.into();
    // Two wires: outer boundary + degenerate inner hole.
    let face = Face::new_unchecked(vec![outer_wire, hole_wire], surface);
    vec![face].into()
}

/// Constructs a shell with a face whose trim wire contains a collapsed
/// (zero-length) edge segment: two consecutive vertices at the same 3D position.
fn collapsed_edge_shell() -> Shell {
    let p0 = Point3::new(0.0, 0.0, 0.0);
    let p1 = Point3::new(1.0, 0.0, 0.0);
    let p2 = Point3::new(1.0, 1.0, 0.0);
    let p3 = Point3::new(0.0, 1.0, 0.0);

    let v0 = builder::vertex(p0);
    let v1 = builder::vertex(p1);
    let v2 = builder::vertex(p2);
    // v2_dup is at the exact same location as v2.
    let v2_dup = builder::vertex(p2);
    let v3 = builder::vertex(p3);

    let e01 = builder::line(&v0, &v1);
    let e12 = builder::line(&v1, &v2);
    // Zero-length edge from v2 to v2_dup (same 3D position).
    let e_collapsed = builder::line(&v2, &v2_dup);
    let e_dup3 = builder::line(&v2_dup, &v3);
    let e30 = builder::line(&v3, &v0);

    let wire: Wire = vec![e01, e12, e_collapsed, e_dup3, e30].into();
    let plane = Plane::new(p0, p1, p3);
    let surface: Surface = plane.into();
    let face = Face::new_unchecked(vec![wire], surface);
    vec![face].into()
}

/// Constructs a shell with a self-touching trim boundary (bowtie).
/// The wire visits a center point twice, creating a figure-8.
fn self_touching_shell() -> Shell {
    let center = Point3::new(0.5, 0.5, 0.0);
    let p0 = Point3::new(0.2, 0.2, 0.0);
    let p1 = Point3::new(0.8, 0.2, 0.0);
    let p2 = Point3::new(0.8, 0.8, 0.0);
    let p3 = Point3::new(0.2, 0.8, 0.0);

    // Two separate vertex handles at the center to form a closed wire.
    let vc0 = builder::vertex(center);
    let vc1 = builder::vertex(center);
    let vp0 = builder::vertex(p0);
    let vp1 = builder::vertex(p1);
    let vp2 = builder::vertex(p2);
    let vp3 = builder::vertex(p3);

    // Wire: p0 -> p1 -> center0 -> p2 -> p3 -> center1 -> p0
    let e_0_1 = builder::line(&vp0, &vp1);
    let e_1_c0 = builder::line(&vp1, &vc0);
    let e_c0_2 = builder::line(&vc0, &vp2);
    let e_2_3 = builder::line(&vp2, &vp3);
    let e_3_c1 = builder::line(&vp3, &vc1);
    let e_c1_0 = builder::line(&vc1, &vp0);

    let wire: Wire = vec![e_0_1, e_1_c0, e_c0_2, e_2_3, e_3_c1, e_c1_0].into();

    let plane = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let surface: Surface = plane.into();
    let face = Face::new_unchecked(vec![wire], surface);
    vec![face].into()
}

/// Creates a shell with multiple degenerate holes of varying sizes.
/// Tests that the degenerate loop filter works across a batch of loops.
fn face_with_multiple_degenerate_holes() -> Shell {
    // Outer boundary: unit square.
    let v0 = builder::vertex(Point3::origin());
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 1.0, 0.0));

    let e01 = builder::line(&v0, &v1);
    let e12 = builder::line(&v1, &v2);
    let e23 = builder::line(&v2, &v3);
    let e30 = builder::line(&v3, &v0);
    let outer_wire: Wire = vec![e01, e12, e23, e30].into();

    let mut wires = vec![outer_wire];

    // Add 5 tiny degenerate holes at different positions.
    for i in 0..5 {
        let eps = 1e-12;
        let cx = 0.2 + 0.15 * i as f64;
        let cy = 0.5;
        let center = Point3::new(cx, cy, 0.0);
        let h0 = builder::vertex(center);
        let h1 = builder::vertex(center + Vector3::new(eps, 0.0, 0.0));
        let h2 = builder::vertex(center + Vector3::new(0.0, eps, 0.0));

        let he01 = builder::line(&h0, &h1);
        let he12 = builder::line(&h1, &h2);
        let he20 = builder::line(&h2, &h0);
        wires.push(vec![he01, he12, he20].into());
    }

    let plane = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let surface: Surface = plane.into();
    let face = Face::new_unchecked(wires, surface);
    vec![face].into()
}

#[test]
fn near_zero_area_trim_loop_no_panic() {
    let shell = near_zero_area_shell();
    // Must not panic. Result can be Some (valid mesh) or None (gracefully skipped).
    let meshed = shell.robust_triangulation(0.01);
    let face = meshed.face_iter().next().unwrap();
    let surface = face.surface();
    if let Some(ref poly) = surface {
        assert!(
            !poly.positions().is_empty(),
            "mesh produced but has no positions"
        );
    }
}

/// A face with a degenerate inner hole should still produce a valid mesh
/// for the outer boundary. The degenerate inner loop should be detected
/// and skipped, preserving the face's mesh.
#[test]
fn degenerate_hole_preserves_face_mesh() {
    let shell = face_with_degenerate_hole();
    let meshed = shell.robust_triangulation(0.01);
    let face = meshed.face_iter().next().unwrap();
    let surface = face.surface();
    // After degenerate loop detection, the face should still produce a mesh.
    assert!(
        surface.is_some(),
        "face with degenerate hole should still produce a valid mesh"
    );
    let poly = surface.unwrap();
    // The mesh should have a reasonable number of triangles for a unit square.
    assert!(
        poly.tri_faces().len() >= 2,
        "unit square face should produce at least 2 triangles, got {}",
        poly.tri_faces().len()
    );
}

/// Multiple degenerate holes should all be filtered, leaving the face mesh intact.
#[test]
fn multiple_degenerate_holes_preserves_face_mesh() {
    let shell = face_with_multiple_degenerate_holes();
    let meshed = shell.robust_triangulation(0.01);
    let face = meshed.face_iter().next().unwrap();
    let surface = face.surface();
    assert!(
        surface.is_some(),
        "face with multiple degenerate holes should still produce a valid mesh"
    );
    let poly = surface.unwrap();
    assert!(
        poly.tri_faces().len() >= 2,
        "unit square face should produce at least 2 triangles, got {}",
        poly.tri_faces().len()
    );
}

#[test]
fn self_touching_trim_boundary_no_panic() {
    let shell = self_touching_shell();
    // Must not panic and should produce valid mesh output.
    let meshed = shell.robust_triangulation(0.01);
    let face = meshed.face_iter().next().unwrap();
    let surface = face.surface();
    if let Some(ref poly) = surface {
        assert!(
            !poly.positions().is_empty(),
            "mesh produced but has no positions"
        );
    }
}

#[test]
fn collapsed_edge_in_trim_wire_no_panic() {
    let shell = collapsed_edge_shell();
    // Must not panic.
    let meshed = shell.robust_triangulation(0.01);
    let face = meshed.face_iter().next().unwrap();
    let surface = face.surface();
    // The face should still produce a valid mesh despite the collapsed edge.
    assert!(
        surface.is_some(),
        "face with collapsed edge should still produce a valid mesh"
    );
}

#[test]
fn watertight_boundary_after_degenerate_trim() {
    // Construct two adjacent planar faces sharing an edge.
    let v0 = builder::vertex(Point3::origin());
    let v1 = builder::vertex(Point3::new(1.0, 0.0, 0.0));
    let v2 = builder::vertex(Point3::new(1.0, 1.0, 0.0));
    let v3 = builder::vertex(Point3::new(0.0, 1.0, 0.0));
    let v4 = builder::vertex(Point3::new(2.0, 0.0, 0.0));
    let v5 = builder::vertex(Point3::new(2.0, 1.0, 0.0));

    // Face 1: v0-v1-v2-v3 (unit square).
    let e01 = builder::line(&v0, &v1);
    let e12 = builder::line(&v1, &v2);
    let e23 = builder::line(&v2, &v3);
    let e30 = builder::line(&v3, &v0);

    let outer_wire1: Wire = vec![e01, e12.clone(), e23, e30].into();
    let face1 = builder::try_attach_plane(&[outer_wire1]).unwrap();

    // Face 2: v1-v4-v5-v2 (adjacent square sharing edge e12).
    let e14 = builder::line(&v1, &v4);
    let e45 = builder::line(&v4, &v5);
    let e52 = builder::line(&v5, &v2);

    let outer_wire2: Wire = vec![e14, e45, e52, e12.inverse()].into();
    let face2 = builder::try_attach_plane(&[outer_wire2]).unwrap();

    let shell: Shell = vec![face1, face2].into();
    let meshed = shell.robust_triangulation(0.01);
    let mut poly = meshed.to_polygon();
    poly.put_together_same_attrs(TOLERANCE * 2.0)
        .remove_degenerate_faces()
        .remove_unused_attrs();

    // The shared boundary should be watertight.
    let condition = poly.shell_condition();
    assert!(
        condition == ShellCondition::Closed || condition == ShellCondition::Oriented,
        "shared boundary should be watertight, got: {condition:?}"
    );
}

#[test]
fn robust_vs_regular_degenerate_trim() {
    // Build a face with a degenerate near-zero-area trim.
    let shell = near_zero_area_shell();

    let regular = shell.triangulation(0.01);
    let regular_meshed_count = regular
        .face_iter()
        .filter(|face| face.surface().is_some())
        .count();

    let robust = shell.robust_triangulation(0.01);
    let robust_meshed_count = robust
        .face_iter()
        .filter(|face| face.surface().is_some())
        .count();

    // Robust should recover at least as many faces as regular.
    assert!(
        robust_meshed_count >= regular_meshed_count,
        "robust_triangulation should recover at least as many faces: \
         robust={robust_meshed_count}, regular={regular_meshed_count}"
    );

    // The robust version should produce at least one face.
    assert!(
        robust_meshed_count > 0,
        "robust_triangulation should recover at least one face on degenerate input"
    );
}
