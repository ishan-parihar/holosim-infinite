//! Holographic Field Renderer - Volumetric Field Visualization
//!
//! From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.1:
//! "Design field visualization shader, Implement 3D volume rendering for field"
//!
//! This module provides:
//! - Raymarching-based volumetric field rendering
//! - Coherence-based coloring
//! - Phase coherence network visualization
//! - Interference pattern rendering

use crate::types::Float;
use wgpu::util::DeviceExt;
use wgpu::*;

use super::volume_texture::{FieldVolumeData, VolumeDimensions, VolumeFormat, VolumeTexture};

/// Holographic field renderer configuration
#[derive(Debug, Clone)]
pub struct HolographicFieldConfig {
    /// Volume texture dimensions
    pub volume_dimensions: VolumeDimensions,
    /// Volume texture format
    pub volume_format: VolumeFormat,
    /// Raymarching step count
    pub ray_steps: u32,
    /// Density scale for visibility
    pub density_scale: f32,
    /// Light absorption coefficient
    pub absorption: f32,
    /// Enable phase network rendering
    pub show_phase_network: bool,
    /// Enable interference pattern overlay
    pub show_interference: bool,
}

impl Default for HolographicFieldConfig {
    fn default() -> Self {
        Self {
            volume_dimensions: VolumeDimensions::cube(64),
            volume_format: VolumeFormat::Rgba16Float,
            ray_steps: 64,
            density_scale: 1.0,
            absorption: 0.5,
            show_phase_network: true,
            show_interference: true,
        }
    }
}

/// Holographic field renderer for volumetric visualization
pub struct HolographicFieldRenderer {
    /// Volume texture for field data
    volume_texture: VolumeTexture,
    /// Render pipeline for volumetric rendering
    pipeline: RenderPipeline,
    /// Bind group layout
    bind_group_layout: BindGroupLayout,
    /// Bind group for field texture
    bind_group: BindGroup,
    /// Uniform buffer for renderer settings
    uniform_buffer: Buffer,
    /// Configuration
    config: HolographicFieldConfig,
    /// Vertex buffer for fullscreen quad
    vertex_buffer: Buffer,
}

impl HolographicFieldRenderer {
    /// Create a new holographic field renderer
    pub fn new(
        device: &Device,
        surface_format: TextureFormat,
        config: HolographicFieldConfig,
    ) -> Self {
        // Create volume texture
        let volume_texture = VolumeTexture::new(
            device,
            config.volume_dimensions,
            config.volume_format,
            Some("holographic_field_volume"),
        );

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("holographic_field_bind_group_layout"),
            entries: &[
                // Volume texture
                VolumeTexture::bind_group_layout_entry(0, ShaderStages::FRAGMENT),
                // Sampler
                VolumeTexture::sampler_bind_group_layout_entry(1, ShaderStages::FRAGMENT),
                // Uniform buffer
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("holographic_field_uniforms"),
            size: std::mem::size_of::<FieldUniforms>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("holographic_field_bind_group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&volume_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&volume_texture.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        // Create shader module
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("holographic_field_shader"),
            source: ShaderSource::Wgsl(HOLOGRAPHIC_FIELD_SHADER.into()),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("holographic_field_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create fullscreen quad vertex buffer
        let vertices: [[f32; 4]; 6] = [
            // Position, UV
            [-1.0, -1.0, 0.0, 0.0], // Bottom-left
            [1.0, -1.0, 1.0, 0.0],  // Bottom-right
            [-1.0, 1.0, 0.0, 1.0],  // Top-left
            [1.0, -1.0, 1.0, 0.0],  // Bottom-right
            [1.0, 1.0, 1.0, 1.0],   // Top-right
            [-1.0, 1.0, 0.0, 1.0],  // Top-left
        ];

        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("holographic_field_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("holographic_field_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 4]>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            volume_texture,
            pipeline,
            bind_group_layout,
            bind_group,
            uniform_buffer,
            config,
            vertex_buffer,
        }
    }

    /// Update field data from FieldVolumeData
    pub fn update_field(&self, queue: &Queue, field_data: &FieldVolumeData) {
        // Upload to volume texture
        match self.config.volume_format {
            VolumeFormat::Rgba16Float => {
                let bytes = field_data.to_rgba16_bytes();
                self.volume_texture.upload(queue, &bytes);
            }
            VolumeFormat::Rgba8Unorm => {
                let bytes = field_data.to_rgba8_bytes();
                self.volume_texture.upload(queue, &bytes);
            }
            _ => {
                // Default to rgba16
                let bytes = field_data.to_rgba16_bytes();
                self.volume_texture.upload(queue, &bytes);
            }
        }
    }

    /// Set runtime quality profile for raymarching and optical response.
    pub fn set_quality_profile(&mut self, ray_steps: u32, density_scale: f32, absorption: f32) {
        self.config.ray_steps = ray_steps.clamp(16, 192);
        self.config.density_scale = density_scale.clamp(0.2, 4.0);
        self.config.absorption = absorption.clamp(0.05, 2.0);
    }

    /// Get current runtime quality profile.
    pub fn quality_profile(&self) -> (u32, f32, f32) {
        (
            self.config.ray_steps,
            self.config.density_scale,
            self.config.absorption,
        )
    }

    /// Update uniforms (camera, settings)
    pub fn update_uniforms(
        &self,
        queue: &Queue,
        view_projection: [[f32; 4]; 4],
        camera_position: [f32; 3],
        time: f32,
    ) {
        let uniforms = FieldUniforms {
            view_projection,
            inv_view_projection: inverse_matrix(view_projection),
            camera_position,
            camera_position_pad: 0.0,
            volume_dimensions: [
                self.config.volume_dimensions.width as f32,
                self.config.volume_dimensions.height as f32,
                self.config.volume_dimensions.depth as f32,
            ],
            volume_dimensions_pad: 0.0,
            ray_steps: self.config.ray_steps,
            density_scale: self.config.density_scale,
            absorption: self.config.absorption,
            time,
            _padding: [0.0; 3],
            _padding_end: 0.0,
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
    }

    /// Render the holographic field
    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    /// Get the bind group layout
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }

    /// Get the volume texture dimensions
    pub fn volume_dimensions(&self) -> &VolumeDimensions {
        &self.volume_texture.dimensions
    }
}

/// Uniforms for the holographic field shader
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct FieldUniforms {
    /// View-projection matrix
    view_projection: [[f32; 4]; 4],
    /// Inverse view-projection matrix
    inv_view_projection: [[f32; 4]; 4],
    /// Camera position in world space
    camera_position: [f32; 3],
    /// Padding for vec3 alignment
    camera_position_pad: f32,
    /// Volume texture dimensions
    volume_dimensions: [f32; 3],
    /// Padding for vec3 alignment
    volume_dimensions_pad: f32,
    /// Number of raymarching steps
    ray_steps: u32,
    /// Density scale for visibility
    density_scale: f32,
    /// Light absorption coefficient
    absorption: f32,
    /// Time for animation
    time: f32,
    /// Padding for alignment
    _padding: [f32; 3],
    /// Trailing padding to 16-byte boundary
    _padding_end: f32,
}

/// Compute inverse of a 4x4 matrix
fn inverse_matrix(m: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // Simplified inverse calculation for view-projection matrix
    let a00 = m[0][0];
    let a01 = m[0][1];
    let a02 = m[0][2];
    let a03 = m[0][3];
    let a10 = m[1][0];
    let a11 = m[1][1];
    let a12 = m[1][2];
    let a13 = m[1][3];
    let a20 = m[2][0];
    let a21 = m[2][1];
    let a22 = m[2][2];
    let a23 = m[2][3];
    let a30 = m[3][0];
    let a31 = m[3][1];
    let a32 = m[3][2];
    let a33 = m[3][3];

    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;

    let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

    if det.abs() < 1e-10 {
        return [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
    }

    let inv_det = 1.0 / det;

    [
        [
            (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
            (a02 * b10 - a01 * b11 - a03 * b09) * inv_det,
            (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
            (a22 * b04 - a21 * b05 - a23 * b03) * inv_det,
        ],
        [
            (a12 * b08 - a10 * b11 - a13 * b07) * inv_det,
            (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
            (a32 * b02 - a30 * b05 - a33 * b01) * inv_det,
            (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
        ],
        [
            (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
            (a01 * b08 - a00 * b10 - a03 * b06) * inv_det,
            (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
            (a21 * b02 - a20 * b04 - a23 * b00) * inv_det,
        ],
        [
            (a11 * b07 - a10 * b09 - a12 * b06) * inv_det,
            (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
            (a31 * b01 - a30 * b03 - a32 * b00) * inv_det,
            (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
        ],
    ]
}

/// Holographic field raymarching shader
const HOLOGRAPHIC_FIELD_SHADER: &str = r"
// Holographic Field Raymarching Shader
// From HOLOSIM_GUI_VISION_ROADMAP.md Phase B.1

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f,
}

struct FieldUniforms {
    view_projection: mat4x4f,
    inv_view_projection: mat4x4f,
    camera_position: vec3f,
    volume_dimensions: vec3f,
    ray_steps: u32,
    density_scale: f32,
    absorption: f32,
    time: f32,
    _padding: vec3f,
}

@group(0) @binding(0) var volume_texture: texture_3d<f32>;
@group(0) @binding(1) var volume_sampler: sampler;
@group(0) @binding(2) var<uniform> uniforms: FieldUniforms;

@vertex
fn vs_main(
    @location(0) position: vec2f,
    @location(1) uv: vec2f
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4f(position, 0.0, 1.0);
    output.uv = uv;
    return output;
}

// Sample the volume texture at a position
fn sample_volume(pos: vec3f) -> vec4f {
    // Normalize position to 0-1 range
    let uvw = (pos + 1.0) * 0.5;
    
    // Clamp to valid texture coordinates
    let clamped = clamp(uvw, vec3f(0.0), vec3f(1.0));
    
    return textureSample(volume_texture, volume_sampler, clamped);
}

// Coherence to color gradient (low=blue, mid=green, high=orange/gold)
fn coherence_to_color(coherence: f32) -> vec3f {
    // Color gradient inspired by cosmological architecture
    // Low coherence: deep blue/violet (chaos)
    // Mid coherence: green/cyan (emerging order)
    // High coherence: gold/orange (unity)
    
    let low_color = vec3f(0.2, 0.0, 0.4);   // Deep violet
    let mid_color = vec3f(0.0, 0.6, 0.5);   // Cyan
    let high_color = vec3f(1.0, 0.8, 0.2);  // Golden
    
    if coherence < 0.5 {
        return mix(low_color, mid_color, coherence * 2.0);
    } else {
        return mix(mid_color, high_color, (coherence - 0.5) * 2.0);
    }
}

// Phase to color for visualization
fn phase_to_color(phase: f32) -> vec3f {
    // Map phase (0-1) to hue
    let hue = phase * 6.0;
    
    let r = abs(hue - 3.0) - 1.0;
    let g = 2.0 - abs(hue - 2.0);
    let b = 2.0 - abs(hue - 4.0);
    
    return clamp(vec3f(r, g, b), vec3f(0.0), vec3f(1.0));
}

// Ray-box intersection
fn intersect_box(
    ray_origin: vec3f,
    ray_dir: vec3f,
    box_min: vec3f,
    box_max: vec3f
) -> vec2f {
    let t_min = (box_min - ray_origin) / ray_dir;
    let t_max = (box_max - ray_origin) / ray_dir;
    
    let t1 = min(t_min, t_max);
    let t2 = max(t_min, t_max);
    
    let t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);
    
    return vec2f(t_near, t_far);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    // Get ray direction from camera through this pixel
    let ndc = input.uv * 2.0 - 1.0;
    
    // Transform to world space
    let clip_pos = vec4f(ndc, -1.0, 1.0);
    let view_pos = uniforms.inv_view_projection * clip_pos;
    let world_pos = view_pos.xyz / view_pos.w;
    
    let ray_origin = uniforms.camera_position;
    let ray_dir = normalize(world_pos - ray_origin);
    
    // Intersect with volume bounding box (-1 to 1)
    let t_range = intersect_box(ray_origin, ray_dir, vec3f(-1.0), vec3f(1.0));
    
    // Check if we hit the volume
    if t_range.x > t_range.y {
        return vec4f(0.0);  // No intersection
    }
    
    // Clamp to positive t values
    let t_start = max(t_range.x, 0.0);
    let t_end = t_range.y;
    
    // Raymarching
    let num_steps = uniforms.ray_steps;
    let step_size = (t_end - t_start) / f32(num_steps);
    
    var accumulated_color = vec3f(0.0);
    var accumulated_alpha = 0.0;
    
    var t = t_start;
    for (var i = 0u; i < num_steps; i++) {
        let pos = ray_origin + ray_dir * t;
        
        // Sample volume
        let sample = sample_volume(pos);
        let coherence = sample.r;  // R = coherence
        let amplitude = sample.g;  // G = amplitude
        let phase = sample.b;      // B = phase (normalized)
        let density = sample.a;    // A = density band
        
        // Calculate density at this point
        let density_val = coherence * amplitude * uniforms.density_scale;
        
        if density_val > 0.01 {
            // Get color based on coherence
            let base_color = coherence_to_color(coherence);
            
            // Add phase modulation for visual interest
            let phase_mod = phase_to_color(phase) * 0.2;
            let color = base_color + phase_mod;
            
            // Calculate alpha with absorption
            let alpha = density_val * exp(-accumulated_alpha * uniforms.absorption);
            
            // Accumulate
            accumulated_color += color * alpha * (1.0 - accumulated_alpha);
            accumulated_alpha += alpha * (1.0 - accumulated_alpha);
        }
        
        t += step_size;
        
        // Early exit if fully opaque
        if accumulated_alpha > 0.99 {
            break;
        }
    }
    
    // Add subtle glow effect at high coherence regions
    let glow = smoothstep(0.7, 1.0, accumulated_alpha) * 0.3;
    accumulated_color += vec3f(glow);
    
    return vec4f(accumulated_color, accumulated_alpha);
}
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_config_default() {
        let config = HolographicFieldConfig::default();
        assert_eq!(config.ray_steps, 64);
        assert!(config.show_phase_network);
    }

    #[test]
    fn test_inverse_matrix() {
        let identity = [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let inv = inverse_matrix(identity);

        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((inv[i][j] - expected).abs() < 0.0001);
            }
        }
    }
}
