//! A2: The Potentiator of Mind
// Catalyst for mind activation - The potentiator of mind
// Tarot: The High Priestess (II)
// Sigma Axis: σM (Mind Capacity)
// Functional Pair: Matrix Pair (with A1)

use crate::archetypes::archetype_traits::PotentiatorArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};

/// Potentiator of Mind Archetype - Catalyst for mind activation
#[derive(Debug, Clone)]
pub struct PotentiatorMindArchetype {
    pub archetype_id: u8,
    pub name: String,
    pub tarot_correlation: TarotCorrelation,
    pub sigma_axis: SigmaAxis,
    pub functional_pair: FunctionalPair,
    pub complex: ArchetypeComplex,
    pub role: ArchetypeRole,
    pub developmental_position: DevelopmentalPosition,
    pub polarization: Polarity,
    pub health_status: HealthStatus,
    pub holonic_level: HolonicLevel,
    pub lambda_measurement: LambdaMeasurement,
    pub is_active: bool,
    pub resource_accessibility: Float,
    pub resource_quality: Float,
    pub resource_depth: Float,
}

impl PotentiatorMindArchetype {
    pub fn new() -> Self {
        PotentiatorMindArchetype {
            archetype_id: 2,
            name: "Potentiator of Mind".to_string(),
            tarot_correlation: TarotCorrelation::new("The High Priestess (II)".to_string()),
            sigma_axis: SigmaAxis::MindCapacity,
            functional_pair: FunctionalPair::MatrixPair,
            complex: ArchetypeComplex::Mind,
            role: ArchetypeRole::Potentiator,
            developmental_position: DevelopmentalPosition::Catalyst,
            polarization: Polarity::Neutral,
            health_status: HealthStatus::Balanced,
            holonic_level: HolonicLevel::Micro,
            lambda_measurement: LambdaMeasurement::new(1.0, LambdaMeasurementType::Capacity),
            is_active: false,
            resource_accessibility: 0.5,
            resource_quality: 0.5,
            resource_depth: 0.5,
        }
    }
}

impl PotentiatorArchetypeTrait for PotentiatorMindArchetype {
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement {
        self.lambda_measurement.clone()
    }

    fn get_resource_accessibility(&self) -> Float {
        self.resource_accessibility
    }

    fn get_resource_quality(&self) -> Float {
        self.resource_quality
    }

    fn get_resource_depth(&self) -> Float {
        self.resource_depth
    }

    fn set_resource_accessibility(&mut self, value: Float) {
        self.resource_accessibility = value;
    }

    fn set_resource_quality(&mut self, value: Float) {
        self.resource_quality = value;
    }

    fn set_resource_depth(&mut self, value: Float) {
        self.resource_depth = value;
    }

    fn increase_resource_depth(&mut self, increase: Float) {
        self.resource_depth += increase;
    }

    fn get_developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }

    fn get_activated_rungs(&self) -> Vec<Rung> {
        let mut rungs = Vec::new();
        if self.lambda_measurement.value > 0.1 {
            rungs.push(Rung::R1);
        }
        if self.lambda_measurement.value > 0.25 {
            rungs.push(Rung::R2);
        }
        if self.lambda_measurement.value > 0.4 {
            rungs.push(Rung::R3);
        }
        if self.lambda_measurement.value > 0.55 {
            rungs.push(Rung::R4);
        }
        if self.lambda_measurement.value > 0.7 {
            rungs.push(Rung::R5);
        }
        if self.lambda_measurement.value > 0.85 {
            rungs.push(Rung::R6);
        }
        if self.lambda_measurement.value > 0.95 {
            rungs.push(Rung::R7);
        }
        rungs
    }

    fn get_activation_level(&self, rung: Rung) -> Float {
        match rung.value() {
            1 => self.lambda_measurement.value,
            2 => self.lambda_measurement.value * 0.9,
            3 => self.lambda_measurement.value * 0.8,
            4 => self.lambda_measurement.value * 0.7,
            5 => self.lambda_measurement.value * 0.6,
            6 => self.lambda_measurement.value * 0.5,
            7 => self.lambda_measurement.value * 0.4,
            _ => 0.0,
        }
    }

    fn set_activation_level(&mut self, rung: Rung, level: Float) {
        self.lambda_measurement.value = level;
    }

    fn calculate_receptivity(&self) -> Float {
        self.resource_accessibility * self.resource_quality
    }

    fn calculate_regulatory_intensity(&self) -> Float {
        self.resource_depth * self.resource_quality
    }

    fn calculate_illumination_intensity(&self) -> Float {
        self.resource_accessibility * self.resource_depth
    }

    fn calculate_resource_availability(&self) -> Float {
        self.resource_accessibility * self.resource_depth
    }

    fn calculate_resource_diversity(&self) -> Float {
        self.resource_quality * self.resource_depth
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        self.health_status
    }
}

impl ArchetypeTrait for PotentiatorMindArchetype {
    fn role(&self) -> ArchetypeRole {
        self.role
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        // Process catalyst based on developmental position
        self.lambda_measurement.value += catalyst * 0.1;
        if self.lambda_measurement.value > 1.0 {
            self.lambda_measurement.value = 1.0;
        }
    }

    fn sigma_axis(&self) -> SigmaAxis {
        self.sigma_axis
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda_measurement.value = value;
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        "Potentiator of Mind - Deep resource for Mind complex"
    }

    fn complex(&self) -> ArchetypeComplex {
        self.complex
    }

    fn functional_pair(&self) -> FunctionalPair {
        self.functional_pair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda_measurement
    }

    fn is_healthy(&self) -> bool {
        self.lambda_measurement.is_healthy()
    }

    fn health_status(&self) -> HealthStatus {
        self.health_status
    }

    fn activate(&mut self, intensity: Float) {
        self.lambda_measurement.value = intensity;
    }

    fn deactivate(&mut self) {
        self.lambda_measurement.value = 0.0;
    }

    fn is_active(&self) -> bool {
        self.lambda_measurement.value > 0.0
    }
}

impl Developmental for PotentiatorMindArchetype {
    fn develop(&mut self, catalyst: Float) {
        self.lambda_measurement.value += catalyst * 0.1;
        if self.lambda_measurement.value > 1.0 {
            self.lambda_measurement.value = 1.0;
        }
    }

    fn regress(&mut self, resistance: Float) {
        self.lambda_measurement.value -= resistance * 0.1;
        if self.lambda_measurement.value < 0.0 {
            self.lambda_measurement.value = 0.0;
        }
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        match rung.value() {
            1 => self.lambda_measurement.value,
            2 => self.lambda_measurement.value * 0.9,
            3 => self.lambda_measurement.value * 0.8,
            4 => self.lambda_measurement.value * 0.7,
            5 => self.lambda_measurement.value * 0.6,
            6 => self.lambda_measurement.value * 0.5,
            7 => self.lambda_measurement.value * 0.4,
            _ => 0.0,
        }
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        let mut rungs = Vec::new();
        if self.lambda_measurement.value > 0.1 {
            rungs.push(Rung::R1);
        }
        if self.lambda_measurement.value > 0.25 {
            rungs.push(Rung::R2);
        }
        if self.lambda_measurement.value > 0.4 {
            rungs.push(Rung::R3);
        }
        if self.lambda_measurement.value > 0.55 {
            rungs.push(Rung::R4);
        }
        if self.lambda_measurement.value > 0.7 {
            rungs.push(Rung::R5);
        }
        if self.lambda_measurement.value > 0.85 {
            rungs.push(Rung::R6);
        }
        if self.lambda_measurement.value > 0.95 {
            rungs.push(Rung::R7);
        }
        rungs
    }

    fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }
}

impl Holonic for PotentiatorMindArchetype {
    fn get_holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn set_holonic_level(&mut self, level: HolonicLevel) {
        self.holonic_level = level;
    }

    fn transcend(&mut self) -> bool {
        // Attempt to transcend to next holonic level
        match self.holonic_level {
            HolonicLevel::Micro => {
                if self.lambda_measurement.value > 0.7 {
                    self.holonic_level = HolonicLevel::Meso;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meso => {
                if self.lambda_measurement.value > 0.8 {
                    self.holonic_level = HolonicLevel::Macro;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Macro => {
                if self.lambda_measurement.value > 0.9 {
                    self.holonic_level = HolonicLevel::Meta;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meta => false,
        }
    }

    fn include(&mut self, _other: &dyn Holonic) -> bool {
        // Include another holonic entity
        true
    }

    fn holonic_level(&self) -> HolonicLevel {
        self.holonic_level
    }

    fn integration_capacity(&self) -> Float {
        self.lambda_measurement.value
    }
}

impl Paired for PotentiatorMindArchetype {
    fn get_pair(&self) -> Option<u8> {
        Some(1) // Paired with A1
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda_measurement
    }

    fn paired_archetype_id(&self) -> Option<u8> {
        Some(1) // Paired with A1
    }

    fn calculate_pair_tension(&self, paired: &dyn Paired) -> Float {
        let self_lambda = ArchetypeTrait::lambda(self).value;
        let paired_lambda = paired.lambda().value;
        (self_lambda - paired_lambda).abs()
    }

    fn calculate_pair_balance(&self, paired: &dyn Paired) -> Float {
        let self_lambda = ArchetypeTrait::lambda(self).value;
        let paired_lambda = paired.lambda().value;
        (self_lambda + paired_lambda) / 2.0
    }
}

impl LambdaMeasurable for PotentiatorMindArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda_measurement.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda_measurement.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        self.lambda_measurement.value
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();
        if self.lambda_measurement.value < self.lambda_measurement.healthy_min {
            indicators.push("Lambda too low".to_string());
        }
        if self.lambda_measurement.value > self.lambda_measurement.healthy_max {
            indicators.push("Lambda too high".to_string());
        }
        indicators
    }

    fn healthy_range(&self) -> (Float, Float) {
        (
            self.lambda_measurement.healthy_min,
            self.lambda_measurement.healthy_max,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potentiator_mind_creation() {
        let archetype = PotentiatorMindArchetype::new();
        assert_eq!(archetype.archetype_id, 2);
        assert_eq!(archetype.name, "Potentiator of Mind");
    }

    #[test]
    fn test_potentiator_mind_activation() {
        let mut archetype = PotentiatorMindArchetype::new();
        archetype.activate(0.5);
        assert!(archetype.is_active());
        assert_eq!(LambdaMeasurable::get_lambda(&archetype), 0.5);
    }
}
