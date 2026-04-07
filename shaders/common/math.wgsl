// ============================================================================
// Common Math Utilities — HoloSim Infinite GPU Compute Shaders
// ============================================================================
// Shared helper functions for all compute shaders.
// ============================================================================

// Clamp a value to [0, 1] range
fn clamp01(x: f32) -> f32 {
    return clamp(x, 0.0, 1.0);
}

// Cosine similarity between two vec4 vectors
// Returns a value in approximately [-1, 1], higher means more similar
fn cosine_similarity(a: vec4<f32>, b: vec4<f32>) -> f32 {
    let dot_ab = dot(a, b);
    let len_a = length(a);
    let len_b = length(b);
    return dot_ab / (len_a * len_b + 1e-8);
}

// Dot product for two 6-element arrays
fn dot_product_6(a: array<f32, 6>, b: array<f32, 6>) -> f32 {
    return a[0] * b[0]
         + a[1] * b[1]
         + a[2] * b[2]
         + a[3] * b[3]
         + a[4] * b[4]
         + a[5] * b[5];
}

// Harmonic match between two frequencies
// Returns 1.0 when frequencies are identical, 0.0 when maximally different
fn harmonic_match(freq_a: f32, freq_b: f32) -> f32 {
    let diff = abs(freq_a - freq_b);
    let max_f = max(freq_a, max(freq_b, 1e-8));
    return 1.0 - min(diff / max_f, 1.0);
}
