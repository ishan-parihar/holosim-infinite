//! ObservableEntityRenderer — Simple WGPU instanced renderer for ObservableProperties.
//!
//! Draws colored quads, one per entity, with hit-testing support.

use crate::gui::observable_properties::{EntityShape, ObservableProperties};
use wgpu::util::DeviceExt;

/// GPU instance data — must match shader layout exactly.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceData {
    position: [f32; 2],
    size: f32,
    _pad0: f32,
    color: [f32; 3],
    glow: f32,
    shape: u32,
    _pad1: u32,
}

/// Simple vertex for a quad.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

pub struct ObservableEntityRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_count: usize,
}

impl ObservableEntityRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("EntitySimpleShader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "../shaders/entity_simple.wgsl"
            ))),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("EntitySimpleLayout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Quad vertices: unit square from -1 to 1
        let vertices: [Vertex; 6] = [
            Vertex {
                position: [-1.0, -1.0],
            },
            Vertex {
                position: [1.0, -1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
            },
            Vertex {
                position: [1.0, 1.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("EntityQuadVertexBuffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let max_instances = 10000;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("EntityInstanceBuffer"),
            size: (std::mem::size_of::<InstanceData>() * max_instances) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertex_buffers = [
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                }],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<InstanceData>() as u64,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                    wgpu::VertexAttribute {
                        offset: 8,
                        shader_location: 2,
                        format: wgpu::VertexFormat::Float32,
                    },
                    wgpu::VertexAttribute {
                        offset: 16,
                        shader_location: 3,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    wgpu::VertexAttribute {
                        offset: 28,
                        shader_location: 4,
                        format: wgpu::VertexFormat::Float32,
                    },
                    wgpu::VertexAttribute {
                        offset: 32,
                        shader_location: 5,
                        format: wgpu::VertexFormat::Uint32,
                    },
                ],
            },
        ];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("EntitySimplePipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
        });

        Self {
            pipeline,
            vertex_buffer,
            instance_buffer,
            instance_count: 0,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, entities: &[ObservableProperties]) {
        let instances: Vec<InstanceData> = entities
            .iter()
            .map(|e| InstanceData {
                position: e.position,
                size: e.size,
                _pad0: 0.0,
                color: e.color,
                glow: e.glow,
                shape: e.shape as u32,
                _pad1: 0,
            })
            .collect();
        self.instance_count = instances.len();
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count == 0 {
            return;
        }
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..self.instance_count as u32);
    }

    /// Hit-test in world space against entity shapes. Returns entity index or None.
    pub fn hit_test(
        &self,
        world_pos: [f32; 2],
        entities: &[ObservableProperties],
    ) -> Option<usize> {
        hit_test_shape(world_pos, entities)
    }
}

/// Shape-aware hit-test — dispatches to shape-specific logic.
fn hit_test_shape(world_pos: [f32; 2], entities: &[ObservableProperties]) -> Option<usize> {
    for (i, e) in entities.iter().enumerate() {
        let dx = world_pos[0] - e.position[0];
        let dy = world_pos[1] - e.position[1];
        let local = [dx / e.size, dy / e.size];
        if hit_test_shape_local(local, e.shape) {
            return Some(i);
        }
    }
    None
}

/// Test if a point in local space [-1,1] is inside the given shape.
fn hit_test_shape_local(local: [f32; 2], shape: EntityShape) -> bool {
    match shape {
        EntityShape::Circle => hit_test_circle(local),
        EntityShape::Triangle => hit_test_triangle(local),
        EntityShape::Square => hit_test_square(local),
        EntityShape::Diamond => hit_test_diamond(local),
        EntityShape::Hexagon => hit_test_hexagon(local),
        EntityShape::Star => hit_test_star(local),
    }
}

/// Circle hit test: point within radius 1 in local space.
fn hit_test_circle(local: [f32; 2]) -> bool {
    local[0] * local[0] + local[1] * local[1] < 1.0
}

/// Equilateral triangle hit test.
/// Vertices at: top(0,1), bottom-right(√3/2,-0.5), bottom-left(-√3/2,-0.5).
/// Uses barycentric edge tests.
fn hit_test_triangle(local: [f32; 2]) -> bool {
    let sqrt3: f32 = 1.7320508;
    let ax = 0.0;
    let ay = 1.0;
    let bx = sqrt3 * 0.5;
    let by = -0.5;
    let cx = -sqrt3 * 0.5;
    let cy = -0.5;
    let px = local[0];
    let py = local[1];
    let e0 = (bx - ax) * (py - ay) - (by - ay) * (px - ax);
    let e1 = (cx - bx) * (py - by) - (cy - by) * (px - bx);
    let e2 = (ax - cx) * (py - cy) - (ay - cy) * (px - cx);
    (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) || (e0 <= 0.0 && e1 <= 0.0 && e2 <= 0.0)
}

/// Square hit test: axis-aligned, fills [-1,1]².
fn hit_test_square(local: [f32; 2]) -> bool {
    local[0].abs() < 1.0 && local[1].abs() < 1.0
}

/// Diamond hit test: |x| + |y| < 1.
fn hit_test_diamond(local: [f32; 2]) -> bool {
    local[0].abs() + local[1].abs() < 1.0
}

/// Flat-topped regular hexagon hit test inscribed in unit circle.
/// Apothem = √3/2 ≈ 0.8660254.
fn hit_test_hexagon(local: [f32; 2]) -> bool {
    let x = local[0].abs();
    let y = local[1].abs();
    let hex = if x * 0.8660254 + y * 0.5 > y {
        x * 0.8660254 + y * 0.5
    } else {
        y
    };
    hex < 0.8660254
}

/// 5-pointed star hit test inscribed in unit circle.
/// Uses the same angular sector logic as the shader SDF.
fn hit_test_star(local: [f32; 2]) -> bool {
    let r = (local[0] * local[0] + local[1] * local[1]).sqrt();
    if r < 0.0001 {
        return true; // center is inside
    }
    let pi = std::f32::consts::PI;
    let mut a = local[0].atan2(local[1]) + pi; // [0, 2π]
    let seg = (a * 2.5 / pi).floor();
    a -= seg * 1.25663706; // 2π/5
    a -= 0.62831853; // π/5
    if a > 0.31415926 {
        // π/10
        a = 0.62831853 - a;
    }
    let inner_r: f32 = 0.381966;
    let star_r = inner_r / (inner_r * a.cos() + 0.9238795 * a.sin());
    r < star_r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_data_is_pod() {
        fn assert_pod<T: bytemuck::Pod>() {}
        assert_pod::<InstanceData>();
    }

    #[test]
    fn test_vertex_is_pod() {
        fn assert_pod<T: bytemuck::Pod>() {}
        assert_pod::<Vertex>();
    }

    #[test]
    fn test_instance_data_size() {
        // InstanceData: [f32;2]=8 + f32=4 + f32(pad)=4 + [f32;3]=12 + f32=4 + u32=4 + u32(pad)=4 = 40 bytes
        assert_eq!(std::mem::size_of::<InstanceData>(), 40);
    }

    #[test]
    fn test_hit_test_finds_entity() {
        let entities = vec![ObservableProperties {
            position: [5.0, 3.0],
            size: 2.0,
            ..Default::default()
        }];
        let result = hit_test_shape([5.5, 3.5], &entities);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_hit_test_misses_far_entity() {
        let entities = vec![ObservableProperties {
            position: [0.0, 0.0],
            size: 1.0,
            ..Default::default()
        }];
        let result = hit_test_shape([100.0, 100.0], &entities);
        assert_eq!(result, None);
    }

    #[test]
    fn test_circle_hit_center() {
        assert!(hit_test_circle([0.0, 0.0]));
    }

    #[test]
    fn test_circle_hit_inside() {
        assert!(hit_test_circle([0.5, 0.5]));
    }

    #[test]
    fn test_circle_miss_outside() {
        assert!(!hit_test_circle([1.0, 1.0]));
    }

    #[test]
    fn test_triangle_hit_center() {
        assert!(hit_test_triangle([0.0, 0.0]));
    }

    #[test]
    fn test_triangle_hit_top_vertex() {
        assert!(hit_test_triangle([0.0, 0.9]));
    }

    #[test]
    fn test_triangle_miss_outside() {
        assert!(!hit_test_triangle([0.9, 0.9]));
    }

    #[test]
    fn test_square_hit_center() {
        assert!(hit_test_square([0.0, 0.0]));
    }

    #[test]
    fn test_square_hit_edge() {
        assert!(hit_test_square([0.99, 0.99]));
    }

    #[test]
    fn test_square_miss_outside() {
        assert!(!hit_test_square([1.1, 0.0]));
    }

    #[test]
    fn test_diamond_hit_center() {
        assert!(hit_test_diamond([0.0, 0.0]));
    }

    #[test]
    fn test_diamond_hit_on_axis() {
        assert!(hit_test_diamond([0.5, 0.0]));
    }

    #[test]
    fn test_diamond_miss_corner() {
        assert!(!hit_test_diamond([0.8, 0.8]));
    }

    #[test]
    fn test_hexagon_hit_center() {
        assert!(hit_test_hexagon([0.0, 0.0]));
    }

    #[test]
    fn test_hexagon_hit_inside() {
        assert!(hit_test_hexagon([0.3, 0.3]));
    }

    #[test]
    fn test_hexagon_miss_outside() {
        assert!(!hit_test_hexagon([1.0, 1.0]));
    }

    #[test]
    fn test_star_hit_center() {
        assert!(hit_test_star([0.0, 0.0]));
    }

    #[test]
    fn test_star_hit_tip() {
        assert!(hit_test_star([0.0, 0.9]));
    }

    #[test]
    fn test_star_miss_outside() {
        assert!(!hit_test_star([1.0, 1.0]));
    }

    #[test]
    fn test_shape_dispatch_circle() {
        assert!(hit_test_shape_local([0.0, 0.0], EntityShape::Circle));
        assert!(!hit_test_shape_local([1.0, 1.0], EntityShape::Circle));
    }

    #[test]
    fn test_shape_dispatch_triangle() {
        assert!(hit_test_shape_local([0.0, 0.0], EntityShape::Triangle));
        assert!(!hit_test_shape_local([0.9, 0.9], EntityShape::Triangle));
    }

    #[test]
    fn test_shape_dispatch_square() {
        assert!(hit_test_shape_local([0.5, 0.5], EntityShape::Square));
        assert!(!hit_test_shape_local([1.5, 0.0], EntityShape::Square));
    }

    #[test]
    fn test_shape_dispatch_diamond() {
        assert!(hit_test_shape_local([0.3, 0.3], EntityShape::Diamond));
        assert!(!hit_test_shape_local([0.8, 0.8], EntityShape::Diamond));
    }

    #[test]
    fn test_shape_dispatch_hexagon() {
        assert!(hit_test_shape_local([0.0, 0.5], EntityShape::Hexagon));
        assert!(!hit_test_shape_local([1.0, 1.0], EntityShape::Hexagon));
    }

    #[test]
    fn test_shape_dispatch_star() {
        assert!(hit_test_shape_local([0.0, 0.5], EntityShape::Star));
        assert!(!hit_test_shape_local([1.0, 1.0], EntityShape::Star));
    }

    #[test]
    fn test_hit_test_with_different_shapes() {
        let entities = vec![
            ObservableProperties {
                position: [0.0, 0.0],
                size: 1.0,
                shape: EntityShape::Circle,
                ..Default::default()
            },
            ObservableProperties {
                position: [3.0, 0.0],
                size: 1.0,
                shape: EntityShape::Square,
                ..Default::default()
            },
            ObservableProperties {
                position: [6.0, 0.0],
                size: 1.0,
                shape: EntityShape::Star,
                ..Default::default()
            },
        ];
        let circle_hit = hit_test_shape([0.0, 0.0], &entities);
        assert_eq!(circle_hit, Some(0));

        let square_hit = hit_test_shape([3.5, 0.5], &entities);
        assert_eq!(square_hit, Some(1));

        let star_hit = hit_test_shape([6.0, 0.5], &entities);
        assert_eq!(star_hit, Some(2));
    }
}
