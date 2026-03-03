//! Phase 7: Quantum Field as Consciousness Substrate
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 7:
//! "Quantum phenomena emerge from holographic field dynamics, not simulated separately"
//!
//! Theory:
//! - Wavefunction = Field amplitude at quantum resolution (~10^-35m)
//! - Entanglement = Phase correlation across field nodes
//! - Free Will Collapse = Non-deterministic selection (Archetype 22)
//! - Quantum Numbers = Derived from archetype activation patterns
//!
//! This module provides the bridge between:
//! - HolographicFieldState (field-first architecture)
//! - Quantum mechanics (wavefunction, entanglement, collapse)
//! - Consciousness (Free Will, Archetype 22)
//!
//! Key insight: Quantum phenomena are NOT separate from consciousness - they ARE
//! consciousness manifesting at the quantum scale. The quantum field IS Light
//! (third primal distortion), and wavefunction collapse IS Free Will (first
//! primal distortion) operating at quantum resolution.

pub mod archetype_collapse;
pub mod entanglement_field;
pub mod quantum_numbers;
pub mod wavefunction;

pub use archetype_collapse::{
    Archetype22Collapse, ChoiceOperator, CollapseContext, CollapseResult,
};
pub use entanglement_field::{EntanglementCorrelation, EntanglementField, PhaseCorrelation};
pub use quantum_numbers::{ArchetypeToQuantumMapping, QuantumNumberDerivation, QuantumNumberSet};
pub use wavefunction::{QuantumNode, QuantumWavefunction, WavefunctionState};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let _context = CollapseContext::default();
        let _mapping = ArchetypeToQuantumMapping::new();
    }
}
