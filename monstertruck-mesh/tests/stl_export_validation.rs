use monstertruck_mesh::*;
use stl::{StlFace, StlReader, StlType};
type Result<T> = std::result::Result<T, errors::Error>;

/// Creates a tetrahedron as 4 `StlFace` entries with outward-pointing normals.
fn tetrahedron_faces() -> Vec<StlFace> {
    // Vertices of a regular-ish tetrahedron.
    let v0 = [0.0_f32, 0.0, 0.0];
    let v1 = [1.0, 0.0, 0.0];
    let v2 = [0.5, 1.0, 0.0];
    let v3 = [0.5, 0.3, 0.8];

    // Helper: compute outward normal for a triangle face.
    let normal = |a: [f32; 3], b: [f32; 3], c: [f32; 3]| -> [f32; 3] {
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let n = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        let mag = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        [n[0] / mag, n[1] / mag, n[2] / mag]
    };

    vec![
        StlFace {
            normal: normal(v0, v2, v1),
            vertices: [v0, v2, v1],
        },
        StlFace {
            normal: normal(v0, v1, v3),
            vertices: [v0, v1, v3],
        },
        StlFace {
            normal: normal(v1, v2, v3),
            vertices: [v1, v2, v3],
        },
        StlFace {
            normal: normal(v0, v3, v2),
            vertices: [v0, v3, v2],
        },
    ]
}

#[test]
fn stl_binary_header_valid() {
    let faces = tetrahedron_faces();
    let face_count = faces.len();

    let mut binary = Vec::new();
    stl::write(faces.iter().cloned(), &mut binary, StlType::Binary).unwrap();

    // Binary STL format: 80 byte header + 4 byte face count + 50 bytes per face.
    let expected_len = 84 + 50 * face_count;
    assert_eq!(
        binary.len(),
        expected_len,
        "Binary STL length should be 84 + 50 * {face_count} = {expected_len}, got {}",
        binary.len()
    );

    // Read face count from bytes 80..84 as little-endian u32.
    let stored_count = u32::from_le_bytes([binary[80], binary[81], binary[82], binary[83]]);
    assert_eq!(
        stored_count, face_count as u32,
        "Stored face count should match input face count"
    );
}

#[test]
fn stl_ascii_format_valid() {
    let faces = tetrahedron_faces();
    let face_count = faces.len();

    let mut ascii = Vec::new();
    stl::write(faces.iter().cloned(), &mut ascii, StlType::Ascii).unwrap();
    let text = String::from_utf8(ascii).unwrap();

    // Verify start and end markers.
    assert!(
        text.starts_with("solid"),
        "ASCII STL should start with 'solid'"
    );
    assert!(
        text.trim_end().ends_with("endsolid"),
        "ASCII STL should end with 'endsolid'"
    );

    // Count "facet normal" occurrences.
    let facet_count = text.matches("facet normal").count();
    assert_eq!(
        facet_count, face_count,
        "facet normal count should match face count"
    );

    // Count vertex lines: lines containing "vertex" but not "facet".
    let vertex_line_count = text
        .lines()
        .filter(|l| {
            let trimmed = l.trim();
            trimmed.starts_with("vertex") && !trimmed.contains("facet")
        })
        .count();
    assert_eq!(
        vertex_line_count,
        3 * face_count,
        "vertex line count should be 3 * face_count"
    );
}

#[test]
fn stl_normal_orientation_consistent() {
    let faces = tetrahedron_faces();

    let mut binary = Vec::new();
    stl::write(faces.iter().cloned(), &mut binary, StlType::Binary).unwrap();
    let read_faces: Vec<StlFace> = StlReader::<&[u8]>::new(&binary, StlType::Automatic)
        .unwrap()
        .collect::<Result<Vec<_>>>()
        .unwrap();

    for face in &read_faces {
        let v0 = face.vertices[0];
        let v1 = face.vertices[1];
        let v2 = face.vertices[2];
        // Compute geometric normal via cross product (v1-v0) x (v2-v0).
        let u = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let v = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
        let geo_normal = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        // Dot product of stored normal and geometric normal should be positive.
        let dot = face.normal[0] * geo_normal[0]
            + face.normal[1] * geo_normal[1]
            + face.normal[2] * geo_normal[2];
        assert!(
            dot > 0.0,
            "Stored normal should point in same direction as geometric normal, dot = {dot}"
        );
    }
}

#[test]
fn stl_binary_ascii_roundtrip_equivalent() {
    let faces = tetrahedron_faces();

    let mut ascii = Vec::new();
    stl::write(faces.iter().cloned(), &mut ascii, StlType::Ascii).unwrap();
    let mut binary = Vec::new();
    stl::write(faces.iter().cloned(), &mut binary, StlType::Binary).unwrap();

    let ascii_faces: Vec<StlFace> = StlReader::<&[u8]>::new(&ascii, StlType::Automatic)
        .unwrap()
        .collect::<Result<Vec<_>>>()
        .unwrap();
    let binary_faces: Vec<StlFace> = StlReader::<&[u8]>::new(&binary, StlType::Automatic)
        .unwrap()
        .collect::<Result<Vec<_>>>()
        .unwrap();

    assert_eq!(
        ascii_faces.len(),
        binary_faces.len(),
        "ASCII and binary face counts should match"
    );

    let tol = 1.0e-5;
    for (a, b) in ascii_faces.iter().zip(binary_faces.iter()) {
        for i in 0..3 {
            assert!(
                (a.normal[i] - b.normal[i]).abs() < tol,
                "Normal component {i} mismatch: {} vs {}",
                a.normal[i],
                b.normal[i]
            );
            for j in 0..3 {
                assert!(
                    (a.vertices[i][j] - b.vertices[i][j]).abs() < tol,
                    "Vertex [{i}][{j}] mismatch: {} vs {}",
                    a.vertices[i][j],
                    b.vertices[i][j]
                );
            }
        }
    }
}
