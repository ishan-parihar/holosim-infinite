//! Collective Manifestation (Building) System
//!
//! From MASTER_R&D_ROADMAP.md Phase 6 Week 89-92:
//! "Implement collective manifestation (building) including building as collective manifestation,
//! collective resonance calculation, resonance requirement checks, and holographic structure unfolding."
//!
//! Building in Holonic Realms is not assembly but manifestation - structures emerge from the
//! combined resonance of multiple entities. The collective resonance creates an interference
//! pattern that unfolds into holographic structures with unique properties derived from the
//! contributors' archetype patterns.

pub mod collective_resonance;
pub mod holographic_structure;
pub mod manifestation_manager;

// Re-export commonly used types
pub use collective_resonance::{CollectiveResonanceCalculator, CollectiveResonanceResult};

pub use holographic_structure::{
    FunctionalProperties, GeometricPattern, GeometricPatternType, HolographicStructure,
    MaterialAppearance, StructureProperties, StructureType, VisualProperties,
};

pub use manifestation_manager::{ManifestationError, ManifestationManager, ManifestationProject};
