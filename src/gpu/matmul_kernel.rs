use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::buffers::GpuBuffer;
use super::ShaderComposer;

/// Tensor shape uniform — matches WGSL TensorShape (16 bytes).
///
/// Layout:
/// | Field | Offset |
/// |-------|--------|
/// | rows  | 0      |
/// | cols  | 4      |
/// | depth | 8      |
/// | _pad  | 12     |
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TensorShape {
    pub rows: u32,
    pub cols: u32,
    pub depth: u32,
    pub _pad: u32,
}

const _: () = assert!(
    std::mem::size_of::<TensorShape>() == 16,
    "TensorShape must be 16 bytes to match WGSL TensorShape struct"
);

const TILE_SIZE: u32 = 16;

pub struct MatmulKernel {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl MatmulKernel {
    pub fn new(device: &wgpu::Device, shader_source: &str) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("matmul_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("matmul_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            std::num::NonZeroU64::new(std::mem::size_of::<TensorShape>() as u64)
                                .unwrap(),
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            std::num::NonZeroU64::new(std::mem::size_of::<TensorShape>() as u64)
                                .unwrap(),
                        ),
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("matmul_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("matmul_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main_matmul",
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }

    pub fn from_composer(device: &wgpu::Device, composer: &ShaderComposer) -> Self {
        let shader_source = composer.compose("matmul");
        Self::new(device, &shader_source)
    }

    /// Dispatch the matmul kernel: C = A × B.
    ///
    /// - `buf_a`: Matrix A (rows_a × cols_a)
    /// - `buf_b`: Matrix B (rows_b × cols_b), must have rows_b == cols_a
    /// - `buf_c`: Output matrix C (rows_a × cols_b)
    pub fn dispatch(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        buf_a: &GpuBuffer,
        buf_b: &GpuBuffer,
        buf_c: &GpuBuffer,
        rows_a: u32,
        cols_a: u32,
        cols_b: u32,
    ) {
        let shape_a = TensorShape {
            rows: rows_a,
            cols: cols_a,
            depth: 1,
            _pad: 0,
        };
        let shape_b = TensorShape {
            rows: cols_a,
            cols: cols_b,
            depth: 1,
            _pad: 0,
        };

        let shape_a_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("matmul_shape_a"),
            contents: bytemuck::bytes_of(&shape_a),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let shape_b_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("matmul_shape_b"),
            contents: bytemuck::bytes_of(&shape_b),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("matmul_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buf_a.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buf_b.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buf_c.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &shape_a_buf,
                        offset: 0,
                        size: std::num::NonZeroU64::new(std::mem::size_of::<TensorShape>() as u64),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &shape_b_buf,
                        offset: 0,
                        size: std::num::NonZeroU64::new(std::mem::size_of::<TensorShape>() as u64),
                    }),
                },
            ],
        });

        let workgroup_count_x = (cols_b + TILE_SIZE - 1) / TILE_SIZE;
        let workgroup_count_y = (rows_a + TILE_SIZE - 1) / TILE_SIZE;

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("matmul_pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
    }

    /// Output element count for C = A(rows_a, cols_a) × B(cols_a, cols_b).
    pub fn output_count(rows_a: u32, cols_b: u32) -> usize {
        (rows_a * cols_b) as usize
    }

    /// Output buffer size in bytes.
    pub fn output_size(rows_a: u32, cols_b: u32) -> usize {
        Self::output_count(rows_a, cols_b) * std::mem::size_of::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_shape_size() {
        assert_eq!(std::mem::size_of::<TensorShape>(), 16);
        assert_eq!(std::mem::align_of::<TensorShape>(), 4);
    }

    #[test]
    fn test_tensor_shape_is_pod() {
        let shape = TensorShape {
            rows: 64,
            cols: 128,
            depth: 1,
            _pad: 0,
        };

        let bytes = bytemuck::bytes_of(&shape);
        assert_eq!(bytes.len(), 16);

        let recovered: &TensorShape = bytemuck::from_bytes(bytes);
        assert_eq!(recovered.rows, 64);
        assert_eq!(recovered.cols, 128);
        assert_eq!(recovered.depth, 1);
    }

    #[test]
    fn test_tensor_shape_is_zeroable() {
        let zeroed: TensorShape = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.rows, 0);
        assert_eq!(zeroed.cols, 0);
        assert_eq!(zeroed.depth, 0);
        assert_eq!(zeroed._pad, 0);
    }

    #[test]
    fn test_output_count_and_size() {
        // 4×3 × 3×2 = 4×2 → 8 elements
        assert_eq!(MatmulKernel::output_count(4, 2), 8);
        assert_eq!(MatmulKernel::output_size(4, 2), 8 * 4);

        // 128×64 × 64×128 = 128×128 → 16384 elements
        assert_eq!(MatmulKernel::output_count(128, 128), 16_384);
        assert_eq!(MatmulKernel::output_size(128, 128), 16_384 * 4);
    }

    #[test]
    fn test_tensor_shape_defaults() {
        let shape = TensorShape {
            rows: 1,
            cols: 1,
            depth: 1,
            _pad: 0,
        };
        assert_eq!(shape.rows, 1);
        assert_eq!(shape.cols, 1);
    }
}
