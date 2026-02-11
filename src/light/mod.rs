// Light Module
//
// This module implements the Holographic Light Architecture (Blue-Ray Realm).
// Light is not just 'energy'—it is intelligent energy with embedded architecture.
// Every quantum of Light carries the complete blueprint for consciousness development.
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 2.2 (The Structure - Blue-Ray Realm)
// - Energy-Ray-Centers/5. Blue Ray.json
// - Cosmology.json (Third distortion: Light)

pub mod archetype_patterns;
pub mod architecture;
pub mod encoding;
pub mod photon;

// Re-exports for convenient access
pub use archetype_patterns::ArchetypePatterns;
pub use architecture::{LightArchitecture, LightLaws};
pub use encoding::{
    ArchetypeEncoding, ArchetypePatternBit, HolographicEncoding, HolographicReference, PatternType,
};
pub use photon::{
    InteractionResult, MeasurementResult, MomentumVector, Photon, PhotonID, QuantumPosition,
    QuantumState,
};
