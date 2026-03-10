---
phase: 4-strategic-evolution
plan: 4
type: execute
wave: 3
depends_on: ["4-2", "4-3"]
files_modified:
  - monstertruck-gpu/Cargo.toml
  - monstertruck-gpu/src/compute.rs
  - monstertruck-gpu/src/lib.rs
  - monstertruck-gpu/shaders/nurbs_tessellation.wgsl
  - monstertruck-gpu/tests/compute_tessellation.rs
  - monstertruck-gpu/benches/gpu_vs_cpu_tessellation.rs
autonomous: false
must_haves:
  truths:
    - "User can submit NURBS control points and knot vectors to the GPU and receive tessellated vertices"
    - "GPU tessellation output matches CPU tessellation output within a configurable tolerance"
    - "The compute shader performs adaptive subdivision based on surface curvature"
    - "A benchmark compares GPU vs CPU tessellation performance for a reference surface"
    - "WGSL shader declares MAX_DEGREE as an override constant and documents the degree constraint"
  artifacts:
    - path: "monstertruck-gpu/shaders/nurbs_tessellation.wgsl"
      provides: "WGSL compute shader for adaptive NURBS surface subdivision with MAX_DEGREE override constant"
      min_lines: 100
      contains: "@compute"
    - path: "monstertruck-gpu/src/compute.rs"
      provides: "Rust host code for GPU compute pipeline setup, buffer management, and result readback"
      min_lines: 150
      contains: "ComputePipeline"
    - path: "monstertruck-gpu/tests/compute_tessellation.rs"
      provides: "Integration test comparing GPU tessellation output to CPU reference"
      min_lines: 80
      contains: "tolerance"
  key_links:
    - from: "monstertruck-gpu/src/compute.rs"
      to: "monstertruck-gpu/shaders/nurbs_tessellation.wgsl"
      via: "shader loading and compute pipeline creation"
      pattern: "nurbs_tessellation.wgsl"
    - from: "monstertruck-gpu/tests/compute_tessellation.rs"
      to: "monstertruck-gpu/src/compute.rs"
      via: "test exercises the compute tessellation API"
      pattern: "GpuTessellator"
    - from: "monstertruck-gpu/src/compute.rs"
      to: "monstertruck-gpu/src/lib.rs"
      via: "module declaration and public API export"
      pattern: "mod compute"
---

<objective>
Prototype GPU-accelerated NURBS tessellation using WebGPU compute shaders, producing tessellation output that matches the CPU path within tolerance, as a proof-of-concept for future GPU-accelerated geometry processing.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-gpu/src/lib.rs
@monstertruck-gpu/Cargo.toml
@monstertruck-gpu/wgsl-utils/math.wgsl
@monstertruck-meshing/src/tessellation/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create WGSL compute shader for NURBS surface evaluation</name>
  <files>monstertruck-gpu/shaders/nurbs_tessellation.wgsl</files>
  <action>
Create a WGSL compute shader that evaluates a NURBS surface at a grid of parameter points:

1. **Create directory** `monstertruck-gpu/shaders/` if not exists.

2. **Create `nurbs_tessellation.wgsl`** with:

   **Override constant for degree limitation**:
   ```wgsl
   // WGSL does not support dynamic array sizes in functions.
   // MAX_DEGREE defines the maximum supported NURBS degree.
   // Surfaces with degree > MAX_DEGREE will produce incorrect results.
   // The host code must validate degree <= MAX_DEGREE before dispatch.
   override MAX_DEGREE: u32 = 8u;
   ```
   All basis function arrays must be declared as `array<f32, 9>` (MAX_DEGREE + 1) or use the override constant where WGSL allows it. Document this constraint prominently at the top of the shader file.

   **Input buffers** (bind group 0):
   - `control_points: array<vec4<f32>>` -- homogeneous control points (x,y,z,w)
   - `knots_u: array<f32>` -- knot vector in u direction
   - `knots_v: array<f32>` -- knot vector in v direction
   - `params: ControlParams` -- struct with degree_u, degree_v, num_cp_u, num_cp_v, num_knots_u, num_knots_v, grid_u, grid_v, tolerance

   **Output buffer** (bind group 1):
   - `output_vertices: array<vec4<f32>>` -- evaluated surface points (x,y,z,1)
   - `output_normals: array<vec4<f32>>` -- surface normals at each point

   **Compute shader logic**:
   - Each workgroup thread evaluates one (u,v) parameter pair
   - Implement `find_span(knots, n, degree, t) -> u32` -- find knot span for parameter t
   - Implement `basis_funs(knots, span, degree, t) -> array<f32, 9>` -- compute B-spline basis functions. The array size is MAX_DEGREE + 1 (fixed at 9 for MAX_DEGREE=8). Only indices 0..degree are valid; remaining entries are 0.
   - Implement `surface_point(u, v)` -- evaluate NURBS surface using tensor product of basis functions
   - Implement `surface_normal(u, v)` -- compute normal via partial derivatives (finite differences or analytic)
   - Use `@workgroup_size(8, 8, 1)` for 2D grid dispatch
   - Handle rational projection (divide by weight component)

   **Adaptive subdivision** (simplified):
   - First pass: evaluate on coarse grid
   - Second pass: for cells where normals deviate beyond tolerance, subdivide further
   - Use atomics or prefix sum for dynamic output sizing (or use fixed max grid for prototype)

   Note: For the prototype, a fixed-resolution grid is acceptable. Adaptive refinement can use a multi-pass approach where the host reads back curvature data and re-dispatches.
  </action>
  <verify>Compile the shader using naga validation: the test infrastructure already has naga as a dev-dependency. Write a minimal test that loads and validates the WGSL shader syntax. Verify the MAX_DEGREE override constant is declared and documented.</verify>
  <done>WGSL compute shader for NURBS surface evaluation was created with MAX_DEGREE override constant, basis function evaluation, surface point computation, and normal calculation.</done>
</task>

<task type="auto">
  <name>Task 2: Implement Rust host code for GPU compute tessellation</name>
  <files>monstertruck-gpu/src/compute.rs, monstertruck-gpu/src/lib.rs, monstertruck-gpu/Cargo.toml</files>
  <action>
Create the Rust-side infrastructure for GPU compute tessellation:

1. **Update `monstertruck-gpu/Cargo.toml`**:
   - Add `bytemuck` derive features if not present (needed for buffer data)
   - Ensure `wgpu` has compute shader support (it does by default)
   - Note: monstertruck-gpu depends on monstertruck-core (which after plan 4-2 uses nalgebra via monstertruck-math). The cgmath64 module in monstertruck-core is preserved as a compatibility shim, so existing GPU imports (`use monstertruck_core::cgmath64::*`) continue to work.

2. **CPU tessellation reference**: For the comparison tests in Task 3, the compute module needs a CPU evaluator for NURBS surfaces. Options:
   - Use an inline NURBS evaluator within the test (preferred for prototype -- avoids adding monstertruck-meshing as a dependency)
   - Implement a simple `cpu_evaluate_nurbs_surface(surface_data, u, v) -> [f32; 3]` function in the test file that mirrors the shader logic
   - Document this decision in the compute.rs module docs

3. **Create `monstertruck-gpu/src/compute.rs`**:

   ```rust
   pub struct GpuTessellator { ... }
   ```

   **GpuTessellator fields**:
   - `device: &Device` and `queue: &Queue` (borrowed from DeviceHandler)
   - `pipeline: ComputePipeline`
   - `bind_group_layout: BindGroupLayout`

   **GpuTessellator methods**:
   - `new(device_handler: &DeviceHandler) -> Self` -- create compute pipeline from shader
   - `tessellate(surface: &NurbsSurfaceData, grid_u: u32, grid_v: u32, tolerance: f32) -> TessellationResult`
     - **Validate** `surface.degree_u <= MAX_DEGREE && surface.degree_v <= MAX_DEGREE` before dispatch. Return an error if exceeded.
     - Upload control points, knot vectors, and params to GPU buffers
     - Create bind groups
     - Dispatch compute shader
     - Read back results from output buffer
     - Return vertices and normals

   **NurbsSurfaceData struct** (input):
   - `control_points: Vec<[f32; 4]>` -- homogeneous control points
   - `knots_u: Vec<f32>`
   - `knots_v: Vec<f32>`
   - `degree_u: u32`
   - `degree_v: u32`
   - `num_cp_u: u32`
   - `num_cp_v: u32`

   **TessellationResult struct** (output):
   - `vertices: Vec<[f32; 3]>`
   - `normals: Vec<[f32; 3]>`
   - `grid_u: u32`
   - `grid_v: u32`

   **Buffer management**:
   - Use `wgpu::util::DeviceExt::create_buffer_init` for input buffers
   - Use staging buffer pattern for reading back compute results
   - Map output buffer and copy data to CPU

   **Adaptive refinement** (multi-pass):
   - First dispatch on coarse grid (e.g., 16x16)
   - Read back normals, identify cells with high curvature variation
   - Re-dispatch on refined grid for those regions
   - Merge results

4. **Update `monstertruck-gpu/src/lib.rs`**:
   - Add `pub mod compute;`
   - Re-export `GpuTessellator`, `NurbsSurfaceData`, `TessellationResult`
  </action>
  <verify>`cargo check -p monstertruck-gpu` compiles. The compute module is properly integrated into the crate. Verify degree validation exists in the tessellate method.</verify>
  <done>GPU compute tessellation host code was implemented with pipeline setup, buffer management, MAX_DEGREE validation, and result readback.</done>
</task>

<task type="checkpoint:human-verify">
  <name>Task 3: Integration test comparing GPU and CPU tessellation</name>
  <files>monstertruck-gpu/tests/compute_tessellation.rs, monstertruck-gpu/benches/gpu_vs_cpu_tessellation.rs</files>
  <action>
Create integration tests and a benchmark comparing GPU and CPU tessellation:

1. **Create `monstertruck-gpu/tests/compute_tessellation.rs`**:

   **Inline CPU evaluator**: Implement a simple `cpu_evaluate_nurbs_surface(data: &NurbsSurfaceData, u: f32, v: f32) -> [f32; 3]` that mirrors the WGSL shader's logic (find_span, basis_funs, tensor product evaluation). This avoids adding monstertruck-meshing as a dependency.

   **Test: `test_flat_plane_tessellation`**:
   - Create a flat bilinear NURBS patch (degree 1x1, 4 control points)
   - Tessellate on GPU with 8x8 grid
   - Verify all output vertices lie on the expected plane
   - Verify normals are uniform and correct

   **Test: `test_sphere_tessellation_matches_cpu`**:
   - Create a NURBS sphere octant (degree 2x2, rational)
   - Tessellate on GPU with 16x16 grid
   - Evaluate the same surface on CPU at the same parameter values using the inline evaluator
   - Assert each GPU point matches the CPU point within tolerance (1e-4)
   - Assert normals match within angular tolerance

   **Test: `test_adaptive_refinement`**:
   - Create a surface with varying curvature (e.g., a cylinder with a bump)
   - Run adaptive tessellation
   - Verify that high-curvature regions have more points than flat regions

   **Test: `test_degenerate_surface`**:
   - Test with a degenerate NURBS surface (collapsed edge)
   - Verify no panics and reasonable output

   **Test: `test_degree_exceeds_max`**:
   - Create a surface with degree > MAX_DEGREE
   - Verify the tessellate method returns an error (not a GPU crash)

   Note: These tests require GPU access. Use `pollster::block_on` for async wgpu operations. Skip tests gracefully if no GPU adapter is available.

2. **Create `monstertruck-gpu/benches/gpu_vs_cpu_tessellation.rs`**:
   - Add `[[bench]]` entry to Cargo.toml
   - Benchmark GPU tessellation of a NURBS torus at various grid sizes (16x16, 32x32, 64x64)
   - Compare against CPU evaluation (using the inline evaluator) at the same grid sizes
   - Use criterion for benchmarking

3. **Human verification**: After tests run, user should verify that:
   - GPU output visually matches CPU output (if visualization is available)
   - Performance numbers are reasonable (GPU should be faster for large grids)
   - Error tolerance between GPU and CPU results is within acceptable bounds
  </action>
  <verify>
`cargo test -p monstertruck-gpu compute_tessellation` passes (or is skipped if no GPU).
`cargo bench -p monstertruck-gpu --bench gpu_vs_cpu_tessellation` produces comparison results.
User reviews test output and confirms GPU/CPU match within tolerance.
  </verify>
  <done>Integration tests and benchmarks comparing GPU and CPU tessellation were created and verified by user.</done>
</task>

</tasks>

<verification>
1. `monstertruck-gpu/shaders/nurbs_tessellation.wgsl` exists, passes WGSL validation, and declares MAX_DEGREE override constant
2. `cargo check -p monstertruck-gpu` compiles including the compute module
3. `cargo test -p monstertruck-gpu compute_tessellation` passes on systems with GPU
4. GPU tessellation output matches CPU output within 1e-4 tolerance
5. Degree validation prevents dispatch for surfaces with degree > MAX_DEGREE
6. Benchmark shows GPU tessellation performance characteristics
</verification>

<success_criteria>
- A WGSL compute shader performs NURBS surface evaluation on GPU with MAX_DEGREE override constant
- Rust host code manages GPU buffers and compute pipeline with degree validation
- GPU output matches CPU tessellation within configurable tolerance (using inline evaluator, no monstertruck-meshing dependency)
- Adaptive subdivision is prototyped (at least multi-pass coarse-to-fine)
- Integration tests validate correctness, benchmark compares performance
</success_criteria>

<output>
After completion, create `.tendrion/phases/4-strategic-evolution/4-4-SUMMARY.md`
</output>
