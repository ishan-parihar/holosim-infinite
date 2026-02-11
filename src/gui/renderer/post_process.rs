//! Post-Processing Pipeline - Bloom, Glow, and Visual Effects
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 4 Week 9:
//! "Post-processing pipeline: Bloom effect for high-consciousness entities,
//!  Glow for emergence events, Spectrum color grading"
//!
//! This module provides:
//! - Multi-pass post-processing pipeline
//! - Bloom effect with configurable intensity
//! - Glow for high-consciousness entities and emergence events
//! - Spectrum-based color grading
//! - Framebuffer management for off-screen rendering

use crate::gui::renderer::WgpuContext;

/// Post-processing pipeline configuration
#[derive(Debug, Clone, Copy)]
pub struct PostProcessConfig {
    /// Enable bloom effect
    pub bloom_enabled: bool,
    /// Bloom intensity (0.0 to 2.0)
    pub bloom_intensity: f32,
    /// Bloom threshold (pixels brighter than this contribute to bloom)
    pub bloom_threshold: f32,
    /// Number of bloom blur passes
    pub bloom_passes: u32,
    /// Enable glow effect
    pub glow_enabled: bool,
    /// Glow intensity for high-consciousness entities
    pub glow_intensity: f32,
    /// Enable spectrum color grading
    pub spectrum_grading_enabled: bool,
    /// Color grading intensity
    pub grading_intensity: f32,
}

impl Default for PostProcessConfig {
    fn default() -> Self {
        PostProcessConfig {
            bloom_enabled: true,
            bloom_intensity: 0.8,
            bloom_threshold: 0.7,
            bloom_passes: 4,
            glow_enabled: true,
            glow_intensity: 1.2,
            spectrum_grading_enabled: true,
            grading_intensity: 0.3,
        }
    }
}

impl PostProcessConfig {
    /// Create default configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable bloom
    pub fn with_bloom(mut self, enabled: bool) -> Self {
        self.bloom_enabled = enabled;
        self
    }

    /// Set bloom intensity
    pub fn with_bloom_intensity(mut self, intensity: f32) -> Self {
        self.bloom_intensity = intensity.clamp(0.0, 2.0);
        self
    }

    /// Set bloom threshold
    pub fn with_bloom_threshold(mut self, threshold: f32) -> Self {
        self.bloom_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Enable or disable glow
    pub fn with_glow(mut self, enabled: bool) -> Self {
        self.glow_enabled = enabled;
        self
    }

    /// Set glow intensity
    pub fn with_glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 3.0);
        self
    }

    /// Enable or disable spectrum color grading
    pub fn with_spectrum_grading(mut self, enabled: bool) -> Self {
        self.spectrum_grading_enabled = enabled;
        self
    }
}

/// Post-processing pipeline state
#[derive(Debug)]
pub struct PostProcessPipeline {
    /// Configuration
    pub config: PostProcessConfig,
    /// Screen width
    pub width: u32,
    /// Screen height
    pub height: u32,
    /// Bloom extraction texture
    pub bloom_texture: Option<wgpu::Texture>,
    /// Bloom blur textures (ping-pong)
    pub bloom_textures: Vec<wgpu::Texture>,
    /// Glow overlay texture
    pub glow_texture: Option<wgpu::Texture>,
    /// Output texture
    pub output_texture: Option<wgpu::Texture>,
    /// Post-process bind group layout
    pub bind_group_layout: Option<wgpu::BindGroupLayout>,
    /// Post-process pipeline
    pub pipeline: Option<wgpu::RenderPipeline>,
    /// Uniform buffer for post-process settings
    pub uniform_buffer: Option<wgpu::Buffer>,
}

impl PostProcessPipeline {
    /// Create a new post-processing pipeline
    pub fn new(config: PostProcessConfig, width: u32, height: u32) -> Self {
        PostProcessPipeline {
            config,
            width,
            height,
            bloom_texture: None,
            bloom_textures: Vec::new(),
            glow_texture: None,
            output_texture: None,
            bind_group_layout: None,
            pipeline: None,
            uniform_buffer: None,
        }
    }

    /// Initialize the pipeline with WGPU context
    pub fn initialize(&mut self, context: &WgpuContext) {
        // Create textures
        self.create_textures(context);

        // Create bind group layout
        self.create_bind_group_layout(context);

        // Create render pipeline
        self.create_pipeline(context);

        // Create uniform buffer
        self.create_uniform_buffer(context);
    }

    /// Create all required textures
    fn create_textures(&mut self, context: &WgpuContext) {
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Post-Process Texture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        // Create bloom texture
        self.bloom_texture = Some(context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Bloom Texture"),
            ..texture_desc
        }));

        // Create ping-pong blur textures
        self.bloom_textures.clear();
        for i in 0..2 {
            self.bloom_textures
                .push(context.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some(&format!("Bloom Blur Texture {}", i)),
                    ..texture_desc
                }));
        }

        // Create glow texture
        self.glow_texture = Some(context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glow Texture"),
            ..texture_desc
        }));

        // Create output texture
        self.output_texture = Some(context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Post-Process Output"),
            ..texture_desc
        }));
    }

    /// Create bind group layout for post-processing
    fn create_bind_group_layout(&mut self, context: &WgpuContext) {
        self.bind_group_layout = Some(context.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Post-Process Bind Group Layout"),
                entries: &[
                    // Scene texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    // Bloom texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    // Glow texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    // Sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Uniforms
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            },
        ));
    }

    /// Create render pipeline
    fn create_pipeline(&mut self, context: &WgpuContext) {
        let shader = context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Post-Process Shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(POST_PROCESS_SHADER)),
            });

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Post-Process Pipeline Layout"),
                    bind_group_layouts: &[self.bind_group_layout.as_ref().unwrap()],
                    push_constant_ranges: &[],
                });

        self.pipeline = Some(context.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Post-Process Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba16Float,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            },
        ));
    }

    /// Create uniform buffer
    fn create_uniform_buffer(&mut self, context: &WgpuContext) {
        let uniforms = PostProcessUniforms {
            bloom_intensity: self.config.bloom_intensity,
            bloom_threshold: self.config.bloom_threshold,
            glow_intensity: self.config.glow_intensity,
            grading_intensity: self.config.grading_intensity,
            _padding: [0.0; 4],
        };

        let buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Post-Process Uniforms"),
            size: std::mem::size_of::<PostProcessUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        context
            .queue
            .write_buffer(&buffer, 0, bytemuck::cast_slice(&[uniforms]));
        self.uniform_buffer = Some(buffer);
    }

    /// Update uniform buffer with current settings
    pub fn update_uniforms(&self, queue: &wgpu::Queue) {
        if let Some(buffer) = &self.uniform_buffer {
            let uniforms = PostProcessUniforms {
                bloom_intensity: self.config.bloom_intensity,
                bloom_threshold: self.config.bloom_threshold,
                glow_intensity: self.config.glow_intensity,
                grading_intensity: self.config.grading_intensity,
                _padding: [0.0; 4],
            };
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }
    }

    /// Resize the pipeline
    pub fn resize(&mut self, context: &WgpuContext, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.create_textures(context);
    }

    /// Perform post-processing
    pub fn process(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        scene_view: &wgpu::TextureView,
        output_view: &wgpu::TextureView,
        device: &wgpu::Device,
    ) {
        if !self.config.bloom_enabled
            && !self.config.glow_enabled
            && !self.config.spectrum_grading_enabled
        {
            // No post-processing needed, just copy
            return;
        }

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Post-Process Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Post-Process Bind Group"),
            layout: self.bind_group_layout.as_ref().unwrap(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(scene_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &self
                            .bloom_texture
                            .as_ref()
                            .unwrap()
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(
                        &self
                            .glow_texture
                            .as_ref()
                            .unwrap()
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: self.uniform_buffer.as_ref().unwrap().as_entire_binding(),
                },
            ],
        });

        // Render post-processing
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Post-Process Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(self.pipeline.as_ref().unwrap());
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1); // Full-screen triangle
    }
}

/// Post-processing uniform data
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PostProcessUniforms {
    pub bloom_intensity: f32,
    pub bloom_threshold: f32,
    pub glow_intensity: f32,
    pub grading_intensity: f32,
    pub _padding: [f32; 4],
}

/// Post-processing shader (WGSL)
pub const POST_PROCESS_SHADER: &str = r#"
struct PostProcessUniforms {
    bloom_intensity: f32,
    bloom_threshold: f32,
    glow_intensity: f32,
    grading_intensity: f32,
};

@group(0) @binding(0)
var scene_texture: texture_2d<f32>;

@group(0) @binding(1)
var bloom_texture: texture_2d<f32>;

@group(0) @binding(2)
var glow_texture: texture_2d<f32>;

@group(0) @binding(3)
var texture_sampler: sampler;

@group(0) @binding(4)
var<uniform> uniforms: PostProcessUniforms;

// Full-screen triangle vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Generate full-screen triangle
    let x = f32(vertex_index % 2u) * 4.0 - 1.0; // 0 -> -1, 1 -> 3
    let y = f32(vertex_index / 2u) * 4.0 - 1.0; // 0 -> -1, 1 -> 3
    return vec4<f32>(x, y, 0.0, 1.0);
}

// Post-process fragment shader
@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(textureDimensions(scene_texture));
    
    // Sample scene
    let scene_color = textureSample(scene_texture, texture_sampler, uv);
    
    // Sample bloom
    let bloom_color = textureSample(bloom_texture, texture_sampler, uv);
    
    // Sample glow
    let glow_color = textureSample(glow_texture, texture_sampler, uv);
    
    // Apply bloom
    var final_color = scene_color.rgb;
    if (uniforms.bloom_intensity > 0.0) {
        final_color = final_color + bloom_color.rgb * uniforms.bloom_intensity;
    }
    
    // Apply glow
    if (uniforms.glow_intensity > 0.0) {
        final_color = final_color + glow_color.rgb * uniforms.glow_intensity;
    }
    
    // Apply spectrum color grading (warm/cool tint based on position)
    if (uniforms.grading_intensity > 0.0) {
        let grading = spectrum_color_grading(uv, uniforms.grading_intensity);
        final_color = mix(final_color, final_color * grading, uniforms.grading_intensity);
    }
    
    // Tone mapping (simple Reinhard)
    final_color = final_color / (final_color + vec3<f32>(1.0));
    
    // Gamma correction
    final_color = pow(final_color, vec3<f32>(1.0 / 2.2));
    
    return vec4<f32>(final_color, scene_color.a);
}

// Spectrum color grading function
fn spectrum_color_grading(uv: vec2<f32>, intensity: f32) -> vec3<f32> {
    // Create a subtle gradient from bottom-left (warm/orange) to top-right (cool/blue)
    // This represents the space/time to time/space spectrum
    let warm = vec3<f32>(1.0, 0.8, 0.6);   // Space/Time - warm
    let cool = vec3<f32>(0.6, 0.7, 1.0);   // Time/Space - cool
    
    let t = (uv.x + uv.y) * 0.5;
    return mix(warm, cool, t);
}

// Extract bright areas for bloom
fn extract_bright_areas(color: vec3<f32>, threshold: f32) -> vec3<f32> {
    let luminance = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    let contribution = max(luminance - threshold, 0.0);
    return color * contribution;
}
"#;

/// Visual effects manager
#[derive(Debug)]
pub struct VisualEffects {
    /// Active effects
    pub effects: Vec<ActiveEffect>,
    /// Particle system state
    pub particles: ParticleSystem,
}

impl VisualEffects {
    /// Create new visual effects manager
    pub fn new() -> Self {
        VisualEffects {
            effects: Vec::new(),
            particles: ParticleSystem::new(),
        }
    }

    /// Add an emergence glow effect
    pub fn add_emergence_glow(&mut self, position: [f32; 3], intensity: f32, duration: f32) {
        self.effects.push(ActiveEffect {
            effect_type: EffectType::EmergenceGlow,
            position,
            intensity,
            duration,
            elapsed: 0.0,
        });
    }

    /// Add a catalyst shockwave
    pub fn add_shockwave(&mut self, position: [f32; 3], intensity: f32, duration: f32) {
        self.effects.push(ActiveEffect {
            effect_type: EffectType::Shockwave,
            position,
            intensity,
            duration,
            elapsed: 0.0,
        });
    }

    /// Add a density transition effect
    pub fn add_density_transition(&mut self, position: [f32; 3], from_density: u8, to_density: u8) {
        self.effects.push(ActiveEffect {
            effect_type: EffectType::DensityTransition {
                from_density,
                to_density,
            },
            position,
            intensity: 1.0,
            duration: 2.0,
            elapsed: 0.0,
        });
    }

    /// Update all effects
    pub fn update(&mut self, delta_time: f32) {
        // Update effect timers
        for effect in &mut self.effects {
            effect.elapsed += delta_time;
        }

        // Remove completed effects
        self.effects.retain(|e| e.elapsed < e.duration);

        // Update particles
        self.particles.update(delta_time);
    }

    /// Get total glow contribution at a position
    pub fn get_glow_at(&self, position: [f32; 3]) -> f32 {
        let mut total_glow = 0.0;

        for effect in &self.effects {
            let distance = ((effect.position[0] - position[0]).powi(2)
                + (effect.position[1] - position[1]).powi(2)
                + (effect.position[2] - position[2]).powi(2))
            .sqrt();

            let progress = effect.elapsed / effect.duration;
            let falloff = 1.0 - (distance / 100.0).min(1.0);
            let time_falloff = 1.0 - progress;

            match effect.effect_type {
                EffectType::EmergenceGlow => {
                    total_glow += effect.intensity * falloff * time_falloff * 0.5;
                }
                EffectType::Shockwave => {
                    total_glow += effect.intensity * falloff * time_falloff * 0.3;
                }
                _ => {}
            }
        }

        total_glow.min(1.0)
    }
}

impl Default for VisualEffects {
    fn default() -> Self {
        Self::new()
    }
}

/// Active visual effect
#[derive(Debug, Clone)]
pub struct ActiveEffect {
    /// Type of effect
    pub effect_type: EffectType,
    /// Position in world space
    pub position: [f32; 3],
    /// Effect intensity
    pub intensity: f32,
    /// Total duration in seconds
    pub duration: f32,
    /// Elapsed time in seconds
    pub elapsed: f32,
}

/// Type of visual effect
#[derive(Debug, Clone)]
pub enum EffectType {
    /// Emergence glow for new consciousness
    EmergenceGlow,
    /// Shockwave from catalyst events
    Shockwave,
    /// Density transition effect
    DensityTransition { from_density: u8, to_density: u8 },
    /// Trail effect for moving entities
    Trail,
}

/// Simple particle system
#[derive(Debug)]
pub struct ParticleSystem {
    /// Maximum number of particles
    pub max_particles: usize,
    /// Active particles
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    /// Create new particle system
    pub fn new() -> Self {
        ParticleSystem {
            max_particles: 10000,
            particles: Vec::with_capacity(10000),
        }
    }

    /// Spawn a particle
    pub fn spawn(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        color: [f32; 4],
        lifetime: f32,
    ) {
        if self.particles.len() < self.max_particles {
            self.particles.push(Particle {
                position,
                velocity,
                color,
                lifetime,
                age: 0.0,
            });
        }
    }

    /// Spawn emergence particles
    pub fn spawn_emergence(&mut self, position: [f32; 3], count: u32) {
        for _ in 0..count {
            let velocity = [
                (rand::random::<f32>() - 0.5) * 2.0,
                (rand::random::<f32>() - 0.5) * 2.0,
                (rand::random::<f32>() - 0.5) * 2.0,
            ];
            let color = [1.0, 0.9, 0.5, 1.0]; // Golden emergence
            self.spawn(position, velocity, color, 2.0 + rand::random::<f32>() * 2.0);
        }
    }

    /// Update all particles
    pub fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.age += delta_time;
            particle.position[0] += particle.velocity[0] * delta_time;
            particle.position[1] += particle.velocity[1] * delta_time;
            particle.position[2] += particle.velocity[2] * delta_time;
        }

        // Remove dead particles
        self.particles.retain(|p| p.age < p.lifetime);
    }

    /// Get particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual particle
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    /// Position in world space
    pub position: [f32; 3],
    /// Velocity
    pub velocity: [f32; 3],
    /// Color (RGBA)
    pub color: [f32; 4],
    /// Maximum lifetime in seconds
    pub lifetime: f32,
    /// Current age in seconds
    pub age: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_process_config() {
        let config = PostProcessConfig::new()
            .with_bloom(true)
            .with_bloom_intensity(1.0)
            .with_glow(true)
            .with_glow_intensity(1.5);

        assert!(config.bloom_enabled);
        assert_eq!(config.bloom_intensity, 1.0);
        assert!(config.glow_enabled);
        assert_eq!(config.glow_intensity, 1.5);
    }

    #[test]
    fn test_visual_effects() {
        let mut effects = VisualEffects::new();

        effects.add_emergence_glow([0.0, 0.0, 0.0], 1.0, 2.0);
        effects.add_shockwave([10.0, 0.0, 0.0], 0.8, 1.5);

        assert_eq!(effects.effects.len(), 2);

        // Update
        effects.update(0.5);
        assert_eq!(effects.effects.len(), 2);

        // Update past duration
        effects.update(2.0);
        assert_eq!(effects.effects.len(), 0);
    }

    #[test]
    fn test_particle_system() {
        let mut particles = ParticleSystem::new();

        particles.spawn([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0], 2.0);
        assert_eq!(particles.particle_count(), 1);

        particles.update(1.0);
        assert_eq!(particles.particle_count(), 1);

        particles.update(1.1);
        assert_eq!(particles.particle_count(), 0);
    }
}
