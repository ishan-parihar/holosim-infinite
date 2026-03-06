// A19: The Significator of Spirit
// Living Entity - Identity Agent for Spirit Complex
// The living entity which either radiates or absorbs love and light of One Infinite Creator

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, DevelopmentalPosition, FunctionalPair,
    HealthStatus, HolonicLevel, LambdaMeasurement, LambdaMeasurementType, SigmaAxis,
    TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Significator of Spirit - Living Entity
/// The living entity which either radiates or absorbs love and light of One Infinite Creator
#[derive(Debug, Clone)]
pub struct SignificatorSpiritArchetype {
    pub archetype_id: u8,
    pub active: bool,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // A19-specific fields
    /// How clear is the living entity identity (0.0-1.0)
    pub living_entity_sense: Float,

    /// Rate of love and light flow (0.0-1.0) - measures adept power
    pub flux_rate: Float,

    /// Direction of love/light flow (negative = absorbs, positive = radiates)
    pub radiation_absorption_balance: Float,

    /// How much power does flux rate indicate (0.0-1.0)
    pub power_measurement: Float,

    /// How well is identity coherent (0.0-1.0)
    pub identity_coherence: Float,

    /// How much is significator influenced by veil (0.0-1.0)
    pub veil_influence: Float,

    /// How well is polarity manifested (0.0-1.0)
    pub polarity_manifestation: Float,

    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
    pub polarity: Polarity,
}

impl Default for SignificatorSpiritArchetype {
    fn default() -> Self {
        Self::new()
    }
}

impl SignificatorSpiritArchetype {
    pub fn new() -> Self {
        // Healthy initial values for Significator of Spirit
        // Lambda: 0.65 (within healthy range 0.5-0.8)
        let lambda = LambdaMeasurement::new(0.65, LambdaMeasurementType::SignificatorCoherence);

        let tarot = TarotCorrelation::new("The Sun (XIX): Illumination, vitality, success, joy, spiritual enlightenment".to_string());

        let developmental_position = DevelopmentalPosition::new_with_octant_rung(Octant::O1, 4);

        // Initialize activation levels for all rungs
        let mut activation_levels = HashMap::new();
        let all_rungs = vec![
            Rung::R1,
            Rung::R2,
            Rung::R3,
            Rung::R4,
            Rung::R5,
            Rung::R6,
            Rung::R7,
        ];
        for rung in all_rungs {
            activation_levels.insert(rung, 0.0);
        }
        activation_levels.insert(Rung::R4, 0.65); // Activate heart rung

        SignificatorSpiritArchetype {
            archetype_id: 19,
            active: true,
            lambda,
            tarot_correlation: tarot,
            living_entity_sense: 0.65,      // Clear living entity identity
            flux_rate: 0.65,                 // Balanced flow rate
            radiation_absorption_balance: 0.65, // Slightly radiating (positive polarity)
            power_measurement: 0.65,        // Adequate power indication
            identity_coherence: 0.65,       // Coherent identity
            veil_influence: 0.5,            // Moderately influenced by veil
            polarity_manifestation: 0.65,   // Good polarity manifestation
            developmental_position,
            activated_rungs: vec![Rung::R4],
            activation_levels,
            description: "Significator of Spirit - Living Entity which either radiates or absorbs love and light of One Infinite Creator".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.65,
            polarity: Polarity::STO, // Female polarity assignment after veiling
        }
    }

    // ========================================================================
    // A19-SPECIFIC METHODS
    // ========================================================================

    /// Calculate flux rate score - measures adept power
    pub fn calculate_flux_rate_score(&self) -> Float {
        let radiation_component = self.radiation_absorption_balance.max(0.0); // Only positive (radiation)
        let flow_component = self.flux_rate;
        let significator_strength = self.identity_coherence;

        if significator_strength > 0.0 {
            (radiation_component * flow_component) / significator_strength
        } else {
            0.0
        }
    }

    /// Calculate power measurement score
    pub fn calculate_power_measurement_score(&self) -> Float {
        // Flux rate measures power
        (self.flux_rate * 0.6) + (self.identity_coherence * 0.4)
    }

    /// Calculate living entity identity score
    pub fn calculate_living_entity_score(&self) -> Float {
        let entity_sense = self.living_entity_sense;
        let coherence = self.identity_coherence;
        let veil_effect = 1.0 - self.veil_influence; // Less veil influence = clearer identity

        (entity_sense * 0.4) + (coherence * 0.4) + (veil_effect * 0.2)
    }

    /// Calculate radiation vs absorption balance
    pub fn calculate_radiation_absorption_balance(&self) -> Float {
        self.radiation_absorption_balance
    }

    /// Update living entity sense
    pub fn update_living_entity_sense(&mut self, value: Float) {
        self.living_entity_sense = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update flux rate
    pub fn update_flux_rate(&mut self, value: Float) {
        self.flux_rate = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update radiation/absorption balance
    pub fn update_radiation_absorption_balance(&mut self, value: Float) {
        self.radiation_absorption_balance = value.clamp(-1.0, 1.0);
        // Update polarity based on radiation/absorption
        if value > 0.3 {
            self.polarity = Polarity::STO;
        } else if value < -0.3 {
            self.polarity = Polarity::STS;
        } else {
            self.polarity = Polarity::SinkholeOfIndifference;
        }
        self.recalculate_lambda();
    }

    /// Update power measurement
    pub fn update_power_measurement(&mut self, value: Float) {
        self.power_measurement = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update identity coherence
    pub fn update_identity_coherence(&mut self, value: Float) {
        self.identity_coherence = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update veil influence
    pub fn update_veil_influence(&mut self, value: Float) {
        self.veil_influence = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update polarity manifestation
    pub fn update_polarity_manifestation(&mut self, value: Float) {
        self.polarity_manifestation = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Recalculate lambda based on current state
    pub fn recalculate_lambda(&mut self) {
        let flux_score = self.calculate_flux_rate_score();
        let power_score = self.calculate_power_measurement_score();
        let living_entity_score = self.calculate_living_entity_score();

        // Weighted average of all components
        let new_lambda = (flux_score * 0.4) + (power_score * 0.3) + (living_entity_score * 0.3);
        self.lambda.value = new_lambda.clamp(0.0, 1.0);

        // Update integration capacity
        self.integration_capacity = self.calculate_integration_capacity_from_state();
    }

    /// Calculate integration capacity from state
    pub fn calculate_integration_capacity_from_state(&self) -> Float {
        let component1 = self.flux_rate * 0.3;
        let component2 = self.identity_coherence * 0.35;
        let component3 = self.polarity_manifestation * 0.35;
        component1 + component2 + component3
    }

    /// Check if living entity is radiating
    pub fn is_radiating(&self) -> bool {
        self.radiation_absorption_balance > 0.3
    }

    /// Check if living entity is absorbing
    pub fn is_absorbing(&self) -> bool {
        self.radiation_absorption_balance < -0.3
    }

    /// Check if living entity has balanced flow
    pub fn is_balanced_flow(&self) -> bool {
        self.radiation_absorption_balance >= -0.3 && self.radiation_absorption_balance <= 0.3
    }

    /// Check if living entity has high power
    pub fn has_high_power(&self) -> bool {
        self.flux_rate > 0.7 && self.power_measurement > 0.7
    }

    /// Check if living entity has low power
    pub fn has_low_power(&self) -> bool {
        self.flux_rate < 0.3 || self.power_measurement < 0.3
    }

    /// Check if identity is clear
    pub fn is_identity_clear(&self) -> bool {
        self.living_entity_sense > 0.6 && self.identity_coherence > 0.6
    }

    /// Check if identity is confused
    pub fn is_identity_confused(&self) -> bool {
        self.living_entity_sense < 0.4 || self.identity_coherence < 0.4
    }

    /// Check if veil influence is high
    pub fn is_veil_influence_high(&self) -> bool {
        self.veil_influence > 0.7
    }

    /// Check if veil influence is low
    pub fn is_veil_influence_low(&self) -> bool {
        self.veil_influence < 0.3
    }

    /// Get diagnostic information
    pub fn get_diagnostic_info(&self) -> String {
        format!(
            "Significator of Spirit (A19) - Living Entity\n\
             -------------------------------------------\n\
             Flux Rate: {:.2} (Power Measurement)\n\
             Radiation/Absorption Balance: {:.2} ({})\n\
             Living Entity Sense: {:.2}\n\
             Identity Coherence: {:.2}\n\
             Veil Influence: {:.2}\n\
             Polarity Manifestation: {:.2}\n\
             Polarity: {}\n\
             Lambda: {:.2} ({})\n\
             Flux Rate Score: {:.2}\n\
             Power Measurement Score: {:.2}\n\
             Living Entity Score: {:.2}\n\
             Integration Capacity: {:.2}",
            self.flux_rate,
            self.radiation_absorption_balance,
            if self.is_radiating() {
                "Radiating"
            } else if self.is_absorbing() {
                "Absorbing"
            } else {
                "Balanced"
            },
            self.living_entity_sense,
            self.identity_coherence,
            self.veil_influence,
            self.polarity_manifestation,
            self.polarity,
            self.lambda.value,
            self.lambda.health_status(),
            self.calculate_flux_rate_score(),
            self.calculate_power_measurement_score(),
            self.calculate_living_entity_score(),
            self.integration_capacity
        )
    }
}

// Trait implementations follow...

// ArchetypeTrait impl is at the end of file
impl ArchetypeTrait for SignificatorSpiritArchetype {
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
        "The Significator of Spirit"
    }

    fn description(&self) -> &str {
        "Significator of Spirit - Significator in Spirit complex"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::SignificatorPair
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

    // Mock archetype for testing paired relationships
    #[allow(dead_code)]
    struct MockGreatWaySpirit {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    #[allow(dead_code)]
    impl MockGreatWaySpirit {
        fn new(lambda_value: Float) -> Self {
            MockGreatWaySpirit {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::GreatWayClarity,
                ),
                tarot: TarotCorrelation::new("The World".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockGreatWaySpirit {
        fn archetype_id(&self) -> u8 {
            21
        }

        fn name(&self) -> &str {
            "Mock Great Way Spirit"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::GreatWay
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
