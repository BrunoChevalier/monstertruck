use monstertruck_topology::*;
use monstertruck_topology::errors::Error;
use std::panic::AssertUnwindSafe;

// ---------------------------------------------------------------------------
// Vertex tests
// ---------------------------------------------------------------------------

#[test]
fn test_vertex_creation_and_identity() {
    let v0 = Vertex::new(());
    let v1 = Vertex::new(());
    let v2 = v0.clone();

    // Different constructors yield different ids.
    assert_ne!(v0, v1);
    assert_ne!(v0.id(), v1.id());

    // Clone shares id.
    assert_eq!(v0, v2);
    assert_eq!(v0.id(), v2.id());

    // Batch creation.
    let batch = Vertex::news([(); 4]);
    assert_eq!(batch.len(), 4);
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(batch[i], batch[j]);
        }
    }
}

#[test]
fn test_vertex_point_synchronization() {
    let v0 = Vertex::new(42);
    let v1 = v0.clone();
    assert_eq!(v0.point(), 42);
    assert_eq!(v1.point(), 42);

    v0.set_point(99);
    assert_eq!(v0.point(), 99);
    assert_eq!(v1.point(), 99);
}

#[test]
fn test_vertex_count() {
    let v0 = Vertex::new(());
    assert_eq!(v0.count(), 1);

    let v1 = v0.clone();
    assert_eq!(v0.count(), 2);
    assert_eq!(v1.count(), 2);

    drop(v1);
    assert_eq!(v0.count(), 1);
}

#[test]
fn test_vertex_mapped() {
    let v0 = Vertex::new(5);
    let v1 = v0.mapped(|x| *x * 2);
    assert_eq!(v1.point(), 10);
    // Mapped vertex is a new vertex with a different id.
    assert_ne!(v0.id(), v1.id());
}

#[test]
fn test_vertex_display_formats() {
    let v = Vertex::new([1, 2]);
    let full = format!("{:?}", v.display(VertexDisplayFormat::Full));
    let id_tuple = format!("{:?}", v.display(VertexDisplayFormat::IDTuple));
    let point_tuple = format!("{:?}", v.display(VertexDisplayFormat::PointTuple));
    let as_point = format!("{:?}", v.display(VertexDisplayFormat::AsPoint));

    assert!(!full.is_empty());
    assert!(!id_tuple.is_empty());
    assert!(!point_tuple.is_empty());
    assert!(!as_point.is_empty());

    assert!(full.contains("Vertex"));
    assert!(id_tuple.contains("Vertex"));
    assert!(point_tuple.contains("Vertex"));
    assert_eq!(as_point, "[1, 2]");
}

// ---------------------------------------------------------------------------
// Edge tests
// ---------------------------------------------------------------------------

#[test]
fn test_edge_try_new_same_vertex() {
    let v = Vertex::new(());
    assert_eq!(Edge::try_new(&v, &v, ()), Err(Error::SameVertex));
}

#[test]
fn test_edge_new_different_vertices() {
    let v = Vertex::news([(), ()]);
    let edge = Edge::new(&v[0], &v[1], ());
    assert_eq!(edge.front(), &v[0]);
    assert_eq!(edge.back(), &v[1]);
}

#[test]
fn test_edge_inverse_and_orientation() {
    let v = Vertex::news([(), ()]);
    let edge = Edge::new(&v[0], &v[1], ());
    assert!(edge.orientation());

    let inv = edge.inverse();
    assert!(!inv.orientation());
    assert_eq!(edge.front(), inv.back());
    assert_eq!(edge.back(), inv.front());
    assert_eq!(edge.id(), inv.id());
    assert!(edge.is_same(&inv));
}

#[test]
fn test_edge_absolute_endpoints() {
    let v = Vertex::news([(), ()]);
    let edge = Edge::new(&v[0], &v[1], ());
    let inv = edge.inverse();

    // Absolute endpoints are invariant under inversion.
    assert_eq!(edge.absolute_front(), inv.absolute_front());
    assert_eq!(edge.absolute_back(), inv.absolute_back());

    // But front/back are swapped.
    assert_eq!(inv.front(), &v[1]);
    assert_eq!(inv.back(), &v[0]);
}

#[test]
fn test_edge_absolute_clone() {
    let v = Vertex::news([(), ()]);
    let edge0 = Edge::new(&v[0], &v[1], ());
    let edge1 = edge0.inverse();
    let edge2 = edge1.absolute_clone();

    assert!(edge2.orientation());
    assert!(edge1.is_same(&edge2));
    assert_eq!(edge0, edge2);
    assert_ne!(edge1, edge2);
}

#[test]
fn test_edge_curve_get_set() {
    let v = Vertex::news([(), ()]);
    let edge = Edge::new(&v[0], &v[1], 42);
    assert_eq!(edge.curve(), 42);

    edge.set_curve(99);
    let edge_clone = edge.clone();
    assert_eq!(edge.curve(), 99);
    assert_eq!(edge_clone.curve(), 99);
}

#[test]
fn test_edge_count() {
    let v = Vertex::news([(), ()]);
    let e0 = Edge::new(&v[0], &v[1], ());
    assert_eq!(e0.count(), 1);

    let e1 = e0.clone();
    assert_eq!(e0.count(), 2);
    assert_eq!(e1.count(), 2);

    drop(e1);
    assert_eq!(e0.count(), 1);
}

#[test]
fn test_edge_equality_vs_is_same() {
    let v = Vertex::news([(), ()]);
    let e0 = Edge::new(&v[0], &v[1], ());
    let e1 = Edge::new(&v[0], &v[1], ());

    // Two separately constructed edges have different ids.
    assert_ne!(e0, e1);
    assert_ne!(e0.id(), e1.id());
    assert!(!e0.is_same(&e1));

    // Cloned edges are equal.
    let e2 = e0.clone();
    assert_eq!(e0, e2);
    assert!(e0.is_same(&e2));

    // Inversed edges: not equal (different orientation) but is_same.
    let e3 = e0.inverse();
    assert_ne!(e0, e3);
    assert!(e0.is_same(&e3));
}

#[test]
fn test_edge_debug_new() {
    let v = Vertex::new(());
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        Edge::debug_new(&v, &v, ());
    }));
    // In debug mode, debug_new panics on same vertex.
    assert!(result.is_err());
}

#[test]
fn test_edge_display_formats() {
    let v = Vertex::news([0, 1]);
    let edge = Edge::new(&v[0], &v[1], 2);
    let vf = VertexDisplayFormat::AsPoint;

    let as_curve = format!("{:?}", edge.display(EdgeDisplayFormat::AsCurve));
    let vt = format!(
        "{:?}",
        edge.display(EdgeDisplayFormat::VerticesTuple {
            vertex_format: vf,
        })
    );
    let vts = format!(
        "{:?}",
        edge.display(EdgeDisplayFormat::VerticesTupleStruct {
            vertex_format: vf,
        })
    );
    let full = format!(
        "{:?}",
        edge.display(EdgeDisplayFormat::Full {
            vertex_format: vf,
        })
    );
    let vt_id = format!(
        "{:?}",
        edge.display(EdgeDisplayFormat::VerticesTupleAndID {
            vertex_format: vf,
        })
    );
    let vt_curve = format!(
        "{:?}",
        edge.display(EdgeDisplayFormat::VerticesTupleAndCurve {
            vertex_format: vf,
        })
    );

    assert!(!as_curve.is_empty());
    assert!(!vt.is_empty());
    assert!(!vts.is_empty());
    assert!(!full.is_empty());
    assert!(!vt_id.is_empty());
    assert!(!vt_curve.is_empty());

    assert_eq!(as_curve, "2");
    assert_eq!(vt, "(0, 1)");
    assert_eq!(vts, "Edge(0, 1)");
}

#[test]
fn test_edge_oriented_curve() {
    // Use (usize, usize) which implements Invertible.
    let v = Vertex::news([(), ()]);
    let edge = Edge::new(&v[0], &v[1], (0_usize, 1_usize));
    assert_eq!(edge.curve(), (0, 1));
    assert_eq!(edge.oriented_curve(), (0, 1));

    let inv = edge.inverse();
    // curve() returns absolute (non-inverted).
    assert_eq!(inv.curve(), (0, 1));
    // oriented_curve() returns inverted.
    assert_eq!(inv.oriented_curve(), (1, 0));
}

#[test]
fn test_edge_ends() {
    let v = Vertex::news([(), ()]);
    let edge = Edge::new(&v[0], &v[1], ());
    assert_eq!(edge.ends(), (&v[0], &v[1]));

    let inv = edge.inverse();
    assert_eq!(inv.ends(), (&v[1], &v[0]));
    assert_eq!(inv.absolute_ends(), (&v[0], &v[1]));
}

// ---------------------------------------------------------------------------
// Wire tests
// ---------------------------------------------------------------------------

#[test]
fn test_wire_new_empty() {
    let wire = Wire::<(), ()>::new();
    assert!(wire.is_empty());
    assert!(wire.is_continuous());
    assert!(wire.is_cyclic());
    assert!(wire.is_simple());
}

#[test]
fn test_wire_from_vec() {
    let v = Vertex::news([(); 3]);
    let e0 = Edge::new(&v[0], &v[1], ());
    let e1 = Edge::new(&v[1], &v[2], ());
    let wire = Wire::from(vec![e0, e1]);

    assert_eq!(wire.len(), 2);
    assert_eq!(wire.front_vertex(), Some(&v[0]));
    assert_eq!(wire.back_vertex(), Some(&v[2]));
    assert_eq!(wire.ends_vertices(), Some((&v[0], &v[2])));
}

#[test]
fn test_wire_with_capacity() {
    let wire = Wire::<(), ()>::with_capacity(10);
    assert!(wire.is_empty());
}

#[test]
fn test_wire_push_front_back() {
    let v = Vertex::news([(); 3]);
    let mut wire = Wire::new();
    wire.push_back(Edge::new(&v[1], &v[2], ()));
    wire.push_front(Edge::new(&v[0], &v[1], ()));

    assert_eq!(wire.front_vertex(), Some(&v[0]));
    assert_eq!(wire.back_vertex(), Some(&v[2]));
    assert!(wire.is_continuous());
}

#[test]
fn test_wire_is_continuous() {
    let v = Vertex::news([(); 4]);

    // Continuous wire.
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
    ]);
    assert!(wire.is_continuous());

    // Discontinuous wire.
    let wire2 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[2], &v[3], ()),
    ]);
    assert!(!wire2.is_continuous());
}

#[test]
fn test_wire_is_cyclic() {
    let v = Vertex::news([(); 3]);

    // Non-cyclic.
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
    ]);
    assert!(!wire.is_cyclic());

    // Cyclic (front vertex == back vertex).
    let mut wire2 = wire.clone();
    wire2.push_back(Edge::new(&v[2], &v[0], ()));
    assert!(wire2.is_cyclic());
}

#[test]
fn test_wire_is_closed() {
    let v = Vertex::news([(); 4]);

    // Continuous but not cyclic.
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
    ]);
    assert!(!wire.is_closed());

    // Cyclic but discontinuous.
    let wire2 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    assert!(!wire2.is_closed());

    // Properly closed.
    let wire3 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    assert!(wire3.is_closed());
}

#[test]
fn test_wire_is_simple() {
    let v = Vertex::news([(); 4]);

    // Simple closed wire (triangle).
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    assert!(wire.is_simple());

    // Non-simple wire (v[1] appears more than twice, figure-8 shape).
    let edge0 = Edge::new(&v[0], &v[1], ());
    let edge1 = Edge::new(&v[1], &v[2], ());
    let edge2 = Edge::new(&v[2], &v[3], ());
    let edge3 = Edge::new(&v[3], &v[1], ());
    let wire2 = wire![&edge0, &edge1, &edge2, &edge3];
    assert!(!wire2.is_simple());
}

#[test]
fn test_wire_disjoint_wires() {
    let v = Vertex::news([(); 5]);
    let e0 = Edge::new(&v[0], &v[1], ());
    let e1 = Edge::new(&v[1], &v[2], ());
    let e2 = Edge::new(&v[2], &v[3], ());
    let e3 = Edge::new(&v[3], &v[4], ());

    let wire0 = Wire::from(vec![e0, e1]);
    let wire1 = Wire::from(vec![e2]);
    let wire2 = Wire::from(vec![e3]);

    // wire0 and wire1 share vertex v[2].
    assert!(!Wire::disjoint_wires(&[wire0.clone(), wire1]));
    // wire0 and wire2 share no vertices.
    assert!(Wire::disjoint_wires(&[wire0, wire2]));
}

#[test]
fn test_wire_invert() {
    let v = Vertex::news([(); 4]);
    let mut wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
    ]);

    wire.invert();

    // After inversion the vertex order is reversed.
    let verts: Vec<_> = wire.vertex_iter().collect();
    assert_eq!(verts[0], v[3]);
    assert_eq!(verts[1], v[2]);
    assert_eq!(verts[2], v[1]);
    assert_eq!(verts[3], v[0]);
}

#[test]
fn test_wire_inverse() {
    let v = Vertex::news([(); 4]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
    ]);

    let inv = wire.inverse();
    let mut wire2 = wire.clone();
    wire2.invert();

    // inverse() and invert() produce the same result.
    for (e0, e1) in inv.edge_iter().zip(wire2.edge_iter()) {
        assert_eq!(e0, e1);
    }
}

#[test]
fn test_wire_split_off() {
    let v = Vertex::news([(); 7]);
    let mut wire = Wire::new();
    for i in 0..6 {
        wire.push_back(Edge::new(&v[i], &v[i + 1], ()));
    }
    let original = wire.clone();

    let mut wire1 = wire.split_off(4);
    assert_eq!(wire.len(), 4);
    assert_eq!(wire1.len(), 2);

    wire.append(&mut wire1);
    assert_eq!(wire, original);
}

#[test]
fn test_wire_swap_edge_into_wire() {
    let v = Vertex::news([(); 5]);
    let edge0 = Edge::new(&v[0], &v[1], 0);
    let edge1 = Edge::new(&v[1], &v[3], 1);
    let edge2 = Edge::new(&v[3], &v[4], 2);
    let edge3 = Edge::new(&v[1], &v[2], 3);
    let edge4 = Edge::new(&v[2], &v[3], 4);

    let mut wire = Wire::from(vec![edge0.clone(), edge1, edge2.clone()]);
    let expected = Wire::from(vec![edge0, edge3.clone(), edge4.clone(), edge2]);

    assert_ne!(wire, expected);
    let result = wire.swap_edge_into_wire(1, Wire::from(vec![edge3, edge4]));
    assert!(result);
    assert_eq!(wire, expected);
}

#[test]
fn test_wire_swap_edge_into_wire_failure() {
    let v = Vertex::news([(); 5]);
    let edge0 = Edge::new(&v[0], &v[1], 0);
    let edge1 = Edge::new(&v[1], &v[3], 1);
    let edge2 = Edge::new(&v[3], &v[4], 2);
    // These edges have endpoints (v[1], v[1]) -- does not match edge1's (v[1], v[3]).
    let edge3 = Edge::new(&v[1], &v[2], 3);
    let edge4 = Edge::new(&v[2], &v[1], 4);

    let mut wire = Wire::from(vec![edge0, edge1, edge2]);
    let backup = wire.clone();

    let result = wire.swap_edge_into_wire(1, Wire::from(vec![edge3, edge4]));
    assert!(!result);
    assert_eq!(wire, backup);
}

#[test]
fn test_wire_vertex_iter() {
    let v = Vertex::news([(); 3]);
    // Closed triangular wire.
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);

    let verts: Vec<_> = wire.vertex_iter().collect();
    // Cyclic wire: does not repeat the start vertex at the end.
    assert_eq!(verts.len(), 3);
    assert_eq!(verts[0], v[0]);
    assert_eq!(verts[1], v[1]);
    assert_eq!(verts[2], v[2]);
}

#[test]
fn test_wire_vertex_iter_open() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
    ]);

    let verts: Vec<_> = wire.vertex_iter().collect();
    assert_eq!(verts.len(), 3);
    assert_eq!(verts[0], v[0]);
    assert_eq!(verts[1], v[1]);
    assert_eq!(verts[2], v[2]);
}

#[test]
fn test_wire_edge_iter() {
    let v = Vertex::news([(); 4]);
    let edges = vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
    ];
    let wire = Wire::from(edges.clone());

    let iterated: Vec<_> = wire.edge_iter().collect();
    assert_eq!(iterated.len(), 3);
    for (a, b) in iterated.iter().zip(edges.iter()) {
        assert_eq!(*a, b);
    }
}

#[test]
fn test_wire_display_formats() {
    let v = Vertex::news([0, 1, 2]);
    let wire: Wire<usize, usize> = Wire::from(vec![
        Edge::new(&v[0], &v[1], 100),
        Edge::new(&v[1], &v[2], 200),
    ]);

    let vf = VertexDisplayFormat::AsPoint;
    let ef = EdgeDisplayFormat::VerticesTuple {
        vertex_format: vf,
    };

    let elt = format!(
        "{:?}",
        wire.display(WireDisplayFormat::EdgesListTuple { edge_format: ef })
    );
    let el = format!(
        "{:?}",
        wire.display(WireDisplayFormat::EdgesList { edge_format: ef })
    );
    let vl = format!(
        "{:?}",
        wire.display(WireDisplayFormat::VerticesList {
            vertex_format: vf,
        })
    );

    assert!(!elt.is_empty());
    assert!(!el.is_empty());
    assert!(!vl.is_empty());

    assert!(elt.contains("Wire"));
    assert_eq!(el, "[(0, 1), (1, 2)]");
    assert_eq!(vl, "[0, 1, 2]");
}

#[test]
fn test_wire_clone_eq() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
    ]);
    let mut clone = wire.clone();
    assert_eq!(wire, clone);

    // Modify the clone by pushing an edge.
    let v3 = Vertex::new(());
    clone.push_back(Edge::new(&v[2], &v3, ()));
    assert_ne!(wire, clone);
}

#[test]
fn test_wire_macro() {
    let v = Vertex::news([(); 3]);
    let e0 = Edge::new(&v[0], &v[1], ());
    let e1 = Edge::new(&v[1], &v[2], ());
    let e2 = Edge::new(&v[2], &v[0], ());

    let wire_macro = wire![e0.clone(), e1.clone(), e2.clone()];
    let wire_from_iter = Wire::from_iter([e0, e1, e2]);

    assert_eq!(wire_macro, wire_from_iter);
}
