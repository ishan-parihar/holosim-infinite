//! Cosmos Renderer - Renders cosmic structures at multiple scales
//!
//! From HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md Phase 3:
//! "Visualizes universe, galaxies, stars, planets at multiple scales"
//!
//! This renderer provides:
//! - Star rendering with luminosity-based glow
//! - Planet rendering with orbital positions
//! - Orbital path visualization
//! - Cosmic filament lines
//!
//! CONNECTS TO:
//! - CosmosEngine (src/cosmos/cosmos_engine.rs)
//! - StellarPhysics (src/cosmos/stellar_physics.rs)
//! - PlanetaryFormation (src/cosmos/planetary_formation.rs)

use wgpu::util::DeviceExt;

// ============================================================================
// Vertex Data Structures
// ============================================================================

/// Star vertex data for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StarVertex {
    /// Position in world space
    pub position: [f32; 3],
    /// Star color (RGB, derived from spectral class)
    pub color: [f32; 3],
    /// Luminosity relative to Sun
    pub luminosity: f32,
    /// Temperature in Kelvin
    pub temperature: f32,
    /// Radius relative to Sun
    pub radius: f32,
    /// Evolution stage
    pub stage: u32,
    /// Padding for alignment
    pub _padding: [f32; 2],
}

/// Planet vertex data for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlanetVertex {
    /// Position in world space
    pub position: [f32; 3],
    /// Normal for lighting
    pub normal: [f32; 3],
    /// UV coordinates
    pub uv: [f32; 2],
    /// Planet type
    pub planet_type: u32,
    /// Orbital radius in AU
    pub orbital_radius: f32,
    /// Planet radius relative to Earth
    pub radius: f32,
    /// Surface temperature
    pub temperature: f32,
    /// Padding
    pub _padding: [f32; 1],
}

/// Orbit vertex data for rendering orbital paths
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OrbitVertex {
    /// Position on orbital ellipse
    pub position: [f32; 3],
    /// Color for the orbit line
    pub color: [f32; 3],
    /// Alpha for fading
    pub alpha: f32,
    /// Padding
    pub _padding: [f32; 1],
}

/// Filament vertex data for rendering cosmic web
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FilamentVertex {
    /// Position on filament line
    pub position: [f32; 3],
    /// Color based on density
    pub color: [f32; 3],
    /// Density contrast
    pub density: f32,
    /// Padding
    pub _padding: [f32; 1],
}

/// Camera uniform for cosmos rendering
/// Layout matches WGSL: view_proj (64) + camera_position (12) + time (4) = 80 bytes
/// WGSL vec3 has 16-byte alignment, so camera_position starts at 64, time at 80
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CosmosCameraUniform {
    /// View-projection matrix (64 bytes)
    pub view_proj: [[f32; 4]; 4],
    /// Camera position (12 bytes) + time (4 bytes) packed together
    pub camera_position: [f32; 3],
    pub time: f32,
}

// ============================================================================
// Cosmos Renderer
// ============================================================================

/// Renderer for cosmic structures
pub struct CosmosRenderer {
    // Pipeline for stars (point sprites with glow)
    star_pipeline: wgpu::RenderPipeline,
    star_buffer: wgpu::Buffer,
    star_count: u32,

    // Pipeline for planets (spheres)
    planet_pipeline: wgpu::RenderPipeline,
    planet_buffer: wgpu::Buffer,
    planet_count: u32,

    // Pipeline for orbital paths (lines)
    orbit_pipeline: wgpu::RenderPipeline,
    orbit_buffer: wgpu::Buffer,
    orbit_vertex_count: u32,

    // Pipeline for filaments (lines)
    filament_pipeline: wgpu::RenderPipeline,
    filament_buffer: wgpu::Buffer,
    filament_vertex_count: u32,

    // Camera uniform
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    // Maximum capacities
    max_stars: usize,
    max_planets: usize,
    max_orbit_vertices: usize,
    max_filament_vertices: usize,
}

impl CosmosRenderer {
    /// Create a new cosmos renderer
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        // Load cosmos shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cosmos Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/cosmos.wgsl").into()),
        });

        // Create camera bind group layout
        let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cosmos Camera Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<CosmosCameraUniform>() as u64),
                },
                count: None,
            }],
        });

        // Create camera buffer
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cosmos Camera Buffer"),
            size: std::mem::size_of::<CosmosCameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create camera bind group
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cosmos Camera Bind Group"),
            layout: &camera_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Create pipelines
        let star_pipeline = Self::create_star_pipeline(device, &camera_layout, &shader, surface_config);
        let planet_pipeline = Self::create_planet_pipeline(device, &camera_layout, &shader, surface_config);
        let orbit_pipeline = Self::create_orbit_pipeline(device, &camera_layout, &shader, surface_config);
        let filament_pipeline = Self::create_filament_pipeline(device, &camera_layout, &shader, surface_config);

        // Create buffers with capacities
        let max_stars = 1000;
        let max_planets = 500;
        let max_orbit_vertices = 10000;
        let max_filament_vertices = 10000;

        let star_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Star Vertex Buffer"),
            size: (std::mem::size_of::<StarVertex>() * max_stars) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let planet_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Planet Vertex Buffer"),
            size: (std::mem::size_of::<PlanetVertex>() * max_planets) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let orbit_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Orbit Vertex Buffer"),
            size: (std::mem::size_of::<OrbitVertex>() * max_orbit_vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let filament_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Filament Vertex Buffer"),
            size: (std::mem::size_of::<FilamentVertex>() * max_filament_vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            star_pipeline,
            star_buffer,
            star_count: 0,
            planet_pipeline,
            planet_buffer,
            planet_count: 0,
            orbit_pipeline,
            orbit_buffer,
            orbit_vertex_count: 0,
            filament_pipeline,
            filament_buffer,
            filament_vertex_count: 0,
            camera_buffer,
            camera_bind_group,
            max_stars,
            max_planets,
            max_orbit_vertices,
            max_filament_vertices,
        }
    }

    /// Update camera uniform
    pub fn update_camera(&self, queue: &wgpu::Queue, view_proj: [[f32; 4]; 4], camera_position: [f32; 3], time: f32) {
        let uniform = CosmosCameraUniform {
            view_proj,
            camera_position,
            time,
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Update star data
    pub fn update_stars(&mut self, queue: &wgpu::Queue, stars: &[StarVertex]) {
        let count = stars.len().min(self.max_stars);
        if count > 0 {
            queue.write_buffer(&self.star_buffer, 0, bytemuck::cast_slice(&stars[..count]));
            self.star_count = count as u32;
        }
    }

    /// Update planet data
    pub fn update_planets(&mut self, queue: &wgpu::Queue, planets: &[PlanetVertex]) {
        let count = planets.len().min(self.max_planets);
        if count > 0 {
            queue.write_buffer(&self.planet_buffer, 0, bytemuck::cast_slice(&planets[..count]));
            self.planet_count = count as u32;
        }
    }

    /// Update orbit path data
    pub fn update_orbits(&mut self, queue: &wgpu::Queue, vertices: &[OrbitVertex]) {
        let count = vertices.len().min(self.max_orbit_vertices);
        if count > 0 {
            queue.write_buffer(&self.orbit_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.orbit_vertex_count = count as u32;
        }
    }

    /// Update filament data
    pub fn update_filaments(&mut self, queue: &wgpu::Queue, vertices: &[FilamentVertex]) {
        let count = vertices.len().min(self.max_filament_vertices);
        if count > 0 {
            queue.write_buffer(&self.filament_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.filament_vertex_count = count as u32;
        }
    }

    /// Render all cosmic structures into an existing render pass
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // Render filaments first (background layer)
        if self.filament_vertex_count > 0 {
            render_pass.set_pipeline(&self.filament_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.filament_buffer.slice(..));
            render_pass.draw(0..self.filament_vertex_count, 0..1);
        }

        // Render orbital paths
        if self.orbit_vertex_count > 0 {
            render_pass.set_pipeline(&self.orbit_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.orbit_buffer.slice(..));
            render_pass.draw(0..self.orbit_vertex_count, 0..1);
        }

        // Render planets
        if self.planet_count > 0 {
            render_pass.set_pipeline(&self.planet_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.planet_buffer.slice(..));
            render_pass.draw(0..36, 0..self.planet_count);
        }

        // Render stars (on top, with additive blending)
        if self.star_count > 0 {
            render_pass.set_pipeline(&self.star_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.star_buffer.slice(..));
            render_pass.draw(0..4, 0..self.star_count);
        }
    }

    // ========================================================================
    // Pipeline Creation Methods
    // ========================================================================

    fn create_star_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Star Pipeline Layout"),
            bind_group_layouts: &[layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Star Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_star",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<StarVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 24, shader_location: 2, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 28, shader_location: 3, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 32, shader_location: 4, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 36, shader_location: 5, format: wgpu::VertexFormat::Uint32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_star",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_planet_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Planet Pipeline Layout"),
            bind_group_layouts: &[layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Planet Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_planet",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<PlanetVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 24, shader_location: 2, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Uint32 },
                        wgpu::VertexAttribute { offset: 36, shader_location: 4, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 40, shader_location: 5, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 44, shader_location: 6, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_planet",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_orbit_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Orbit Pipeline Layout"),
            bind_group_layouts: &[layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Orbit Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_orbit",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<OrbitVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 24, shader_location: 2, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_orbit",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_filament_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Filament Pipeline Layout"),
            bind_group_layouts: &[layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Filament Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_filament",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<FilamentVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 24, shader_location: 2, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_filament",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }
}
