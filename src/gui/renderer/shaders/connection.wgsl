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
// - Type 3 (phase coherence): Coherence-gradient cyan/magenta family
// - Type 4 (entanglement): Coherence-gradient indigo/gold family

struct CameraUniform {
    view_proj: mat4x4<f32>,
    warm_cool_bias: f32,
    saturation: f32,
    phase_tint_boost: f32,
    grade_strength: f32,
    time: f32,
    _pad0: f32,  // Changed from vec3 to individual floats for 16-byte alignment
    _pad1: f32,
    _pad2: f32,
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
    @location(5) phase_delta: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) connection_type: u32,
    @location(1) intensity: f32,
    @location(2) phase_delta: f32,
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
    output.phase_delta = connection.phase_delta;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Color based on connection type
    var base_color: vec3<f32>;
    let coherence = clamp(input.intensity, 0.0, 1.0);
    let phase_delta = clamp(input.phase_delta, 0.0, 1.0);
    let is_phase_like = (input.connection_type == 3u || input.connection_type == 4u);
    
    switch (input.connection_type) {
        case 0u: {
            // Parent-child: Blue
            base_color = vec3<f32>(0.2, 0.4, 1.0);
        }
        case 1u: {
            // Composition: Green
            base_color = vec3<f32>(0.2, 1.0, 0.4);
        }
        case 2u: {
            // Environment: Yellow
            base_color = vec3<f32>(1.0, 0.8, 0.2);
        }
        case 3u: {
            // Phase coherence: low coherence -> violet, high coherence -> cyan
            let low = vec3<f32>(0.66, 0.32, 1.0);
            let high = vec3<f32>(0.28, 0.96, 1.0);
            base_color = mix(low, high, coherence);
        }
        case 4u: {
            // Entanglement: low coherence -> indigo, high coherence -> warm gold
            let low = vec3<f32>(0.25, 0.38, 1.0);
            let high = vec3<f32>(1.0, 0.86, 0.45);
            base_color = mix(low, high, coherence);
        }
        default: {
            // Unknown: Gray
            base_color = vec3<f32>(0.5, 0.5, 0.5);
        }
    }

    var graded_color = base_color;

    let cool_tint = vec3<f32>(0.90, 0.98, 1.08);
    let warm_tint = vec3<f32>(1.08, 0.98, 0.90);
    let tint = mix(cool_tint, warm_tint, camera.warm_cool_bias);
    graded_color = graded_color * tint;

    if (is_phase_like) {
        graded_color = mix(
            graded_color,
            vec3<f32>(0.85, 1.0, 1.0),
            camera.phase_tint_boost,
        );
    }

    var pulse = 1.0;
    if (is_phase_like) {
        let freq = 2.5 + coherence * 3.0;
        let phase_offset = phase_delta * 6.2831853;
        let wave = sin(camera.time * freq + phase_offset);
        let pulse_strength = mix(0.18, 0.45, coherence);
        pulse = 1.0 + wave * pulse_strength;
        let delta_mod = 1.0 - 0.35 * phase_delta;
        pulse = pulse * delta_mod;
    }

    let luma = dot(graded_color, vec3<f32>(0.299, 0.587, 0.114));
    let color_sat = mix(vec3<f32>(luma), graded_color, camera.saturation);
    let color = clamp(
        mix(base_color, color_sat, camera.grade_strength),
        vec3<f32>(0.0),
        vec3<f32>(1.0),
    ) * pulse;
    let final_color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    
    // Alpha based on intensity (0.3 to 0.5 range for subtle visualization)
    var alpha = 0.3 + (input.intensity * 0.2);
    if (is_phase_like) {
        alpha = clamp(alpha * (0.95 + 0.35 * (pulse - 0.75)), 0.12, 0.85);
    }
    
    return vec4<f32>(final_color, alpha);
}
