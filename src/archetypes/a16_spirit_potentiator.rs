use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, HealthStatus as CommonHealthStatus, Holonic, HolonicLevel,
    LambdaMeasurable, LambdaMeasurement, LambdaMeasurementType, Paired, SigmaAxis,
    TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Resource structure for Spirit Potentiator archetypes
///
/// Represents different types of spiritual resources available for illumination
#[derive(Debug, Clone)]
pub struct SpiritResourceStructure {
    pub active: bool,

    /// Lightning power resources
    pub lightning_resources: Float,
    /// Infinite potential access
    pub infinite_potential_resources: Float,
    /// Awakening capacity
    pub awakening_resources: Float,
    /// Illuminating resources
    pub illuminating_resources: Float,
    /// Generative resources
    pub generative_resources: Float,
    /// Transformational resources
    pub transformational_resources: Float,
}

impl SpiritResourceStructure {
    /// Calculate total resources (average of all resource types)
    pub fn total_resources(&self) -> Float {
        (self.lightning_resources
            + self.infinite_potential_resources
            + self.awakening_resources
            + self.illuminating_resources
            + self.generative_resources
            + self.transformational_resources)
            / 6.0
    }

    /// Calculate resource diversity (lower standard deviation = more balanced)
    pub fn resource_diversity(&self) -> Float {
        let resources = vec![
            self.lightning_resources,
            self.infinite_potential_resources,
            self.awakening_resources,
            self.illuminating_resources,
            self.generative_resources,
            self.transformational_resources,
        ];

        // Calculate mean
        let mean = resources.iter().sum::<Float>() / resources.len() as Float;

        // Calculate variance
        let variance =
            resources.iter().map(|r| (r - mean).powi(2)).sum::<Float>() / resources.len() as Float;

        // Calculate standard deviation
        let std_dev = variance.sqrt();

        // Diversity = 1 - (standard deviation / max possible deviation)
        1.0 - (std_dev / 0.5).min(1.0)
    }
}

#[derive(Debug, Clone)]
pub struct PotentiatorSpiritArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,
    // A16-specific fields
    pub lightning_power: Float,
    pub infinite_potential_access: Float,
    pub sudden_awakening: Float,
    pub illuminating_influence: Float,
    pub generative_power: Float,
    pub viewpoint_changes_effectiveness: Float,
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
    pub polarity: Polarity,

    // NEW: Resource structure (Phase 5)
    pub resource_structure: SpiritResourceStructure,
    pub resource_quality: Float, // Quality of resources (0.0 - 1.0)
    pub resource_depth: Float,   // Depth of resources (0.0 - 1.0)
}

impl PotentiatorSpiritArchetype {
    pub fn new() -> Self {
        // Initialize resource structure
        let resource_structure = SpiritResourceStructure {
            active: true,
            lightning_resources: 0.7,
            infinite_potential_resources: 0.6,
            awakening_resources: 0.65,
            illuminating_resources: 0.7,
            generative_resources: 0.6,
            transformational_resources: 0.65,
        };

        let resource_quality = resource_structure.total_resources();
        let resource_depth = 0.65;

        PotentiatorSpiritArchetype {
            archetype_id: 16,
            lambda: LambdaMeasurement::new(0.65, LambdaMeasurementType::PotentiatorAccessibility),
            tarot_correlation: TarotCorrelation::new(format!("The Tower (XVI): Sudden awakening, illumination, destruction of illusion, lightning strike")),
            // A16-specific fields
            lightning_power: 0.7,
            infinite_potential_access: 0.6,
            sudden_awakening: 0.65,
            illuminating_influence: 0.7,
            generative_power: 0.6,
            viewpoint_changes_effectiveness: 0.65,
            // Developmental fields
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O1, 5),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4, Rung::R5],
            activation_levels: {
                let mut map = HashMap::new();
                map.insert(Rung::R1, 0.3);
                map.insert(Rung::R2, 0.4);
                map.insert(Rung::R3, 0.5);
                map.insert(Rung::R4, 0.6);
                map.insert(Rung::R5, 0.65);
                map.insert(Rung::R6, 0.0);
                map.insert(Rung::R7, 0.0);
                map
            },
            description: String::from(
                "The Potentiator of Spirit is the most sudden awakening, illuminating, and generative influence - may be seen as Lightning. Originally light in its sudden and fiery form, refined in tarot into Lightning Struck Tower. The Potentiator is identical for both positive and negative adepts - the same lightning can illuminate or destroy depending on the adept's polarization.",
            ),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.65,
            polarity: Polarity::STS, // Male polarity assignment after veiling
            resource_structure,
            resource_quality,
            resource_depth,
        }
    }

    /// Calculate spirit potentiator harmony score
    pub fn calculate_spirit_potentiator_harmony(&self) -> Float {
        let awakening_factor = self.sudden_awakening;
        let illumination_factor = self.illuminating_influence;
        let generative_factor = self.generative_power;
        let lightning_factor = self.lightning_power;

        let harmony =
            (awakening_factor + illumination_factor + generative_factor + lightning_factor) / 4.0;
        harmony
    }

    /// Update lightning power based on spiritual activation
    pub fn update_lightning_power(&mut self, activation: Float) {
        self.lightning_power = (self.lightning_power + activation * 0.1).max(0.0).min(1.0);
        self.update_lambda();
    }

    /// Update infinite potential access based on spiritual depth
    pub fn update_infinite_potential_access(&mut self, depth: Float) {
        self.infinite_potential_access = (self.infinite_potential_access + depth * 0.1)
            .max(0.0)
            .min(1.0);
        self.update_lambda();
    }

    /// Update sudden awakening based on spiritual breakthrough
    pub fn update_sudden_awakening(&mut self, breakthrough: Float) {
        self.sudden_awakening = (self.sudden_awakening + breakthrough * 0.1)
            .max(0.0)
            .min(1.0);
        self.update_lambda();
    }

    /// Update illuminating influence based on spiritual light
    pub fn update_illuminating_influence(&mut self, light: Float) {
        self.illuminating_influence = (self.illuminating_influence + light * 0.1)
            .max(0.0)
            .min(1.0);
        self.update_lambda();
    }

    /// Update generative power based on spiritual movement
    pub fn update_generative_power(&mut self, movement: Float) {
        self.generative_power = (self.generative_power + movement * 0.1).max(0.0).min(1.0);
        self.update_lambda();
    }

    /// Update viewpoint changes effectiveness based on catalyst interaction
    pub fn update_viewpoint_changes_effectiveness(&mut self, effectiveness: Float) {
        self.viewpoint_changes_effectiveness = (self.viewpoint_changes_effectiveness
            + effectiveness * 0.1)
            .max(0.0)
            .min(1.0);
        self.update_lambda();
    }

    /// Increase resource depth for potentiator
    pub fn increase_resource_depth(&mut self, amount: Float) {
        self.resource_depth = (self.resource_depth + amount * 0.1).max(0.0).min(1.0);
        self.resource_quality = self.resource_structure.total_resources();
        self.update_lambda();
    }

    /// Update lambda based on all A16-specific fields
    fn update_lambda(&mut self) {
        let awakening = self.sudden_awakening;
        let illumination = self.illuminating_influence;
        let generative = self.generative_power;
        let lightning = self.lightning_power;

        // Calculate lambda as weighted average
        let new_lambda = awakening * 0.3 + illumination * 0.3 + generative * 0.2 + lightning * 0.2;
        self.lambda.value = new_lambda.max(0.0).min(1.0);
    }

    /// Get diagnostic information about the spirit potentiator
    pub fn get_diagnostic_info(&self) -> String {
        format!(
            "A16 Potentiator of Spirit - Lightning\n\
             Lightning Power: {:.2}\n\
             Infinite Potential Access: {:.2}\n\
             Sudden Awakening: {:.2}\n\
             Illuminating Influence: {:.2}\n\
             Generative Power: {:.2}\n\
             Viewpoint Changes Effectiveness: {:.2}\n\
             Harmony Score: {:.2}\n\
             Lambda: {:.2} (Healthy: {:.2}-{:.2})\n\
             Status: {}",
            self.lightning_power,
            self.infinite_potential_access,
            self.sudden_awakening,
            self.illuminating_influence,
            self.generative_power,
            self.viewpoint_changes_effectiveness,
            self.calculate_spirit_potentiator_harmony(),
            self.lambda.value,
            self.lambda.healthy_min,
            self.lambda.healthy_max,
            self.lambda.health_status()
        )
    }

    /// Calculate Potentiator's illumination intensity (Spirit complex)
    /// Spirit: Potentiator ILLUMINATES Matrix (lightning striking primeval darkness)
    pub fn calculate_illumination_intensity(&self) -> Float {
        // Illumination = lightning_power × infinite_potential_access
        self.lightning_power * self.infinite_potential_access
    }

    // NEW: Resource availability methods (Phase 5)

    /// Calculate resource availability based on spiritual needs
    pub fn calculate_resource_availability(&self) -> Float {
        // Resource availability = total resources × quality × depth
        let total_resources = self.resource_structure.total_resources();
        let availability = total_resources * self.resource_quality * self.resource_depth;
        availability.min(1.0)
    }

    /// Calculate resource diversity score
    pub fn calculate_resource_diversity(&self) -> Float {
        self.resource_structure.resource_diversity()
    }

    /// Initialize resource structure from existing properties
    pub fn initialize_resource_structure(&mut self) {
        self.resource_structure = SpiritResourceStructure {
            active: true,
            lightning_resources: self.lightning_power,
            infinite_potential_resources: self.infinite_potential_access,
            awakening_resources: self.sudden_awakening,
            illuminating_resources: self.illuminating_influence,
            generative_resources: self.generative_power,
            transformational_resources: self.viewpoint_changes_effectiveness,
        };

        // Calculate quality and depth
        self.resource_quality = self.resource_structure.total_resources();
        self.resource_depth = self.infinite_potential_access;
    }
}

// Trait implementations follow...

// ArchetypeTrait impl is at the end of file

impl LambdaMeasurable for PotentiatorSpiritArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        let awakening = self.sudden_awakening;
        let illumination = self.illuminating_influence;
        let generative = self.generative_power;
        let lightning = self.lightning_power;

        // Weighted average of key factors
        let lambda = awakening * 0.3 + illumination * 0.3 + generative * 0.2 + lightning * 0.2;
        lambda.max(0.0).min(1.0)
    }

    fn healthy_range(&self) -> (Float, Float) {
        (self.lambda.healthy_min, self.lambda.healthy_max)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        match self.lambda.health_status() {
            CommonHealthStatus::PathologicalLow => {
                if self.sudden_awakening < 0.5 {
                    indicators.push("Stagnation - no sudden awakening".to_string());
                }
                if self.illuminating_influence < 0.5 {
                    indicators.push("Darkness - no illumination".to_string());
                }
                if self.lightning_power < 0.5 {
                    indicators.push("Weak lightning - lack of activation".to_string());
                }
                indicators.push("Spiritual darkness - lack of awakening".to_string());
            }
            CommonHealthStatus::PathologicalHigh => {
                if self.sudden_awakening > 0.8 {
                    indicators.push("Over-illumination - destructive awakening".to_string());
                }
                if self.lightning_power > 0.8 {
                    indicators.push("Excessive lightning - loss of stability".to_string());
                }
                indicators.push("Uncontrolled awakening - spiritual chaos".to_string());
            }
            CommonHealthStatus::Warning => {
                indicators.push(
                    "Warning: Awakening potential declining, spiritual attention needed"
                        .to_string(),
                );
            }
            CommonHealthStatus::Degraded => {
                indicators.push(
                    "Degraded: Multiple awakening issues, spiritual energy significantly impaired"
                        .to_string(),
                );
            }
            CommonHealthStatus::Pathological => {
                indicators.push(
                    "Pathological: Severe awakening dysfunction, spiritual energy blocked"
                        .to_string(),
                );
            }
            CommonHealthStatus::Healthy => {}
            CommonHealthStatus::Balanced => {}
            CommonHealthStatus::Imbalanced => {
                indicators.push(
                    "Imbalanced: Awakening patterns disrupted, need spiritual rebalancing"
                        .to_string(),
                );
            }
        }

        indicators
    }
}

impl Developmental for PotentiatorSpiritArchetype {
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

impl Paired for PotentiatorSpiritArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(15) // Paired with A15 Matrix of Spirit
    }

    fn get_pair(&self) -> Option<u8> {
        Some(15)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, paired_archetype: &dyn Paired) -> Float {
        let paired_lambda = paired_archetype.lambda().value;
        let tension = (self.lambda.value - paired_lambda).abs();
        tension
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

impl Holonic for PotentiatorSpiritArchetype {
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
        // Transcend through sudden awakening and illumination
        let awakening_threshold = 0.8;
        let illumination_threshold = 0.8;

        if self.sudden_awakening >= awakening_threshold
            && self.illuminating_influence >= illumination_threshold
        {
            self.holonic_level = match self.holonic_level {
                HolonicLevel::Micro => HolonicLevel::Meso,
                HolonicLevel::Meso => HolonicLevel::Macro,
                HolonicLevel::Macro => HolonicLevel::Meta,
                HolonicLevel::Meta => HolonicLevel::Meta,
            };
            true
        } else {
            false
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include lower level by integrating its capacity
        let lower_capacity = lower_level.integration_capacity();
        self.integration_capacity = (self.integration_capacity + lower_capacity * 0.1)
            .max(0.0)
            .min(1.0);
        true
    }
}

// ============================================================================
// POTENTIATOR ARCHETYPE TRAIT IMPLEMENTATION (Phase 10)
// ============================================================================

impl crate::archetypes::archetype_traits::PotentiatorArchetypeTrait for PotentiatorSpiritArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement {
        self.lambda.clone()
    }

    fn get_resource_accessibility(&self) -> Float {
        // Spirit uses infinite potential access as proxy for accessibility
        self.infinite_potential_access
    }

    fn get_resource_quality(&self) -> Float {
        self.resource_quality
    }

    fn get_resource_depth(&self) -> Float {
        self.resource_depth
    }

    // Core setters
    fn set_resource_accessibility(&mut self, value: Float) {
        // Spirit uses infinite potential access as accessibility
        self.infinite_potential_access = value.clamp(0.0, 1.0);
    }

    fn set_resource_quality(&mut self, value: Float) {
        self.resource_quality = value.clamp(0.0, 1.0);
    }

    fn set_resource_depth(&mut self, value: Float) {
        self.resource_depth = value.clamp(0.0, 1.0);
    }

    fn increase_resource_depth(&mut self, increase: Float) {
        self.resource_depth = (self.resource_depth + increase).min(1.0);
    }

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
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

    // Phase 5: Resource structure
    fn calculate_resource_availability(&self) -> Float {
        self.calculate_resource_availability()
    }

    fn calculate_resource_diversity(&self) -> Float {
        self.calculate_resource_diversity()
    }

    // Complex-specific M-P dynamics (Phase 2)
    // Spirit: Potentiator provides ILLUMINATION to strike Matrix
    fn calculate_receptivity(&self) -> Float {
        // Spirit complex doesn't use receptivity dynamics
        0.0
    }

    fn calculate_regulatory_intensity(&self) -> Float {
        // Spirit complex doesn't use regulation dynamics
        0.0
    }

    fn calculate_illumination_intensity(&self) -> Float {
        self.calculate_illumination_intensity()
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        // Health based on lambda value
        let lambda = self.lambda.value;
        if lambda >= 0.7 {
            HealthStatus::Healthy
        } else if lambda >= 0.5 {
            HealthStatus::Warning
        } else if lambda >= 0.3 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Pathological
        }
    }
}

impl ArchetypeTrait for PotentiatorSpiritArchetype {
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
        ArchetypeRole::Potentiator
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        let position_value = position.rung_level() as Float / 7.0;
        self.lambda.adjust(catalyst * position_value);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaB
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
        "The Potentiator of Spirit"
    }

    fn description(&self) -> &str {
        "Potentiator of Spirit - Potential for consciousness development in Spirit complex"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::MatrixPair
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

#[cfg(test)]
mod tests {
    use super::*;

    // Mock A15 Matrix of Spirit for testing paired relationships
    struct MockMatrixSpirit {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockMatrixSpirit {
        fn new(lambda_value: Float) -> Self {
            MockMatrixSpirit {
                lambda: LambdaMeasurement::new(lambda_value, LambdaMeasurementType::MatrixRigidity),
                tarot: TarotCorrelation::new("The Devil".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockMatrixSpirit {
        fn archetype_id(&self) -> u8 {
            15
        }
        fn name(&self) -> &str {
            "Matrix of Spirit"
        }
        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
        }
        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Matrix
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
            FunctionalPair::StructurePair
        }
        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}
        fn description(&self) -> &str {
            "Matrix of Spirit - night of the soul"
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

    impl Paired for MockMatrixSpirit {
        fn get_pair(&self) -> Option<u8> {
            None
        }
        fn paired_archetype_id(&self) -> Option<u8> {
            Some(16) // Paired with A16
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
    fn test_a16_initialization() {
        let potentiator = PotentiatorSpiritArchetype::new();

        assert_eq!(potentiator.archetype_id(), 16);
        assert_eq!(potentiator.name(), "Potentiator of Spirit");
        assert_eq!(potentiator.complex(), ArchetypeComplex::Spirit);
        assert_eq!(potentiator.role(), ArchetypeRole::Potentiator);
        assert_eq!(potentiator.sigma_axis(), SigmaAxis::SigmaC);
        assert_eq!(potentiator.functional_pair(), FunctionalPair::StructurePair);
    }

    #[test]
    fn test_a16_tarot_correlation() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let tarot = potentiator.tarot_correlation();

        assert!(tarot.card.contains("The Tower"));
        assert!(tarot.card.contains("XVI"));
        assert!(tarot.card.contains("Sudden awakening"));
        assert!(tarot.card.contains("illumination"));
    }

    #[test]
    fn test_a16_lambda_initial_value() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let lambda = crate::archetypes::common::ArchetypeTrait::lambda(&potentiator);

        assert!((lambda.value - 0.65).abs() < 0.01);
        assert!((lambda.healthy_min - 0.5).abs() < 0.01);
        assert!((lambda.healthy_max - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_a16_lambda_calculation() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let calculated_lambda = potentiator.calculate_lambda();

        assert!((calculated_lambda - 0.65).abs() < 0.05);
        assert!(calculated_lambda >= 0.0);
        assert!(calculated_lambda <= 1.0);
    }

    #[test]
    fn test_a16_lightning_power_update() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_power = potentiator.lightning_power;

        potentiator.update_lightning_power(0.3);

        assert!(potentiator.lightning_power > initial_power);
        assert!(potentiator.lightning_power <= 1.0);
    }

    #[test]
    fn test_a16_sudden_awakening_update() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_awakening = potentiator.sudden_awakening;

        potentiator.update_sudden_awakening(0.3);

        assert!(potentiator.sudden_awakening > initial_awakening);
        assert!(potentiator.sudden_awakening <= 1.0);
    }

    #[test]
    fn test_a16_illuminating_influence_update() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_illumination = potentiator.illuminating_influence;

        potentiator.update_illuminating_influence(0.3);

        assert!(potentiator.illuminating_influence > initial_illumination);
        assert!(potentiator.illuminating_influence <= 1.0);
    }

    #[test]
    fn test_a16_generative_power_update() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_generative = potentiator.generative_power;

        potentiator.update_generative_power(0.3);

        assert!(potentiator.generative_power > initial_generative);
        assert!(potentiator.generative_power <= 1.0);
    }

    #[test]
    fn test_a16_viewpoint_changes_effectiveness_update() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_effectiveness = potentiator.viewpoint_changes_effectiveness;

        potentiator.update_viewpoint_changes_effectiveness(0.3);

        assert!(potentiator.viewpoint_changes_effectiveness > initial_effectiveness);
        assert!(potentiator.viewpoint_changes_effectiveness <= 1.0);
    }

    #[test]
    fn test_a16_harmony_score() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let harmony = potentiator.calculate_spirit_potentiator_harmony();

        assert!(harmony >= 0.0);
        assert!(harmony <= 1.0);
        assert!((harmony - 0.65).abs() < 0.1);
    }

    #[test]
    fn test_a16_process() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_illumination = potentiator.illuminating_influence;
        let initial_generative = potentiator.generative_power;

        potentiator.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Process should update illuminating influence and generative power
        assert!(potentiator.illuminating_influence >= initial_illumination);
        assert!(potentiator.generative_power >= initial_generative);
    }

    #[test]
    fn test_a16_polarity() {
        let potentiator = PotentiatorSpiritArchetype::new();

        assert_eq!(potentiator.polarity, Polarity::STS);
    }

    #[test]
    fn test_a16_developmental_position() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let position = potentiator.developmental_position();

        // DevelopmentalPosition is an enum, compare directly
        assert_eq!(position, DevelopmentalPosition::Catalyst);
    }

    #[test]
    fn test_a16_activated_rungs() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let rungs = potentiator.activated_rungs();

        assert_eq!(rungs.len(), 5);
        assert!(rungs.contains(&Rung::R1));
        assert!(rungs.contains(&Rung::R5));
        assert!(!rungs.contains(&Rung::R7));
    }

    #[test]
    fn test_a16_rung_activation_levels() {
        let potentiator = PotentiatorSpiritArchetype::new();

        assert!((potentiator.rung_activation_level(Rung::R1) - 0.3).abs() < 0.01);
        assert!((potentiator.rung_activation_level(Rung::R5) - 0.65).abs() < 0.01);
        assert!((potentiator.rung_activation_level(Rung::R6) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_a16_paired_archetype_id() {
        let potentiator = PotentiatorSpiritArchetype::new();

        assert_eq!(potentiator.paired_archetype_id(), Some(15));
    }

    #[test]
    fn test_a16_pair_tension() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let matrix = MockMatrixSpirit::new(0.6);

        let tension = potentiator.calculate_pair_tension(&matrix);

        assert!(tension >= 0.0);
        assert!(tension <= 1.0);
    }

    #[test]
    fn test_a16_pair_balance() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let matrix = MockMatrixSpirit::new(0.65);

        let balance = potentiator.calculate_pair_balance(&matrix);

        assert!(balance >= 0.0);
        assert!(balance <= 1.0);
    }

    #[test]
    fn test_a16_holonic_level() {
        let potentiator = PotentiatorSpiritArchetype::new();

        assert_eq!(potentiator.holonic_level(), HolonicLevel::Meso);
    }

    #[test]
    fn test_a16_integration_capacity() {
        let potentiator = PotentiatorSpiritArchetype::new();

        assert!((potentiator.integration_capacity() - 0.65).abs() < 0.01);
    }

    #[test]
    fn test_a16_transcend() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_level = potentiator.holonic_level();

        // Set awakening and illumination to transcend thresholds
        potentiator.sudden_awakening = 0.85;
        potentiator.illuminating_influence = 0.85;

        let transcended = potentiator.transcend();

        assert!(transcended);
        assert!(potentiator.holonic_level() != initial_level);
    }

    #[test]
    fn test_a16_include() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let mut lower_level = PotentiatorSpiritArchetype::new();
        lower_level.integration_capacity = 0.5;

        let initial_capacity = potentiator.integration_capacity;

        potentiator.include(&lower_level);

        assert!(potentiator.integration_capacity > initial_capacity);
        assert!(potentiator.integration_capacity <= 1.0);
    }

    #[test]
    fn test_a16_pathological_indicators_low() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        potentiator.lambda.value = 0.3;
        potentiator.sudden_awakening = 0.4;
        potentiator.illuminating_influence = 0.4;
        potentiator.lightning_power = 0.4;

        let indicators = potentiator.pathological_indicators();

        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Stagnation")));
        assert!(indicators.iter().any(|i| i.contains("Darkness")));
    }

    #[test]
    fn test_a16_pathological_indicators_high() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        potentiator.lambda.value = 0.9;
        potentiator.sudden_awakening = 0.85;
        potentiator.lightning_power = 0.85;

        let indicators = potentiator.pathological_indicators();

        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Over-illumination")));
        assert!(indicators.iter().any(|i| i.contains("Excessive lightning")));
    }

    #[test]
    fn test_a16_diagnostic_info() {
        let potentiator = PotentiatorSpiritArchetype::new();
        let diagnostic = potentiator.get_diagnostic_info();

        assert!(diagnostic.contains("A16 Potentiator of Spirit"));
        assert!(diagnostic.contains("Lightning Power"));
        assert!(diagnostic.contains("Sudden Awakening"));
        assert!(diagnostic.contains("Illuminating Influence"));
        assert!(diagnostic.contains("Lambda"));
    }

    // NEW: Phase 5 - Spirit resource mechanics tests

    #[test]
    fn test_spirit_resource_structure_initialization() {
        let potentiator = PotentiatorSpiritArchetype::new();

        // Check that resource structure is initialized
        assert!(potentiator.resource_structure.lightning_resources > 0.0);
        assert!(potentiator.resource_structure.infinite_potential_resources > 0.0);
        assert!(potentiator.resource_structure.awakening_resources > 0.0);
        assert!(potentiator.resource_structure.illuminating_resources > 0.0);
        assert!(potentiator.resource_structure.generative_resources > 0.0);
        assert!(potentiator.resource_structure.transformational_resources > 0.0);
    }

    #[test]
    fn test_spirit_total_resources() {
        let potentiator = PotentiatorSpiritArchetype::new();

        let total = potentiator.resource_structure.total_resources();

        // Total should be average of all resources
        assert!(total >= 0.0);
        assert!(total <= 1.0);
        assert!((total - 0.65).abs() < 0.1); // Should be around 0.65
    }

    #[test]
    fn test_spirit_resource_diversity() {
        let potentiator = PotentiatorSpiritArchetype::new();

        let diversity = potentiator.resource_structure.resource_diversity();

        // Diversity should be between 0 and 1
        assert!(diversity >= 0.0);
        assert!(diversity <= 1.0);

        // With balanced resources, diversity should be high
        assert!(diversity > 0.5);
    }

    #[test]
    fn test_spirit_resource_diversity_unbalanced() {
        let mut resource_structure = SpiritResourceStructure {
            active: true,
            lightning_resources: 1.0,
            infinite_potential_resources: 0.1,
            awakening_resources: 0.1,
            illuminating_resources: 0.1,
            generative_resources: 0.1,
            transformational_resources: 0.1,
        };

        let diversity = resource_structure.resource_diversity();

        // Unbalanced resources should have low diversity
        assert!(diversity < 0.5);
    }

    #[test]
    fn test_spirit_resource_availability() {
        let potentiator = PotentiatorSpiritArchetype::new();

        let availability = potentiator.calculate_resource_availability();

        // Availability should be between 0 and 1
        assert!(availability >= 0.0);
        assert!(availability <= 1.0);
    }

    #[test]
    fn test_spirit_resource_diversity_calculation() {
        let potentiator = PotentiatorSpiritArchetype::new();

        let diversity = potentiator.calculate_resource_diversity();

        // Should match the resource structure's diversity
        assert!((diversity - potentiator.resource_structure.resource_diversity()).abs() < 0.001);
    }

    #[test]
    fn test_spirit_initialize_resource_structure() {
        let mut potentiator = PotentiatorSpiritArchetype::new();

        // Set specific values
        potentiator.lightning_power = 0.8;
        potentiator.infinite_potential_access = 0.75;
        potentiator.sudden_awakening = 0.7;
        potentiator.illuminating_influence = 0.78;
        potentiator.generative_power = 0.72;
        potentiator.viewpoint_changes_effectiveness = 0.68;

        potentiator.initialize_resource_structure();

        // Check that resource structure is updated
        assert!((potentiator.resource_structure.lightning_resources - 0.8).abs() < 0.01);
        assert!((potentiator.resource_structure.infinite_potential_resources - 0.75).abs() < 0.01);
        assert!((potentiator.resource_structure.awakening_resources - 0.7).abs() < 0.01);

        // Check that quality and depth are updated
        assert!(potentiator.resource_quality > 0.0);
        assert!(potentiator.resource_depth > 0.0);
    }

    #[test]
    fn test_spirit_increase_resource_depth() {
        let mut potentiator = PotentiatorSpiritArchetype::new();
        let initial_depth = potentiator.resource_depth;

        potentiator.increase_resource_depth(0.1);

        assert!(potentiator.resource_depth > initial_depth);
        assert!(potentiator.resource_depth <= 1.0);
    }

    #[test]
    fn test_spirit_increase_resource_depth_clamping() {
        let mut potentiator = PotentiatorSpiritArchetype::new();

        // Set depth near maximum
        potentiator.resource_depth = 0.95;

        potentiator.increase_resource_depth(0.1);

        // Should be clamped to 1.0
        assert!((potentiator.resource_depth - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_spirit_resource_metrics_initialization() {
        let potentiator = PotentiatorSpiritArchetype::new();

        // Check that resource metrics are initialized
        assert!(potentiator.resource_quality > 0.0);
        assert!(potentiator.resource_quality <= 1.0);
        assert!(potentiator.resource_depth > 0.0);
        assert!(potentiator.resource_depth <= 1.0);
    }

    #[test]
    fn test_spirit_resource_availability_clamping() {
        let mut potentiator = PotentiatorSpiritArchetype::new();

        // Set values that would exceed 1.0
        potentiator.resource_quality = 1.0;
        potentiator.resource_depth = 1.0;
        potentiator.resource_structure = SpiritResourceStructure {
            active: true,
            lightning_resources: 1.0,
            infinite_potential_resources: 1.0,
            awakening_resources: 1.0,
            illuminating_resources: 1.0,
            generative_resources: 1.0,
            transformational_resources: 1.0,
        };

        let availability = potentiator.calculate_resource_availability();

        // Should be clamped to 1.0
        assert!((availability - 1.0).abs() < 0.001);
    }
}
