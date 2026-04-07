use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::buffers::GpuBuffer;
use super::ShaderComposer;

/// Upsample parameters uniform — matches WGSL UpsampleParams (16 bytes).
///
/// Layout:
/// | Field        | Offset |
/// |--------------|--------|
/// | input_rows   | 0      |
/// | input_cols   | 4      |
/// | _pad         | 8      |
/// | _pad2        | 12     |
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct UpsampleParams {
    pub input_rows: u32,
    pub input_cols: u32,
    pub _pad: u32,
    pub _pad2: u32,
}

const _: () = assert!(
    std::mem::size_of::<UpsampleParams>() == 16,
    "UpsampleParams must be 16 bytes to match WGSL UpsampleParams struct"
);

const TILE_SIZE: u32 = 16;

pub struct UpsampleKernel {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl UpsampleKernel {
    pub fn new(device: &wgpu::Device, shader_source: &str) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("upsample_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("upsample_bind_group_layout"),
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
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(
                            std::num::NonZeroU64::new(std::mem::size_of::<UpsampleParams>() as u64)
                                .unwrap(),
                        ),
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("upsample_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("upsample_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main_upsample",
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }

    pub fn from_composer(device: &wgpu::Device, composer: &ShaderComposer) -> Self {
        let shader_source = composer.compose("upsample");
        Self::new(device, &shader_source)
    }

    /// Dispatch the 2× nearest-neighbor upsampling kernel.
    ///
    /// - `input_buf`: Input matrix (input_rows × input_cols)
    /// - `output_buf`: Output matrix (input_rows*2 × input_cols*2)
    pub fn dispatch(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input_buf: &GpuBuffer,
        output_buf: &GpuBuffer,
        input_rows: u32,
        input_cols: u32,
    ) {
        let params = UpsampleParams {
            input_rows,
            input_cols,
            _pad: 0,
            _pad2: 0,
        };

        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("upsample_params"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("upsample_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buf.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buf.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buf,
                        offset: 0,
                        size: std::num::NonZeroU64::new(
                            std::mem::size_of::<UpsampleParams>() as u64
                        ),
                    }),
                },
            ],
        });

        // Output is 2× input in each dimension
        let output_cols = input_cols * 2;
        let output_rows = input_rows * 2;
        let workgroup_count_x = (output_cols + TILE_SIZE - 1) / TILE_SIZE;
        let workgroup_count_y = (output_rows + TILE_SIZE - 1) / TILE_SIZE;

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("upsample_pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
    }

    /// Output element count for 2× upsampled matrix.
    pub fn output_count(input_rows: u32, input_cols: u32) -> usize {
        ((input_rows * 2) * (input_cols * 2)) as usize
    }

    /// Output buffer size in bytes.
    pub fn output_size(input_rows: u32, input_cols: u32) -> usize {
        Self::output_count(input_rows, input_cols) * std::mem::size_of::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upsample_params_size() {
        assert_eq!(std::mem::size_of::<UpsampleParams>(), 16);
        assert_eq!(std::mem::align_of::<UpsampleParams>(), 4);
    }

    #[test]
    fn test_upsample_params_is_pod() {
        let params = UpsampleParams {
            input_rows: 32,
            input_cols: 64,
            _pad: 0,
            _pad2: 0,
        };

        let bytes = bytemuck::bytes_of(&params);
        assert_eq!(bytes.len(), 16);

        let recovered: &UpsampleParams = bytemuck::from_bytes(bytes);
        assert_eq!(recovered.input_rows, 32);
        assert_eq!(recovered.input_cols, 64);
    }

    #[test]
    fn test_upsample_params_is_zeroable() {
        let zeroed: UpsampleParams = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.input_rows, 0);
        assert_eq!(zeroed.input_cols, 0);
        assert_eq!(zeroed._pad, 0);
        assert_eq!(zeroed._pad2, 0);
    }

    #[test]
    fn test_output_count_and_size() {
        // 4×4 input → 8×8 output → 64 elements
        assert_eq!(UpsampleKernel::output_count(4, 4), 64);
        assert_eq!(UpsampleKernel::output_size(4, 4), 64 * 4);

        // 8×8 input → 16×16 output → 256 elements
        assert_eq!(UpsampleKernel::output_count(8, 8), 256);
        assert_eq!(UpsampleKernel::output_size(8, 8), 256 * 4);

        // 128×128 → 256×256 → 65536 elements
        assert_eq!(UpsampleKernel::output_count(128, 128), 65_536);
        assert_eq!(UpsampleKernel::output_size(128, 128), 65_536 * 4);
    }

    #[test]
    fn test_upsample_params_4x_factor() {
        // Output is always 4× the input element count (2× each dimension)
        let input_elements = 32 * 16;
        let output_elements = UpsampleKernel::output_count(32, 16);
        assert_eq!(output_elements, input_elements * 4);
    }
}
