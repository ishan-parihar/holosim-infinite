// ============================================================================
// Common WGSL Types — HoloSim Infinite GPU Compute Shaders
// ============================================================================
// Shared struct definitions for all compute shaders.
// Layouts sourced from .sisyphus/plans/wgsl-architecture.md (lines 186-322).
// ============================================================================

// ---------------------------------------------------------------------------
// EntityGPUData — Flattened per-entity input buffer
// Array stride: 208 bytes (next multiple of 16 after 200)
// ---------------------------------------------------------------------------
struct EntityGPUData {
    // Entity ID packed as 2x u32 (hash of UUID string + incarnation_number)
    entity_id_hash: u32,           // offset 0
    incarnation_number: u32,       // offset 4
    _pad0: u32,                    // offset 8 — alignment to 16

    // Spectrum configuration (from spectrum_configuration.ratio)
    spectrum_ratio: f32,           // offset 12
    space_time_access: f32,        // offset 16
    oneness_access: f32,           // offset 20
    veil_transparency: f32,        // offset 24

    // Vibrational state
    frequency: f32,                // offset 28
    coherence: f32,                // offset 32
    _pad1: u32,                    // offset 36 — alignment

    // Archetype activation (22 x f32 = 88 bytes, rounded to 96 for alignment)
    archetype_activation: vec4<f32>,    // A1-A4,   offset 40
    archetype_activation_1: vec4<f32>,  // A5-A8,   offset 56
    archetype_activation_2: vec4<f32>,  // A9-A12,  offset 72
    archetype_activation_3: vec4<f32>,  // A13-A16, offset 88
    archetype_activation_4: vec4<f32>,  // A17-A20, offset 104
    archetype_activation_5: vec2<f32>,  // A21-A22, offset 120
    _pad2: vec2<f32>,                   // padding, offset 128

    // Polarity (ServiceToOthers=1.0, ServiceToSelf=-1.0, Neutral=0.0)
    polarity_direction: f32,       // offset 136
    polarity_strength: f32,        // offset 140
    evolutionary_rate: f32,        // offset 144
    _pad3: f32,                    // offset 148 — alignment

    // Holographic signature (8 x f32 = 32 bytes)
    holographic_signature_0: vec4<f32>, // offset 152
    holographic_signature_1: vec4<f32>, // offset 168

    // Density level (1-8 encoded as f32)
    density_level: f32,            // offset 184
    _pad4: vec3<f32>,              // offset 188 — alignment to 16
}
// Total: 200 bytes -> array_stride: 208 bytes

// ---------------------------------------------------------------------------
// ConnectionOutput — Result buffer (O(n^2) pairwise connections)
// Size: 32 bytes (naturally aligned, multiple of 16)
// Buffer organization: linear array indexed by entity_a * N + entity_b
// ---------------------------------------------------------------------------
struct ConnectionOutput {
    strength: f32,                 // Combined connection strength [0, 1]
    archetype_similarity: f32,     // Component metric
    spectrum_similarity: f32,      // Component metric
    blueprint_alignment: f32,      // Component metric
    resonance_match: f32,          // Component metric
    phase_coherence: f32,          // Component metric
    karmic_correlation: f32,       // Component metric
    connection_type: u32,          // 0=Resonant, 1=Harmonic, 2=Entangled, 3=Antiphase, 4=Weak
}
// Size: 32 bytes

// ---------------------------------------------------------------------------
// ResonanceOutput — Result buffer (O(n^2) resonance pairs)
// Size: 32 bytes
// ---------------------------------------------------------------------------
struct ResonanceOutput {
    resonance_score: f32,          // Overall weighted score [0, 1]
    spectrum_resonance: f32,       // Component (30% weight)
    holographic_resonance: f32,    // Component (40% weight)
    polarity_resonance: f32,       // Component (30% weight)
    resonance_type: u32,           // 0=None, 1=Low, 2=Medium, 3=High
    _pad: vec3<u32>,
}
// Size: 32 bytes

// ---------------------------------------------------------------------------
// DispatchParams — Uniform buffer for compute dispatch configuration
// Size: 80 bytes (multiple of 16)
// ---------------------------------------------------------------------------
struct DispatchParams {
    num_entities: u32,             // N
    max_connections: u32,          // Max connections per entity (50/100/N)
    _pad0: u32,
    _pad1: u32,

    // Weights (matching CPU calculation)
    weight_archetype: f32,         // 0.35
    weight_spectrum: f32,          // 0.20
    weight_blueprint: f32,         // 0.15
    weight_resonance: f32,         // 0.10
    weight_phase: f32,             // 0.10
    weight_karmic: f32,            // 0.10
    _pad2: u32,
    _pad3: u32,

    // Resonance weights
    weight_spectrum_res: f32,      // 0.30
    weight_holographic_res: f32,   // 0.40
    weight_polarity_res: f32,      // 0.30
    max_spectrum_diff: f32,        // 20.0 (normalization)
}
// Size: 80 bytes
