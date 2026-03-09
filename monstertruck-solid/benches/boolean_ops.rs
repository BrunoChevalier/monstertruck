use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monstertruck_modeling::*;

/// Creates a unit cube at the given origin offset.
fn make_cube(offset: Vector3) -> Solid {
    let v = builder::vertex(Point3::from_vec(offset));
    let e = builder::extrude(&v, Vector3::unit_x());
    let f = builder::extrude(&e, Vector3::unit_y());
    builder::extrude(&f, Vector3::unit_z())
}

/// Benchmarks boolean intersection of two overlapping cubes.
fn bench_boolean_and(c: &mut Criterion) {
    let cube0 = make_cube(Vector3::new(0.0, 0.0, 0.0));
    let cube1 = make_cube(Vector3::new(0.5, 0.5, 0.5));

    c.bench_function("boolean_and_cubes", |b| {
        b.iter(|| {
            black_box(
                monstertruck_solid::and(black_box(&cube0), black_box(&cube1), black_box(0.05)),
            )
        })
    });
}

/// Benchmarks boolean union of two overlapping cubes.
fn bench_boolean_or(c: &mut Criterion) {
    let cube0 = make_cube(Vector3::new(0.0, 0.0, 0.0));
    let cube1 = make_cube(Vector3::new(0.5, 0.5, 0.5));

    c.bench_function("boolean_or_cubes", |b| {
        b.iter(|| {
            black_box(
                monstertruck_solid::or(black_box(&cube0), black_box(&cube1), black_box(0.05)),
            )
        })
    });
}

criterion_group!(benches, bench_boolean_and, bench_boolean_or);
criterion_main!(benches);
