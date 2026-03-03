//! Particle Module - Phase 2.3: Archetype-Derived Particle Properties
//!
//! This module provides particle properties derived from archetype activation
//! patterns, connecting physics to the holographic discovery system.
//!
//! # Key Components
//!
//! - **ParticleProperties**: Core struct for derived particle properties
//! - **PropertyValidation**: Validation against known particle types
//!
//! # Derivation Rules
//!
//! - **Charge**: Catalyst archetype (A3) determines charge
//! - **Spin**: Matrix + Great Way average determines spin
//! - **Mass**: Coherence from activation pattern
//! - **Lifetime**: Stability from Matrix + Great Way

pub mod archetype_properties;

// Re-export main types for convenience
pub use archetype_properties::{
    coordinate_to_position, position_to_coordinate, ParticleProperties, PropertyValidation,
    ELEMENTARY_CHARGE, H_BAR, PLANCK_MASS, SPEED_OF_LIGHT,
};
