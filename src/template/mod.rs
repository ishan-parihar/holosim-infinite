//! Template Module - Phase 1: Universal Template Architecture
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 1:
//! "Implement the UniversalTemplate that instantiates the same logic for all scales"
//!
//! This module provides:
//! - EntityData struct for entity-specific data
//! - Universal template infrastructure from holographic module
//! - Transcend and Include universal mechanism
//! - Migration utilities for SubSubLogos → Entity conversion
//!
//! ## Key Design Principles
//!
//! 1. **Build Once, Instantiate Many**: All holographic operations are defined ONCE
//!    on the UniversalTemplate and apply to ANY component type (quantum, atomic,
//!    molecular, cellular, biological, planetary, stellar, galactic, entity, logos)
//!
//! 2. **Shared References**: The holographic field is stored as Arc<HolographicField>
//!    so all template instances share the same field reference (O(1) memory per instance)
//!
//! 3. **Transcend and Include**: Every layer transition follows the universal constant:
//!    - INCLUDE: Retain all previous development
//!    - TRANSCEND: Add new development
//!    - EVOLVES INTO: Create attractor-field for next stage

pub mod entity_data;
pub mod migration;
pub mod transcend_include;

// Re-export key types for convenience
pub use entity_data::{Entity, EntityData};

// Re-export from holographic module
pub use crate::holographic::universal_template::{
    ArchetypeActivationProfile, ArchetypicalInterference, Choice, Possibility, PossibilitySpace,
    SpectrumConfiguration, TemplateConfig, TemplateFactory, UniversalTemplate,
};

// Re-export Transcend and Include types
pub use transcend_include::{
    AttractorField, Feature, LayerTransition, Orientation, TargetDensity, TranscendInclude,
    TranscendIncludeBuilder,
};
