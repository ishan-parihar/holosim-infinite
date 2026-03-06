// A10: The Catalyst of Body
// Input friction - Interaction with Other-Selves, physical sensation
// Tarot: Wheel of Fortune (X)
// Sigma Axis: σB (Body Capacity)
// Functional Pair: Process Pair (with A11)

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, DevelopmentalPosition,
    FunctionalPair, HealthStatus, HolonicLevel, LambdaMeasurement,
    LambdaMeasurementType, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Catalyst of Body Archetype - Input friction for Body complex
/// Represents interaction with other-selves, physical sensation, and catalyst processing
#[derive(Debug, Clone)]
pub struct CatalystBodyArchetype {
    pub archetype_id: u8,
    pub active: bool,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,
    // A10-specific fields
    pub sensation_intensity: Float,
    pub physical_friction: Float,
    pub processing_capacity: Float,
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
    pub polarity: Polarity,
}

impl Default for CatalystBodyArchetype {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalystBodyArchetype {
    /// Create a new Catalyst of Body archetype with healthy initial values
    pub fn new() -> Self {
        let mut activation_levels = HashMap::new();

        // Initialize activation levels for each rung
        // R1: Survival catalyst (survival interactions and threats)
        activation_levels.insert(Rung::R1, 0.5);
        // R2: Sensation catalyst (sensation interactions and pleasures)
        activation_levels.insert(Rung::R2, 0.6);
        // R3: Coordination catalyst (skill interactions and challenges)
        activation_levels.insert(Rung::R3, 0.7);
        // R4: Heart catalyst (relationship interactions and emotional exchanges)
        activation_levels.insert(Rung::R4, 0.8);
        // R5: Integral catalyst (holistic interactions and synthesis)
        activation_levels.insert(Rung::R5, 0.75);
        // R6: Transpersonal catalyst (transpersonal interactions and spiritual exchanges)
        activation_levels.insert(Rung::R6, 0.7);
        // R7: Unity catalyst (unity interactions and non-dual experiences)
        activation_levels.insert(Rung::R7, 0.65);

        CatalystBodyArchetype {
            archetype_id: 10,
            active: true,
            lambda: LambdaMeasurement {
                value: 0.65, // Initial healthy processing level
                healthy_min: 0.5,
                healthy_max: 0.8,
                measurement_type: LambdaMeasurementType::CatalystProcessingRate,
            },
            tarot_correlation: TarotCorrelation::new("Wheel of Fortune (X): Cyclical nature of catalyst, turning of fortune".to_string()),
            sensation_intensity: 0.7,
            physical_friction: 0.6,
            processing_capacity: 0.65,
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O4, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,
            description: "The Catalyst of the Body represents interaction with other-selves - each catalyst is dealing with the nature of those experiences entering the energy web and vibratory perceptions. All that assaults the senses is catalyst, and all that is unprocessed that has come before notice of the complex is catalyst. It provides both positive and negative experiences necessary for polarization and learning.".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.6,
            polarity: Polarity::STO, // Service to Others - female polarity assignment after veiling
        }
    }

    /// Process catalyst input - interaction with other-selves
    pub fn process_catalyst(&mut self, intensity: Float) {
        self.sensation_intensity = Float::clamp(intensity, 0.0, 1.0);
        // Update lambda based on processing capacity
        let new_lambda = (self.sensation_intensity * self.processing_capacity
            + self.physical_friction * 0.3)
            .min(1.0);
        self.lambda.value = new_lambda;
    }

    /// Calculate catalyst processing score
    pub fn calculate_processing_score(&self) -> Float {
        (self.sensation_intensity * self.processing_capacity * 2.0).min(1.0)
    }

    /// Get engagement level with other-selves
    pub fn get_engagement_level(&self) -> Float {
        self.sensation_intensity
    }

    /// Get positive/negative balance
    pub fn get_balance(&self) -> Float {
        // Balanced when processing is healthy
        if self.lambda.is_healthy() {
            0.5 + (self.lambda.value - 0.5) * 0.5
        } else {
            0.5 - (0.5 - self.lambda.value).abs() * 0.5
        }
    }

    /// Get octant-specific expression
    pub fn get_octant_expression(&self, octant: Octant) -> String {
        match octant {
            Octant::O1 => "Personal catalyst: Individual physical interactions and direct experiences".to_string(),
            Octant::O2 => "Interpersonal catalyst: One-on-one physical interactions and exchanges".to_string(),
            Octant::O3 => "Intrapersonal catalyst: Individual physical interactions and sensations".to_string(),
            Octant::O4 => "Systemic catalyst: Systemic biological interactions and physiological stimuli".to_string(),
            Octant::O5 => "Institutional catalyst: Institutional physical interactions and organizational catalysts".to_string(),
            Octant::O6 => "Communal catalyst: Communal physical interactions and collective catalysts".to_string(),
            Octant::O7 => "Behavioral catalyst: Behavioral interactions and social catalysts".to_string(),
            Octant::O8 => "Transpersonal catalyst: Transpersonal physical interactions and universal catalysts".to_string(),
            Octant(0_u8) => "Invalid octant".to_string(),
            Octant(9_u8..=u8::MAX) => "Invalid octant".to_string(),
        }
    }

    /// Get rung-specific development
    pub fn get_rung_development(&self, rung: Rung) -> String {
        match rung {
            Rung::R1 => "Survival catalyst: Survival interactions and threats".to_string(),
            Rung::R2 => "Sensation catalyst: Sensation interactions and pleasures".to_string(),
            Rung::R3 => "Coordination catalyst: Skill interactions and challenges".to_string(),
            Rung::R4 => {
                "Heart catalyst: Relationship interactions and emotional exchanges".to_string()
            }
            Rung::R5 => "Integral catalyst: Holistic interactions and synthesis".to_string(),
            Rung::R6 => {
                "Transpersonal catalyst: Transpersonal interactions and spiritual exchanges"
                    .to_string()
            }
            Rung::R7 => "Unity catalyst: Unity interactions and non-dual exchanges".to_string(),
            Rung(0_u8) => "Invalid rung".to_string(),
            Rung(8_u8..=u8::MAX) => "Invalid rung".to_string(),
        }
    }

    /// Check if catalyst is being processed effectively
    pub fn is_processing_effectively(&self) -> bool {
        self.lambda.is_healthy() && self.processing_capacity > 0.5
    }

    /// Get catalyst type (positive, negative, or balanced)
    pub fn get_catalyst_type(&self) -> &'static str {
        if !self.lambda.is_healthy() {
            return "Unprocessed";
        }
        if self.lambda.value < 0.65 {
            "Balanced"
        } else if self.lambda.value < 0.75 {
            "Challenging"
        } else {
            "Intense"
        }
    }
}

impl ArchetypeTrait for CatalystBodyArchetype {
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
        ArchetypeRole::Catalyst
    }

    fn process(&mut self, catalyst: Float, _position: DevelopmentalPosition) {
        self.process_catalyst(catalyst);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaB
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
        self.processing_capacity = self.lambda.value;
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Catalyst of Body"
    }

    fn description(&self) -> &str {
        "Catalyst of Body - The processing of catalyst by the physical vehicle"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Body
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::ProcessPair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock archetype for testing
    #[allow(dead_code)]
    struct MockExperienceBody {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    #[allow(dead_code)]
    impl MockExperienceBody {
        fn new() -> Self {
            Self {
                lambda: LambdaMeasurement::new(0.7, LambdaMeasurementType::ExperienceDepth),
                tarot: TarotCorrelation::new(format!("{} - {}", "Justice", "XI")),
            }
        }
    }

    impl ArchetypeTrait for MockExperienceBody {
        fn archetype_id(&self) -> u8 {
            11
        }

        fn name(&self) -> &str {
            "Mock Experience Body"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Body
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Experience
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
            SigmaAxis::SigmaB
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
