//! Validation Module
//!
//! This module provides validation mechanisms for cosmological architecture principles:
//! - Holographic Principle: Each entity contains all densities/sub-densities
//! - Consciousness-First: Spectrum patterns exist before physical matter
//! - Density Transitions: Entities can transition between densities
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "Validation mechanisms ensure that the
//! simulation correctly implements the cosmological architecture principles."

pub mod holographic_principle;

pub use holographic_principle::{
    HolographicPrincipleValidationResult, HolographicPrincipleValidator, MicrocosmMacrocosmResult,
    OctaveContainmentResult, SubDensityContainmentResult,
};
