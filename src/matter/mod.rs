// Matter Module - Phase 2: Physical Manifestation
//
// This module implements matter (particles, atoms, molecules) that emerge from Light
// condensation with archetype-based properties.
//
// Key Principles:
// 1. All matter properties derive from archetype activation patterns
// 2. Particles condense from Light when energy thresholds are met
// 3. Physics properties (mass, charge, spin, lifetime) emerge from archetypes
// 4. Wavefunction evolution follows quantum mechanical principles
// 5. Particle interactions governed by archetype compatibility
//
// Knowledge Base References:
// - COMPLETE_REFACTOR_ROADMAP_V4.md Phase 2: Physical Manifestation
// - "Light carries the complete blueprint for consciousness"
// - "Matter is condensed Light with holographic encoding"

pub mod particle;

// Re-export main types for convenience
pub use particle::{
    Atom, BondType, Cell, Coordinate3D, DNABase, Matter, MolecularStructure, Molecule, Nucleus,
    OrganelleType, Particle, ParticleID, RNABase, Vector3D,
};

// Re-export holographic addressing (Phase 1 migration)
pub use crate::holographic::field_address::HolographicAddress;
