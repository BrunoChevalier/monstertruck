use monstertruck_topology::compress::CompressedShell;
use monstertruck_topology::errors::Error;
use monstertruck_topology::shell::ShellCondition;
use monstertruck_topology::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a tetrahedron shell (4 triangular faces, closed, oriented).
fn tetrahedron_shell() -> Shell<(), (), ()> {
    let v = Vertex::news([(); 4]);
    let edge = [
        Edge::new(&v[0], &v[1], ()), // 0
        Edge::new(&v[0], &v[2], ()), // 1
        Edge::new(&v[0], &v[3], ()), // 2
        Edge::new(&v[1], &v[2], ()), // 3
        Edge::new(&v[1], &v[3], ()), // 4
        Edge::new(&v[2], &v[3], ()), // 5
    ];
    let wire = vec![
        wire![&edge[0], &edge[3], &edge[1].inverse()],
        wire![&edge[1], &edge[5], &edge[2].inverse()],
        wire![&edge[2], &edge[4].inverse(), &edge[0].inverse()],
        wire![&edge[3], &edge[5], &edge[4].inverse()],
    ];
    let mut faces: Vec<Face<(), (), ()>> =
        wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    faces[3].invert();
    faces.into()
}

/// Build a cube shell (6 quadrilateral faces, closed, oriented).
fn cube_shell() -> Shell<(), (), ()> {
    let v = Vertex::news([(); 8]);
    let edge = [
        Edge::new(&v[0], &v[1], ()), // 0
        Edge::new(&v[1], &v[2], ()), // 1
        Edge::new(&v[2], &v[3], ()), // 2
        Edge::new(&v[3], &v[0], ()), // 3
        Edge::new(&v[0], &v[4], ()), // 4
        Edge::new(&v[1], &v[5], ()), // 5
        Edge::new(&v[2], &v[6], ()), // 6
        Edge::new(&v[3], &v[7], ()), // 7
        Edge::new(&v[4], &v[5], ()), // 8
        Edge::new(&v[5], &v[6], ()), // 9
        Edge::new(&v[6], &v[7], ()), // 10
        Edge::new(&v[7], &v[4], ()), // 11
    ];
    let wire = vec![
        wire![&edge[0], &edge[1], &edge[2], &edge[3]],
        wire![&edge[4], &edge[8], &edge[5].inverse(), &edge[0].inverse()],
        wire![&edge[5], &edge[9], &edge[6].inverse(), &edge[1].inverse()],
        wire![&edge[6], &edge[10], &edge[7].inverse(), &edge[2].inverse()],
        wire![&edge[7], &edge[11], &edge[4].inverse(), &edge[3].inverse()],
        wire![
            &edge[11].inverse(),
            &edge[10].inverse(),
            &edge[9].inverse(),
            &edge[8].inverse()
        ],
    ];
    wire.into_iter().map(|w| Face::new(vec![w], ())).collect()
}

// ===========================================================================
// Face creation and validation tests
// ===========================================================================

#[test]
fn test_face_try_new_empty_wire() {
    let result = Face::try_new(vec![Wire::<(), ()>::new()], ());
    assert_eq!(result, Err(Error::EmptyWire));
}

#[test]
fn test_face_try_new_not_closed() {
    let v = Vertex::news([(); 2]);
    let wire = Wire::from(vec![Edge::new(&v[0], &v[1], ())]);
    let result = Face::try_new(vec![wire], ());
    assert_eq!(result, Err(Error::NotClosedWire));
}

#[test]
fn test_face_try_new_not_simple() {
    // Figure-8: v0->v1->v2->v3->v1->v0 (vertex v1 visited twice).
    let v = Vertex::news([(); 4]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[1], ()),
        Edge::new(&v[1], &v[0], ()),
    ]);
    let result = Face::try_new(vec![wire], ());
    assert_eq!(result, Err(Error::NotSimpleWire));
}

#[test]
fn test_face_try_new_success() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    assert!(Face::try_new(vec![wire], ()).is_ok());
}

#[test]
fn test_face_new_quad() {
    let v = Vertex::news([(); 4]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 4);
}

#[test]
fn test_face_try_new_non_disjoint_wires() {
    // Two boundary wires sharing vertex v[2].
    let v = Vertex::news([(); 5]);
    let wire0 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let wire1 = Wire::from(vec![
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[2], ()),
    ]);
    let result = Face::try_new(vec![wire0, wire1], ());
    assert_eq!(result, Err(Error::NotSimpleWire));
}

// ===========================================================================
// Face identity and properties
// ===========================================================================

#[test]
fn test_face_id_uniqueness() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face0 = Face::new(vec![wire.clone()], ());
    let face1 = Face::new(vec![wire], ());
    // Different Face::new calls yield different ids.
    assert_ne!(face0.id(), face1.id());
    // Cloned face shares id.
    let face2 = face0.clone();
    assert_eq!(face0.id(), face2.id());
    // Inverse face has same id.
    let face3 = face0.inverse();
    assert_eq!(face0.id(), face3.id());
}

#[test]
fn test_face_is_same() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face0 = Face::new(vec![wire.clone()], ());
    let face1 = face0.inverse();
    assert!(face0.is_same(&face1));
    let face2 = Face::new(vec![wire], ());
    assert!(!face0.is_same(&face2));
}

#[test]
fn test_face_count() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face0 = Face::new(vec![wire], ());
    assert_eq!(face0.count(), 1);
    let face1 = face0.clone();
    assert_eq!(face0.count(), 2);
    assert_eq!(face1.count(), 2);
    drop(face1);
    assert_eq!(face0.count(), 1);
}

#[test]
fn test_face_surface_get_set() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], 42);
    assert_eq!(face.surface(), 42);
    face.set_surface(99);
    let cloned = face.clone();
    assert_eq!(cloned.surface(), 99);
}

#[test]
fn test_face_orientation() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let mut face = Face::new(vec![wire], ());
    assert!(face.orientation());
    face.invert();
    assert!(!face.orientation());
}

// ===========================================================================
// Face boundary traversal
// ===========================================================================

#[test]
fn test_face_boundaries() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let boundaries = face.boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 3);
    let verts: Vec<_> = boundaries[0].vertex_iter().collect();
    assert_eq!(verts, vec![v[0].clone(), v[1].clone(), v[2].clone()]);
}

#[test]
fn test_face_boundaries_inverted() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let mut face = Face::new(vec![wire.clone()], ());
    let orig = face.boundaries();
    face.invert();
    let inverted = face.boundaries();
    assert_eq!(inverted[0], orig[0].inverse());
}

#[test]
fn test_face_absolute_boundaries() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let mut face = Face::new(vec![wire], ());
    let orig = face.boundaries();
    face.invert();
    // absolute_boundaries returns the original (non-inverted) wire.
    assert_eq!(face.absolute_boundaries(), &orig);
}

#[test]
fn test_face_absolute_clone() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face0 = Face::new(vec![wire], ());
    let face1 = face0.inverse();
    let face2 = face1.absolute_clone();
    assert!(face2.orientation());
    assert!(face2.is_same(&face0));
    assert_eq!(face0, face2);
}

#[test]
fn test_face_into_boundaries() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let boundaries = face.boundaries().clone();
    let into = face.into_boundaries();
    assert_eq!(into, boundaries);
}

#[test]
fn test_face_boundary_iters() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire.clone()], ());
    let biters = face.boundary_iters();
    assert_eq!(biters.len(), 1);
    let edges: Vec<_> = biters.into_iter().flatten().collect();
    assert_eq!(edges.len(), 3);
    for (e, w) in edges.iter().zip(wire.iter()) {
        assert_eq!(e, w);
    }

    // Inverted face: boundary_iters yields inverted edges.
    let mut inv_face = face.clone();
    inv_face.invert();
    let inv_biters = inv_face.boundary_iters();
    let inv_edges: Vec<_> = inv_biters.into_iter().flatten().collect();
    let wire_inv = wire.inverse();
    for (e, w) in inv_edges.iter().zip(wire_inv.iter()) {
        assert_eq!(e, w);
    }
}

#[test]
fn test_face_boundary_iter_double_ended() {
    let v = Vertex::news([(); 4]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[0], ()),
    ]);
    let face = Face::new(vec![wire.clone()], ());
    let iter = &mut face.boundary_iters()[0];
    assert_eq!(iter.next().as_ref(), Some(&wire[0]));
    assert_eq!(iter.next_back().as_ref(), Some(&wire[3]));
    assert_eq!(iter.next().as_ref(), Some(&wire[1]));
    assert_eq!(iter.next().as_ref(), Some(&wire[2]));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_face_edge_iter() {
    let v = Vertex::news([(); 6]);
    let wire0 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let wire1 = Wire::from(vec![
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ()),
    ]);
    let face = Face::new(vec![wire0, wire1], ());
    let count = face.edge_iter().count();
    assert_eq!(count, 6);
}

#[test]
fn test_face_vertex_iter() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let verts: Vec<_> = face.vertex_iter().collect();
    assert_eq!(verts.len(), 3);
}

#[test]
fn test_face_add_boundary() {
    let v = Vertex::news([(); 6]);
    let wire0 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let wire1 = Wire::from(vec![
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ()),
    ]);
    let mut face = Face::new(vec![wire0], ());
    face.add_boundary(wire1);
    assert_eq!(face.boundaries().len(), 2);
}

#[test]
fn test_face_try_add_boundary_errors() {
    let v = Vertex::news([(); 6]);
    let wire0 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);

    // Empty wire.
    let mut face = Face::new(vec![wire0.clone()], ());
    assert_eq!(
        face.try_add_boundary(Wire::new()),
        Err(Error::EmptyWire)
    );

    // Open wire.
    let mut face = Face::new(vec![wire0.clone()], ());
    let open_wire = Wire::from(vec![Edge::new(&v[3], &v[4], ())]);
    assert_eq!(
        face.try_add_boundary(open_wire),
        Err(Error::NotClosedWire)
    );

    // Non-simple wire: figure-8 shape.
    let v2 = Vertex::news([(); 4]);
    let non_simple_closed = Wire::from(vec![
        Edge::new(&v2[0], &v2[1], ()),
        Edge::new(&v2[1], &v2[2], ()),
        Edge::new(&v2[2], &v2[3], ()),
        Edge::new(&v2[3], &v2[1], ()),
        Edge::new(&v2[1], &v2[0], ()),
    ]);
    let mut face = Face::new(vec![wire0.clone()], ());
    assert_eq!(
        face.try_add_boundary(non_simple_closed),
        Err(Error::NotSimpleWire)
    );

    // Non-disjoint wire (shares vertex with existing boundary).
    let non_disjoint = Wire::from(vec![
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[2], ()),
    ]);
    let mut face = Face::new(vec![wire0], ());
    assert_eq!(
        face.try_add_boundary(non_disjoint),
        Err(Error::NotDisjointWires)
    );
}

// ===========================================================================
// Face cutting and gluing
// ===========================================================================

#[test]
fn test_face_cut_by_edge() {
    let v = Vertex::news([(); 4]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let (face0, face1) = face.cut_by_edge(Edge::new(&v[1], &v[3], ())).unwrap();
    let v0: Vec<Vertex<()>> = face0.boundaries()[0].vertex_iter().collect();
    assert_eq!(v0, vec![v[3].clone(), v[0].clone(), v[1].clone()]);
    let v1: Vec<Vertex<()>> = face1.boundaries()[0].vertex_iter().collect();
    assert_eq!(v1, vec![v[1].clone(), v[2].clone(), v[3].clone()]);
}

#[test]
fn test_face_cut_by_edge_fails_multiple_boundaries() {
    let v = Vertex::news([(); 6]);
    let wire0 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let wire1 = Wire::from(vec![
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ()),
    ]);
    let face = Face::new(vec![wire0, wire1], ());
    assert!(face.cut_by_edge(Edge::new(&v[1], &v[2], ())).is_none());
}

#[test]
fn test_face_cut_by_edge_fails_missing_vertex() {
    let v = Vertex::news([(); 5]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    assert!(face.cut_by_edge(Edge::new(&v[1], &v[4], ())).is_none());
}

#[test]
fn test_face_cut_by_wire() {
    let v = Vertex::news([(); 5]);
    let boundary = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[0], ()),
    ]);
    let face = Face::new(vec![boundary], ());
    let cut_wire = Wire::from(vec![
        Edge::new(&v[1], &v[4], ()),
        Edge::new(&v[4], &v[3], ()),
    ]);
    let (face0, face1) = face.cut_by_wire(cut_wire).unwrap();
    let v0: Vec<Vertex<()>> = face0.boundaries()[0].vertex_iter().collect();
    assert_eq!(
        v0,
        vec![
            v[3].clone(),
            v[0].clone(),
            v[1].clone(),
            v[4].clone()
        ]
    );
    let v1: Vec<Vertex<()>> = face1.boundaries()[0].vertex_iter().collect();
    assert_eq!(
        v1,
        vec![
            v[1].clone(),
            v[2].clone(),
            v[3].clone(),
            v[4].clone()
        ]
    );
}

#[test]
fn test_face_border_on() {
    let v = Vertex::news([(); 4]);
    let shared_edge = Edge::new(&v[0], &v[1], ());
    let wire0 = wire![
        &Edge::new(&v[2], &v[0], ()),
        &shared_edge,
        &Edge::new(&v[1], &v[2], ())
    ];
    let face0 = Face::new(vec![wire0], ());
    let wire1 = wire![
        &Edge::new(&v[3], &v[0], ()),
        &shared_edge,
        &Edge::new(&v[1], &v[3], ())
    ];
    let face1 = Face::new(vec![wire1], ());
    assert!(face0.border_on(&face1));

    // Faces with no shared edge.
    let another_edge = Edge::new(&v[0], &v[1], ());
    let wire2 = wire![
        &Edge::new(&v[2], &v[0], ()),
        &another_edge,
        &Edge::new(&v[1], &v[2], ())
    ];
    let face2 = Face::new(vec![wire2], ());
    assert!(!face2.border_on(&face1));
}

#[test]
fn test_face_border_wires() {
    let v = Vertex::news([(); 5]);
    let shared_edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[2], &v[3], ()),
        Edge::new(&v[3], &v[4], ()),
    ];
    let boundary0 = wire![
        shared_edge[0].clone(),
        Edge::new(&v[1], &v[2], ()),
        shared_edge[1].clone(),
        shared_edge[2].clone(),
        Edge::new(&v[4], &v[0], ()),
    ];
    let face0 = Face::new(vec![boundary0], ());
    let boundary1 = wire![
        shared_edge[1].inverse(),
        Edge::new(&v[2], &v[1], ()),
        shared_edge[0].inverse(),
        Edge::new(&v[0], &v[4], ()),
        shared_edge[2].inverse(),
    ];
    let face1 = Face::new(vec![boundary1], ());
    let borders = face0.border_wires(&face1);
    assert_eq!(borders.len(), 2);
}

#[test]
fn test_face_glue_at_boundaries() {
    let v = Vertex::news([(); 8]);
    let edge = vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ()),
        Edge::new(&v[6], &v[2], ()),
        Edge::new(&v[1], &v[6], ()),
        Edge::new(&v[7], &v[5], ()),
        Edge::new(&v[4], &v[7], ()),
    ];
    let wire0 = Wire::from(vec![edge[0].clone(), edge[1].clone(), edge[2].clone()]);
    let wire1 = Wire::from(vec![edge[3].clone(), edge[4].clone(), edge[5].clone()]);
    let wire2 = Wire::from(vec![edge[6].clone(), edge[1].inverse(), edge[7].clone()]);
    let wire3 = Wire::from(vec![edge[8].clone(), edge[4].inverse(), edge[9].clone()]);
    let face0 = Face::new(vec![wire0, wire1], ());
    let face1 = Face::new(vec![wire2, wire3], ());
    let face = face0.glue_at_boundaries(&face1).unwrap();
    let boundaries = face.boundary_iters();
    assert_eq!(boundaries.len(), 2);
    assert_eq!(boundaries[0].len(), 4);
    assert_eq!(boundaries[1].len(), 4);
}

#[test]
fn test_face_display_formats() {
    let v = Vertex::news([0, 1, 2]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], 42);
    let vertex_format = VertexDisplayFormat::AsPoint;
    let edge_format = EdgeDisplayFormat::VerticesTuple { vertex_format };
    let wire_format = WireDisplayFormat::EdgesList { edge_format };

    let s = format!("{:?}", face.display(FaceDisplayFormat::Full { wire_format }));
    assert!(!s.is_empty());
    let s = format!(
        "{:?}",
        face.display(FaceDisplayFormat::BoundariesAndID { wire_format })
    );
    assert!(!s.is_empty());
    let s = format!(
        "{:?}",
        face.display(FaceDisplayFormat::BoundariesAndSurface { wire_format })
    );
    assert!(!s.is_empty());
    let s = format!(
        "{:?}",
        face.display(FaceDisplayFormat::LoopsListTuple { wire_format })
    );
    assert!(!s.is_empty());
    let s = format!(
        "{:?}",
        face.display(FaceDisplayFormat::LoopsList { wire_format })
    );
    assert!(!s.is_empty());
    let s = format!("{:?}", face.display(FaceDisplayFormat::AsSurface));
    assert!(!s.is_empty());
}

// ===========================================================================
// Shell construction tests
// ===========================================================================

#[test]
fn test_shell_new_empty() {
    let shell = Shell::<(), (), ()>::new();
    assert!(shell.is_empty());
    assert!(shell.is_connected());
}

#[test]
fn test_shell_with_capacity() {
    let shell = Shell::<(), (), ()>::with_capacity(10);
    assert!(shell.is_empty());
}

#[test]
fn test_shell_from_vec() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let shell = Shell::from(vec![face]);
    assert_eq!(shell.len(), 1);
    assert_eq!(shell.face_iter().count(), 1);
}

#[test]
fn test_shell_from_iter() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let shell: Shell<_, _, _> = vec![face].into_iter().collect();
    assert_eq!(shell.len(), 1);
}

#[test]
fn test_shell_push_and_deref() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let face = Face::new(vec![wire], ());
    let mut shell = Shell::new();
    shell.push(face.clone());
    // Access via Deref to Vec.
    assert_eq!(&shell[0], &face);
}

#[test]
fn test_shell_append() {
    let v = Vertex::news([(); 6]);
    let wire0 = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let wire1 = Wire::from(vec![
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ()),
    ]);
    let mut shell0 = Shell::from(vec![Face::new(vec![wire0], ())]);
    let mut shell1 = Shell::from(vec![Face::new(vec![wire1], ())]);
    shell0.append(&mut shell1);
    assert_eq!(shell0.len(), 2);
    assert!(shell1.is_empty());
}

#[test]
fn test_shell_macro() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let shell = shell![Face::new(vec![wire], ())];
    assert_eq!(shell.len(), 1);
}

// ===========================================================================
// Shell condition tests
// ===========================================================================

#[test]
fn test_shell_condition_irregular() {
    // Three faces share edge[0] -> irregular.
    let v = Vertex::news([(); 5]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[0], &v[2], ()),
        Edge::new(&v[0], &v[3], ()),
        Edge::new(&v[0], &v[4], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[1], &v[4], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[4], &edge[1].inverse()],
        wire![&edge[0], &edge[5], &edge[2].inverse()],
        wire![&edge[0], &edge[6], &edge[3].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert_eq!(shell.shell_condition(), ShellCondition::Irregular);
}

#[test]
fn test_shell_condition_regular() {
    // Orientations incompatible on shared edge.
    let v = Vertex::news([(); 6]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[0], &v[2], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[1], &v[4], ()),
        Edge::new(&v[2], &v[4], ()),
        Edge::new(&v[2], &v[5], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[2], &edge[1].inverse()],
        wire![&edge[3], &edge[7], &edge[4].inverse()],
        wire![&edge[5], &edge[8], &edge[6].inverse()],
        // Note: edge[2] used in same orientation as face[0] -> incompatible.
        wire![&edge[2], &edge[5], &edge[4].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert_eq!(shell.shell_condition(), ShellCondition::Regular);
}

#[test]
fn test_shell_condition_oriented() {
    let v = Vertex::news([(); 6]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[0], &v[2], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[1], &v[4], ()),
        Edge::new(&v[2], &v[4], ()),
        Edge::new(&v[2], &v[5], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[2], &edge[1].inverse()],
        wire![&edge[3], &edge[7], &edge[4].inverse()],
        wire![&edge[5], &edge[8], &edge[6].inverse()],
        wire![&edge[2].inverse(), &edge[4], &edge[5].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert_eq!(shell.shell_condition(), ShellCondition::Oriented);
}

#[test]
fn test_shell_condition_closed() {
    let shell = cube_shell();
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
}

#[test]
fn test_shell_condition_bitand() {
    assert_eq!(
        ShellCondition::Closed & ShellCondition::Oriented,
        ShellCondition::Oriented
    );
    assert_eq!(
        ShellCondition::Closed & ShellCondition::Regular,
        ShellCondition::Regular
    );
    assert_eq!(
        ShellCondition::Closed & ShellCondition::Irregular,
        ShellCondition::Irregular
    );
    assert_eq!(
        ShellCondition::Oriented & ShellCondition::Regular,
        ShellCondition::Regular
    );
    assert_eq!(
        ShellCondition::Closed & ShellCondition::Closed,
        ShellCondition::Closed
    );
}

// ===========================================================================
// Shell connectivity tests
// ===========================================================================

#[test]
fn test_shell_is_connected_single_face() {
    let v = Vertex::news([(); 3]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
    ]);
    let shell = Shell::from(vec![Face::new(vec![wire], ())]);
    assert!(shell.is_connected());
}

#[test]
fn test_shell_is_connected_shared_edge() {
    let v = Vertex::news([(); 4]);
    let shared_edge = Edge::new(&v[1], &v[2], ());
    let wire0 = wire![
        &Edge::new(&v[0], &v[1], ()),
        &shared_edge,
        &Edge::new(&v[2], &v[0], ())
    ];
    let wire1 = wire![
        &Edge::new(&v[3], &v[1], ()),
        &shared_edge,
        &Edge::new(&v[2], &v[3], ())
    ];
    let shell = Shell::from(vec![
        Face::new(vec![wire0], ()),
        Face::new(vec![wire1], ()),
    ]);
    assert!(shell.is_connected());
}

#[test]
fn test_shell_is_connected_disconnected() {
    let v = Vertex::news([(); 6]);
    let wire0 = wire![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ())
    ];
    let wire1 = wire![
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ())
    ];
    let shell = Shell::from(vec![
        Face::new(vec![wire0], ()),
        Face::new(vec![wire1], ()),
    ]);
    assert!(!shell.is_connected());
}

#[test]
fn test_shell_connected_components() {
    let v = Vertex::news([(); 6]);
    let wire0 = wire![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ())
    ];
    let wire1 = wire![
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
        Edge::new(&v[5], &v[3], ())
    ];
    let shell = Shell::from(vec![
        Face::new(vec![wire0], ()),
        Face::new(vec![wire1], ()),
    ]);
    let components = shell.connected_components();
    assert_eq!(components.len(), 2);
    assert_eq!(components[0].len(), 1);
    assert_eq!(components[1].len(), 1);
}

#[test]
fn test_shell_connected_components_empty() {
    let shell = Shell::<(), (), ()>::new();
    assert!(shell.connected_components().is_empty());
}

// ===========================================================================
// Shell adjacency tests
// ===========================================================================

#[test]
fn test_shell_vertex_adjacency() {
    let v = Vertex::news([(); 4]);
    let edge = [
        Edge::new(&v[0], &v[2], ()),
        Edge::new(&v[0], &v[3], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[2], &v[3], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[4], &edge[1].inverse()],
        wire![&edge[2], &edge[4], &edge[3].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    let adjacency = shell.vertex_adjacency();
    // v[0] should be adjacent to v[2] and v[3].
    let v0_adj = adjacency.get(&v[0].id()).unwrap();
    assert_eq!(v0_adj.len(), 2);
}

#[test]
fn test_shell_face_adjacency() {
    let v = Vertex::news([(); 6]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[0], &v[2], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[1], &v[4], ()),
        Edge::new(&v[2], &v[4], ()),
        Edge::new(&v[2], &v[5], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[2], &edge[1].inverse()],
        wire![&edge[3], &edge[7], &edge[4].inverse()],
        wire![&edge[5], &edge[8], &edge[6].inverse()],
        wire![&edge[2].inverse(), &edge[4], &edge[5].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    let face_adjacency = shell.face_adjacency();
    // shell[3] is the central face, adjacent to 3 others.
    assert_eq!(face_adjacency[&shell[3]].len(), 3);
    assert_eq!(face_adjacency[&shell[0]].len(), 1);
}

// ===========================================================================
// Shell boundary and singular vertex tests
// ===========================================================================

#[test]
fn test_shell_extract_boundaries() {
    let v = Vertex::news([(); 6]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[0], &v[2], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[1], &v[4], ()),
        Edge::new(&v[2], &v[4], ()),
        Edge::new(&v[2], &v[5], ()),
        Edge::new(&v[3], &v[4], ()),
        Edge::new(&v[4], &v[5], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[2], &edge[1].inverse()],
        wire![&edge[3], &edge[7], &edge[4].inverse()],
        wire![&edge[5], &edge[8], &edge[6].inverse()],
        wire![&edge[2].inverse(), &edge[4], &edge[5].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    let boundaries = shell.extract_boundaries();
    assert_eq!(boundaries.len(), 1);
    assert_eq!(boundaries[0].len(), 6);
}

#[test]
fn test_shell_singular_vertices_none() {
    // Mobius strip pattern: regular manifold with no singular vertices.
    let v = Vertex::news([(); 4]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[0], ()),
        Edge::new(&v[1], &v[3], ()),
        Edge::new(&v[3], &v[2], ()),
        Edge::new(&v[0], &v[3], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[3], &edge[4], &edge[2]],
        wire![&edge[1], &edge[2], &edge[5], &edge[3].inverse()],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert!(shell.singular_vertices().is_empty());
}

#[test]
fn test_shell_singular_vertices_present() {
    // Wedge of two spheres: v[0] is a singular vertex.
    let v = Vertex::news([(); 7]);
    let edge = [
        Edge::new(&v[0], &v[1], ()), // 0
        Edge::new(&v[0], &v[2], ()), // 1
        Edge::new(&v[0], &v[3], ()), // 2
        Edge::new(&v[1], &v[2], ()), // 3
        Edge::new(&v[2], &v[3], ()), // 4
        Edge::new(&v[3], &v[1], ()), // 5
        Edge::new(&v[0], &v[4], ()), // 6
        Edge::new(&v[0], &v[5], ()), // 7
        Edge::new(&v[0], &v[6], ()), // 8
        Edge::new(&v[4], &v[5], ()), // 9
        Edge::new(&v[5], &v[6], ()), // 10
        Edge::new(&v[6], &v[4], ()), // 11
    ];
    let wire = vec![
        wire![&edge[0].inverse(), &edge[1], &edge[3].inverse()],
        wire![&edge[1].inverse(), &edge[2], &edge[4].inverse()],
        wire![&edge[2].inverse(), &edge[0], &edge[5].inverse()],
        wire![&edge[3], &edge[4], &edge[5]],
        wire![&edge[6].inverse(), &edge[7], &edge[9].inverse()],
        wire![&edge[7].inverse(), &edge[8], &edge[10].inverse()],
        wire![&edge[8].inverse(), &edge[6], &edge[11].inverse()],
        wire![&edge[9], &edge[10], &edge[11]],
    ];
    let shell: Shell<_, _, _> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert_eq!(shell.shell_condition(), ShellCondition::Closed);
    assert!(shell.is_connected());
    assert_eq!(shell.singular_vertices(), vec![v[0].clone()]);
}

// ===========================================================================
// Shell iteration tests
// ===========================================================================

#[test]
fn test_shell_face_iter() {
    let shell = cube_shell();
    assert_eq!(shell.face_iter().count(), 6);
}

#[test]
fn test_shell_edge_iter() {
    let shell = cube_shell();
    // Each of the 6 faces has 4 edges, so 24 edge references total.
    assert_eq!(shell.edge_iter().count(), 24);
}

#[test]
fn test_shell_vertex_iter() {
    let shell = cube_shell();
    // Same count as edge_iter: one vertex (front) per edge.
    assert_eq!(shell.vertex_iter().count(), 24);
}

// ===========================================================================
// Solid tests
// ===========================================================================

#[test]
fn test_solid_new_valid() {
    let shell = tetrahedron_shell();
    let solid = Solid::try_new(vec![shell]);
    assert!(solid.is_ok());
}

#[test]
fn test_solid_try_new_empty_shell() {
    let result = Solid::try_new(vec![Shell::<(), (), ()>::new()]);
    assert_eq!(result, Err(Error::EmptyShell));
}

#[test]
fn test_solid_try_new_not_connected() {
    let v = Vertex::news([(); 4]);
    let wire = vec![
        Wire::from(vec![
            Edge::new(&v[0], &v[1], ()),
            Edge::new(&v[1], &v[0], ()),
        ]),
        Wire::from(vec![
            Edge::new(&v[2], &v[3], ()),
            Edge::new(&v[3], &v[2], ()),
        ]),
    ];
    let shell: Shell<(), (), ()> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert_eq!(Solid::try_new(vec![shell]), Err(Error::NotConnected));
}

#[test]
fn test_solid_try_new_not_closed() {
    let v = Vertex::news([(); 2]);
    let wire = Wire::from(vec![
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[0], ()),
    ]);
    let shell: Shell<(), (), ()> = vec![Face::new(vec![wire], ())].into();
    assert_eq!(Solid::try_new(vec![shell]), Err(Error::NotClosedShell));
}

#[test]
fn test_solid_try_new_not_manifold() {
    // Wedge of two spheres.
    let v = Vertex::news([(); 3]);
    let edge = [
        Edge::new(&v[0], &v[1], ()),
        Edge::new(&v[1], &v[0], ()),
        Edge::new(&v[1], &v[2], ()),
        Edge::new(&v[2], &v[1], ()),
    ];
    let wire = vec![
        wire![&edge[0], &edge[1]],
        wire![&edge[1].inverse(), &edge[0].inverse()],
        wire![&edge[2], &edge[3]],
        wire![&edge[3].inverse(), &edge[2].inverse()],
    ];
    let shell: Shell<(), (), ()> = wire.into_iter().map(|w| Face::new(vec![w], ())).collect();
    assert_eq!(Solid::try_new(vec![shell]), Err(Error::NotManifold));
}

#[test]
fn test_solid_boundaries() {
    let solid = Solid::new(vec![cube_shell()]);
    assert_eq!(solid.boundaries().len(), 1);
}

#[test]
fn test_solid_into_boundaries() {
    let solid = Solid::new(vec![cube_shell()]);
    let boundaries = solid.into_boundaries();
    assert_eq!(boundaries.len(), 1);
}

#[test]
fn test_solid_face_iter() {
    let solid = Solid::new(vec![cube_shell()]);
    assert_eq!(solid.face_iter().count(), 6);
}

#[test]
fn test_solid_edge_iter() {
    let solid = Solid::new(vec![cube_shell()]);
    // 6 faces * 4 edges = 24 edge references.
    assert_eq!(solid.edge_iter().count(), 24);
}

#[test]
fn test_solid_vertex_iter() {
    let solid = Solid::new(vec![cube_shell()]);
    assert_eq!(solid.vertex_iter().count(), 24);
}

#[test]
fn test_solid_not() {
    let mut solid = Solid::new(vec![cube_shell()]);
    // All faces start with orientation true.
    let all_true = solid.face_iter().all(|f| f.orientation());
    // Invert.
    solid.not();
    let all_flipped = solid.face_iter().all(|f| f.orientation() != all_true);
    assert!(all_flipped);
}

#[test]
fn test_solid_display_formats() {
    let solid = Solid::new(vec![cube_shell()]);
    let vertex_format = VertexDisplayFormat::AsPoint;
    let edge_format = EdgeDisplayFormat::VerticesTuple { vertex_format };
    let wire_format = WireDisplayFormat::EdgesList { edge_format };
    let face_format = FaceDisplayFormat::LoopsList { wire_format };
    let shell_format = ShellDisplayFormat::FacesList { face_format };

    let s = format!(
        "{:?}",
        solid.display(SolidDisplayFormat::Struct { shell_format })
    );
    assert!(!s.is_empty());
    let s = format!(
        "{:?}",
        solid.display(SolidDisplayFormat::ShellsListTuple { shell_format })
    );
    assert!(!s.is_empty());
    let s = format!(
        "{:?}",
        solid.display(SolidDisplayFormat::ShellsList { shell_format })
    );
    assert!(!s.is_empty());
}

// ===========================================================================
// Compress round-trip test
// ===========================================================================

#[test]
fn test_shell_compress_extract_roundtrip() {
    let shell0 = cube_shell();
    let compressed: CompressedShell<(), (), ()> = shell0.compress();
    let shell1 = Shell::extract(compressed).unwrap();
    assert_eq!(shell0.len(), shell1.len());
    // Verify structural equivalence by checking boundary iter lengths match.
    for (f0, f1) in shell0.face_iter().zip(shell1.face_iter()) {
        let b0 = f0.boundary_iters();
        let b1 = f1.boundary_iters();
        assert_eq!(b0.len(), b1.len());
        for (bi0, bi1) in b0.into_iter().zip(b1) {
            assert_eq!(bi0.len(), bi1.len());
        }
    }
}
