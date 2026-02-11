//! A1: The Matrix of Mind
// Input potential - The Mind as a potential for experience
// Tarot: The Magician (I)
// Sigma Axis: σM (Mind Capacity)
// Functional Pair: Matrix Pair (with A2)

use crate::archetypes::archetype_traits::MatrixArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};

/// Matrix of Mind Archetype - Input potential for Mind complex
#[derive(Debug, Clone)]
pub struct MatrixMindArchetype {
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
    pub structural_permeability: Float,
    pub resource_access: Float,
    pub willful_integration: Float,
    pub conscious_coherence: Float,
    pub structural_tension: Float,
    pub integration_capacity: Float,
    pub structural_capacity: Float,
    pub current_load: Float,
    pub processing_efficiency: Float,
    pub accumulation_rate: Float,
}

impl MatrixMindArchetype {
    pub fn new() -> Self {
        MatrixMindArchetype {
            archetype_id: 1,
            name: "Matrix of Mind".to_string(),
            tarot_correlation: TarotCorrelation::new("The Magician (I)".to_string()),
            sigma_axis: SigmaAxis::MindCapacity,
            functional_pair: FunctionalPair::MatrixPair,
            complex: ArchetypeComplex::Mind,
            role: ArchetypeRole::Matrix,
            developmental_position: DevelopmentalPosition::Input,
            polarization: Polarity::Neutral,
            health_status: HealthStatus::Balanced,
            holonic_level: HolonicLevel::Micro,
            lambda_measurement: LambdaMeasurement::new(1.0, LambdaMeasurementType::Capacity),
            is_active: false,
            structural_permeability: 0.5,
            resource_access: 0.5,
            willful_integration: 0.5,
            conscious_coherence: 0.5,
            structural_tension: 0.5,
            integration_capacity: 1.0,
            structural_capacity: 1.0,
            current_load: 0.5,
            processing_efficiency: 0.5,
            accumulation_rate: 0.5,
        }
    }
}

impl MatrixArchetypeTrait for MatrixMindArchetype {
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_structural_permeability(&self) -> Float {
        self.structural_permeability
    }

    fn get_resource_access(&self) -> Float {
        self.resource_access
    }

    fn get_willful_integration(&self) -> Float {
        self.willful_integration
    }

    fn get_conscious_coherence(&self) -> Float {
        self.conscious_coherence
    }

    fn get_structural_tension(&self) -> Float {
        self.structural_tension
    }

    fn get_integration_capacity(&self) -> Float {
        self.integration_capacity
    }

    fn set_structural_permeability(&mut self, value: Float) {
        self.structural_permeability = value;
    }

    fn set_resource_access(&mut self, value: Float) {
        self.resource_access = value;
    }

    fn set_willful_integration(&mut self, value: Float) {
        self.willful_integration = value;
    }

    fn set_conscious_coherence(&mut self, value: Float) {
        self.conscious_coherence = value;
    }

    fn set_structural_tension(&mut self, value: Float) {
        self.structural_tension = value;
    }

    fn set_integration_capacity(&mut self, value: Float) {
        self.integration_capacity = value;
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
        // Simplified: update lambda based on rung activation
        self.lambda_measurement.value = level;
    }

    fn get_structural_capacity(&self) -> Float {
        self.structural_capacity
    }

    fn get_current_load(&self) -> Float {
        self.current_load
    }

    fn get_processing_efficiency(&self) -> Float {
        self.processing_efficiency
    }

    fn get_accumulation_rate(&self) -> Float {
        self.accumulation_rate
    }

    fn set_structural_capacity(&mut self, value: Float) {
        self.structural_capacity = value;
    }

    fn set_current_load(&mut self, value: Float) {
        self.current_load = value;
    }

    fn set_processing_efficiency(&mut self, value: Float) {
        self.processing_efficiency = value;
    }

    fn set_accumulation_rate(&mut self, value: Float) {
        self.accumulation_rate = value;
    }

    fn available_capacity(&self) -> Float {
        self.structural_capacity - self.current_load
    }

    fn is_overloaded(&self) -> bool {
        self.current_load > self.structural_capacity
    }

    fn increase_load(&mut self, load: Float) {
        self.current_load += load;
    }

    fn decrease_load(&mut self, load: Float) {
        self.current_load -= load;
        if self.current_load < 0.0 {
            self.current_load = 0.0;
        }
    }

    fn expand_capacity(&mut self, experience: Float) {
        self.structural_capacity += experience * 0.1;
    }

    fn calculate_reaching_intensity(&self) -> Float {
        self.willful_integration * self.resource_access
    }

    fn calculate_regulatory_susceptibility(&self) -> Float {
        self.structural_permeability * self.conscious_coherence
    }

    fn calculate_illumination_receptivity(&self) -> Float {
        self.structural_tension * self.integration_capacity
    }

    fn calculate_structural_transformation(&self) -> Float {
        self.structural_permeability * self.willful_integration
    }

    fn calculate_state_transformation(&self) -> Float {
        self.conscious_coherence * self.integration_capacity
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        self.health_status
    }
}

impl ArchetypeTrait for MatrixMindArchetype {
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
        "Matrix of Mind - Input potential for Mind complex"
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

impl Developmental for MatrixMindArchetype {
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

impl Holonic for MatrixMindArchetype {
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
            HolonicLevel::Macro => false,
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

impl Paired for MatrixMindArchetype {
    fn get_pair(&self) -> Option<u8> {
        Some(2) // Paired with A2
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda_measurement
    }

    fn paired_archetype_id(&self) -> Option<u8> {
        Some(2) // Paired with A2
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

impl LambdaMeasurable for MatrixMindArchetype {
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
    fn test_matrix_mind_creation() {
        let archetype = MatrixMindArchetype::new();
        assert_eq!(archetype.archetype_id, 1);
        assert_eq!(archetype.name, "Matrix of Mind");
    }

    #[test]
    fn test_matrix_mind_activation() {
        let mut archetype = MatrixMindArchetype::new();
        archetype.activate(0.5);
        assert!(archetype.is_active());
        assert_eq!(archetype.get_lambda(), 0.5);
    }

    #[test]
    fn test_matrix_mind_development() {
        let mut archetype = MatrixMindArchetype::new();
        archetype.develop(0.5);
        assert!(archetype.get_lambda() > 0.0);
    }
}
