// A22: The Choice - Unifying Archetype
// The fundamental choice of polarity - service-to-others or service-to-self
// Necessary for harvestability from third density

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// A22: The Choice - Unifying Archetype
///
/// The Choice is the twenty-second archetype that represents the fundamental choice
/// of polarity - service-to-others or service-to-self - which is necessary for
/// harvestability from third density. Only this archetype is relatively fixed and
/// single among all archetypes.
///
/// Key characteristics:
/// - Relatively fixed and single (unique among archetypes)
/// - Fundamental choice of polarity (STO or STS)
/// - Necessary for harvestability from third density
/// - Higher density foundation (higher densities work due to polarity gained)
/// - Unifying archetype (brings together all other archetypes)
/// - Faith aspect (walking into space without regard for what comes next)
/// - Sharpening effect on Significator, Transformation, and Great Way
#[derive(Debug, Clone)]
pub struct ChoiceArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // A22-specific fields
    pub polarization_strength: Float,
    pub faith: Float,
    pub harvestability: Float,
    pub unification: Float,
    pub service_to_others: Float,
    pub service_to_self: Float,
    pub polarization_choice: Float,

    // Developmental fields
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,

    // Description
    pub description: String,

    // Holonic fields
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,

    // The fundamental polarity choice (STO, STS, or SinkholeOfIndifference)
    pub polarity: Polarity,
}

impl Default for ChoiceArchetype {
    fn default() -> Self {
        Self::new()
    }
}

impl ChoiceArchetype {
    /// Create a new Choice archetype with healthy initial values
    pub fn new() -> Self {
        let lambda = LambdaMeasurement::new(0.65, LambdaMeasurementType::ChoicePolarity);

        let tarot = TarotCorrelation::new("The Fool (0 or XXII): Fundamental choice of polarity, faith, harvestability".to_string());

        let developmental_position = DevelopmentalPosition::new_with_octant_rung(Octant::O1, 6);

        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.2);
        activation_levels.insert(Rung::R2, 0.3);
        activation_levels.insert(Rung::R3, 0.4);
        activation_levels.insert(Rung::R4, 0.5);
        activation_levels.insert(Rung::R5, 0.7);
        activation_levels.insert(Rung::R6, 0.9);
        activation_levels.insert(Rung::R7, 1.0);

        let mut choice = ChoiceArchetype {
            archetype_id: 22,
            lambda,
            tarot_correlation: tarot,

            // A22-specific fields with healthy initial values
            polarization_strength: 0.65,
            faith: 0.7,
            harvestability: 0.6,
            unification: 0.7,
            service_to_others: 0.6,
            service_to_self: 0.2,
            polarization_choice: 0.65,

            // Developmental fields
            developmental_position,
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4, Rung::R5, Rung::R6],
            activation_levels,

            // Description
            description: "A22: The Choice - The fundamental choice of polarity (service-to-others or service-to-self) necessary for harvestability from third density. Only this archetype is relatively fixed and single among all archetypes.".to_string(),

            // Holonic fields
            holonic_level: HolonicLevel::Meta,
            integration_capacity: 0.7,

            // The fundamental polarity choice
            polarity: Polarity::STO,
        };

        // Recalculate lambda based on initial state
        choice.lambda.value = choice.calculate_lambda();

        choice
    }

    // ============================================================================
    // CALCULATION METHODS
    // ============================================================================

    /// Calculate lambda based on polarization strength, faith, and harvestability
    pub fn calculate_lambda_from_state(&self) -> Float {
        let polarization_component = self.polarization_strength * 0.4;
        let faith_component = self.faith * 0.3;
        let harvestability_component = self.harvestability * 0.2;
        let unification_component = self.unification * 0.1;

        polarization_component + faith_component + harvestability_component + unification_component
    }

    /// Calculate polarization strength from service orientation
    pub fn calculate_polarization_strength(&self) -> Float {
        let sto_strength = self.service_to_others;
        let sts_strength = self.service_to_self;

        // Polarity strength is the maximum of the two orientations
        sto_strength.max(sts_strength)
    }

    /// Calculate harvestability potential
    pub fn calculate_harvestability(&self) -> Float {
        let polarization = self.polarization_strength;
        let faith = self.faith;
        let choice = self.polarization_choice;

        // Harvestability requires strong polarization AND effective faith
        (polarization * 0.5) + (faith * 0.3) + (choice * 0.2)
    }

    /// Calculate unification of all archetypes
    pub fn calculate_unification(&self) -> Float {
        let polarization = self.polarization_strength;
        let faith = self.faith;
        let harvestability = self.harvestability;

        // Unification brings together all archetypes through polarization
        (polarization * 0.4) + (faith * 0.3) + (harvestability * 0.3)
    }

    /// Calculate net polarization (STO positive, STS negative)
    pub fn calculate_net_polarization(&self) -> Float {
        self.service_to_others - self.service_to_self
    }

    /// Determine polarity from service orientation
    pub fn determine_polarity(&self) -> Polarity {
        let net = self.calculate_net_polarization();
        let total = self.service_to_others + self.service_to_self;

        if total < 0.2 {
            Polarity::SinkholeOfIndifference
        } else if net > 0.0 {
            Polarity::STO
        } else {
            Polarity::STS
        }
    }

    /// Calculate sharpening effect on Significator, Transformation, Great Way
    pub fn calculate_sharpening_effect(&self) -> Float {
        let polarization = self.polarization_strength;
        let faith = self.faith;
        let unification = self.unification;

        // Sharpening effect requires proper understanding (unification)
        (polarization * 0.4) + (faith * 0.3) + (unification * 0.3)
    }

    // ============================================================================
    // UPDATE METHODS
    // ============================================================================

    /// Update polarization strength
    pub fn update_polarization_strength(&mut self, value: Float) {
        self.polarization_strength = value.clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda_from_state();
        self.polarity = self.determine_polarity();
    }

    /// Update faith
    pub fn update_faith(&mut self, value: Float) {
        self.faith = value.clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Update harvestability
    pub fn update_harvestability(&mut self, value: Float) {
        self.harvestability = value.clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Update unification
    pub fn update_unification(&mut self, value: Float) {
        self.unification = value.clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Update service-to-others orientation
    pub fn update_service_to_others(&mut self, value: Float) {
        self.service_to_others = value.clamp(0.0, 1.0);
        self.polarization_strength = self.calculate_polarization_strength();
        self.lambda.value = self.calculate_lambda_from_state();
        self.polarity = self.determine_polarity();
    }

    /// Update service-to-self orientation
    pub fn update_service_to_self(&mut self, value: Float) {
        self.service_to_self = value.clamp(0.0, 1.0);
        self.polarization_strength = self.calculate_polarization_strength();
        self.lambda.value = self.calculate_lambda_from_state();
        self.polarity = self.determine_polarity();
    }

    /// Update polarization choice
    pub fn update_polarization_choice(&mut self, value: Float) {
        self.polarization_choice = value.clamp(0.0, 1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Update all polarization metrics
    pub fn update_polarization_metrics(&mut self, sto: Float, sts: Float) {
        self.service_to_others = sto.clamp(0.0, 1.0);
        self.service_to_self = sts.clamp(0.0, 1.0);
        self.polarization_strength = self.calculate_polarization_strength();
        self.harvestability = self.calculate_harvestability();
        self.unification = self.calculate_unification();
        self.lambda.value = self.calculate_lambda_from_state();
        self.polarity = self.determine_polarity();
    }

    // ============================================================================
    // QUERY METHODS
    // ============================================================================

    /// Get current polarization strength
    pub fn polarization_strength(&self) -> Float {
        self.polarization_strength
    }

    /// Get current faith level
    pub fn faith(&self) -> Float {
        self.faith
    }

    /// Get current harvestability
    pub fn harvestability(&self) -> Float {
        self.harvestability
    }

    /// Get current unification level
    pub fn unification(&self) -> Float {
        self.unification
    }

    /// Get service-to-others orientation
    pub fn service_to_others(&self) -> Float {
        self.service_to_others
    }

    /// Get service-to-self orientation
    pub fn service_to_self(&self) -> Float {
        self.service_to_self
    }

    /// Get net polarization
    pub fn net_polarization(&self) -> Float {
        self.calculate_net_polarization()
    }

    /// Get sharpening effect
    pub fn sharpening_effect(&self) -> Float {
        self.calculate_sharpening_effect()
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

impl LambdaMeasurable for ChoiceArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        self.calculate_lambda_from_state()
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.value < 0.5 {
            indicators.push("No polarization".to_string());
            indicators.push("No faith".to_string());
            indicators.push("No harvestability".to_string());
            indicators.push("Poor understanding of choice".to_string());
            indicators.push("Cannot bring together archetypes".to_string());
        } else if self.lambda.value > 0.8 {
            indicators.push("Rigid polarization".to_string());
            indicators.push("Fanaticism".to_string());
            indicators.push("Loss of flexibility".to_string());
            indicators.push("Over-rigid identification".to_string());
        }

        indicators
    }
}

impl Developmental for ChoiceArchetype {
    fn develop(&mut self, catalyst: Float) {
        self.lambda.adjust(catalyst * 0.1);
    }

    fn regress(&mut self, resistance: Float) {
        self.lambda.adjust(-resistance * 0.1);
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        *self.activation_levels.get(&rung).unwrap_or(&0.0)
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }
}

impl Paired for ChoiceArchetype {
    fn get_pair(&self) -> Option<u8> {
        Some(5) // Paired with A5 Significator of Mind
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn paired_archetype_id(&self) -> Option<u8> {
        Some(5) // A22 is paired with A5 Significator of Mind
    }

    fn calculate_pair_tension(&self, paired: &dyn Paired) -> Float {
        let paired_lambda = paired.lambda().value;
        (self.lambda.value - paired_lambda).abs()
    }

    fn calculate_pair_balance(&self, paired: &dyn Paired) -> Float {
        let paired_lambda = paired.lambda().value;
        let sum = self.lambda.value + paired_lambda;

        if sum > 0.0 {
            1.0 - (self.lambda.value - paired_lambda).abs() / sum
        } else {
            0.0
        }
    }
}

impl Holonic for ChoiceArchetype {
    fn get_holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn set_holonic_level(&mut self, level: HolonicLevel) {
        self.holonic_level = level;
    }

    fn transcend(&mut self) -> bool {
        // Choice is already at Meta level (unifying archetype)
        // Transcend operation returns false as it's already at highest level
        false
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include all lower archetypes into unifying choice
        let lower_integration = lower_level.integration_capacity();

        // Increase integration capacity based on included level
        if lower_integration > 0.0 {
            self.integration_capacity =
                (self.integration_capacity + lower_integration * 0.1).min(1.0);
            true
        } else {
            false
        }
    }

    fn holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn integration_capacity(&self) -> Float {
        self.integration_capacity
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

impl ArchetypeTrait for ChoiceArchetype {
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
        ArchetypeRole::Choice
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        let position_value = position.rung_level() as Float / 7.0;
        self.lambda.adjust(catalyst * position_value);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaA
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
        "The Choice"
    }

    fn description(&self) -> &str {
        "Choice - The unifying archetype that brings together all other archetypes"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Unified
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::IdentityPair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn is_healthy(&self) -> bool {
        self.lambda.value >= 0.5
    }

    fn health_status(&self) -> HealthStatus {
        match self.lambda.value {
            x if x >= 0.9 => HealthStatus::Healthy,
            x if x >= 0.7 => HealthStatus::Balanced,
            x if x >= 0.5 => HealthStatus::Warning,
            x if x >= 0.3 => HealthStatus::Pathological,
            x if x >= 0.1 => HealthStatus::PathologicalLow,
            _ => HealthStatus::PathologicalHigh,
        }
    }
}
mod tests {
    use super::*;
    use crate::archetypes::common::{ArchetypeComplex, ArchetypeRole};

    // Mock archetype for testing paired relationships
    #[allow(dead_code)]
    struct MockArchetype {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    #[allow(dead_code)]
    impl MockArchetype {
        fn new(lambda_value: Float) -> Self {
            MockArchetype {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::SignificatorCoherence,
                ),
                tarot: TarotCorrelation::new(
                    "The Hierophant (V): Spiritual wisdom, teaching, tradition".to_string(),
                ),
            }
        }
    }

    impl ArchetypeTrait for MockArchetype {
        fn archetype_id(&self) -> u8 {
            5
        }

        fn name(&self) -> &str {
            "Mock Significator Mind"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Significator
        }

        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }

        fn is_healthy(&self) -> bool {
            self.lambda.is_healthy()
        }

        fn health_status(&self) -> HealthStatus {
            self.lambda.health_status()
        }

        fn activate(&mut self, _intensity: Float) {
            self.lambda.adjust(0.1);
        }

        fn deactivate(&mut self) {
            self.lambda.adjust(-0.1);
        }

        fn is_active(&self) -> bool {
            self.lambda.value > 0.0
        }

        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
            // No-op for mock
        }

        fn sigma_axis(&self) -> SigmaAxis {
            SigmaAxis::SigmaC
        }

        fn tarot_correlation(&self) -> TarotCorrelation {
            self.tarot.clone()
        }

        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.clamp(0.0, 1.0);
        }

        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::ProcessPair
        }
    }
}
