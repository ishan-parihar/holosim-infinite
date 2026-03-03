//! # Template Module - Phase 0: Universal Template Architecture
//!
//! This module implements the foundational template-based architecture for HoloSim Infinite.
//!
//! ## Core Concepts
//!
//! ### UniversalTemplate<T>
//! Every component in the simulation follows the same template structure:
//! - Shared reference to HolographicField (Arc<HolographicField>)
//! - Spectrum configuration (Space/Time ↔ Time/Space ratio)
//! - Archetype activation profile (22 coefficients)
//! - Density (1st → 8th)
//! - Free Will seed (non-deterministic choice)
//! - Component-specific data (T)
//!
//! ### Transcend and Include (LayerReference)
//! Each layer INCLUDES all previous layers (via Arc reference) and TRANSCENDS
//! by adding new data. This implements the holographic principle where each
//! part contains the whole.
//!
//! ### Memory Efficiency
//! - Traditional: O(n) memory for n entities (each owns all data)
//! - Holographic: O(n^2/3) via shared references (entities share common field data)
//!
//! ### Phase 0 Deliverables (from HOLOSIM_INFINITE_V6_R&D_REFACTOR_ROADMAP.md)
//! - [x] UniversalTemplate<T> generic type
//! - [x] HolographicFieldState with recursive subdivision
//! - [x] TemplateFactory for instantiation with caching
//! - [x] Reference-based "include" mechanism
//! - [x] Multi-scale field storage (8 scale levels)
//! - [x] EntityData component type
//! - [x] ParticleData component type  
//! - [x] AtomData component type
//! - [x] WorldData component type
//! - [x] StarData component type
//! - [x] GalaxyData component type
//! - [x] CellData component type
//!
//! ### Phase 1.5: Entity Migration
//! - EntityBehavior trait: Unified interface for all entity types
//! - ExtractedEntityPotential → Entity conversion
//! - Backward compatibility with SubSubLogos
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Implement holographic logic ONCE, it applies to EVERYTHING."

pub mod component_data;
pub mod entity_behavior;
pub mod entity_data;
pub mod layer_reference;
pub mod migration;
pub mod template_factory;
pub mod transcend_include;

// Re-export main types from component_data
pub use component_data::{
    AtmosphereData, AtomData, CellData, CellType, DivisionState, GalaxyData, GalaxyType,
    HydrologySummary, MolecularGeometry, MolecularShape, MoleculeData, ParticleData, ParticleType,
    SpectralClass, StarData, TemplateComponent, TerrainSummary, WorldData,
};
pub use entity_behavior::EntityBehavior;
pub use entity_data::EntityData;
pub use layer_reference::*;
pub use template_factory::*;
pub use transcend_include::{
    AttractorField, Feature, LayerTransition, Orientation, TargetDensity, TranscendInclude,
};

// Re-export from existing universal_template.rs
pub use crate::holographic::universal_template::{
    ArchetypeActivationProfile, Possibility, PossibilitySpace, SpectrumConfiguration,
    TemplateConfig, TemplateKey,
};

// Re-export coordinate types from matter module
pub use crate::matter::particle::{Complex, Coordinate3D, Vector3D};

// Type alias for Entity using UniversalTemplate
pub use entity_data::Entity;
