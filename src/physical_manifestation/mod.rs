// Physical Manifestation Module
//
// This module implements the conversion of Light and Intelligent Energy into matter
// (particles, atoms, molecules) with accurate physics properties.
//
// This is Phase 2 of the refactor, covering:
// - Light-to-matter condensation
// - Particle system enhancement
// - Atom formation
// - Molecular bonding
// - Space/time coordinate system
// - Physics engine integration
// - Energy conservation
// - Matter-antimatter balance
// - Quantum mechanics
// - Thermodynamics
//
// Phase 7 Update: Consciousness-First Cosmology
// - Quantum energy pools with holographic information
// - Consciousness-to-matter transition
// - Quantum decoherence and wave function collapse
// - Attractor fields for stable quantum states
// - Periodic table attractor fields
//
// Phase 3 Update: Density Octave Integration
// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md:
// "Integrate Physical Manifestation"
// - Bridge between Density Octave and Physical Manifestation
// - Holographic Blueprint Encoding (DNA/RNA as spectrum configurations)
// - Physical Unfolding Sequence (Quantum → Atomic → Molecular → Cellular → Life → Societal)
// - Consciousness-First Demonstration (spectrum patterns exist BEFORE physical matter)

pub mod atom_formation;
pub mod consciousness_to_matter;
pub mod hierarchy;
pub mod light_condensation;
pub mod structures;

// Phase 3: Density Octave Integration modules
pub mod blueprint;
pub mod consciousness_first;
pub mod integration;
pub mod unfolding;

// Re-exports for convenient access

pub use structures::{PhysicalStructure, PhysicalStructureManager, PhysicalStructureType};

pub use hierarchy::{HierarchicalCompositionManager, SimultaneousEmergenceManager};

// Phase 3: Re-exports
pub use blueprint::{PhysicalBlueprintEncoding, SpectrumConfiguration};
pub use consciousness_first::{
    ConsciousnessFirstDemonstration, ConsciousnessFirstTimeline, TimelineEvent, ValidationResult,
};
pub use integration::{
    DensityToPhysicalBridge, PhysicalManifestation, PhysicalManifestationError,
    PhysicalManifestationResult,
};
pub use unfolding::{PhysicalUnfolding, UnfoldingError, UnfoldingResult, UnfoldingStage};
