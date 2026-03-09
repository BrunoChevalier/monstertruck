use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monstertruck_meshing::prelude::*;
use monstertruck_modeling::*;

/// Benchmarks tessellation of a unit cube built via extrusion.
fn bench_cube_triangulation(c: &mut Criterion) {
    let v = builder::vertex(Point3::origin());
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    let cube: Solid = builder::extrude(&f, Vector3::unit_z());

    c.bench_function("cube_triangulation", |b| {
        b.iter(|| black_box(cube.triangulation(black_box(0.01))))
    });
}

/// Benchmarks tessellation of a translated cube (exercising non-axis-aligned geometry).
fn bench_translated_cube_triangulation(c: &mut Criterion) {
    let v = builder::vertex(Point3::new(0.5, 0.5, 0.5));
    let e = builder::extrude(&v, Vector3::new(1.0, 0.2, 0.0));
    let f = builder::extrude(&e, Vector3::new(0.0, 1.0, 0.3));
    let solid: Solid = builder::extrude(&f, Vector3::new(-0.1, 0.0, 1.0));

    c.bench_function("translated_cube_triangulation", |b| {
        b.iter(|| black_box(solid.triangulation(black_box(0.01))))
    });
}

criterion_group!(benches, bench_cube_triangulation, bench_translated_cube_triangulation);
criterion_main!(benches);
