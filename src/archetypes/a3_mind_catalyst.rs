//! A3: The Catalyst of Mind
// Input friction - Interaction with Other-Selves, mental challenge
// Tarot: The Empress (III)
// Sigma Axis: σM (Mind Capacity)
// Functional Pair: Process Pair (with A4)

use crate::archetypes::archetype_traits::CatalystArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};

/// Catalyst of Mind Archetype - Input friction for Mind complex
#[derive(Debug, Clone)]
pub struct CatalystMindArchetype {
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
    pub processing_rate: Float,
    pub catalyst_inflow: Float,
    pub processing_capacity: Float,
    pub catalyst_quality: Float,
    pub veil_transparency: Float,
    pub veil_filtering: Float,
    pub polarization_potential: Float,
    pub current_polarization: Float,
    pub processing_efficiency: Float,
}

impl CatalystMindArchetype {
    pub fn new() -> Self {
        CatalystMindArchetype {
            archetype_id: 3,
            name: "Catalyst of Mind".to_string(),
            tarot_correlation: TarotCorrelation::new("The Empress (III)".to_string()),
            sigma_axis: SigmaAxis::MindCapacity,
            functional_pair: FunctionalPair::ProcessPair,
            complex: ArchetypeComplex::Mind,
            role: ArchetypeRole::Catalyst,
            developmental_position: DevelopmentalPosition::Input,
            polarization: Polarity::Neutral,
            health_status: HealthStatus::Balanced,
            holonic_level: HolonicLevel::Micro,
            lambda_measurement: LambdaMeasurement::new(1.0, LambdaMeasurementType::Capacity),
            is_active: false,
            processing_rate: 0.5,
            catalyst_inflow: 0.5,
            processing_capacity: 1.0,
            catalyst_quality: 0.5,
            veil_transparency: 0.5,
            veil_filtering: 0.5,
            polarization_potential: 0.5,
            current_polarization: 0.5,
            processing_efficiency: 0.5,
        }
    }
}

impl CatalystArchetypeTrait for CatalystMindArchetype {
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_processing_rate(&self) -> Float {
        self.processing_rate
    }

    fn get_catalyst_inflow(&self) -> Float {
        self.catalyst_inflow
    }

    fn get_processing_capacity(&self) -> Float {
        self.processing_capacity
    }

    fn get_accumulation_rate(&self) -> Float {
        // Calculate accumulation rate from catalyst_inflow and processing_capacity
        if self.processing_capacity > 0.0 {
            (self.catalyst_inflow / self.processing_capacity).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    fn get_processing_efficiency(&self) -> Float {
        self.processing_efficiency
    }

    fn get_catalyst_quality(&self) -> Float {
        self.catalyst_quality
    }

    fn set_processing_rate(&mut self, value: Float) {
        self.processing_rate = value;
    }

    fn set_catalyst_inflow(&mut self, value: Float) {
        self.catalyst_inflow = value;
    }

    fn set_processing_capacity(&mut self, value: Float) {
        self.processing_capacity = value;
    }

    fn set_accumulation_rate(&mut self, value: Float) {
        // Set catalyst_inflow to achieve desired accumulation rate
        self.catalyst_inflow =
            (value * self.processing_capacity).clamp(0.0, self.processing_capacity);
    }

    fn set_processing_efficiency(&mut self, value: Float) {
        self.processing_efficiency = value;
    }

    fn set_catalyst_quality(&mut self, value: Float) {
        self.catalyst_quality = value;
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

    fn get_veil_transparency(&self) -> Float {
        self.veil_transparency
    }

    fn get_veil_filtering(&self) -> Float {
        self.veil_filtering
    }

    fn set_veil_transparency(&mut self, value: Float) {
        self.veil_transparency = value;
    }

    fn set_veil_filtering(&mut self, value: Float) {
        self.veil_filtering = value;
    }

    fn get_polarization_potential(&self) -> Float {
        self.polarization_potential
    }

    fn get_current_polarization(&self) -> Option<Polarity> {
        if self.current_polarization > 0.5 {
            Some(Polarity::ServiceToOthers)
        } else if self.current_polarization < -0.5 {
            Some(Polarity::ServiceToSelf)
        } else {
            Some(Polarity::Neutral)
        }
    }

    fn set_current_polarization(&mut self, polarization: Option<Polarity>) {
        match polarization {
            Some(Polarity::ServiceToOthers) | Some(Polarity::STO) => {
                self.current_polarization = 1.0
            }
            Some(Polarity::ServiceToSelf) | Some(Polarity::STS) => self.current_polarization = -1.0,
            Some(Polarity::Neutral) | Some(Polarity::SinkholeOfIndifference) => {
                self.current_polarization = 0.0
            }
            None => self.current_polarization = 0.0,
        }
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        self.health_status
    }

    fn clear_flows(&mut self) {
        self.catalyst_inflow = 0.0;
    }

    fn add_flow(&mut self, rung: Rung, flow: Float) {
        self.catalyst_inflow += flow;
    }

    fn get_flow(&self, rung: Rung) -> Float {
        self.catalyst_inflow
    }
}

impl ArchetypeTrait for CatalystMindArchetype {
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
        "Catalyst of Mind - Catalyst for Mind complex"
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

impl Developmental for CatalystMindArchetype {
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

impl Holonic for CatalystMindArchetype {
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

impl Paired for CatalystMindArchetype {
    fn get_pair(&self) -> Option<u8> {
        Some(4) // Paired with A4
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda_measurement
    }

    fn paired_archetype_id(&self) -> Option<u8> {
        Some(4) // Paired with A4
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

impl LambdaMeasurable for CatalystMindArchetype {
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
    fn test_catalyst_mind_creation() {
        let archetype = CatalystMindArchetype::new();
        assert_eq!(archetype.archetype_id, 3);
        assert_eq!(archetype.name, "Catalyst of Mind");
    }

    #[test]
    fn test_catalyst_mind_activation() {
        let mut archetype = CatalystMindArchetype::new();
        archetype.activate(0.5);
        assert!(archetype.is_active());
        assert_eq!(archetype.get_lambda(), 0.5);
    }
}
