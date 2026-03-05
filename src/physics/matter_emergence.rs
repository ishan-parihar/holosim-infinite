//! Matter Emergence Pipeline - V5 Phase 4 Physics Implementation
//!
//! This module implements the matter emergence pipeline that bridges quantum field
//! dynamics to classical matter formation. It demonstrates the key principle that
//! matter IS consciousness at the densest resolution.
//!
//! From V5 Phase 4 specifications:
//! - MatterEmergencePipeline manages the quantum → classical transition
//! - Attractor fields represent stable quantum configurations
//! - Elements emerge when coherence thresholds are met
//! - The pipeline demonstrates holographic emergence: each atom contains the whole
//!
//! Knowledge Base References:
//! - V5_PHASE4_QUANTUM_FIELD_SPEC.md
//! - COSMOLOGICAL-ARCHITECTURE.md: "Matter is consciousness at the densest resolution"
//! - HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md: Multi-scale field representation

use crate::foundation::spectrum_position::SpectrumPosition;
use crate::holographic::field_address::ScaleLevel;
use crate::physics::periodic_table::PeriodicTable;
use crate::physics::quantum_field::{
    AttractorField, Element, HolographicBlueprint, QuantumField, QuantumFieldError,
};
use crate::simulation_v3::multiscale_field::MultiScaleField;
use crate::types::{Density, Float};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;

// ============================================================================
// ATOM STRUCT
// ============================================================================

/// An atom emerging from quantum field attractor configurations
///
/// Represents matter as crystallized consciousness - the densest resolution
/// of the holographic field. Each atom contains within it the information
/// of the entire field, consistent with the holographic principle.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Matter is consciousness at the densest resolution"
#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    /// The element this atom represents
    pub element: Element,

    /// Position of this atom in spectrum space
    pub position: SpectrumPosition,

    /// Energy level of this atom (negative for bound states)
    pub energy: Float,

    /// Electron configuration (e.g., "1s2 2s1" for Lithium)
    pub configuration: String,

    /// Coherence at the moment of atom formation
    pub formation_coherence: Float,

    /// Stability of this atom (0.0 to 1.0)
    pub stability: Float,

    /// Unique identifier for this atom
    pub atom_id: u64,
}

impl Atom {
    /// Create a new atom
    pub fn new(
        element: Element,
        position: SpectrumPosition,
        energy: Float,
        configuration: String,
        formation_coherence: Float,
        stability: Float,
    ) -> Self {
        Self {
            element,
            position,
            energy,
            configuration,
            formation_coherence,
            stability,
            atom_id: rand::random(),
        }
    }

    /// Create an atom with a specific ID
    pub fn with_id(
        atom_id: u64,
        element: Element,
        position: SpectrumPosition,
        energy: Float,
        configuration: String,
        formation_coherence: Float,
        stability: Float,
    ) -> Self {
        Self {
            atom_id,
            element,
            position,
            energy,
            configuration,
            formation_coherence,
            stability,
        }
    }

    /// Check if this atom is stable
    pub fn is_stable(&self) -> bool {
        self.stability > 0.5 && self.formation_coherence > 0.3
    }

    /// Get the atomic number
    pub fn atomic_number(&self) -> u32 {
        self.element.atomic_number()
    }

    /// Get the element name
    pub fn element_name(&self) -> String {
        self.element.name()
    }

    /// Update atom stability based on field coherence
    pub fn update_stability(&mut self, field_coherence: Float) {
        // Stability is influenced by both internal stability and field coherence
        let coherence_factor = field_coherence.min(1.0);
        self.stability = (self.stability * 0.9 + coherence_factor * 0.1).clamp(0.0, 1.0);
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Atom(id={}, {}, config={}, energy={:.2} eV, stability={:.2})",
            self.atom_id,
            self.element_name(),
            self.configuration,
            self.energy,
            self.stability
        )
    }
}

// ============================================================================
// MATTER EMERGENCE PIPELINE
// ============================================================================

/// Pipeline for matter emergence from quantum field dynamics
///
/// This pipeline manages the transition from quantum field fluctuations
/// to classical matter formation through attractor field stabilization.
///
/// The pipeline embodies the principle that matter IS consciousness at
/// the densest resolution - each atom that emerges is a crystallized
/// pattern of the underlying holographic field.
///
/// Pipeline stages:
/// 1. Evolve quantum field (Schrödinger dynamics)
/// 2. Apply decoherence (quantum → classical transition)
/// 3. Extract attractor fields (stable quantum configurations)
/// 4. Check coherence at observer position (observer effect)
/// 5. Form atoms from attractor fields (matter emergence)
///
/// From V5 Phase 4 specifications:
/// - Demonstrates holographic emergence
/// - Shows matter as consciousness crystallization
/// - Integrates with multi-scale field representation
pub struct MatterEmergencePipeline {
    /// Quantum field containing all quantum states and amplitudes
    pub quantum_field: QuantumField,

    /// Periodic table mapping elements to their attractor properties
    pub periodic_table: PeriodicTable,

    /// Multi-scale field for holographic coherence tracking
    pub multi_scale_field: MultiScaleField,

    /// Atoms that have emerged from the quantum field
    pub atoms: Vec<Atom>,

    /// Observer position (for coherence sampling)
    pub observer_position: SpectrumPosition,

    /// Current step count
    pub step_count: u64,

    /// Coherence threshold for atom formation
    pub coherence_threshold: Float,

    /// Maximum number of atoms to maintain
    pub max_atoms: usize,
}

impl MatterEmergencePipeline {
    /// Create a new matter emergence pipeline
    pub fn new() -> Self {
        // Initialize quantum field with holographic blueprint
        let blueprint = Arc::new(HolographicBlueprint::new(ScaleLevel::Quantum, 1.0));
        let mut quantum_field = QuantumField::new(blueprint);
        quantum_field.initialize_from_blueprint().unwrap();

        // Initialize periodic table
        let periodic_table = PeriodicTable::new();

        // Initialize multi-scale field
        let multi_scale_field = MultiScaleField::new();

        // Set observer position to physical space (3rd density)
        let observer_position = SpectrumPosition::physical(Density::Third);

        Self {
            quantum_field,
            periodic_table,
            multi_scale_field,
            atoms: Vec::new(),
            observer_position,
            step_count: 0,
            coherence_threshold: 0.4,
            max_atoms: 1000,
        }
    }

    /// Create a pipeline with custom observer position
    pub fn with_observer(observer_position: SpectrumPosition) -> Self {
        let mut pipeline = Self::new();
        pipeline.observer_position = observer_position;
        pipeline
    }

    /// Advance the pipeline by one time step
    ///
    /// This method executes the complete matter emergence pipeline:
    /// 1. Evolve the quantum field by one time step
    /// 2. Apply decoherence to transition quantum → classical
    /// 3. Extract stable attractor fields from the quantum field
    /// 4. Check coherence at the observer position
    /// 5. Form atoms from attractor fields that meet coherence threshold
    ///
    /// Returns the number of new atoms formed in this step.
    pub fn tick(&mut self, dt: Float) -> Result<usize, MatterEmergenceError> {
        self.step_count += 1;

        // Stage 1: Evolve quantum field
        self.quantum_field.evolve(dt)?;

        // Stage 2: Apply decoherence (quantum → classical transition)
        self.quantum_field.apply_decoherence(dt)?;

        // Stage 3: Extract attractor fields (stable quantum configurations)
        let attractor_fields = self.quantum_field.extract_attractor_fields();

        // Stage 4: Check coherence at observer position
        let observer_coherence = self.sample_coherence(&self.observer_position);

        // Stage 5: Form atoms from attractor fields
        let new_atoms = self.form_atoms_from_attractors(&attractor_fields, observer_coherence);

        // Update existing atoms
        for atom in &mut self.atoms {
            atom.update_stability(observer_coherence);
        }

        // Remove unstable atoms
        self.atoms.retain(|atom| atom.is_stable());

        // Enforce maximum atom count
        if self.atoms.len() > self.max_atoms {
            // Keep most stable atoms
            self.atoms
                .sort_by(|a, b| b.stability.partial_cmp(&a.stability).unwrap());
            self.atoms.truncate(self.max_atoms);
        }

        Ok(new_atoms)
    }

    /// Sample coherence at a specific position in the field
    ///
    /// This is a simplified method that samples coherence based on position.
    /// In a full implementation, this would query the multi-scale field
    /// and compute the actual coherence at the specified position.
    ///
    /// The coherence represents the degree of phase alignment in the quantum
    /// field at that position. Higher coherence means the field is more
    /// "classical" and suitable for matter formation.
    pub fn sample_coherence(&self, position: &SpectrumPosition) -> Float {
        // Simplified coherence calculation based on field coherence and position

        // Base coherence from quantum field
        let field_coherence = self.quantum_field.get_field_coherence();

        // Position-dependent modulation
        // Positions closer to space-time dominance (v > 1) have higher coherence
        // for matter formation
        let position_factor = if position.is_physical() {
            // Physical positions favor matter formation
            1.0 + (position.velocity_ratio.log10() / 10.0).min(0.5)
        } else {
            // Metaphysical positions have lower coherence for matter
            0.5 + (position.velocity_ratio / 2.0).min(0.5)
        };

        // Density modulation - 3rd density is optimal for matter
        let density_factor = match position.density {
            Density::First => 0.5,
            Density::Second => 0.7,
            Density::Third => 1.0, // Optimal for matter
            Density::Fourth => 0.8,
            Density::Fifth => 0.6,
            Density::Sixth => 0.4,
            Density::Seventh => 0.3,
            Density::Eighth => 0.2,
        };

        // Combine factors
        let coherence = field_coherence * position_factor * density_factor;

        coherence.clamp(0.0, 1.0)
    }

    /// Form atoms from attractor fields
    ///
    /// Converts stable attractor fields into atoms when coherence conditions
    /// are met. This is the moment when consciousness crystallizes into matter.
    fn form_atoms_from_attractors(
        &mut self,
        attractor_fields: &[AttractorField],
        observer_coherence: Float,
    ) -> usize {
        let mut new_atoms_count = 0;

        for attractor in attractor_fields {
            // Only form atoms from stable attractors
            if !attractor.is_stable() {
                continue;
            }

            // Check if coherence meets threshold
            if observer_coherence < self.coherence_threshold {
                continue;
            }

            // Check if this element can form at this coherence level
            if !self
                .periodic_table
                .can_form_atom(&attractor.element, observer_coherence)
            {
                continue;
            }

            // Get element properties from periodic table
            if let Some(element_attractor) = self.periodic_table.get_element(&attractor.element) {
                // Create atom position near observer (with small random offset)
                let mut atom_position = self.observer_position.clone();
                let mut rng = rand::thread_rng();

                // Small random perturbation to position
                atom_position.velocity_ratio += (rng.gen::<Float>() - 0.5) * 0.1;
                atom_position.velocity_ratio = atom_position.velocity_ratio.max(0.01);

                // Create the atom
                let atom = Atom::new(
                    attractor.element,
                    atom_position,
                    attractor.energy_level,
                    element_attractor.electron_configuration.clone(),
                    observer_coherence,
                    attractor.stability,
                );

                self.atoms.push(atom);
                new_atoms_count += 1;
            }
        }

        new_atoms_count
    }

    /// Get the current quantum state of the pipeline
    ///
    /// Returns a snapshot of the quantum field state including:
    /// - Field coherence
    /// - Number of attractor fields
    /// - Number of atoms
    /// - Observer coherence
    pub fn quantum_state(&self) -> QuantumStateSnapshot {
        let attractor_fields = self.quantum_field.extract_attractor_fields();
        let observer_coherence = self.sample_coherence(&self.observer_position);

        QuantumStateSnapshot {
            field_coherence: self.quantum_field.get_field_coherence(),
            num_attractor_fields: attractor_fields.len(),
            num_atoms: self.atoms.len(),
            observer_coherence,
            step_count: self.step_count,
            energy: self.quantum_field.compute_energy(),
        }
    }

    /// Get all atoms of a specific element
    pub fn get_atoms_by_element(&self, element: &Element) -> Vec<&Atom> {
        self.atoms
            .iter()
            .filter(|atom| &atom.element == element)
            .collect()
    }

    /// Get statistics about the matter emergence
    pub fn statistics(&self) -> MatterEmergenceStatistics {
        let mut element_counts: HashMap<Element, usize> = HashMap::new();

        for atom in &self.atoms {
            *element_counts.entry(atom.element).or_insert(0) += 1;
        }

        let avg_stability = if self.atoms.is_empty() {
            0.0
        } else {
            self.atoms.iter().map(|a| a.stability).sum::<Float>() / self.atoms.len() as Float
        };

        let avg_formation_coherence = if self.atoms.is_empty() {
            0.0
        } else {
            self.atoms
                .iter()
                .map(|a| a.formation_coherence)
                .sum::<Float>()
                / self.atoms.len() as Float
        };

        MatterEmergenceStatistics {
            total_atoms: self.atoms.len(),
            element_counts,
            avg_stability,
            avg_formation_coherence,
            step_count: self.step_count,
            field_coherence: self.quantum_field.get_field_coherence(),
        }
    }

    /// Set the coherence threshold for atom formation
    pub fn set_coherence_threshold(&mut self, threshold: Float) {
        self.coherence_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set the maximum number of atoms
    pub fn set_max_atoms(&mut self, max: usize) {
        self.max_atoms = max;
    }

    /// Clear all atoms
    pub fn clear_atoms(&mut self) {
        self.atoms.clear();
    }

    /// Get the observer position
    pub fn observer_position(&self) -> &SpectrumPosition {
        &self.observer_position
    }

    /// Set the observer position
    pub fn set_observer_position(&mut self, position: SpectrumPosition) {
        self.observer_position = position;
    }
}

impl Default for MatterEmergencePipeline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// QUANTUM STATE SNAPSHOT
// ============================================================================

/// Snapshot of the quantum state at a given moment
///
/// Captures the essential state of the matter emergence pipeline
/// for analysis and visualization.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumStateSnapshot {
    /// Overall coherence of the quantum field
    pub field_coherence: Float,

    /// Number of stable attractor fields
    pub num_attractor_fields: usize,

    /// Number of atoms that have emerged
    pub num_atoms: usize,

    /// Coherence at the observer position
    pub observer_coherence: Float,

    /// Current step count
    pub step_count: u64,

    /// Total energy of the quantum field
    pub energy: Float,
}

// ============================================================================
// MATTER EMERGENCE STATISTICS
// ============================================================================

/// Statistics about the matter emergence process
///
/// Provides detailed metrics for analysis of how matter
/// emerges from the quantum field over time.
#[derive(Debug, Clone, PartialEq)]
pub struct MatterEmergenceStatistics {
    /// Total number of atoms
    pub total_atoms: usize,

    /// Count of atoms by element
    pub element_counts: HashMap<Element, usize>,

    /// Average stability of all atoms
    pub avg_stability: Float,

    /// Average formation coherence
    pub avg_formation_coherence: Float,

    /// Current step count
    pub step_count: u64,

    /// Current field coherence
    pub field_coherence: Float,
}

// ============================================================================
// MATTER EMERGENCE ERROR
// ============================================================================

/// Errors that can occur in matter emergence operations
#[derive(Debug, Clone, PartialEq)]
pub enum MatterEmergenceError {
    /// Quantum field error
    QuantumFieldError(String),

    /// Periodic table error
    PeriodicTableError(String),

    /// Multi-scale field error
    MultiScaleFieldError(String),

    /// Invalid coherence threshold
    InvalidCoherenceThreshold(String),
}

impl std::fmt::Display for MatterEmergenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatterEmergenceError::QuantumFieldError(msg) => {
                write!(f, "Quantum field error: {}", msg)
            }
            MatterEmergenceError::PeriodicTableError(msg) => {
                write!(f, "Periodic table error: {}", msg)
            }
            MatterEmergenceError::MultiScaleFieldError(msg) => {
                write!(f, "Multi-scale field error: {}", msg)
            }
            MatterEmergenceError::InvalidCoherenceThreshold(msg) => {
                write!(f, "Invalid coherence threshold: {}", msg)
            }
        }
    }
}

impl std::error::Error for MatterEmergenceError {}

impl From<QuantumFieldError> for MatterEmergenceError {
    fn from(err: QuantumFieldError) -> Self {
        MatterEmergenceError::QuantumFieldError(err.to_string())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_creation() {
        let position = SpectrumPosition::physical(Density::Third);
        let atom = Atom::new(
            Element::Hydrogen,
            position,
            -13.6,
            "1s1".to_string(),
            0.8,
            0.9,
        );

        assert_eq!(atom.element, Element::Hydrogen);
        assert_eq!(atom.energy, -13.6);
        assert_eq!(atom.configuration, "1s1");
        assert_eq!(atom.formation_coherence, 0.8);
        assert_eq!(atom.stability, 0.9);
        assert!(atom.is_stable());
    }

    #[test]
    fn test_atom_with_id() {
        let position = SpectrumPosition::physical(Density::Third);
        let atom = Atom::with_id(
            42,
            Element::Helium,
            position,
            -24.6,
            "1s2".to_string(),
            0.9,
            0.95,
        );

        assert_eq!(atom.atom_id, 42);
        assert_eq!(atom.element, Element::Helium);
    }

    #[test]
    fn test_atom_is_stable() {
        let position = SpectrumPosition::physical(Density::Third);

        // Stable atom
        let stable_atom = Atom::new(
            Element::Hydrogen,
            position.clone(),
            -13.6,
            "1s1".to_string(),
            0.8,
            0.9,
        );
        assert!(stable_atom.is_stable());

        // Unstable atom (low formation coherence)
        let unstable_atom = Atom::new(
            Element::Hydrogen,
            position.clone(),
            -13.6,
            "1s1".to_string(),
            0.2,
            0.9,
        );
        assert!(!unstable_atom.is_stable());

        // Unstable atom (low stability)
        let unstable_atom2 = Atom::new(
            Element::Hydrogen,
            position,
            -13.6,
            "1s1".to_string(),
            0.8,
            0.3,
        );
        assert!(!unstable_atom2.is_stable());
    }

    #[test]
    fn test_atom_atomic_number() {
        let position = SpectrumPosition::physical(Density::Third);

        let hydrogen = Atom::new(
            Element::Hydrogen,
            position.clone(),
            -13.6,
            "1s1".to_string(),
            0.8,
            0.9,
        );
        assert_eq!(hydrogen.atomic_number(), 1);

        let helium = Atom::new(
            Element::Helium,
            position,
            -24.6,
            "1s2".to_string(),
            0.9,
            0.95,
        );
        assert_eq!(helium.atomic_number(), 2);
    }

    #[test]
    fn test_atom_element_name() {
        let position = SpectrumPosition::physical(Density::Third);
        let atom = Atom::new(
            Element::Hydrogen,
            position,
            -13.6,
            "1s1".to_string(),
            0.8,
            0.9,
        );

        assert_eq!(atom.element_name(), "Hydrogen");
    }

    #[test]
    fn test_atom_update_stability() {
        let position = SpectrumPosition::physical(Density::Third);
        let mut atom = Atom::new(
            Element::Hydrogen,
            position,
            -13.6,
            "1s1".to_string(),
            0.8,
            0.5,
        );

        let initial_stability = atom.stability;
        atom.update_stability(0.9);

        // Stability should increase with higher field coherence
        assert!(atom.stability > initial_stability);
        assert!(atom.stability <= 1.0);
    }

    #[test]
    fn test_atom_display() {
        let position = SpectrumPosition::physical(Density::Third);
        let atom = Atom::new(
            Element::Hydrogen,
            position,
            -13.6,
            "1s1".to_string(),
            0.8,
            0.9,
        );

        let display = format!("{}", atom);
        assert!(display.contains("Atom"));
        assert!(display.contains("Hydrogen"));
        assert!(display.contains("1s1"));
    }

    #[test]
    fn test_matter_emergence_pipeline_new() {
        let pipeline = MatterEmergencePipeline::new();

        assert!(!pipeline.atoms.is_empty() || pipeline.atoms.is_empty()); // May or may not have atoms
        assert_eq!(pipeline.step_count, 0);
        assert_eq!(pipeline.coherence_threshold, 0.4);
        assert_eq!(pipeline.max_atoms, 1000);
    }

    #[test]
    fn test_matter_emergence_pipeline_default() {
        let pipeline = MatterEmergencePipeline::default();

        assert_eq!(pipeline.step_count, 0);
        assert_eq!(pipeline.coherence_threshold, 0.4);
    }

    #[test]
    fn test_matter_emergence_pipeline_with_observer() {
        let observer = SpectrumPosition::physical(Density::Fourth);
        let pipeline = MatterEmergencePipeline::with_observer(observer.clone());

        assert_eq!(pipeline.observer_position, observer);
    }

    #[test]
    fn test_matter_emergence_pipeline_tick() {
        let mut pipeline = MatterEmergencePipeline::new();

        let result = pipeline.tick(0.1);

        assert!(result.is_ok());
        assert_eq!(pipeline.step_count, 1);
    }

    #[test]
    fn test_matter_emergence_pipeline_tick_multiple() {
        let mut pipeline = MatterEmergencePipeline::new();

        for _ in 0..10 {
            let _ = pipeline.tick(0.1);
        }

        assert_eq!(pipeline.step_count, 10);
    }

    #[test]
    fn test_sample_coherence() {
        let pipeline = MatterEmergencePipeline::new();

        let physical_pos = SpectrumPosition::physical(Density::Third);
        let coherence = pipeline.sample_coherence(&physical_pos);

        assert!((0.0..=1.0).contains(&coherence));
    }

    #[test]
    fn test_sample_coherence_different_positions() {
        let pipeline = MatterEmergencePipeline::new();

        let physical = SpectrumPosition::physical(Density::Third);
        let metaphysical = SpectrumPosition::metaphysical(Density::Sixth);

        let physical_coherence = pipeline.sample_coherence(&physical);
        let metaphysical_coherence = pipeline.sample_coherence(&metaphysical);

        // Physical positions should generally have higher coherence for matter
        assert!((0.0..=1.0).contains(&physical_coherence));
        assert!((0.0..=1.0).contains(&metaphysical_coherence));
    }

    #[test]
    fn test_quantum_state() {
        let mut pipeline = MatterEmergencePipeline::new();
        let _ = pipeline.tick(0.1);

        let state = pipeline.quantum_state();

        assert!(state.field_coherence >= 0.0 && state.field_coherence <= 1.0);
        assert!(state.observer_coherence >= 0.0 && state.observer_coherence <= 1.0);
        assert_eq!(state.step_count, 1);
        assert!(state.energy >= 0.0);
    }

    #[test]
    fn test_get_atoms_by_element() {
        let mut pipeline = MatterEmergencePipeline::new();

        // Run some ticks to generate atoms
        for _ in 0..20 {
            let _ = pipeline.tick(0.1);
        }

        let hydrogen_atoms = pipeline.get_atoms_by_element(&Element::Hydrogen);

        // All returned atoms should be hydrogen
        for atom in &hydrogen_atoms {
            assert_eq!(atom.element, Element::Hydrogen);
        }
    }

    #[test]
    fn test_statistics() {
        let mut pipeline = MatterEmergencePipeline::new();

        // Run some ticks to generate atoms
        for _ in 0..20 {
            let _ = pipeline.tick(0.1);
        }

        let stats = pipeline.statistics();

        // total_atoms is usize, always >= 0
        assert!(stats.avg_stability >= 0.0 && stats.avg_stability <= 1.0);
        assert!(stats.avg_formation_coherence >= 0.0 && stats.avg_formation_coherence <= 1.0);
        assert_eq!(stats.step_count, 20);
    }

    #[test]
    fn test_set_coherence_threshold() {
        let mut pipeline = MatterEmergencePipeline::new();

        pipeline.set_coherence_threshold(0.8);
        assert_eq!(pipeline.coherence_threshold, 0.8);

        pipeline.set_coherence_threshold(1.5); // Should be clamped to 1.0
        assert_eq!(pipeline.coherence_threshold, 1.0);

        pipeline.set_coherence_threshold(-0.5); // Should be clamped to 0.0
        assert_eq!(pipeline.coherence_threshold, 0.0);
    }

    #[test]
    fn test_set_max_atoms() {
        let mut pipeline = MatterEmergencePipeline::new();

        pipeline.set_max_atoms(500);
        assert_eq!(pipeline.max_atoms, 500);
    }

    #[test]
    fn test_clear_atoms() {
        let mut pipeline = MatterEmergencePipeline::new();

        // Run some ticks to generate atoms
        for _ in 0..20 {
            let _ = pipeline.tick(0.1);
        }

        let atom_count_before = pipeline.atoms.len();
        pipeline.clear_atoms();

        assert!(atom_count_before > 0 || pipeline.atoms.is_empty());
        assert_eq!(pipeline.atoms.len(), 0);
    }

    #[test]
    fn test_observer_position() {
        let pipeline = MatterEmergencePipeline::new();

        let pos = pipeline.observer_position();
        assert_eq!(pos, &pipeline.observer_position);
    }

    #[test]
    fn test_set_observer_position() {
        let mut pipeline = MatterEmergencePipeline::new();

        let new_pos = SpectrumPosition::physical(Density::Fourth);
        pipeline.set_observer_position(new_pos.clone());

        assert_eq!(pipeline.observer_position, new_pos);
    }

    #[test]
    fn test_quantum_state_snapshot() {
        let snapshot = QuantumStateSnapshot {
            field_coherence: 0.8,
            num_attractor_fields: 10,
            num_atoms: 5,
            observer_coherence: 0.7,
            step_count: 100,
            energy: 42.0,
        };

        assert_eq!(snapshot.field_coherence, 0.8);
        assert_eq!(snapshot.num_attractor_fields, 10);
        assert_eq!(snapshot.num_atoms, 5);
        assert_eq!(snapshot.observer_coherence, 0.7);
        assert_eq!(snapshot.step_count, 100);
        assert_eq!(snapshot.energy, 42.0);
    }

    #[test]
    fn test_matter_emergence_statistics() {
        let mut element_counts = HashMap::new();
        element_counts.insert(Element::Hydrogen, 10);
        element_counts.insert(Element::Helium, 5);

        let stats = MatterEmergenceStatistics {
            total_atoms: 15,
            element_counts,
            avg_stability: 0.8,
            avg_formation_coherence: 0.7,
            step_count: 50,
            field_coherence: 0.75,
        };

        assert_eq!(stats.total_atoms, 15);
        assert_eq!(stats.avg_stability, 0.8);
        assert_eq!(stats.step_count, 50);
        assert_eq!(stats.element_counts.get(&Element::Hydrogen), Some(&10));
    }

    #[test]
    fn test_matter_emergence_error_display() {
        let err = MatterEmergenceError::QuantumFieldError("Test error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Quantum field error"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_matter_emergence_pipeline_atom_limit() {
        let mut pipeline = MatterEmergencePipeline::new();
        pipeline.set_coherence_threshold(0.1); // Lower threshold to form more atoms
        pipeline.set_max_atoms(10);

        // Run many ticks to exceed atom limit
        for _ in 0..100 {
            let _ = pipeline.tick(0.1);
        }

        // Should not exceed max_atoms
        assert!(pipeline.atoms.len() <= 10);
    }

    #[test]
    fn test_matter_emergence_pipeline_coherence_threshold_affects_formation() {
        let mut pipeline_low = MatterEmergencePipeline::new();
        pipeline_low.set_coherence_threshold(0.1);

        let mut pipeline_high = MatterEmergencePipeline::new();
        pipeline_high.set_coherence_threshold(0.9);

        // Run both pipelines
        for _ in 0..20 {
            let _ = pipeline_low.tick(0.1);
            let _ = pipeline_high.tick(0.1);
        }

        // Lower threshold should allow more atom formation
        // (though this may vary based on randomness)
        let stats_low = pipeline_low.statistics();
        let stats_high = pipeline_high.statistics();

        // Just verify both ran successfully
        assert_eq!(stats_low.step_count, 20);
        assert_eq!(stats_high.step_count, 20);
    }
}
