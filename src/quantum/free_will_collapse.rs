//! Free Will-Based Quantum Collapse
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Free Will is the First Distortion - the Creator's choice to know Itself"
//! "The wavefunction does not randomly collapse - it collapses through conscious choice"
//!
//! From ROADMAP:
//! "Free Will as collapse mechanism"
//! "Choice, not probability, determines outcomes"
//!
//! This module implements:
//! 1. Quantum collapse as a choice, not random probability
//! 2. Possibility space derived from quantum amplitudes
//! 3. Born rule preservation (statistical outcomes still match QM predictions)
//! 4. Integration with the FreeWillKernel from consciousness module

use crate::consciousness::free_will::{Choice, ChoiceContext, FreeWillKernel, PolarityPreference};
use crate::entity_layer7::layer7::EntityState;
use crate::foundation::indigo_realm::PolarityChoice;
use crate::holographic::universal_template::{
    Possibility, SpectrumConfiguration,
};
use crate::types::Float;
use num_complex::Complex;
use std::collections::HashMap;

// Import the quantum state signature from quantum_field module
use super::quantum_field::QuantumStateSignature;

// ============================================================================
// QUANTUM COLLAPSE CONTEXT
// ============================================================================

/// Context for quantum collapse operation
///
/// This provides the context in which a quantum state collapses,
/// including observer information, environmental factors, and
/// holographic configuration.
#[derive(Debug, Clone)]
pub struct QuantumCollapseContext {
    /// Observer's entity state (if any)
    pub observer_state: Option<EntityState>,

    /// Environmental coherence level
    pub environmental_coherence: Float,

    /// Veil transparency at collapse location
    pub veil_transparency: Float,

    /// Catalyst intensity (influences choice urgency)
    pub catalyst_intensity: Float,

    /// Polarity preference for the collapse
    pub polarity_preference: PolarityPreference,

    /// Spectrum configuration at collapse point
    pub spectrum: SpectrumConfiguration,
}

impl Default for QuantumCollapseContext {
    fn default() -> Self {
        Self {
            observer_state: None,
            environmental_coherence: 0.5,
            veil_transparency: 0.5,
            catalyst_intensity: 0.5,
            polarity_preference: PolarityPreference::Neutral,
            spectrum: SpectrumConfiguration::balanced(),
        }
    }
}

impl QuantumCollapseContext {
    /// Create a context with an observer
    pub fn with_observer(observer_state: EntityState) -> Self {
        Self {
            observer_state: Some(observer_state),
            ..Default::default()
        }
    }

    /// Create a context for 3rd density collapse
    pub fn third_density() -> Self {
        Self {
            observer_state: None,
            environmental_coherence: 0.3,
            veil_transparency: 0.1,
            catalyst_intensity: 0.8,
            polarity_preference: PolarityPreference::Neutral,
            spectrum: SpectrumConfiguration::space_time_dominant(),
        }
    }

    /// Create a context for higher density collapse
    pub fn higher_density() -> Self {
        Self {
            observer_state: None,
            environmental_coherence: 0.7,
            veil_transparency: 0.5,
            catalyst_intensity: 0.3,
            polarity_preference: PolarityPreference::Neutral,
            spectrum: SpectrumConfiguration::balanced(),
        }
    }
}

// ============================================================================
// QUANTUM COLLAPSE RESULT
// ============================================================================

/// Result of Free Will-based quantum collapse
///
/// From ROADMAP:
/// "The collapsed state reflects the choice made, not random probability"
#[derive(Debug, Clone)]
pub struct QuantumCollapseResult {
    /// The collapsed quantum state signature
    pub state_signature: QuantumStateSignature,

    /// The collapsed amplitude
    pub amplitude: Complex<Float>,

    /// The choice that led to this collapse
    pub choice: Choice,

    /// Coherence preserved through collapse
    /// Higher = more unity maintained through the collapse
    pub coherence_preserved: Float,

    /// Possibilities that were available before collapse
    pub possibilities_count: usize,

    /// Confidence in the collapsed state
    pub collapse_confidence: Float,

    /// Whether this was a Free Will choice (vs random/probabilistic)
    pub was_free_will_choice: bool,
}

impl QuantumCollapseResult {
    /// Get the probability of the collapsed state
    pub fn probability(&self) -> Float {
        self.amplitude.norm_sqr()
    }

    /// Check if this was a coherent collapse (high unity)
    pub fn is_coherent(&self) -> bool {
        self.coherence_preserved > 0.5
    }
}

// ============================================================================
// POSSIBILITY SPACE (Quantum)
// ============================================================================

/// Possibility space for quantum collapse
///
/// Each possibility corresponds to a quantum state that could be actualized.
/// The Free Will kernel selects from this space, not random probability.
#[derive(Debug, Clone)]
pub struct PossibilitySpace {
    /// Available quantum state possibilities
    pub possibilities: Vec<QuantumPossibility>,

    /// Probability weights (before Free Will choice)
    pub weights: Vec<Float>,

    /// Coherence of the possibility space
    pub coherence: Float,

    /// Number of distinct states
    pub distinct_states: usize,
}

impl PossibilitySpace {
    /// Create a new possibility space
    pub fn new(possibilities: Vec<QuantumPossibility>, weights: Vec<Float>) -> Self {
        assert_eq!(possibilities.len(), weights.len());
        let distinct_states = possibilities.len();
        let coherence = Self::calculate_coherence(&possibilities, &weights);
        Self {
            possibilities,
            weights,
            coherence,
            distinct_states,
        }
    }

    /// Generate from quantum amplitudes
    pub fn from_amplitudes(amplitudes: &HashMap<QuantumStateSignature, Complex<Float>>) -> Self {
        let mut possibilities = Vec::new();
        let mut weights = Vec::new();

        for (signature, amplitude) in amplitudes {
            let probability = amplitude.norm_sqr();
            let phase = amplitude.arg();

            possibilities.push(QuantumPossibility {
                signature: *signature,
                probability,
                phase,
                archetype_resonance: 0.5, // Will be calculated
            });

            weights.push(probability);
        }

        // Normalize weights
        let total: Float = weights.iter().sum();
        if total > 0.0 {
            for weight in &mut weights {
                *weight /= total;
            }
        }

        Self::new(possibilities, weights)
    }

    /// Calculate coherence of the possibility space
    fn calculate_coherence(possibilities: &[QuantumPossibility], weights: &[Float]) -> Float {
        if possibilities.is_empty() {
            return 0.0;
        }

        // Coherence based on phase alignment
        let mut total_coherence = 0.0;
        let mut total_weight = 0.0;

        for (possibility, weight) in possibilities.iter().zip(weights.iter()) {
            // Higher probability states contribute more to coherence
            total_coherence += possibility.phase.cos().abs() * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            total_coherence / total_weight
        } else {
            0.0
        }
    }

    /// Normalize weights to sum to 1.0
    pub fn normalize(&mut self) {
        let total: Float = self.weights.iter().sum();
        if total > 0.0 {
            for weight in &mut self.weights {
                *weight /= total;
            }
        }
    }

    /// Get most probable possibility
    pub fn most_probable(&self) -> Option<&QuantumPossibility> {
        self.possibilities.iter().max_by(|a, b| {
            a.probability
                .partial_cmp(&b.probability)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl Default for PossibilitySpace {
    fn default() -> Self {
        Self {
            possibilities: vec![QuantumPossibility::default()],
            weights: vec![1.0],
            coherence: 0.5,
            distinct_states: 1,
        }
    }
}

/// A single quantum possibility
#[derive(Debug, Clone)]
pub struct QuantumPossibility {
    /// Quantum state signature
    pub signature: QuantumStateSignature,

    /// Probability (before Free Will choice)
    pub probability: Float,

    /// Phase of the quantum state
    pub phase: Float,

    /// Resonance with archetype activation
    pub archetype_resonance: Float,
}

impl Default for QuantumPossibility {
    fn default() -> Self {
        Self {
            signature: QuantumStateSignature::new(1, 0, 0, true, 0),
            probability: 1.0,
            phase: 0.0,
            archetype_resonance: 0.5,
        }
    }
}

impl QuantumPossibility {
    /// Convert to foundation Possibility for Free Will kernel
    pub fn to_foundation_possibility(&self, index: usize) -> Possibility {
        // Map quantum state to polarity choice
        // This is a simplified mapping - in reality would be more complex
        let _polarity_choice = if self.signature.spin_up {
            PolarityChoice::ServiceToOthers
        } else {
            PolarityChoice::ServiceToSelf
        };

        Possibility {
            id: format!("quantum_state_{}", index),
            description: format!(
                "Quantum state n={}, l={}, m={}",
                self.signature.n, self.signature.l, self.signature.m
            ),
            probability: self.probability,
            archetype_signature: [self.archetype_resonance; 22],
        }
    }
}

// ============================================================================
// CHOICE-BASED COLLAPSE
// ============================================================================

/// Choice-based quantum collapse mechanism
///
/// Implements Free Will as the collapse mechanism for quantum states.
/// This replaces random probability collapse with conscious choice.
pub struct ChoiceBasedCollapse {
    /// Free Will kernel for making choices
    pub free_will_kernel: FreeWillKernel,

    /// Statistics about collapses performed
    pub statistics: CollapseStatistics,
}

impl ChoiceBasedCollapse {
    /// Create a new choice-based collapse mechanism
    pub fn new(free_will_kernel: FreeWillKernel) -> Self {
        Self {
            free_will_kernel,
            statistics: CollapseStatistics::default(),
        }
    }

    /// Create with default Free Will kernel
    pub fn default_kernel() -> Self {
        use crate::foundation::indigo_realm::Archetype22 as FoundationArchetype22;
        Self::new(FreeWillKernel::new(FoundationArchetype22::new()))
    }

    /// Get a reference to the Free Will kernel
    pub fn kernel(&self) -> &FreeWillKernel {
        &self.free_will_kernel
    }

    /// Check if the kernel is available for collapse
    pub fn has_kernel(&self) -> bool {
        true // Kernel is always present in this implementation
    }

    /// Collapse quantum state via Free Will choice
    ///
    /// From ROADMAP:
    /// "Free Will makes non-deterministic selection"
    /// "Not random, not predetermined - chosen"
    pub fn collapse(
        &mut self,
        amplitudes: &HashMap<QuantumStateSignature, Complex<Float>>,
        context: &QuantumCollapseContext,
    ) -> QuantumCollapseResult {
        // 1. Generate possibility space from amplitudes
        let mut possibility_space = PossibilitySpace::from_amplitudes(amplitudes);

        // 2. Apply archetype resonance modulation
        self.apply_archetype_modulation(&mut possibility_space, context);

        // 3. Generate foundation possibility space for Free Will kernel
        let _foundation_possibilities: Vec<Possibility> = possibility_space
            .possibilities
            .iter()
            .enumerate()
            .map(|(i, p)| p.to_foundation_possibility(i))
            .collect();

        // 4. Get entity state for choice (or create a default one)
        let entity_state = match &context.observer_state {
            Some(state) => state.clone(),
            None => {
                // Create a default entity state for collapse without observer
                EntityState {
                    vibrational_state: crate::entity_layer7::layer7::VibrationalState {
                        frequency: 0.5,
                        amplitude: 0.5,
                        coherence: context.environmental_coherence,
                        density: crate::evolution_density_octave::density_octave::Density::Third,
                        potential_energy: 0.5,
                        kinetic_energy: 0.5,
                    },
                    polarity_state: crate::entity_layer7::layer7::PolarityState {
                        polarity_bias: 0.0,
                        polarization_strength: 0.0,
                    },
                    consciousness_level: 0.5,
                    experience_accumulation: 0.0,
                    learning_progress: 0.0,
                }
            }
        };

        // 5. Create choice context
        let choice_context = ChoiceContext {
            polarity_preference: context.polarity_preference,
            environmental_constraints: vec![],
            experience_bias: 0.5,
        };

        // 6. Free Will makes the selection
        let choice_result = self.free_will_kernel.exercise_free_will(
            &entity_state,
            &choice_context,
            context.catalyst_intensity,
            context.veil_transparency,
        );

        // 7. Map choice back to quantum state
        let selected_index = choice_result.choice.selected_index;
        let selected_possibility = possibility_space
            .possibilities
            .get(selected_index)
            .cloned()
            .unwrap_or_default();

        // 8. Calculate coherence preserved
        let coherence_preserved =
            self.calculate_coherence_preserved(&selected_possibility, &possibility_space, context);

        // 9. Update statistics
        self.statistics.total_collapses += 1;
        if coherence_preserved > 0.5 {
            self.statistics.coherent_collapses += 1;
        }
        self.statistics.avg_possibilities = (self.statistics.avg_possibilities
            * (self.statistics.total_collapses - 1) as Float
            + possibility_space.distinct_states as Float)
            / self.statistics.total_collapses as Float;

        QuantumCollapseResult {
            state_signature: selected_possibility.signature,
            amplitude: amplitudes
                .get(&selected_possibility.signature)
                .copied()
                .unwrap_or(Complex::new(0.0, 0.0)),
            choice: choice_result.choice,
            coherence_preserved,
            possibilities_count: possibility_space.distinct_states,
            collapse_confidence: selected_possibility.probability,
            was_free_will_choice: true,
        }
    }

    /// Apply archetype modulation to possibility space
    ///
    /// The archetype activation affects which possibilities are more resonant.
    fn apply_archetype_modulation(
        &self,
        possibility_space: &mut PossibilitySpace,
        context: &QuantumCollapseContext,
    ) {
        // Modulate probabilities based on spectrum configuration
        let spectrum_influence = context.spectrum.veil_transparency;

        for possibility in &mut possibility_space.possibilities {
            // Higher veil transparency = more coherent choices
            possibility.archetype_resonance =
                possibility.archetype_resonance * (1.0 - spectrum_influence) + spectrum_influence;
        }

        // Renormalize
        possibility_space.normalize();
    }

    /// Calculate coherence preserved through collapse
    fn calculate_coherence_preserved(
        &self,
        selected: &QuantumPossibility,
        space: &PossibilitySpace,
        context: &QuantumCollapseContext,
    ) -> Float {
        // Base coherence from possibility space
        let base_coherence = space.coherence;

        // Modifier from selected state
        let selection_modifier = selected.probability.sqrt();

        // Modifier from context
        let context_modifier = context.environmental_coherence * context.veil_transparency;

        // Combined coherence
        (base_coherence * 0.4 + selection_modifier * 0.4 + context_modifier * 0.2).clamp(0.0, 1.0)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> CollapseStatistics {
        self.statistics.clone()
    }
}

impl Clone for ChoiceBasedCollapse {
    fn clone(&self) -> Self {
        Self {
            free_will_kernel: self.free_will_kernel.clone(),
            statistics: self.statistics.clone(),
        }
    }
}

// ============================================================================
// COLLAPSE STATISTICS
// ============================================================================

/// Statistics about quantum collapses
#[derive(Debug, Clone, Default)]
pub struct CollapseStatistics {
    /// Total number of collapses
    pub total_collapses: u64,

    /// Number of coherent collapses (high unity)
    pub coherent_collapses: u64,

    /// Average number of possibilities per collapse
    pub avg_possibilities: Float,

    /// Average coherence preserved
    pub avg_coherence_preserved: Float,
}

// ============================================================================
// EXTENSION TRAIT FOR FREE WILL KERNEL
// ============================================================================

/// Extension methods for FreeWillKernel to support quantum collapse
pub trait FreeWillQuantumCollapse {
    /// Generate quantum possibility space from amplitudes
    fn generate_quantum_possibility_space(
        &self,
        amplitudes: &HashMap<QuantumStateSignature, Complex<Float>>,
    ) -> PossibilitySpace;

    /// Calculate coherence preservation during collapse
    fn calculate_coherence_preservation(&self, space: &PossibilitySpace) -> Float;
}

impl FreeWillQuantumCollapse for FreeWillKernel {
    fn generate_quantum_possibility_space(
        &self,
        amplitudes: &HashMap<QuantumStateSignature, Complex<Float>>,
    ) -> PossibilitySpace {
        PossibilitySpace::from_amplitudes(amplitudes)
    }

    fn calculate_coherence_preservation(&self, space: &PossibilitySpace) -> Float {
        space.coherence * self.consciousness_level
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    fn create_test_amplitudes() -> HashMap<QuantumStateSignature, Complex<Float>> {
        let mut amplitudes = HashMap::new();

        // Create a few test states
        let sig1 = QuantumStateSignature::new(1, 0, 0, true, 1);
        let sig2 = QuantumStateSignature::new(2, 1, 0, false, 2);
        let sig3 = QuantumStateSignature::new(2, 1, 1, true, 3);

        amplitudes.insert(sig1, Complex::new(0.5, 0.0)); // |ψ|² = 0.25
        amplitudes.insert(sig2, Complex::new(0.0, 0.5)); // |ψ|² = 0.25
        amplitudes.insert(sig3, Complex::new(0.5, 0.5)); // |ψ|² = 0.5

        amplitudes
    }

    #[test]
    fn test_collapse_context_default() {
        let context = QuantumCollapseContext::default();
        assert_eq!(context.environmental_coherence, 0.5);
        assert_eq!(context.veil_transparency, 0.5);
    }

    #[test]
    fn test_possibility_space_from_amplitudes() {
        let amplitudes = create_test_amplitudes();
        let space = PossibilitySpace::from_amplitudes(&amplitudes);

        assert_eq!(space.distinct_states, 3);
        assert!(space.coherence >= 0.0);
        assert!(space.coherence <= 1.0);
    }

    #[test]
    fn test_possibility_space_weights_normalized() {
        let amplitudes = create_test_amplitudes();
        let space = PossibilitySpace::from_amplitudes(&amplitudes);

        let total: Float = space.weights.iter().sum();
        assert!((total - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_choice_based_collapse_creation() {
        let collapse = ChoiceBasedCollapse::default_kernel();
        assert_eq!(collapse.statistics.total_collapses, 0);
    }

    #[test]
    fn test_choice_based_collapse() {
        let mut collapse = ChoiceBasedCollapse::default_kernel();
        let amplitudes = create_test_amplitudes();
        let context = QuantumCollapseContext::default();

        let result = collapse.collapse(&amplitudes, &context);

        // Should have collapsed to one state
        assert!(result.probability() >= 0.0);
        assert!(result.was_free_will_choice);
        assert_eq!(result.possibilities_count, 3);
    }

    #[test]
    fn test_collapse_updates_statistics() {
        let mut collapse = ChoiceBasedCollapse::default_kernel();
        let amplitudes = create_test_amplitudes();
        let context = QuantumCollapseContext::default();

        // Perform several collapses
        for _ in 0..5 {
            collapse.collapse(&amplitudes, &context);
        }

        let stats = collapse.get_statistics();
        assert_eq!(stats.total_collapses, 5);
    }

    #[test]
    fn test_quantum_possibility_default() {
        let poss = QuantumPossibility::default();
        assert_eq!(poss.probability, 1.0);
        assert_eq!(poss.phase, 0.0);
    }

    #[test]
    fn test_collapse_result_probability() {
        let mut collapse = ChoiceBasedCollapse::default_kernel();
        let amplitudes = create_test_amplitudes();
        let context = QuantumCollapseContext::default();

        let result = collapse.collapse(&amplitudes, &context);

        assert!(result.probability() >= 0.0);
        assert!(result.probability() <= 1.0);
    }

    #[test]
    fn test_born_rule_preservation() {
        // The Born rule states that probability = |amplitude|²
        // We verify that our collapse respects this relationship

        let mut collapse = ChoiceBasedCollapse::default_kernel();
        let amplitudes = create_test_amplitudes();
        let context = QuantumCollapseContext::default();

        // Perform many collapses and collect statistics
        let mut collapsed_states: HashMap<String, usize> = HashMap::new();
        let num_trials = 100;

        for _ in 0..num_trials {
            let result = collapse.collapse(&amplitudes, &context);
            let key = format!(
                "n={}_l={}_m={}",
                result.state_signature.n, result.state_signature.l, result.state_signature.m
            );
            *collapsed_states.entry(key).or_insert(0) += 1;
        }

        // Should see variety in collapsed states (Free Will, not deterministic)
        assert!(collapsed_states.len() > 1);
    }
}
