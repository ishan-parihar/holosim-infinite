//! Connection Renderer - Renders hierarchical connection lines
//!
//! Phase 4: Hierarchy Visualization
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md:
//! "Visualize composition hierarchy, parent-child relationships, and environment relationships."
//!
//! This renderer draws colored lines connecting related entities:
//! - Parent-child: Blue lines
//! - Composition: Green lines
//! - Environment: Yellow lines

use crate::entity_layer7::layer7::SubSubLogos;
use crate::gui::renderer::hierarchy_connection::{HierarchyConnection, generate_connections};
use crate::gui::renderer::entity_instance::EntityInstance;
use wgpu::util::DeviceExt;

/// Renderer for hierarchical connection lines
pub struct ConnectionRenderer {
    /// Render pipeline
    pipeline: wgpu::RenderPipeline,

    /// Vertex buffer for line segments
    vertex_buffer: wgpu::Buffer,

    /// Connection data buffer
    connection_buffer: wgpu::Buffer,

    /// Camera uniform buffer
    camera_uniform_buffer: wgpu::Buffer,

    /// Camera bind group
    camera_bind_group: wgpu::BindGroup,

    /// Number of connections
    num_connections: u32,

    /// Maximum number of connections
    max_connections: usize,
}

impl ConnectionRenderer {
    /// Create a new connection renderer
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Connection Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/connection.wgsl").into()),
        });

        // Create camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Connection Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(80),
                    },
                    count: None,
                }],
            });

        // Create camera uniform buffer (4x4 matrix = 16 floats * 4 bytes)
        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Connection Camera Uniform Buffer"),
            size: 80, // 16 floats * 4 bytes + 1 float for time
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create camera bind group
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Connection Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Connection Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Connection Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    // Vertex buffer layout (line endpoint selector)
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        }],
                    },
                    // Connection buffer layout (instance data)
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<HierarchyConnection>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3, // from_position
                            },
                            wgpu::VertexAttribute {
                                offset: 12,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x3, // to_position
                            },
                            wgpu::VertexAttribute {
                                offset: 24,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Uint32, // connection_type
                            },
                            wgpu::VertexAttribute {
                                offset: 28,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32, // intensity
                            },
                        ],
                    },
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
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

        // Create vertex buffer for line segment (2 vertices: start and end)
        let vertices: [[f32; 3]; 2] = [
            [0.0, 0.0, 0.0], // Vertex 0: line start
            [1.0, 0.0, 0.0], // Vertex 1: line end
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Connection Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create connection buffer (max 1000 connections)
        let max_connections = 1000;
        let connection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Connection Data Buffer"),
            size: (std::mem::size_of::<HierarchyConnection>() * max_connections) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            connection_buffer,
            camera_uniform_buffer,
            camera_bind_group,
            num_connections: 0,
            max_connections,
        }
    }

    /// Update connections from entity collection
    pub fn update_connections(
        &mut self,
        queue: &wgpu::Queue,
        entities: &[SubSubLogos],
        entity_instances: &[EntityInstance],
    ) {
        // Generate connections from entity hierarchy
        let connections = generate_connections(entities, entity_instances);
        
        self.num_connections = connections.len() as u32;

        // Upload connection data to GPU
        if !connections.is_empty() {
            let connection_bytes = bytemuck::cast_slice(&connections);
            queue.write_buffer(&self.connection_buffer, 0, connection_bytes);
        }
    }

    /// Update camera matrix
    pub fn update_camera(&mut self, queue: &wgpu::Queue, matrix: [[f32; 4]; 4], time: f32) {
        // Combine matrix and time into a single buffer
        let mut data = [0.0f32; 17]; // 16 floats for matrix + 1 for time
        for row in 0..4 {
            for col in 0..4 {
                data[row * 4 + col] = matrix[row][col];
            }
        }
        data[16] = time;
        
        let matrix_bytes = bytemuck::cast_slice(&data);
        queue.write_buffer(&self.camera_uniform_buffer, 0, matrix_bytes);
    }

    /// Render connections
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.num_connections == 0 {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.connection_buffer.slice(..));
        render_pass.draw(0..2, 0..self.num_connections);
    }

    /// Get number of connections
    pub fn connection_count(&self) -> u32 {
        self.num_connections
    }
}
