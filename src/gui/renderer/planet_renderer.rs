//! Planet Renderer - Renders planetary surfaces with dynamic systems
//!
//! From HOLOSIM_VISUALIZATION_INTEGRATION_ROADMAP.md Phase 4:
//! "Visualizes the four planetary subsystems:
//! - Lithosphere: Tectonic plates, volcanoes, terrain, earthquakes
//! - Hydrosphere: Oceans, rivers, lakes, glaciers, water cycle
//! - Atmosphere: Clouds, storms, weather fronts, wind
//! - EnergyFlow: Solar radiation, day/night, seasons"
//!
//! CONNECTS TO:
//! - Lithosphere (src/planet/lithosphere.rs)
//! - Hydrosphere (src/planet/hydrosphere.rs)
//! - DynamicAtmosphere (src/planet/atmosphere.rs)
//! - EnergyFlowSystem (src/planet/energy_flow.rs)

use wgpu::util::DeviceExt;

// ============================================================================
// Vertex Data Structures
// ============================================================================

/// Terrain vertex data for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TerrainVertex {
    /// Position on sphere
    pub position: [f32; 3],
    /// Surface normal
    pub normal: [f32; 3],
    /// UV coordinates
    pub uv: [f32; 2],
    /// Tectonic plate ID
    pub plate_id: f32,
    /// Volcanic activity level
    pub volcanic_activity: f32,
    /// Padding for alignment
    pub _padding: [f32; 2],
}

/// Water vertex data for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WaterVertex {
    /// Position on sphere
    pub position: [f32; 3],
    /// Surface normal
    pub normal: [f32; 3],
    /// UV coordinates
    pub uv: [f32; 2],
    /// Water depth
    pub depth: f32,
    /// Padding
    pub _padding: [f32; 1],
}

/// Cloud vertex data for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CloudVertex {
    /// Position on sphere
    pub position: [f32; 3],
    /// UV coordinates
    pub uv: [f32; 2],
    /// Cloud density
    pub density: f32,
    /// Padding
    pub _padding: [f32; 2],
}

/// Storm vertex data for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct StormVertex {
    /// Position on sphere surface
    pub position: [f32; 3],
    /// Storm size
    pub size: f32,
    /// Storm intensity
    pub intensity: f32,
    /// Storm rotation
    pub rotation: f32,
    /// Padding
    pub _padding: [f32; 2],
}

/// Settlement vertex data for rendering (Phase 6 preview)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SettlementVertex {
    /// Position on planet surface
    pub position: [f32; 3],
    /// Settlement size
    pub size: f32,
    /// Technology level
    pub tech_level: f32,
    /// Settlement type
    pub settlement_type: f32,
    /// Padding
    pub _padding: [f32; 2],
}

/// Planet uniform for rendering
/// Size: 176 bytes (aligned to 16 for WGSL)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlanetUniform {
    /// View-projection matrix (64 bytes, offset 0)
    pub view_proj: [[f32; 4]; 4],
    /// Planet rotation matrix (64 bytes, offset 64)
    pub rotation: [[f32; 4]; 4],
    /// Time for animations (4 bytes, offset 128)
    pub time: f32,
    /// Axial tilt in radians (4 bytes, offset 132)
    pub axial_tilt: f32,
    /// Angle to star for day/night (4 bytes, offset 136)
    pub solar_angle: f32,
    /// Planet radius (4 bytes, offset 140)
    pub radius: f32,
    /// Camera position (12 bytes, offset 144 - WGSL vec3 has 16-byte alignment)
    pub camera_pos: [f32; 3],
    /// Scale level (4 bytes, offset 156)
    pub scale_level: u32,
    /// Padding (8 bytes, offset 160)
    pub _padding: [f32; 2],
    /// Final padding for 16-byte struct alignment (4 bytes, offset 168)
    pub _final_pad: [f32; 2],
}

// ============================================================================
// Planet Renderer
// ============================================================================

/// Renderer for planetary surfaces
pub struct PlanetRenderer {
    // Terrain pipeline
    terrain_pipeline: wgpu::RenderPipeline,
    terrain_buffer: wgpu::Buffer,
    terrain_count: u32,

    // Water pipeline
    water_pipeline: wgpu::RenderPipeline,
    water_buffer: wgpu::Buffer,
    water_count: u32,

    // Cloud pipeline
    cloud_pipeline: wgpu::RenderPipeline,
    cloud_buffer: wgpu::Buffer,
    cloud_count: u32,

    // Atmosphere pipeline
    atmosphere_pipeline: wgpu::RenderPipeline,

    // Storm pipeline
    storm_pipeline: wgpu::RenderPipeline,
    storm_buffer: wgpu::Buffer,
    storm_count: u32,

    // Settlement pipeline (Phase 6)
    settlement_pipeline: wgpu::RenderPipeline,
    settlement_buffer: wgpu::Buffer,
    settlement_count: u32,

    // Camera/planet uniform
    planet_buffer: wgpu::Buffer,
    planet_bind_group: wgpu::BindGroup,

    // Textures
    terrain_texture: wgpu::Texture,
    terrain_texture_view: wgpu::TextureView,
    cloud_texture: wgpu::Texture,
    cloud_texture_view: wgpu::TextureView,

    // Texture bind groups
    terrain_texture_bind_group: wgpu::BindGroup,
    cloud_texture_bind_group: wgpu::BindGroup,

    // Maximum capacities
    max_terrain_vertices: usize,
    max_water_vertices: usize,
    max_cloud_vertices: usize,
    max_storms: usize,
    max_settlements: usize,
}

impl PlanetRenderer {
    /// Create a new planet renderer
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        // Load planet shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Planet Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/planet.wgsl").into()),
        });

        // Create planet uniform bind group layout
        let planet_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Planet Uniform Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<PlanetUniform>() as u64),
                },
                count: None,
            }],
        });

        // Create texture bind group layouts
        let terrain_texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Terrain Texture Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let cloud_texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cloud Texture Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create planet uniform buffer
        let planet_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Planet Uniform Buffer"),
            size: std::mem::size_of::<PlanetUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create planet bind group
        let planet_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Planet Bind Group"),
            layout: &planet_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: planet_buffer.as_entire_binding(),
            }],
        });

        // Create textures
        // Use Rgba8Unorm which is universally filterable
        let terrain_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Terrain Heightmap"),
            size: wgpu::Extent3d { width: 256, height: 128, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let terrain_texture_view = terrain_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let cloud_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Cloud Coverage"),
            size: wgpu::Extent3d { width: 256, height: 128, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let cloud_texture_view = cloud_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create samplers
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Planet Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create texture bind groups
        let terrain_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Terrain Texture Bind Group"),
            layout: &terrain_texture_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&terrain_texture_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        let cloud_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cloud Texture Bind Group"),
            layout: &cloud_texture_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&cloud_texture_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        // Create pipelines
        let terrain_pipeline = Self::create_terrain_pipeline(device, &planet_layout, &terrain_texture_layout, &shader, surface_config);
        let water_pipeline = Self::create_water_pipeline(device, &planet_layout, &shader, surface_config);
        let cloud_pipeline = Self::create_cloud_pipeline(device, &planet_layout, &cloud_texture_layout, &shader, surface_config);
        let atmosphere_pipeline = Self::create_atmosphere_pipeline(device, &planet_layout, &shader, surface_config);
        let storm_pipeline = Self::create_storm_pipeline(device, &planet_layout, &shader, surface_config);
        let settlement_pipeline = Self::create_settlement_pipeline(device, &planet_layout, &shader, surface_config);

        // Create buffers with capacities
        let max_terrain_vertices = 65000;  // For sphere mesh
        let max_water_vertices = 65000;
        let max_cloud_vertices = 65000;
        let max_storms = 100;
        let max_settlements = 1000;

        let terrain_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Terrain Vertex Buffer"),
            size: (std::mem::size_of::<TerrainVertex>() * max_terrain_vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let water_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Water Vertex Buffer"),
            size: (std::mem::size_of::<WaterVertex>() * max_water_vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let cloud_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cloud Vertex Buffer"),
            size: (std::mem::size_of::<CloudVertex>() * max_cloud_vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let storm_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storm Vertex Buffer"),
            size: (std::mem::size_of::<StormVertex>() * max_storms) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let settlement_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Settlement Vertex Buffer"),
            size: (std::mem::size_of::<SettlementVertex>() * max_settlements) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            terrain_pipeline,
            terrain_buffer,
            terrain_count: 0,
            water_pipeline,
            water_buffer,
            water_count: 0,
            cloud_pipeline,
            cloud_buffer,
            cloud_count: 0,
            atmosphere_pipeline,
            storm_pipeline,
            storm_buffer,
            storm_count: 0,
            settlement_pipeline,
            settlement_buffer,
            settlement_count: 0,
            planet_buffer,
            planet_bind_group,
            terrain_texture,
            terrain_texture_view,
            cloud_texture,
            cloud_texture_view,
            terrain_texture_bind_group,
            cloud_texture_bind_group,
            max_terrain_vertices,
            max_water_vertices,
            max_cloud_vertices,
            max_storms,
            max_settlements,
        }
    }

    /// Update planet uniform
    pub fn update_planet(
        &self,
        queue: &wgpu::Queue,
        view_proj: [[f32; 4]; 4],
        rotation: [[f32; 4]; 4],
        time: f32,
        axial_tilt: f32,
        solar_angle: f32,
        radius: f32,
        camera_pos: [f32; 3],
        scale_level: u32,
    ) {
        let uniform = PlanetUniform {
            view_proj,
            rotation,
            time,
            axial_tilt,
            solar_angle,
            radius,
            camera_pos,
            scale_level,
            _padding: [0.0, 0.0],
            _final_pad: [0.0, 0.0],
        };
        queue.write_buffer(&self.planet_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Update terrain data
    pub fn update_terrain(&mut self, queue: &wgpu::Queue, vertices: &[TerrainVertex]) {
        let count = vertices.len().min(self.max_terrain_vertices);
        if count > 0 {
            queue.write_buffer(&self.terrain_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.terrain_count = count as u32;
        }
    }

    /// Update water data
    pub fn update_water(&mut self, queue: &wgpu::Queue, vertices: &[WaterVertex]) {
        let count = vertices.len().min(self.max_water_vertices);
        if count > 0 {
            queue.write_buffer(&self.water_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.water_count = count as u32;
        }
    }

    /// Update cloud data
    pub fn update_clouds(&mut self, queue: &wgpu::Queue, vertices: &[CloudVertex]) {
        let count = vertices.len().min(self.max_cloud_vertices);
        if count > 0 {
            queue.write_buffer(&self.cloud_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.cloud_count = count as u32;
        }
    }

    /// Update storm data
    pub fn update_storms(&mut self, queue: &wgpu::Queue, vertices: &[StormVertex]) {
        let count = vertices.len().min(self.max_storms);
        if count > 0 {
            queue.write_buffer(&self.storm_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.storm_count = count as u32;
        }
    }

    /// Update settlement data
    pub fn update_settlements(&mut self, queue: &wgpu::Queue, vertices: &[SettlementVertex]) {
        let count = vertices.len().min(self.max_settlements);
        if count > 0 {
            queue.write_buffer(&self.settlement_buffer, 0, bytemuck::cast_slice(&vertices[..count]));
            self.settlement_count = count as u32;
        }
    }

    /// Update terrain heightmap texture
    pub fn update_terrain_texture(&self, queue: &wgpu::Queue, heightmap: &[f32]) {
        // Convert f32 heightmap to Rgba8Unorm format
        let mut rgba_data = Vec::with_capacity(heightmap.len() * 4);
        for &h in heightmap {
            let val = (h.clamp(0.0, 1.0) * 255.0) as u8;
            rgba_data.push(val); // R
            rgba_data.push(0);   // G
            rgba_data.push(0);   // B
            rgba_data.push(255); // A
        }
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.terrain_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(256 * 4),
                rows_per_image: Some(128),
            },
            wgpu::Extent3d { width: 256, height: 128, depth_or_array_layers: 1 },
        );
    }

    /// Update cloud texture
    pub fn update_cloud_texture(&self, queue: &wgpu::Queue, coverage: &[f32]) {
        // Convert f32 coverage to Rgba8Unorm format
        let mut rgba_data = Vec::with_capacity(coverage.len() * 4);
        for &c in coverage {
            let val = (c.clamp(0.0, 1.0) * 255.0) as u8;
            rgba_data.push(val); // R
            rgba_data.push(val); // G
            rgba_data.push(val); // B
            rgba_data.push(255); // A
        }
        
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.cloud_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(256 * 4),
                rows_per_image: Some(128),
            },
            wgpu::Extent3d { width: 256, height: 128, depth_or_array_layers: 1 },
        );
    }

    /// Render all planet surface layers
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // 1. Render terrain (solid, depth write)
        if self.terrain_count > 0 {
            render_pass.set_pipeline(&self.terrain_pipeline);
            render_pass.set_bind_group(0, &self.planet_bind_group, &[]);
            render_pass.set_bind_group(1, &self.terrain_texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.terrain_buffer.slice(..));
            render_pass.draw(0..self.terrain_count, 0..1);
        }

        // 2. Render water (transparent, depth test)
        if self.water_count > 0 {
            render_pass.set_pipeline(&self.water_pipeline);
            render_pass.set_bind_group(0, &self.planet_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.water_buffer.slice(..));
            render_pass.draw(0..self.water_count, 0..1);
        }

        // 3. Render clouds (transparent, above surface)
        if self.cloud_count > 0 {
            render_pass.set_pipeline(&self.cloud_pipeline);
            render_pass.set_bind_group(0, &self.planet_bind_group, &[]);
            render_pass.set_bind_group(1, &self.cloud_texture_bind_group, &[]);  // Changed from group 2 to group 1
            render_pass.set_vertex_buffer(0, self.cloud_buffer.slice(..));
            render_pass.draw(0..self.cloud_count, 0..1);
        }

        // 4. Render atmosphere glow
        render_pass.set_pipeline(&self.atmosphere_pipeline);
        render_pass.set_bind_group(0, &self.planet_bind_group, &[]);
        render_pass.draw(0..6, 0..1);  // Draw 6 vertices for fullscreen quad (2 triangles)

        // 5. Render storms
        if self.storm_count > 0 {
            render_pass.set_pipeline(&self.storm_pipeline);
            render_pass.set_bind_group(0, &self.planet_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.storm_buffer.slice(..));
            render_pass.draw(0..self.storm_count, 0..1);
        }

        // 6. Render settlements (Phase 6)
        if self.settlement_count > 0 {
            render_pass.set_pipeline(&self.settlement_pipeline);
            render_pass.set_bind_group(0, &self.planet_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.settlement_buffer.slice(..));
            render_pass.draw(0..self.settlement_count, 0..1);
        }
    }

    // ========================================================================
    // Pipeline Creation Methods
    // ========================================================================

    fn create_terrain_pipeline(
        device: &wgpu::Device,
        planet_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Terrain Pipeline Layout"),
            bind_group_layouts: &[planet_layout, texture_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Terrain Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_terrain",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TerrainVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 24, shader_location: 2, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 36, shader_location: 4, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_terrain",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_water_pipeline(
        device: &wgpu::Device,
        planet_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Water Pipeline Layout"),
            bind_group_layouts: &[planet_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Water Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_water",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<WaterVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 24, shader_location: 2, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_water",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_cloud_pipeline(
        device: &wgpu::Device,
        planet_layout: &wgpu::BindGroupLayout,
        texture_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cloud Pipeline Layout"),
            bind_group_layouts: &[planet_layout, texture_layout], // Note: bind group 2 for clouds
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cloud Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_cloud",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<CloudVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 20, shader_location: 2, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_cloud",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None, // No culling for clouds (viewed from both sides)
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_atmosphere_pipeline(
        device: &wgpu::Device,
        planet_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Atmosphere Pipeline Layout"),
            bind_group_layouts: &[planet_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Atmosphere Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_atmosphere",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_atmosphere",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }

    fn create_storm_pipeline(
        device: &wgpu::Device,
        planet_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Storm Pipeline Layout"),
            bind_group_layouts: &[planet_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Storm Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_storm",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<StormVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 20, shader_location: 3, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_storm",
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

    fn create_settlement_pipeline(
        device: &wgpu::Device,
        planet_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Settlement Pipeline Layout"),
            bind_group_layouts: &[planet_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Settlement Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_settlement",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<SettlementVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x3 },
                        wgpu::VertexAttribute { offset: 12, shader_location: 1, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32 },
                        wgpu::VertexAttribute { offset: 20, shader_location: 3, format: wgpu::VertexFormat::Float32 },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: "fs_settlement",
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
}
