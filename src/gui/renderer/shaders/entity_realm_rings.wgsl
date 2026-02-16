//! Entity Shader with Realm Ring Visualization - Phase 2
//!
//! Phase 2: Realm Visualization
//!
//! This shader visualizes the 7-layer holonic architecture as concentric rings
//! around each entity, making the involution sequence explicitly visible.
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md Phase 2:
//! "Create 7-layer architectural visualization as concentric rings"
//! "Implement realm intensity animations"
//! "Add realm transition effects"
//!
//! The 7 realms are visualized as rings from center outward:
//! - Ring 1 (center): Violet Realm - Infinity
//! - Ring 2: Indigo Realm - IntelligentInfinity
//! - Ring 3: Blue Realm - Love/Logos
//! - Ring 4: Green Realm - Light/Love
//! - Ring 5: Yellow Realm - Spectrum + Veil
//! - Ring 6: Orange Realm - Galactic spectrum
//! - Ring 7 (outer): Red Realm - Archetypical Mind

// Camera uniform buffer
struct CameraUniforms {
    view_projection: mat4x4<f32>,
    time: f32,  // Time for animations
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    // Position (16 bytes: 12 + 4 padding)
    @location(1) position: vec3<f32>,

    // Realm intensities - 7 layers of holonic architecture (28 bytes)
    @location(2) realm_intensities_low: vec4<f32>,  // violet, indigo, blue, green
    @location(3) realm_intensities_high: vec3<f32>, // yellow, orange, red

    // Consciousness data (8 bytes)
    @location(4) consciousness_data: vec2<f32>,  // consciousness_level, polarization

    // Spectrum data (12 bytes)
    @location(5) spectrum_data: vec3<f32>,  // space_time_ratio, time_space_ratio, veil_transparency

    // Evolution data (8 bytes)
    @location(6) evolution_progress: f32,
    @location(7) density_level_packed: u32,  // density_level (1-8)

    // Archetype summary (8 bytes)
    @location(8) archetype_intensity: f32,
    @location(9) archetype_activated: u32,

    // Entity type and size (8 bytes)
    @location(10) entity_type: u32,
    @location(11) size: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) veil_transparency: f32,
    @location(3) consciousness_level: f32,
    
    // Realm intensities for ring visualization (32 bytes)
    @location(4) realm_violet: f32,
    @location(5) realm_indigo: f32,
    @location(6) realm_blue: f32,
    @location(7) realm_green: f32,
    @location(8) realm_yellow: f32,
    @location(9) realm_orange: f32,
    @location(10) realm_red: f32,
    
    @location(11) size: f32,
    @location(12) density_level: f32,
};

// Realm colors for the 7-layer architecture
fn get_realm_color(realm_index: i32) -> vec3<f32> {
    switch (realm_index) {
        case 0: { return vec3<f32>(0.5, 0.0, 1.0); }  // Violet
        case 1: { return vec3<f32>(0.3, 0.0, 0.5); }  // Indigo
        case 2: { return vec3<f32>(0.0, 0.0, 1.0); }  // Blue
        case 3: { return vec3<f32>(0.0, 1.0, 0.0); }  // Green
        case 4: { return vec3<f32>(1.0, 1.0, 0.0); }  // Yellow
        case 5: { return vec3<f32>(1.0, 0.5, 0.0); }  // Orange
        case 6: { return vec3<f32>(1.0, 0.0, 0.0); }  // Red
        default: { return vec3<f32>(0.5, 0.5, 0.5); }  // Gray (fallback)
    }
}

// Realm ring radii (normalized to entity size)
// Each realm appears as a ring at a specific distance from center
fn get_realm_ring_radius(realm_index: i32) -> f32 {
    switch (realm_index) {
        case 0: { return 0.15; }  // Violet - innermost
        case 1: { return 0.30; }  // Indigo
        case 2: { return 0.45; }  // Blue
        case 3: { return 0.60; }  // Green
        case 4: { return 0.75; }  // Yellow
        case 5: { return 0.85; }  // Orange
        case 6: { return 0.95; }  // Red - outermost
        default: { return 1.0; }   // Fallback
    }
}

// Realm ring thickness
fn get_realm_ring_thickness() -> f32 {
    return 0.04;  // 4% of entity size
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    // Scale vertex by instance size and add instance position
    let world_position = instance.position + (vertex.position * instance.size);

    // Apply camera view-projection matrix
    out.clip_position = camera.view_projection * vec4<f32>(world_position, 1.0);

    // Extract realm intensities
    let violet_intensity = instance.realm_intensities_low.x;
    let indigo_intensity = instance.realm_intensities_low.y;
    let blue_intensity = instance.realm_intensities_low.z;
    let green_intensity = instance.realm_intensities_low.w;
    let yellow_intensity = instance.realm_intensities_high.x;
    let orange_intensity = instance.realm_intensities_high.y;
    let red_intensity = instance.realm_intensities_high.z;

    // Mix colors based on 7 realm intensities (for main circle)
    var mixed_color = vec3<f32>(0.0, 0.0, 0.0);
    var total_intensity = 0.0;

    // Weight each realm color by its intensity
    for (var i = 0; i < 7; i++) {
        let intensity = select(
            select(
                select(
                    select(
                        select(
                            select(
                                red_intensity,
                                orange_intensity,
                                i == 5
                            ),
                            yellow_intensity,
                            i == 4
                        ),
                        green_intensity,
                        i == 3
                    ),
                    blue_intensity,
                    i == 2
                ),
                indigo_intensity,
                i == 1
            ),
            violet_intensity,
            i == 0
        );

        let realm_color = get_realm_color(i);
        mixed_color += realm_color * intensity;
        total_intensity += intensity;
    }

    // Normalize the mixed color
    if (total_intensity > 0.001) {
        mixed_color = mixed_color / total_intensity;
    } else {
        // Fallback to violet if no intensities
        mixed_color = get_realm_color(0);
    }

    // Apply polarization tint overlay
    let polarization = instance.consciousness_data.y;
    let polarization_magnitude = abs(polarization);

    if (polarization > 0.0) {
        // STO: blue tint
        mixed_color = mix(mixed_color, vec3<f32>(0.0, 0.5, 1.0), polarization_magnitude * 0.3);
    } else if (polarization < 0.0) {
        // STS: red tint
        mixed_color = mix(mixed_color, vec3<f32>(1.0, 0.0, 0.0), polarization_magnitude * 0.3);
    }

    // Apply consciousness-based brightness
    let consciousness_level = instance.consciousness_data.x;
    let brightness_boost = 0.5 + (consciousness_level * 0.5);
    mixed_color = mixed_color * brightness_boost;

    // Clamp color to valid range
    mixed_color = clamp(mixed_color, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 1.0));

    // Set output color with full alpha
    out.color = vec4<f32>(mixed_color, 1.0);

    // Pass local position for circle rendering
    out.local_pos = vertex.position.xy;

    // Pass veil transparency for fragment shader effects
    out.veil_transparency = instance.spectrum_data.z;

    // Pass consciousness level for glow effects
    out.consciousness_level = consciousness_level;

    // Pass realm intensities for ring visualization
    out.realm_violet = violet_intensity;
    out.realm_indigo = indigo_intensity;
    out.realm_blue = blue_intensity;
    out.realm_green = green_intensity;
    out.realm_yellow = yellow_intensity;
    out.realm_orange = orange_intensity;
    out.realm_red = red_intensity;

    // Pass size and density
    out.size = instance.size;
    out.density_level = f32(instance.density_level_packed & 0xFF);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Create circle shape using distance from center
    let dist = length(in.local_pos);

    // Hard circle edge for main entity body
    if (dist > 1.0) {
        discard;
    }

    // Phase 2: Realm Ring Visualization
    // Add concentric rings for each of the 7 realms
    var ring_color = vec3<f32>(0.0, 0.0, 0.0);
    var ring_intensity = 0.0;
    
    // Array of realm intensities for iteration
    let realm_intensities = array<f32, 7>(
        in.realm_violet,
        in.realm_indigo,
        in.realm_blue,
        in.realm_green,
        in.realm_yellow,
        in.realm_orange,
        in.realm_red
    );
    
    // Check each realm ring
    for (var realm_idx = 0; realm_idx < 7; realm_idx++) {
        let ring_radius = get_realm_ring_radius(i32(realm_idx));
        let ring_thickness = get_realm_ring_thickness();
        let intensity = realm_intensities[realm_idx];
        
        // Check if we're within this ring's area
        let dist_from_ring = abs(dist - ring_radius);
        
        // Ring intensity based on distance from ring center and realm intensity
        let ring_width = ring_thickness / 2.0;
        
        if (dist_from_ring < ring_width && intensity > 0.01) {
            // Smooth ring edge
            let edge_factor = 1.0 - (dist_from_ring / ring_width);
            edge_factor = smoothstep(0.0, 1.0, edge_factor);
            
            // Add this ring's contribution
            let realm_color = get_realm_color(i32(realm_idx));
            ring_color += realm_color * edge_factor * intensity;
            ring_intensity += edge_factor * intensity;
        }
    }
    
    // Animate rings with subtle pulsing
    let time = camera.time;
    let pulse_speed = 2.0;
    let pulse_amplitude = 0.1;
    let pulse = 1.0 + sin(time * pulse_speed) * pulse_amplitude;
    ring_intensity *= pulse;
    
    // Mix ring color with main entity color
    // Apply depth gradient based on consciousness level
    let gradient = 1.0 - (dist * (0.2 + in.consciousness_level * 0.1));
    
    // Apply veil transparency to alpha
    let veil_factor = 0.5 + (in.veil_transparency * 0.5);
    let alpha = gradient * veil_factor;
    
    // Combine main color and ring colors
    // Ring colors overlay the main body
    let ring_blend = saturate(ring_intensity);
    let final_rgb = mix(in.color.rgb * gradient, ring_color, ring_blend * 0.5);
    
    // Add glow for higher consciousness entities and strong realm activations
    let consciousness_glow = in.consciousness_level * 0.2;
    let realm_glow = ring_intensity * 0.3;
    let glow = consciousness_glow + realm_glow;
    
    final_rgb += vec3<f32>(glow, glow, glow);
    
    // Clamp final color
    let final_color = vec4<f32>(
        clamp(final_rgb, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 1.0)),
        clamp(alpha, 0.0, 1.0)
    );

    return final_color;
}
