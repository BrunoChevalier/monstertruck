//! GPU compute tessellation for NURBS surfaces.
//!
//! This module provides [`GpuTessellator`], which evaluates NURBS surfaces
//! on the GPU using a WebGPU compute shader. The shader performs B-spline
//! basis function evaluation and tensor-product surface point computation.
//!
//! ## Degree constraint
//!
//! The WGSL shader uses fixed-size arrays for basis functions, sized
//! `MAX_DEGREE + 1`. The host code validates `degree <= MAX_DEGREE` before
//! dispatch. The default `MAX_DEGREE` is 8.
//!
//! ## CPU reference
//!
//! For comparison testing, an inline CPU evaluator is provided in the
//! integration tests (`tests/compute_tessellation.rs`), avoiding a dependency
//! on `monstertruck-meshing`.

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use wgpu::*;

use crate::DeviceHandler;

/// Maximum supported NURBS degree. Must match the WGSL shader's
/// `override MAX_DEGREE` default value.
pub const MAX_DEGREE: u32 = 8;

/// Input data describing a NURBS surface.
#[derive(Debug, Clone)]
pub struct NurbsSurfaceData {
    /// Homogeneous control points `[x, y, z, w]`.
    pub control_points: Vec<[f32; 4]>,
    /// Knot vector in the u direction.
    pub knots_u: Vec<f32>,
    /// Knot vector in the v direction.
    pub knots_v: Vec<f32>,
    /// Degree in the u direction.
    pub degree_u: u32,
    /// Degree in the v direction.
    pub degree_v: u32,
    /// Number of control points in the u direction.
    pub num_cp_u: u32,
    /// Number of control points in the v direction.
    pub num_cp_v: u32,
}

/// Result of a GPU tessellation.
#[derive(Debug, Clone)]
pub struct TessellationResult {
    /// Evaluated surface points `[x, y, z]`.
    pub vertices: Vec<[f32; 3]>,
    /// Surface normals `[nx, ny, nz]`.
    pub normals: Vec<[f32; 3]>,
    /// Grid resolution in the u direction.
    pub grid_u: u32,
    /// Grid resolution in the v direction.
    pub grid_v: u32,
}

/// Errors from [`GpuTessellator`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuTessellatorError {
    /// A surface degree exceeds `MAX_DEGREE`.
    DegreeExceedsMax {
        /// The offending degree.
        degree: u32,
        /// The maximum allowed degree.
        max: u32,
    },
}

impl std::fmt::Display for GpuTessellatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DegreeExceedsMax { degree, max } => {
                write!(f, "NURBS degree {degree} exceeds MAX_DEGREE {max}")
            }
        }
    }
}

impl std::error::Error for GpuTessellatorError {}

/// Uniform struct matching the WGSL `ControlParams`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct ControlParams {
    degree_u: u32,
    degree_v: u32,
    num_cp_u: u32,
    num_cp_v: u32,
    num_knots_u: u32,
    num_knots_v: u32,
    grid_u: u32,
    grid_v: u32,
    tolerance_bits: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

/// GPU-based NURBS surface tessellator.
///
/// Holds the compute pipeline and bind group layouts. Created once and reused
/// for multiple tessellation calls.
#[derive(Debug)]
pub struct GpuTessellator<'a> {
    device: &'a Device,
    queue: &'a Queue,
    pipeline: ComputePipeline,
    input_bgl: BindGroupLayout,
    output_bgl: BindGroupLayout,
}

impl<'a> GpuTessellator<'a> {
    /// Create a new [`GpuTessellator`] from a [`DeviceHandler`].
    pub fn new(handler: &'a DeviceHandler) -> Self {
        let device = handler.device();
        let queue = handler.queue();

        let shader_source = include_str!("../shaders/nurbs_tessellation.wgsl");
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("nurbs_tessellation.wgsl"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });

        // Bind group layout 0: inputs.
        let input_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("nurbs_input_bgl"),
            entries: &[
                // control_points.
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // knots_u.
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // knots_v.
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // params.
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Bind group layout 1: outputs.
        let output_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("nurbs_output_bgl"),
            entries: &[
                // output_vertices.
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // output_normals.
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("nurbs_pipeline_layout"),
            bind_group_layouts: &[&input_bgl, &output_bgl],
            immediate_size: 0,
        });

        let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("nurbs_compute_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            device,
            queue,
            pipeline,
            input_bgl,
            output_bgl,
        }
    }

    /// Tessellate a NURBS surface on the GPU.
    ///
    /// Returns an error if the surface degree exceeds [`MAX_DEGREE`].
    pub fn tessellate(
        &self,
        surface: &NurbsSurfaceData,
        grid_u: u32,
        grid_v: u32,
        tolerance: f32,
    ) -> Result<TessellationResult, GpuTessellatorError> {
        // Degree validation.
        if surface.degree_u > MAX_DEGREE {
            return Err(GpuTessellatorError::DegreeExceedsMax {
                degree: surface.degree_u,
                max: MAX_DEGREE,
            });
        }
        if surface.degree_v > MAX_DEGREE {
            return Err(GpuTessellatorError::DegreeExceedsMax {
                degree: surface.degree_v,
                max: MAX_DEGREE,
            });
        }

        let total_points = (grid_u * grid_v) as usize;

        // Create input buffers.
        let cp_buffer = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("control_points"),
            contents: bytemuck::cast_slice(&surface.control_points),
            usage: BufferUsages::STORAGE,
        });

        let knots_u_buffer = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("knots_u"),
            contents: bytemuck::cast_slice(&surface.knots_u),
            usage: BufferUsages::STORAGE,
        });

        let knots_v_buffer = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("knots_v"),
            contents: bytemuck::cast_slice(&surface.knots_v),
            usage: BufferUsages::STORAGE,
        });

        let params = ControlParams {
            degree_u: surface.degree_u,
            degree_v: surface.degree_v,
            num_cp_u: surface.num_cp_u,
            num_cp_v: surface.num_cp_v,
            num_knots_u: surface.knots_u.len() as u32,
            num_knots_v: surface.knots_v.len() as u32,
            grid_u,
            grid_v,
            tolerance_bits: tolerance.to_bits(),
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        };
        let params_buffer = self.device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("params"),
            contents: bytemuck::bytes_of(&params),
            usage: BufferUsages::UNIFORM,
        });

        // Output buffers.
        let output_size = (total_points * size_of::<[f32; 4]>()) as u64;

        let vertices_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("output_vertices"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let normals_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("output_normals"),
            size: output_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Staging buffers for readback.
        let staging_vertices = self.device.create_buffer(&BufferDescriptor {
            label: Some("staging_vertices"),
            size: output_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let staging_normals = self.device.create_buffer(&BufferDescriptor {
            label: Some("staging_normals"),
            size: output_size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind groups.
        let input_bg = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("nurbs_input_bg"),
            layout: &self.input_bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: cp_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: knots_u_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: knots_v_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let output_bg = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("nurbs_output_bg"),
            layout: &self.output_bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: vertices_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: normals_buffer.as_entire_binding(),
                },
            ],
        });

        // Dispatch compute shader.
        let workgroup_x = (grid_u + 7) / 8;
        let workgroup_y = (grid_v + 7) / 8;

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("nurbs_compute_encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("nurbs_compute_pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &input_bg, &[]);
            compute_pass.set_bind_group(1, &output_bg, &[]);
            compute_pass.dispatch_workgroups(workgroup_x, workgroup_y, 1);
        }

        // Copy output to staging buffers.
        encoder.copy_buffer_to_buffer(&vertices_buffer, 0, &staging_vertices, 0, output_size);
        encoder.copy_buffer_to_buffer(&normals_buffer, 0, &staging_normals, 0, output_size);

        self.queue.submit(std::iter::once(encoder.finish()));

        // Read back results.
        let vertices = self.read_buffer_vec4(&staging_vertices, total_points);
        let normals = self.read_buffer_vec4(&staging_normals, total_points);

        Ok(TessellationResult {
            vertices,
            normals,
            grid_u,
            grid_v,
        })
    }

    /// Map a staging buffer and extract `[f32; 3]` from `vec4<f32>` data.
    fn read_buffer_vec4(&self, buffer: &Buffer, count: usize) -> Vec<[f32; 3]> {
        let buffer_slice = buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(MapMode::Read, move |result| {
            // SAFETY: The channel send cannot fail since the receiver lives
            // in this function scope.
            sender.send(result).unwrap();
        });
        // SAFETY: `PollType::Wait` with no submission index or timeout blocks
        // until all submitted work completes.
        self.device
            .poll(PollType::Wait {
                submission_index: None,
                timeout: None,
            })
            .unwrap();
        receiver
            .recv()
            .expect("GPU buffer map callback never fired")
            .expect("GPU buffer map failed");

        let data = buffer_slice.get_mapped_range();
        let floats: &[[f32; 4]] = bytemuck::cast_slice(&data);
        let result: Vec<[f32; 3]> = floats.iter().take(count).map(|v| [v[0], v[1], v[2]]).collect();
        drop(data);
        buffer.unmap();
        result
    }
}
