---
phase: 4-strategic-evolution
plan: 4
tags: [gpu, nurbs, tessellation, wgpu, compute-shader]
key-files:
  - monstertruck-gpu/shaders/nurbs_tessellation.wgsl
  - monstertruck-gpu/src/compute.rs
  - monstertruck-gpu/src/lib.rs
  - monstertruck-gpu/tests/compute_tessellation.rs
  - monstertruck-gpu/tests/nurbs_shader_validation.rs
  - monstertruck-gpu/benches/gpu_vs_cpu_tessellation.rs
  - monstertruck-gpu/Cargo.toml
decisions:
  - "Used inline CPU NURBS evaluator in tests to avoid monstertruck-meshing dependency"
  - "Fixed pre-existing build errors (multiview->multiview_mask, naga ambiguity, const fn) to unblock compilation"
  - "Used fixed-grid with multi-pass host-side adaptive refinement for prototype"
  - "Tolerance field stored as f32 bits in ControlParams for uniform buffer alignment"
metrics:
  tests-added: 12
  tests-passing: 12
  files-created: 5
  files-modified: 4
---

## What was built

- **WGSL compute shader** (`shaders/nurbs_tessellation.wgsl`): Evaluates NURBS surfaces on GPU via B-spline basis functions and tensor-product evaluation. Declares `MAX_DEGREE` override constant (default 8). Uses `@workgroup_size(8, 8, 1)` dispatch. Computes surface normals via finite differences.

- **Rust host code** (`src/compute.rs`): `GpuTessellator` struct manages compute pipeline, bind groups, and buffer staging. `tessellate()` validates degree constraints, uploads data, dispatches shader, and reads back results. `tessellate_adaptive()` implements multi-pass coarse-to-fine refinement based on normal deviation.

- **Integration tests** (`tests/compute_tessellation.rs`): 5 tests -- flat plane, sphere-vs-CPU (within 1e-4 tolerance), degree validation, degenerate surface, adaptive refinement. All tests skip gracefully without GPU.

- **Shader validation tests** (`tests/nurbs_shader_validation.rs`): 7 tests verifying WGSL parse, compute entry point, MAX_DEGREE override, and required functions.

- **Benchmark** (`benches/gpu_vs_cpu_tessellation.rs`): criterion benchmark comparing GPU vs CPU at 16x16, 32x32, 64x64 grid sizes.

## Task commits

| SHA | Message |
|-----|---------|
| `582afb9c` | test(gpu): add failing tests for NURBS shader WGSL validation |
| `c5fba85f` | feat(gpu): create WGSL compute shader for NURBS surface evaluation |
| `551b3cb2` | test(gpu): add failing integration tests for GPU NURBS tessellation |
| `b21f0bc7` | feat(gpu): implement Rust host code for GPU NURBS tessellation |
| `26499dd6` | refactor(gpu): clean up compute module |
| `fa7f5ac1` | feat(gpu): add adaptive tessellation, integration tests, and benchmark |

## Deviations

- **Auto-fix (bug):** Pre-existing compilation errors in monstertruck-gpu: `multiview` renamed to `multiview_mask` in wgpu API, `naga` import ambiguity, `const fn` incompatibility in `tests/bindgroup.rs`. Fixed to unblock builds.

## Checkpoint: human-verify

Task 3 is a `checkpoint:human-verify`. User should verify:
1. GPU output matches CPU within tolerance (tests confirm 1e-4)
2. Benchmark produces reasonable numbers
3. Adaptive refinement generates more points for curved surfaces
