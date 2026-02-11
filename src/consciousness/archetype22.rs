// Archetype 22: The Choice Operator
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "Archetype 22 (The Choice) - The gateway where infinity becomes accessible"
// "Archetype 22 functions as a choice operator that generates possibility space"
//
// This module implements:
// 1. Archetype 22 as the choice operator
// 2. Origin: Emerges from the First Distortion (Free Will) at Indigo-Ray Realm
// 3. Nature: The zero-point polarity moment
// 4. Function: Creates polarity by choosing between Service-to-Others (STO) and Service-to-Self (STS)

use crate::entity_layer7::layer7::EntityState;
use crate::foundation::indigo_realm::IntelligentInfinity;
use crate::types::Float;

/// Archetype 22: The Choice Operator
///
/// The gateway where infinity becomes accessible.
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Origin: Emerges from the First Distortion (Free Will) at Indigo-Ray Realm"
/// "Nature: The zero-point polarity moment"
/// "Function: Creates polarity by choosing between Service-to-Others (STO) and Service-to-Self (STS)"
/// "Significance: All development flows from this moment"
/// "Inheritance: Every Entity contains Archetype 22 embedded in its holographic seed"
#[derive(Debug, Clone)]
pub struct Archetype22 {
    /// The IntelligentInfinity from which Archetype 22 emerges
    pub intelligent_infinity: IntelligentInfinity,

    /// Choice operator state
    pub choice_operator: ChoiceOperator,

    /// Polarity creation state
    pub polarity_creation: PolarityCreation,
}

/// Choice Operator
///
/// Archetype 22 functions as a choice operator that generates possibility space.
#[derive(Debug, Clone)]
pub struct ChoiceOperator {
    /// Operator activation level (0.0 to 1.0)
    pub activation_level: Float,

    /// Possibility space generation capacity
    pub possibility_capacity: Float,

    /// Non-deterministic selection capability
    pub non_deterministic_capability: Float,
}

/// Polarity Creation
///
/// The zero-point polarity moment.
#[derive(Debug, Clone)]
pub struct PolarityCreation {
    /// Polarity state
    pub polarity_state: PolarityState,

    /// STO potential (0.0 to 1.0)
    pub sto_potential: Float,

    /// STS potential (0.0 to 1.0)
    pub sts_potential: Float,

    /// Choice history for polarity development
    pub choice_history: Vec<PolarityChoice>,
}

/// Polarity State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityState {
    /// Unpolarized (neutral)
    Unpolarized,

    /// Polarizing (in process of choosing)
    Polarizing,

    /// Service-to-Others polarized
    ServiceToOthers,

    /// Service-to-Self polarized
    ServiceToSelf,

    /// Balanced (both polarities integrated)
    Balanced,
}

/// Polarity Choice
///
/// A choice that contributes to polarity development.
#[derive(Debug, Clone)]
pub struct PolarityChoice {
    /// Choice orientation
    pub orientation: PolarityOrientation,

    /// Intensity of the choice (0.0 to 1.0)
    pub intensity: Float,

    /// Context of the choice
    pub context: ChoiceContext,
}

/// Polarity Orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarityOrientation {
    /// Service-to-Others
    ServiceToOthers,

    /// Service-to-Self
    ServiceToSelf,

    /// Neutral
    Neutral,
}

/// Choice Context
#[derive(Debug, Clone)]
pub struct ChoiceContext {
    /// Environmental factors
    pub environmental_factors: Vec<EnvironmentalFactor>,

    /// Karmic influences
    pub karmic_influence: Float,

    /// Consciousness level
    pub consciousness_level: Float,
}

/// Environmental Factor
#[derive(Debug, Clone)]
pub struct EnvironmentalFactor {
    /// Factor type
    pub factor_type: FactorType,

    /// Influence strength (-1.0 to 1.0)
    pub influence: Float,
}

/// Factor Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorType {
    /// Physical factor
    Physical,

    /// Mental factor
    Mental,

    /// Emotional factor
    Emotional,

    /// Spiritual factor
    Spiritual,
}

impl Archetype22 {
    /// Create a new Archetype 22 from IntelligentInfinity
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Origin: Emerges from the First Distortion (Free Will) at Indigo-Ray Realm"
    pub fn new(intelligent_infinity: IntelligentInfinity) -> Self {
        Archetype22 {
            intelligent_infinity,
            choice_operator: ChoiceOperator::new(),
            polarity_creation: PolarityCreation::new(),
        }
    }

    /// Generate possibility space
    ///
    /// Archetype 22 functions as a choice operator that generates possibility space.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Possibility space = quantum superposition constrained by entity state"
    pub fn generate_possibility_space(
        &self,
        entity_state: &EntityState,
    ) -> crate::foundation::indigo_realm::PossibilitySpace {
        // Generate possibilities based on entity state
        let base_possibilities = self.generate_base_possibilities(entity_state);

        // Apply constraint from entity state
        let constrained_possibilities = self.apply_constraints(base_possibilities, entity_state);

        // Apply polarity bias
        let polarized_possibilities =
            self.apply_polarity_bias(constrained_possibilities, entity_state);

        // Phase 0: Create EntityConstraints for PossibilitySpace
        let constraints = crate::foundation::indigo_realm::EntityConstraints {
            archetype_activations: [0.0; 22],
            polarization_bias: entity_state.polarity_state.polarity_bias,
            catalyst_intensity: 0.5,
            veil_transparency: entity_state.consciousness_level,
            experience_accumulation: entity_state.experience_accumulation,
            consciousness_level: entity_state.consciousness_level,
        };

        crate::foundation::indigo_realm::PossibilitySpace {
            possibilities: polarized_possibilities,
            constraints,
        }
    }

    /// Make choice
    ///
    /// Non-deterministic selection from possibility space.
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Choice = non-deterministic selection from possibility space"
    /// "The mechanism of conscious selection remains a profound mystery"
    pub fn make_choice(
        &mut self,
        possibility_space: &crate::foundation::indigo_realm::PossibilitySpace,
        entity_state: &EntityState,
    ) -> Choice {
        // Non-deterministic selection (placeholder for the mystery)
        let selected_index = self.select_non_deterministically(possibility_space);

        // Determine polarity orientation
        let orientation =
            self.determine_polarity_orientation(selected_index, possibility_space, entity_state);

        // Calculate intensity
        let intensity = self.calculate_intensity(selected_index, possibility_space);

        // Update polarity creation state
        self.update_polarity_creation(orientation, intensity, entity_state);

        Choice {
            selected_index,
            orientation,
            intensity,
        }
    }

    /// Generate base possibilities
    ///
    /// Phase 0: Generate 3-5 possibilities based on entity state
    fn generate_base_possibilities(
        &self,
        entity_state: &EntityState,
    ) -> Vec<crate::foundation::indigo_realm::Possibility> {
        // Phase 0: Generate 3-5 possibilities (STO, STS, Neutral)
        let mut possibilities = Vec::new();

        // STO possibility
        let sto_probability = 0.4 + entity_state.polarity_state.polarity_bias.max(0.0) * 0.3;
        let sto_archetype_influence = [0.0; 22]; // Placeholder
        possibilities.push(crate::foundation::indigo_realm::Possibility {
            outcome: crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers,
            probability: sto_probability.clamp(0.1, 0.6),
            archetype_influence: sto_archetype_influence,
        });

        // STS possibility
        let sts_probability = 0.4 + entity_state.polarity_state.polarity_bias.abs().max(0.0) * 0.3;
        let sts_archetype_influence = [0.0; 22]; // Placeholder
        possibilities.push(crate::foundation::indigo_realm::Possibility {
            outcome: crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf,
            probability: sts_probability.clamp(0.1, 0.6),
            archetype_influence: sts_archetype_influence,
        });

        // Neutral possibility
        let neutral_probability = if entity_state.polarity_state.polarity_bias.abs() < 0.1 {
            0.4
        } else {
            0.1
        };
        let neutral_archetype_influence = [0.0; 22]; // Placeholder
        possibilities.push(crate::foundation::indigo_realm::Possibility {
            outcome: crate::foundation::indigo_realm::PolarityChoice::Neutral,
            probability: neutral_probability,
            archetype_influence: neutral_archetype_influence,
        });

        possibilities
    }

    /// Apply constraints from entity state
    fn apply_constraints(
        &self,
        possibilities: Vec<crate::foundation::indigo_realm::Possibility>,
        entity_state: &EntityState,
    ) -> Vec<crate::foundation::indigo_realm::Possibility> {
        possibilities
            .into_iter()
            .map(|mut p| {
                // Apply vibrational state constraint
                p.probability *= entity_state.vibrational_state.coherence;

                // Apply polarity state constraint
                let polarity_factor = match entity_state.polarity_state.polarization_strength {
                    x if x < 0.3 => 1.0, // Neutral
                    x if x < 0.7 => 0.9, // Polarizing
                    _ => 0.8,            // Strongly polarized
                };
                p.probability *= polarity_factor;

                p
            })
            .collect()
    }

    /// Apply polarity bias
    fn apply_polarity_bias(
        &self,
        possibilities: Vec<crate::foundation::indigo_realm::Possibility>,
        entity_state: &EntityState,
    ) -> Vec<crate::foundation::indigo_realm::Possibility> {
        // Polarity bias: -1.0 (STS) to 1.0 (STO)
        let polarity_bias = entity_state.polarity_state.polarity_bias;
        let sto_bias = polarity_bias.max(0.0); // STO component (0.0 to 1.0)
        let sts_bias = (-polarity_bias).max(0.0); // STS component (0.0 to 1.0)

        possibilities
            .into_iter()
            .enumerate()
            .map(|(i, mut p)| {
                // Apply STO/STS bias based on index parity
                if i % 2 == 0 {
                    p.probability *= (1.0 + sto_bias * 0.5).min(1.0);
                } else {
                    p.probability *= (1.0 + sts_bias * 0.5).min(1.0);
                }

                p
            })
            .collect()
    }

    /// Select non-deterministically (placeholder for the mystery)
    fn select_non_deterministically(
        &self,
        possibility_space: &crate::foundation::indigo_realm::PossibilitySpace,
    ) -> usize {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let possibilities = &possibility_space.possibilities;
        if possibilities.is_empty() {
            return 0;
        }

        // Weighted random selection (placeholder for true non-determinism)
        let weights: Vec<Float> = possibilities.iter().map(|p| p.probability).collect();
        let total: Float = weights.iter().sum();

        if total > 0.0 {
            let mut cumulative = 0.0;
            let r: Float = rng.gen::<f64>() * total;

            for (i, &weight) in weights.iter().enumerate() {
                cumulative += weight;
                if r <= cumulative {
                    return i;
                }
            }
        }

        possibilities.len() - 1
    }

    /// Determine polarity orientation
    fn determine_polarity_orientation(
        &self,
        selected_index: usize,
        possibility_space: &crate::foundation::indigo_realm::PossibilitySpace,
        _entity_state: &EntityState,
    ) -> PolarityOrientation {
        // Determine orientation based on selected index and entity state
        let selected_possibility = possibility_space.possibilities.get(selected_index);

        if let Some(p) = selected_possibility {
            // Higher probability tends toward STO, lower toward STS
            if p.probability > 0.5 {
                PolarityOrientation::ServiceToOthers
            } else {
                PolarityOrientation::ServiceToSelf
            }
        } else {
            PolarityOrientation::Neutral
        }
    }

    /// Calculate intensity of the choice
    fn calculate_intensity(
        &self,
        selected_index: usize,
        possibility_space: &crate::foundation::indigo_realm::PossibilitySpace,
    ) -> Float {
        if let Some(p) = possibility_space.possibilities.get(selected_index) {
            p.probability
        } else {
            0.5
        }
    }

    /// Update polarity creation state
    fn update_polarity_creation(
        &mut self,
        orientation: PolarityOrientation,
        intensity: Float,
        entity_state: &EntityState,
    ) {
        // Record the choice
        let choice = PolarityChoice {
            orientation,
            intensity,
            context: ChoiceContext {
                environmental_factors: Vec::new(),
                karmic_influence: 0.0,
                consciousness_level: entity_state.consciousness_level,
            },
        };
        self.polarity_creation.choice_history.push(choice);

        // Update potentials
        match orientation {
            PolarityOrientation::ServiceToOthers => {
                self.polarity_creation.sto_potential =
                    (self.polarity_creation.sto_potential + intensity * 0.1).min(1.0);
            }
            PolarityOrientation::ServiceToSelf => {
                self.polarity_creation.sts_potential =
                    (self.polarity_creation.sts_potential + intensity * 0.1).min(1.0);
            }
            PolarityOrientation::Neutral => {}
        }

        // Update polarity state
        self.update_polarity_state();
    }

    /// Update polarity state based on potentials
    fn update_polarity_state(&mut self) {
        let sto = self.polarity_creation.sto_potential;
        let sts = self.polarity_creation.sts_potential;
        let total = sto + sts;

        if total < 0.3 {
            self.polarity_creation.polarity_state = PolarityState::Unpolarized;
        } else if (sto - sts).abs() < 0.2 {
            self.polarity_creation.polarity_state = PolarityState::Polarizing;
        } else if sto > sts {
            self.polarity_creation.polarity_state = PolarityState::ServiceToOthers;
        } else {
            self.polarity_creation.polarity_state = PolarityState::ServiceToSelf;
        }
    }

    /// Get polarity creation statistics
    pub fn get_polarity_statistics(&self) -> PolarityStatistics {
        PolarityStatistics {
            polarity_state: self.polarity_creation.polarity_state,
            sto_potential: self.polarity_creation.sto_potential,
            sts_potential: self.polarity_creation.sts_potential,
            total_choices: self.polarity_creation.choice_history.len() as u64,
        }
    }
}

impl ChoiceOperator {
    /// Create a new choice operator
    pub fn new() -> Self {
        ChoiceOperator {
            activation_level: 0.0,
            possibility_capacity: 1.0,
            non_deterministic_capability: 1.0,
        }
    }
}

impl PolarityCreation {
    /// Create a new polarity creation state
    pub fn new() -> Self {
        PolarityCreation {
            polarity_state: PolarityState::Unpolarized,
            sto_potential: 0.0,
            sts_potential: 0.0,
            choice_history: Vec::new(),
        }
    }
}

/// Choice
///
/// A non-deterministic selection from possibility space.
#[derive(Debug, Clone)]
pub struct Choice {
    /// Selected possibility index
    pub selected_index: usize,

    /// Polarity orientation
    pub orientation: PolarityOrientation,

    /// Intensity of the choice
    pub intensity: Float,
}

/// Polarity Statistics
///
/// Statistics about polarity development.
#[derive(Debug, Clone)]
pub struct PolarityStatistics {
    /// Current polarity state
    pub polarity_state: PolarityState,

    /// STO potential
    pub sto_potential: Float,

    /// STS potential
    pub sts_potential: Float,

    /// Total number of choices
    pub total_choices: u64,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::{PolarityState as EntityPolarityState, VibrationalState};
    use crate::foundation::indigo_realm::IntelligentInfinity;

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

    fn create_test_archetype22() -> Archetype22 {
        let intelligent_infinity = IntelligentInfinity::new();
        Archetype22::new(intelligent_infinity)
    }

    #[test]
    fn test_archetype22_creation() {
        let intelligent_infinity = IntelligentInfinity::new();
        let archetype22 = Archetype22::new(intelligent_infinity);

        assert_eq!(
            archetype22.polarity_creation.polarity_state,
            PolarityState::Unpolarized
        );
        assert_eq!(archetype22.polarity_creation.sto_potential, 0.0);
        assert_eq!(archetype22.polarity_creation.sts_potential, 0.0);
    }

    #[test]
    fn test_generate_possibility_space() {
        let archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        let possibility_space = archetype22.generate_possibility_space(&entity_state);

        assert!(!possibility_space.possibilities.is_empty());
        assert!(possibility_space.possibilities.len() >= 5);
    }

    #[test]
    fn test_make_choice() {
        let mut archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        let possibility_space = archetype22.generate_possibility_space(&entity_state);
        let choice = archetype22.make_choice(&possibility_space, &entity_state);

        assert!(choice.selected_index < possibility_space.possibilities.len());
        assert!(choice.intensity >= 0.0);
        assert!(choice.intensity <= 1.0);
    }

    #[test]
    fn test_polarity_creation_update() {
        let mut archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        let initial_polarity_state = archetype22.polarity_creation.polarity_state;

        // Make several choices
        for _ in 0..10 {
            let possibility_space = archetype22.generate_possibility_space(&entity_state);
            archetype22.make_choice(&possibility_space, &entity_state);
        }

        // Polarity state should have updated
        let stats = archetype22.get_polarity_statistics();
        assert!(stats.total_choices > 0);
        assert!(stats.sto_potential > 0.0 || stats.sts_potential > 0.0);
    }

    #[test]
    fn test_polarity_potential_growth() {
        let mut archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        let initial_sto = archetype22.polarity_creation.sto_potential;
        let initial_sts = archetype22.polarity_creation.sts_potential;

        // Make several choices
        for _ in 0..10 {
            let possibility_space = archetype22.generate_possibility_space(&entity_state);
            archetype22.make_choice(&possibility_space, &entity_state);
        }

        // Potentials should have grown
        assert!(
            archetype22.polarity_creation.sto_potential
                + archetype22.polarity_creation.sts_potential
                > initial_sto + initial_sts
        );
    }

    #[test]
    fn test_polarity_state_transitions() {
        let mut archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        // Start unpolarized
        assert_eq!(
            archetype22.polarity_creation.polarity_state,
            PolarityState::Unpolarized
        );

        // Make choices to develop polarity
        for _ in 0..20 {
            let possibility_space = archetype22.generate_possibility_space(&entity_state);
            archetype22.make_choice(&possibility_space, &entity_state);
        }

        // Should have transitioned to a polarized state
        let final_state = archetype22.polarity_creation.polarity_state;
        assert!(final_state != PolarityState::Unpolarized);
    }

    #[test]
    fn test_polarity_statistics() {
        let mut archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        // Make some choices
        for _ in 0..5 {
            let possibility_space = archetype22.generate_possibility_space(&entity_state);
            archetype22.make_choice(&possibility_space, &entity_state);
        }

        let stats = archetype22.get_polarity_statistics();

        assert_eq!(stats.total_choices, 5);
        assert!(stats.sto_potential >= 0.0);
        assert!(stats.sts_potential >= 0.0);
    }

    #[test]
    fn test_non_deterministic_selection() {
        let archetype22 = create_test_archetype22();
        let entity_state = create_test_entity_state(0.5);

        let possibility_space = archetype22.generate_possibility_space(&entity_state);

        // Make multiple selections and check for non-determinism
        let mut selected_indices = Vec::new();

        for _ in 0..50 {
            let index = archetype22.select_non_deterministically(&possibility_space);
            selected_indices.push(index);
        }

        // Non-deterministic means we should see variety in selections
        let unique_indices: std::collections::HashSet<_> = selected_indices.into_iter().collect();
        assert!(
            unique_indices.len() > 1,
            "Selection should be non-deterministic"
        );
    }
}
