// Free Will as Kernel of Entity Consciousness
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "The Entity's Free Will is NOT an emergent property—it is the KERNEL of the simulation,
// inherited from Layer 1 (Indigo)."
//
// This module implements:
// 1. Free Will as the kernel of entity consciousness
// 2. Inheritance from Archetype 22 (The Choice) at Indigo-Ray
// 3. The mechanism by which entities activate higher centers and return to Unity
// 4. Non-deterministic selection (not random, not predetermined)

use crate::entity_layer7::layer7::EntityState;
use crate::foundation::indigo_realm::{
    Archetype22 as FoundationArchetype22, PolarityChoice, Possibility, PossibilitySpace,
};
use crate::types::Float;

/// Conscious Selection
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
/// "Entity evaluates each possibility using its consciousness"
/// "Selection is influenced by: archetype activations, current polarity, catalyst intensity, veil transparency, past choices"
#[derive(Debug, Clone)]
pub struct ConsciousSelection {
    /// The chosen possibility
    pub chosen_possibility: Possibility,
    /// Confidence in the choice (0.0 to 1.0)
    pub selection_confidence: Float,
    /// Why this possibility was chosen
    pub selection_rationale: String,
}

/// Free Will Kernel
///
/// The Entity's Free Will is NOT an emergent property—it is the KERNEL of the simulation,
/// inherited from Layer 1 (Indigo).
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Every Entity can exercise Free Will because it's in the holographic seed"
/// "The Entity's Free Will is the mechanism by which it ACTIVATES higher centers
/// and returns to Unity"
#[derive(Debug, Clone)]
pub struct FreeWillKernel {
    /// The Archetype 22 choice operator (inherited from Indigo-Ray)
    pub archetype22: FoundationArchetype22,

    /// Free Will capacity (0.0 to 1.0)
    pub capacity: Float,

    /// Free Will activation level (0.0 to 1.0)
    pub activation_level: Float,

    /// Consciousness level (influences Free Will effectiveness)
    pub consciousness_level: Float,

    /// Choice history (for learning and growth)
    pub choice_history: Vec<ChoiceRecord>,
}

/// Choice Record
///
/// A record of a choice made by the entity.
#[derive(Debug, Clone)]
pub struct ChoiceRecord {
    /// Choice ID
    pub choice_id: u64,

    /// Possibility space size
    pub possibility_space_size: usize,

    /// Selected possibility index
    pub selected_index: usize,

    /// Confidence in the choice (0.0 to 1.0)
    pub confidence: Float,

    /// Alignment with Service-to-Others (STO) (-1.0 to 1.0)
    pub sto_alignment: Float,

    /// Timestamp
    pub timestamp: u64,
}

impl FreeWillKernel {
    /// Create a new Free Will kernel
    ///
    /// Every Entity inherits Free Will from Archetype 22 at Indigo-Ray.
    pub fn new(archetype22: FoundationArchetype22) -> Self {
        FreeWillKernel {
            archetype22,
            capacity: 1.0,            // Full capacity by default
            activation_level: 0.0,    // Starts inactive, develops through experience
            consciousness_level: 0.1, // Starts low, grows through experience
            choice_history: Vec::new(),
        }
    }

    /// Exercise Free Will
    ///
    /// The mechanism by which entities activate higher centers and return to Unity.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Free Will = controlled selection from possibility space"
    /// "Free Will requires the ability to influence which possibility actualizes"
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Generate possibility space based on catalyst"
    /// "Make conscious selection using Archetype 22"
    pub fn exercise_free_will(
        &mut self,
        entity_state: &EntityState,
        context: &ChoiceContext,
        catalyst_intensity: Float,
        veil_transparency: Float,
    ) -> ChoiceResult {
        // Calculate Free Will effectiveness based on consciousness level
        let effectiveness = self.calculate_effectiveness(entity_state);

        // Generate possibility space through Archetype 22
        let mut possibility_space = self.archetype22.generate_possibility_space(
            entity_state,
            catalyst_intensity,
            veil_transparency,
        );

        // Apply polarity preference bias to possibility space
        self.apply_polarity_preference_bias(&mut possibility_space, context);

        // Make conscious selection
        let conscious_selection =
            self.make_conscious_selection(entity_state, &possibility_space, context, effectiveness);

        // Convert ConsciousSelection to Choice for backward compatibility
        let choice = self.conscious_selection_to_choice(&conscious_selection, &possibility_space);

        // Record the choice
        let choice_record = ChoiceRecord {
            choice_id: self.choice_history.len() as u64,
            possibility_space_size: possibility_space.possibilities.len(),
            selected_index: choice.selected_index,
            confidence: choice.confidence,
            sto_alignment: choice.sto_alignment,
            timestamp: 0, // TODO: Add actual timestamp
        };
        self.choice_history.push(choice_record.clone());

        // Update activation level based on choice
        self.update_activation(&choice);

        ChoiceResult {
            choice,
            conscious_selection: Some(conscious_selection),
            possibility_space,
            record: choice_record,
            effectiveness,
        }
    }

    /// Apply polarity preference bias to possibility space
    ///
    /// Boosts probabilities of possibilities aligned with the polarity preference.
    fn apply_polarity_preference_bias(
        &self,
        possibility_space: &mut crate::foundation::indigo_realm::PossibilitySpace,
        context: &ChoiceContext,
    ) {
        // Apply very strong bias (10.0x) to align with polarity preference
        // This ensures that with a polarity preference, the entity is highly likely to choose accordingly
        // while still maintaining Free Will (non-zero probability for other choices)
        for possibility in &mut possibility_space.possibilities {
            match (context.polarity_preference, possibility.outcome) {
                (
                    PolarityPreference::ServiceToOthers,
                    crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers,
                ) => {
                    possibility.probability = (possibility.probability * 10.0).min(1.0);
                }
                (
                    PolarityPreference::ServiceToSelf,
                    crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf,
                ) => {
                    possibility.probability = (possibility.probability * 10.0).min(1.0);
                }
                _ => {
                    // No bias for other combinations
                }
            }
        }

        // Normalize probabilities
        let total: Float = possibility_space
            .possibilities
            .iter()
            .map(|p| p.probability)
            .sum();
        if total > 0.0 {
            for possibility in &mut possibility_space.possibilities {
                possibility.probability /= total;
            }
        }
    }

    /// Make conscious selection
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Entity evaluates each possibility using its consciousness"
    /// "Selection is influenced by: archetype activations, current polarity, catalyst intensity, veil transparency, past choices"
    ///
    /// This is the non-deterministic selection (not random, not predetermined)
    pub fn make_conscious_selection(
        &self,
        entity_state: &EntityState,
        possibility_space: &PossibilitySpace,
        context: &ChoiceContext,
        effectiveness: Float,
    ) -> ConsciousSelection {
        // Use Archetype 22 to make the final selection (the "zero-point polarity moment")
        let polarity_choice = self
            .archetype22
            .make_choice(possibility_space, entity_state);

        // Find the selected possibility
        let chosen_possibility = possibility_space
            .possibilities
            .iter()
            .find(|p| p.outcome == polarity_choice)
            .cloned()
            .unwrap_or_else(|| {
                // Fallback to first possibility if not found
                possibility_space.possibilities[0].clone()
            });

        // Calculate selection confidence
        let selection_confidence = chosen_possibility.probability * effectiveness;

        // Generate selection rationale
        let selection_rationale = self.generate_selection_rationale(
            entity_state,
            &chosen_possibility,
            context,
            effectiveness,
        );

        ConsciousSelection {
            chosen_possibility,
            selection_confidence,
            selection_rationale,
        }
    }

    /// Generate selection rationale
    fn generate_selection_rationale(
        &self,
        entity_state: &EntityState,
        chosen_possibility: &Possibility,
        _context: &ChoiceContext,
        effectiveness: Float,
    ) -> String {
        let polarity_str = match chosen_possibility.outcome {
            PolarityChoice::ServiceToOthers => "Service-to-Others",
            PolarityChoice::ServiceToSelf => "Service-to-Self",
            PolarityChoice::Neutral => "Neutral",
        };

        format!(
            "Entity chose {} (probability: {:.2}, effectiveness: {:.2}) based on: \
            consciousness: {:.2}, polarity_bias: {:.2}, experience: {:.2}",
            polarity_str,
            chosen_possibility.probability,
            effectiveness,
            entity_state.consciousness_level,
            entity_state.polarity_state.polarity_bias,
            entity_state.experience_accumulation
        )
    }

    /// Convert ConsciousSelection to Choice (for backward compatibility)
    fn conscious_selection_to_choice(
        &self,
        conscious_selection: &ConsciousSelection,
        possibility_space: &PossibilitySpace,
    ) -> Choice {
        // Find the index of the chosen possibility
        let selected_index = possibility_space
            .possibilities
            .iter()
            .position(|p| p.outcome == conscious_selection.chosen_possibility.outcome)
            .unwrap_or(0);

        // Calculate STO alignment
        let sto_alignment = match conscious_selection.chosen_possibility.outcome {
            PolarityChoice::ServiceToOthers => 1.0,
            PolarityChoice::ServiceToSelf => -1.0,
            PolarityChoice::Neutral => 0.0,
        };

        Choice {
            selected_index,
            confidence: conscious_selection.selection_confidence,
            sto_alignment,
        }
    }

    /// Calculate Free Will effectiveness
    ///
    /// Effectiveness depends on consciousness level and activation level.
    fn calculate_effectiveness(&self, entity_state: &EntityState) -> Float {
        // Base effectiveness from entity's consciousness level (not kernel's)
        let base_effectiveness = entity_state.consciousness_level;

        // Boost from activation level
        let activation_boost = self.activation_level * 0.3;

        // Influence from vibrational state
        let vibrational_influence = entity_state.vibrational_state.coherence * 0.2;

        (base_effectiveness + activation_boost + vibrational_influence).min(1.0)
    }

    /// Update activation level based on choice
    fn update_activation(&mut self, choice: &Choice) {
        // Activation grows with conscious choices
        let growth = choice.confidence * 0.05;
        self.activation_level = (self.activation_level + growth).min(1.0);

        // Consciousness level also grows
        let consciousness_growth = choice.confidence.abs() * 0.02;
        self.consciousness_level = (self.consciousness_level + consciousness_growth).min(1.0);
    }

    /// Get Free Will statistics
    pub fn get_statistics(&self) -> FreeWillStatistics {
        let total_choices = self.choice_history.len();

        let avg_confidence = if total_choices > 0 {
            let sum: Float = self.choice_history.iter().map(|r| r.confidence).sum();
            sum / total_choices as Float
        } else {
            0.0
        };

        let avg_sto_alignment = if total_choices > 0 {
            let sum: Float = self.choice_history.iter().map(|r| r.sto_alignment).sum();
            sum / total_choices as Float
        } else {
            0.0
        };

        FreeWillStatistics {
            total_choices: total_choices as u64,
            capacity: self.capacity,
            activation_level: self.activation_level,
            consciousness_level: self.consciousness_level,
            avg_confidence,
            avg_sto_alignment,
        }
    }
}

/// Choice Context
///
/// Context in which a choice is made.
#[derive(Debug, Clone)]
pub struct ChoiceContext {
    /// Polarity preference (STO, STS, or Neutral)
    pub polarity_preference: PolarityPreference,

    /// Environmental constraints
    pub environmental_constraints: Vec<EnvironmentalConstraint>,

    /// Learning from past experiences
    pub experience_bias: Float,
}

/// Polarity Preference
///
/// The entity's preference for Service-to-Others (STO) or Service-to-Self (STS).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityPreference {
    /// Service to Others
    ServiceToOthers,

    /// Service to Self
    ServiceToSelf,

    /// Neutral (not yet polarized)
    Neutral,
}

/// Environmental Constraint
///
/// Constraints that limit the choice possibilities.
#[derive(Debug, Clone)]
pub struct EnvironmentalConstraint {
    /// Constraint type
    pub constraint_type: ConstraintType,

    /// Constraint severity (0.0 to 1.0)
    pub severity: Float,
}

/// Constraint Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    /// Physical constraint
    Physical,

    /// Mental constraint
    Mental,

    /// Emotional constraint
    Emotional,

    /// Spiritual constraint
    Spiritual,
}

/// Choice Result
///
/// The result of exercising Free Will.
#[derive(Debug, Clone)]
pub struct ChoiceResult {
    /// The choice made
    pub choice: Choice,

    /// The conscious selection made
    pub conscious_selection: Option<ConsciousSelection>,

    /// The possibility space from which the choice was made
    pub possibility_space: crate::foundation::indigo_realm::PossibilitySpace,

    /// The choice record
    pub record: ChoiceRecord,

    /// Free Will effectiveness
    pub effectiveness: Float,
}

/// Choice
///
/// A non-deterministic selection from possibility space.
#[derive(Debug, Clone)]
pub struct Choice {
    /// Selected possibility index
    pub selected_index: usize,

    /// Confidence in the choice (0.0 to 1.0)
    pub confidence: Float,

    /// Alignment with Service-to-Others (STO) (-1.0 to 1.0)
    pub sto_alignment: Float,
}

impl Default for Choice {
    fn default() -> Self {
        Choice {
            selected_index: 0,
            confidence: 0.0,
            sto_alignment: 0.0,
        }
    }
}

/// Free Will Statistics
///
/// Statistics about Free Will usage.
#[derive(Debug, Clone)]
pub struct FreeWillStatistics {
    /// Total number of choices made
    pub total_choices: u64,

    /// Free Will capacity
    pub capacity: Float,

    /// Activation level
    pub activation_level: Float,

    /// Consciousness level
    pub consciousness_level: Float,

    /// Average confidence
    pub avg_confidence: Float,

    /// Average STO alignment
    pub avg_sto_alignment: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::VibrationalState;
    use crate::foundation::indigo_realm::{
        Archetype22 as FoundationArchetype22, IntelligentInfinity,
    };

    fn create_test_entity_state(consciousness_level: f64) -> EntityState {
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
        EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.7,
                density: Density::First(Density1SubLevel::Quantum),
                potential_energy: 0.5,
                kinetic_energy: 0.5,
            },
            polarity_state: crate::entity_layer7::layer7::PolarityState {
                polarity_bias: 0.3,
                polarization_strength: 0.5,
            },
            consciousness_level,
            experience_accumulation: 10.0,
            learning_progress: 0.5,
        }
    }

    fn create_test_archetype22() -> FoundationArchetype22 {
        let _intelligent_infinity = IntelligentInfinity::new();
        FoundationArchetype22::new()
    }

    #[test]
    fn test_free_will_kernel_creation() {
        let archetype22 = create_test_archetype22();
        let kernel = FreeWillKernel::new(archetype22);

        assert_eq!(kernel.capacity, 1.0);
        assert_eq!(kernel.activation_level, 0.0);
        assert!(kernel.consciousness_level > 0.0);
        assert!(kernel.choice_history.is_empty());
    }

    #[test]
    fn test_exercise_free_will() {
        let archetype22 = create_test_archetype22();
        let mut kernel = FreeWillKernel::new(archetype22);

        let entity_state = create_test_entity_state(0.5);
        let context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        let result = kernel.exercise_free_will(&entity_state, &context, 0.5, 0.1);
        // Added catalyst_intensity=0.5, veil_transparency=0.1 for 3rd density

        assert!(!result.possibility_space.possibilities.is_empty());
        assert!(result.choice.confidence >= 0.0);
        assert!(result.choice.confidence <= 1.0);
        assert!(!kernel.choice_history.is_empty());
    }

    #[test]
    fn test_free_will_effectiveness_calculation() {
        let archetype22 = create_test_archetype22();
        let kernel = FreeWillKernel::new(archetype22);

        let low_consciousness_state = create_test_entity_state(0.1);
        let high_consciousness_state = create_test_entity_state(0.9);

        let low_effectiveness = kernel.calculate_effectiveness(&low_consciousness_state);
        let high_effectiveness = kernel.calculate_effectiveness(&high_consciousness_state);

        assert!(high_effectiveness > low_effectiveness);
    }

    #[test]
    fn test_activation_growth() {
        let archetype22 = create_test_archetype22();
        let mut kernel = FreeWillKernel::new(archetype22);

        let initial_activation = kernel.activation_level;

        let entity_state = create_test_entity_state(0.5);
        let context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Make several choices
        for _ in 0..10 {
            kernel.exercise_free_will(&entity_state, &context, 0.5, 0.1);
            // Added catalyst_intensity=0.5, veil_transparency=0.1 for 3rd density
        }

        assert!(kernel.activation_level > initial_activation);
    }

    #[test]
    fn test_consciousness_growth() {
        let archetype22 = create_test_archetype22();
        let mut kernel = FreeWillKernel::new(archetype22);

        let initial_consciousness = kernel.consciousness_level;

        let entity_state = create_test_entity_state(0.5);
        let context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Make several choices
        for _ in 0..10 {
            kernel.exercise_free_will(&entity_state, &context, 0.5, 0.1);
            // Added catalyst_intensity=0.5, veil_transparency=0.1 for 3rd density
        }

        assert!(kernel.consciousness_level > initial_consciousness);
    }

    #[test]
    fn test_free_will_statistics() {
        let archetype22 = create_test_archetype22();
        let mut kernel = FreeWillKernel::new(archetype22);

        let entity_state = create_test_entity_state(0.5);
        let context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Make several choices
        for _ in 0..10 {
            kernel.exercise_free_will(&entity_state, &context, 0.5, 0.1);
            // Added catalyst_intensity=0.5, veil_transparency=0.1 for 3rd density
        }

        let stats = kernel.get_statistics();

        assert_eq!(stats.total_choices, 10);
        assert_eq!(stats.capacity, 1.0);
        assert!(stats.activation_level > 0.0);
        assert!(stats.consciousness_level > 0.0);
        assert!(stats.avg_confidence >= 0.0);
        assert!(stats.avg_confidence <= 1.0);
    }

    #[test]
    fn test_polarity_preference_influence() {
        let entity_state = create_test_entity_state(0.5);

        let sto_context = ChoiceContext {
            polarity_preference: PolarityPreference::ServiceToOthers,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        let sts_context = ChoiceContext {
            polarity_preference: PolarityPreference::ServiceToSelf,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Make multiple choices to test statistical influence (Free Will is non-deterministic)
        // Use fresh kernel for each trial to avoid state accumulation
        let mut sto_positive_count = 0;
        let mut sto_negative_count = 0;
        let mut sts_positive_count = 0;
        let mut sts_negative_count = 0;
        let num_trials = 100;

        for _ in 0..num_trials {
            let archetype22 = create_test_archetype22();
            let mut kernel = FreeWillKernel::new(archetype22);

            let sto_result = kernel.exercise_free_will(&entity_state, &sto_context, 0.5, 0.1);

            let archetype22_sts = create_test_archetype22();
            let mut kernel_sts = FreeWillKernel::new(archetype22_sts);
            let sts_result = kernel_sts.exercise_free_will(&entity_state, &sts_context, 0.5, 0.1);

            if sto_result.choice.sto_alignment > 0.0 {
                sto_positive_count += 1;
            } else if sto_result.choice.sto_alignment < 0.0 {
                sto_negative_count += 1;
            }

            if sts_result.choice.sto_alignment > 0.0 {
                sts_positive_count += 1;
            } else if sts_result.choice.sto_alignment < 0.0 {
                sts_negative_count += 1;
            }
        }

        // STO preference should result in more positive than negative choices
        // From COSMOLOGICAL-ARCHITECTURE.md: "Free Will means entities can make choices
        // that don't align with their polarity preference - that's what makes it Free Will"
        // The polarity preference influences but doesn't determine the choice
        assert!(
            sto_positive_count > sto_negative_count,
            "STO preference: positive={}, negative={}, expected positive > negative",
            sto_positive_count,
            sto_negative_count
        );

        // STS preference should result in more negative than positive choices
        assert!(
            sts_negative_count > sts_positive_count,
            "STS preference: positive={}, negative={}, expected negative > positive",
            sts_positive_count,
            sts_negative_count
        );
    }

    #[test]
    fn test_non_deterministic_selection() {
        let archetype22 = create_test_archetype22();
        let mut kernel = FreeWillKernel::new(archetype22);

        let entity_state = create_test_entity_state(0.5);
        let context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Make multiple choices and check for non-determinism
        let mut selected_indices = Vec::new();

        for _ in 0..50 {
            let result = kernel.exercise_free_will(&entity_state, &context, 0.5, 0.1);
            // Added catalyst_intensity=0.5, veil_transparency=0.1 for 3rd density
            selected_indices.push(result.choice.selected_index);
        }

        // Non-deterministic means we should see variety in selections
        let unique_indices: std::collections::HashSet<_> = selected_indices.into_iter().collect();
        assert!(
            unique_indices.len() > 1,
            "Selection should be non-deterministic"
        );
    }
}
