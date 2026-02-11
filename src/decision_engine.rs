// Decision-Making Engine - Phase 14
// Implements traceable decision-making logic with both STO and STS outcomes
//
// This module provides the core decision-making functionality for entities,
// enabling both Service-to-Others (STO) and Service-to-Self (STS) choices
// based on 8 factors: archetypes, physical experiences, veil thickness,
// free will capacity, previous choices, environmental influences,
// natural laws, and probability distribution.

use crate::fractal_holographic_structure::{
    ChangeType, HolographicContainer, StateChange, StateID,
};
use crate::natural_laws::NaturalLaws;
use crate::physical_dimension::{PhysicalExperience, PhysicalExperienceType};
use crate::probability::{Complex, ProbabilitySpace};
use crate::types::{Density, Float};

/// Choice type (STO or STS)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Choice {
    /// Service-to-Others (positive polarity)
    STO,
    /// Service-to-Self (negative polarity)
    STS,
}

/// Entity ID type
pub type EntityID = u64;

/// Environmental influence that affects decision-making
#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentalInfluence {
    pub influence_type: EnvironmentalInfluenceType,
    pub intensity: Float, // 0.0 to 1.0
    pub duration: Float,
    pub timestamp: Float,
}

/// Types of environmental influences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnvironmentalInfluenceType {
    Scarcity,
    Abundance,
    Danger,
    Safety,
    Competition,
    Cooperation,
    Isolation,
    Community,
    Chaos,
    Order,
}

impl EnvironmentalInfluenceType {
    /// Returns whether this influence type tends toward STO or STS
    pub fn polarization_tendency(&self) -> Option<Choice> {
        match self {
            EnvironmentalInfluenceType::Scarcity
            | EnvironmentalInfluenceType::Danger
            | EnvironmentalInfluenceType::Competition
            | EnvironmentalInfluenceType::Isolation
            | EnvironmentalInfluenceType::Chaos => Some(Choice::STS),

            EnvironmentalInfluenceType::Abundance
            | EnvironmentalInfluenceType::Safety
            | EnvironmentalInfluenceType::Cooperation
            | EnvironmentalInfluenceType::Community
            | EnvironmentalInfluenceType::Order => Some(Choice::STO),
        }
    }
}

impl EnvironmentalInfluence {
    /// Calculate the STO influence of this environmental factor
    pub fn sto_influence(&self) -> Float {
        match self.influence_type {
            EnvironmentalInfluenceType::Abundance
            | EnvironmentalInfluenceType::Safety
            | EnvironmentalInfluenceType::Cooperation
            | EnvironmentalInfluenceType::Community
            | EnvironmentalInfluenceType::Order => self.intensity,
            _ => 0.0,
        }
    }

    /// Calculate the STS influence of this environmental factor
    pub fn sts_influence(&self) -> Float {
        match self.influence_type {
            EnvironmentalInfluenceType::Scarcity
            | EnvironmentalInfluenceType::Danger
            | EnvironmentalInfluenceType::Competition
            | EnvironmentalInfluenceType::Isolation
            | EnvironmentalInfluenceType::Chaos => self.intensity,
            _ => 0.0,
        }
    }
}

/// Explanation of a decision
#[derive(Debug, Clone, PartialEq)]
pub struct DecisionExplanation {
    pub sto_weight: Float,
    pub sts_weight: Float,
    pub contributing_factors: Vec<ContributingFactor>,
    pub final_choice: Choice,
    pub decision_timestamp: Float,
}

/// A factor that contributed to a decision
#[derive(Debug, Clone, PartialEq)]
pub struct ContributingFactor {
    pub factor_name: String,
    pub influence: Float, // -1.0 to 1.0
    pub direction: InfluenceDirection,
    pub weight: Float, // How much this factor contributed (0.0 to 1.0)
}

/// Direction of influence (STO, STS, or Neutral)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfluenceDirection {
    STO,
    STS,
    Neutral,
}

/// Main decision-making engine
#[derive(Debug, Clone)]
pub struct DecisionEngine {
    /// Archetype activations (22 archetypes from EntityArchetypicalMind)
    pub archetype_activations: [Float; 22],

    /// Physical experiences that influence choices
    pub physical_experiences: Vec<PhysicalExperience>,

    /// Thickness of the veil (0.0 = no veil, 1.0 = thickest veil)
    pub veil_thickness: Float,

    /// Free will capacity (0.0 = no free will, 1.0 = full free will)
    pub free_will_capacity: Float,

    /// Previous choices (affects future choices through karma)
    pub previous_choices: Vec<Choice>,

    /// Environmental influences
    pub environmental_influences: Vec<EnvironmentalInfluence>,

    /// Natural laws that constrain choices
    pub natural_laws: NaturalLaws,

    /// Probability space for collapsing possibilities
    pub probability_space: ProbabilitySpace,

    /// Holographic container with 343 states
    pub holographic_container: HolographicContainer,

    /// Current density level
    pub current_density: Density,

    /// Entity ID
    pub entity_id: EntityID,

    /// Seed for deterministic behavior
    pub seed: u64,
}

impl DecisionEngine {
    /// Mix two seed values to create a well-distributed seed
    ///
    /// This function combines two seed values using bit manipulation and
    /// hashing operations to ensure the resulting seed is well-distributed
    /// and doesn't produce correlated random values.
    fn mix_seed(base_seed: u64, variation: u64) -> u64 {
        let mut result = base_seed ^ variation;
        result = result.wrapping_mul(0x9e3779b97f4a7c15);
        result = result.rotate_left(31);
        result = result.wrapping_mul(0xbf58476d1ce4e5b9);
        result ^= result >> 27;
        result = result.wrapping_mul(0x94d049bb133111eb);
        result ^= result >> 31;
        result
    }

    /// Create a new decision engine with default values
    pub fn new(entity_id: EntityID, seed: u64) -> Self {
        // Initialize archetype activations with balanced values
        let archetype_activations = [0.5; 22];

        // Initialize holographic container
        let holographic_container = HolographicContainer::new();

        // Initialize probability space with 2 outcomes (STO and STS)
        let probability_space = ProbabilitySpace::with_outcomes(2, seed);

        // Initialize natural laws with default values
        let natural_laws = NaturalLaws::default();

        DecisionEngine {
            archetype_activations,
            physical_experiences: Vec::new(),
            veil_thickness: 0.7,     // Default veil thickness
            free_will_capacity: 0.5, // Default free will
            previous_choices: Vec::new(),
            environmental_influences: Vec::new(),
            natural_laws,
            probability_space,
            holographic_container,
            current_density: Density::Third, // Start at third density
            entity_id,
            seed,
        }
    }

    /// Create a decision engine with custom configuration
    pub fn with_config(
        entity_id: EntityID,
        seed: u64,
        archetype_activations: [Float; 22],
        veil_thickness: Float,
        free_will_capacity: Float,
        current_density: Density,
    ) -> Self {
        let holographic_container = HolographicContainer::new();
        let probability_space = ProbabilitySpace::with_outcomes(2, seed);
        let natural_laws = NaturalLaws::default();

        DecisionEngine {
            archetype_activations,
            physical_experiences: Vec::new(),
            veil_thickness: veil_thickness.clamp(0.0, 1.0),
            free_will_capacity: free_will_capacity.clamp(0.0, 1.0),
            previous_choices: Vec::new(),
            environmental_influences: Vec::new(),
            natural_laws,
            probability_space,
            holographic_container,
            current_density,
            entity_id,
            seed,
        }
    }

    /// Create a decision engine from seed with archetype diversity
    ///
    /// This creates a decision engine with archetype activations initialized
    /// based on the seed, ensuring diversity between entities while maintaining
    /// deterministic behavior.
    pub fn from_seed(entity_id: EntityID, seed: u64) -> Self {
        // Initialize archetype activations with seed-based variations
        // Phase 1 Task 1.1: Ensure diversity in STO/STS tendencies
        let mut archetype_activations = [0.5; 22];

        // Calculate a polarity bias from the seed (-1.0 to 1.0)
        // This determines if the entity tends toward STO or STS
        let polarity_bias = ((seed as Float).sin() * 2.0).clamp(-1.0, 1.0);

        for i in 0..22 {
            // Use deterministic pseudo-random based on seed and archetype index
            let combined = (seed as Float) * ((i + 1) as Float);
            let variation = (combined.sin() * 0.5).clamp(-0.3, 0.3);

            // Apply polarity bias to specific archetypes
            // STO archetypes: A7 (Great Way), A8 (Matrix), A9 (Potentiator), A10 (Catalyst)
            // STS archetypes: A11 (Experience), A12 (Significator), A13 (Transformation)
            let polarity_adjustment = match i {
                6 | 7 | 8 | 9 => polarity_bias * 0.3, // STO-biased archetypes
                10 | 11 | 12 => -polarity_bias * 0.3, // STS-biased archetypes
                _ => 0.0,
            };

            archetype_activations[i] = (0.5 + variation + polarity_adjustment).clamp(0.0, 1.0);
        }

        // Initialize holographic container
        let holographic_container = HolographicContainer::new();

        // Initialize probability space with 2 outcomes (STO and STS)
        let probability_space = ProbabilitySpace::with_outcomes(2, seed);

        // Initialize natural laws with default values
        let natural_laws = NaturalLaws::default();

        DecisionEngine {
            archetype_activations,
            physical_experiences: Vec::new(),
            veil_thickness: 0.7,     // Default veil thickness
            free_will_capacity: 0.5, // Default free will
            previous_choices: Vec::new(),
            environmental_influences: Vec::new(),
            natural_laws,
            probability_space,
            holographic_container,
            current_density: Density::Third, // Start at third density
            entity_id,
            seed,
        }
    }

    /// Make a choice based on all 8 factors
    pub fn make_choice(&mut self) -> Choice {
        // Calculate STO and STS weights
        let sto_weight = self.calculate_sto_weight();
        let sts_weight = self.calculate_sts_weight();

        // Normalize weights
        let total = sto_weight + sts_weight;
        let normalized_sto = if total > 0.0 { sto_weight / total } else { 0.5 };
        let normalized_sts = if total > 0.0 { sts_weight / total } else { 0.5 };

        // Phase 1 Task 1.1: Reset probability space before each choice
        // This ensures each choice is independent and can be different
        self.probability_space.reset();

        // Phase 1 Task 1.1: Update seed for each choice to ensure diversity
        // Use a mixing function to ensure seeds are well-distributed
        let choice_index = self.previous_choices.len() as u64;
        let current_seed = Self::mix_seed(self.seed, choice_index);
        self.probability_space.seed = current_seed;

        // Set probability amplitudes based on weights
        self.probability_space.amplitudes[0] = Complex::from_polar(normalized_sto.sqrt(), 0.0);
        self.probability_space.amplitudes[1] = Complex::from_polar(normalized_sts.sqrt(), 0.0);
        self.probability_space.normalize();

        // Collapse probability into choice
        let outcome = self.probability_space.collapse(None);
        let choice = if outcome.id == 0 {
            Choice::STO
        } else {
            Choice::STS
        };

        // Record the choice
        self.previous_choices.push(choice);

        // Update holographic container based on choice
        self.update_holographic_container(choice);

        choice
    }

    /// Calculate STO weight based on all factors
    pub fn calculate_sto_weight(&self) -> Float {
        // STO drivers: Love, abundance, cooperation, service, wisdom
        let love = self.love_score();
        let abundance = self.abundance_score();
        let cooperation = self.cooperation_score();
        let service = self.service_score();
        let wisdom = self.wisdom_score();

        // Weighted combination
        let base_weight =
            love * 0.3 + abundance * 0.2 + cooperation * 0.2 + service * 0.2 + wisdom * 0.1;

        // Adjust by free will capacity
        let free_will_adjustment = base_weight * self.free_will_capacity;

        // Adjust by veil thickness (thicker veil = more uncertainty)
        let veil_adjustment = free_will_adjustment * (1.0 - self.veil_thickness * 0.3);

        veil_adjustment.clamp(0.0, 1.0)
    }

    /// Calculate STS weight based on all factors
    pub fn calculate_sts_weight(&self) -> Float {
        // STS drivers: Fear, scarcity, competition, power, self-preservation
        let fear = self.fear_score();
        let scarcity = self.scarcity_score();
        let competition = self.competition_score();
        let power = self.power_score();
        let self_preservation = self.self_preservation_score();

        // Weighted combination
        let base_weight =
            fear * 0.3 + scarcity * 0.2 + competition * 0.2 + power * 0.2 + self_preservation * 0.1;

        // Adjust by free will capacity
        let free_will_adjustment = base_weight * self.free_will_capacity;

        // Adjust by veil thickness (thicker veil = more uncertainty)
        let veil_adjustment = free_will_adjustment * (1.0 - self.veil_thickness * 0.3);

        veil_adjustment.clamp(0.0, 1.0)
    }

    /// Calculate love score (STO driver)
    fn love_score(&self) -> Float {
        // Love comes from archetype A7 (Great Way)
        let archetype_love = self.archetype_activations[6]; // A7 Great Way

        // Also influenced by positive physical experiences
        let experience_love: Float = self
            .physical_experiences
            .iter()
            .map(|exp| exp.pleasure_pain.max(0.0) * 0.5 + exp.success_failure.max(0.0) * 0.3)
            .sum::<Float>()
            .min(1.0);

        // Also influenced by environmental factors
        let environmental_love: Float = self
            .environmental_influences
            .iter()
            .map(|inf| inf.sto_influence())
            .sum::<Float>()
            .min(1.0);

        // Combine all factors
        let combined = archetype_love * 0.5 + experience_love * 0.3 + environmental_love * 0.2;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate abundance score (STO driver)
    fn abundance_score(&self) -> Float {
        // Abundance from positive experiences
        let experience_abundance: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| exp.experience_type == PhysicalExperienceType::Abundance)
            .map(|exp| exp.success_failure.max(0.0))
            .sum::<Float>()
            .min(1.0);

        // Abundance from environment
        let environmental_abundance: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Abundance)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = experience_abundance * 0.6 + environmental_abundance * 0.4;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate cooperation score (STO driver)
    fn cooperation_score(&self) -> Float {
        // Cooperation from experiences
        let experience_cooperation: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| exp.experience_type == PhysicalExperienceType::Cooperation)
            .map(|exp| exp.success_failure.max(0.0))
            .sum::<Float>()
            .min(1.0);

        // Cooperation from environment
        let environmental_cooperation: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Cooperation)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Community also contributes
        let environmental_community: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Community)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = experience_cooperation * 0.5
            + environmental_cooperation * 0.3
            + environmental_community * 0.2;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate service score (STO driver)
    fn service_score(&self) -> Float {
        // Service comes from helping others (positive social experiences)
        let service_experiences: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| matches!(exp.experience_type, PhysicalExperienceType::Cooperation))
            .map(|exp| exp.pleasure_pain.max(0.0))
            .sum::<Float>()
            .min(1.0);

        // Service from environment (community)
        let environmental_service: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Community)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = service_experiences * 0.7 + environmental_service * 0.3;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate wisdom score (STO driver)
    fn wisdom_score(&self) -> Float {
        // Wisdom comes from archetype A5 (Transformation) and A7 (Great Way)
        let archetype_wisdom =
            (self.archetype_activations[4] + self.archetype_activations[6]) / 2.0;

        // Wisdom from diverse experiences (both positive and negative)
        let experience_diversity = if !self.physical_experiences.is_empty() {
            let positive_count = self
                .physical_experiences
                .iter()
                .filter(|exp| exp.pleasure_pain > 0.5)
                .count();
            let negative_count = self
                .physical_experiences
                .iter()
                .filter(|exp| exp.pleasure_pain < -0.5)
                .count();

            // Balance of positive and negative experiences
            if positive_count > 0 && negative_count > 0 {
                1.0 - (positive_count as Float - negative_count as Float).abs()
                    / (positive_count as Float + negative_count as Float)
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Combine
        let combined = archetype_wisdom * 0.6 + experience_diversity * 0.4;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate fear score (STS driver)
    fn fear_score(&self) -> Float {
        // Fear from experiences
        let experience_fear: Float = self
            .physical_experiences
            .iter()
            .map(|exp| exp.fear_relief.max(0.0))
            .sum::<Float>()
            .min(1.0);

        // Fear from environment
        let environmental_fear: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Danger)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = experience_fear * 0.6 + environmental_fear * 0.4;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate scarcity score (STS driver)
    fn scarcity_score(&self) -> Float {
        // Scarcity from experiences
        let experience_scarcity: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| exp.experience_type == PhysicalExperienceType::Scarcity)
            .map(|exp| exp.success_failure.abs())
            .sum::<Float>()
            .min(1.0);

        // Scarcity from environment
        let environmental_scarcity: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Scarcity)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = experience_scarcity * 0.6 + environmental_scarcity * 0.4;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate competition score (STS driver)
    fn competition_score(&self) -> Float {
        // Competition from experiences
        let experience_competition: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| {
                exp.experience_type == PhysicalExperienceType::Competition
                    && exp.success_failure > 0.0
            })
            .map(|exp| exp.success_failure)
            .sum::<Float>()
            .min(1.0);

        // Competition from environment
        let environmental_competition: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Competition)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Chaos also contributes
        let environmental_chaos: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Chaos)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = experience_competition * 0.5
            + environmental_competition * 0.3
            + environmental_chaos * 0.2;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate power score (STS driver)
    fn power_score(&self) -> Float {
        // Power comes from archetype A6 (Transformation)
        let archetype_power = self.archetype_activations[5]; // A6 Transformation

        // Power from successful competition
        let competition_power: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| {
                exp.experience_type == PhysicalExperienceType::Competition
                    && exp.success_failure > 0.0
            })
            .map(|exp| exp.success_failure)
            .sum::<Float>()
            .min(1.0);

        // Power from environmental dominance
        let environmental_power: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| {
                matches!(
                    inf.influence_type,
                    EnvironmentalInfluenceType::Competition | EnvironmentalInfluenceType::Chaos
                )
            })
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = archetype_power * 0.4 + competition_power * 0.3 + environmental_power * 0.3;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate self-preservation score (STS driver)
    fn self_preservation_score(&self) -> Float {
        // Self-preservation from fear of death, pain, etc.
        let survival_fear: Float = self
            .physical_experiences
            .iter()
            .filter(|exp| {
                matches!(
                    exp.experience_type,
                    PhysicalExperienceType::Death | PhysicalExperienceType::Pain
                )
            })
            .map(|exp| exp.fear_relief.max(0.0))
            .sum::<Float>()
            .min(1.0);

        // Self-preservation from environmental danger
        let environmental_danger: Float = self
            .environmental_influences
            .iter()
            .filter(|inf| inf.influence_type == EnvironmentalInfluenceType::Danger)
            .map(|inf| inf.intensity)
            .sum::<Float>()
            .min(1.0);

        // Combine
        let combined = survival_fear * 0.6 + environmental_danger * 0.4;

        combined.clamp(0.0, 1.0)
    }

    /// Explain why a choice was made
    pub fn explain_choice(&self, choice: Choice) -> DecisionExplanation {
        let sto_weight = self.calculate_sto_weight();
        let sts_weight = self.calculate_sts_weight();
        let contributing_factors = self.get_contributing_factors();

        DecisionExplanation {
            sto_weight,
            sts_weight,
            contributing_factors,
            final_choice: choice,
            decision_timestamp: 0.0, // Would need timestamp from PhysicalExperience
        }
    }

    /// Get all contributing factors for the current decision
    fn get_contributing_factors(&self) -> Vec<ContributingFactor> {
        let mut factors = Vec::new();

        // STO factors
        factors.push(ContributingFactor {
            factor_name: "Love".to_string(),
            influence: self.love_score(),
            direction: InfluenceDirection::STO,
            weight: 0.3,
        });

        factors.push(ContributingFactor {
            factor_name: "Abundance".to_string(),
            influence: self.abundance_score(),
            direction: InfluenceDirection::STO,
            weight: 0.2,
        });

        factors.push(ContributingFactor {
            factor_name: "Cooperation".to_string(),
            influence: self.cooperation_score(),
            direction: InfluenceDirection::STO,
            weight: 0.2,
        });

        factors.push(ContributingFactor {
            factor_name: "Service".to_string(),
            influence: self.service_score(),
            direction: InfluenceDirection::STO,
            weight: 0.2,
        });

        factors.push(ContributingFactor {
            factor_name: "Wisdom".to_string(),
            influence: self.wisdom_score(),
            direction: InfluenceDirection::STO,
            weight: 0.1,
        });

        // STS factors
        factors.push(ContributingFactor {
            factor_name: "Fear".to_string(),
            influence: self.fear_score(),
            direction: InfluenceDirection::STS,
            weight: 0.3,
        });

        factors.push(ContributingFactor {
            factor_name: "Scarcity".to_string(),
            influence: self.scarcity_score(),
            direction: InfluenceDirection::STS,
            weight: 0.2,
        });

        factors.push(ContributingFactor {
            factor_name: "Competition".to_string(),
            influence: self.competition_score(),
            direction: InfluenceDirection::STS,
            weight: 0.2,
        });

        factors.push(ContributingFactor {
            factor_name: "Power".to_string(),
            influence: self.power_score(),
            direction: InfluenceDirection::STS,
            weight: 0.2,
        });

        factors.push(ContributingFactor {
            factor_name: "Self-Preservation".to_string(),
            influence: self.self_preservation_score(),
            direction: InfluenceDirection::STS,
            weight: 0.1,
        });

        // Modifying factors
        factors.push(ContributingFactor {
            factor_name: "Free Will Capacity".to_string(),
            influence: self.free_will_capacity,
            direction: InfluenceDirection::Neutral,
            weight: 0.5,
        });

        factors.push(ContributingFactor {
            factor_name: "Veil Thickness".to_string(),
            influence: self.veil_thickness,
            direction: InfluenceDirection::Neutral,
            weight: 0.3,
        });

        factors
    }

    /// Add a physical experience to the engine
    pub fn add_physical_experience(&mut self, experience: PhysicalExperience) {
        // Update holographic container with the new experience
        self.update_holographic_with_experience(&experience);

        // Add to experiences list
        self.physical_experiences.push(experience);
    }

    /// Add an environmental influence to the engine
    pub fn add_environmental_influence(&mut self, influence: EnvironmentalInfluence) {
        self.environmental_influences.push(influence);
    }

    /// Update holographic container based on a choice
    fn update_holographic_container(&mut self, choice: Choice) {
        // Get the current state
        let state_id =
            StateID::from_linear_index((self.current_density.as_u8() - 1) as usize).unwrap();

        // Create a state change (using ChangeType::ArchetypeInfluence for polarization)
        let change = StateChange {
            change_type: ChangeType::ArchetypeInfluence,
            magnitude: if choice == Choice::STO { 0.1 } else { -0.1 },
            affected_attributes: vec!["polarization".to_string()],
        };

        // Propagate the change holographically
        self.holographic_container
            .propagate_change(state_id, change);
    }

    /// Update holographic container with a new experience
    fn update_holographic_with_experience(&mut self, experience: &PhysicalExperience) {
        // Get the current state
        let state_id =
            StateID::from_linear_index((self.current_density.as_u8() - 1) as usize).unwrap();

        // Create a state change based on experience
        let magnitude = experience.pleasure_pain * 0.5 + experience.success_failure * 0.3
            - experience.fear_relief * 0.2;
        let change = StateChange {
            change_type: ChangeType::Experience,
            magnitude,
            affected_attributes: vec!["experience".to_string()],
        };

        // Propagate the change holographically
        self.holographic_container
            .propagate_change(state_id, change);
    }

    /// Get the polarization history
    pub fn get_polarization_history(&self) -> (Float, Float) {
        let sto_count = self
            .previous_choices
            .iter()
            .filter(|&&c| c == Choice::STO)
            .count() as Float;

        let sts_count = self
            .previous_choices
            .iter()
            .filter(|&&c| c == Choice::STS)
            .count() as Float;

        let total = (sto_count + sts_count).max(1.0);
        (sto_count / total, sts_count / total)
    }

    /// Get the choice counts
    ///
    /// Returns the actual count of STO and STS choices made.
    pub fn get_choice_counts(&self) -> (usize, usize) {
        let sto_count = self
            .previous_choices
            .iter()
            .filter(|&&c| c == Choice::STO)
            .count();

        let sts_count = self
            .previous_choices
            .iter()
            .filter(|&&c| c == Choice::STS)
            .count();

        (sto_count, sts_count)
    }

    /// Adjust free will capacity
    pub fn adjust_free_will_capacity(&mut self, delta: Float) {
        self.free_will_capacity = (self.free_will_capacity + delta).clamp(0.0, 1.0);
    }

    /// Adjust veil thickness
    pub fn adjust_veil_thickness(&mut self, delta: Float) {
        self.veil_thickness = (self.veil_thickness + delta).clamp(0.0, 1.0);
    }

    /// Clear old experiences (keep only recent ones)
    pub fn clear_old_experiences(&mut self, keep_count: usize) {
        if self.physical_experiences.len() > keep_count {
            let start = self.physical_experiences.len() - keep_count;
            self.physical_experiences = self.physical_experiences[start..].to_vec();
        }
    }

    /// Clear old environmental influences
    pub fn clear_old_influences(&mut self, keep_count: usize) {
        if self.environmental_influences.len() > keep_count {
            let start = self.environmental_influences.len() - keep_count;
            self.environmental_influences = self.environmental_influences[start..].to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_engine_creation() {
        let engine = DecisionEngine::new(1, 42);
        assert_eq!(engine.entity_id, 1);
        assert_eq!(engine.seed, 42);
        assert_eq!(engine.archetype_activations.len(), 22);
    }

    #[test]
    fn test_make_choice() {
        let mut engine = DecisionEngine::new(1, 42);

        // Make a choice
        let choice = engine.make_choice();

        // Choice should be either STO or STS
        assert!(choice == Choice::STO || choice == Choice::STS);

        // Should be recorded in previous choices
        assert_eq!(engine.previous_choices.len(), 1);
        assert_eq!(engine.previous_choices[0], choice);
    }

    #[test]
    fn test_explain_choice() {
        let mut engine = DecisionEngine::new(1, 42);

        // Add some experiences
        engine.add_physical_experience(PhysicalExperience {
            experience_type: PhysicalExperienceType::Pleasure,
            pleasure_pain: 0.9,
            success_failure: 0.8,
            fear_relief: -0.5,
            entity_id: "entity_1".to_string(),
        });

        // Make a choice
        let choice = engine.make_choice();

        // Get explanation
        let explanation = engine.explain_choice(choice);

        assert_eq!(explanation.final_choice, choice);
        assert!(!explanation.contributing_factors.is_empty());
        assert!(explanation.sto_weight >= 0.0 && explanation.sto_weight <= 1.0);
        assert!(explanation.sts_weight >= 0.0 && explanation.sts_weight <= 1.0);
    }

    #[test]
    fn test_both_sto_and_sts_possible() {
        // Test STO choices
        let mut engine_sto = DecisionEngine::new(1, 42);
        engine_sto.archetype_activations[6] = 1.0; // High love
        engine_sto.add_physical_experience(PhysicalExperience {
            experience_type: PhysicalExperienceType::Pleasure,
            pleasure_pain: 0.9,
            success_failure: 0.8,
            fear_relief: -0.5,
            entity_id: "entity_1".to_string(),
        });

        // Test STS choices
        let mut engine_sts = DecisionEngine::new(1, 43);
        engine_sts.add_physical_experience(PhysicalExperience {
            experience_type: PhysicalExperienceType::Pain,
            pleasure_pain: -0.9,
            success_failure: -0.5,
            fear_relief: 0.8,
            entity_id: "entity_1".to_string(),
        });
        engine_sts.add_environmental_influence(EnvironmentalInfluence {
            influence_type: EnvironmentalInfluenceType::Danger,
            intensity: 0.9,
            duration: 10.0,
            timestamp: 0.0,
        });

        // Both should be able to make choices
        let choice_sto = engine_sto.make_choice();
        let choice_sts = engine_sts.make_choice();

        assert!(choice_sto == Choice::STO || choice_sto == Choice::STS);
        assert!(choice_sts == Choice::STO || choice_sts == Choice::STS);
    }
}
