// Civilization Shader - Visualizes settlements, trade routes, and population density
//
// Phase 5: Civilization System
// Renders:
// - Settlements (points with size based on population)
// - Trade routes (lines with thickness based on volume)
// - Population density (heat map)

// ============================================================================
// SETTLEMENT RENDERING
// ============================================================================

struct SettlementUniforms {
    view_proj: mat4x4<f32>,
    time: f32,
    highlight_id: u32,
    _padding: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: SettlementUniforms;

struct SettlementVertex {
    @location(0) position: vec3<f32>,
    @location(1) size: f32,
    @location(2) tech_level: f32,
    @location(3) settlement_type: f32, // 0=Hamlet, 1=Village, 2=Town, 3=City, 4=Metropolis, 5=Megalopolis
    @location(4) _padding: vec2<f32>,
};

struct SettlementOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tech_level: f32,
    @location(2) settlement_type: f32,
    @location(3) is_highlighted: f32,
};

@vertex
fn vs_settlement(
    vertex: SettlementVertex,
    @builtin(vertex_index) vertex_index: u32,
) -> SettlementOutput {
    // Generate billboard quad vertices
    let quad_positions = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0,  1.0),
    );

    let quad_index = vertex_index % 4u;
    let quad_pos = quad_positions[quad_index];

    // Billboard scaling based on settlement size
    let scale = vertex.size * (1.0 + vertex.tech_level * 0.5);
    let offset = quad_pos * scale;

    var output: SettlementOutput;
    output.clip_position = uniforms.view_proj * vec4<f32>(
        vertex.position + vec3<f32>(offset, 0.0),
        1.0
    );

    // UV for fragment shader
    output.uv = quad_pos;

    // Pass through for coloring
    output.tech_level = vertex.tech_level;
    output.settlement_type = vertex.settlement_type;

    // Highlight check
    output.is_highlighted = select(0.0, 1.0, vertex_index == uniforms.highlight_id);

    return output;
}

@fragment
fn fs_settlement(input: SettlementOutput) -> @location(0) vec4<f32> {
    // Distance from center for circular shape
    let dist = length(input.uv);

    // Discard pixels outside circle
    if (dist > 1.0) {
        discard;
    }

    // Smooth edge
    let edge = 1.0 - smoothstep(0.85, 1.0, dist);

    // Color based on settlement type
    let type_colors = array<vec3<f32>, 6>(
        vec3<f32>(0.6, 0.5, 0.4),  // Hamlet - earth brown
        vec3<f32>(0.5, 0.7, 0.5),  // Village - forest green
        vec3<f32>(0.5, 0.6, 0.8),  // Town - sky blue
        vec3<f32>(0.9, 0.8, 0.3),  // City - gold
        vec3<f32>(0.9, 0.5, 0.2),  // Metropolis - orange
        vec3<f32>(0.9, 0.2, 0.3),  // Megalopolis - red
    );

    let type_index = min(u32(input.settlement_type), 5u);
    var color = type_colors[type_index];

    // Tech level brightening
    color = mix(color, vec3<f32>(1.0), input.tech_level * 0.3);

    // Pulsing animation for highlights
    if (input.is_highlighted > 0.5) {
        let pulse = 0.5 + 0.5 * sin(uniforms.time * 3.0);
        color = mix(color, vec3<f32>(1.0, 1.0, 0.0), pulse * 0.5);
    }

    // Inner glow
    let inner_glow = 1.0 - smoothstep(0.0, 0.3, dist);
    color = mix(color, vec3<f32>(1.0), inner_glow * 0.3);

    return vec4<f32>(color, edge);
}

// ============================================================================
// TRADE ROUTE RENDERING
// ============================================================================

struct TradeRouteVertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) volume: f32,
    @location(3) _padding: f32,
};

struct TradeRouteOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) volume: f32,
};

@vertex
fn vs_trade_route(vertex: TradeRouteVertex) -> TradeRouteOutput {
    var output: TradeRouteOutput;
    output.clip_position = uniforms.view_proj * vec4<f32>(vertex.position, 1.0);
    output.color = vertex.color;
    output.volume = vertex.volume;
    return output;
}

@fragment
fn fs_trade_route(input: TradeRouteOutput) -> @location(0) vec4<f32> {
    // Color based on trade volume
    let intensity = 0.3 + input.volume * 0.7;
    let color = input.color * intensity;

    // Animated dash pattern would require additional uniform
    return vec4<f32>(color, 0.6);
}

// ============================================================================
// POPULATION DENSITY HEATMAP
// ============================================================================

struct DensityVertex {
    @location(0) position: vec3<f32>,
    @location(1) density: f32,
    @location(2) _padding: vec2<f32>,
};

struct DensityOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) density: f32,
};

@vertex
fn vs_density(vertex: DensityVertex) -> DensityOutput {
    var output: DensityOutput;
    output.clip_position = uniforms.view_proj * vec4<f32>(vertex.position, 1.0);
    output.density = vertex.density;
    return output;
}

@fragment
fn fs_density(input: DensityOutput) -> @location(0) vec4<f32> {
    // Heat map color gradient
    // Low density: blue -> green -> yellow -> red : high density
    let d = input.density;

    var color: vec3<f32>;
    if (d < 0.25) {
        color = mix(
            vec3<f32>(0.0, 0.0, 0.5),  // Dark blue
            vec3<f32>(0.0, 0.5, 1.0),  // Light blue
            d * 4.0
        );
    } else if (d < 0.5) {
        color = mix(
            vec3<f32>(0.0, 0.5, 1.0),  // Light blue
            vec3<f32>(0.0, 1.0, 0.0),  // Green
            (d - 0.25) * 4.0
        );
    } else if (d < 0.75) {
        color = mix(
            vec3<f32>(0.0, 1.0, 0.0),  // Green
            vec3<f32>(1.0, 1.0, 0.0),  // Yellow
            (d - 0.5) * 4.0
        );
    } else {
        color = mix(
            vec3<f32>(1.0, 1.0, 0.0),  // Yellow
            vec3<f32>(1.0, 0.0, 0.0),  // Red
            (d - 0.75) * 4.0
        );
    }

    return vec4<f32>(color, 0.4 * d);
}

// ============================================================================
// CIVILIZATION BOUNDARIES
// ============================================================================

struct BoundaryVertex {
    @location(0) position: vec3<f32>,
    @location(1) polarization: f32,  // -1 (STS) to +1 (STO)
    @location(2) _padding: vec2<f32>,
};

struct BoundaryOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) polarization: f32,
};

@vertex
fn vs_boundary(vertex: BoundaryVertex) -> BoundaryOutput {
    var output: BoundaryOutput;
    output.clip_position = uniforms.view_proj * vec4<f32>(vertex.position, 1.0);
    output.polarization = vertex.polarization;
    return output;
}

@fragment
fn fs_boundary(input: BoundaryOutput) -> @location(0) vec4<f32> {
    // STS: Red, STO: Blue, Mixed: Purple
    let p = (input.polarization + 1.0) * 0.5; // Normalize to 0-1

    let sts_color = vec3<f32>(0.8, 0.2, 0.2);
    let sto_color = vec3<f32>(0.2, 0.4, 0.9);

    let color = mix(sts_color, sto_color, p);

    return vec4<f32>(color, 0.5);
}
