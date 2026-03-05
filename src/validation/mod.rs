//! Validation Module
//!
//! This module provides validation mechanisms for cosmological architecture principles:
//! - Holographic Principle: Each entity contains all densities/sub-densities
//! - Consciousness-First: Spectrum patterns exist before physical matter
//! - Density Transitions: Entities can transition between densities
//!
//! From COSMOLOGICAL-ARCHITECTURE.md: "Validation mechanisms ensure that the
//! simulation correctly implements the cosmological architecture principles."

pub mod architecture_tests;
pub mod holographic_principle;
pub mod holographic_tests;
pub mod physics_recovery;
pub mod simulation_v3_validator;

/// Float type for validation calculations
pub type Float = f64;

pub use holographic_principle::{
    HolographicPrincipleValidationResult, HolographicPrincipleValidator, MicrocosmMacrocosmResult,
    OctaveContainmentResult, SubDensityContainmentResult,
};

pub use architecture_tests::{ArchitectureTestResult, ArchitectureValidator};

pub use holographic_tests::{HolographicReconstructionResult, HolographicReconstructionValidator};

pub use physics_recovery::{PhysicsRecoveryResult, PhysicsRecoveryValidator};

pub use simulation_v3_validator::{
    ArchitectureValidationResult, ComprehensiveValidationResult, IntegrationValidationResult,
    PerformanceValidationResult, SimulationV3Validator, ValidationResult,
};
