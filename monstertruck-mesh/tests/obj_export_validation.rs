use monstertruck_mesh::*;

mod cube {
    use super::*;
    pub const POSITIONS: [Point3; 8] = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
    ];

    pub const UV_COORDS: [Vector2; 4] = [
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
    ];

    pub const NORMALS: [Vector3; 6] = [
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(-1.0, 0.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
    ];
}

/// Creates a cube mesh with positions only (no normals/UVs).
fn make_positions_only_cube() -> PolygonMesh {
    let faces = Faces::from_iter(&[
        [(0, None, None), (1, None, None), (2, None, None)].as_ref(),
        &[(4, None, None), (2, None, None), (1, None, None)],
        &[(1, None, None), (0, None, None), (3, None, None)],
        &[(1, None, None), (3, None, None), (5, None, None)],
        &[
            (1, None, None),
            (5, None, None),
            (4, None, None),
            (7, None, None),
        ],
        &[(2, None, None), (4, None, None), (7, None, None)],
        &[(2, None, None), (7, None, None), (6, None, None)],
        &[
            (0, None, None),
            (2, None, None),
            (6, None, None),
            (3, None, None),
        ],
        &[(3, None, None), (6, None, None), (7, None, None)],
        &[(3, None, None), (7, None, None), (5, None, None)],
    ]);
    PolygonMesh::new(
        StandardAttributes {
            positions: cube::POSITIONS.to_vec(),
            ..Default::default()
        },
        faces,
    )
}

#[test]
fn obj_export_vertex_count_matches() {
    let mesh = make_positions_only_cube();
    let vertex_count = mesh.positions().len();
    let face_count = mesh.faces().len();

    let mut gened_obj: Vec<u8> = Vec::new();
    obj::write(&mesh, &mut gened_obj).unwrap();
    let text = String::from_utf8(gened_obj).unwrap();

    let v_lines = text.lines().filter(|l| l.starts_with("v ")).count();
    assert_eq!(
        v_lines, vertex_count,
        "OBJ vertex line count should match source position count"
    );

    let f_lines = text.lines().filter(|l| l.starts_with("f ")).count();
    assert_eq!(
        f_lines, face_count,
        "OBJ face line count should match source face count"
    );
}

#[test]
fn obj_export_vertex_format_valid() {
    let mesh = make_positions_only_cube();
    let vertex_count = mesh.positions().len();

    let mut gened_obj: Vec<u8> = Vec::new();
    obj::write(&mesh, &mut gened_obj).unwrap();
    let text = String::from_utf8(gened_obj).unwrap();

    // Each "v " line should have exactly 3 floats.
    for line in text.lines().filter(|l| l.starts_with("v ")) {
        let coords: Vec<f64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<f64>().expect("vertex coordinate should be a float"))
            .collect();
        assert_eq!(
            coords.len(),
            3,
            "Each vertex line should have exactly 3 coordinates, got: {line}"
        );
    }

    // Each "f " line indices should be in [1, vertex_count].
    for line in text.lines().filter(|l| l.starts_with("f ")) {
        for token in line.split_whitespace().skip(1) {
            // Index is the first component before any '/' separator.
            let idx_str = token.split('/').next().unwrap();
            let idx: usize = idx_str
                .parse()
                .unwrap_or_else(|_| panic!("face index should be a number: {token}"));
            assert!(
                idx >= 1 && idx <= vertex_count,
                "face index {idx} out of range [1, {vertex_count}]"
            );
        }
    }
}

#[test]
fn obj_export_with_normals_format() {
    let faces = Faces::from_iter(&[
        [(0, None, Some(5)), (1, None, Some(5)), (2, None, Some(5))].as_ref(),
        &[(4, None, Some(5)), (2, None, Some(5)), (1, None, Some(5))],
        &[(1, None, Some(4)), (0, None, Some(4)), (3, None, Some(4))],
        &[(1, None, Some(4)), (3, None, Some(4)), (5, None, Some(4))],
        &[
            (1, None, Some(0)),
            (5, None, Some(0)),
            (4, None, Some(0)),
            (7, None, Some(0)),
        ],
        &[(2, None, Some(1)), (4, None, Some(1)), (7, None, Some(1))],
        &[(2, None, Some(1)), (7, None, Some(1)), (6, None, Some(1))],
        &[
            (0, None, Some(3)),
            (2, None, Some(3)),
            (6, None, Some(3)),
            (3, None, Some(3)),
        ],
        &[(3, None, Some(2)), (6, None, Some(2)), (7, None, Some(2))],
        &[(3, None, Some(2)), (7, None, Some(2)), (5, None, Some(2))],
    ]);
    let mesh = PolygonMesh::new(
        StandardAttributes {
            positions: cube::POSITIONS.to_vec(),
            normals: cube::NORMALS.to_vec(),
            ..Default::default()
        },
        faces,
    );
    let vertex_count = mesh.positions().len();
    let normal_count = mesh.normals().len();

    let mut gened_obj: Vec<u8> = Vec::new();
    obj::write(&mesh, &mut gened_obj).unwrap();
    let text = String::from_utf8(gened_obj).unwrap();

    // Verify "vn " lines exist and each has exactly 3 floats.
    let vn_lines: Vec<&str> = text.lines().filter(|l| l.starts_with("vn ")).collect();
    assert_eq!(
        vn_lines.len(),
        normal_count,
        "OBJ normal line count should match source normal count"
    );
    for line in &vn_lines {
        let coords: Vec<f64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<f64>().expect("normal coordinate should be a float"))
            .collect();
        assert_eq!(
            coords.len(),
            3,
            "Each normal line should have exactly 3 coordinates, got: {line}"
        );
    }

    // Verify "f " lines use the `v//vn` format with valid indices.
    for line in text.lines().filter(|l| l.starts_with("f ")) {
        for token in line.split_whitespace().skip(1) {
            assert!(
                token.contains("//"),
                "face token should use v//vn format, got: {token}"
            );
            let parts: Vec<&str> = token.split("//").collect();
            let pos_idx: usize = parts[0].parse().unwrap();
            let nor_idx: usize = parts[1].parse().unwrap();
            assert!(
                pos_idx >= 1 && pos_idx <= vertex_count,
                "position index {pos_idx} out of range"
            );
            assert!(
                nor_idx >= 1 && nor_idx <= normal_count,
                "normal index {nor_idx} out of range"
            );
        }
    }
}

#[test]
fn obj_export_with_texcoords_format() {
    let faces = Faces::from_iter(&[
        [(0, Some(0), None), (1, Some(1), None), (2, Some(2), None)].as_ref(),
        &[(4, Some(3), None), (2, Some(2), None), (1, Some(1), None)],
        &[(1, Some(0), None), (0, Some(1), None), (3, Some(2), None)],
        &[(1, Some(3), None), (3, Some(2), None), (5, Some(1), None)],
        &[
            (1, Some(0), None),
            (5, Some(1), None),
            (4, Some(2), None),
            (7, Some(1), None),
        ],
        &[(2, Some(0), None), (4, Some(1), None), (7, Some(2), None)],
        &[(2, Some(3), None), (7, Some(2), None), (6, Some(1), None)],
        &[
            (0, Some(0), None),
            (2, Some(1), None),
            (6, Some(2), None),
            (3, Some(1), None),
        ],
        &[(3, Some(0), None), (6, Some(1), None), (7, Some(2), None)],
        &[(3, Some(3), None), (7, Some(2), None), (5, Some(1), None)],
    ]);
    let mesh = PolygonMesh::new(
        StandardAttributes {
            positions: cube::POSITIONS.to_vec(),
            uv_coords: cube::UV_COORDS.to_vec(),
            ..Default::default()
        },
        faces,
    );

    let mut gened_obj: Vec<u8> = Vec::new();
    obj::write(&mesh, &mut gened_obj).unwrap();
    let text = String::from_utf8(gened_obj).unwrap();

    // Verify "vt " lines exist and each has exactly 2 floats.
    let vt_lines: Vec<&str> = text.lines().filter(|l| l.starts_with("vt ")).collect();
    assert_eq!(
        vt_lines.len(),
        cube::UV_COORDS.len(),
        "OBJ texcoord line count should match source UV count"
    );
    for line in &vt_lines {
        let coords: Vec<f64> = line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<f64>().expect("texcoord should be a float"))
            .collect();
        assert_eq!(
            coords.len(),
            2,
            "Each texcoord line should have exactly 2 coordinates, got: {line}"
        );
    }

    // Verify "f " lines use the `v/vt` format.
    for line in text.lines().filter(|l| l.starts_with("f ")) {
        for token in line.split_whitespace().skip(1) {
            assert!(
                token.contains('/') && !token.contains("//"),
                "face token should use v/vt format, got: {token}"
            );
        }
    }
}

#[test]
fn obj_export_roundtrip_vertex_count_preserved() {
    let mesh = make_positions_only_cube();
    let original_count = mesh.positions().len();

    let mut gened_obj: Vec<u8> = Vec::new();
    obj::write(&mesh, &mut gened_obj).unwrap();
    let read_mesh = obj::read(AsRef::<[u8]>::as_ref(&gened_obj)).unwrap();

    assert_eq!(
        read_mesh.positions().len(),
        original_count,
        "Vertex count should be preserved through OBJ round-trip"
    );
}
