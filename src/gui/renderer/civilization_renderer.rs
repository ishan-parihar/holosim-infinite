//! CivilizationRenderer - WGPU-based rendering for civilizations
//!
//! Phase 5: Civilization System
//!
//! Renders:
//! - Settlements (points with size based on population)
//! - Trade routes (lines between settlements)
//! - Population density heat map
//! - Civilization boundaries (colored by polarization)

use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::util::DeviceExt;

// ============================================================================
// VERTEX STRUCTURES
// ============================================================================

/// Vertex for settlement rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SettlementVertex {
    pub position: [f32; 3],
    pub size: f32,
    pub tech_level: f32,
    pub settlement_type: f32,
    pub _padding: [f32; 2],
}

impl SettlementVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Vertex for trade route rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TradeRouteVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub volume: f32,
    pub _padding: f32,
}

impl TradeRouteVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Vertex for population density rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct DensityVertex {
    pub position: [f32; 3],
    pub density: f32,
    pub _padding: [f32; 2],
}

impl DensityVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Vertex for civilization boundary rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct BoundaryVertex {
    pub position: [f32; 3],
    pub polarization: f32,
    pub _padding: [f32; 2],
}

impl BoundaryVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

// ============================================================================
// UNIFORM STRUCTURES
// ============================================================================

/// Uniform buffer for civilization shaders
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CivilizationUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub time: f32,
    pub highlight_id: u32,
    pub _padding: [f32; 2],
}

// ============================================================================
// CIVILIZATION RENDERER
// ============================================================================

/// Renderer for civilization visualization
pub struct CivilizationRenderer {
    // Pipelines
    settlement_pipeline: wgpu::RenderPipeline,
    trade_route_pipeline: wgpu::RenderPipeline,
    density_pipeline: wgpu::RenderPipeline,
    boundary_pipeline: wgpu::RenderPipeline,

    // Uniforms
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    // Vertex buffers (recreated each frame)
    settlement_buffer: Option<wgpu::Buffer>,
    trade_route_buffer: Option<wgpu::Buffer>,
    density_buffer: Option<wgpu::Buffer>,
    boundary_buffer: Option<wgpu::Buffer>,

    // Quad vertex buffer for billboards
    quad_vertex_buffer: wgpu::Buffer,

    // Instance counts
    settlement_count: u32,
    trade_route_vertex_count: u32,
    density_vertex_count: u32,
    boundary_vertex_count: u32,

    // Time for animations
    time: f32,
}

impl CivilizationRenderer {
    /// Create a new CivilizationRenderer
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        // Create uniform buffer
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Civilization Uniform Buffer"),
            contents: bytemuck::cast_slice(&[CivilizationUniforms {
                view_proj: [[0.0; 4]; 4],
                time: 0.0,
                highlight_id: 0,
                _padding: [0.0; 2],
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Civilization Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Civilization Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Civilization Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Load shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Civilization Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/civilization.wgsl").into()),
        });

        // Create settlement pipeline
        let settlement_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Settlement Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_settlement",
                buffers: &[SettlementVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_settlement",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Create trade route pipeline
        let trade_route_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Trade Route Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_trade_route",
                buffers: &[TradeRouteVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_trade_route",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Create density pipeline
        let density_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Density Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_density",
                buffers: &[DensityVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_density",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Create boundary pipeline
        let boundary_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Boundary Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_boundary",
                buffers: &[BoundaryVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_boundary",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Create quad vertex buffer for billboards (4 vertices per settlement)
        let quad_vertices: [[f32; 2]; 4] = [
            [-1.0, -1.0],
            [ 1.0, -1.0],
            [-1.0,  1.0],
            [ 1.0,  1.0],
        ];
        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Settlement Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            settlement_pipeline,
            trade_route_pipeline,
            density_pipeline,
            boundary_pipeline,
            uniform_buffer,
            uniform_bind_group,
            settlement_buffer: None,
            trade_route_buffer: None,
            density_buffer: None,
            boundary_buffer: None,
            quad_vertex_buffer,
            settlement_count: 0,
            trade_route_vertex_count: 0,
            density_vertex_count: 0,
            boundary_vertex_count: 0,
            time: 0.0,
        }
    }

    /// Update civilization data
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view_proj: &[[f32; 4]; 4],
        settlements: &[SettlementVertex],
        trade_routes: &[TradeRouteVertex],
        density_points: &[DensityVertex],
        boundaries: &[BoundaryVertex],
        highlighted_settlement: Option<u32>,
    ) {
        // Update time
        self.time += 0.016; // Approximate 60 FPS

        // Update uniforms
        let uniforms = CivilizationUniforms {
            view_proj: *view_proj,
            time: self.time,
            highlight_id: highlighted_settlement.unwrap_or(u32::MAX),
            _padding: [0.0; 2],
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // Update settlement buffer
        if !settlements.is_empty() {
            self.settlement_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Settlement Vertex Buffer"),
                    contents: bytemuck::cast_slice(settlements),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));
            self.settlement_count = settlements.len() as u32;
        } else {
            self.settlement_buffer = None;
            self.settlement_count = 0;
        }

        // Update trade route buffer
        if !trade_routes.is_empty() {
            self.trade_route_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Trade Route Vertex Buffer"),
                    contents: bytemuck::cast_slice(trade_routes),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));
            self.trade_route_vertex_count = trade_routes.len() as u32;
        } else {
            self.trade_route_buffer = None;
            self.trade_route_vertex_count = 0;
        }

        // Update density buffer
        if !density_points.is_empty() {
            self.density_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Density Vertex Buffer"),
                    contents: bytemuck::cast_slice(density_points),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));
            self.density_vertex_count = density_points.len() as u32;
        } else {
            self.density_buffer = None;
            self.density_vertex_count = 0;
        }

        // Update boundary buffer
        if !boundaries.is_empty() {
            self.boundary_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Boundary Vertex Buffer"),
                    contents: bytemuck::cast_slice(boundaries),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));
            self.boundary_vertex_count = boundaries.len() as u32;
        } else {
            self.boundary_buffer = None;
            self.boundary_vertex_count = 0;
        }
    }

    /// Render civilizations
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        // Set bind group
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

        // Render density (background layer)
        if let Some(buffer) = &self.density_buffer {
            if self.density_vertex_count > 0 {
                render_pass.set_pipeline(&self.density_pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..self.density_vertex_count, 0..1);
            }
        }

        // Render boundaries
        if let Some(buffer) = &self.boundary_buffer {
            if self.boundary_vertex_count > 0 {
                render_pass.set_pipeline(&self.boundary_pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..self.boundary_vertex_count, 0..1);
            }
        }

        // Render trade routes
        if let Some(buffer) = &self.trade_route_buffer {
            if self.trade_route_vertex_count > 0 {
                render_pass.set_pipeline(&self.trade_route_pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..self.trade_route_vertex_count, 0..1);
            }
        }

        // Render settlements (top layer)
        if let Some(buffer) = &self.settlement_buffer {
            if self.settlement_count > 0 {
                render_pass.set_pipeline(&self.settlement_pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                // Each settlement is rendered as a quad (4 vertices)
                render_pass.draw(0..4, 0..self.settlement_count);
            }
        }
    }

    /// Get settlement count
    pub fn settlement_count(&self) -> u32 {
        self.settlement_count
    }

    /// Get trade route count
    pub fn trade_route_count(&self) -> u32 {
        self.trade_route_vertex_count / 2
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert settlement data from simulation to render vertices
pub fn create_settlement_vertices(
    settlements: &[crate::gui::simulation_adapter::SettlementData],
) -> Vec<SettlementVertex> {
    settlements
        .iter()
        .map(|s| {
            let settlement_type = match s.settlement_type.as_str() {
                "Hamlet" => 0.0,
                "Village" => 1.0,
                "Town" => 2.0,
                "City" => 3.0,
                "Metropolis" => 4.0,
                "Megalopolis" => 5.0,
                _ => 0.0,
            };

            let size = (s.population as f64).log10() as f32 * 0.01 + 0.02;

            SettlementVertex {
                position: [s.position.0 as f32, 0.0, s.position.1 as f32],
                size,
                tech_level: (s.population as f64).log10() as f32 / 8.0, // Normalize tech level
                settlement_type,
                _padding: [0.0; 2],
            }
        })
        .collect()
}

/// Convert trade route data to render vertices
pub fn create_trade_route_vertices(
    routes: &[crate::gui::simulation_adapter::TradeRoute],
    settlements: &[crate::gui::simulation_adapter::SettlementData],
) -> Vec<TradeRouteVertex> {
    let mut vertices = Vec::new();

    for route in routes {
        // Find settlement positions
        let from_settlement = settlements.iter().find(|s| s.position.0 as u64 == route.from_settlement);
        let to_settlement = settlements.iter().find(|s| s.position.0 as u64 == route.to_settlement);

        if let (Some(from), Some(to)) = (from_settlement, to_settlement) {
            // Color based on trade volume
            let color = [
                0.3 + route.volume as f32 * 0.5,
                0.5,
                0.8 - route.volume as f32 * 0.3,
            ];

            // Start vertex
            vertices.push(TradeRouteVertex {
                position: [from.position.0 as f32, 0.01, from.position.1 as f32],
                color,
                volume: route.volume as f32,
                _padding: 0.0,
            });

            // End vertex
            vertices.push(TradeRouteVertex {
                position: [to.position.0 as f32, 0.01, to.position.1 as f32],
                color,
                volume: route.volume as f32,
                _padding: 0.0,
            });
        }
    }

    vertices
}

/// Convert population density to render vertices
pub fn create_density_vertices(
    density: &[crate::gui::simulation_adapter::PopulationDensity],
) -> Vec<DensityVertex> {
    density
        .iter()
        .map(|d| DensityVertex {
            position: [d.position.0 as f32, 0.005, d.position.1 as f32],
            density: d.density as f32,
            _padding: [0.0; 2],
        })
        .collect()
}

/// Create boundary vertices from civilization polarization
pub fn create_boundary_vertices(
    civilizations: &[crate::gui::simulation_adapter::CivilizationSummary],
) -> Vec<BoundaryVertex> {
    // For now, create simple boundary points
    // In a full implementation, this would trace the actual territory boundaries
    civilizations
        .iter()
        .flat_map(|c| {
            // Create a simple circle boundary
            let segments = 16u32;
            let tech_level_f32 = c.tech_level as f32;
            (0..segments)
                .map(|i| {
                    let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
                    let radius = 0.1 + tech_level_f32 * 0.1;
                    BoundaryVertex {
                        position: [
                            tech_level_f32 * 0.5 + radius * angle.cos(),
                            0.02,
                            tech_level_f32 * 0.3 + radius * angle.sin(),
                        ],
                        polarization: c.polarization as f32,
                        _padding: [0.0; 2],
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
