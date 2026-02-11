//! Instanced Renderer - High-performance rendering for multiple entities
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 4 Week 8:
//! "Instanced rendering implementation - Refactor entity rendering for instancing, Instance buffers for entity data, Reduce draw calls from 1000+ to ~10"

use crate::gui::renderer::{
    buffers::{Entity3DInstanceData, InstanceBufferManager},
    shaders::generate_sphere_geometry,
    wgpu_context::WgpuContext,
};
use wgpu::{
    util::DeviceExt, BindGroup, BindGroupLayout, Buffer, BufferUsages, CommandEncoder, Device,
    IndexFormat, Queue, RenderPass, RenderPipeline as WgpuRenderPipeline, SurfaceConfiguration,
};

/// Instanced renderer for 3D entities
#[derive(Debug)]
pub struct InstancedRenderer {
    pipeline: WgpuRenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    normal_buffer: Buffer,
    uv_buffer: Buffer,
    num_indices: u32,
    instance_manager: InstanceBufferManager,
    camera_bind_group_layout: BindGroupLayout,
    light_bind_group_layout: BindGroupLayout,
    instance_bind_group_layout: BindGroupLayout,
    camera_bind_group: Option<BindGroup>,
    light_bind_group: Option<BindGroup>,
    instance_bind_group: Option<BindGroup>,
}

/// Camera uniforms for 3D rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniforms {
    pub view_proj: [[f32; 4]; 4],
    pub camera_pos: [f32; 3],
    pub _padding: f32,
}

/// Light uniforms for 3D rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniforms {
    pub light_pos: [f32; 3],
    pub light_color: [f32; 3],
    pub light_intensity: f32,
    pub ambient_intensity: f32,
    pub _padding: [f32; 2],
}

impl InstancedRenderer {
    /// Create new instanced renderer
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        max_instances: usize,
    ) -> Self {
        // Generate sphere geometry
        let (vertices, normals, uvs, indices) = generate_sphere_geometry(32, 32);

        // Create vertex buffers
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let normal_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Normal Buffer"),
            contents: bytemuck::cast_slice(&normals),
            usage: BufferUsages::VERTEX,
        });

        let uv_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere UV Buffer"),
            contents: bytemuck::cast_slice(&uvs),
            usage: BufferUsages::VERTEX,
        });

        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        // Create instance buffer manager
        let instance_manager = InstanceBufferManager::new(device, max_instances);

        // Create bind group layouts
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
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

        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Light Bind Group Layout"),
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

        let instance_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Instance Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Instanced Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &light_bind_group_layout,
                &instance_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        // Create shader module
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Entity 3D Shader"),
            source: wgpu::ShaderSource::Wgsl(
                crate::gui::renderer::shaders::entity::ENTITY_SHADER.into(),
            ),
        });

        // Create render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Instanced Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[
                    // Vertex position buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        }],
                    },
                    // Normal buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        }],
                    },
                    // UV buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x2,
                        }],
                    },
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Entity3DInstanceData>()
                            as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: 16,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: 32,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: 48,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: 64,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: 80,
                                shader_location: 8,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: 96,
                                shader_location: 9,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        InstancedRenderer {
            pipeline,
            vertex_buffer,
            index_buffer,
            normal_buffer,
            uv_buffer,
            num_indices: indices.len() as u32,
            instance_manager,
            camera_bind_group_layout,
            light_bind_group_layout,
            instance_bind_group_layout,
            camera_bind_group: None,
            light_bind_group: None,
            instance_bind_group: None,
        }
    }

    /// Update camera uniforms
    pub fn update_camera(
        &mut self,
        device: &Device,
        queue: &Queue,
        camera_uniforms: &CameraUniforms,
    ) {
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*camera_uniforms]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        self.camera_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &self.camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        }));
    }

    /// Update light uniforms
    pub fn update_light(&mut self, device: &Device, queue: &Queue, light_uniforms: &LightUniforms) {
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Uniform Buffer"),
            contents: bytemuck::cast_slice(&[*light_uniforms]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        self.light_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light Bind Group"),
            layout: &self.light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        }));
    }

    /// Update instance bind group
    pub fn update_instance_bind_group(&mut self, device: &Device) {
        self.instance_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Instance Bind Group"),
            layout: &self.instance_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.instance_manager.instance_buffer().as_entire_binding(),
            }],
        }));
    }

    /// Update instance data
    pub fn update_instances(&self, queue: &Queue, instances: &[Entity3DInstanceData]) {
        self.instance_manager.update_instances(queue, instances);
    }

    /// Render instances
    pub fn render<'a>(&'a self, pass: &mut RenderPass<'a>, instance_count: u32) {
        let camera_bg = match &self.camera_bind_group {
            Some(bg) => bg,
            None => return,
        };
        let light_bg = match &self.light_bind_group {
            Some(bg) => bg,
            None => return,
        };
        let instance_bg = match &self.instance_bind_group {
            Some(bg) => bg,
            None => return,
        };

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, camera_bg, &[]);
        pass.set_bind_group(1, light_bg, &[]);
        pass.set_bind_group(2, instance_bg, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, self.normal_buffer.slice(..));
        pass.set_vertex_buffer(2, self.uv_buffer.slice(..));
        pass.set_vertex_buffer(3, self.instance_manager.instance_buffer().slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        pass.draw_indexed(0..self.num_indices, 0, 0..instance_count);
    }

    /// Get max instances
    pub fn max_instances(&self) -> usize {
        self.instance_manager.max_instances()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_uniforms_size() {
        assert_eq!(std::mem::size_of::<CameraUniforms>(), 80);
    }

    #[test]
    fn test_light_uniforms_size() {
        // LightUniforms: light_pos[3] (12) + light_color[3] (12) + light_intensity (4) + ambient_intensity (4) + _padding[2] (8) = 40
        assert_eq!(std::mem::size_of::<LightUniforms>(), 40);
    }
}
