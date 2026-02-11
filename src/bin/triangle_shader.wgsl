// Triangle Shader - Basic vertex/fragment shader for SDL2 test
// Renders a simple colored triangle to verify WGPU rendering works

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Triangle vertices
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    
    // Vertex colors
    switch in_vertex_index {
        case 0u: { out.color = vec3<f32>(1.0, 0.0, 0.0); } // Red
        case 1u: { out.color = vec3<f32>(0.0, 1.0, 0.0); } // Green
        case 2u: { out.color = vec3<f32>(0.0, 0.0, 1.0); } // Blue
        default: { out.color = vec3<f32>(1.0, 1.0, 1.0); } // White
    }
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
