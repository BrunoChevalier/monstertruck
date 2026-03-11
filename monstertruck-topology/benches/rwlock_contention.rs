use criterion::{Criterion, black_box, criterion_group, criterion_main};
use monstertruck_topology::*;
use rayon::prelude::*;

/// Builds a topology with the given number of vertices and edges for benchmarking.
fn build_topology(n_verts: usize) -> (Vec<Vertex<[f64; 3]>>, Vec<Edge<[f64; 3], [f64; 6]>>) {
    let vertices: Vec<Vertex<[f64; 3]>> = (0..n_verts)
        .map(|i| {
            let t = i as f64 / n_verts as f64;
            Vertex::new([t.cos(), t.sin(), t * 0.5])
        })
        .collect();
    let edges: Vec<Edge<[f64; 3], [f64; 6]>> = (0..n_verts)
        .map(|i| {
            let j = (i + 1) % n_verts;
            let pi = vertices[i].point();
            let pj = vertices[j].point();
            Edge::new(
                &vertices[i],
                &vertices[j],
                [pi[0], pi[1], pi[2], pj[0], pj[1], pj[2]],
            )
        })
        .collect();
    (vertices, edges)
}

/// Benchmark concurrent reads on vertex points via rayon.
fn concurrent_read_points(c: &mut Criterion) {
    let (vertices, _edges) = build_topology(40);
    c.bench_function("concurrent_read_points", |b| {
        b.iter(|| {
            let sum: f64 = vertices
                .par_iter()
                .map(|v| {
                    let p = v.point();
                    black_box(p[0] + p[1] + p[2])
                })
                .sum();
            black_box(sum)
        })
    });
}

/// Benchmark concurrent reads on edge curves via rayon.
fn concurrent_read_curves(c: &mut Criterion) {
    let (_vertices, edges) = build_topology(40);
    c.bench_function("concurrent_read_curves", |b| {
        b.iter(|| {
            let sum: f64 = edges
                .par_iter()
                .map(|e| {
                    let curve = e.curve();
                    black_box(curve[0] + curve[1] + curve[2])
                })
                .sum();
            black_box(sum)
        })
    });
}

/// Benchmark mixed read-write: one writer updating a vertex while readers access it.
fn mixed_read_write(c: &mut Criterion) {
    let (vertices, _edges) = build_topology(40);
    c.bench_function("mixed_read_write", |b| {
        b.iter(|| {
            // Readers read all vertices concurrently, writer updates one.
            rayon::join(
                || {
                    vertices.par_iter().for_each(|v| {
                        black_box(v.point());
                    });
                },
                || {
                    vertices[0].set_point([99.0, 99.0, 99.0]);
                },
            );
        })
    });
}

criterion_group!(
    benches,
    concurrent_read_points,
    concurrent_read_curves,
    mixed_read_write,
);
criterion_main!(benches);
