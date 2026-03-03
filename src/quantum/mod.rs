//! Quantum mechanics module integrated with holographic architecture
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "The quantum field is the third primal distortion - Light"
//! "Light is the mechanism by which Intelligent Infinity knows Itself"
//!
//! This module implements quantum mechanics derived from holographic principles:
//!
//! # Key Concepts
//!
//! - **Light as Field**: Quantum field IS Light (third primal distortion), not empty space
//! - **Free Will Collapse**: Wavefunction collapse is a choice, not random
//! - **Holographic Connection**: Entanglement via shared spectrum configuration
//! - **Born Rule Preservation**: Statistical outcomes still match QM predictions
//!
//! # Architecture
//!
//! The quantum module is organized into:
//!
//! - `quantum_field`: Quantum field derived from HolographicField
//! - `free_will_collapse`: Choice-based collapse mechanism
//! - `entanglement`: Holographic entanglement via shared spectrum

pub mod entanglement;
pub mod free_will_collapse;
pub mod quantum_field;

// Re-exports for convenience
pub use entanglement::{
    EntanglementLink, EntanglementStatistics, InstantEffect, Measurement, SharedSpectrum,
};
pub use free_will_collapse::{
    ChoiceBasedCollapse, PossibilitySpace as QuantumPossibilitySpace, QuantumCollapseContext,
    QuantumCollapseResult,
};
pub use quantum_field::{
    LightLoveRatio, QuantumField, QuantumFieldConfig, QuantumFieldError, QuantumFieldStatistics,
    QuantumStateSignature,
};
