use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::buffers::GpuBuffer;
use super::ShaderComposer;

/// Wavelet parameters uniform — matches WGSL WaveletParams (16 bytes).
///
/// Layout:
/// | Field   | Offset |
/// |---------|--------|
/// | rows    | 0      |
/// | cols    | 4      |
/// | _pad    | 8      |
/// | _pad2   | 12     |
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct WaveletParams {
    pub rows: u32,
    pub cols: u32,
    pub _pad: u32,
    pub _pad2: u32,
}

const _: () = assert!(
    std::mem::size_of::<WaveletParams>() == 16,
    "WaveletParams must be 16 bytes to match WGSL WaveletParams struct"
);

const TILE_SIZE: u32 = 16;

pub struct WaveletKernel {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl WaveletKernel {
    pub fn new(device: &wgpu::Device, shader_source: &str) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("wavelet_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("wavelet_bind_group_layout"),
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
                            std::num::NonZeroU64::new(std::mem::size_of::<WaveletParams>() as u64)
                                .unwrap(),
                        ),
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("wavelet_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("wavelet_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main_wavelet",
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }

    pub fn from_composer(device: &wgpu::Device, composer: &ShaderComposer) -> Self {
        let shader_source = composer.compose("wavelet");
        Self::new(device, &shader_source)
    }

    /// Dispatch the 2D Haar wavelet kernel.
    ///
    /// - `input_buf`: Input matrix (rows × cols)
    /// - `output_buf`: Output matrix (same size, quadrant layout)
    pub fn dispatch(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        input_buf: &GpuBuffer,
        output_buf: &GpuBuffer,
        rows: u32,
        cols: u32,
    ) {
        let params = WaveletParams {
            rows,
            cols,
            _pad: 0,
            _pad2: 0,
        };

        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("wavelet_params"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("wavelet_bind_group"),
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
                                std::mem::size_of::<WaveletParams>() as u64
                            ),
                        }),
                    },
                ],
            });

        // Each workgroup handles TILE_SIZE×TILE_SIZE 2×2 blocks
        let workgroup_count_x = ((cols + 1) / 2 + TILE_SIZE - 1) / TILE_SIZE;
        let workgroup_count_y = ((rows + 1) / 2 + TILE_SIZE - 1) / TILE_SIZE;

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("wavelet_pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
    }

    /// Output element count (same as input).
    pub fn output_count(rows: u32, cols: u32) -> usize {
        (rows * cols) as usize
    }

    /// Output buffer size in bytes.
    pub fn output_size(rows: u32, cols: u32) -> usize {
        Self::output_count(rows, cols) * std::mem::size_of::<f32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavelet_params_size() {
        assert_eq!(std::mem::size_of::<WaveletParams>(), 16);
        assert_eq!(std::mem::align_of::<WaveletParams>(), 4);
    }

    #[test]
    fn test_wavelet_params_is_pod() {
        let params = WaveletParams {
            rows: 64,
            cols: 64,
            _pad: 0,
            _pad2: 0,
        };

        let bytes = bytemuck::bytes_of(&params);
        assert_eq!(bytes.len(), 16);

        let recovered: &WaveletParams = bytemuck::from_bytes(bytes);
        assert_eq!(recovered.rows, 64);
        assert_eq!(recovered.cols, 64);
    }

    #[test]
    fn test_wavelet_params_is_zeroable() {
        let zeroed: WaveletParams = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.rows, 0);
        assert_eq!(zeroed.cols, 0);
        assert_eq!(zeroed._pad, 0);
        assert_eq!(zeroed._pad2, 0);
    }

    #[test]
    fn test_output_count_and_size() {
        // 8×8 matrix → 64 elements
        assert_eq!(WaveletKernel::output_count(8, 8), 64);
        assert_eq!(WaveletKernel::output_size(8, 8), 64 * 4);

        // 128×128 → 16384 elements
        assert_eq!(WaveletKernel::output_count(128, 128), 16_384);
        assert_eq!(WaveletKernel::output_size(128, 128), 16_384 * 4);
    }

    #[test]
    fn test_wavelet_params_odd_dimensions() {
        let params = WaveletParams {
            rows: 7,
            cols: 5,
            _pad: 0,
            _pad2: 0,
        };
        assert_eq!(params.rows, 7);
        assert_eq!(params.cols, 5);
        assert_eq!(WaveletKernel::output_count(7, 5), 35);
    }
}
