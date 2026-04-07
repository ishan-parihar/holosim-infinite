// ============================================================================
// Resonance Matrix — HoloSim Infinite
// ============================================================================
// Computes pairwise resonance scores between entities.
//
// Three resonance metrics (from holographic_field.rs:1652-1776):
//   - Spectrum resonance (weight 0.30): ratio similarity
//   - Holographic resonance (weight 0.40): archetype cosine similarity
//   - Polarity resonance (weight 0.30): polarity direction match
//
// Dispatch: 2D grid where gid.x = entity_a, gid.y = entity_b
// Workgroup: 16x16 = 256 entities per workgroup in each dimension
// ============================================================================

#include "common/types.wgsl"
#include "common/math.wgsl"

// ---------------------------------------------------------------------------
// Resonance type classification
// ---------------------------------------------------------------------------
const RESONANCE_TYPE_NONE: u32 = 0u;
const RESONANCE_TYPE_LOW: u32 = 1u;
const RESONANCE_TYPE_MEDIUM: u32 = 2u;
const RESONANCE_TYPE_HIGH: u32 = 3u;

// ---------------------------------------------------------------------------
// Buffers
// ---------------------------------------------------------------------------
@group(0) @binding(0)
var<storage, read> entities: array<EntityGPUData>;

@group(0) @binding(1)
var<storage, read_write> resonance: array<ResonanceOutput>;

@group(0) @binding(2)
var<uniform> params: DispatchParams;

// ---------------------------------------------------------------------------
// Spectrum resonance: similarity of space/time ratios
// From CPU line 1711: max_difference = 20.0
// ---------------------------------------------------------------------------
fn compute_spectrum_resonance(a: EntityGPUData, b: EntityGPUData) -> f32 {
    let diff = abs(a.spectrum_ratio - b.spectrum_ratio);
    let max_diff = params.max_spectrum_diff; // 20.0
    return 1.0 - min(diff / max_diff, 1.0);
}

// ---------------------------------------------------------------------------
// Holographic resonance: cosine similarity of 22-element archetype vectors
// Matches CPU calculate_holographic_resonance (lines 1720-1743)
// ---------------------------------------------------------------------------
fn compute_holographic_resonance(a: EntityGPUData, b: EntityGPUData) -> f32 {
    // Compute dot product and magnitudes across all archetype segments
    var dot_product: f32 = 0.0;
    var mag_a_sq: f32 = 0.0;
    var mag_b_sq: f32 = 0.0;

    // Segment 1: A1-A4 (vec4)
    dot_product += dot(a.archetype_activation, b.archetype_activation);
    mag_a_sq += dot(a.archetype_activation, a.archetype_activation);
    mag_b_sq += dot(b.archetype_activation, b.archetype_activation);

    // Segment 2: A5-A8 (vec4)
    dot_product += dot(a.archetype_activation_1, b.archetype_activation_1);
    mag_a_sq += dot(a.archetype_activation_1, a.archetype_activation_1);
    mag_b_sq += dot(b.archetype_activation_1, b.archetype_activation_1);

    // Segment 3: A9-A12 (vec4)
    dot_product += dot(a.archetype_activation_2, b.archetype_activation_2);
    mag_a_sq += dot(a.archetype_activation_2, a.archetype_activation_2);
    mag_b_sq += dot(b.archetype_activation_2, b.archetype_activation_2);

    // Segment 4: A13-A16 (vec4)
    dot_product += dot(a.archetype_activation_3, b.archetype_activation_3);
    mag_a_sq += dot(a.archetype_activation_3, a.archetype_activation_3);
    mag_b_sq += dot(b.archetype_activation_3, b.archetype_activation_3);

    // Segment 5: A17-A20 (vec4)
    dot_product += dot(a.archetype_activation_4, b.archetype_activation_4);
    mag_a_sq += dot(a.archetype_activation_4, a.archetype_activation_4);
    mag_b_sq += dot(b.archetype_activation_4, b.archetype_activation_4);

    // Segment 6: A21-A22 (vec2)
    dot_product += dot(a.archetype_activation_5, b.archetype_activation_5);
    mag_a_sq += dot(a.archetype_activation_5, a.archetype_activation_5);
    mag_b_sq += dot(b.archetype_activation_5, b.archetype_activation_5);

    let mag_a = sqrt(mag_a_sq);
    let mag_b = sqrt(mag_b_sq);

    if (mag_a == 0.0 || mag_b == 0.0) {
        return 0.0;
    }

    return dot_product / (mag_a * mag_b);
}

// ---------------------------------------------------------------------------
// Polarity resonance: match of polarity direction
// polarity_direction: >0 = ServiceToOthers, <0 = ServiceToSelf, ~0 = Neutral
// Matches CPU calculate_polarity_resonance (lines 1750-1775)
// ---------------------------------------------------------------------------
fn compute_polarity_resonance(a: EntityGPUData, b: EntityGPUData) -> f32 {
    let dir_a = a.polarity_direction;
    let dir_b = b.polarity_direction;
    let abs_a = abs(dir_a);
    let abs_b = abs(dir_b);
    let neutral_threshold = 0.1;

    // Both near zero (Neutral-Neutral)
    if (abs_a < neutral_threshold && abs_b < neutral_threshold) {
        return 0.5;
    }

    // One near zero (Neutral with non-Neutral)
    if (abs_a < neutral_threshold || abs_b < neutral_threshold) {
        return 0.3;
    }

    // Both same sign (both positive or both negative)
    if ((dir_a > 0.0 && dir_b > 0.0) || (dir_a < 0.0 && dir_b < 0.0)) {
        return 1.0;
    }

    // Opposite signs
    return 0.0;
}

// ---------------------------------------------------------------------------
// Classify resonance type based on score
// ---------------------------------------------------------------------------
fn classify_resonance(score: f32) -> u32 {
    if (score > 0.8) {
        return RESONANCE_TYPE_HIGH;
    }
    if (score > 0.5) {
        return RESONANCE_TYPE_MEDIUM;
    }
    if (score > 0.2) {
        return RESONANCE_TYPE_LOW;
    }
    return RESONANCE_TYPE_NONE;
}

// ---------------------------------------------------------------------------
// Main entry point — 2D grid: gid.x = entity_a, gid.y = entity_b
// ---------------------------------------------------------------------------
@compute @workgroup_size(16, 16, 1)
fn main_resonance_matrix(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let entity_a = gid.x;
    let entity_b = gid.y;
    let n = params.num_entities;

    // Out of bounds — write defaults
    if (entity_a >= n || entity_b >= n) {
        return;
    }

    // Linear index into the N*N output buffer
    let idx = entity_a * n + entity_b;

    let a = entities[entity_a];
    let b = entities[entity_b];

    // Self-connection: perfect resonance
    if (entity_a == entity_b) {
        resonance[idx] = ResonanceOutput(
            1.0,   // resonance_score
            1.0,   // spectrum_resonance
            1.0,   // holographic_resonance
            1.0,   // polarity_resonance
            RESONANCE_TYPE_HIGH,
            vec3<u32>(0u, 0u, 0u),
        );
        return;
    }

    // Compute three resonance components
    let spectrum_res = compute_spectrum_resonance(a, b);
    let holographic_res = compute_holographic_resonance(a, b);
    let polarity_res = compute_polarity_resonance(a, b);

    // Weighted overall score
    let score = params.weight_spectrum_res * spectrum_res
              + params.weight_holographic_res * holographic_res
              + params.weight_polarity_res * polarity_res;

    let clamped_score = clamp01(score);

    // Classify and write output
    resonance[idx] = ResonanceOutput(
        clamped_score,
        spectrum_res,
        holographic_res,
        polarity_res,
        classify_resonance(clamped_score),
        vec3<u32>(0u, 0u, 0u),
    );
}
