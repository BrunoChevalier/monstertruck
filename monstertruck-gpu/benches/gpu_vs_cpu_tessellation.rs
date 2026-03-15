//! Benchmark comparing GPU and CPU NURBS tessellation.
//!
//! Uses [`criterion`] to measure performance at various grid sizes.
//! Skips GPU benchmarks if no adapter is available.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use monstertruck_gpu::compute::{GpuTessellator, NurbsSurfaceData};
use monstertruck_gpu::DeviceHandler;

// ============================================================================
// Inline CPU evaluator (matches the WGSL shader logic).
// ============================================================================

fn cpu_find_span(knots: &[f32], n: u32, degree: u32, t: f32) -> u32 {
    if t >= knots[(n + 1) as usize] {
        return n;
    }
    if t <= knots[degree as usize] {
        return degree;
    }
    let mut lo = degree;
    let mut hi = n + 1;
    for _ in 0..64 {
        if hi - lo <= 1 {
            break;
        }
        let mid = (lo + hi) / 2;
        if t < knots[mid as usize] {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    lo
}

fn cpu_basis_funs(knots: &[f32], span: u32, degree: u32, t: f32) -> Vec<f32> {
    let mut n_basis = vec![0.0f32; (degree + 1) as usize];
    let mut left = vec![0.0f32; (degree + 1) as usize];
    let mut right = vec![0.0f32; (degree + 1) as usize];
    n_basis[0] = 1.0;
    for j in 1..=degree as usize {
        left[j] = t - knots[(span + 1 - j as u32) as usize];
        right[j] = knots[(span + j as u32) as usize] - t;
        let mut saved = 0.0f32;
        for r in 0..j {
            let temp = n_basis[r] / (right[r + 1] + left[j - r]);
            n_basis[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        n_basis[j] = saved;
    }
    n_basis
}

fn cpu_evaluate_nurbs_surface(data: &NurbsSurfaceData, u: f32, v: f32) -> [f32; 3] {
    let n_u = data.num_cp_u - 1;
    let n_v = data.num_cp_v - 1;
    let span_u = cpu_find_span(&data.knots_u, n_u, data.degree_u, u);
    let span_v = cpu_find_span(&data.knots_v, n_v, data.degree_v, v);
    let basis_u = cpu_basis_funs(&data.knots_u, span_u, data.degree_u, u);
    let basis_v = cpu_basis_funs(&data.knots_v, span_v, data.degree_v, v);

    let mut point = [0.0f32; 4];
    for l in 0..=data.degree_v {
        let mut temp = [0.0f32; 4];
        let row = span_v - data.degree_v + l;
        for k in 0..=data.degree_u {
            let col = span_u - data.degree_u + k;
            let idx = (row * data.num_cp_u + col) as usize;
            let cp = data.control_points[idx];
            for c in 0..4 {
                temp[c] += basis_u[k as usize] * cp[c];
            }
        }
        for c in 0..4 {
            point[c] += basis_v[l as usize] * temp[c];
        }
    }
    let w = point[3];
    if w.abs() > 1e-10 {
        [point[0] / w, point[1] / w, point[2] / w]
    } else {
        [point[0], point[1], point[2]]
    }
}

fn cpu_tessellate_grid(data: &NurbsSurfaceData, grid: u32) -> Vec<[f32; 3]> {
    let mut vertices = Vec::with_capacity((grid * grid) as usize);
    for iv in 0..grid {
        let v = if grid > 1 {
            iv as f32 / (grid - 1) as f32
        } else {
            0.0
        };
        for iu in 0..grid {
            let u = if grid > 1 {
                iu as f32 / (grid - 1) as f32
            } else {
                0.0
            };
            vertices.push(cpu_evaluate_nurbs_surface(data, u, v));
        }
    }
    vertices
}

// ============================================================================
// Test data: NURBS torus (degree 2x2 rational).
// ============================================================================

/// Approximate torus-like surface using a degree-2 rational NURBS patch.
fn torus_data() -> NurbsSurfaceData {
    let w = std::f32::consts::FRAC_1_SQRT_2;
    // 5x5 control points for a simple curved surface (not a full torus, but
    // enough to benchmark the compute kernel).
    let mut cps = Vec::new();
    for j in 0..5 {
        let v_angle = std::f32::consts::FRAC_PI_2 * j as f32 / 4.0;
        let r = 2.0 + 0.5 * v_angle.cos();
        let z = 0.5 * v_angle.sin();
        let wt = if j % 2 == 1 { w } else { 1.0 };
        for i in 0..5 {
            let u_angle = std::f32::consts::FRAC_PI_2 * i as f32 / 4.0;
            let x = r * u_angle.cos() * wt;
            let y = r * u_angle.sin() * wt;
            cps.push([x, y, z * wt, wt]);
        }
    }
    NurbsSurfaceData {
        control_points: cps,
        knots_u: vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        knots_v: vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        degree_u: 2,
        degree_v: 2,
        num_cp_u: 5,
        num_cp_v: 5,
    }
}

fn try_init_device() -> Option<DeviceHandler> {
    pollster::block_on(async {
        let instance = monstertruck_gpu::wgpu::Instance::new(
            &monstertruck_gpu::wgpu::InstanceDescriptor::default(),
        );
        let adapter = instance
            .request_adapter(&monstertruck_gpu::wgpu::RequestAdapterOptions {
                power_preference: monstertruck_gpu::wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok()?;
        let (device, queue) = adapter
            .request_device(&monstertruck_gpu::wgpu::DeviceDescriptor::default())
            .await
            .ok()?;
        Some(DeviceHandler::new(adapter, device, queue))
    })
}

fn bench_gpu_vs_cpu(c: &mut Criterion) {
    let data = torus_data();
    let handler = try_init_device();
    let gpu_tess = handler.as_ref().map(GpuTessellator::new);

    let mut group = c.benchmark_group("nurbs_tessellation");

    for grid_size in [16u32, 32, 64] {
        group.bench_with_input(
            BenchmarkId::new("cpu", grid_size),
            &grid_size,
            |b, &grid| {
                b.iter(|| cpu_tessellate_grid(&data, grid));
            },
        );

        if let Some(ref tess) = gpu_tess {
            group.bench_with_input(
                BenchmarkId::new("gpu", grid_size),
                &grid_size,
                |b, &grid| {
                    b.iter(|| tess.tessellate(&data, grid, grid, 1e-4).unwrap());
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, bench_gpu_vs_cpu);
criterion_main!(benches);
