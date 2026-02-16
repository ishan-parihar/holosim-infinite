// Connection Line Shader - Renders hierarchical connection lines
//
// Phase 4: Hierarchy Visualization
//
// From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md:
// "Visualize composition hierarchy, parent-child relationships, and environment relationships."
//
// Connection types:
// - Type 0 (parent-child): Blue tint (0.2, 0.4, 1.0)
// - Type 1 (composition): Green tint (0.2, 1.0, 0.4)
// - Type 2 (environment): Yellow tint (1.0, 0.8, 0.2)

struct CameraUniform {
    view_proj: mat4x4<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) line_vertex: vec3<f32>,  // 0.0 for start, 1.0 for end
};

struct ConnectionInput {
    @location(1) from_position: vec3<f32>,  // Start position (parent/container/environment)
    @location(2) to_position: vec3<f32>,    // End position (child/component/entity)
    @location(3) connection_type: u32,
    @location(4) intensity: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) connection_type: u32,
    @location(1) intensity: f32,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    connection: ConnectionInput,
) -> VertexOutput {
    // Interpolate between from_position and to_position based on line_vertex
    // line_vertex.x is 0.0 for start vertex, 1.0 for end vertex
    let world_pos = mix(connection.from_position, connection.to_position, vertex.line_vertex.x);
    
    // Transform to clip space
    let clip_pos = camera.view_proj * vec4<f32>(world_pos, 1.0);
    
    var output: VertexOutput;
    output.position = clip_pos;
    output.connection_type = connection.connection_type;
    output.intensity = connection.intensity;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Color based on connection type
    var color: vec3<f32>;
    
    switch (input.connection_type) {
        case 0u: {
            // Parent-child: Blue
            color = vec3<f32>(0.2, 0.4, 1.0);
        }
        case 1u: {
            // Composition: Green
            color = vec3<f32>(0.2, 1.0, 0.4);
        }
        case 2u: {
            // Environment: Yellow
            color = vec3<f32>(1.0, 0.8, 0.2);
        }
        default: {
            // Unknown: Gray
            color = vec3<f32>(0.5, 0.5, 0.5);
        }
    }
    
    // Alpha based on intensity (0.3 to 0.5 range for subtle visualization)
    let alpha = 0.3 + (input.intensity * 0.2);
    
    return vec4<f32>(color, alpha);
}
