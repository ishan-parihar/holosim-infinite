// A11: The Experience of Body
// Processed output - Embodied wisdom, the Enchantress
// Tarot: Justice (XI)
// Sigma Axis: σB (Body Capacity)
// Functional Pair: Process Pair (with A10)

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, DevelopmentalPosition,
    FunctionalPair, HealthStatus, HolonicLevel, LambdaMeasurement,
    LambdaMeasurementType, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Body Bias Structure - Different types of biases created by body experience
#[derive(Debug, Clone)]
pub struct BodyBiasStructure {
    pub active: bool,

    /// How sensory perception is biased (-1.0 to 1.0)
    pub sensory_bias: Float,
    /// How motor patterns are biased (-1.0 to 1.0)
    pub motor_bias: Float,
    /// How health perception is biased (-1.0 to 1.0)
    pub health_bias: Float,
    /// How energy perception is biased (-1.0 to 1.0)
    pub energy_bias: Float,
    /// How instinctual responses are biased (-1.0 to 1.0)
    pub instinctual_bias: Float,
}

impl BodyBiasStructure {
    /// Calculate total bias (average of all bias types)
    pub fn total_bias(&self) -> Float {
        (self.sensory_bias
            + self.motor_bias
            + self.health_bias
            + self.energy_bias
            + self.instinctual_bias)
            / 5.0
    }

    /// Get bias direction (positive = integration, negative = distortion, 0 = neutral)
    pub fn bias_direction(&self) -> Float {
        let total = self.total_bias();
        if total > 0.5 {
            1.0 // Positive/integration
        } else if total < -0.5 {
            -1.0 // Negative/distortion
        } else if total.abs() < 0.1 {
            0.0 // Neutral (close to zero)
        } else {
            // Return the actual bias value for intermediate states
            total
        }
    }
}

/// Experience of Body Archetype - Processed output for Body complex
/// Represents catalyst that has been processed, called the Enchantress because it produces growth seeds
#[derive(Debug, Clone)]
pub struct ExperienceBodyArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,
    // A11-specific fields
    pub embodied_wisdom: Float,
    pub integrated_experience: Float,
    pub processing_depth: Float,
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
    pub polarity: Polarity,
    // NEW: Body bias structure
    pub bias_structure: BodyBiasStructure,
    pub bias_formation_rate: Float,
    pub bias_strength_evolution: Vec<Float>,
}

impl ExperienceBodyArchetype {
    /// Create a new Experience of Body archetype with healthy initial values
    pub fn new() -> Self {
        let mut activation_levels = HashMap::new();

        // Initialize activation levels for each rung
        // R1: Survival experience (survival experiences and learning)
        activation_levels.insert(Rung::R1, 0.5);
        // R2: Sensation experience (sensation experiences and pleasure learning)
        activation_levels.insert(Rung::R2, 0.6);
        // R3: Coordination experience (coordination experiences and skill learning)
        activation_levels.insert(Rung::R3, 0.7);
        // R4: Heart experience (relationship experiences and emotional learning)
        activation_levels.insert(Rung::R4, 0.8);
        // R5: Integral experience (integral experiences and holistic learning)
        activation_levels.insert(Rung::R5, 0.75);
        // R6: Transpersonal experience (transpersonal experiences and spiritual learning)
        activation_levels.insert(Rung::R6, 0.7);
        // R7: Unity experience (unity experiences and non-dual learning)
        activation_levels.insert(Rung::R7, 0.65);

        // Initialize body bias structure with neutral values
        let bias_structure = BodyBiasStructure {
            active: true,
            sensory_bias: 0.0,
            motor_bias: 0.0,
            health_bias: 0.0,
            energy_bias: 0.0,
            instinctual_bias: 0.0,
        };

        ExperienceBodyArchetype {
            archetype_id: 11,
            embodied_wisdom: 0.5,
            lambda: LambdaMeasurement {
                value: 0.65, // Initial healthy experience depth
                healthy_min: 0.5,
                healthy_max: 0.8,
                measurement_type: LambdaMeasurementType::ExperienceDepth,
            },
            tarot_correlation: TarotCorrelation::new("Justice (VIII): Balance, harmony, and fair processing of experience".to_string()),
            processing_depth: 0.7,
            integrated_experience: 0.65,
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O3, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,
            description: "The Experience of the Body represents the catalyst that has been processed by the mind/body/spirit complex - called the Enchantress because it produces further seed for growth. It is the result of catalyst being processed and serves as the foundation for further evolution and learning. The experience formed is unique to each complex, influenced by its biases.".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.6,
            polarity: Polarity::STO, // Service to Others - female polarity assignment after veiling
            bias_structure,
            bias_formation_rate: 0.3,
            bias_strength_evolution: Vec::new(),
        }
    }

    /// Process catalyst input and convert to experience
    pub fn process_catalyst(&mut self, catalyst_intensity: Float) {
        // Process catalyst into experience
        let processing_quality = (catalyst_intensity * self.processing_depth * 2.0).min(1.0);
        self.embodied_wisdom = (self.embodied_wisdom * 0.8 + processing_quality * 0.2).min(1.0);

        // Update lambda based on processing quality
        let new_lambda = (processing_quality + self.integrated_experience * 0.3).min(1.0);
        self.lambda.value = new_lambda;
    }

    /// Calculate experience quality score
    pub fn calculate_experience_quality(&self) -> Float {
        (self.embodied_wisdom * self.processing_depth * 2.0).min(1.0)
    }

    /// Get growth seeds produced
    pub fn get_growth_seeds(&self) -> Float {
        // Growth seeds = processed experience * enchantress quality
        (self.lambda.value * self.embodied_wisdom * 2.0).min(1.0)
    }

    /// Get enchantress quality
    pub fn get_enchantress_quality(&self) -> Float {
        self.embodied_wisdom
    }

    /// Get octant-specific expression
    pub fn get_octant_expression(&self, octant: Octant) -> String {
        match octant {
            Octant::O1 => "Personal experience: Individual physical experiences and direct learning".to_string(),
            Octant::O2 => "Interpersonal experience: One-on-one physical experiences and exchange learning".to_string(),
            Octant::O3 => "Intrapersonal experience: Individual physical experiences and bodily learning".to_string(),
            Octant::O4 => "Systemic experience: Systemic biological experiences and adaptations".to_string(),
            Octant::O5 => "Institutional experience: Institutional physical experiences and organizational learning".to_string(),
            Octant::O6 => "Communal experience: Communal physical experiences and collective learning".to_string(),
            Octant::O7 => "Behavioral experience: Behavioral experiences and skill acquisition".to_string(),
            Octant::O8 => "Transpersonal experience: Transpersonal physical experiences and universal learning".to_string(),
            Octant(0_u8) => "Invalid octant".to_string(),
            Octant(9_u8..=u8::MAX) => "Invalid octant".to_string(),
        }
    }

    /// Get rung-specific development
    pub fn get_rung_development(&self, rung: Rung) -> String {
        match rung {
            Rung::R1 => "Survival experience: Survival experiences and learning".to_string(),
            Rung::R2 => {
                "Sensation experience: Sensation experiences and pleasure learning".to_string()
            }
            Rung::R3 => {
                "Coordination experience: Coordination experiences and skill learning".to_string()
            }
            Rung::R4 => {
                "Heart experience: Relationship experiences and emotional learning".to_string()
            }
            Rung::R5 => {
                "Integral experience: Integral experiences and holistic learning".to_string()
            }
            Rung::R6 => {
                "Transpersonal experience: Transpersonal experiences and spiritual learning"
                    .to_string()
            }
            Rung::R7 => "Unity experience: Unity experiences and non-dual learning".to_string(),
            Rung(0_u8) => "Invalid rung".to_string(),
            Rung(8_u8..=u8::MAX) => "Invalid rung".to_string(),
        }
    }

    /// Check if experience is being processed effectively
    pub fn is_processing_effectively(&self) -> bool {
        self.lambda.is_healthy() && self.processing_depth > 0.5
    }

    /// Get experience type (distorted, balanced, or over-processed)
    pub fn get_experience_type(&self) -> &'static str {
        if !self.lambda.is_healthy() {
            return "Distorted";
        }
        if self.lambda.value < 0.65 {
            "Balanced"
        } else if self.lambda.value < 0.75 {
            "Enriched"
        } else {
            "Over-processed"
        }
    }

    // ============================================================================
    // PHASE 6: BODY BIAS STRUCTURE METHODS
    // ============================================================================

    /// Form continuing bias from body experience
    ///
    /// Bias formation = experience depth × bias formation rate
    /// Updates all body bias dimensions based on experience
    pub fn form_continuing_bias(&mut self, _experience: Float) {
        // Bias formation = experience depth × bias formation rate
        let bias_formation = self.integrated_experience * self.bias_formation_rate;

        // Update bias structure based on experience
        self.bias_structure.sensory_bias =
            (self.bias_structure.sensory_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.motor_bias =
            (self.bias_structure.motor_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.health_bias =
            (self.bias_structure.health_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.energy_bias =
            (self.bias_structure.energy_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.instinctual_bias =
            (self.bias_structure.instinctual_bias + bias_formation * 0.2).clamp(-1.0, 1.0);

        // Record bias strength evolution
        let total_bias = self.bias_structure.total_bias();
        self.bias_strength_evolution.push(total_bias);
        if self.bias_strength_evolution.len() > 100 {
            self.bias_strength_evolution.remove(0);
        }
    }

    /// Get bias direction (positive = integration, negative = distortion, 0 = neutral)
    pub fn get_bias_direction(&self) -> Float {
        self.bias_structure.bias_direction()
    }

    /// Get total bias (average of all bias types)
    pub fn get_total_bias(&self) -> Float {
        self.bias_structure.total_bias()
    }

    /// Get bias strength evolution history
    pub fn get_bias_strength_evolution(&self) -> &Vec<Float> {
        &self.bias_strength_evolution
    }

    /// Check if bias is positive (integration-oriented)
    pub fn is_bias_positive(&self) -> bool {
        self.get_bias_direction() > 0.0
    }

    /// Check if bias is negative (distortion-oriented)
    pub fn is_bias_negative(&self) -> bool {
        self.get_bias_direction() < 0.0
    }

    /// Check if bias is neutral
    pub fn is_bias_neutral(&self) -> bool {
        self.get_bias_direction() == 0.0
    }

    /// Get bias strength for a specific dimension
    pub fn get_bias_dimension(&self, dimension: &str) -> Float {
        match dimension {
            "sensory" => self.bias_structure.sensory_bias,
            "motor" => self.bias_structure.motor_bias,
            "health" => self.bias_structure.health_bias,
            "energy" => self.bias_structure.energy_bias,
            "instinctual" => self.bias_structure.instinctual_bias,
            _ => 0.0,
        }
    }

    /// Reset bias structure to neutral
    pub fn reset_bias(&mut self) {
        self.bias_structure.sensory_bias = 0.0;
        self.bias_structure.motor_bias = 0.0;
        self.bias_structure.health_bias = 0.0;
        self.bias_structure.energy_bias = 0.0;
        self.bias_structure.instinctual_bias = 0.0;
        self.bias_strength_evolution.clear();
    }
}

impl ArchetypeTrait for ExperienceBodyArchetype {
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
        ArchetypeRole::Experience
    }

    fn process(&mut self, catalyst: Float, _position: DevelopmentalPosition) {
        // Experience processing
        let processing = catalyst * self.processing_depth;
        self.integrated_experience += processing * 0.01;
        self.integrated_experience = self.integrated_experience.clamp(0.0, 1.0);
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
        "The Experience of Body"
    }

    fn description(&self) -> &str {
        "Experience of Body - The processing of catalyst into bodily experience"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Body
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::ExperiencePair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock archetype for testing
    struct MockCatalystBody {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockCatalystBody {
        fn new() -> Self {
            Self {
                lambda: LambdaMeasurement::new(0.65, LambdaMeasurementType::CatalystProcessingRate),
                tarot: TarotCorrelation::new("Wheel of Fortune".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockCatalystBody {
        fn archetype_id(&self) -> u8 {
            10
        }

        fn name(&self) -> &str {
            "Mock Catalyst Body"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Body
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Catalyst
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
