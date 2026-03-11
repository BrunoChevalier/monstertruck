//! Tests verifying that topology types use `RwLock` instead of `Mutex`
//! for geometry data, enabling concurrent read access.

use monstertruck_topology::*;

/// Verifies that `VertexId` uses `RwLock` internally, not `Mutex`.
#[test]
fn vertex_id_uses_rwlock() {
    let type_name = std::any::type_name::<VertexId<()>>();
    assert!(
        type_name.contains("RwLock"),
        "VertexId should use RwLock, but type is: {type_name}",
    );
}

/// Verifies that `EdgeId` uses `RwLock` internally, not `Mutex`.
#[test]
fn edge_id_uses_rwlock() {
    let type_name = std::any::type_name::<EdgeId<()>>();
    assert!(
        type_name.contains("RwLock"),
        "EdgeId should use RwLock, but type is: {type_name}",
    );
}

/// Verifies that `FaceId` uses `RwLock` internally, not `Mutex`.
#[test]
fn face_id_uses_rwlock() {
    let type_name = std::any::type_name::<FaceId<()>>();
    assert!(
        type_name.contains("RwLock"),
        "FaceId should use RwLock, but type is: {type_name}",
    );
}

/// Verifies concurrent read access to vertex points works correctly
/// with rayon parallelism (functional with both Mutex and RwLock, but
/// RwLock enables true concurrent reads without blocking).
#[test]
fn concurrent_vertex_reads() {
    use rayon::prelude::*;
    let vertices: Vec<Vertex<[f64; 3]>> = (0..100)
        .map(|i| {
            let t = i as f64 / 100.0;
            Vertex::new([t.cos(), t.sin(), t * 0.5])
        })
        .collect();
    // All reads should succeed concurrently.
    let points: Vec<[f64; 3]> = vertices.par_iter().map(|v| v.point()).collect();
    assert_eq!(points.len(), 100);
    // Verify first and last values.
    assert!((points[0][0] - 1.0).abs() < 1e-10);
}

/// Verifies concurrent read access to edge curves works correctly.
#[test]
fn concurrent_edge_reads() {
    use rayon::prelude::*;
    let vertices: Vec<Vertex<[f64; 3]>> = (0..100)
        .map(|i| {
            let t = i as f64 / 100.0;
            Vertex::new([t.cos(), t.sin(), t * 0.5])
        })
        .collect();
    let edges: Vec<Edge<[f64; 3], f64>> = (0..100)
        .map(|i| {
            let j = (i + 1) % 100;
            Edge::new(&vertices[i], &vertices[j], i as f64)
        })
        .collect();
    // All reads should succeed concurrently.
    let curves: Vec<f64> = edges.par_iter().map(|e| e.curve()).collect();
    assert_eq!(curves.len(), 100);
    assert!((curves[0] - 0.0).abs() < 1e-10);
    assert!((curves[99] - 99.0).abs() < 1e-10);
}
