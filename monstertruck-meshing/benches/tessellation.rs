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

/// Benchmarks tessellation of a cylinder (revolve-based surface).
fn bench_cylinder_triangulation(c: &mut Criterion) {
    let v = builder::vertex(Point3::new(0.5, 0.0, 0.0));
    let edge = builder::extrude(&v, Vector3::unit_z());
    let face: Face = builder::revolve(
        &edge,
        Point3::origin(),
        Vector3::unit_z(),
        Rad(std::f64::consts::TAU),
    );
    let shell: Shell = shell![face];
    let solid: Solid = Solid::new(vec![shell]);

    c.bench_function("cylinder_triangulation", |b| {
        b.iter(|| black_box(solid.triangulation(black_box(0.05))))
    });
}

criterion_group!(benches, bench_cube_triangulation, bench_cylinder_triangulation);
criterion_main!(benches);
