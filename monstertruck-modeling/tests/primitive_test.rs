use monstertruck_modeling::*;

#[test]
fn rect_xy_plane() {
    let bbox = BoundingBox::from_iter([Point2::new(-1.0, -2.0), Point2::new(2.0, 1.0)]);
    let rect: Wire = primitive::rect(bbox, Plane::xy());
    assert_eq!(rect.len(), 4);
    // Wire is closed: last edge's back vertex equals first edge's front vertex.
    assert_eq!(rect[3].back().id(), rect[0].front().id(),);
    // Verify vertex positions match corners mapped through XY plane.
    assert!(rect[0].front().point().near(&Point3::new(-1.0, -2.0, 0.0)));
    assert!(rect[1].front().point().near(&Point3::new(2.0, -2.0, 0.0)));
    assert!(rect[2].front().point().near(&Point3::new(2.0, 1.0, 0.0)));
    assert!(rect[3].front().point().near(&Point3::new(-1.0, 1.0, 0.0)));
}

#[test]
fn rect_zx_plane() {
    let bbox = BoundingBox::from_iter([Point2::new(-1.0, -2.0), Point2::new(2.0, 1.0)]);
    let rect: Wire = primitive::rect(bbox, Plane::zx());
    assert_eq!(rect.len(), 4);
    // ZX plane maps (u,v) -> (v, 0, u).
    assert!(rect[0].front().point().near(&Point3::new(-2.0, 0.0, -1.0)));
    assert!(rect[1].front().point().near(&Point3::new(-2.0, 0.0, 2.0)));
    assert!(rect[2].front().point().near(&Point3::new(1.0, 0.0, 2.0)));
    assert!(rect[3].front().point().near(&Point3::new(1.0, 0.0, -1.0)));
}

#[test]
fn circle_division_2() {
    let origin = Point3::origin();
    let start = Point3::new(3.0, 0.0, 0.0);
    let axis = Vector3::unit_z();
    let wire: Wire = primitive::circle(start, origin, axis, 2);
    assert_eq!(wire.len(), 2);
    // Wire is closed.
    assert_eq!(wire[1].back().id(), wire[0].front().id());
    // All vertices equidistant from origin (radius = 3).
    wire.vertex_iter().for_each(|v| {
        assert!(v.point().distance(origin).near(&3.0));
    });
}

#[test]
fn circle_division_4() {
    let origin = Point3::new(1.0, -2.0, 3.0);
    let axis = Vector3::unit_y();
    let start = origin + Vector3::new(5.0, 0.0, 0.0);
    let wire: Wire = primitive::circle(start, origin, axis, 4);
    assert_eq!(wire.len(), 4);
    // Wire is closed.
    assert_eq!(wire[3].back().id(), wire[0].front().id());
    // All vertices equidistant from origin (radius = 5).
    wire.vertex_iter().for_each(|v| {
        assert!(v.point().distance(origin).near(&5.0));
    });
}

#[test]
fn cuboid_topology() {
    let p = Point3::new(-1.0, 2.0, -3.0);
    let q = Point3::new(10.0, -5.0, 4.0);
    let bbox = BoundingBox::from_iter([p, q]);
    let solid: Solid = primitive::cuboid(bbox);
    assert_eq!(solid.boundaries().len(), 1);
    let shell = &solid.boundaries()[0];
    assert_eq!(shell.len(), 6);
    let unique_edges: std::collections::HashSet<_> = shell.edge_iter().map(|e| e.id()).collect();
    assert_eq!(unique_edges.len(), 12);
    let unique_verts: std::collections::HashSet<_> = shell.vertex_iter().map(|v| v.id()).collect();
    assert_eq!(unique_verts.len(), 8);
}

#[test]
fn cuboid_vertex_positions() {
    let p = Point3::new(-1.0, 2.0, -3.0);
    let q = Point3::new(10.0, -5.0, 4.0);
    let bbox = BoundingBox::from_iter([p, q]);
    let solid: Solid = primitive::cuboid(bbox);
    for v in solid.vertex_iter() {
        let x = v.point();
        assert!(x.x.near(&p.x) || x.x.near(&q.x));
        assert!(x.y.near(&p.y) || x.y.near(&q.y));
        assert!(x.z.near(&p.z) || x.z.near(&q.z));
    }
}

#[test]
fn cuboid_geometric_consistency() {
    let bbox = BoundingBox::from_iter([Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 3.0, 4.0)]);
    let solid: Solid = primitive::cuboid(bbox);
    assert!(solid.is_geometric_consistent());
}
