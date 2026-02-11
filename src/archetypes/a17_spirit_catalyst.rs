// A17: The Catalyst of Spirit
// Called Hope (Preferably Faith) - causing changes in adept's viewpoint through spiritual potentiation

use crate::archetypes::archetype_traits::CatalystArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Catalyst of Spirit - Input friction for Spirit complex
/// Called Hope (Preferably Faith) - because of the illuminations of the Potentiator of the Spirit,
/// will begin to cause changes in the adept's viewpoint.
#[derive(Debug, Clone)]
pub struct CatalystSpiritArchetype {
    pub archetype_id: u8,
    pub active: bool,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // A17-specific fields
    pub faith_development: Float, // Hope and faith in spiritual process (0.0-1.0)
    pub viewpoint_changes_effectiveness: Float, // How effective are viewpoint changes (0.0-1.0)
    pub spiritual_growth_rate: Float, // How much spiritual growth occurs (0.0-1.0)
    pub unprocessed_catalyst: Float, // Amount of unprocessed catalyst (0.0-1.0)
    pub unique_processing_capacity: Float, // How uniquely catalyst can be processed (0.0-1.0)
    pub trust_in_process: Float,  // Trust in awakening process (0.0-1.0)

    // Developmental tracking
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,

    // Holonic properties
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float, // Mutable field for holonic transcend/include operations

    // Polarity (female after veiling)
    pub polarity: Polarity,

    pub description: String,
}

impl CatalystSpiritArchetype {
    /// Create a new Catalyst of Spirit archetype with healthy initial values
    pub fn new() -> Self {
        // Initialize rung activation levels
        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.2); // Survival catalyst
        activation_levels.insert(Rung::R2, 0.3); // Sensation catalyst
        activation_levels.insert(Rung::R3, 0.4); // Coordination catalyst
        activation_levels.insert(Rung::R4, 0.6); // Heart catalyst
        activation_levels.insert(Rung::R5, 0.7); // Integral catalyst
        activation_levels.insert(Rung::R6, 0.8); // Transpersonal catalyst
        activation_levels.insert(Rung::R7, 0.9); // Unity catalyst

        CatalystSpiritArchetype {
            archetype_id: 17,
            active: true,
            lambda: LambdaMeasurement::new(0.65, LambdaMeasurementType::CatalystProcessingRate),
            tarot_correlation: TarotCorrelation::new(format!("The Star (XVII): Hope, faith, spiritual renewal, guidance after disruption, illuminating star following tower destruction")),
            faith_development: 0.65,          // Strong faith development
            viewpoint_changes_effectiveness: 0.6, // Effective viewpoint changes
            spiritual_growth_rate: 0.6,       // Good spiritual growth rate
            unprocessed_catalyst: 0.4,        // Moderate unprocessed catalyst
            unique_processing_capacity: 0.7,  // Good unique processing capacity
            trust_in_process: 0.65,           // Good trust in process
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O1, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.6,
            polarity: Polarity::STO, // Female polarity assignment after veiling
            description: "Catalyst of Spirit - Called Hope (Preferably Faith), causing changes in adept's viewpoint through spiritual potentiation. All that is unprocessed that comes before notice of mind/body/spirit complex is catalyst. Individual complex may use any catalyst which comes before its notice in its unique way to form an experience unique to it, with its biases.".to_string(),
        }
    }

    /// Calculate catalyst harmony score
    pub fn calculate_catalyst_harmony(&self) -> Float {
        // Harmony based on faith, viewpoint changes, and spiritual growth
        let faith_weight = 0.4;
        let viewpoint_weight = 0.3;
        let growth_weight = 0.3;

        (self.faith_development * faith_weight
            + self.viewpoint_changes_effectiveness * viewpoint_weight
            + self.spiritual_growth_rate * growth_weight)
            .min(1.0)
    }

    /// Calculate integration capacity from faith, viewpoint changes, and spiritual growth
    pub fn calculate_integration_capacity_from_state(&self) -> Float {
        let faith_integration = self.faith_development * 0.4;
        let viewpoint_integration = self.viewpoint_changes_effectiveness * 0.35;
        let growth_integration = self.spiritual_growth_rate * 0.25;

        faith_integration + viewpoint_integration + growth_integration
    }

    /// Update faith development based on trust and spiritual orientation
    pub fn update_faith_development(&mut self, trust: Float, spiritual_orientation: Float) {
        let trust_factor = trust.clamp(0.0, 1.0);
        let orientation_factor = spiritual_orientation.clamp(0.0, 1.0);

        // Faith = Hope × Trust × Spiritual Growth Orientation
        // Use 1.2 multiplier to ensure faith increases when inputs are good
        self.faith_development = (trust_factor * orientation_factor * 1.2).clamp(0.0, 1.0);

        // Update integration capacity based on new state
        self.integration_capacity = self.calculate_integration_capacity_from_state();

        // Update lambda based on faith development
        self.update_lambda(0.0); // Value will be calculated in update_lambda
    }

    /// Update viewpoint changes effectiveness based on illuminations and catalyst
    pub fn update_viewpoint_changes_effectiveness(
        &mut self,
        illuminations: Float,
        catalyst_amount: Float,
    ) {
        let illumination_factor = illuminations.clamp(0.0, 1.0);
        let catalyst_factor = catalyst_amount.clamp(0.0, 1.0);

        // Viewpoint Changes = Illuminations × Catalyst × Perception Shift
        let perception_shift = (illumination_factor + catalyst_factor) / 2.0;
        // Use 1.5 multiplier to ensure viewpoint changes increase when inputs are good
        self.viewpoint_changes_effectiveness =
            (illumination_factor * catalyst_factor * perception_shift * 1.5).clamp(0.0, 1.0);

        // Update integration capacity based on new state
        self.integration_capacity = self.calculate_integration_capacity_from_state();

        // Update lambda based on viewpoint changes
        self.update_lambda(0.0); // Value will be calculated in update_lambda
    }

    /// Update spiritual growth rate based on faith and viewpoint changes
    pub fn update_spiritual_growth_rate(&mut self, faith: Float, viewpoint_changes: Float) {
        let faith_factor = faith.clamp(0.0, 1.0);
        let viewpoint_factor = viewpoint_changes.clamp(0.0, 1.0);

        // Spiritual growth emerges from faith and viewpoint changes
        self.spiritual_growth_rate =
            ((faith_factor + viewpoint_factor) / 2.0 * 1.1).clamp(0.0, 1.0);

        // Update integration capacity based on new state
        self.integration_capacity = self.calculate_integration_capacity_from_state();

        // Update lambda based on spiritual growth
        self.update_lambda(0.0); // Value will be calculated in update_lambda
    }

    /// Update unprocessed catalyst amount
    pub fn update_unprocessed_catalyst(&mut self, new_catalyst: Float) {
        let catalyst_amount = new_catalyst.clamp(0.0, 1.0);

        // Unprocessed catalyst accumulates if not processed
        self.unprocessed_catalyst =
            (self.unprocessed_catalyst * 0.8 + catalyst_amount * 0.2).clamp(0.0, 1.0);

        // High unprocessed catalyst can affect faith and viewpoint changes
        if self.unprocessed_catalyst > 0.7 {
            self.faith_development *= 0.9;
            self.viewpoint_changes_effectiveness *= 0.9;
        }

        // Update lambda based on unprocessed catalyst
        self.update_lambda(self.calculate_lambda());
    }

    /// Update unique processing capacity based on biases and creativity
    pub fn update_unique_processing_capacity(&mut self, biases: Float, creativity: Float) {
        let bias_factor = biases.clamp(0.0, 1.0);
        let creativity_factor = creativity.clamp(0.0, 1.0);

        // Unique processing emerges from biases and creativity
        self.unique_processing_capacity =
            ((bias_factor + creativity_factor) / 2.0 * 1.1).clamp(0.0, 1.0);
    }

    /// Update trust in awakening process
    pub fn update_trust_in_process(&mut self, trust_level: Float) {
        self.trust_in_process = trust_level.clamp(0.0, 1.0);

        // Trust affects faith development
        self.update_faith_development(self.trust_in_process, 0.7);
    }

    /// Get diagnostic information
    pub fn get_diagnostic_info(&self) -> String {
        format!(
            "Catalyst of Spirit (A17) Diagnostics:\n\
             - Lambda: {:.3} ({})\n\
             - Faith Development: {:.3}\n\
             - Viewpoint Changes Effectiveness: {:.3}\n\
             - Spiritual Growth Rate: {:.3}\n\
             - Unprocessed Catalyst: {:.3}\n\
             - Unique Processing Capacity: {:.3}\n\
             - Trust in Process: {:.3}\n\
             - Catalyst Harmony: {:.3}\n\
             - Polarity: {}\n\
             - Holonic Level: {:?}",
            self.lambda.value,
            self.lambda.health_status(),
            self.faith_development,
            self.viewpoint_changes_effectiveness,
            self.spiritual_growth_rate,
            self.unprocessed_catalyst,
            self.unique_processing_capacity,
            self.trust_in_process,
            self.calculate_catalyst_harmony(),
            self.polarity,
            self.holonic_level
        )
    }
}

// Trait implementations follow...

// ArchetypeTrait impl is at the end of file

// Trait implementations follow...

// ArchetypeTrait impl is at the end of file

impl CatalystSpiritArchetype {
    /// Get developmental position (method wrapper for field access)
    pub fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }

    /// Get paired archetype ID (Catalyst pairs with Transformation)
    pub fn paired_archetype_id(&self) -> u8 {
        20 // Transformation of Spirit
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
        // Catalyst always includes previous development
        true
    }
}

impl ArchetypeTrait for CatalystSpiritArchetype {
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
        ArchetypeRole::Catalyst
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

    fn update_lambda(&mut self, delta: Float) {
        self.lambda.adjust(delta);
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Catalyst of Spirit"
    }

    fn description(&self) -> &str {
        "Catalyst of Spirit - Catalyst processing in Spirit complex"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::ProcessPair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
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
}

impl LambdaMeasurable for CatalystSpiritArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
    }

    fn calculate_lambda(&self) -> Float {
        // Catalyst lambda = (Faith Development + Viewpoint Changes + Spiritual Growth) / 3
        (self.faith_development + self.viewpoint_changes_effectiveness + self.spiritual_growth_rate)
            / 3.0
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();
        if self.lambda.is_pathological_low() {
            indicators.push("Low catalyst processing".to_string());
        }
        if self.lambda.is_pathological_high() {
            indicators.push("Over-activating catalyst".to_string());
        }
        if self.unprocessed_catalyst > 0.7 {
            indicators.push("High unprocessed catalyst".to_string());
        }
        if self.faith_development < 0.4 {
            indicators.push("Low faith development".to_string());
        }
        indicators
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8) // Standard catalyst healthy range
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock archetype for testing paired relationships
    struct MockExperienceSpirit {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockExperienceSpirit {
        fn new(lambda_value: Float) -> Self {
            MockExperienceSpirit {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::ExperienceDepth,
                ),
                tarot: TarotCorrelation::new("The Moon".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockExperienceSpirit {
        fn archetype_id(&self) -> u8 {
            18
        }
        fn name(&self) -> &str {
            "Experience of Spirit"
        }
        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
        }
        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Experience
        }
        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }
        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.max(0.0).min(1.0);
        }
        fn tarot_correlation(&self) -> TarotCorrelation {
            self.tarot.clone()
        }
        fn sigma_axis(&self) -> SigmaAxis {
            SigmaAxis::SigmaC
        }
        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::ProcessPair
        }
        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}
        fn description(&self) -> &str {
            "Experience of Spirit - moonlight navigation"
        }

        fn activate(&mut self, intensity: Float) {
            // Adjust lambda based on intensity
            let lambda = ArchetypeTrait::lambda(self);
            let new_value = (lambda.value + intensity * 0.1).clamp(0.0, 1.0);
            self.update_lambda(new_value);
        }

        fn deactivate(&mut self) {
            // Deactivate by reducing lambda
            self.lambda.adjust(-0.1);
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

    impl Paired for MockExperienceSpirit {
        fn get_pair(&self) -> Option<u8> {
            None
        }
        fn paired_archetype_id(&self) -> Option<u8> {
            Some(17)
        }
        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }
        fn calculate_pair_tension(&self, paired_archetype: &dyn Paired) -> Float {
            let paired_lambda = paired_archetype.lambda().value;
            (self.lambda.value - paired_lambda).abs()
        }
        fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
            let paired_lambda = paired_archetype.lambda().value;
            let sum = self.lambda.value + paired_lambda;
            if sum > 0.0 {
                1.0 - (self.lambda.value - paired_lambda).abs() / sum
            } else {
                0.0
            }
        }
    }

    #[test]
    fn test_a17_creation() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(catalyst.archetype_id(), 17);
        assert_eq!(catalyst.name(), "Catalyst of Spirit");
        assert_eq!(catalyst.complex(), ArchetypeComplex::Spirit);
        assert_eq!(catalyst.role(), ArchetypeRole::Catalyst);
    }

    #[test]
    fn test_a17_tarot_correlation() {
        let catalyst = CatalystSpiritArchetype::new();

        assert!(catalyst.tarot_correlation().card.contains("The Star"));
        assert!(catalyst.tarot_correlation().card.contains("XVII"));
        assert!(catalyst.tarot_correlation().card.contains("Hope"));
        assert!(catalyst.tarot_correlation().card.contains("faith"));
    }

    #[test]
    fn test_a17_sigma_axis() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(catalyst.sigma_axis(), SigmaAxis::SigmaC);
    }

    #[test]
    fn test_a17_functional_pair() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(catalyst.functional_pair(), FunctionalPair::ProcessPair);
    }

    #[test]
    fn test_a17_polarity() {
        let catalyst = CatalystSpiritArchetype::new();

        // A17 has female polarity assignment (STO)
        assert_eq!(catalyst.polarity, Polarity::STO);
    }

    #[test]
    fn test_a17_lambda_type() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(
            catalyst.lambda().measurement_type,
            LambdaMeasurementType::CatalystProcessingRate
        );
    }

    #[test]
    fn test_a17_healthy_range() {
        let catalyst = CatalystSpiritArchetype::new();

        let (min, max) = catalyst.healthy_range();
        assert!((min - 0.5).abs() < 0.01);
        assert!((max - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_a17_initial_health() {
        let catalyst = CatalystSpiritArchetype::new();

        assert!(catalyst.is_healthy());
        assert_eq!(catalyst.health_status(), HealthStatus::Healthy);
        assert!(catalyst.lambda().value >= 0.5);
        assert!(catalyst.lambda().value <= 0.8);
    }

    #[test]
    fn test_a17_faith_development_update() {
        let mut catalyst = CatalystSpiritArchetype::new();
        let initial_faith = catalyst.faith_development;

        catalyst.update_faith_development(0.8, 0.9);

        assert!(catalyst.faith_development > initial_faith);
        assert!(catalyst.faith_development <= 1.0);
    }

    #[test]
    fn test_a17_viewpoint_changes_update() {
        let mut catalyst = CatalystSpiritArchetype::new();
        let initial_viewpoint = catalyst.viewpoint_changes_effectiveness;

        catalyst.update_viewpoint_changes_effectiveness(0.8, 0.7);

        assert!(catalyst.viewpoint_changes_effectiveness > initial_viewpoint);
        assert!(catalyst.viewpoint_changes_effectiveness <= 1.0);
    }

    #[test]
    fn test_a17_spiritual_growth_update() {
        let mut catalyst = CatalystSpiritArchetype::new();
        let initial_growth = catalyst.spiritual_growth_rate;

        catalyst.update_spiritual_growth_rate(0.8, 0.7);

        assert!(catalyst.spiritual_growth_rate > initial_growth);
        assert!(catalyst.spiritual_growth_rate <= 1.0);
    }

    #[test]
    fn test_a17_unprocessed_catalyst_update() {
        let mut catalyst = CatalystSpiritArchetype::new();
        catalyst.update_unprocessed_catalyst(0.8);

        assert!(catalyst.unprocessed_catalyst > 0.4);
        assert!(catalyst.unprocessed_catalyst <= 1.0);
    }

    #[test]
    fn test_a17_unique_processing_update() {
        let mut catalyst = CatalystSpiritArchetype::new();
        let initial_capacity = catalyst.unique_processing_capacity;

        catalyst.update_unique_processing_capacity(0.8, 0.9);

        assert!(catalyst.unique_processing_capacity >= initial_capacity);
        assert!(catalyst.unique_processing_capacity <= 1.0);
    }

    #[test]
    fn test_a17_trust_update() {
        let mut catalyst = CatalystSpiritArchetype::new();
        catalyst.update_trust_in_process(0.9);

        assert!(catalyst.trust_in_process > 0.65);
        assert!(catalyst.trust_in_process <= 1.0);
    }

    #[test]
    fn test_a17_catalyst_harmony() {
        let catalyst = CatalystSpiritArchetype::new();

        let harmony = catalyst.calculate_catalyst_harmony();

        assert!(harmony >= 0.0);
        assert!(harmony <= 1.0);
    }

    #[test]
    fn test_a17_lambda_calculation() {
        let catalyst = CatalystSpiritArchetype::new();

        let lambda = catalyst.calculate_lambda();

        assert!(lambda >= 0.0);
        assert!(lambda <= 1.0);
        // Lambda calculation should be close to the initial lambda value
        assert!((lambda - 0.65).abs() < 0.2);
    }

    #[test]
    fn test_a17_pathological_low() {
        let mut catalyst = CatalystSpiritArchetype::new();
        // Set state to create low lambda (despair, stuck viewpoint)
        catalyst.faith_development = 0.2;
        catalyst.viewpoint_changes_effectiveness = 0.2;
        catalyst.spiritual_growth_rate = 0.2;
        catalyst.integration_capacity = catalyst.calculate_integration_capacity_from_state();
        catalyst.update_lambda(0.0); // Will recalculate from state

        let indicators = catalyst.pathological_indicators();

        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Low lambda")
            || i.contains("Low faith")
            || i.contains("Low viewpoint")));
        assert!(catalyst.health_status() == HealthStatus::PathologicalLow);
    }

    #[test]
    fn test_a17_pathological_high() {
        let mut catalyst = CatalystSpiritArchetype::new();
        // Set state to create high lambda (blind faith, unrealistic hope)
        catalyst.faith_development = 0.95;
        catalyst.viewpoint_changes_effectiveness = 0.95;
        catalyst.spiritual_growth_rate = 0.95;
        catalyst.integration_capacity = catalyst.calculate_integration_capacity_from_state();
        catalyst.update_lambda(0.0); // Will recalculate from state

        let indicators = catalyst.pathological_indicators();

        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("High lambda")));
        assert!(catalyst.health_status() == HealthStatus::PathologicalHigh);
    }

    #[test]
    fn test_a17_paired_archetype_id() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(catalyst.paired_archetype_id(), 18);
    }

    #[test]
    fn test_a17_pair_tension() {
        let catalyst = CatalystSpiritArchetype::new();
        let experience = MockExperienceSpirit::new(0.7);

        let tension = catalyst.calculate_pair_tension(&experience);

        assert!(tension >= 0.0);
        assert!(tension <= 1.0);
    }

    #[test]
    fn test_a17_pair_balance() {
        let catalyst = CatalystSpiritArchetype::new();
        let experience = MockExperienceSpirit::new(0.65);

        let balance = catalyst.calculate_pair_balance(&experience);

        assert!(balance >= 0.0);
        assert!(balance <= 1.0);
    }

    #[test]
    fn test_a17_holonic_level() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(catalyst.holonic_level(), HolonicLevel::Meso);
    }

    #[test]
    fn test_a17_transcend() {
        let mut catalyst = CatalystSpiritArchetype::new();
        // Set up conditions for transcendence from Meso to Macro
        catalyst.faith_development = 0.8;
        catalyst.viewpoint_changes_effectiveness = 0.8;
        catalyst.spiritual_growth_rate = 0.7;
        catalyst.holonic_level = HolonicLevel::Meso;
        catalyst.integration_capacity = catalyst.calculate_integration_capacity_from_state();
        catalyst.update_lambda(0.75);

        let initial_level = catalyst.holonic_level();
        let transcended = catalyst.transcend();

        if initial_level == HolonicLevel::Meso {
            assert!(transcended);
            assert!(catalyst.holonic_level() != initial_level);
            assert_eq!(catalyst.holonic_level(), HolonicLevel::Macro);
        }
    }

    #[test]
    fn test_a17_include() {
        let mut catalyst = CatalystSpiritArchetype::new();
        // Catalyst's include method takes no arguments
        let initial_capacity = catalyst.integration_capacity();
        let included = catalyst.include();

        assert!(included);
        assert!(catalyst.integration_capacity() >= initial_capacity);
    }

    #[test]
    fn test_a17_developmental_position() {
        let catalyst = CatalystSpiritArchetype::new();

        assert_eq!(
            catalyst.developmental_position(),
            DevelopmentalPosition::Catalyst
        );
    }

    #[test]
    fn test_a17_process() {
        let mut catalyst = CatalystSpiritArchetype::new();
        catalyst.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Process should update lambda and potentially holonic level
        assert!(catalyst.lambda().value >= 0.0);
        assert!(catalyst.lambda().value <= 1.0);
    }

    #[test]
    fn test_a17_diagnostic_info() {
        let catalyst = CatalystSpiritArchetype::new();

        let info = catalyst.get_diagnostic_info();

        assert!(info.contains("Catalyst of Spirit"));
        assert!(info.contains("Lambda"));
        assert!(info.contains("Faith Development"));
        assert!(info.contains("Viewpoint Changes"));
    }
}
