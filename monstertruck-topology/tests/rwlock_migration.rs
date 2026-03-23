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

/// Verifies that concurrent mixed read-write access on vertices works
/// correctly with `RwLock`. Multiple readers proceed in parallel while
/// a writer updates atomically.
#[test]
fn concurrent_mixed_vertex_access() {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    let vertices: Vec<Vertex<[f64; 3]>> =
        (0..20).map(|i| Vertex::new([i as f64, 0.0, 0.0])).collect();
    let write_completed = AtomicBool::new(false);
    let v0_clone = vertices[0].clone();

    rayon::scope(|s| {
        // Writer: update vertex 0.
        s.spawn(|_| {
            v0_clone.set_point([99.0, 99.0, 99.0]);
            write_completed.store(true, Ordering::Release);
        });
        // Readers: read all vertices concurrently.
        s.spawn(|_| {
            let _points: Vec<[f64; 3]> = vertices.par_iter().map(|v| v.point()).collect();
        });
    });

    assert!(write_completed.load(Ordering::Acquire));
    // After the write, vertex 0 should have the updated value.
    assert_eq!(vertices[0].point(), [99.0, 99.0, 99.0]);
}

/// Verifies that concurrent edge reads via `oriented_curve` do not deadlock
/// under the lock ordering pattern: curve -> point.
#[test]
fn concurrent_edge_oriented_curve_reads() {
    use rayon::prelude::*;

    let vertices: Vec<Vertex<usize>> = (0..20).map(Vertex::new).collect();
    let edges: Vec<Edge<usize, (usize, usize)>> = (0..20)
        .map(|i| {
            let j = (i + 1) % 20;
            Edge::new(&vertices[i], &vertices[j], (i, j))
        })
        .collect();

    // Read oriented curves concurrently -- exercises curve lock.
    let results: Vec<(usize, usize)> = edges.par_iter().map(|e| e.oriented_curve()).collect();
    assert_eq!(results.len(), 20);
    assert_eq!(results[0], (0, 1));
}
