// ============================================================================
// Connection Metrics — HoloSim Infinite
// ============================================================================
// Computes pairwise connection metrics between entities.
// Each invocation computes ONE pair (entity_a, entity_b).
// Workgroup size: 16×16 = 256 invocations per workgroup.
// Output indexed by: entity_a * num_entities + entity_b.
// ============================================================================

#include "common/types.wgsl"
#include "common/math.wgsl"

@group(0) @binding(0)
var<storage, read> entities: array<EntityGPUData>;

@group(0) @binding(1)
var<storage, read_write> connections: array<ConnectionOutput>;

@group(0) @binding(2)
var<uniform> params: DispatchParams;

// ============================================================================
// Helper: Cosine similarity of 22-element archetype activation vectors
// ============================================================================
fn archetype_cosine_similarity(a: EntityGPUData, b: EntityGPUData) -> f32 {
    // Compute dot product across all 22 elements
    let dot_0 = dot(a.archetype_activation, b.archetype_activation);
    let dot_1 = dot(a.archetype_activation_1, b.archetype_activation_1);
    let dot_2 = dot(a.archetype_activation_2, b.archetype_activation_2);
    let dot_3 = dot(a.archetype_activation_3, b.archetype_activation_3);
    let dot_4 = dot(a.archetype_activation_4, b.archetype_activation_4);
    let dot_5 = dot(a.archetype_activation_5, b.archetype_activation_5);

    let dot_total = dot_0 + dot_1 + dot_2 + dot_3 + dot_4 + dot_5;

    // Compute magnitudes
    let mag_a_sq = dot(a.archetype_activation, a.archetype_activation)
                 + dot(a.archetype_activation_1, a.archetype_activation_1)
                 + dot(a.archetype_activation_2, a.archetype_activation_2)
                 + dot(a.archetype_activation_3, a.archetype_activation_3)
                 + dot(a.archetype_activation_4, a.archetype_activation_4)
                 + dot(a.archetype_activation_5, a.archetype_activation_5);

    let mag_b_sq = dot(b.archetype_activation, b.archetype_activation)
                 + dot(b.archetype_activation_1, b.archetype_activation_1)
                 + dot(b.archetype_activation_2, b.archetype_activation_2)
                 + dot(b.archetype_activation_3, b.archetype_activation_3)
                 + dot(b.archetype_activation_4, b.archetype_activation_4)
                 + dot(b.archetype_activation_5, b.archetype_activation_5);

    let mag_a = sqrt(mag_a_sq);
    let mag_b = sqrt(mag_b_sq);

    // Normalize to [0, 1] with abs()
    return abs(dot_total / (mag_a * mag_b + 1e-8));
}

// ============================================================================
// Helper: Cosine similarity of 8-element holographic signature vectors
// ============================================================================
fn blueprint_cosine_similarity(a: EntityGPUData, b: EntityGPUData) -> f32 {
    let dot_0 = dot(a.holographic_signature_0, b.holographic_signature_0);
    let dot_1 = dot(a.holographic_signature_1, b.holographic_signature_1);
    let dot_total = dot_0 + dot_1;

    let mag_a_sq = dot(a.holographic_signature_0, a.holographic_signature_0)
                 + dot(a.holographic_signature_1, a.holographic_signature_1);

    let mag_b_sq = dot(b.holographic_signature_0, b.holographic_signature_0)
                 + dot(b.holographic_signature_1, b.holographic_signature_1);

    let mag_a = sqrt(mag_a_sq);
    let mag_b = sqrt(mag_b_sq);

    return clamp01(dot_total / (mag_a * mag_b + 1e-8));
}

// ============================================================================
// Determine connection type from metric values
// Matches CPU implementation at holographic_field.rs:1244-1272
// ============================================================================
fn determine_connection_type(
    strength: f32,
    archetype_sim: f32,
    resonance: f32,
    phase: f32,
) -> u32 {
    // Resonant (0): archetype_sim > 0.7 && resonance > 0.6 && phase > 0.6
    if (archetype_sim > 0.7 && resonance > 0.6 && phase > 0.6) {
        return 0u;
    }
    // Harmonic (1): archetype_sim > 0.4 && resonance > 0.5
    if (archetype_sim > 0.4 && resonance > 0.5) {
        return 1u;
    }
    // Entangled (2): strength > 0.6 && phase > 0.6
    if (strength > 0.6 && phase > 0.6) {
        return 2u;
    }
    // Antiphase (3): phase < 0.4
    if (phase < 0.4) {
        return 3u;
    }
    // Weak (4): default
    return 4u;
}

// ============================================================================
// Main entry point — pairwise connection metrics
// ============================================================================
@compute @workgroup_size(16, 16, 1)
fn main_connection_metrics(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let entity_a_idx = gid.x;
    let entity_b_idx = gid.y;
    let num_entities = params.num_entities;

    // Boundary guard
    if (entity_a_idx >= num_entities || entity_b_idx >= num_entities) {
        return;
    }

    let a = entities[entity_a_idx];
    let b = entities[entity_b_idx];

    // Linear index into the N×N output matrix
    let output_idx = entity_a_idx * num_entities + entity_b_idx;

    // Self-connection: identity connection
    if (entity_a_idx == entity_b_idx) {
        connections[output_idx] = ConnectionOutput(
            1.0,  // strength
            1.0,  // archetype_similarity
            1.0,  // spectrum_similarity
            1.0,  // blueprint_alignment
            1.0,  // resonance_match
            1.0,  // phase_coherence
            1.0,  // karmic_correlation
            0u,   // connection_type = Resonant
        );
        return;
    }

    // --- Metric 1: Archetype similarity (weight 0.35) ---
    // Cosine similarity of 22-element activation vectors, normalized to [0,1]
    let arch_sim = archetype_cosine_similarity(a, b);

    // --- Metric 2: Spectrum similarity (weight 0.20) ---
    // 1.0 - abs(a.ratio - b.ratio) / (a.ratio + b.ratio + epsilon)
    let spec_ratio_sum = a.spectrum_ratio + b.spectrum_ratio + 1e-8;
    let spec_sim = clamp01(1.0 - abs(a.spectrum_ratio - b.spectrum_ratio) / spec_ratio_sum);

    // --- Metric 3: Blueprint alignment (weight 0.15) ---
    // Cosine similarity of 8-element holographic_signature vectors
    let blueprint = blueprint_cosine_similarity(a, b);

    // --- Metric 4: Resonance match (weight 0.10) ---
    // Use harmonic_match from common/math.wgsl
    let resonance = harmonic_match(a.frequency, b.frequency);

    // --- Metric 5: Phase coherence (weight 0.10) ---
    // Average of both entities' coherence values
    let phase = (a.coherence + b.coherence) / 2.0;

    // --- Metric 6: Karmic correlation (weight 0.10) ---
    // Proxy using evolutionary_rate similarity
    let karmic = clamp01(1.0 - abs(a.evolutionary_rate - b.evolutionary_rate));

    // --- Weighted strength ---
    let strength = clamp01(
        arch_sim * params.weight_archetype
        + spec_sim * params.weight_spectrum
        + blueprint * params.weight_blueprint
        + resonance * params.weight_resonance
        + phase * params.weight_phase
        + karmic * params.weight_karmic
    );

    // --- Connection type determination ---
    let conn_type = determine_connection_type(strength, arch_sim, resonance, phase);

    // --- Write output ---
    connections[output_idx] = ConnectionOutput(
        strength,
        arch_sim,
        spec_sim,
        blueprint,
        resonance,
        phase,
        karmic,
        conn_type,
    );
}
