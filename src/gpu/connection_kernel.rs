use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::buffers::GpuBuffer;
use super::ShaderComposer;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ConnectionWeights {
    pub weight_archetype: f32,
    pub weight_spectrum: f32,
    pub weight_blueprint: f32,
    pub weight_resonance: f32,
    pub weight_phase: f32,
    pub weight_karmic: f32,
}

impl Default for ConnectionWeights {
    fn default() -> Self {
        Self {
            weight_archetype: 0.35,
            weight_spectrum: 0.20,
            weight_blueprint: 0.15,
            weight_resonance: 0.10,
            weight_phase: 0.10,
            weight_karmic: 0.10,
        }
    }
}

/// Matches DispatchParams WGSL struct.
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

/// Result of a single pairwise connection computation.
///
/// Layout matches ConnectionOutput WGSL struct (32 bytes):
/// | Field                | Offset |
/// |----------------------|--------|
/// | strength             | 0      |
/// | archetype_similarity | 4      |
/// | spectrum_similarity  | 8      |
/// | blueprint_alignment  | 12     |
/// | resonance_match      | 16     |
/// | phase_coherence      | 20     |
/// | karmic_correlation   | 24     |
/// | connection_type      | 28     |
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ConnectionResult {
    pub strength: f32,
    pub archetype_similarity: f32,
    pub spectrum_similarity: f32,
    pub blueprint_alignment: f32,
    pub resonance_match: f32,
    pub phase_coherence: f32,
    pub karmic_correlation: f32,
    pub connection_type: u32,
}

const _: () = assert!(
    std::mem::size_of::<ConnectionResult>() == 32,
    "ConnectionResult must be 32 bytes to match WGSL ConnectionOutput struct"
);

const TILE_SIZE: u32 = 16;

pub struct ConnectionKernel {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl ConnectionKernel {
    pub fn new(device: &wgpu::Device, shader_source: &str) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("connection_metrics_shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("connection_metrics_bind_group_layout"),
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
            label: Some("connection_metrics_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("connection_metrics_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: "main_connection_metrics",
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }

    pub fn from_composer(device: &wgpu::Device, composer: &ShaderComposer) -> Self {
        let shader_source = composer.compose("connection_metrics");
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
        weights: &ConnectionWeights,
    ) {
        let num_entities_u32 = num_entities as u32;
        let workgroup_count_x = (num_entities_u32 + TILE_SIZE - 1) / TILE_SIZE;
        let workgroup_count_y = (num_entities_u32 + TILE_SIZE - 1) / TILE_SIZE;

        let uniform_data = DispatchParamsUniform {
            num_entities: num_entities_u32,
            max_connections: num_entities_u32,
            _pad0: 0,
            _pad1: 0,
            weight_archetype: weights.weight_archetype,
            weight_spectrum: weights.weight_spectrum,
            weight_blueprint: weights.weight_blueprint,
            weight_resonance: weights.weight_resonance,
            weight_phase: weights.weight_phase,
            weight_karmic: weights.weight_karmic,
            _pad2: 0,
            _pad3: 0,
            weight_spectrum_res: 0.30,
            weight_holographic_res: 0.40,
            weight_polarity_res: 0.30,
            max_spectrum_diff: 20.0,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("connection_metrics_uniforms"),
            contents: bytemuck::bytes_of(&uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("connection_metrics_bind_group"),
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
            label: Some("connection_metrics_pass"),
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
        Self::output_count(num_entities) * std::mem::size_of::<ConnectionResult>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_weights_defaults() {
        let weights = ConnectionWeights::default();
        assert!((weights.weight_archetype - 0.35).abs() < f32::EPSILON);
        assert!((weights.weight_spectrum - 0.20).abs() < f32::EPSILON);
        assert!((weights.weight_blueprint - 0.15).abs() < f32::EPSILON);
        assert!((weights.weight_resonance - 0.10).abs() < f32::EPSILON);
        assert!((weights.weight_phase - 0.10).abs() < f32::EPSILON);
        assert!((weights.weight_karmic - 0.10).abs() < f32::EPSILON);

        let sum = weights.weight_archetype
            + weights.weight_spectrum
            + weights.weight_blueprint
            + weights.weight_resonance
            + weights.weight_phase
            + weights.weight_karmic;
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_connection_result_is_pod() {
        let result = ConnectionResult {
            strength: 0.5,
            archetype_similarity: 0.7,
            spectrum_similarity: 0.6,
            blueprint_alignment: 0.4,
            resonance_match: 0.8,
            phase_coherence: 0.3,
            karmic_correlation: 0.9,
            connection_type: 1,
        };

        let bytes = bytemuck::bytes_of(&result);
        assert_eq!(bytes.len(), 32);

        let recovered: &ConnectionResult = bytemuck::from_bytes(bytes);
        assert!((recovered.strength - 0.5).abs() < f32::EPSILON);
        assert_eq!(recovered.connection_type, 1);
    }

    #[test]
    fn test_connection_result_is_zeroable() {
        let zeroed: ConnectionResult = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.strength, 0.0);
        assert_eq!(zeroed.archetype_similarity, 0.0);
        assert_eq!(zeroed.connection_type, 0);
    }

    #[test]
    fn test_connection_result_size() {
        assert_eq!(std::mem::size_of::<ConnectionResult>(), 32);
        assert_eq!(std::mem::align_of::<ConnectionResult>(), 4);
    }

    #[test]
    fn test_dispatch_params_uniform_size() {
        assert_eq!(std::mem::size_of::<DispatchParamsUniform>(), 64);
    }

    #[test]
    fn test_output_count_and_size() {
        assert_eq!(ConnectionKernel::output_count(10), 100);
        assert_eq!(ConnectionKernel::output_size(10), 100 * 32);
        assert_eq!(ConnectionKernel::output_count(100), 10_000);
        assert_eq!(ConnectionKernel::output_size(100), 10_000 * 32);
    }

    #[test]
    fn test_weights_pod_safety() {
        let weights = ConnectionWeights::default();
        let bytes = bytemuck::bytes_of(&weights);
        assert_eq!(bytes.len(), std::mem::size_of::<ConnectionWeights>());

        let recovered: &ConnectionWeights = bytemuck::from_bytes(bytes);
        assert!((recovered.weight_archetype - 0.35).abs() < f32::EPSILON);
    }

    #[test]
    fn test_weights_zeroable() {
        let zeroed: ConnectionWeights = bytemuck::Zeroable::zeroed();
        assert_eq!(zeroed.weight_archetype, 0.0);
        assert_eq!(zeroed.weight_karmic, 0.0);
    }
}
