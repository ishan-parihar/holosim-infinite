// A5: The Significator of Mind
// Documentation reference: 01_Metaphysics/archetypes/A5_Significator.md

use crate::archetypes::archetype_traits::SignificatorArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// A5: The Significator of Mind - Identity Agent / Choosing Entity
///
/// The Significator is the heart of the mind complex — the dynamic entity which
/// absorbs, seeks, and attempts to learn. It represents the choosing entity,
/// the 'I' that makes decisions and polarizes.
///
/// Core Functions:
/// - Identity agent: The choosing entity at the heart of mind complex
/// - Harvest of biases: Biases from all previous incarnational experiences
/// - Will to know: Possessed of free will and, more especially, will
/// - Both actor and acted upon: Dynamic interaction with experience
#[derive(Debug, Clone)]
pub struct SignificatorMindArchetype {
    /// Archetype ID (A5)
    pub archetype_id: u8,

    /// Lambda measurement - Significator coherence and choice-making capacity
    pub lambda: LambdaMeasurement,

    /// Tarot correlation
    pub tarot_correlation: TarotCorrelation,

    /// Identity coherence - how clear is sense of self? (0.0 to 1.0)
    pub identity_coherence: Float,

    /// Choice clarity - how clear is choice-making process? (0.0 to 1.0)
    pub choice_clarity: Float,

    /// Will power - strength of will (0.0 to 1.0)
    pub will_power: Float,

    /// Knowledge application - how well is knowledge applied? (0.0 to 1.0)
    pub knowledge_application: Float,

    /// Bias harvest - biases from previous incarnations (7 rungs)
    pub bias_harvest: Vec<Float>,

    /// Veil influence - extent of veil influence (0.0 to 1.0)
    pub veil_influence: Float,

    /// Actor ratio - ratio of acting to being acted upon (0.0 to 1.0)
    pub actor_ratio: Float,

    /// Covenant strength - strength of covenant with spirit (0.0 to 1.0)
    pub covenant_strength: Float,

    /// Look direction - -1.0 (left) to 1.0 (right)
    pub look_direction: Float,

    /// Hand gesture - -1.0 (absorb) to 1.0 (offer)
    pub hand_gesture: Float,

    /// Polarity tendency - -1.0 (STS) to 1.0 (STO)
    pub polarization_tendency: Float,

    /// Developmental position in Sigma Network
    pub developmental_position: DevelopmentalPosition,

    /// Activated rungs (R1-R7)
    pub activated_rungs: Vec<Rung>,

    /// Activation levels for each rung (0.0 to 1.0)
    pub activation_levels: HashMap<Rung, Float>,

    /// Description
    pub description: String,

    /// Holonic level
    pub holonic_level: HolonicLevel,

    /// Integration capacity (0.0 to 1.0)
    pub integration_capacity: Float,

    /// Pair balance with Great Way (0.0 to 1.0)
    pub pair_balance: Float,

    /// Will strength - strength of will for making choices
    pub will_strength: Float,
}

impl SignificatorMindArchetype {
    /// Create a new Significator archetype with default values
    pub fn new() -> Self {
        // Initialize with optimal lambda value (middle of healthy range)
        let mut lambda = LambdaMeasurement::new(0.65, LambdaMeasurementType::SignificatorCoherence);
        lambda.healthy_min = 0.5;
        lambda.healthy_max = 0.8;

        let tarot_correlation = TarotCorrelation::new(format!(
            "The Hierophant (V): The choosing entity at the heart of the mind complex"
        ));

        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.5);
        activation_levels.insert(Rung::R2, 0.6);
        activation_levels.insert(Rung::R3, 0.75); // Yellow - rational identity
        activation_levels.insert(Rung::R4, 0.7);
        activation_levels.insert(Rung::R5, 0.65);
        activation_levels.insert(Rung::R6, 0.6);
        activation_levels.insert(Rung::R7, 0.5);

        SignificatorMindArchetype {
            archetype_id: 5,
            lambda,
            tarot_correlation,
            identity_coherence: 0.85, // Increased to ensure healthy lambda
            choice_clarity: 0.85,     // Increased to ensure healthy lambda
            will_power: 0.85,         // Increased to ensure healthy lambda
            knowledge_application: 0.85, // Increased to ensure healthy lambda
            bias_harvest: vec![0.0; 7], // 7 rungs of bias harvest
            veil_influence: 0.5,
            actor_ratio: 0.5, // Balanced actor and acted upon
            covenant_strength: 0.75,
            look_direction: -0.3, // Slight left bias (tendency to notice negative)
            hand_gesture: 0.0,    // Neutral initially
            polarization_tendency: 0.0, // Neutral initially
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O5, 3),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3],
            activation_levels,
            description: "The Significator of Mind - Identity Agent / Choosing Entity".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.75,
            pair_balance: 0.5,
            will_strength: 0.85, // Will strength for making choices
        }
    }

    /// Calculate Significator coherence
    ///
    /// Λ_Significator = Significator Coherence × Choice-Making Capacity
    pub fn calculate_coherence(&self) -> Float {
        let identity_factor = self.identity_coherence;
        let choice_factor = self.choice_clarity;
        let will_factor = self.will_power;
        let learning_factor = self.knowledge_application;

        // Coherence is weighted average of all factors
        (identity_factor * 0.3)
            + (choice_factor * 0.3)
            + (will_factor * 0.2)
            + (learning_factor * 0.2)
    }

    /// Calculate choice-making capacity
    ///
    /// Choice-Making Capacity = f(Identity Coherence, Will Power, Knowledge Application)
    pub fn calculate_choice_capacity(&self) -> Float {
        let clarity = self.choice_clarity;
        let will = self.will_power;
        let knowledge = self.knowledge_application;

        // Choice capacity requires clarity, will, and knowledge
        (clarity * 0.4) + (will * 0.4) + (knowledge * 0.2)
    }

    /// Process catalyst and create unique experience
    ///
    /// Experience_unique = Catalyst × Significator Application
    pub fn process_catalyst(&mut self, catalyst_intensity: Float) -> Float {
        // Unique experience depends on Significator's application
        let application = (self.calculate_choice_capacity() * self.calculate_coherence()) / 2.0;
        let unique_experience = catalyst_intensity * application;

        // Record experience in bias harvest
        self.record_experience(unique_experience);

        unique_experience
    }

    /// Record experience in bias harvest
    pub fn record_experience(&mut self, experience: Float) {
        // Experience contributes to bias harvest across rungs
        for i in 0..self.bias_harvest.len() {
            let contribution = experience * (1.0 - (i as Float * 0.1)); // Higher rungs more affected
            self.bias_harvest[i] = (self.bias_harvest[i] + contribution).min(1.0);
        }
    }

    /// Retrieve bias from harvest for new experience
    pub fn retrieve_bias(&self, rung: usize) -> Float {
        if rung < self.bias_harvest.len() {
            self.bias_harvest[rung]
        } else {
            0.0
        }
    }

    /// Calculate actor vs. acted upon balance
    ///
    /// Actor ratio = Acting / (Acting + Being Acted Upon)
    pub fn calculate_actor_ratio(&self) -> Float {
        // Significator is both actor and acted upon
        // Balanced ratio is around 0.5
        self.actor_ratio
    }

    /// Update actor ratio based on experience
    pub fn update_actor_ratio(&mut self, acting: Float, acted_upon: Float) {
        let total = acting + acted_upon;
        if total > 0.0 {
            self.actor_ratio = acting / total;
        }
    }

    /// Calculate will to know
    ///
    /// Will to Know = Curiosity × Intention × Purpose
    pub fn calculate_will_to_know(&self) -> Float {
        let curiosity = self.identity_coherence; // Curiosity tied to identity
        let intention = self.choice_clarity; // Intention tied to choice
        let purpose = self.will_power; // Purpose tied to will

        curiosity * intention * purpose
    }

    /// Apply knowledge with purpose
    ///
    /// Knowledge application = f(Knowledge, Will, Purpose)
    pub fn apply_knowledge(&mut self, knowledge: Float, purpose: Float) -> Float {
        let application = knowledge * self.will_power * purpose;
        self.knowledge_application = (self.knowledge_application + application).min(1.0);
        application
    }

    /// Calculate polarization tendency
    ///
    /// Polarity = f(Hand Gesture, Look Direction, Covenant Strength)
    pub fn calculate_polarization(&self) -> Float {
        let gesture = self.hand_gesture;
        let look = self.look_direction;
        let covenant = self.covenant_strength;

        // Polarity based on gesture, look direction, and covenant
        (gesture * 0.5) + (look * 0.3) + (covenant * 0.2)
    }

    /// Make a choice (STO or STS)
    ///
    /// Choice = f(Polarity Tendency, Will Power, Knowledge)
    pub fn make_choice(&mut self) -> Polarity {
        let polarization = self.calculate_polarization();

        // Update hand gesture based on choice
        if polarization > 0.0 {
            self.hand_gesture = (self.hand_gesture + 0.1).min(1.0); // STO: offering light outward
            Polarity::STO
        } else if polarization < 0.0 {
            self.hand_gesture = (self.hand_gesture - 0.1).max(-1.0); // STS: absorbing power for self
            Polarity::STS
        } else {
            Polarity::SinkholeOfIndifference
        }
    }

    /// Check covenant with spirit
    ///
    /// Covenant = Significator ↔ Spirit
    pub fn check_covenant(&self) -> Float {
        self.covenant_strength
    }

    /// Strengthen covenant with spirit
    pub fn strengthen_covenant(&mut self, amount: Float) {
        self.covenant_strength = (self.covenant_strength + amount).min(1.0);
    }

    /// Calculate alignment score with Great Way
    ///
    /// Alignment Score = (Significator Choice × Great Way Perception) / ||Great Way||
    pub fn calculate_alignment_score(&self, great_way_perception: Float) -> Float {
        let choice = self.calculate_choice_capacity();
        let alignment = (choice * great_way_perception) / (great_way_perception + 0.001);
        alignment
    }

    /// Update lambda value
    pub fn update_lambda(&mut self) {
        self.lambda.value = self.calculate_coherence() * self.calculate_choice_capacity();
    }

    /// Get health diagnosis
    pub fn get_health_diagnosis(&self) -> String {
        let lambda = self.lambda.value;

        if lambda < self.lambda.healthy_min {
            format!(
                "IDENTITY CONFUSION: Lambda = {:.2} (< {:.2}). \
                Significator coherence too low. Identity confusion, paralysis, lack of clarity. \
                Need to strengthen sense of self and choice-making capacity.",
                lambda, self.lambda.healthy_min
            )
        } else if lambda > self.lambda.healthy_max {
            format!(
                "OVER-RIGID IDENTIFICATION: Lambda = {:.2} (> {:.2}). \
                Significator coherence too high. Over-rigid identification, inflexibility. \
                Need to allow for identity evolution and flexibility.",
                lambda, self.lambda.healthy_max
            )
        } else {
            format!(
                "HEALTHY SIGNIFICATOR: Lambda = {:.2} (range: {:.2} - {:.2}). \
                Coherent identity with clear choice-making capacity.",
                lambda, self.lambda.healthy_min, self.lambda.healthy_max
            )
        }
    }

    // ============================================================================
    // PHASE 8: GREATER CYCLE REGULATION
    // ============================================================================

    /// Make choice based on observed Lesser Cycle state
    /// Phase 8: Extended method that considers Lesser Cycle state
    pub fn make_choice_with_state(
        &mut self,
        lesser_cycle_state: &crate::complex::LesserCycleState,
    ) -> crate::complex::Choice {
        // Update will power based on experience depth
        self.will_power = (self.will_power + lesser_cycle_state.experience_output * 0.01).min(1.0);

        // Calculate polarization direction based on microcosmic tension
        let choice_type = if lesser_cycle_state.microcosmic_tension > 0.5 {
            crate::complex::ChoiceType::ServiceToOthers // STO tendency
        } else if lesser_cycle_state.microcosmic_tension < 0.3 {
            crate::complex::ChoiceType::ServiceToSelf // STS tendency
        } else {
            crate::complex::ChoiceType::Neutral // Neutral
        };

        // Calculate choice intensity based on will power and choice clarity
        let intensity = self.will_power * self.choice_clarity;

        crate::complex::Choice {
            choice_type,
            intensity,
            timestamp: 0,
            chosen_archetype: None,
            chosen_intensity: intensity,
        }
    }
}

// Additional methods for archetype processing

impl SignificatorMindArchetype {
    /// Set will strength
    pub fn set_will_strength(&mut self, strength: Float) {
        self.will_power = strength.clamp(0.0, 1.0);
    }
}

// Implement ArchetypeTrait
impl ArchetypeTrait for SignificatorMindArchetype {
    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Significator of Mind"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Mind
    }

    fn activate(&mut self, intensity: Float) {
        self.lambda.adjust(intensity);
    }

    fn deactivate(&mut self) {
        self.lambda.adjust(-0.1);
    }

    fn is_active(&self) -> bool {
        self.lambda.value > 0.0
    }

    fn role(&self) -> ArchetypeRole {
        ArchetypeRole::Significator
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.max(0.0).min(1.0);
        self.identity_coherence = self.lambda.value;
    }

    fn health_status(&self) -> HealthStatus {
        self.lambda.health_status()
    }

    fn is_healthy(&self) -> bool {
        self.lambda.is_healthy()
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaA
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::IdentityPair
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        self.update_lambda();
    }

    fn description(&self) -> &str {
        &self.description
    }
}

// Implement LambdaMeasurable
impl LambdaMeasurable for SignificatorMindArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        let coherence = self.calculate_coherence();
        let choice_capacity = self.calculate_choice_capacity();

        // Lambda = Significator Coherence × Choice-Making Capacity
        coherence * choice_capacity
    }

    fn healthy_range(&self) -> (Float, Float) {
        (self.lambda.healthy_min, self.lambda.healthy_max)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.is_pathological_low() {
            indicators.push("Identity confusion".to_string());
            indicators.push("Choice paralysis".to_string());
            indicators.push("Lack of coherence".to_string());
        }

        if self.lambda.is_pathological_high() {
            indicators.push("Over-rigid identification".to_string());
            indicators.push("Inflexibility".to_string());
        }

        indicators
    }
}

// Implement Developmental
impl Developmental for SignificatorMindArchetype {
    fn develop(&mut self, catalyst: Float) {
        self.lambda.value += catalyst * 0.1;
        if self.lambda.value > 1.0 {
            self.lambda.value = 1.0;
        }
    }

    fn regress(&mut self, resistance: Float) {
        self.lambda.value -= resistance * 0.1;
        if self.lambda.value < 0.0 {
            self.lambda.value = 0.0;
        }
    }

    fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        *self.activation_levels.get(&rung).unwrap_or(&0.0)
    }
}

// Implement Paired
impl Paired for SignificatorMindArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(7) // A7: The Great Way of Mind
    }

    fn get_pair(&self) -> Option<u8> {
        Some(7)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        // Identity Pair tension: |Significator coherence - Great Way alignment|
        // This is a placeholder - actual calculation requires Great Way reference
        let choice_strength = self.calculate_choice_capacity();
        let perception_alignment = self.covenant_strength;
        (choice_strength - perception_alignment).abs()
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        // Balance = 1.0 - |tension - 0.5|
        let tension = self.calculate_pair_tension(paired_archetype);
        1.0 - (tension - 0.5).abs()
    }
}

// Implement Holonic
impl Holonic for SignificatorMindArchetype {
    fn get_holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn set_holonic_level(&mut self, level: HolonicLevel) {
        self.holonic_level = level;
    }

    fn holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn integration_capacity(&self) -> Float {
        self.integration_capacity
    }

    fn transcend(&mut self) -> bool {
        // Transcend: Identity becomes more unified
        let transcend_factor = self.identity_coherence * self.covenant_strength;

        if transcend_factor > 0.8 {
            // Move to next holonic level
            match self.holonic_level {
                HolonicLevel::Micro => {
                    self.holonic_level = HolonicLevel::Meso;
                    true
                }
                HolonicLevel::Meso => {
                    self.holonic_level = HolonicLevel::Macro;
                    true
                }
                HolonicLevel::Macro => {
                    self.holonic_level = HolonicLevel::Meta;
                    true
                }
                HolonicLevel::Meta => false, // Already at highest level
            }
        } else {
            false
        }
    }

    fn include(&mut self, _lower_level: &dyn Holonic) -> bool {
        // Include: Identity incorporates all experiences
        let bias_sum: Float = self.bias_harvest.iter().sum();
        let include_factor = bias_sum / self.bias_harvest.len() as Float;

        if include_factor > 0.7 {
            self.integration_capacity = (self.integration_capacity + include_factor).min(1.0);
            true
        } else {
            false
        }
    }
}

// ============================================================================
// SIGNIFICATOR ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl SignificatorArchetypeTrait for SignificatorMindArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_identity_coherence(&self) -> Float {
        self.identity_coherence
    }

    fn get_choice_clarity(&self) -> Float {
        self.choice_clarity
    }

    fn get_will_power(&self) -> Float {
        self.will_power
    }

    fn get_polarization_tendency(&self) -> Float {
        // Calculate polarization tendency from actor_ratio and veil_influence
        // Higher actor_ratio and lower veil_influence → stronger polarization tendency
        (self.actor_ratio * (1.0 - self.veil_influence)).clamp(0.0, 1.0)
    }

    // Core setters
    fn set_identity_coherence(&mut self, value: Float) {
        self.identity_coherence = value.clamp(0.0, 1.0);
    }

    fn set_choice_clarity(&mut self, value: Float) {
        self.choice_clarity = value.clamp(0.0, 1.0);
    }

    fn set_will_power(&mut self, value: Float) {
        self.will_power = value.clamp(0.0, 1.0);
    }

    fn set_polarization_tendency(&mut self, value: Float) {
        // Map polarization tendency to actor_ratio
        self.actor_ratio = value.clamp(0.0, 1.0);
    }

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position.clone()
    }

    fn get_activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn get_activation_level(&self, rung: Rung) -> Float {
        self.activation_levels.get(&rung).copied().unwrap_or(0.0)
    }

    fn set_activation_level(&mut self, rung: Rung, level: Float) {
        self.activation_levels.insert(rung, level.clamp(0.0, 1.0));
    }

    // Choice mechanism
    fn make_choice_with_state(
        &mut self,
        state: &crate::complex::LesserCycleState,
    ) -> crate::complex::Choice {
        SignificatorMindArchetype::make_choice_with_state(self, state)
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        // Map common HealthStatus to complex HealthStatus
        let common_health = SignificatorMindArchetype::health_status(self);
        match common_health {
            HealthStatus::Healthy => HealthStatus::Healthy,
            HealthStatus::PathologicalLow => HealthStatus::Degraded,
            HealthStatus::PathologicalHigh => HealthStatus::Pathological,
            _ => HealthStatus::Warning,
        }
    }

    // Flow management methods (used by complex.rs)
    fn clear_flows(&mut self) {
        // Significator doesn't manage flows directly - no-op
    }

    fn add_flow(&mut self, _rung: Rung, _flow: Float) {
        // Significator doesn't manage flows directly - no-op
    }

    fn get_flow(&self, _rung: Rung) -> Float {
        // Significator doesn't manage flows directly - return 0
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_significator_creation() {
        let significator = SignificatorMindArchetype::new();

        assert_eq!(significator.archetype_id, 5);
        assert_eq!(significator.archetype_id(), 5);
        assert_eq!(significator.name(), "The Significator of Mind");
        assert_eq!(significator.complex(), ArchetypeComplex::Mind);
        assert_eq!(significator.role(), ArchetypeRole::Significator);
    }

    #[test]
    fn test_significator_lambda_measurement() {
        let significator = SignificatorMindArchetype::new();

        // Default values should be in healthy range
        let lambda = significator.calculate_lambda();
        assert!(lambda >= significator.healthy_range().0);
        assert!(lambda <= significator.healthy_range().1);
        assert!(significator.is_healthy());

        // Test pathological low
        let mut low_significator = significator.clone();
        low_significator.identity_coherence = 0.3;
        low_significator.choice_clarity = 0.3;
        low_significator.update_lambda();
        assert!(!low_significator.is_healthy());

        // Test pathological high
        let mut high_significator = significator.clone();
        high_significator.identity_coherence = 1.0;
        high_significator.choice_clarity = 1.0;
        high_significator.will_power = 1.0;
        high_significator.knowledge_application = 1.0;
        high_significator.update_lambda();
        assert!(!high_significator.is_healthy());
    }

    #[test]
    fn test_catalyst_processing() {
        let mut significator = SignificatorMindArchetype::new();

        // Process catalyst
        let catalyst_intensity = 0.8;
        let unique_experience = significator.process_catalyst(catalyst_intensity);

        assert!(unique_experience > 0.0);
        assert!(unique_experience <= catalyst_intensity);

        // Check bias harvest updated
        let bias_sum: Float = significator.bias_harvest.iter().sum();
        assert!(bias_sum > 0.0);
    }

    #[test]
    fn test_choice_making() {
        let mut significator = SignificatorMindArchetype::new();

        // Test STO choice
        significator.hand_gesture = 0.5;
        significator.covenant_strength = 0.8;
        let choice = significator.make_choice();
        assert_eq!(choice, Polarity::STO);

        // Test STS choice
        significator.hand_gesture = -0.5;
        significator.covenant_strength = 0.3;
        let choice = significator.make_choice();
        assert_eq!(choice, Polarity::STS);
    }

    #[test]
    fn test_functional_pair_dynamics() {
        let significator = SignificatorMindArchetype::new();

        // Test Identity Pair
        assert_eq!(significator.functional_pair(), FunctionalPair::IdentityPair);
        assert_eq!(significator.paired_archetype_id(), Some(7));

        // Create mock Great Way for testing
        struct MockGreatWay {
            lambda: LambdaMeasurement,
            tarot: TarotCorrelation,
        }
        impl MockGreatWay {
            fn new() -> Self {
                Self {
                    lambda: LambdaMeasurement::new(0.6, LambdaMeasurementType::GreatWayClarity),
                    tarot: TarotCorrelation::new(format!("{} - {}", "The Chariot", "VII")),
                }
            }
        }
        impl ArchetypeTrait for MockGreatWay {
            fn archetype_id(&self) -> u8 {
                7
            }
            fn name(&self) -> &str {
                "Mock Great Way"
            }
            fn complex(&self) -> ArchetypeComplex {
                ArchetypeComplex::Mind
            }
            fn role(&self) -> ArchetypeRole {
                ArchetypeRole::GreatWay
            }
            fn lambda(&self) -> &LambdaMeasurement {
                &self.lambda
            }
            fn update_lambda(&mut self, _value: Float) {}
            fn tarot_correlation(&self) -> TarotCorrelation {
                self.tarot.clone()
            }
            fn sigma_axis(&self) -> SigmaAxis {
                SigmaAxis::SigmaA
            }
            fn functional_pair(&self) -> FunctionalPair {
                FunctionalPair::IdentityPair
            }
            fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}
            fn description(&self) -> &str {
                "Mock great way for testing"
            }
            fn activate(&mut self, _intensity: Float) {}
            fn deactivate(&mut self) {}
            fn is_active(&self) -> bool {
                true
            }
            fn is_healthy(&self) -> bool {
                self.lambda.health_status() == HealthStatus::Healthy
            }
            fn health_status(&self) -> HealthStatus {
                self.lambda.health_status()
            }
        }
        impl Paired for MockGreatWay {
            fn get_pair(&self) -> Option<u8> {
                None
            }
            fn paired_archetype_id(&self) -> Option<u8> {
                Some(5)
            }
            fn lambda(&self) -> &LambdaMeasurement {
                &self.lambda
            }
            fn calculate_pair_tension(&self, _paired: &dyn Paired) -> Float {
                0.3
            }
            fn calculate_pair_balance(&self, _paired: &dyn Paired) -> Float {
                0.7
            }
        }

        let mock_great_way = MockGreatWay::new();
        let tension = significator.calculate_pair_tension(&mock_great_way);
        let balance = significator.calculate_pair_balance(&mock_great_way);
        assert!(tension >= 0.0 && tension <= 1.0);
        assert!(balance >= 0.0 && balance <= 1.0);
    }

    #[test]
    fn test_developmental_progression() {
        let mut significator = SignificatorMindArchetype::new();

        // Test initial rung
        assert_eq!(
            significator.developmental_position(),
            DevelopmentalPosition::Significator
        );

        // Test activation levels
        let activation = significator.rung_activation_level(Rung::R3);
        assert!(activation > 0.0);

        // Test activated rungs
        let rungs = significator.activated_rungs();
        assert!(rungs.contains(&Rung::R1));
        assert!(rungs.contains(&Rung::R2));
        assert!(rungs.contains(&Rung::R3));
    }

    #[test]
    fn test_holonic_integration() {
        let significator = SignificatorMindArchetype::new();

        // Test initial holonic level
        assert_eq!(significator.holonic_level(), HolonicLevel::Meso);

        // Test integration capacity
        let capacity = significator.integration_capacity();
        assert!(capacity >= 0.0 && capacity <= 1.0);
    }

    #[test]
    fn test_will_to_know() {
        let significator = SignificatorMindArchetype::new();

        let will_to_know = significator.calculate_will_to_know();
        assert!(will_to_know >= 0.0 && will_to_know <= 1.0);

        // Test that will to know increases with coherence
        let mut higher_significator = significator.clone();
        higher_significator.identity_coherence = 0.9;
        higher_significator.choice_clarity = 0.9;
        higher_significator.will_power = 0.9;

        assert!(higher_significator.calculate_will_to_know() > will_to_know);
    }

    #[test]
    fn test_actor_ratio() {
        let mut significator = SignificatorMindArchetype::new();

        // Test initial balanced ratio
        assert!((significator.actor_ratio - 0.5).abs() < 0.1);

        // Test update
        significator.update_actor_ratio(0.7, 0.3);
        assert!((significator.actor_ratio - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_covenant_with_spirit() {
        let mut significator = SignificatorMindArchetype::new();

        // Test initial covenant
        assert!(significator.check_covenant() > 0.5);

        // Test strengthen covenant
        significator.strengthen_covenant(0.2);
        assert!(significator.check_covenant() > 0.9);

        // Test max covenant
        significator.strengthen_covenant(1.0);
        assert_eq!(significator.check_covenant(), 1.0);
    }

    #[test]
    fn test_health_diagnosis() {
        let significator = SignificatorMindArchetype::new();

        // Test healthy diagnosis
        let diagnosis = significator.get_health_diagnosis();
        assert!(diagnosis.contains("HEALTHY SIGNIFICATOR"));

        // Test pathological low diagnosis
        let mut low_significator = significator.clone();
        low_significator.identity_coherence = 0.3;
        low_significator.choice_clarity = 0.3;
        low_significator.update_lambda();
        let diagnosis = low_significator.get_health_diagnosis();
        assert!(diagnosis.contains("IDENTITY CONFUSION"));

        // Test pathological high diagnosis
        let mut high_significator = significator.clone();
        high_significator.identity_coherence = 1.0;
        high_significator.choice_clarity = 1.0;
        high_significator.will_power = 1.0;
        high_significator.knowledge_application = 1.0;
        high_significator.update_lambda();
        let diagnosis = high_significator.get_health_diagnosis();
        assert!(diagnosis.contains("OVER-RIGID IDENTIFICATION"));
    }
}
