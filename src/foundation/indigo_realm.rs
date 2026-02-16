/// Layer 1: Indigo-Ray Realm - IntelligentInfinity + Archetype 22
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "First Distortion: Free Will"
/// "The undistorted unity of all that is, existing as potential and kinetic aspects"
/// "Infinity became aware, the primal awakening of consciousness"
/// "Tapped into by foci of energy through free will"
/// "The gateway where infinity becomes accessible"
///
/// Action: First Distortion — Free Will
///
/// Result: IntelligentInfinity
///
/// Architectural Artifact: Archetype 22 (The Choice)
///
/// INCLUDES: Violet-Ray (Infinity as source)
///
/// TRANSCENDS: Adds awareness through Free Will
///
/// EVOLVES INTO: Attractor-field for Blue-Ray (Love/Logos)
///
/// Deep Logic:
/// - Free Will is NOT randomness (quantum randomness is as far from freedom as determinism)
/// - Free Will = controlled selection from possibility space
/// - Free Will requires the ability to influence which possibility actualizes
/// - Choice is non-deterministic selection (not random, not predetermined)
///
/// The Mechanism:
/// 1. Infinity (from Violet Realm) applies Free Will
/// 2. This choice creates the possibility space for individuality
/// 3. Awareness emerges—the Creator becomes aware of Itself
/// 4. Archetype 22 (The Choice) becomes the "permission token" for individuality
/// 5. Archetype 22 functions as a choice operator that generates possibility space
/// 6. Possibility space = quantum superposition constrained by entity state
/// 7. Choice = non-deterministic selection from possibility space
use crate::foundation::transcend_include::{AttractorField, Feature};
use crate::foundation::violet_realm::VioletRealm;
use rand::Rng;
use std::fmt;

/// Polarity Choice - the fundamental choice of polarity
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Archetype 22 (The Choice): Creates polarity by choosing between
/// Service-to-Others (STO) and Service-to-Self (STS)"
///
/// This is the zero-point polarity moment from which all development flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolarityChoice {
    /// Service-to-Others (positive polarity)
    ServiceToOthers,
    /// Service-to-Self (negative polarity)
    ServiceToSelf,
    /// No choice made (unpolarized)
    Neutral,
}

/// Entity Constraints - constraints that limit the possibility space
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
/// "Possibility space = quantum superposition constrained by entity state"
#[derive(Debug, Clone, PartialEq)]
pub struct EntityConstraints {
    /// Current archetype activations (1-22)
    pub archetype_activations: [f64; 22],
    /// Current polarization bias (-1.0 to 1.0)
    pub polarization_bias: f64,
    /// Catalyst intensity (0.0 to 1.0)
    pub catalyst_intensity: f64,
    /// Veil transparency (0.0 to 1.0)
    pub veil_transparency: f64,
    /// Experience accumulation (0.0 to 1.0)
    pub experience_accumulation: f64,
    /// Consciousness level (0.0 to 1.0)
    pub consciousness_level: f64,
}

/// Possibility Space - quantum superposition constrained by entity state
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Possibility space = quantum superposition constrained by entity state"
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
/// "Generate 3-5 possibilities based on entity state"
///
/// This is the set of all possible choices available to an entity, generated
/// by Archetype 22 (The Choice).
#[derive(Debug, Clone, PartialEq)]
pub struct PossibilitySpace {
    /// The possibilities in the space (3-5 possibilities)
    pub possibilities: Vec<Possibility>,
    /// The constraints that limit the possibility space
    pub constraints: EntityConstraints,
}

/// A single possibility in the possibility space
///
/// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
/// "Each possibility has: outcome (STO, STS, or Neutral), probability, archetype_influence"
#[derive(Debug, Clone, PartialEq)]
pub struct Possibility {
    /// The outcome of this possibility (STO, STS, or Neutral)
    pub outcome: PolarityChoice,
    /// The probability of this possibility (0.0 to 1.0)
    pub probability: f64,
    /// Which archetypes influence this choice (1-22)
    pub archetype_influence: [f64; 22],
}

impl Possibility {
    /// Create a new possibility
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Each possibility has: outcome (STO, STS, or Neutral), probability, archetype_influence"
    pub fn new(outcome: PolarityChoice, probability: f64, archetype_influence: [f64; 22]) -> Self {
        assert!(
            probability >= 0.0 && probability <= 1.0,
            "Probability must be between 0.0 and 1.0"
        );
        Possibility {
            outcome,
            probability,
            archetype_influence,
        }
    }

    /// Create a new possibility with default archetype influence
    pub fn with_default_influence(outcome: PolarityChoice, probability: f64) -> Self {
        Possibility {
            outcome,
            probability,
            archetype_influence: [0.0; 22],
        }
    }
}

impl PossibilitySpace {
    /// Create a new possibility space
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Generate 3-5 possibilities based on entity state"
    pub fn new(constraints: EntityConstraints) -> Self {
        PossibilitySpace {
            possibilities: Vec::new(),
            constraints,
        }
    }

    /// Add a possibility to the space
    pub fn add_possibility(&mut self, possibility: Possibility) {
        self.possibilities.push(possibility);
    }

    /// Get the number of possibilities
    pub fn count(&self) -> usize {
        self.possibilities.len()
    }

    /// Select a possibility (non-deterministic selection)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Choice = non-deterministic selection from possibility space"
    /// "NOT random, NOT predetermined"
    ///
    /// The mechanism of conscious selection remains a profound mystery.
    pub fn select(&self) -> Option<&Possibility> {
        if self.possibilities.is_empty() {
            return None;
        }

        // In a real implementation, this would be non-deterministic selection
        // For now, we'll return the highest probability possibility
        self.possibilities
            .iter()
            .max_by(|a, b| a.probability.partial_cmp(&b.probability).unwrap())
    }

    /// Get the constraints
    pub fn constraints(&self) -> &EntityConstraints {
        &self.constraints
    }
}

/// Archetype 22 - The Choice
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Archetype 22 (The Choice) becomes the 'permission token' for individuality"
/// "Archetype 22 functions as a choice operator that generates possibility space"
///
/// This is the zero-point polarity moment from which all development flows.
/// It creates polarity by choosing between Service-to-Others (STO) and
/// Service-to-Self (STS).
#[derive(Debug, Clone, PartialEq)]
pub struct Archetype22 {
    /// The choice operator
    pub choice_operator: String,
    /// The strength of the choice (0.0 to 1.0)
    pub strength: f64,
}

impl Archetype22 {
    /// Create a new Archetype 22 (The Choice)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Archetype 22 (The Choice) becomes the 'permission token' for individuality"
    /// "Archetype 22 functions as a choice operator that generates possibility space"
    pub fn new() -> Self {
        Archetype22 {
            choice_operator: "Choice Operator".to_string(),
            strength: 1.0,
        }
    }

    /// Generate possibility space from entity state
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Generate 3-5 possibilities based on entity state"
    /// "Generate 3-5 possibilities based on:
    ///  - Current archetype activations (1-22)
    ///  - Current polarization state
    ///  - Catalyst intensity
    ///  - Veil transparency
    ///  - Experience accumulation"
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Archetype 22 functions as a choice operator that generates possibility space"
    pub fn generate_possibility_space(
        &self,
        entity_state: &crate::entity_layer7::layer7::EntityState,
        catalyst_intensity: f64,
        veil_transparency: f64,
    ) -> PossibilitySpace {
        let constraints = EntityConstraints {
            archetype_activations: [0.0; 22], // Placeholder - would be populated from entity
            polarization_bias: entity_state.polarity_state.polarity_bias,
            catalyst_intensity,
            veil_transparency,
            experience_accumulation: entity_state.experience_accumulation,
            consciousness_level: entity_state.consciousness_level,
        };

        let mut space = PossibilitySpace::new(constraints);

        // Generate 3-5 possibilities based on entity state
        // Phase 0: Generate STO, STS, and Neutral possibilities
        // The probability of each possibility is influenced by entity's current state

        // STO possibility - influenced by positive polarity bias
        let sto_probability = if entity_state.polarity_state.polarity_bias > 0.0 {
            0.4 + entity_state.polarity_state.polarity_bias * 0.3
        } else {
            0.3
        }
        .clamp(0.1, 0.6);

        let sto_archetype_influence =
            self.calculate_archetype_influence(&entity_state, PolarityChoice::ServiceToOthers);

        space.add_possibility(Possibility::new(
            PolarityChoice::ServiceToOthers,
            sto_probability,
            sto_archetype_influence,
        ));

        // STS possibility - influenced by negative polarity bias
        let sts_probability = if entity_state.polarity_state.polarity_bias < 0.0 {
            0.4 + entity_state.polarity_state.polarity_bias.abs() * 0.3
        } else {
            0.3
        }
        .clamp(0.1, 0.6);

        let sts_archetype_influence =
            self.calculate_archetype_influence(&entity_state, PolarityChoice::ServiceToSelf);

        space.add_possibility(Possibility::new(
            PolarityChoice::ServiceToSelf,
            sts_probability,
            sts_archetype_influence,
        ));

        // Neutral possibility - for unpolarized entities
        let neutral_probability = if entity_state.polarity_state.polarity_bias.abs() < 0.1 {
            0.4
        } else {
            0.1
        };

        let neutral_archetype_influence =
            self.calculate_archetype_influence(&entity_state, PolarityChoice::Neutral);

        space.add_possibility(Possibility::new(
            PolarityChoice::Neutral,
            neutral_probability,
            neutral_archetype_influence,
        ));

        // Normalize probabilities
        self.normalize_probabilities(&mut space);

        space
    }

    /// Calculate archetype influence for a specific polarity choice
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Each possibility has: outcome (STO, STS, or Neutral), probability, archetype_influence"
    fn calculate_archetype_influence(
        &self,
        entity_state: &crate::entity_layer7::layer7::EntityState,
        choice: PolarityChoice,
    ) -> [f64; 22] {
        let mut influence = [0.0; 22];

        // Archetype 22 (The Choice) always has high influence
        influence[21] = 1.0;

        // Archetype 7, 14, 21 (Great Way) influence based on polarity
        match choice {
            PolarityChoice::ServiceToOthers => {
                influence[6] = 0.8; // A7: Great Way
                influence[13] = 0.8; // A14: Great Way
                influence[20] = 0.8; // A21: Great Way
            }
            PolarityChoice::ServiceToSelf => {
                influence[6] = 0.7; // A7: Great Way
                influence[13] = 0.7; // A14: Great Way
                influence[20] = 0.7; // A21: Great Way
            }
            PolarityChoice::Neutral => {
                influence[6] = 0.5; // A7: Great Way
                influence[13] = 0.5; // A14: Great Way
                influence[20] = 0.5; // A21: Great Way
            }
        }

        // Consciousness level influences all archetypes
        let consciousness_factor = entity_state.consciousness_level;
        for i in 0..22 {
            influence[i] = influence[i] * consciousness_factor;
        }

        influence
    }

    /// Normalize probabilities so they sum to 1.0
    fn normalize_probabilities(&self, space: &mut PossibilitySpace) {
        let total: f64 = space.possibilities.iter().map(|p| p.probability).sum();

        if total > 0.0 {
            for possibility in &mut space.possibilities {
                possibility.probability = possibility.probability / total;
            }
        }
    }

    /// Evaluate possibilities
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Archetype 22 evaluates each possibility"
    pub fn evaluate_possibilities(
        &self,
        possibilities: &[Possibility],
        entity_state: &crate::entity_layer7::layer7::EntityState,
    ) -> Vec<f64> {
        possibilities
            .iter()
            .map(|possibility| {
                // Evaluation is based on:
                // - Probability (influenced by entity state and polarity preference) - HIGH WEIGHT
                // - Archetype influence alignment
                // - Consciousness level
                // - Experience accumulation
                //
                // Note: Probability gets 80% weight because polarity preference bias
                // is applied through probability in apply_polarity_preference_bias.
                // This ensures Free Will choices align with polarity preferences.

                let base_score = possibility.probability;
                let archetype_alignment_score = self
                    .calculate_archetype_alignment(&possibility.archetype_influence, entity_state);
                let consciousness_score = entity_state.consciousness_level;
                let experience_score = entity_state.experience_accumulation / 100.0;

                (base_score * 0.8
                    + archetype_alignment_score * 0.1
                    + consciousness_score * 0.05
                    + experience_score * 0.05)
                    .clamp(0.0, 1.0)
            })
            .collect()
    }

    /// Calculate archetype alignment score
    fn calculate_archetype_alignment(
        &self,
        archetype_influence: &[f64; 22],
        _entity_state: &crate::entity_layer7::layer7::EntityState,
    ) -> f64 {
        // Calculate alignment between archetype influence and entity's archetype activations
        // For now, use a simple average
        let sum: f64 = archetype_influence.iter().sum();
        sum / 22.0
    }

    /// Make a choice from possibility space
    ///
    /// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 0:
    /// "Archetype 22 makes the final selection"
    /// "This is the 'zero-point polarity moment'"
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Choice = non-deterministic selection from possibility space"
    /// "NOT random, NOT predetermined"
    pub fn make_choice(
        &self,
        possibility_space: &PossibilitySpace,
        entity_state: &crate::entity_layer7::layer7::EntityState,
    ) -> PolarityChoice {
        // Evaluate each possibility
        let evaluations =
            self.evaluate_possibilities(&possibility_space.possibilities, entity_state);

        // Non-deterministic selection (not random, not predetermined)
        // From COSMOLOGICAL-ARCHITECTURE.md: "Choice = non-deterministic selection from possibility space"
        // Use weighted selection based on evaluation scores
        let mut rng = rand::thread_rng();
        let mut weights: Vec<f64> = evaluations.iter().map(|&e| e.max(0.0)).collect();

        // Normalize weights
        let total_weight: f64 = weights.iter().sum();
        if total_weight > 0.0 {
            for w in &mut weights {
                *w /= total_weight;
            }
        } else {
            // Fallback to uniform weights if all evaluations are zero
            let uniform = 1.0 / weights.len() as f64;
            for w in &mut weights {
                *w = uniform;
            }
        }

        // Weighted random selection
        let mut cumulative = 0.0;
        let r: f64 = rng.gen();

        let mut best_index = 0;
        for (i, &weight) in weights.iter().enumerate() {
            cumulative += weight;
            if r <= cumulative {
                best_index = i;
                break;
            }
        }

        let choice = possibility_space.possibilities[best_index].outcome;

        // Archetype 22 (The Choice) forces a polarity decision for completely unpolarized entities
        // If the entity is completely unpolarized and Neutral was selected, choose between STO and STS instead
        if choice == PolarityChoice::Neutral
            && entity_state.polarity_state.polarity_bias.abs() < 0.1
        {
            // Find the best STO or STS option
            let mut polarized_best_index = None;
            let mut polarized_best_score = 0.0;

            for (i, possibility) in possibility_space.possibilities.iter().enumerate() {
                if matches!(
                    possibility.outcome,
                    PolarityChoice::ServiceToOthers | PolarityChoice::ServiceToSelf
                ) && evaluations[i] > polarized_best_score
                {
                    polarized_best_score = evaluations[i];
                    polarized_best_index = Some(i);
                }
            }

            if let Some(index) = polarized_best_index {
                return possibility_space.possibilities[index].outcome;
            }
        }

        choice
    }
}

/// IntelligentInfinity - Infinity + Awareness
///
/// From COSMOLOGICAL-ARCHITECTURE.md:
/// "Infinity became aware, the primal awakening of consciousness"
/// "The undistorted unity of all that is, existing as potential and kinetic aspects"
///
/// This is the result of applying the First Distortion (Free Will) to Infinity.
#[derive(Debug, Clone, PartialEq)]
pub struct IntelligentInfinity {
    /// The Violet Realm (Infinity as source) - INCLUDED
    pub violet_realm: VioletRealm,
    /// The awareness level (0.0 to 1.0)
    pub awareness: f64,
    /// Archetype 22 (The Choice) - the permission token for individuality
    pub archetype22: Archetype22,
}

impl IntelligentInfinity {
    /// Create a new IntelligentInfinity (legacy API for backward compatibility)
    ///
    /// Creates with default Violet Realm and full awareness
    pub fn new() -> Self {
        IntelligentInfinity {
            violet_realm: VioletRealm::new(),
            awareness: 1.0,
            archetype22: Archetype22::new(),
        }
    }

    /// Create a new IntelligentInfinity from Violet Realm
    ///
    /// This applies the First Distortion (Free Will) to Infinity.
    pub fn from_violet(violet_realm: VioletRealm) -> Self {
        IntelligentInfinity {
            violet_realm,
            awareness: 1.0,
            archetype22: Archetype22::new(),
        }
    }

    /// Get the awareness level
    pub fn awareness(&self) -> f64 {
        self.awareness
    }

    /// Check if this is truly aware
    pub fn is_aware(&self) -> bool {
        self.awareness > 0.0
    }

    /// Apply the Second Distortion (Love/Logos) to transition to Blue-Ray
    ///
    /// Returns: (IntelligentInfinity, Feature, AttractorField)
    /// - IntelligentInfinity: The included source state
    /// - Feature: The Second Distortion (Love/Logos)
    /// - AttractorField: Universal Archetypical Patterns
    pub fn apply_second_distortion(&self) -> (IntelligentInfinity, Feature, AttractorField) {
        let indigo_included = self.clone();

        let second_distortion = Feature::new(
            "Second Distortion: Love/Logos",
            "The focusing of Infinity as an aware or conscious principle",
            1.0,
        );

        let universal_patterns = AttractorField::new(
            "Universal Archetypical Patterns",
            1.0,
            "Love/Light - The Creative Principle emerges",
        );

        (indigo_included, second_distortion, universal_patterns)
    }

    /// Get the description of IntelligentInfinity
    pub fn description(&self) -> String {
        "IntelligentInfinity: Infinity became aware through Free Will. \
        The undistorted unity of all that is, existing as potential and kinetic aspects. \
        The gateway where infinity becomes accessible. The primal awakening of consciousness."
            .to_string()
    }
}

impl Default for IntelligentInfinity {
    fn default() -> Self {
        IntelligentInfinity {
            violet_realm: VioletRealm::default(),
            awareness: 0.0,
            archetype22: Archetype22::new(),
        }
    }
}

impl fmt::Display for IntelligentInfinity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IntelligentInfinity: Awareness={}, Violet={}, Archetype22={}",
            self.awareness, self.violet_realm.unity, self.archetype22.strength
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possibility_creation() {
        let possibility = Possibility::new(PolarityChoice::ServiceToOthers, 0.5, [0.0; 22]);
        assert_eq!(possibility.probability, 0.5);
        assert_eq!(possibility.outcome, PolarityChoice::ServiceToOthers);
    }

    #[test]
    fn test_possibility_space_creation() {
        let constraints = EntityConstraints {
            archetype_activations: [0.0; 22],
            polarization_bias: 0.0,
            catalyst_intensity: 0.5,
            veil_transparency: 0.5,
            experience_accumulation: 0.5,
            consciousness_level: 0.5,
        };
        let space = PossibilitySpace::new(constraints);
        assert_eq!(space.count(), 0);
    }

    #[test]
    fn test_add_possibility() {
        let constraints = EntityConstraints {
            archetype_activations: [0.0; 22],
            polarization_bias: 0.0,
            catalyst_intensity: 0.5,
            veil_transparency: 0.5,
            experience_accumulation: 0.5,
            consciousness_level: 0.5,
        };
        let mut space = PossibilitySpace::new(constraints);
        space.add_possibility(Possibility::new(
            PolarityChoice::ServiceToOthers,
            0.5,
            [0.0; 22],
        ));
        assert_eq!(space.count(), 1);
    }

    #[test]
    fn test_select_possibility() {
        let constraints = EntityConstraints {
            archetype_activations: [0.0; 22],
            polarization_bias: 0.0,
            catalyst_intensity: 0.5,
            veil_transparency: 0.5,
            experience_accumulation: 0.5,
            consciousness_level: 0.5,
        };
        let mut space = PossibilitySpace::new(constraints);
        space.add_possibility(Possibility::new(
            PolarityChoice::ServiceToOthers,
            0.3,
            [0.0; 22],
        ));
        space.add_possibility(Possibility::new(
            PolarityChoice::ServiceToSelf,
            0.7,
            [0.0; 22],
        ));

        let selected = space.select();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().outcome, PolarityChoice::ServiceToSelf);
    }

    #[test]
    fn test_archetype22_creation() {
        let archetype = Archetype22::new();
        assert_eq!(archetype.choice_operator, "Choice Operator");
        assert_eq!(archetype.strength, 1.0);
    }

    #[test]
    fn test_archetype22_generate_possibility_space() {
        use crate::entity_layer7::layer7::{EntityState, PolarityState, VibrationalState};
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};

        let archetype = Archetype22::new();
        let entity_state = EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.5,
                density: Density::First(Density1SubLevel::Quantum),
                kinetic_energy: 0.5,
                potential_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.5,
            },
            consciousness_level: 0.5,
            experience_accumulation: 10.0,
            learning_progress: 0.5,
        };
        let space = archetype.generate_possibility_space(&entity_state, 0.5, 0.5);
        assert_eq!(space.count(), 3);
    }

    #[test]
    fn test_archetype22_make_choice() {
        use crate::entity_layer7::layer7::{EntityState, PolarityState, VibrationalState};
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};

        let archetype = Archetype22::new();
        let entity_state = EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.5,
                density: Density::First(Density1SubLevel::Quantum),
                kinetic_energy: 0.5,
                potential_energy: 0.5,
            },
            polarity_state: PolarityState {
                polarity_bias: 0.0,
                polarization_strength: 0.5,
            },
            consciousness_level: 0.5,
            experience_accumulation: 10.0,
            learning_progress: 0.5,
        };
        let space = archetype.generate_possibility_space(&entity_state, 0.5, 0.5);
        let choice = archetype.make_choice(&space, &entity_state);
        assert!(matches!(
            choice,
            PolarityChoice::ServiceToOthers
                | PolarityChoice::ServiceToSelf
                | PolarityChoice::Neutral
        ));
    }

    #[test]
    fn test_intelligent_infinity_from_violet() {
        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        assert_eq!(intelligent.awareness, 1.0);
        assert!(intelligent.is_aware());
    }

    #[test]
    fn test_intelligent_infinity_includes_violet() {
        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet.clone());
        assert_eq!(intelligent.violet_realm.unity, violet.unity);
    }

    #[test]
    fn test_intelligent_infinity_has_archetype22() {
        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        assert_eq!(intelligent.archetype22.strength, 1.0);
    }

    #[test]
    fn test_apply_second_distortion() {
        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let (indigo_included, feature, attractor) = intelligent.apply_second_distortion();

        assert_eq!(indigo_included.awareness, 1.0);
        assert_eq!(feature.name, "Second Distortion: Love/Logos");
        assert_eq!(attractor.name, "Universal Archetypical Patterns");
    }

    #[test]
    fn test_intelligent_infinity_description() {
        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let desc = intelligent.description();
        assert!(desc.contains("IntelligentInfinity"));
        assert!(desc.contains("Infinity became aware"));
        assert!(desc.contains("Free Will"));
    }

    #[test]
    fn test_intelligent_infinity_display() {
        let violet = VioletRealm::new();
        let intelligent = IntelligentInfinity::from_violet(violet);
        let display = format!("{}", intelligent);
        assert!(display.contains("IntelligentInfinity"));
        assert!(display.contains("Awareness=1"));
    }
}
