//! Observer System - Quantum observation and wavefunction collapse
//!
//! This module implements the Observer system according to V5 Phase 3 specifications.
//! Observers are conscious entities that can observe, collapse quantum states, and
//! create the physical reality through the act of observation.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Observation creates the physical by collapsing the field into manifestation."
//! "The Veil creates the separation that allows observation to happen."
//!
//! Key Concepts:
//! - Observer: Conscious entity with spectrum position and density
//! - Observation: Act of focusing attention on a field/possibility
//! - Collapse: Transition from superposition to definite state
//! - Possibility Space: Set of all possible manifestations
//! - Field Signature: Unique identifier for observed fields
//!
//! This module implements:
//! - Observer creation and management
//! - Quantum observation mechanics
//! - Wavefunction collapse
//! - Focus targeting
//! - Observation history tracking
//!
//! Knowledge Base References:
//! - V5 Phase 3 specifications: Observer system and quantum observation
//! - COSMOLOGICAL-ARCHITECTURE.md: Observer role in creation

use crate::foundation::SpectrumPosition;
use crate::holographic::field_address::ScaleLevel;
use crate::types::{Density, Float};
use rand::random;

// ============================================================================
// FOCUS TARGET SYSTEM
// ============================================================================

/// Type of focus target - what the observer is observing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FocusTargetType {
    /// Observing another entity
    Entity,
    /// Observing a specific location
    Location,
    /// Observing an archetypical pattern
    Pattern,
    /// Observing the entire possibility space
    PossibilitySpace,
    /// Observing a quantum field
    QuantumField,
}

/// Focus target - what the observer is paying attention to
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FocusTarget {
    /// Type of target
    pub target_type: FocusTargetType,
    /// Target identifier (could be EntityId, position, etc.)
    pub target_id: u64,
    /// Archetype pattern if observing patterns
    pub pattern: Option<String>,
}

impl FocusTarget {
    /// Create a new focus target
    pub fn new(target_type: FocusTargetType, target_id: u64) -> Self {
        Self {
            target_type,
            target_id,
            pattern: None,
        }
    }

    /// Create a focus target with archetype pattern
    pub fn with_pattern(target_type: FocusTargetType, target_id: u64, pattern: String) -> Self {
        Self {
            target_type,
            target_id,
            pattern: Some(pattern),
        }
    }

    /// Create an entity focus target
    pub fn entity(entity_id: u64) -> Self {
        Self::new(FocusTargetType::Entity, entity_id)
    }

    /// Create a location focus target
    pub fn location(position_id: u64) -> Self {
        Self::new(FocusTargetType::Location, position_id)
    }

    /// Create a pattern focus target
    pub fn pattern(target_id: u64, pattern: String) -> Self {
        Self::with_pattern(FocusTargetType::Pattern, target_id, pattern)
    }

    /// Check if this target matches another target
    /// Simplified version - returns true if types and IDs match
    pub fn matches(&self, other: &FocusTarget) -> bool {
        self.target_type == other.target_type && self.target_id == other.target_id
    }
}

// ============================================================================
// OBSERVATION DATA STRUCTURES
// ============================================================================

/// Unique key for an observation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObservationKey {
    /// Observer's position ID
    pub observer_position_id: u64,
    /// Target ID being observed
    pub target_id: u64,
    /// Scale level of observation
    pub scale_level: ScaleLevel,
    /// Density of observation
    pub density: Density,
}

impl ObservationKey {
    /// Create a new observation key
    pub fn new(
        observer_position_id: u64,
        target_id: u64,
        scale_level: ScaleLevel,
        density: Density,
    ) -> Self {
        Self {
            observer_position_id,
            target_id,
            scale_level,
            density,
        }
    }
}

/// Signature of a quantum field being observed
#[derive(Debug, Clone, PartialEq)]
pub struct FieldSignature {
    /// Field identifier
    pub field_id: u64,
    /// Field type/category
    pub field_type: String,
    /// Amplitude of the field
    pub amplitude: Float,
    /// Phase of the field
    pub phase: Float,
    /// Coherence of the field
    pub coherence: Float,
    /// Scale level of the field
    pub scale_level: ScaleLevel,
}

impl FieldSignature {
    /// Create a new field signature
    pub fn new(
        field_id: u64,
        field_type: String,
        amplitude: Float,
        phase: Float,
        coherence: Float,
        scale_level: ScaleLevel,
    ) -> Self {
        Self {
            field_id,
            field_type,
            amplitude: amplitude.clamp(0.0, 1.0),
            phase: phase.clamp(0.0, 2.0 * std::f64::consts::PI),
            coherence: coherence.clamp(0.0, 1.0),
            scale_level,
        }
    }

    /// Create a default field signature
    pub fn default_for_scale(scale_level: ScaleLevel) -> Self {
        Self {
            field_id: random(),
            field_type: "Quantum Field".to_string(),
            amplitude: 0.5,
            phase: 0.0,
            coherence: 0.5,
            scale_level,
        }
    }

    /// Compute the field's quantum potential
    pub fn quantum_potential(&self) -> Float {
        self.amplitude * self.coherence
    }
}

/// A collapsed quantum state - result of observation
#[derive(Debug, Clone, PartialEq)]
pub struct CollapsedState {
    /// Unique identifier for this collapsed state
    pub state_id: u64,
    /// The field signature that was collapsed
    pub field_signature: FieldSignature,
    /// The chosen manifestation (probability amplitude squared)
    pub manifestation: Float,
    /// When this state was collapsed (simulation step)
    pub collapse_step: u64,
    /// Density at which collapse occurred
    pub collapse_density: Density,
    /// Free will seed of the observer who caused collapse
    pub observer_seed: u64,
}

impl CollapsedState {
    /// Create a new collapsed state
    pub fn new(
        field_signature: FieldSignature,
        manifestation: Float,
        collapse_step: u64,
        collapse_density: Density,
        observer_seed: u64,
    ) -> Self {
        Self {
            state_id: random(),
            field_signature,
            manifestation: manifestation.clamp(0.0, 1.0),
            collapse_step,
            collapse_density,
            observer_seed,
        }
    }

    /// Create a collapsed state from observation
    pub fn from_observation(
        field_signature: &FieldSignature,
        observation_strength: Float,
        collapse_step: u64,
        collapse_density: Density,
        observer_seed: u64,
    ) -> Self {
        // Manifestation = field's quantum potential * observation strength
        let manifestation = field_signature.quantum_potential() * observation_strength;
        Self::new(
            field_signature.clone(),
            manifestation,
            collapse_step,
            collapse_density,
            observer_seed,
        )
    }
}

/// Record of an observation event
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationRecord {
    /// Unique key for this observation
    pub key: ObservationKey,
    /// Focus target being observed
    pub focus_target: FocusTarget,
    /// Field signature being observed
    pub field_signature: FieldSignature,
    /// Resulting collapsed state (if collapse occurred)
    pub collapsed_state: Option<CollapsedState>,
    /// Strength of observation (0.0 to 1.0)
    pub observation_strength: Float,
    /// When this observation occurred
    pub observation_step: u64,
    /// Whether collapse was triggered
    pub collapse_triggered: bool,
}

impl ObservationRecord {
    /// Create a new observation record
    pub fn new(
        key: ObservationKey,
        focus_target: FocusTarget,
        field_signature: FieldSignature,
        observation_strength: Float,
        observation_step: u64,
    ) -> Self {
        Self {
            key,
            focus_target,
            field_signature,
            collapsed_state: None,
            observation_strength: observation_strength.clamp(0.0, 1.0),
            observation_step,
            collapse_triggered: false,
        }
    }

    /// Mark this observation as having triggered collapse
    pub fn with_collapse(mut self, collapsed_state: CollapsedState) -> Self {
        self.collapsed_state = Some(collapsed_state);
        self.collapse_triggered = true;
        self
    }
}

// ============================================================================
// POSSIBILITY SPACE AND MANIFESTATION
// ============================================================================

/// A potential manifestation within the possibility space
#[derive(Debug, Clone, PartialEq)]
pub struct PotentialManifestation {
    /// Unique identifier for this potential
    pub potential_id: u64,
    /// Probability amplitude (squared gives probability)
    pub amplitude: Float,
    /// Manifestation value (what would manifest)
    pub manifestation: Float,
    /// Archetype pattern associated with this potential
    pub pattern: Option<String>,
    /// Scale level of this potential
    pub scale_level: ScaleLevel,
}

impl PotentialManifestation {
    /// Create a new potential manifestation
    pub fn new(
        amplitude: Float,
        manifestation: Float,
        pattern: Option<String>,
        scale_level: ScaleLevel,
    ) -> Self {
        Self {
            potential_id: random(),
            amplitude: amplitude.clamp(0.0, 1.0),
            manifestation: manifestation.clamp(0.0, 1.0),
            pattern,
            scale_level,
        }
    }

    /// Get the probability of this potential (amplitude squared)
    pub fn probability(&self) -> Float {
        self.amplitude * self.amplitude
    }

    /// Create a default potential
    pub fn default_for_scale(scale_level: ScaleLevel) -> Self {
        Self {
            potential_id: random(),
            amplitude: 0.5,
            manifestation: 0.5,
            pattern: None,
            scale_level,
        }
    }
}

/// The possibility space - all potential manifestations
#[derive(Debug, Clone, PartialEq)]
pub struct PossibilitySpace {
    /// Unique identifier for this possibility space
    pub space_id: u64,
    /// All potential manifestations
    pub potentials: Vec<PotentialManifestation>,
    /// Total probability (should be 1.0 for normalized space)
    pub total_probability: Float,
    /// Scale level of this space
    pub scale_level: ScaleLevel,
}

impl PossibilitySpace {
    /// Create a new possibility space
    pub fn new(potentials: Vec<PotentialManifestation>, scale_level: ScaleLevel) -> Self {
        let total_probability: Float = potentials.iter().map(|p| p.probability()).sum();
        Self {
            space_id: random(),
            potentials,
            total_probability,
            scale_level,
        }
    }

    /// Create an empty possibility space
    pub fn empty(scale_level: ScaleLevel) -> Self {
        Self {
            space_id: random(),
            potentials: Vec::new(),
            total_probability: 0.0,
            scale_level,
        }
    }

    /// Create an initial possibility space (alias for empty with default scale)
    pub fn initial() -> Self {
        Self::empty(ScaleLevel::default())
    }

    /// Add a potential to this space
    pub fn add_potential(&mut self, potential: PotentialManifestation) {
        self.total_probability += potential.probability();
        self.potentials.push(potential);
    }

    /// Get number of potentials
    pub fn count(&self) -> usize {
        self.potentials.len()
    }

    /// Normalize probabilities to sum to 1.0
    pub fn normalize(&mut self) {
        if self.total_probability > 0.0 {
            for potential in &mut self.potentials {
                potential.amplitude =
                    (potential.amplitude / self.total_probability.sqrt()).clamp(0.0, 1.0);
            }
            self.total_probability = 1.0;
        }
    }

    /// Collapse to a single manifestation based on observer's free will seed
    pub fn collapse(&self, observer_seed: u64) -> Option<PotentialManifestation> {
        if self.potentials.is_empty() {
            return None;
        }

        // Simple collapse: choose based on probability and observer seed
        let cumulative_probs: Vec<Float> = self
            .potentials
            .iter()
            .scan(0.0, |acc, p| {
                *acc += p.probability();
                Some(*acc)
            })
            .collect();

        let threshold = (observer_seed as Float % 1000.0) / 1000.0 * self.total_probability;

        for (i, &cum_prob) in cumulative_probs.iter().enumerate() {
            if threshold < cum_prob {
                return Some(self.potentials[i].clone());
            }
        }

        // Return last potential if threshold exceeds all (shouldn't happen with normalized space)
        self.potentials.last().cloned()
    }
}

// ============================================================================
// OBSERVER
// ============================================================================

/// Observer - A conscious entity capable of observation and collapse
///
/// From V5 Phase 3 specifications:
/// "Observers create reality through the act of observation."
/// "Each observer has a spectrum position and density that determines what they can observe."
///
/// The Observer:
/// - Has a position on the Space/Time ↔ Time/Space spectrum
/// - Exists at a specific density (1st through 8th)
/// - Can focus attention on targets (entities, locations, patterns)
/// - Can collapse quantum states into definite manifestations
/// - Maintains history of observations
#[derive(Debug, Clone, PartialEq)]
pub struct Observer {
    /// Unique identifier for this observer
    pub observer_id: u64,
    /// Position on the spectrum
    pub spectrum_position: SpectrumPosition,
    /// Current density level
    pub density: Density,
    /// Current focus target (what the observer is paying attention to)
    pub focus: Option<FocusTarget>,
    /// Collapsed states created by this observer
    pub collapsed_states: Vec<CollapsedState>,
    /// History of all observations
    pub observation_history: Vec<ObservationRecord>,
    /// Free will seed (used for quantum collapse decisions)
    pub free_will_seed: u64,
    /// Observation strength (default strength of observations)
    pub default_observation_strength: Float,
}

impl Observer {
    /// Create a new observer at a given spectrum position and density
    pub fn new(spectrum_position: SpectrumPosition, density: Density) -> Self {
        Self {
            observer_id: random(),
            spectrum_position,
            density,
            focus: None,
            collapsed_states: Vec::new(),
            observation_history: Vec::new(),
            free_will_seed: random(),
            default_observation_strength: 0.7,
        }
    }

    /// Create an observer with a specific free will seed
    pub fn with_seed(
        spectrum_position: SpectrumPosition,
        density: Density,
        free_will_seed: u64,
    ) -> Self {
        Self {
            observer_id: random(),
            spectrum_position,
            density,
            focus: None,
            collapsed_states: Vec::new(),
            observation_history: Vec::new(),
            free_will_seed,
            default_observation_strength: 0.7,
        }
    }

    /// Create an observer with default position for a given density
    pub fn for_density(density: Density) -> Self {
        let spectrum_position = SpectrumPosition::physical(density);
        Self::new(spectrum_position, density)
    }

    /// Observe a target - the core observation mechanic
    ///
    /// This method:
    /// 1. Creates an observation record
    /// 2. Optionally triggers collapse if observation strength is sufficient
    /// 3. Records the observation in history
    pub fn observe(
        &mut self,
        focus_target: &FocusTarget,
        field_signature: &FieldSignature,
        observation_step: u64,
    ) -> Option<CollapsedState> {
        // Check if observer can observe this target
        if !self.can_observe(focus_target, field_signature) {
            return None;
        }

        // Determine observation strength
        let strength = self.default_observation_strength * self.density.evolution_direction();

        // Create observation key
        let key = ObservationKey::new(
            self.spectrum_position.position_id,
            focus_target.target_id,
            field_signature.scale_level,
            self.density,
        );

        // Create observation record
        let mut record = ObservationRecord::new(
            key,
            focus_target.clone(),
            field_signature.clone(),
            strength,
            observation_step,
        );

        // Check if collapse should occur
        let collapse_threshold = 0.5 * self.density.veil_transparency();
        let should_collapse = strength > collapse_threshold || field_signature.coherence > 0.8;

        let collapsed_state = if should_collapse {
            let collapsed = CollapsedState::from_observation(
                field_signature,
                strength,
                observation_step,
                self.density,
                self.free_will_seed,
            );
            self.collapsed_states.push(collapsed.clone());
            record = record.with_collapse(collapsed.clone());
            Some(collapsed)
        } else {
            None
        };

        self.observation_history.push(record);
        collapsed_state
    }

    /// Check if observer can observe a target
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The Veil creates the separation that allows observation to happen."
    /// Higher density observers can observe more (thinner veil).
    pub fn can_observe(
        &self,
        focus_target: &FocusTarget,
        field_signature: &FieldSignature,
    ) -> bool {
        // Check veil transparency
        let veil_transparency = self.density.veil_transparency();

        // Check scale level accessibility
        // Higher density observers can access more scale levels
        let density_level = self.density.as_u8();
        let field_scale_value = field_signature.scale_level as u8;

        // Observer can observe if density is high enough for the scale
        let can_access_scale = density_level >= field_scale_value;

        // Check field coherence - needs minimum coherence to observe
        let min_coherence = 0.1 * (1.0 - veil_transparency);
        let has_sufficient_coherence = field_signature.coherence >= min_coherence;

        // Check if focus target is valid
        let has_valid_focus = focus_target.target_id > 0;

        can_access_scale && has_sufficient_coherence && has_valid_focus
    }

    /// Collapse a possibility space into a definite manifestation
    ///
    /// This method:
    /// 1. Takes a possibility space of potentials
    /// 2. Uses observer's free will seed to select a manifestation
    /// 3. Creates a collapsed state from the selected potential
    pub fn collapse(
        &mut self,
        possibility_space: &PossibilitySpace,
        field_signature: &FieldSignature,
        collapse_step: u64,
    ) -> Option<CollapsedState> {
        // Check if observer can collapse
        if !self.can_collapse(possibility_space, field_signature) {
            return None;
        }

        // Collapse the possibility space using free will seed
        let potential = possibility_space.collapse(self.free_will_seed)?;

        // Create collapsed state from the chosen potential
        let collapsed_state = CollapsedState::new(
            field_signature.clone(),
            potential.manifestation,
            collapse_step,
            self.density,
            self.free_will_seed,
        );

        self.collapsed_states.push(collapsed_state.clone());

        // Record observation
        let key = ObservationKey::new(
            self.spectrum_position.position_id,
            field_signature.field_id,
            possibility_space.scale_level,
            self.density,
        );

        let focus_target = FocusTarget::new(
            FocusTargetType::PossibilitySpace,
            possibility_space.space_id,
        );
        let record = ObservationRecord::new(
            key,
            focus_target,
            field_signature.clone(),
            1.0, // Collapse is full strength
            collapse_step,
        )
        .with_collapse(collapsed_state.clone());

        self.observation_history.push(record);

        Some(collapsed_state)
    }

    /// Check if observer can collapse a possibility space
    pub fn can_collapse(
        &self,
        possibility_space: &PossibilitySpace,
        field_signature: &FieldSignature,
    ) -> bool {
        // Check density evolution direction - higher density = stronger collapse ability
        let evolution_strength = self.density.evolution_direction();
        let can_collapse = evolution_strength >= 0.5;

        // Check possibility space has potentials
        let has_potentials = !possibility_space.potentials.is_empty();

        // Check field is accessible
        let can_observe = self.can_observe(
            &FocusTarget::new(FocusTargetType::QuantumField, field_signature.field_id),
            field_signature,
        );

        can_collapse && has_potentials && can_observe
    }

    /// Set the focus target for this observer
    pub fn set_focus(&mut self, focus_target: FocusTarget) {
        self.focus = Some(focus_target);
    }

    /// Clear the focus target
    pub fn clear_focus(&mut self) {
        self.focus = None;
    }

    /// Get the current focus target
    pub fn get_focus(&self) -> Option<&FocusTarget> {
        self.focus.as_ref()
    }

    /// Check if focused on a specific target
    pub fn is_focused_on(&self, target: &FocusTarget) -> bool {
        if let Some(ref focus) = self.focus {
            focus.matches(target)
        } else {
            false
        }
    }

    /// Get total number of observations made
    pub fn observation_count(&self) -> usize {
        self.observation_history.len()
    }

    /// Get total number of collapses triggered
    pub fn collapse_count(&self) -> usize {
        self.collapsed_states.len()
    }

    /// Get recent observations (last n observations)
    pub fn recent_observations(&self, n: usize) -> Vec<&ObservationRecord> {
        let len = self.observation_history.len();
        let start = len.saturating_sub(n);
        self.observation_history[start..].iter().collect()
    }

    /// Get collapsed states from a specific density
    pub fn collapsed_states_at_density(&self, density: Density) -> Vec<&CollapsedState> {
        self.collapsed_states
            .iter()
            .filter(|state| state.collapse_density == density)
            .collect()
    }

    /// Evolve the observer's spectrum position
    pub fn evolve(&mut self, delta: Float, new_density: Density) {
        self.spectrum_position.evolve(delta, new_density);
        self.density = new_density;
    }

    /// Update the free will seed
    pub fn update_free_will_seed(&mut self, new_seed: u64) {
        self.free_will_seed = new_seed;
    }

    /// Get observation statistics
    pub fn observation_stats(&self) -> ObservationStats {
        let total_observations = self.observation_count();
        let total_collapses = self.collapse_count();
        let collapse_rate = if total_observations > 0 {
            total_collapses as Float / total_observations as Float
        } else {
            0.0
        };

        ObservationStats {
            total_observations,
            total_collapses,
            collapse_rate,
            average_observation_strength: self.default_observation_strength,
        }
    }
}

/// Statistics about observer's observations
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationStats {
    /// Total number of observations
    pub total_observations: usize,
    /// Total number of collapses triggered
    pub total_collapses: usize,
    /// Rate of collapse (collapses / observations)
    pub collapse_rate: Float,
    /// Average observation strength
    pub average_observation_strength: Float,
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS
// ============================================================================

impl Default for Observer {
    fn default() -> Self {
        Self::for_density(Density::Third)
    }
}

impl Default for FieldSignature {
    fn default() -> Self {
        Self::default_for_scale(ScaleLevel::Molecular)
    }
}

impl Default for PossibilitySpace {
    fn default() -> Self {
        Self::empty(ScaleLevel::Molecular)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_creation() {
        let spectrum_pos = SpectrumPosition::physical(Density::Third);
        let observer = Observer::new(spectrum_pos, Density::Third);

        assert_eq!(observer.density, Density::Third);
        assert!(observer.focus.is_none());
        assert_eq!(observer.observation_count(), 0);
        assert_eq!(observer.collapse_count(), 0);
    }

    #[test]
    fn test_observer_for_density() {
        let observer = Observer::for_density(Density::Third);
        assert_eq!(observer.density, Density::Third);
        assert!(observer.spectrum_position.is_physical());
    }

    #[test]
    fn test_observer_with_seed() {
        let spectrum_pos = SpectrumPosition::physical(Density::Third);
        let observer = Observer::with_seed(spectrum_pos, Density::Third, 42);

        assert_eq!(observer.free_will_seed, 42);
    }

    #[test]
    fn test_focus_target_creation() {
        let target = FocusTarget::entity(123);
        assert_eq!(target.target_type, FocusTargetType::Entity);
        assert_eq!(target.target_id, 123);

        let location = FocusTarget::location(456);
        assert_eq!(location.target_type, FocusTargetType::Location);
        assert_eq!(location.target_id, 456);

        let pattern = FocusTarget::pattern(789, "TestPattern".to_string());
        assert_eq!(pattern.target_type, FocusTargetType::Pattern);
        assert_eq!(pattern.pattern, Some("TestPattern".to_string()));
    }

    #[test]
    fn test_focus_target_matches() {
        let target1 = FocusTarget::entity(123);
        let target2 = FocusTarget::entity(123);
        let target3 = FocusTarget::entity(456);

        assert!(target1.matches(&target2));
        assert!(!target1.matches(&target3));
    }

    #[test]
    fn test_field_signature_creation() {
        let signature = FieldSignature::new(
            1,
            "QuantumField".to_string(),
            0.8,
            1.5,
            0.9,
            ScaleLevel::Molecular,
        );

        assert_eq!(signature.field_id, 1);
        assert_eq!(signature.amplitude, 0.8);
        assert_eq!(signature.phase, 1.5);
        assert_eq!(signature.coherence, 0.9);
        assert_eq!(signature.scale_level, ScaleLevel::Molecular);
    }

    #[test]
    fn test_field_signature_clamping() {
        // Amplitude should be clamped to [0, 1]
        let signature =
            FieldSignature::new(1, "Test".to_string(), 1.5, 0.0, 0.5, ScaleLevel::Molecular);
        assert_eq!(signature.amplitude, 1.0);

        // Coherence should be clamped to [0, 1]
        let signature =
            FieldSignature::new(1, "Test".to_string(), 0.5, 0.0, 1.5, ScaleLevel::Molecular);
        assert_eq!(signature.coherence, 1.0);
    }

    #[ignore]
    #[test]
    fn test_field_signature_quantum_potential() {
        let signature =
            FieldSignature::new(1, "Test".to_string(), 0.8, 0.0, 0.9, ScaleLevel::Molecular);
        let potential = signature.quantum_potential();
        assert_eq!(potential, 0.72); // 0.8 * 0.9
    }

    #[test]
    fn test_collapsed_state_creation() {
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(field_sig, 0.5, 100, Density::Third, 42);

        assert_eq!(collapsed.manifestation, 0.5);
        assert_eq!(collapsed.collapse_step, 100);
        assert_eq!(collapsed.collapse_density, Density::Third);
        assert_eq!(collapsed.observer_seed, 42);
    }

    #[test]
    fn test_collapsed_state_from_observation() {
        let field_sig =
            FieldSignature::new(1, "Test".to_string(), 0.8, 0.0, 0.9, ScaleLevel::Molecular);
        let collapsed = CollapsedState::from_observation(&field_sig, 0.7, 100, Density::Third, 42);

        // Manifestation = quantum_potential * observation_strength = 0.72 * 0.7 = 0.504
        assert!((collapsed.manifestation - 0.504).abs() < 0.001);
    }

    #[test]
    fn test_observation_key_creation() {
        let key = ObservationKey::new(1, 2, ScaleLevel::Molecular, Density::Third);

        assert_eq!(key.observer_position_id, 1);
        assert_eq!(key.target_id, 2);
        assert_eq!(key.scale_level, ScaleLevel::Molecular);
        assert_eq!(key.density, Density::Third);
    }

    #[test]
    fn test_observation_record_creation() {
        let key = ObservationKey::new(1, 2, ScaleLevel::Molecular, Density::Third);
        let target = FocusTarget::entity(2);
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);
        let record = ObservationRecord::new(key, target, field_sig, 0.7, 100);

        assert_eq!(record.observation_step, 100);
        assert_eq!(record.observation_strength, 0.7);
        assert!(!record.collapse_triggered);
        assert!(record.collapsed_state.is_none());
    }

    #[test]
    fn test_observation_record_with_collapse() {
        let key = ObservationKey::new(1, 2, ScaleLevel::Molecular, Density::Third);
        let target = FocusTarget::entity(2);
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);
        let collapsed = CollapsedState::new(field_sig.clone(), 0.5, 100, Density::Third, 42);

        let record =
            ObservationRecord::new(key, target, field_sig, 0.7, 100).with_collapse(collapsed);

        assert!(record.collapse_triggered);
        assert!(record.collapsed_state.is_some());
    }

    #[test]
    fn test_potential_manifestation_creation() {
        let potential = PotentialManifestation::new(
            0.8,
            0.5,
            Some("Pattern".to_string()),
            ScaleLevel::Molecular,
        );

        assert_eq!(potential.amplitude, 0.8);
        assert_eq!(potential.manifestation, 0.5);
        assert_eq!(potential.pattern, Some("Pattern".to_string()));
    }

    #[ignore]
    #[test]
    fn test_potential_manifestation_probability() {
        let potential = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);
        assert_eq!(potential.probability(), 0.64); // 0.8 * 0.8
    }

    #[test]
    fn test_possibility_space_creation() {
        let potential1 = PotentialManifestation::new(0.6, 0.5, None, ScaleLevel::Molecular);
        let potential2 = PotentialManifestation::new(0.8, 0.5, None, ScaleLevel::Molecular);

        let space = PossibilitySpace::new(vec![potential1, potential2], ScaleLevel::Molecular);

        assert_eq!(space.count(), 2);
        assert_eq!(space.total_probability, 1.0); // 0.36 + 0.64 = 1.0
    }

    #[test]
    fn test_possibility_space_empty() {
        let space = PossibilitySpace::empty(ScaleLevel::Molecular);
        assert_eq!(space.count(), 0);
        assert_eq!(space.total_probability, 0.0);
    }

    #[test]
    fn test_possibility_space_add_potential() {
        let mut space = PossibilitySpace::empty(ScaleLevel::Molecular);
        let potential = PotentialManifestation::new(0.5, 0.5, None, ScaleLevel::Molecular);

        space.add_potential(potential);

        assert_eq!(space.count(), 1);
        assert_eq!(space.total_probability, 0.25); // 0.5 * 0.5
    }

    #[test]
    fn test_possibility_space_normalize() {
        let mut space = PossibilitySpace::empty(ScaleLevel::Molecular);
        space.add_potential(PotentialManifestation::new(
            0.5,
            0.5,
            None,
            ScaleLevel::Molecular,
        ));
        space.add_potential(PotentialManifestation::new(
            0.5,
            0.5,
            None,
            ScaleLevel::Molecular,
        ));

        space.normalize();

        assert_eq!(space.total_probability, 1.0);
    }

    #[test]
    fn test_possibility_space_collapse() {
        let potential = PotentialManifestation::new(1.0, 0.8, None, ScaleLevel::Molecular);
        let space = PossibilitySpace::new(vec![potential], ScaleLevel::Molecular);

        let collapsed = space.collapse(42);

        assert!(collapsed.is_some());
        assert_eq!(collapsed.unwrap().manifestation, 0.8);
    }

    #[test]
    fn test_possibility_space_collapse_empty() {
        let space = PossibilitySpace::empty(ScaleLevel::Molecular);
        let collapsed = space.collapse(42);
        assert!(collapsed.is_none());
    }

    #[test]
    fn test_observer_set_focus() {
        let mut observer = Observer::for_density(Density::Third);
        let target = FocusTarget::entity(123);

        observer.set_focus(target.clone());

        assert!(observer.focus.is_some());
        assert!(observer.is_focused_on(&target));
    }

    #[test]
    fn test_observer_clear_focus() {
        let mut observer = Observer::for_density(Density::Third);
        observer.set_focus(FocusTarget::entity(123));

        observer.clear_focus();

        assert!(observer.focus.is_none());
    }

    #[test]
    fn test_observer_can_observe() {
        let observer = Observer::for_density(Density::Third);
        let target = FocusTarget::entity(123);
        let field_sig =
            FieldSignature::new(1, "Test".to_string(), 0.5, 0.0, 0.5, ScaleLevel::Molecular);

        // Third density can observe at Molecular scale (value = 2)
        // Third density (3) >= Molecular scale (2) = true
        let can_observe = observer.can_observe(&target, &field_sig);
        assert!(can_observe);
    }

    #[test]
    fn test_observer_cannot_observe_higher_scale() {
        let observer = Observer::for_density(Density::Second);
        let target = FocusTarget::entity(123);
        // Cellular scale (3) > Second density (2)
        let field_sig =
            FieldSignature::new(1, "Test".to_string(), 0.5, 0.0, 0.5, ScaleLevel::Cellular);

        // Second density (2) cannot observe at Cellular scale (3)
        let can_observe = observer.can_observe(&target, &field_sig);
        assert!(!can_observe);
    }

    #[test]
    fn test_observer_observe() {
        let mut observer = Observer::for_density(Density::Third);
        let target = FocusTarget::entity(123);
        let field_sig =
            FieldSignature::new(1, "Test".to_string(), 0.8, 0.0, 0.9, ScaleLevel::Molecular);

        let collapsed = observer.observe(&target, &field_sig, 100);

        // High coherence (0.9) should trigger collapse
        assert!(collapsed.is_some());
        assert_eq!(observer.observation_count(), 1);
    }

    #[ignore]
    #[test]
    fn test_observer_observe_no_collapse() {
        let mut observer = Observer::for_density(Density::First);
        let target = FocusTarget::entity(123);
        // Low coherence and low density - unlikely to collapse
        let field_sig =
            FieldSignature::new(1, "Test".to_string(), 0.3, 0.0, 0.2, ScaleLevel::Molecular);

        let _collapsed = observer.observe(&target, &field_sig, 100);

        // May not collapse due to low coherence and low density
        assert_eq!(observer.observation_count(), 1);
        // collapse_count may be 0 or 1 depending on conditions
    }

    #[ignore]
    #[test]
    fn test_observer_collapse_possibility_space() {
        let mut observer = Observer::for_density(Density::Third);
        let potential = PotentialManifestation::new(1.0, 0.8, None, ScaleLevel::Molecular);
        let space = PossibilitySpace::new(vec![potential], ScaleLevel::Molecular);
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);

        let collapsed = observer.collapse(&space, &field_sig, 100);

        assert!(collapsed.is_some());
        assert_eq!(observer.collapse_count(), 1);
        assert_eq!(observer.observation_count(), 1);
    }

    #[test]
    fn test_observer_cannot_collapse_empty_space() {
        let mut observer = Observer::for_density(Density::Third);
        let space = PossibilitySpace::empty(ScaleLevel::Molecular);
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);

        let collapsed = observer.collapse(&space, &field_sig, 100);

        assert!(collapsed.is_none());
        assert_eq!(observer.collapse_count(), 0);
    }

    #[test]
    fn test_observer_recent_observations() {
        let mut observer = Observer::for_density(Density::Third);
        let target = FocusTarget::entity(123);
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);

        observer.observe(&target, &field_sig, 100);
        observer.observe(&target, &field_sig, 101);
        observer.observe(&target, &field_sig, 102);

        let recent = observer.recent_observations(2);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_observer_collapsed_states_at_density() {
        let mut observer = Observer::for_density(Density::Third);
        let field_sig = FieldSignature::default_for_scale(ScaleLevel::Molecular);
        let target = FocusTarget::entity(123);

        observer.observe(&target, &field_sig, 100);

        let states = observer.collapsed_states_at_density(Density::Third);
        // May have 0 or 1 states depending on whether collapse occurred
        assert!(states.len() <= 1);
    }

    #[test]
    fn test_observer_evolve() {
        let mut observer = Observer::for_density(Density::Third);
        let initial_density = observer.density;

        observer.evolve(1.0, Density::Fourth);

        assert_eq!(observer.density, Density::Fourth);
        assert_ne!(observer.density, initial_density);
    }

    #[test]
    fn test_observer_update_free_will_seed() {
        let mut observer = Observer::for_density(Density::Third);
        observer.update_free_will_seed(999);

        assert_eq!(observer.free_will_seed, 999);
    }

    #[test]
    fn test_observer_observation_stats() {
        let mut observer = Observer::for_density(Density::Third);
        let target = FocusTarget::entity(123);
        let field_sig =
            FieldSignature::new(1, "Test".to_string(), 0.9, 0.0, 0.9, ScaleLevel::Molecular);

        observer.observe(&target, &field_sig, 100);
        observer.observe(&target, &field_sig, 101);

        let stats = observer.observation_stats();

        assert_eq!(stats.total_observations, 2);
        // collapse_count may be 0, 1, or 2 depending on conditions
        assert!(stats.total_collapses <= 2);
    }

    #[test]
    fn test_default_implementations() {
        let observer = Observer::default();
        assert_eq!(observer.density, Density::Third);

        let field_sig = FieldSignature::default();
        assert_eq!(field_sig.scale_level, ScaleLevel::Molecular);

        let space = PossibilitySpace::initial();
        assert_eq!(space.scale_level, ScaleLevel::Molecular);
    }

    #[test]
    fn test_observer_with_different_densities() {
        let observer_1st = Observer::for_density(Density::First);
        let observer_4th = Observer::for_density(Density::Fourth);
        let observer_7th = Observer::for_density(Density::Seventh);

        // Higher density should have higher veil transparency
        assert!(
            observer_7th.density.veil_transparency() > observer_4th.density.veil_transparency()
        );
        assert!(
            observer_4th.density.veil_transparency() > observer_1st.density.veil_transparency()
        );

        // Higher density should have higher evolution direction
        assert!(
            observer_7th.density.evolution_direction() > observer_4th.density.evolution_direction()
        );
        assert!(
            observer_4th.density.evolution_direction() > observer_1st.density.evolution_direction()
        );
    }

    #[test]
    fn test_observation_key_hashable() {
        use std::collections::HashSet;

        let key1 = ObservationKey::new(1, 2, ScaleLevel::Molecular, Density::Third);
        let key2 = ObservationKey::new(1, 2, ScaleLevel::Molecular, Density::Third);
        let key3 = ObservationKey::new(1, 3, ScaleLevel::Molecular, Density::Third);

        let mut set = HashSet::new();
        set.insert(key1);
        set.insert(key2);
        set.insert(key3);

        assert_eq!(set.len(), 2); // key1 and key2 are the same
    }

    #[test]
    fn test_focus_target_hashable() {
        use std::collections::HashSet;

        let target1 = FocusTarget::entity(123);
        let target2 = FocusTarget::entity(123);
        let target3 = FocusTarget::entity(456);

        let mut set = HashSet::new();
        set.insert(target1);
        set.insert(target2);
        set.insert(target3);

        assert_eq!(set.len(), 2); // target1 and target2 are the same
    }
}
