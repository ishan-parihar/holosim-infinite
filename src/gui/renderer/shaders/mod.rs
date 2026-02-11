//! Shader Module - WGSL shaders for rendering and compute
//!
//! From GUI_IMPLEMENTATION_ROADMAP.md Phase 4 Week 8:
//! "Advanced shaders - 3D sphere rendering with lighting, normal mapping, specular highlights"
//! "Compute shaders - Collective dynamics calculations, Resonance field computation, Particle system updates"

pub mod compute;
pub mod entity;

pub use compute::{
    CollectiveData, EntityData, ParticleData, ParticleEmitter, ResonanceField, SimulationParams,
    COMPUTE_SHADER,
};
pub use entity::{generate_sphere_geometry, ENTITY_SHADER};

/// Basic 2D circle rendering shader (from Phase 1 Week 1)
pub const BASIC_SHADER: &str = r#"
struct Uniforms {
    screen_size: vec2<f32>,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) center: vec2<f32>,
    @location(3) radius: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let world_pos = (input.position * uniforms.zoom) + vec2<f32>(uniforms.offset_x, uniforms.offset_y);
    let screen_pos = (world_pos - uniforms.screen_size * 0.5) / uniforms.screen_size * 2.0;
    let clip_pos = vec4<f32>(screen_pos.x, -screen_pos.y, 0.0, 1.0);

    var output: VertexOutput;
    output.clip_position = clip_pos;
    output.color = input.color;
    output.uv = input.position;
    output.center = input.position;
    output.radius = 10.0;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_pos = input.uv * uniforms.zoom + vec2<f32>(uniforms.offset_x, uniforms.offset_y);
    let center_screen = input.center * uniforms.zoom + vec2<f32>(uniforms.offset_x, uniforms.offset_y);
    let dist = length(pixel_pos - center_screen);
    let radius_screen = input.radius * uniforms.zoom;

    if (dist > radius_screen) {
        discard;
    }

    let edge_width = 2.0 * uniforms.zoom;
    let alpha = 1.0 - smoothstep(radius_screen - edge_width, radius_screen, dist);

    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}
"#;
