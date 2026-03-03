//! Entity Shader - Renders entities with enhanced visualization
//!
//! Phase 1: Enhanced Instance Data Visualization
//!
//! This shader visualizes the 7-layer holonic architecture of each entity:
//! - Realm-based color mixing (7 intensities: violet through red)
//! - Polarization tint overlay (blue for STO, red for STS)
//! - Consciousness-based glow/brightness
//! - Veil transparency effects
//!
//! Phase 16: Holographic Entity Geometry
//! - Full 22-archetype storage buffer access
//! - Archetype interference patterns for geometry
//! - Field-based emission effects
//! - Consciousness-as-light visualization
//!
//! From HOLOGRAPHIC_ARCHITECTURE_AUDIT_REPORT.md:
//! "Transform the visualization to make the most critical aspects of the
//! entity architecture visible beyond just 'colorful dots'."

// Camera uniform buffer

struct CameraUniforms {
    view_projection: mat4x4<f32>,
    time: f32,  // Phase 2: Time for animations
    morphology_mode: f32,
    selection_enabled: f32,
    selection_radius: f32,
    selection_position: vec3<f32>,
};


@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

// Phase 16: Archetype storage buffer (22 activations per entity)
// Group 1, Binding 1 - matches archetype_bind_group_layout
struct ArchetypeData {
    activations: array<f32, 22>,
}

@group(1) @binding(1)
var<storage, read> archetype_data: array<ArchetypeData>;

// Phase 16: Archetype interference pattern functions
// These compute holographic geometry from the 22 archetype activations

// Archetype categories (from COSMOLOGICAL-ARCHITECTURE.md):
// A0-A6: Mind archetypes (Matrix of the Mind)
// A7-A13: Body archetypes (Matrix of the Body)  
// A14-A20: Spirit archetypes (Matrix of the Spirit)
// A21: Choice (Free Will, Archetype 22)

fn compute_archetype_interference(activations: array<f32, 22>, angle: f32, time: f32) -> f32 {
    // Compute interference pattern from archetype wave superposition
    // NOTE: WGSL requires constant array indexing - loops unrolled manually
    var interference = 0.0;
    
    // Mind archetypes (A0-A6): Higher frequency, cognitive patterns
    interference += activations[0] * sin(angle * 3.0 + time * 0.7 + activations[0] * 6.2831853);
    interference += activations[1] * sin(angle * 3.5 + time * 0.7 + activations[1] * 6.2831853);
    interference += activations[2] * sin(angle * 4.0 + time * 0.7 + activations[2] * 6.2831853);
    interference += activations[3] * sin(angle * 4.5 + time * 0.7 + activations[3] * 6.2831853);
    interference += activations[4] * sin(angle * 5.0 + time * 0.7 + activations[4] * 6.2831853);
    interference += activations[5] * sin(angle * 5.5 + time * 0.7 + activations[5] * 6.2831853);
    interference += activations[6] * sin(angle * 6.0 + time * 0.7 + activations[6] * 6.2831853);
    
    // Body archetypes (A7-A13): Medium frequency, structural patterns
    interference += activations[7]  * cos(angle * 2.0 + time * 0.5 + activations[7]  * 6.2831853);
    interference += activations[8]  * cos(angle * 2.4 + time * 0.5 + activations[8]  * 6.2831853);
    interference += activations[9]  * cos(angle * 2.8 + time * 0.5 + activations[9]  * 6.2831853);
    interference += activations[10] * cos(angle * 3.2 + time * 0.5 + activations[10] * 6.2831853);
    interference += activations[11] * cos(angle * 3.6 + time * 0.5 + activations[11] * 6.2831853);
    interference += activations[12] * cos(angle * 4.0 + time * 0.5 + activations[12] * 6.2831853);
    interference += activations[13] * cos(angle * 4.4 + time * 0.5 + activations[13] * 6.2831853);
    
    // Spirit archetypes (A14-A20): Lower frequency, transcendental patterns
    interference += activations[14] * sin(angle * 1.5 + time * 0.3 + activations[14] * 6.2831853);
    interference += activations[15] * sin(angle * 1.8 + time * 0.3 + activations[15] * 6.2831853);
    interference += activations[16] * sin(angle * 2.1 + time * 0.3 + activations[16] * 6.2831853);
    interference += activations[17] * sin(angle * 2.4 + time * 0.3 + activations[17] * 6.2831853);
    interference += activations[18] * sin(angle * 2.7 + time * 0.3 + activations[18] * 6.2831853);
    interference += activations[19] * sin(angle * 3.0 + time * 0.3 + activations[19] * 6.2831853);
    interference += activations[20] * sin(angle * 3.3 + time * 0.3 + activations[20] * 6.2831853);
    
    // Choice (A21): Non-deterministic modulation
    interference += activations[21] * sin(angle * 7.0 + time * 1.2 + activations[21] * 3.14159);
    
    return interference / 22.0;
}

fn compute_mind_body_balance(activations: array<f32, 22>) -> f32 {
    let mind_sum = activations[0] + activations[1] + activations[2] + activations[3] 
                 + activations[4] + activations[5] + activations[6];
    let body_sum = activations[7] + activations[8] + activations[9] + activations[10]
                 + activations[11] + activations[12] + activations[13];
    return (mind_sum - body_sum) / 7.0;
}

fn compute_spirit_alignment(activations: array<f32, 22>) -> f32 {
    // Spirit archetypes (A14-A20) - fixed indexing for WGSL
    let spirit_sum = activations[14] + activations[15] + activations[16] + activations[17] 
                   + activations[18] + activations[19] + activations[20];
    return spirit_sum / 7.0;
}

fn compute_archetype_emission(activations: array<f32, 22>, consciousness: f32) -> vec3<f32> {
    // Emission color based on archetype dominance
    let mind_avg = (activations[0] + activations[1] + activations[2]) / 3.0;
    let body_avg = (activations[7] + activations[8] + activations[9]) / 3.0;
    let spirit_avg = (activations[14] + activations[15] + activations[16]) / 3.0;
    let choice = activations[21];
    
    // Mind -> Blue/Violet, Body -> Green/Yellow, Spirit -> Orange/Red, Choice -> White
    return vec3<f32>(
        body_avg * 0.3 + spirit_avg * 0.6 + choice * 0.4,
        mind_avg * 0.2 + body_avg * 0.5 + spirit_avg * 0.3,
        mind_avg * 0.7 + choice * 0.3
    ) * consciousness;
}

// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
};


struct InstanceInput {
    // Position (16 bytes)
    @location(1) position: vec3<f32>,

    // Realm intensities - 7 layers (28 bytes)
    @location(2) realm_intensities_low: vec4<f32>,  // violet, indigo, blue, green
    @location(3) realm_intensities_high: vec3<f32>, // yellow, orange, red

    // Consciousness data (8 bytes)
    @location(4) consciousness_data: vec2<f32>,  // consciousness_level, polarization

    // Spectrum data (12 bytes)
    @location(5) spectrum_data: vec3<f32>,  // space_time_ratio, time_space_ratio, veil_transparency

    // Evolution data (8 bytes)
    @location(6) evolution_progress: f32,
    @location(7) density_level_packed: u32,  // density_level (1-8)

    // Archetype summary (8 bytes) - FIXED ORDER TO MATCH EntityInstance
    @location(8) archetype_activated: u32,   // archetype_activated (u32) at offset 80
    @location(9) archetype_intensity: f32,   // archetype_intensity (f32) at offset 84

    // Entity type and size (8 bytes)
    @location(10) entity_type: u32,
    @location(11) size: f32,

    // Hierarchy data (8 bytes) - ADDED FROM RENDERER
    @location(12) parent_id: u32,
    @location(13) environment_id: u32,

    // Morphology parameters (16 bytes)
    // x: anisotropy, y: depth_bias, z: lobe_count, w: interference_phase
    @location(14) morphology_params: vec4<f32>,
};


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) veil_transparency: f32,
    @location(3) consciousness_level: f32,
    @location(4) density_level: f32,
    @location(5) selection_factor: f32,
};

// Realm colors for the 7-layer architecture
// From COSMOLOGICAL-ARCHITECTURE.md
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

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
    @builtin(instance_index) instance_idx: u32,
) -> VertexOutput {
    var out: VertexOutput;

    var local_pos = vertex.position.xy;
    var local_z = vertex.position.z;

    // Phase 16: Access archetype data from storage buffer
    var archetype_activations: array<f32, 22>;
    let archetype_count = arrayLength(&archetype_data);
    let use_archetype_buffer = archetype_count > 0u && instance_idx < archetype_count;
    
    if (use_archetype_buffer) {
        archetype_activations = archetype_data[instance_idx].activations;
    } else {
        // Fallback: synthesize from instance data - fixed array initialization for WGSL
        let fallback_val = instance.archetype_intensity * 0.5;
        archetype_activations = array<f32, 22>(
            fallback_val, fallback_val, fallback_val, fallback_val, fallback_val,
            fallback_val, fallback_val, fallback_val, fallback_val, fallback_val,
            fallback_val, fallback_val, fallback_val, fallback_val, fallback_val,
            fallback_val, fallback_val, fallback_val, fallback_val, fallback_val,
            fallback_val, fallback_val
        );
    }

    if (camera.morphology_mode >= 0.5) {
        let anisotropy = clamp(instance.morphology_params.x, 0.0, 1.0);
        let depth_bias = clamp(instance.morphology_params.y, 0.0, 1.0);
        let lobe_count = max(2.0, instance.morphology_params.z);
        let interference_phase = instance.morphology_params.w * 6.2831853;

        let local_angle = atan2(local_pos.y, local_pos.x);
        let radial = length(local_pos);
        let spectral_delta = clamp(instance.spectrum_data.x - instance.spectrum_data.y, -1.5, 1.5);

        // Phase 16: Compute archetype interference pattern for geometry
        let archetype_interference = compute_archetype_interference(archetype_activations, local_angle, camera.time);
        let mind_body_balance = compute_mind_body_balance(archetype_activations);
        let spirit_alignment = compute_spirit_alignment(archetype_activations);

        let orientation = camera.time * 0.18 + interference_phase + f32(instance.entity_type) * 0.31 
                        + mind_body_balance * 0.5;
        let major_axis = vec2<f32>(cos(orientation), sin(orientation));
        let minor_axis = vec2<f32>(-major_axis.y, major_axis.x);

        // Anisotropy enhanced by mind-body balance
        let enhanced_anisotropy = anisotropy + abs(mind_body_balance) * 0.2;
        let major = dot(local_pos, major_axis) * (1.0 + enhanced_anisotropy * 0.35);
        let minor = dot(local_pos, minor_axis) * (1.0 - enhanced_anisotropy * 0.25);
        local_pos = major_axis * major + minor_axis * minor;

        // Lobe modulation enhanced by archetype interference
        let archetype_lobe_mod = 1.0 + (0.08 + anisotropy * 0.16) * sin(lobe_count * local_angle + camera.time * 0.65 + interference_phase * 2.0)
                               + archetype_interference * 0.15;
        local_pos = local_pos * archetype_lobe_mod;

        // Depth wave enhanced by spirit alignment
        let depth_wave = sin(local_angle * (lobe_count * 0.5) + interference_phase + camera.time * 0.4)
                       + spirit_alignment * sin(local_angle * 3.0 + camera.time * 0.8) * 0.3;
        local_z = local_z + radial * (0.04 + depth_bias * 0.24) * depth_wave * spectral_delta;
    }

    // Scale deformed vertex by instance size and add instance position
    let world_position = instance.position + (vec3<f32>(local_pos, local_z) * instance.size);

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

    // Mix colors based on 7 realm intensities
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
    // polarization > 0 (STO): add blue tint
    // polarization < 0 (STS): add red tint
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
    // Higher consciousness = brighter, more vibrant colors
    let consciousness_level = instance.consciousness_data.x;
    let brightness_boost = 0.5 + (consciousness_level * 0.5);
    mixed_color = mixed_color * brightness_boost;

    // Clamp color to valid range
    mixed_color = clamp(mixed_color, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 1.0));

    // Set output color with full alpha
    out.color = vec4<f32>(mixed_color, 1.0);

    // Pass local position for circle rendering
    out.local_pos = local_pos;

    // Pass veil transparency for fragment shader effects
    out.veil_transparency = instance.spectrum_data.z;

    // Pass consciousness level for glow effects
    out.consciousness_level = consciousness_level;

    // Pass density level for stage-coded morphology in fragment shader
    out.density_level = f32(instance.density_level_packed);

    if (camera.selection_enabled >= 0.5) {
        let selection_dist = length(instance.position - camera.selection_position);
        out.selection_factor = 1.0 - smoothstep(0.0, camera.selection_radius, selection_dist);
    } else {
        out.selection_factor = 0.0;
    }

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Create morphology-aware shape boundary using distance from center.
    // morphology_mode < 0.5 keeps the legacy circle behavior.
    let dist = length(in.local_pos);
    let angle = atan2(in.local_pos.y, in.local_pos.x);
    var boundary = 1.0;

    if (camera.morphology_mode >= 0.5) {
        // Map packed density to 6 stage families.
        // Typical density values are 1..8; this mapping keeps transitions smooth.
        var stage = 5;
        if (in.density_level <= 1.5) {
            stage = 0;
        } else if (in.density_level <= 2.5) {
            stage = 1;
        } else if (in.density_level <= 3.5) {
            stage = 2;
        } else if (in.density_level <= 4.5) {
            stage = 3;
        } else if (in.density_level <= 6.5) {
            stage = 4;
        }

        switch (stage) {
            // Quantum: subtle 6-lobed wave
            case 0: {
                boundary = 1.0 + 0.045 * sin(6.0 * angle + camera.time * 0.9);
            }
            // Atomic: 4-lobed profile
            case 1: {
                boundary = 1.0 + 0.055 * cos(4.0 * angle + camera.time * 0.75);
            }
            // Molecular: triangular / 3-fold modulation
            case 2: {
                boundary = 1.0 + 0.060 * sin(3.0 * angle + camera.time * 0.6);
            }
            // Cellular: 8-fold fine membrane-like ripple
            case 3: {
                boundary =
                    1.0
                    + 0.030 * sin(8.0 * angle + camera.time * 1.2)
                    + 0.015 * sin(16.0 * angle - camera.time * 0.85);
            }
            // Planetary: 5-fold gentle contour
            case 4: {
                boundary = 1.0 + 0.040 * cos(5.0 * angle + camera.time * 0.5);
            }
            // Integration: near-circle with low-frequency modulation
            default: {
                boundary = 1.0 + 0.020 * sin(2.0 * angle + camera.time * 0.35);
            }
        }
    }

    // Hard circle edge
    if (dist > boundary) {
        discard;
    }

    let normalized_dist = dist / max(boundary, 0.001);

    // Apply depth gradient based on consciousness level
    // Higher consciousness = more depth/vibrancy
    let gradient = 1.0 - (normalized_dist * (0.2 + in.consciousness_level * 0.1));

    // Apply veil transparency to alpha
    // Thicker veil (lower transparency) = more opaque appearance
    // Thinner veil (higher transparency) = more ethereal/glowing
    let veil_factor = 0.5 + (in.veil_transparency * 0.5);
    let alpha = gradient * veil_factor;

    // Phase 16: Enhanced consciousness-based emission
    // Create a field-based glow that radiates from the entity center
    // Higher consciousness = stronger emission, larger glow radius
    
    // Inner core glow (bright center)
    let core_intensity = 1.0 - smoothstep(0.0, 0.4, normalized_dist);
    let core_glow = in.consciousness_level * core_intensity * 0.4;
    
    // Mid-field emission (holographic aura)
    let mid_intensity = 1.0 - smoothstep(0.2, 0.7, normalized_dist);
    let aura_glow = in.consciousness_level * mid_intensity * 0.25;
    
    // Outer field emission (subtle radiance)
    let outer_intensity = 1.0 - smoothstep(0.5, 1.0, normalized_dist);
    let field_glow = in.consciousness_level * outer_intensity * 0.15;
    
    // Combine emission layers
    let emission = core_glow + aura_glow + field_glow;
    
    // Add subtle glow for higher consciousness entities
    let glow = in.consciousness_level * 0.2;
    var final_rgb = in.color.rgb * gradient + vec3<f32>(glow, glow, glow) + in.color.rgb * emission;
    var final_alpha = clamp(alpha, 0.0, 1.0);
    
    // Enhance alpha with consciousness-based translucency
    // Higher consciousness = slightly more visible in the aura region
    let aura_alpha_boost = in.consciousness_level * mid_intensity * 0.15;
    final_alpha = clamp(final_alpha + aura_alpha_boost, 0.0, 1.0);

    if (in.selection_factor > 0.0) {
        let sf = clamp(in.selection_factor, 0.0, 1.0);
        let boundary_band = 1.0 - abs(normalized_dist - 1.0);
        let ring = smoothstep(0.90, 1.0, boundary_band);
        let core = 1.0 - smoothstep(0.0, 0.75, normalized_dist);
        let selection_tint = vec3<f32>(1.0, 0.95, 0.72);
        let highlight = selection_tint * (0.10 * sf * core + 0.18 * sf * ring);
        final_rgb = clamp(final_rgb + highlight, vec3<f32>(0.0), vec3<f32>(1.0));
        final_alpha = clamp(final_alpha + 0.06 * sf * ring, 0.0, 1.0);
    }

    // Clamp final color
    let final_color = vec4<f32>(
        clamp(final_rgb, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 1.0)),
        final_alpha
    );

    return final_color;
}
