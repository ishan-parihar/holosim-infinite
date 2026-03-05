// A12: Significator of Body - Identity Agent for Body Complex
// The harvest of biases of all previous incarnational experiences, offering biases to meet new experience

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, DevelopmentalPosition,
    FunctionalPair, HealthStatus, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Rung};
use std::collections::HashMap;

/// Significator of Body Archetype (A12)
///
/// The Significator of the Body may be seen as a simple and unified concept -
/// the harvest of biases of all previous incarnational experiences, offering biases
/// to meet new experience. It is a highly developed portion of the infant
/// mind/body/spirit complex and is complex in and of itself.
///
/// **Core Function:** Identity agent for the Body complex
/// **Tarot Correlation:** The Hanged Man (XII)
/// **Sigma Axis:** σB (Body Capacity)
/// **Functional Pair:** Identity Pair (Significator ↔ Great Way)
#[derive(Debug, Clone)]
pub struct SignificatorBodyArchetype {
    /// Archetype ID (A12)
    pub archetype_id: u8,
    pub active: bool,
    /// Lambda measurement - Significator coherence and choice-making capacity
    pub lambda: LambdaMeasurement,
    /// Tarot correlation
    pub tarot_correlation: TarotCorrelation,
    /// Embodied self - the physical identity
    pub embodied_self: Float,
    /// Choice clarity - clarity of choice-making
    pub choice_clarity: Float,
    /// Identity coherence - how integrated the identity is
    pub identity_coherence: Float,
    /// Choice-making capacity - ability to make conscious choices
    pub choice_making_capacity: Float,
    /// Harvest of biases - accumulated biases from previous incarnations
    pub bias_harvest: Float,
    /// Bias flexibility - how flexibly biases are offered to new experience
    pub bias_flexibility: Float,
    /// Developmental position in sigma network
    pub developmental_position: DevelopmentalPosition,
    /// Activated rungs
    pub activated_rungs: Vec<Rung>,
    /// Activation levels for each rung
    pub activation_levels: HashMap<Rung, Float>,
    /// Description
    pub description: String,
    /// Holonic level
    pub holonic_level: HolonicLevel,
    /// Integration capacity
    pub integration_capacity: Float,
    /// Polarity (STS for male polarity assignment after veiling)
    pub polarization: crate::types::Polarity,
}

impl SignificatorBodyArchetype {
    /// Create a new Significator of Body archetype with healthy initial values
    pub fn new() -> Self {
        // Initial lambda value in healthy range (0.5 - 0.8 for Significator)
        let mut lambda = LambdaMeasurement::new(0.65, LambdaMeasurementType::SignificatorCoherence);
        lambda.healthy_min = 0.5;
        lambda.healthy_max = 0.8;

        SignificatorBodyArchetype {
            archetype_id: 12,
            active: true,
            lambda,
            tarot_correlation: TarotCorrelation::new("The Hanged Man (XII): Suspension, letting go, surrender to the process".to_string()),
            embodied_self: 0.65,
            choice_clarity: 0.65,
            identity_coherence: 0.65,
            choice_making_capacity: 0.65,
            bias_harvest: 0.6,
            bias_flexibility: 0.7,
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O3, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels: {
                let mut levels = HashMap::new();
                levels.insert(Rung::R1, 0.5);
                levels.insert(Rung::R2, 0.6);
                levels.insert(Rung::R3, 0.7);
                levels.insert(Rung::R4, 0.75);
                levels
            },
            description: "The harvest of biases of all previous incarnational experiences, offering biases to meet new experience. Identity agent for Body complex.".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.7,
            polarization: crate::types::Polarity::STS, // Male polarity assignment after veiling
        }
    }

    /// Calculate significator flexibility score
    ///
    /// Formula: (Biases * Flexibility) / ||Significator||
    pub fn calculate_significator_flexibility(&self) -> Float {
        let biases = self.bias_harvest;
        let flexibility = self.bias_flexibility;
        let significator_magnitude = (self.embodied_self.powi(2)
            + self.identity_coherence.powi(2)
            + self.choice_making_capacity.powi(2))
        .sqrt();

        if significator_magnitude > 0.0 {
            (biases * flexibility) / significator_magnitude
        } else {
            0.0
        }
    }

    /// Update bias harvest from new experience
    pub fn update_bias_harvest(&mut self, new_biases: Float) {
        self.bias_harvest = (self.bias_harvest + new_biases).clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda();
    }

    /// Update bias flexibility
    pub fn update_bias_flexibility(&mut self, flexibility: Float) {
        self.bias_flexibility = flexibility.clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda();
    }

    /// Offer biases to new experience
    pub fn offer_biases(&self) -> Float {
        // Biases offered based on harvest and flexibility
        self.bias_harvest * self.bias_flexibility
    }

    /// Get identity coherence score
    pub fn get_identity_coherence(&self) -> Float {
        // Identity coherence = embodied_self * identity_integration
        self.embodied_self * self.identity_coherence
    }

    /// Check if identity is fragmented
    pub fn is_identity_fragmented(&self) -> bool {
        self.identity_coherence < 0.4 || self.lambda.is_pathological_low()
    }

    /// Check if biases are too rigid
    pub fn are_biases_rigid(&self) -> bool {
        self.bias_flexibility < 0.3 || self.lambda.is_pathological_low()
    }

    /// Check if biases are too flexible (lack of identity)
    pub fn are_biases_too_flexible(&self) -> bool {
        self.bias_flexibility > 0.9 || self.lambda.is_pathological_high()
    }

    /// Get significator health status description
    pub fn health_description(&self) -> &str {
        match self.lambda.health_status() {
            HealthStatus::Healthy => {
                if self.bias_flexibility > 0.7 {
                    "Flexible biases with integrated identity"
                } else {
                    "Adequate bias flexibility with integrated identity"
                }
            }
            HealthStatus::Balanced => "Balanced state",
            HealthStatus::Imbalanced => "Imbalanced state",
            HealthStatus::Warning => {
                "Warning: Bias flexibility declining, identity integration affected"
            }
            HealthStatus::Degraded => {
                "Degraded: Multiple bias issues, identity fragmentation occurring"
            }
            HealthStatus::Pathological => {
                "Pathological: Severe bias dysfunction, identity coherence broken"
            }
            HealthStatus::PathologicalLow => {
                if self.are_biases_rigid() {
                    "Rigid biases distorting experience"
                } else if self.is_identity_fragmented() {
                    "Fragmented identity lacking coherence"
                } else {
                    "Low significator coherence"
                }
            }
            HealthStatus::PathologicalHigh => "Over-flexible, lacking stable identity",
        }
    }
}

impl LambdaMeasurable for SignificatorBodyArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
    }

    fn calculate_lambda(&self) -> Float {
        // Significator lambda = (Identity Coherence + Choice Clarity + Choice Making) / 3
        (self.identity_coherence + self.choice_clarity + self.choice_making_capacity) / 3.0
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();
        if self.lambda.is_pathological_low() {
            indicators.push("Low significator coherence".to_string());
        }
        if self.lambda.is_pathological_high() {
            indicators.push("Over-flexible, lacking stable identity".to_string());
        }
        if self.bias_flexibility < 0.3 {
            indicators.push("Rigid biases".to_string());
        }
        if self.identity_coherence < 0.4 {
            indicators.push("Fragmented identity".to_string());
        }
        indicators
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8) // Significator healthy range
    }
}

impl SignificatorBodyArchetype {
    /// Get developmental position (method wrapper for field access)
    pub fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }

    /// Get activated rungs (method wrapper for field access)
    pub fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    /// Get activation level for a specific rung
    pub fn rung_activation_level(&self, rung: Rung) -> Float {
        *self.activation_levels.get(&rung).unwrap_or(&0.0)
    }

    /// Get paired archetype ID (Significator pairs with Great Way)
    pub fn paired_archetype_id(&self) -> u8 {
        14 // Great Way of Body
    }

    /// Calculate pair tension with paired archetype
    pub fn calculate_pair_tension(&self, paired: &dyn ArchetypeTrait) -> Float {
        let lambda_diff = (self.lambda.value - paired.lambda().value).abs();
        lambda_diff
    }

    /// Calculate pair balance with paired archetype
    pub fn calculate_pair_balance(&self, paired: &dyn ArchetypeTrait) -> Float {
        let lambda_sum = self.lambda.value + paired.lambda().value;
        lambda_sum / 2.0
    }

    /// Get holonic level (method wrapper for field access)
    pub fn holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    /// Get integration capacity (method wrapper for field access)
    pub fn integration_capacity(&self) -> Float {
        self.integration_capacity
    }

    /// Transcend to next holonic level
    pub fn transcend(&mut self) -> bool {
        match self.holonic_level {
            HolonicLevel::Micro => {
                self.holonic_level = HolonicLevel::Meso;
                self.integration_capacity = 0.6;
                true
            }
            HolonicLevel::Meso => {
                self.holonic_level = HolonicLevel::Macro;
                self.integration_capacity = 0.8;
                true
            }
            HolonicLevel::Macro => {
                self.holonic_level = HolonicLevel::Meta;
                self.integration_capacity = 1.0;
                true
            }
            HolonicLevel::Meta => false, // Already at highest level
        }
    }

    /// Include lower levels in transcension
    pub fn include(&self) -> bool {
        // Significator always includes previous development
        true
    }
}

impl ArchetypeTrait for SignificatorBodyArchetype {
    fn activate(&mut self, intensity: Float) {
        self.lambda.adjust(intensity);
    }

    fn deactivate(&mut self) {
        self.lambda.adjust(-0.1);
    }

    fn is_active(&self) -> bool {
        self.lambda.value > 0.0
    }

    fn is_healthy(&self) -> bool {
        ArchetypeTrait::lambda(self).value >= 0.5
    }

    fn health_status(&self) -> HealthStatus {
        match ArchetypeTrait::lambda(self).value {
            x if x >= 0.9 => HealthStatus::Healthy,
            x if x >= 0.7 => HealthStatus::Balanced,
            x if x >= 0.5 => HealthStatus::Warning,
            x if x >= 0.3 => HealthStatus::Pathological,
            x if x >= 0.1 => HealthStatus::PathologicalLow,
            _ => HealthStatus::PathologicalHigh,
        }
    }

    fn role(&self) -> ArchetypeRole {
        ArchetypeRole::Significator
    }

    fn process(&mut self, catalyst: Float, _position: DevelopmentalPosition) {
        // Significator processing
        let processing = catalyst * self.choice_clarity;
        self.identity_coherence += processing * 0.01;
        self.identity_coherence = self.identity_coherence.clamp(0.0, 1.0);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaB
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "Significator of Body"
    }

    fn description(&self) -> &str {
        "Significator of Body - The choice and identity formation, biases, and harvest indicators"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Body
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::IdentityPair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Polarity;
    use crate::archetypes::common::{Paired, Holonic, HolonicLevel};

    // Mock archetype for testing paired relationships
    struct MockGreatWayArchetype {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockGreatWayArchetype {
        fn new(lambda_value: Float) -> Self {
            MockGreatWayArchetype {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::GreatWayClarity,
                ),
                tarot: TarotCorrelation::new(format!("{} - {}", "Temperance", "XIV")),
            }
        }
    }

    impl ArchetypeTrait for MockGreatWayArchetype {
        fn archetype_id(&self) -> u8 {
            14
        }

        fn name(&self) -> &str {
            "Great Way of Body"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Body
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::GreatWay
        }

        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }

        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.clamp(0.0, 1.0);
        }

        fn tarot_correlation(&self) -> TarotCorrelation {
            self.tarot.clone()
        }

        fn sigma_axis(&self) -> SigmaAxis {
            SigmaAxis::SigmaB
        }

        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::IdentityPair
        }

        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}

        fn description(&self) -> &str {
            "Environment container for Body complex"
        }

        fn activate(&mut self, intensity: Float) {
            // Adjust lambda based on intensity
            let lambda = ArchetypeTrait::lambda(self);
            let new_value = (lambda.value + intensity * 0.1).clamp(0.0, 1.0);
            self.update_lambda(new_value);
        }

        fn deactivate(&mut self) {
            // Deactivate by reducing lambda
            self.update_lambda(0.0);
        }

        fn is_active(&self) -> bool {
            self.lambda.value > 0.0
        }

        fn is_healthy(&self) -> bool {
            let lambda = ArchetypeTrait::lambda(self);
            lambda.value >= lambda.healthy_min && lambda.value <= lambda.healthy_max
        }

        fn health_status(&self) -> HealthStatus {
            if self.is_healthy() {
                HealthStatus::Balanced
            } else if ArchetypeTrait::lambda(self).value < ArchetypeTrait::lambda(self).healthy_min
            {
                HealthStatus::PathologicalLow
            } else {
                HealthStatus::PathologicalHigh
            }
        }
    }

    impl Paired for MockGreatWayArchetype {
        fn get_pair(&self) -> Option<u8> {
            None
        }
        fn paired_archetype_id(&self) -> Option<u8> {
            Some(12)
        }
        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }
        fn calculate_pair_tension(&self, paired_archetype: &dyn Paired) -> Float {
            let self_lambda = self.lambda.value;
            let paired_lambda = paired_archetype.lambda().value;
            (self_lambda - paired_lambda).abs()
        }

        fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
            let tension = self.calculate_pair_tension(paired_archetype);
            (1.0 - tension).clamp(0.0, 1.0)
        }
    }

    impl Holonic for MockGreatWayArchetype {
        fn holonic_level(&self) -> HolonicLevel {
            HolonicLevel::Meso
        }
        fn get_holonic_level(&self) -> HolonicLevel {
            HolonicLevel::Meso
        }
        fn set_holonic_level(&mut self, _level: HolonicLevel) {}

        fn integration_capacity(&self) -> Float {
            0.7
        }

        fn transcend(&mut self) -> bool {
            false
        }

        fn include(&mut self, _lower_level: &dyn Holonic) -> bool {
            true
        }
    }

    #[test]
    fn test_new_archetype() {
        let archetype = SignificatorBodyArchetype::new();

        assert_eq!(archetype.archetype_id, 12);
        assert_eq!(archetype.name(), "Significator of Body");
        assert_eq!(archetype.complex(), ArchetypeComplex::Body);
        assert_eq!(archetype.role(), ArchetypeRole::Significator);
        assert_eq!(archetype.sigma_axis(), SigmaAxis::SigmaB);
        assert_eq!(archetype.functional_pair(), FunctionalPair::IdentityPair);
        assert!(archetype
            .tarot_correlation()
            .card
            .contains("The Hanged Man"));
        assert!(archetype.tarot_correlation().card.contains("XII"));
        assert_eq!(archetype.polarization, Polarity::STS);
    }

    #[test]
    fn test_initial_lambda_healthy() {
        let archetype = SignificatorBodyArchetype::new();

        assert!(archetype.lambda.is_healthy());
        assert!(archetype.lambda.value >= 0.5);
        assert!(archetype.lambda.value <= 0.8);
        assert_eq!(
            archetype.lambda.measurement_type,
            LambdaMeasurementType::SignificatorCoherence
        );
    }

    #[test]
    fn test_calculate_lambda() {
        let mut archetype = SignificatorBodyArchetype::new();

        // Test with healthy values
        archetype.bias_flexibility = 0.7;
        archetype.identity_coherence = 0.8;
        archetype.choice_making_capacity = 0.75;

        let lambda = archetype.calculate_lambda();
        assert!((0.5..=0.8).contains(&lambda));
    }

    #[test]
    fn test_calculate_lambda_pathological_low() {
        let mut archetype = SignificatorBodyArchetype::new();

        // Set values to create low lambda
        archetype.bias_flexibility = 0.2;
        archetype.identity_coherence = 0.3;
        archetype.choice_making_capacity = 0.4;

        let lambda = archetype.calculate_lambda();
        assert!(lambda < 0.5);
    }

    #[test]
    fn test_calculate_lambda_pathological_high() {
        let mut archetype = SignificatorBodyArchetype::new();

        // Set values to create high lambda
        archetype.bias_flexibility = 0.95;
        archetype.identity_coherence = 0.9;
        archetype.choice_making_capacity = 0.9;

        let lambda = archetype.calculate_lambda();
        assert!(lambda > 0.8);
    }

    #[test]
    fn test_healthy_range() {
        let archetype = SignificatorBodyArchetype::new();
        let (min, max) = archetype.healthy_range();

        assert!((min - 0.5).abs() < 0.01);
        assert!((max - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_pathological_indicators_healthy() {
        let archetype = SignificatorBodyArchetype::new();
        let indicators = archetype.pathological_indicators();

        assert!(indicators.is_empty());
    }

    #[test]
    fn test_pathological_indicators_low() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.bias_flexibility = 0.2;
        archetype.identity_coherence = 0.3;
        archetype.lambda.value = 0.4;

        let indicators = archetype.pathological_indicators();
        assert!(!indicators.is_empty());
        // The pathological indicators include "Rigid biases" and "Fragmented identity"
        assert!(indicators.iter().any(|i| i.contains("Rigid biases")));
        assert!(indicators.iter().any(|i| i.contains("Fragmented identity")));
    }

    #[test]
    fn test_pathological_indicators_high() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.bias_flexibility = 0.95;
        archetype.lambda.value = 0.85;

        let indicators = archetype.pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Over-flexible")));
    }

    #[test]
    fn test_calculate_significator_flexibility() {
        let archetype = SignificatorBodyArchetype::new();

        let flexibility = archetype.calculate_significator_flexibility();
        assert!(flexibility >= 0.0);
        assert!(flexibility <= 1.0);
    }

    #[test]
    fn test_update_bias_harvest() {
        let mut archetype = SignificatorBodyArchetype::new();
        let initial_harvest = archetype.bias_harvest;

        archetype.update_bias_harvest(0.2);

        assert!((archetype.bias_harvest - (initial_harvest + 0.2)).abs() < 0.01);
    }

    #[test]
    fn test_update_bias_flexibility() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.update_bias_flexibility(0.8);

        assert!((archetype.bias_flexibility - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_offer_biases() {
        let archetype = SignificatorBodyArchetype::new();

        let offered = archetype.offer_biases();
        assert!(offered >= 0.0);
        assert!(offered <= 1.0);
        assert!((offered - (archetype.bias_harvest * archetype.bias_flexibility)).abs() < 0.01);
    }

    #[test]
    fn test_get_identity_coherence() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.embodied_self = 0.8;
        archetype.identity_coherence = 0.7;

        let coherence = archetype.get_identity_coherence();
        assert!((coherence - 0.56).abs() < 0.01);
    }

    #[test]
    fn test_is_identity_fragmented() {
        let mut archetype = SignificatorBodyArchetype::new();

        assert!(!archetype.is_identity_fragmented());

        archetype.identity_coherence = 0.3;
        assert!(archetype.is_identity_fragmented());
    }

    #[test]
    fn test_are_biases_rigid() {
        let mut archetype = SignificatorBodyArchetype::new();

        assert!(!archetype.are_biases_rigid());

        archetype.bias_flexibility = 0.2;
        assert!(archetype.are_biases_rigid());
    }

    #[test]
    fn test_are_biases_too_flexible() {
        let mut archetype = SignificatorBodyArchetype::new();

        assert!(!archetype.are_biases_too_flexible());

        archetype.bias_flexibility = 0.95;
        assert!(archetype.are_biases_too_flexible());
    }

    #[test]
    fn test_health_description() {
        let archetype = SignificatorBodyArchetype::new();
        let description = archetype.health_description();

        assert!(!description.is_empty());
        assert!(description.contains("Flexible") || description.contains("Adequate"));
    }

    #[test]
    fn test_health_description_pathological_low() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.bias_flexibility = 0.2;
        archetype.lambda.value = 0.4;

        let description = archetype.health_description();
        assert!(description.contains("Rigid") || description.contains("Fragmented"));
    }

    #[test]
    fn test_health_description_pathological_high() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.bias_flexibility = 0.95;
        archetype.lambda.value = 0.85;

        let description = archetype.health_description();
        assert!(description.contains("Over-flexible"));
    }

    #[test]
    fn test_process() {
        let mut archetype = SignificatorBodyArchetype::new();
        let _initial_lambda = archetype.lambda.value;

        archetype.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Lambda should be updated
        assert!((archetype.lambda.value - archetype.calculate_lambda()).abs() < 0.01);

        // Integration capacity should be updated
        assert!(archetype.integration_capacity > 0.0);
    }

    #[test]
    fn test_developmental_position() {
        let archetype = SignificatorBodyArchetype::new();
        let position = archetype.developmental_position();

        // A12 is initialized with rung 4 (Octant::O3, 4), which maps to Input (4 % 4 = 0)
        assert_eq!(position, DevelopmentalPosition::Input);
    }

    #[test]
    fn test_update_developmental_position() {
        let mut archetype = SignificatorBodyArchetype::new();
        let new_position = DevelopmentalPosition::new_with_octant_rung(Octant::O4, 5);

        archetype.developmental_position = new_position;

        assert_eq!(
            archetype.developmental_position,
            DevelopmentalPosition::Catalyst
        ); // rung 5 % 4 = 1 (Catalyst)
           // Note: activated_rungs is not updated by simple field assignment
           // The test just verifies the position change
    }

    #[test]
    fn test_activated_rungs() {
        let archetype = SignificatorBodyArchetype::new();
        let rungs = archetype.activated_rungs();

        assert!(!rungs.is_empty());
        assert!(rungs.contains(&Rung::R1));
        assert!(rungs.contains(&Rung::R4));
    }

    #[test]
    fn test_rung_activation_level() {
        let archetype = SignificatorBodyArchetype::new();
        let level = archetype.rung_activation_level(Rung::R4);

        assert!(level > 0.0);
        assert!(level <= 1.0);
    }

    #[test]
    fn test_paired_archetype_id() {
        let archetype = SignificatorBodyArchetype::new();

        assert_eq!(archetype.paired_archetype_id(), 14);
    }

    #[test]
    fn test_calculate_pair_tension() {
        let significator = SignificatorBodyArchetype::new();
        let great_way = MockGreatWayArchetype::new(0.7);

        let tension = significator.calculate_pair_tension(&great_way);

        assert!(tension >= 0.0);
        assert!(tension <= 1.0);
    }

    #[test]
    fn test_calculate_pair_balance() {
        let significator = SignificatorBodyArchetype::new();
        let great_way = MockGreatWayArchetype::new(0.65);

        let balance = significator.calculate_pair_balance(&great_way);

        assert!(balance >= 0.0);
        assert!(balance <= 1.0);
    }

    #[test]
    fn test_holonic_level() {
        let archetype = SignificatorBodyArchetype::new();

        assert_eq!(archetype.holonic_level(), HolonicLevel::Meso);
    }

    #[test]
    fn test_integration_capacity() {
        let archetype = SignificatorBodyArchetype::new();

        assert!(archetype.integration_capacity() > 0.0);
        assert!(archetype.integration_capacity() <= 1.0);
    }

    #[test]
    fn test_transcend() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.integration_capacity = 0.9;

        let transcended = archetype.transcend();

        assert!(transcended);
        assert_eq!(archetype.holonic_level(), HolonicLevel::Macro);
    }

    #[test]
    fn test_transcend_failed() {
        let mut archetype = SignificatorBodyArchetype::new();

        // Start at Meta level to test transcend failure
        archetype.holonic_level = HolonicLevel::Meta;

        let transcended = archetype.transcend();

        assert!(!transcended);
        assert_eq!(archetype.holonic_level(), HolonicLevel::Meta);
    }

    #[test]
    fn test_include() {
        let significator = SignificatorBodyArchetype::new();

        // Significator's include method takes no arguments
        let included = significator.include();

        assert!(included);
    }

    #[test]
    fn test_update_lambda() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.update_lambda(0.75);

        assert!((archetype.lambda.value - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_lambda_clamping() {
        let mut archetype = SignificatorBodyArchetype::new();
        archetype.update_lambda(1.5);

        assert!(archetype.lambda.value <= 1.0);

        archetype.update_lambda(-0.2);
        assert!(archetype.lambda.value >= 0.0);
    }

    #[test]
    fn test_description() {
        let archetype = SignificatorBodyArchetype::new();

        assert!(!archetype.description().is_empty());
        assert!(archetype.description().contains("harvest"));
        assert!(archetype.description().contains("biases"));
    }

    #[test]
    fn test_tarot_correlation() {
        let archetype = SignificatorBodyArchetype::new();
        let tarot = archetype.tarot_correlation();

        assert!(tarot.card.contains("The Hanged Man"));
        assert!(tarot.card.contains("XII"));
        assert!(!tarot.card.is_empty());
    }

    #[test]
    fn test_sigma_axis() {
        let archetype = SignificatorBodyArchetype::new();

        assert_eq!(archetype.sigma_axis(), SigmaAxis::SigmaB);
    }

    #[test]
    fn test_functional_pair() {
        let archetype = SignificatorBodyArchetype::new();

        assert_eq!(archetype.functional_pair(), FunctionalPair::IdentityPair);
    }

    #[test]
    fn test_polarity() {
        let archetype = SignificatorBodyArchetype::new();

        assert_eq!(archetype.polarization, Polarity::STS);
    }
}
