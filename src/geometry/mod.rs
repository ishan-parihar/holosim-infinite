//! Geometry Module
//!
//! From HOLOSIM_INFINITE_RnD_ROADMAP_V5.md Phase 8:
//! "Morphogenetic templates define the standing wave patterns that guide emergence"
//!
//! ## Overview
//!
//! This module provides geometric templates and patterns for physical manifestation:
//!
//! - **Morphogenetic Templates**: Standing wave patterns for matter formation
//! - **Integration with Sacred Geometry**: Golden ratio scaling, Platonic solids
//! - **Multi-scale Templates**: Atom → Molecule → Cell → Organism
//!
//! ## Key Concepts
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Matter condenses from field coherence according to morphogenetic templates"
//!
//! The templates operate at multiple scales:
//! - Atomic: Electron shell standing waves
//! - Molecular: Bond geometry templates
//! - Cellular: Membrane and organelle templates
//! - Organism: Body plan templates

pub mod morphogenetic;

// Re-exports for convenience
pub use morphogenetic::{
    BodyPlan, CellType, DevelopmentalStage, ElementType, MorphogeneticTemplate,
    MorphogeneticTemplateLibrary, StabilityConditions, TemplateType,
};
