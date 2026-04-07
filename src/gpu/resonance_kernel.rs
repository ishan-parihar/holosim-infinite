use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::buffers::GpuBuffer;
use super::ShaderComposer;

/// Resonance-specific weights (default from CPU: 0.30/0.40/0.30).
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ResonanceWeights {
    pub weight_spectrum_res: f32,
    pub weight_holographic_res: f32,
    pub weight_polarity_res: f32,
}

impl Default for ResonanceWeights {
    fn default() -> Self {
        Self {
            weight_spectrum_res: 0.30,
            weight_holographic_res: 0.40,
            weight_polarity_res: 0.30,
        }
    }
}

/// Matches DispatchParams WGSL struct layout.
/// Size: 64 bytes (16 fields × 4 bytes each).
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct DispatchParamsUniform {
    num_entities: u32,
    max_connections: u32,
    _pad0: u32,
    _pad1: u32,
    weight_archetype: f32,
    weight_spectrum: f32,
    weight_blueprint: f32,
    weight_resonance: f32,
    weight_phase: f32,
    weight_karmic: f32,
    _pad2: u32,
    _pad3: u32,
    weight_spectrum_res: f32,
    weight_holographic_res: f32,
    weight_polarity_res: f32,
    max_spectrum_diff: f32,
}

const _: () = assert!(
    std::mem::size_of::<DispatchParamsUniform>() == 64,
    "DispatchParamsUniform must be 64 bytes to match WGSL DispatchParams struct"
);

/// Result of a single pairwise resonance computation.
///
/// Layout matches ResonanceOutput WGSL struct (32 bytes):
/// | Field                  | Offset |
/// |------------------------|--------|
/// | resonance_score        | 0      |
/// | spectrum_resonance     | 4      |
/// | holographic_resonance  | 8      |
/// | polarity_resonance     | 12     |
/// | resonance_type         | 16     |
/// | _pad                   | 20     |
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ResonanceResult {
    pub resonance_score: f32,
    pub spectrum_resonance: f32,
    pub holographic_resonance: f32,
    pub polarity_resonance: f32,
    pub resonance_type: u32,
    pub _pad: [u32; 3],
}

const _: () = assert!(
    std::mem::size_of::<ResonanceResult>() == 32,
    "ResonanceResult must be 32 bytes to match WGSL ResonanceOutput struct"
);

const TILE_SIZE: u32 = 16;

pub struct ResonanceKernel {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl ResonanceKernel {
    pub fn new(device: &wgpu::Device, shader_source: &str) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("resonance_matrix_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("resonance_matrix_bind_group_layout"),
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
                            std::num::NonZeroU64::new(
                                std::mem::size_of::<DispatchParamsUniform>() as u64
                            )
                            .unwrap(),
                        ),
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("resonance_matrix_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("resonance_matrix_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main_resonance_matrix",
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }

    pub fn from_composer(device: &wgpu::Device, composer: &ShaderComposer) -> Self {
        let shader_source = composer.compose("resonance_matrix");
        Self::new(device, &shader_source)
    }

    pub fn dispatch(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        entity_buffer: &GpuBuffer,
        output_buffer: &GpuBuffer,
        num_entities: usize,
        weights: &ResonanceWeights,
    ) {
        let num_entities_u32 = num_entities as u32;
        let workgroup_count_x = (num_entities_u32 + TILE_SIZE - 1) / TILE_SIZE;
        let workgroup_count_y = (num_entities_u32 + TILE_SIZE - 1) / TILE_SIZE;

        let uniform_data = DispatchParamsUniform {
            num_entities: num_entities_u32,
            max_connections: num_entities_u32,
            _pad0: 0,
            _pad1: 0,
            weight_archetype: 0.0, // Not used by resonance kernel
            weight_spectrum: 0.0,
            weight_blueprint: 0.0,
            weight_resonance: 0.0,
            weight_phase: 0.0,
            weight_karmic: 0.0,
            _pad2: 0,
            _pad3: 0,
            weight_spectrum_res: weights.weight_spectrum_res,
            weight_holographic_res: weights.weight_holographic_res,
            weight_polarity_res: weights.weight_polarity_res,
            max_spectrum_diff: 20.0,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("resonance_matrix_uniforms"),
            contents: bytemuck::bytes_of(&uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("resonance_matrix_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: entity_buffer.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output_buffer.as_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: std::num::NonZeroU64::new(
                            std::mem::size_of::<DispatchParamsUniform>() as u64,
                        ),
                    }),
                },
            ],
        });

        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("resonance_matrix_pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&self.pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroup_count_x, workgroup_count_y, 1);
    }

    pub fn output_count(num_entities: usize) -> usize {
        num_entities * num_entities
    }

    pub fn output_size(num_entities: usize) -> usize {
        Self::output_count(num_entities) * std::mem::size_of::<ResonanceResult>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_weights_defaults() {
        let weights = ResonanceWeights::default();
        assert!((weights.weight_spectrum_res - 0.30).abs() < f32::EPSILON);
        assert!((weights.weight_holographic_res - 0.40).abs() < f32::EPSILON);
        assert!((weights.weight_polarity_res - 0.30).abs() < f32::EPSILON);

        let sum = weights.weight_spectrum_res
            + weights.weight_holographic_res
            + weights.weight_polarity_res;
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_resonance_result_is_pod() {
        let result = ResonanceResult {
            resonance_score: 0.75,
            spectrum_resonance: 0.8,
            holographic_resonance: 0.7,
            polarity_resonance: 1.0,
            resonance_type: 2,
            _pad: [0, 0, 0],
        };

        let bytes = bytemuck::bytes_of(&result);
        assert_eq!(bytes.len(), 32);

        let recovered: &ResonanceResult = bytemuck::from_bytes(bytes);
        assert!((recovered.resonance_score - 0.75).abs() < f32::EPSILON);
        assert_eq!(recovered.resonance_type, 2);
    }

    #[test]
    fn test_resonance_result_is_zeroable() {
        let zeroed: ResonanceResult = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.resonance_score, 0.0);
        assert_eq!(zeroed.spectrum_resonance, 0.0);
        assert_eq!(zeroed.holographic_resonance, 0.0);
        assert_eq!(zeroed.polarity_resonance, 0.0);
        assert_eq!(zeroed.resonance_type, 0);
        assert_eq!(zeroed._pad, [0, 0, 0]);
    }

    #[test]
    fn test_resonance_result_size() {
        assert_eq!(std::mem::size_of::<ResonanceResult>(), 32);
        assert_eq!(std::mem::align_of::<ResonanceResult>(), 4);
    }

    #[test]
    fn test_dispatch_params_uniform_size() {
        assert_eq!(std::mem::size_of::<DispatchParamsUniform>(), 64);
    }

    #[test]
    fn test_output_count_and_size() {
        assert_eq!(ResonanceKernel::output_count(10), 100);
        assert_eq!(ResonanceKernel::output_size(10), 100 * 32);
        assert_eq!(ResonanceKernel::output_count(128), 16_384);
        assert_eq!(ResonanceKernel::output_size(128), 16_384 * 32);
    }

    #[test]
    fn test_weights_pod_safety() {
        let weights = ResonanceWeights::default();
        let bytes = bytemuck::bytes_of(&weights);
        assert_eq!(bytes.len(), std::mem::size_of::<ResonanceWeights>());

        let recovered: &ResonanceWeights = bytemuck::from_bytes(bytes);
        assert!((recovered.weight_holographic_res - 0.40).abs() < f32::EPSILON);
    }

    #[test]
    fn test_weights_zeroable() {
        let zeroed: ResonanceWeights = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.weight_spectrum_res, 0.0);
        assert_eq!(zeroed.weight_holographic_res, 0.0);
        assert_eq!(zeroed.weight_polarity_res, 0.0);
    }

    #[test]
    fn test_resonance_type_values() {
        // Resonance type classification: 0=None, 1=Low, 2=Medium, 3=High
        let result_none = ResonanceResult {
            resonance_score: 0.0,
            spectrum_resonance: 0.0,
            holographic_resonance: 0.0,
            polarity_resonance: 0.0,
            resonance_type: 0,
            _pad: [0, 0, 0],
        };
        assert_eq!(result_none.resonance_type, 0);

        let result_high = ResonanceResult {
            resonance_score: 1.0,
            spectrum_resonance: 1.0,
            holographic_resonance: 1.0,
            polarity_resonance: 1.0,
            resonance_type: 3,
            _pad: [0, 0, 0],
        };
        assert_eq!(result_high.resonance_type, 3);
    }

    #[test]
    fn test_resonance_weights_custom() {
        let weights = ResonanceWeights {
            weight_spectrum_res: 0.25,
            weight_holographic_res: 0.50,
            weight_polarity_res: 0.25,
        };
        assert!((weights.weight_spectrum_res - 0.25).abs() < f32::EPSILON);
        assert!((weights.weight_holographic_res - 0.50).abs() < f32::EPSILON);
        assert!((weights.weight_polarity_res - 0.25).abs() < f32::EPSILON);
    }
}
