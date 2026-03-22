#![cfg(feature = "fillet")]

use monstertruck_modeling::*;

/// Build a closed box shell, fillet one edge, verify the shell gains faces.
#[test]
fn fillet_box_edge() {
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
    let v: Vec<Vertex> = Vertex::news(p);

    let line_edge =
        |i: usize, j: usize| -> Edge { Edge::new(&v[i], &v[j], Curve::Line(Line(p[i], p[j]))) };

    let edge = [
        line_edge(0, 1),
        line_edge(1, 2),
        line_edge(2, 3),
        line_edge(3, 0),
        line_edge(0, 4),
        line_edge(1, 5),
        line_edge(2, 6),
        line_edge(3, 7),
        line_edge(4, 5),
        line_edge(5, 6),
        line_edge(6, 7),
        line_edge(7, 4),
    ];

    let plane_face = |i: usize, j: usize, k: usize, l: usize| -> Face {
        let plane = Plane::new(p[i], p[j], p[l]);
        let wire: Wire = [i, j, k, l]
            .into_iter()
            .zip([i, j, k, l].iter().cycle().skip(1))
            .map(|(a, &b)| {
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
                    .unwrap()
            })
            .collect();
        Face::new(vec![wire], Surface::Plane(plane))
    };

    // Build a closed box shell (all 6 faces).
    let mut shell: Shell = [
        plane_face(0, 1, 2, 3), // top
        plane_face(5, 4, 7, 6), // bottom (reversed winding)
        plane_face(1, 0, 4, 5), // front
        plane_face(2, 1, 5, 6), // right
        plane_face(3, 2, 6, 7), // back
        plane_face(0, 3, 7, 4), // left
    ]
    .into();

    let initial_face_count = shell.len();

    let params = FilletOptions {
        radius: RadiusSpec::Constant(0.4),
        ..Default::default()
    };
    fillet_edges(&mut shell, &[edge[5].clone()], Some(&params)).unwrap();

    assert!(
        shell.len() > initial_face_count,
        "expected fillet to add faces: got {} (was {})",
        shell.len(),
        initial_face_count
    );
}
