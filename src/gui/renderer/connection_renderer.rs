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
use crate::gui::renderer::entity_instance::EntityInstance;
use crate::gui::renderer::field_visual_bridge::PhaseCoherenceEdge;
use crate::gui::renderer::hierarchy_connection::{
    generate_connections, generate_holo_connections, ConnectionType, HierarchyConnection,
};
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

    /// Number of structural (non-phase) connections in active buffer
    structural_count: u32,

    /// Number of phase-coherence connections in active buffer
    phase_count: u32,

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
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(96),
                    },
                    count: None,
                }],
            });

        // Create camera uniform buffer (4x4 matrix + params, padded to 24 floats / 96 bytes)
        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Connection Camera Uniform Buffer"),
            size: 96,
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
                        array_stride: std::mem::size_of::<HierarchyConnection>()
                            as wgpu::BufferAddress,
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
                            wgpu::VertexAttribute {
                                offset: 32,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32, // phase_delta
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
            structural_count: 0,
            phase_count: 0,
            max_connections,
        }
    }

    fn finalize_connections(
        &mut self,
        queue: &wgpu::Queue,
        mut connections: Vec<HierarchyConnection>,
    ) {
        if connections.len() > self.max_connections {
            connections.truncate(self.max_connections);
        }

        self.phase_count = connections
            .iter()
            .filter(|connection| Self::is_phase_like_connection(connection.connection_type))
            .count() as u32;
        self.structural_count = connections.len() as u32 - self.phase_count;
        self.num_connections = connections.len() as u32;

        if !connections.is_empty() {
            let connection_bytes = bytemuck::cast_slice(&connections);
            queue.write_buffer(&self.connection_buffer, 0, connection_bytes);
        }
    }

    fn apply_profile(
        connections: &mut [HierarchyConnection],
        structural_scale: f32,
        phase_scale: f32,
        intensity_bias: f32,
    ) {
        for connection in connections {
            let scale = if Self::is_phase_like_connection(connection.connection_type) {
                phase_scale
            } else {
                structural_scale
            };
            connection.intensity = (connection.intensity * scale + intensity_bias).clamp(0.0, 1.0);
        }
    }

    #[inline]
    fn is_phase_like_connection(connection_type: u32) -> bool {
        connection_type == ConnectionType::PhaseCoherence as u32
            || connection_type == ConnectionType::Entanglement as u32
    }

    fn append_phase_network_edges(
        connections: &mut Vec<HierarchyConnection>,
        phase_edges: &[PhaseCoherenceEdge],
        max_connections: usize,
    ) {
        const ENTANGLEMENT_THRESHOLD: f32 = 0.85;

        for edge in phase_edges {
            if connections.len() >= max_connections {
                break;
            }

            let coherence = (edge.coherence as f32).clamp(0.0, 1.0);
            let phase_delta = 1.0 - coherence;

            let connection = if coherence >= ENTANGLEMENT_THRESHOLD {
                HierarchyConnection::entanglement(edge.start, edge.end, coherence, phase_delta)
            } else {
                HierarchyConnection::phase_coherence_with_delta(
                    edge.start,
                    edge.end,
                    coherence,
                    phase_delta,
                )
            };

            connections.push(connection);
        }
    }

    fn apply_focus_lens(
        connections: &mut [HierarchyConnection],
        focus_pos: [f32; 3],
        radius: f32,
        non_focus_scale: f32,
        edge_boost: f32,
    ) {
        let safe_radius = radius.max(f32::EPSILON);

        for connection in connections {
            let midpoint = [
                (connection.from_position[0] + connection.to_position[0]) * 0.5,
                (connection.from_position[1] + connection.to_position[1]) * 0.5,
                (connection.from_position[2] + connection.to_position[2]) * 0.5,
            ];

            let dx = midpoint[0] - focus_pos[0];
            let dy = midpoint[1] - focus_pos[1];
            let dz = midpoint[2] - focus_pos[2];
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();

            let t = (dist / safe_radius).clamp(0.0, 1.0);
            let influence = 1.0 - (t * t * (3.0 - 2.0 * t));
            let scale = non_focus_scale + (1.0 - non_focus_scale) * influence;

            connection.intensity =
                (connection.intensity * scale + edge_boost * influence).clamp(0.0, 1.0);
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
        self.finalize_connections(queue, connections);
    }

    /// Update structural connections and append field-derived phase-coherence edges
    pub fn update_connections_with_phase_network(
        &mut self,
        queue: &wgpu::Queue,
        entities: &[SubSubLogos],
        entity_instances: &[EntityInstance],
        phase_edges: &[PhaseCoherenceEdge],
    ) {
        let mut connections = generate_connections(entities, entity_instances);
        Self::append_phase_network_edges(&mut connections, phase_edges, self.max_connections);

        self.finalize_connections(queue, connections);
    }

    /// Update structural + phase connections with a stage-aware intensity profile
    pub fn update_connections_with_phase_network_profile(
        &mut self,
        queue: &wgpu::Queue,
        entities: &[SubSubLogos],
        entity_instances: &[EntityInstance],
        phase_edges: &[PhaseCoherenceEdge],
        structural_scale: f32,
        phase_scale: f32,
        intensity_bias: f32,
    ) {
        let mut connections = generate_connections(entities, entity_instances);
        Self::append_phase_network_edges(&mut connections, phase_edges, self.max_connections);

        Self::apply_profile(
            &mut connections,
            structural_scale,
            phase_scale,
            intensity_bias,
        );
        self.finalize_connections(queue, connections);
    }

    /// Update structural + phase connections with profile and neighborhood focus lens
    pub fn update_connections_with_phase_network_profile_lens(
        &mut self,
        queue: &wgpu::Queue,
        entities: &[SubSubLogos],
        entity_instances: &[EntityInstance],
        phase_edges: &[PhaseCoherenceEdge],
        structural_scale: f32,
        phase_scale: f32,
        intensity_bias: f32,
        focus_pos: [f32; 3],
        radius: f32,
        non_focus_scale: f32,
        edge_boost: f32,
    ) {
        let mut connections = generate_connections(entities, entity_instances);
        Self::append_phase_network_edges(&mut connections, phase_edges, self.max_connections);

        Self::apply_profile(
            &mut connections,
            structural_scale,
            phase_scale,
            intensity_bias,
        );
        Self::apply_focus_lens(
            &mut connections,
            focus_pos,
            radius,
            non_focus_scale,
            edge_boost,
        );
        self.finalize_connections(queue, connections);
    }

    /// Update structural connections directly from field-derived entities
    pub fn update_holo_connections(
        &mut self,
        queue: &wgpu::Queue,
        holo_entities: &[crate::hpo::RenderableEntity],
    ) {
        let connections = generate_holo_connections(holo_entities);
        self.finalize_connections(queue, connections);
    }

    /// Update field-derived structural connections and append phase-coherence edges
    pub fn update_holo_connections_with_phase_network(
        &mut self,
        queue: &wgpu::Queue,
        holo_entities: &[crate::hpo::RenderableEntity],
        phase_edges: &[PhaseCoherenceEdge],
    ) {
        let mut connections = generate_holo_connections(holo_entities);
        Self::append_phase_network_edges(&mut connections, phase_edges, self.max_connections);

        self.finalize_connections(queue, connections);
    }

    /// Update field-derived structural + phase connections with a stage-aware intensity profile
    pub fn update_holo_connections_with_phase_network_profile(
        &mut self,
        queue: &wgpu::Queue,
        holo_entities: &[crate::hpo::RenderableEntity],
        phase_edges: &[PhaseCoherenceEdge],
        structural_scale: f32,
        phase_scale: f32,
        intensity_bias: f32,
    ) {
        let mut connections = generate_holo_connections(holo_entities);
        Self::append_phase_network_edges(&mut connections, phase_edges, self.max_connections);

        Self::apply_profile(
            &mut connections,
            structural_scale,
            phase_scale,
            intensity_bias,
        );
        self.finalize_connections(queue, connections);
    }

    /// Update field-derived structural + phase connections with profile and neighborhood focus lens
    pub fn update_holo_connections_with_phase_network_profile_lens(
        &mut self,
        queue: &wgpu::Queue,
        holo_entities: &[crate::hpo::RenderableEntity],
        phase_edges: &[PhaseCoherenceEdge],
        structural_scale: f32,
        phase_scale: f32,
        intensity_bias: f32,
        focus_pos: [f32; 3],
        radius: f32,
        non_focus_scale: f32,
        edge_boost: f32,
    ) {
        let mut connections = generate_holo_connections(holo_entities);
        Self::append_phase_network_edges(&mut connections, phase_edges, self.max_connections);

        Self::apply_profile(
            &mut connections,
            structural_scale,
            phase_scale,
            intensity_bias,
        );
        Self::apply_focus_lens(
            &mut connections,
            focus_pos,
            radius,
            non_focus_scale,
            edge_boost,
        );
        self.finalize_connections(queue, connections);
    }

    /// Update camera matrix
    pub fn update_camera(
        &mut self,
        queue: &wgpu::Queue,
        matrix: [[f32; 4]; 4],
        time: f32,
        warm_cool_bias: f32,
        saturation: f32,
        phase_tint_boost: f32,
        grade_strength: f32,
    ) {
        let warm_cool_bias = warm_cool_bias.clamp(0.0, 1.0);
        let saturation = saturation.clamp(0.5, 1.8);
        let phase_tint_boost = phase_tint_boost.clamp(0.0, 0.6);
        let grade_strength = grade_strength.clamp(0.0, 1.0);

        // Combine matrix, grading parameters, and animation time into a 96-byte buffer.
        let mut data = [0.0f32; 24];
        for row in 0..4 {
            for col in 0..4 {
                data[row * 4 + col] = matrix[row][col];
            }
        }
        data[16] = warm_cool_bias;
        data[17] = saturation;
        data[18] = phase_tint_boost;
        data[19] = grade_strength;
        data[20] = time;

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

    /// Get number of active structural (non-phase) connections
    pub fn structural_connection_count(&self) -> u32 {
        self.structural_count
    }

    /// Get number of active phase-network connections (phase coherence + entanglement)
    pub fn phase_connection_count(&self) -> u32 {
        self.phase_count
    }
}
