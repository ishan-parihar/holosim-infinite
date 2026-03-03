//! Entity Renderer - Renders simulation entities
//!
//! Phase 1: Basic entity rendering with instancing

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::renderer::entity_instance::{ArchetypeData, EntityInstance};
use crate::hpo::RenderableEntity;
use wgpu::util::DeviceExt;

/// Renderer for simulation entities
pub struct EntityRenderer {
    /// Render pipeline
    pipeline: wgpu::RenderPipeline,

    /// Vertex buffer for circle geometry
    vertex_buffer: wgpu::Buffer,

    /// Instance buffer for entity data
    instance_buffer: wgpu::Buffer,

    /// Camera uniform buffer
    camera_uniform_buffer: wgpu::Buffer,

    /// Camera bind group
    camera_bind_group: wgpu::BindGroup,

    /// Phase 3: Archetype storage buffer (22 activations per entity)
    archetype_buffer: wgpu::Buffer,

    /// Phase 3: Archetype bind group
    archetype_bind_group: wgpu::BindGroup,

    /// Number of vertices per circle
    num_vertices: u32,

    /// Maximum number of instances
    max_instances: usize,

    /// Current number of instances
    current_instance_count: u32,
}

impl EntityRenderer {
    /// Create a new entity renderer
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Entity Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/entity.wgsl").into()),
        });

        // Phase 3: Create archetype bind group layout (storage buffer)
        let archetype_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Archetype Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,  // Phase 16: Vertex needs archetype interference
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<ArchetypeData>() as u64,
                        ),
                    },
                    count: None,
                }],
            });

        // Create camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(96),
                    },
                    count: None,
                }],
            });

        // Create camera uniform buffer (4x4 matrix = 16 floats * 4 bytes)
        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: 96, // 24 floats * 4 bytes (matrix + selection focus data)
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create camera bind group
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        // Phase 3: Create archetype storage buffer
        let max_instances = 1000;
        let archetype_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Archetype Storage Buffer"),
            size: (std::mem::size_of::<ArchetypeData>() * max_instances) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Phase 3: Create archetype bind group
        let archetype_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Archetype Bind Group"),
            layout: &archetype_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 1,
                resource: archetype_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout with camera bind group
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Entity Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &archetype_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Entity Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    // Vertex buffer layout
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        }],
                    },
                    // Instance buffer layout
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<EntityInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // Position (offset 0, 12 bytes + 4 padding = 16 bytes)
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            // Realm intensities (offset 16, 28 bytes)
                            wgpu::VertexAttribute {
                                offset: 16,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4, // violet, indigo, blue, green
                            },
                            wgpu::VertexAttribute {
                                offset: 32,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x3, // yellow, orange, red
                            },
                            // Consciousness data (offset 44, 8 bytes)
                            wgpu::VertexAttribute {
                                offset: 44,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x2, // consciousness_level, polarization
                            },
                            // Spectrum data (offset 52, 12 bytes)
                            wgpu::VertexAttribute {
                                offset: 52,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x3, // space_time_ratio, time_space_ratio, veil_transparency
                            },
                            // Evolution data (offset 64, 8 bytes)
                            wgpu::VertexAttribute {
                                offset: 64,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32, // evolution_progress
                            },
                            wgpu::VertexAttribute {
                                offset: 68,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Uint32, // density_level (packed with padding)
                            },
                            // Archetype summary (offset 80-87, 8 bytes)
                            wgpu::VertexAttribute {
                                offset: 80,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Uint32, // archetype_activated - FIXED TYPE
                            },
                            wgpu::VertexAttribute {
                                offset: 84,
                                shader_location: 9,
                                format: wgpu::VertexFormat::Float32, // archetype_intensity - FIXED TYPE
                            },
                            // Entity type and size (offset 88-95, 8 bytes)
                            wgpu::VertexAttribute {
                                offset: 88,
                                shader_location: 10,
                                format: wgpu::VertexFormat::Uint32, // entity_type
                            },
                            wgpu::VertexAttribute {
                                offset: 92,
                                shader_location: 11,
                                format: wgpu::VertexFormat::Float32, // size
                            },
                            // Hierarchy data (offset 72-79, 8 bytes)
                            wgpu::VertexAttribute {
                                offset: 72,
                                shader_location: 12,
                                format: wgpu::VertexFormat::Uint32, // parent_id
                            },
                            wgpu::VertexAttribute {
                                offset: 76,
                                shader_location: 13,
                                format: wgpu::VertexFormat::Uint32, // environment_id
                            },
                            // Morphology parameters (offset 96-111, 16 bytes)
                            wgpu::VertexAttribute {
                                offset: 96,
                                shader_location: 14,
                                format: wgpu::VertexFormat::Float32x4, // anisotropy, depth_bias, lobe_count, interference_phase
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Create circle vertex buffer
        let (vertices, num_vertices) = Self::create_circle_vertices(32);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Circle Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create instance buffer (max 1000 entities)
        let max_instances = 1000;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Entity Instance Buffer"),
            size: (std::mem::size_of::<EntityInstance>() * max_instances) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            instance_buffer,
            camera_uniform_buffer,
            camera_bind_group,
            archetype_buffer,
            archetype_bind_group,
            num_vertices,
            max_instances,
            current_instance_count: 0,
        }
    }

    /// Update entity instances
    pub fn update_entities(&mut self, queue: &wgpu::Queue, entities: &[SubSubLogos]) {
        // Convert entities to instances
        let instances: Vec<EntityInstance> = entities
            .iter()
            .enumerate()
            .map(|(i, entity)| EntityInstance::from_entity(entity, i))
            .take(self.max_instances)
            .collect();

        self.current_instance_count = instances.len() as u32;

        // Upload to GPU
        if !instances.is_empty() {
            let instance_data = bytemuck::cast_slice(&instances);
            queue.write_buffer(&self.instance_buffer, 0, instance_data);
        }

        // Phase 3: Extract and upload archetype activations
        let archetype_data: Vec<ArchetypeData> = entities
            .iter()
            .take(self.max_instances)
            .map(|entity| ArchetypeData::from_entity(entity))
            .collect();

        if !archetype_data.is_empty() {
            let archetype_bytes = bytemuck::cast_slice(&archetype_data);
            queue.write_buffer(&self.archetype_buffer, 0, archetype_bytes);
        }
    }

    /// Update entity instances from field-derived holographic entities
    pub fn update_holo_entities(&mut self, queue: &wgpu::Queue, entities: &[RenderableEntity]) {
        let instances: Vec<EntityInstance> = entities
            .iter()
            .map(EntityInstance::from_renderable_entity)
            .take(self.max_instances)
            .collect();

        self.current_instance_count = instances.len() as u32;

        if !instances.is_empty() {
            let instance_data = bytemuck::cast_slice(&instances);
            queue.write_buffer(&self.instance_buffer, 0, instance_data);
        }

        let archetype_data: Vec<ArchetypeData> = entities
            .iter()
            .take(self.max_instances)
            .map(|entity| {
                let mut data = ArchetypeData {
                    activations: [0.0; 22],
                };
                let seed = (entity.density_band.index().min(7) as f32) / 7.0;
                data.activations[0] = seed;
                data.activations[1] = entity.consciousness.clamp(0.0, 1.0) as f32;
                data
            })
            .collect();

        if !archetype_data.is_empty() {
            let archetype_bytes = bytemuck::cast_slice(&archetype_data);
            queue.write_buffer(&self.archetype_buffer, 0, archetype_bytes);
        }
    }

    /// Update with test instances (for debugging)
    pub fn update_test_instances(&mut self, queue: &wgpu::Queue, count: usize) {
        let instances: Vec<EntityInstance> =
            (0..count).map(EntityInstance::test_instance).collect();

        self.current_instance_count = instances.len() as u32;

        if !instances.is_empty() {
            let instance_data = bytemuck::cast_slice(&instances);
            queue.write_buffer(&self.instance_buffer, 0, instance_data);
        }

        // Phase 3: Update test archetype data
        let archetype_data: Vec<ArchetypeData> = (0..count).map(ArchetypeData::test_data).collect();

        if !archetype_data.is_empty() {
            let archetype_bytes = bytemuck::cast_slice(&archetype_data);
            queue.write_buffer(&self.archetype_buffer, 0, archetype_bytes);
        }
    }

    /// Update camera uniform buffer
    pub fn update_camera(
        &self,
        queue: &wgpu::Queue,
        view_projection: [[f32; 4]; 4],
        time: f32,
        morphology_enabled: bool,
        selected_focus: Option<[f32; 3]>,
        selection_focus_enabled: bool,
    ) {
        // Camera buffer layout at 96 bytes:
        // 16 floats matrix + time + morphology_mode + selection fields + padding.
        let mut camera_data = [0.0_f32; 24];

        // Add 4x4 view_projection matrix (16 floats)
        for (row_index, row) in view_projection.iter().enumerate() {
            let base = row_index * 4;
            camera_data[base..base + 4].copy_from_slice(row);
        }

        // Add time and morphology mode flag
        camera_data[16] = time;
        camera_data[17] = if morphology_enabled { 1.0 } else { 0.0 };
        camera_data[18] = if selection_focus_enabled && selected_focus.is_some() {
            1.0
        } else {
            0.0
        };
        camera_data[19] = 0.20;
        if let Some(position) = selected_focus {
            camera_data[20] = position[0];
            camera_data[21] = position[1];
            camera_data[22] = position[2];
        }
        camera_data[23] = 0.0;

        queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&camera_data),
        );
    }

    /// Render entities
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.current_instance_count == 0 {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        // Phase 3: Set archetype bind group
        render_pass.set_bind_group(1, &self.archetype_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..self.current_instance_count);
    }

    /// Get the current number of instances being rendered
    pub fn current_instance_count(&self) -> u32 {
        self.current_instance_count
    }

    /// Create circle vertices as triangle fan
    fn create_circle_vertices(segments: u32) -> (Vec<[f32; 3]>, u32) {
        let mut vertices = Vec::new();

        // Center vertex
        vertices.push([0.0, 0.0, 0.0]);

        // Create vertices around the circle
        for i in 0..=segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
            vertices.push([angle.cos(), angle.sin(), 0.0]);
        }

        // Convert triangle fan to triangles
        let mut triangles = Vec::new();
        for i in 1..=segments {
            triangles.push(vertices[0]); // Center
            triangles.push(vertices[i as usize]);
            triangles.push(vertices[(i + 1) as usize]);
        }

        let num_vertices = triangles.len() as u32;
        (triangles, num_vertices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_vertices() {
        let (vertices, count) = EntityRenderer::create_circle_vertices(32);
        assert!(!vertices.is_empty());
        assert_eq!(count, 96); // 32 segments * 3 vertices per triangle
    }
}
