//! Logarithmic depth shader for multi-scale rendering
//!
//! This shader implements logarithmic depth buffering to handle 61 orders of magnitude
//! (10^-35m to 10^26m) without z-fighting issues.

/// Logarithmic depth shader
pub const LOGARITHMIC_DEPTH_SHADER: &str = r#"
struct Uniforms {
    screen_size: vec2<f32>,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
    log_depth_scale: f32,
    log_depth_offset: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) log_depth: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) log_depth: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    // Apply camera transform (zoom and offset)
    let world_pos = (input.position * uniforms.zoom) + vec2<f32>(uniforms.offset_x, uniforms.offset_y);
    
    // Convert world coordinates to clip space
    let screen_pos = (world_pos - uniforms.screen_size * 0.5) / uniforms.screen_size * 2.0;
    
    // Calculate logarithmic depth
    let log_z = input.log_depth * uniforms.log_depth_scale + uniforms.log_depth_offset;
    
    var output: VertexOutput;
    output.clip_position = vec4<f32>(screen_pos.x, -screen_pos.y, log_z, 1.0);
    output.color = input.color;
    output.uv = input.position;
    output.log_depth = input.log_depth;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple color output for now
    return input.color;
}
"#;

/// Shader with smooth LOD transitions
pub const LOD_SHADER: &str = r#"
struct Uniforms {
    screen_size: vec2<f32>,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
    lod_factor: f32,
    log_depth_scale: f32,
    log_depth_offset: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) log_depth: f32,
    @location(3) radius: f32,
    @location(4) lod_level: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) log_depth: f32,
    @location(3) radius: f32,
    @location(4) lod_level: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    // Apply camera transform (zoom and offset)
    let world_pos = (input.position * uniforms.zoom) + vec2<f32>(uniforms.offset_x, uniforms.offset_y);
    
    // Convert world coordinates to clip space
    let screen_pos = (world_pos - uniforms.screen_size * 0.5) / uniforms.screen_size * 2.0;
    
    // Calculate logarithmic depth
    let log_z = input.log_depth * uniforms.log_depth_scale + uniforms.log_depth_offset;
    
    var output: VertexOutput;
    output.clip_position = vec4<f32>(screen_pos.x, -screen_pos.y, log_z, 1.0);
    output.color = input.color;
    output.uv = input.position;
    output.log_depth = input.log_depth;
    output.radius = input.radius;
    output.lod_level = input.lod_level;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate screen-space radius
    let radius_screen = input.radius * uniforms.zoom;
    
    // LOD-based rendering
    let lod_factor = clamp(input.lod_level, 0.0, 1.0);
    
    // Distance from center (normalized)
    let dist = length(input.uv) / radius_screen;
    
    if (dist > 1.0) {
        discard;
    }
    
    // Anti-aliasing with LOD-aware edge width
    let edge_width = 2.0 * uniforms.zoom * (1.0 - lod_factor * 0.5);
    let alpha = 1.0 - smoothstep(1.0 - edge_width / radius_screen, 1.0, dist);
    
    // Reduce detail at lower LOD levels
    let detail_factor = 1.0 - lod_factor * 0.5;
    let color = input.color * detail_factor;
    
    return vec4<f32>(color.rgb, input.color.a * alpha);
}
"#;
