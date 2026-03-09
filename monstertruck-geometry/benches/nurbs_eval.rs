use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monstertruck_geometry::prelude::*;

/// Creates a degree-3 B-spline curve with `n` control points arranged in a helix.
fn make_test_curve(n: usize) -> BsplineCurve<Point3> {
    let degree = 3usize;
    let points: Vec<Point3> = (0..n)
        .map(|i| {
            let t = i as f64 / (n - 1) as f64;
            Point3::new(t.cos(), t.sin(), t * 0.5)
        })
        .collect();
    let knots: KnotVector = std::iter::repeat_n(0.0, degree + 1)
        .chain((1..n - degree).map(|i| i as f64 / (n - degree) as f64))
        .chain(std::iter::repeat_n(1.0, degree + 1))
        .collect();
    BsplineCurve::new(knots, points)
}

/// Creates a degree-3 x degree-3 B-spline surface with an `n x n` grid of control points.
fn make_test_surface(n: usize) -> BsplineSurface<Point3> {
    let degree = 3usize;
    let knots: KnotVector = std::iter::repeat_n(0.0, degree + 1)
        .chain((1..n - degree).map(|i| i as f64 / (n - degree) as f64))
        .chain(std::iter::repeat_n(1.0, degree + 1))
        .collect();
    let control_points: Vec<Vec<Point3>> = (0..n)
        .map(|i| {
            let u = i as f64 / (n - 1) as f64;
            (0..n)
                .map(|j| {
                    let v = j as f64 / (n - 1) as f64;
                    Point3::new(u, v, (u * std::f64::consts::PI).sin() * (v * std::f64::consts::PI).sin() * 0.3)
                })
                .collect()
        })
        .collect();
    BsplineSurface::new((knots.clone(), knots), control_points)
}

fn bench_bspline_curve_eval(c: &mut Criterion) {
    let curve = make_test_curve(20);
    c.bench_function("bspline_curve_evaluate", |b| {
        b.iter(|| black_box(curve.evaluate(black_box(0.5))))
    });
}

fn bench_bspline_curve_derivative(c: &mut Criterion) {
    let curve = make_test_curve(20);
    c.bench_function("bspline_curve_derivative", |b| {
        b.iter(|| black_box(curve.derivative(black_box(0.5))))
    });
}

fn bench_bspline_surface_eval(c: &mut Criterion) {
    let surface = make_test_surface(8);
    c.bench_function("bspline_surface_evaluate", |b| {
        b.iter(|| black_box(surface.evaluate(black_box(0.5), black_box(0.5))))
    });
}

criterion_group!(
    benches,
    bench_bspline_curve_eval,
    bench_bspline_curve_derivative,
    bench_bspline_surface_eval
);
criterion_main!(benches);
