//! Integration tests for GPU NURBS tessellation.
//!
//! These tests compare GPU compute output against a CPU reference evaluator.
//! Tests are skipped gracefully if no GPU adapter is available.

use monstertruck_gpu::compute::{
    AdaptiveResult, GpuTessellator, GpuTessellatorError, NurbsSurfaceData, TessellationResult,
    MAX_DEGREE,
};
use monstertruck_gpu::DeviceHandler;

/// Attempt to create a [`DeviceHandler`]. Returns `None` if no GPU adapter is
/// available (e.g. CI without a GPU).
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

// ============================================================================
// Inline CPU NURBS evaluator (mirrors the WGSL shader logic).
// ============================================================================

fn cpu_find_span(knots: &[f32], n: u32, degree: u32, t: f32) -> u32 {
    let lo_init = degree;
    let hi_init = n + 1;
    if t >= knots[hi_init as usize] {
        return n;
    }
    if t <= knots[lo_init as usize] {
        return degree;
    }
    let mut lo = lo_init;
    let mut hi = hi_init;
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

// ============================================================================
// Test helpers
// ============================================================================

/// Create a flat bilinear NURBS patch (degree 1x1, 4 control points).
/// The patch spans from (0,0,0) to (1,1,0) -- a unit square in the XY plane.
fn flat_plane_data() -> NurbsSurfaceData {
    NurbsSurfaceData {
        control_points: vec![
            [0.0, 0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [1.0, 1.0, 0.0, 1.0],
        ],
        knots_u: vec![0.0, 0.0, 1.0, 1.0],
        knots_v: vec![0.0, 0.0, 1.0, 1.0],
        degree_u: 1,
        degree_v: 1,
        num_cp_u: 2,
        num_cp_v: 2,
    }
}

/// Create a NURBS sphere octant (degree 2x2, rational).
fn sphere_octant_data() -> NurbsSurfaceData {
    let w = std::f32::consts::FRAC_1_SQRT_2;
    // 3x3 control points for a rational quadratic sphere octant.
    NurbsSurfaceData {
        control_points: vec![
            // row 0
            [0.0, 0.0, 1.0, 1.0],
            [w * 1.0, 0.0, w * 1.0, w],
            [1.0, 0.0, 0.0, 1.0],
            // row 1
            [0.0, w * 1.0, w * 1.0, w],
            [0.5, 0.5, 0.5, 0.5],
            [w * 1.0, w * 1.0, 0.0, w],
            // row 2
            [0.0, 1.0, 0.0, 1.0],
            [w * 1.0, w * 1.0, 0.0, w],
            [1.0, 0.0, 0.0, 1.0],
        ],
        knots_u: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        knots_v: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        degree_u: 2,
        degree_v: 2,
        num_cp_u: 3,
        num_cp_v: 3,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[test]
fn test_flat_plane_tessellation() {
    let handler = match try_init_device() {
        Some(h) => h,
        None => {
            eprintln!("Skipping test_flat_plane_tessellation: no GPU adapter available.");
            return;
        }
    };

    let tess = GpuTessellator::new(&handler);
    let data = flat_plane_data();
    let result: TessellationResult = tess.tessellate(&data, 8, 8, 1e-4).unwrap();

    assert_eq!(result.grid_u, 8);
    assert_eq!(result.grid_v, 8);
    assert_eq!(result.vertices.len(), 64);
    assert_eq!(result.normals.len(), 64);

    // All vertices should lie on the z=0 plane.
    for (i, vert) in result.vertices.iter().enumerate() {
        assert!(
            vert[2].abs() < 1e-3,
            "Vertex {i} z-component {z} is not on the z=0 plane.",
            z = vert[2]
        );
    }

    // All normals should point in the +Z or -Z direction.
    for (i, norm) in result.normals.iter().enumerate() {
        let nz = norm[2].abs();
        assert!(
            nz > 0.9,
            "Normal {i} z-component {nz} is not close to 1.0."
        );
    }
}

#[test]
fn test_sphere_tessellation_matches_cpu() {
    let handler = match try_init_device() {
        Some(h) => h,
        None => {
            eprintln!(
                "Skipping test_sphere_tessellation_matches_cpu: no GPU adapter available."
            );
            return;
        }
    };

    let tess = GpuTessellator::new(&handler);
    let data = sphere_octant_data();
    let grid = 16u32;
    let result = tess.tessellate(&data, grid, grid, 1e-4).unwrap();

    assert_eq!(result.vertices.len() as u32, grid * grid);

    let tolerance = 1e-4;
    for iv in 0..grid {
        for iu in 0..grid {
            let u = iu as f32 / (grid - 1) as f32;
            let v = iv as f32 / (grid - 1) as f32;
            let cpu_pt = cpu_evaluate_nurbs_surface(&data, u, v);
            let gpu_pt = result.vertices[(iv * grid + iu) as usize];

            for c in 0..3 {
                let diff = (cpu_pt[c] - gpu_pt[c]).abs();
                assert!(
                    diff < tolerance,
                    "Mismatch at ({iu},{iv}) component {c}: cpu={cpu} gpu={gpu} diff={diff}",
                    cpu = cpu_pt[c],
                    gpu = gpu_pt[c],
                );
            }
        }
    }
}

#[test]
fn test_degree_exceeds_max() {
    let handler = match try_init_device() {
        Some(h) => h,
        None => {
            eprintln!("Skipping test_degree_exceeds_max: no GPU adapter available.");
            return;
        }
    };

    let tess = GpuTessellator::new(&handler);
    let mut data = flat_plane_data();
    data.degree_u = MAX_DEGREE + 1;

    let result = tess.tessellate(&data, 4, 4, 1e-4);
    assert!(
        result.is_err(),
        "Expected an error for degree > MAX_DEGREE."
    );
    match result {
        Err(GpuTessellatorError::DegreeExceedsMax { degree, max }) => {
            assert_eq!(degree, MAX_DEGREE + 1);
            assert_eq!(max, MAX_DEGREE);
        }
        other => panic!("Expected DegreeExceedsMax error, got: {other:?}"),
    }
}

#[test]
fn test_degenerate_surface() {
    let handler = match try_init_device() {
        Some(h) => h,
        None => {
            eprintln!("Skipping test_degenerate_surface: no GPU adapter available.");
            return;
        }
    };

    let tess = GpuTessellator::new(&handler);
    // Degenerate: all control points at the origin (collapsed patch).
    let data = NurbsSurfaceData {
        control_points: vec![
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
        knots_u: vec![0.0, 0.0, 1.0, 1.0],
        knots_v: vec![0.0, 0.0, 1.0, 1.0],
        degree_u: 1,
        degree_v: 1,
        num_cp_u: 2,
        num_cp_v: 2,
    };

    // Should not panic, even if normals are degenerate.
    let result = tess.tessellate(&data, 4, 4, 1e-4).unwrap();
    assert_eq!(result.vertices.len(), 16);

    // All vertices should be at the origin.
    for vert in &result.vertices {
        for c in 0..3 {
            assert!(vert[c].abs() < 1e-6, "Degenerate vertex not at origin.");
        }
    }
}

#[test]
fn test_adaptive_refinement() {
    let handler = match try_init_device() {
        Some(h) => h,
        None => {
            eprintln!("Skipping test_adaptive_refinement: no GPU adapter available.");
            return;
        }
    };

    let tess = GpuTessellator::new(&handler);
    // Use the sphere octant which has varying curvature.
    let data = sphere_octant_data();

    // Run adaptive tessellation with a tight tolerance to force refinement.
    let result: AdaptiveResult = tess.tessellate_adaptive(&data, 8, 8, 0.01).unwrap();

    // The adaptive result should have more points than the coarse grid.
    let coarse_count = 8u32 * 8;
    assert!(
        result.total_vertices() as u32 > coarse_count,
        "Adaptive tessellation should produce more vertices ({}) than coarse grid ({coarse_count}).",
        result.total_vertices()
    );

    // Verify the coarse pass has the expected dimensions.
    assert_eq!(result.coarse.grid_u, 8);
    assert_eq!(result.coarse.grid_v, 8);

    // Verify that at least one refinement pass occurred.
    assert!(
        !result.refinements.is_empty(),
        "Adaptive tessellation should have at least one refinement pass."
    );
}
