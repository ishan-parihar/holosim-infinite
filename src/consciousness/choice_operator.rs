// Choice Operator Implementation
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Choice = non-deterministic selection from possibility space"
// "Free Will = controlled selection from possibility space"
// "Free Will requires the ability to influence which possibility actualizes"
//
// This module implements:
// 1. The choice operator that makes non-deterministic selections
// 2. The mechanism of conscious selection (the profound mystery)
// 3. Free Will as controlled selection from possibility space

use crate::consciousness::possibility_space::{Possibility, PossibilitySpace};
use crate::entity_layer7::layer7::EntityState;
use crate::types::Float;

/// Choice Operator
///
/// The mechanism by which entities make non-deterministic selections from possibility space.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Choice = non-deterministic selection from possibility space"
/// "The mechanism of conscious selection remains a profound mystery"
#[derive(Debug, Clone)]
pub struct ChoiceOperator {
    /// Choice capacity (0.0 to 1.0)
    pub capacity: Float,

    /// Choice effectiveness (0.0 to 1.0)
    pub effectiveness: Float,

    /// Choice history
    pub choice_history: Vec<ChoiceRecord>,
}

/// Choice Record
///
/// A record of a choice made by the choice operator.
#[derive(Debug, Clone)]
pub struct ChoiceRecord {
    /// Choice ID
    pub choice_id: u64,

    /// Selected possibility index
    pub selected_index: usize,

    /// Confidence in the choice (0.0 to 1.0)
    pub confidence: Float,

    /// Alignment with Service-to-Others (STO) (-1.0 to 1.0)
    pub sto_alignment: Float,

    /// Choice quality (0.0 to 1.0)
    pub quality: Float,

    /// Timestamp
    pub timestamp: u64,
}

/// Choice Result
///
/// The result of making a choice.
#[derive(Debug, Clone)]
pub struct ChoiceResult {
    /// The selected possibility
    pub selected_possibility: Possibility,

    /// Choice confidence
    pub confidence: Float,

    /// STO alignment
    pub sto_alignment: Float,

    /// Choice quality
    pub quality: Float,

    /// The choice record
    pub record: ChoiceRecord,
}

/// Choice Context
///
/// Context in which a choice is made.
#[derive(Debug, Clone)]
pub struct ChoiceContext {
    /// Polarity preference
    pub polarity_preference: PolarityPreference,

    /// Environmental constraints
    pub environmental_constraints: Vec<EnvironmentalConstraint>,

    /// Learning from past experiences
    pub experience_bias: Float,

    /// Consciousness level
    pub consciousness_level: Float,
}

/// Polarity Preference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityPreference {
    /// Service to Others
    ServiceToOthers,

    /// Service to Self
    ServiceToSelf,

    /// Neutral
    Neutral,
}

/// Environmental Constraint
#[derive(Debug, Clone)]
pub struct EnvironmentalConstraint {
    /// Constraint type
    pub constraint_type: ConstraintType,

    /// Constraint severity (0.0 to 1.0)
    pub severity: Float,

    /// Constraint direction (-1.0 to 1.0)
    pub direction: Float,
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

    /// Karmic constraint
    Karmic,
}

impl Default for ChoiceOperator {
    fn default() -> Self {
        Self::new()
    }
}

impl ChoiceOperator {
    /// Create a new choice operator
    pub fn new() -> Self {
        ChoiceOperator {
            capacity: 1.0,
            effectiveness: 0.0,
            choice_history: Vec::new(),
        }
    }

    /// Make a choice from possibility space
    ///
    /// Non-deterministic selection (not random, not predetermined).
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Choice = non-deterministic selection from possibility space"
    /// "The mechanism of conscious selection remains a profound mystery"
    pub fn make_choice(
        &mut self,
        possibility_space: &PossibilitySpace,
        entity_state: &EntityState,
        context: &ChoiceContext,
    ) -> ChoiceResult {
        // Calculate choice effectiveness
        let effectiveness = self.calculate_effectiveness(entity_state, context);

        // Select non-deterministically (the profound mystery)
        let selected_index =
            self.select_non_deterministically(possibility_space, entity_state, context);

        // Get the selected possibility
        let selected_possibility = possibility_space
            .possibilities
            .get(selected_index)
            .cloned()
            .unwrap_or_default();

        // Calculate confidence
        let confidence = self.calculate_confidence(&selected_possibility, effectiveness);

        // Calculate STO alignment
        let sto_alignment = self.calculate_sto_alignment(&selected_possibility, context);

        // Calculate choice quality
        let quality = self.calculate_quality(&selected_possibility, confidence, sto_alignment);

        // Create choice record
        let record = ChoiceRecord {
            choice_id: self.choice_history.len() as u64,
            selected_index,
            confidence,
            sto_alignment,
            quality,
            timestamp: 0, // TODO: Add actual timestamp
        };

        // Record the choice
        self.choice_history.push(record.clone());

        // Update effectiveness
        self.effectiveness = effectiveness;

        ChoiceResult {
            selected_possibility,
            confidence,
            sto_alignment,
            quality,
            record,
        }
    }

    /// Calculate choice effectiveness
    ///
    /// Effectiveness depends on consciousness level, capacity, and context.
    fn calculate_effectiveness(
        &self,
        entity_state: &EntityState,
        context: &ChoiceContext,
    ) -> Float {
        // Base effectiveness from consciousness level
        let base_effectiveness = entity_state.consciousness_level;

        // Boost from capacity
        let capacity_boost = self.capacity * 0.3;

        // Boost from vibrational state
        let vibrational_boost = entity_state.vibrational_state.coherence * 0.2;

        // Influence from experience bias
        let experience_boost = context.experience_bias * 0.1;

        // Apply environmental constraints
        let constraint_penalty: Float = context
            .environmental_constraints
            .iter()
            .map(|c| c.severity * 0.1)
            .sum();

        (base_effectiveness + capacity_boost + vibrational_boost + experience_boost
            - constraint_penalty)
            .clamp(0.0, 1.0)
    }

    /// Select non-deterministically (the profound mystery)
    ///
    /// Non-deterministic selection (not random, not predetermined).
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The mechanism of conscious selection remains a profound mystery"
    fn select_non_deterministically(
        &self,
        possibility_space: &PossibilitySpace,
        _entity_state: &EntityState,
        context: &ChoiceContext,
    ) -> usize {
        // The mystery: Non-deterministic selection
        // NOT random (quantum randomness is as far from freedom as determinism)
        // NOT predetermined (not determined by prior state)

        // This is a simulation placeholder for the profound mystery
        // In actual implementation, this would interface with consciousness

        let possibilities = &possibility_space.possibilities;
        if possibilities.is_empty() {
            return 0;
        }

        // Calculate choice weights
        let mut weights: Vec<Float> = possibilities
            .iter()
            .map(|p| {
                let base_weight = p.probability;

                // Apply strong polarity preference (2.0x boost for matching outcomes)
                let polarity_bias = match (context.polarity_preference, p.outcome.outcome_type) {
                    (
                        PolarityPreference::ServiceToOthers,
                        crate::consciousness::possibility_space::OutcomeType::ServiceToOthers,
                    ) => base_weight * 2.0,
                    (
                        PolarityPreference::ServiceToSelf,
                        crate::consciousness::possibility_space::OutcomeType::ServiceToSelf,
                    ) => base_weight * 2.0,
                    _ => 0.0,
                };

                // Apply experience bias
                let experience_factor = context.experience_bias * 0.1;

                // Apply environmental constraints
                let constraint_factor: Float = context
                    .environmental_constraints
                    .iter()
                    .map(|c| c.severity * c.direction * 0.1)
                    .sum();

                // Apply consciousness influence
                let consciousness_factor = context.consciousness_level * 0.1;

                (base_weight
                    + polarity_bias
                    + experience_factor
                    + constraint_factor
                    + consciousness_factor)
                    .max(0.0)
            })
            .collect();

        // Normalize weights
        let total_weight: Float = weights.iter().sum();
        if total_weight > 0.0 {
            for w in &mut weights {
                *w /= total_weight;
            }
        }

        // Non-deterministic selection (placeholder for the mystery)
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut cumulative = 0.0;
        let r: Float = rng.gen();

        for (i, &weight) in weights.iter().enumerate() {
            cumulative += weight;
            if r <= cumulative {
                return i;
            }
        }

        possibilities.len() - 1
    }

    /// Calculate confidence in the choice
    fn calculate_confidence(&self, possibility: &Possibility, effectiveness: Float) -> Float {
        // Confidence is based on possibility probability and effectiveness
        let base_confidence = possibility.probability;
        let effectiveness_modifier = effectiveness * 0.5;

        (base_confidence + effectiveness_modifier).min(1.0)
    }

    /// Calculate STO alignment
    fn calculate_sto_alignment(&self, possibility: &Possibility, context: &ChoiceContext) -> Float {
        // Base alignment from outcome type
        let base_alignment = match possibility.outcome.outcome_type {
            crate::consciousness::possibility_space::OutcomeType::ServiceToOthers => 1.0,
            crate::consciousness::possibility_space::OutcomeType::ServiceToSelf => -1.0,
            crate::consciousness::possibility_space::OutcomeType::Neutral => 0.0,
            crate::consciousness::possibility_space::OutcomeType::Mixed => 0.0,
        };

        // Apply polarity preference
        let preference_modifier = match context.polarity_preference {
            PolarityPreference::ServiceToOthers => 0.2,
            PolarityPreference::ServiceToSelf => -0.2,
            PolarityPreference::Neutral => 0.0,
        };

        // Apply polarity bias from the possibility
        let bias_modifier = possibility.polarity_bias * 0.3;

        (base_alignment + preference_modifier + bias_modifier)
            .clamp(-1.0, 1.0)
    }

    /// Calculate choice quality
    fn calculate_quality(
        &self,
        possibility: &Possibility,
        confidence: Float,
        sto_alignment: Float,
    ) -> Float {
        // Quality is based on confidence, alignment, and outcome impact
        let confidence_factor = confidence * 0.4;
        let alignment_factor = sto_alignment.abs() * 0.3;
        let impact_factor = possibility.outcome.impact * 0.3;

        confidence_factor + alignment_factor + impact_factor
    }

    /// Get choice statistics
    pub fn get_statistics(&self) -> ChoiceStatistics {
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

        let avg_quality = if total_choices > 0 {
            let sum: Float = self.choice_history.iter().map(|r| r.quality).sum();
            sum / total_choices as Float
        } else {
            0.0
        };

        ChoiceStatistics {
            total_choices: total_choices as u64,
            capacity: self.capacity,
            effectiveness: self.effectiveness,
            avg_confidence,
            avg_sto_alignment,
            avg_quality,
        }
    }
}

impl Default for Possibility {
    fn default() -> Self {
        Possibility {
            probability: 0.5,
            outcome: crate::consciousness::possibility_space::Outcome::default(),
            constraints: Vec::new(),
            polarity_bias: 0.0,
        }
    }
}

/// Choice Statistics
///
/// Statistics about choices made by the choice operator.
#[derive(Debug, Clone)]
pub struct ChoiceStatistics {
    /// Total number of choices
    pub total_choices: u64,

    /// Choice capacity
    pub capacity: Float,

    /// Current effectiveness
    pub effectiveness: Float,

    /// Average confidence
    pub avg_confidence: Float,

    /// Average STO alignment
    pub avg_sto_alignment: Float,

    /// Average quality
    pub avg_quality: Float,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::{PolarityState as EntityPolarityState, VibrationalState};

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
            polarity_state: EntityPolarityState {
                polarity_bias: 0.3,
                polarization_strength: 0.5,
            },
            consciousness_level,
            experience_accumulation: 10.0,
            learning_progress: 0.5,
        }
    }

    fn create_test_choice_context(polarity_preference: PolarityPreference) -> ChoiceContext {
        ChoiceContext {
            polarity_preference,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
            consciousness_level: 0.5,
        }
    }

    #[test]
    fn test_choice_operator_creation() {
        let operator = ChoiceOperator::new();

        assert_eq!(operator.capacity, 1.0);
        assert_eq!(operator.effectiveness, 0.0);
        assert!(operator.choice_history.is_empty());
    }

    #[test]
    fn test_make_choice() {
        let mut operator = ChoiceOperator::new();
        let entity_state = create_test_entity_state(0.5);
        let possibility_space = PossibilitySpace::from_entity_state(&entity_state);
        let context = create_test_choice_context(PolarityPreference::Neutral);

        let result = operator.make_choice(&possibility_space, &entity_state, &context);

        assert!(result.confidence >= 0.0);
        assert!(result.confidence <= 1.0);
        assert!(result.sto_alignment >= -1.0);
        assert!(result.sto_alignment <= 1.0);
        assert!(result.quality >= 0.0);
        assert!(result.quality <= 1.0);
        assert_eq!(operator.choice_history.len(), 1);
    }

    #[test]
    fn test_effectiveness_calculation() {
        let operator = ChoiceOperator::new();
        let low_consciousness_state = create_test_entity_state(0.1);
        let high_consciousness_state = create_test_entity_state(0.9);
        let context = create_test_choice_context(PolarityPreference::Neutral);

        let low_effectiveness =
            operator.calculate_effectiveness(&low_consciousness_state, &context);
        let high_effectiveness =
            operator.calculate_effectiveness(&high_consciousness_state, &context);

        assert!(high_effectiveness > low_effectiveness);
    }

    #[test]
    fn test_polarity_preference_influence() {
        let entity_state = create_test_entity_state(0.5);
        let possibility_space = PossibilitySpace::from_entity_state(&entity_state);

        let sto_context = create_test_choice_context(PolarityPreference::ServiceToOthers);
        let sts_context = create_test_choice_context(PolarityPreference::ServiceToSelf);

        // Make multiple choices to test statistical influence (Free Will is non-deterministic)
        // Use fresh operator for each trial to avoid state accumulation
        let mut sto_positive_count = 0;
        let mut sto_negative_count = 0;
        let mut sts_positive_count = 0;
        let mut sts_negative_count = 0;
        let num_trials = 100;

        for _ in 0..num_trials {
            let mut operator = ChoiceOperator::new();
            let sto_result = operator.make_choice(&possibility_space, &entity_state, &sto_context);

            let mut operator_sts = ChoiceOperator::new();
            let sts_result =
                operator_sts.make_choice(&possibility_space, &entity_state, &sts_context);

            if sto_result.sto_alignment > 0.0 {
                sto_positive_count += 1;
            } else if sto_result.sto_alignment < 0.0 {
                sto_negative_count += 1;
            }

            if sts_result.sto_alignment > 0.0 {
                sts_positive_count += 1;
            } else if sts_result.sto_alignment < 0.0 {
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
        let mut operator = ChoiceOperator::new();
        let entity_state = create_test_entity_state(0.5);
        let possibility_space = PossibilitySpace::from_entity_state(&entity_state);
        let context = create_test_choice_context(PolarityPreference::Neutral);

        // Make multiple choices and check for non-determinism
        let mut selected_indices = Vec::new();

        for _ in 0..50 {
            let result = operator.make_choice(&possibility_space, &entity_state, &context);
            selected_indices.push(result.record.selected_index);
        }

        // Non-deterministic means we should see variety in selections
        let unique_indices: std::collections::HashSet<_> = selected_indices.into_iter().collect();
        assert!(
            unique_indices.len() > 1,
            "Selection should be non-deterministic"
        );
    }

    #[test]
    fn test_choice_statistics() {
        let mut operator = ChoiceOperator::new();
        let entity_state = create_test_entity_state(0.5);
        let possibility_space = PossibilitySpace::from_entity_state(&entity_state);
        let context = create_test_choice_context(PolarityPreference::Neutral);

        // Make several choices
        for _ in 0..5 {
            operator.make_choice(&possibility_space, &entity_state, &context);
        }

        let stats = operator.get_statistics();

        assert_eq!(stats.total_choices, 5);
        assert_eq!(stats.capacity, 1.0);
        assert!(stats.effectiveness > 0.0);
        assert!(stats.avg_confidence >= 0.0);
        assert!(stats.avg_confidence <= 1.0);
        assert!(stats.avg_sto_alignment >= -1.0);
        assert!(stats.avg_sto_alignment <= 1.0);
        assert!(stats.avg_quality >= 0.0);
        assert!(stats.avg_quality <= 1.0);
    }

    #[test]
    fn test_environmental_constraints() {
        let operator = ChoiceOperator::new();
        let entity_state = create_test_entity_state(0.5);
        let context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: vec![EnvironmentalConstraint {
                constraint_type: ConstraintType::Physical,
                severity: 0.5,
                direction: -0.5,
            }],
            experience_bias: 0.0,
            consciousness_level: 0.5,
        };

        let effectiveness = operator.calculate_effectiveness(&entity_state, &context);

        // Environmental constraints should reduce effectiveness
        let context_no_constraints = create_test_choice_context(PolarityPreference::Neutral);
        let effectiveness_no_constraints =
            operator.calculate_effectiveness(&entity_state, &context_no_constraints);

        assert!(effectiveness < effectiveness_no_constraints);
    }

    #[test]
    fn test_experience_bias_influence() {
        let operator = ChoiceOperator::new();
        let entity_state = create_test_entity_state(0.5);

        let low_bias_context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
            consciousness_level: 0.5,
        };

        let high_bias_context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.8,
            consciousness_level: 0.5,
        };

        let low_effectiveness = operator.calculate_effectiveness(&entity_state, &low_bias_context);
        let high_effectiveness =
            operator.calculate_effectiveness(&entity_state, &high_bias_context);

        // Higher experience bias should increase effectiveness
        assert!(high_effectiveness > low_effectiveness);
    }
}
