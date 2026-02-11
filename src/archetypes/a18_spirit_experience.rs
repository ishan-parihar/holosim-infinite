// A18: The Experience of Spirit
// Called the Moon - Processed Output for Spirit Complex
// More manifest of influences upon polarity of adept
// Shadow work, polarity determination, moonlight navigation

use crate::archetypes::archetype_traits::{
    ExperienceArchetypeTrait, MatrixArchetypeTrait, PotentiatorArchetypeTrait,
};
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Spirit Bias Structure - Different types of biases created by spirit experience
#[derive(Debug, Clone)]
pub struct SpiritBiasStructure {
    pub active: bool,

    /// How discernment is biased (-1.0 to 1.0)
    pub discernment_bias: Float,
    /// How shadow work is biased (-1.0 to 1.0)
    pub shadow_work_bias: Float,
    /// How light descending is biased (-1.0 to 1.0)
    pub light_descending_bias: Float,
    /// How polarity development is biased (-1.0 to 1.0)
    pub polarity_bias: Float,
    /// How moonlight navigation is biased (-1.0 to 1.0)
    pub moonlight_bias: Float,
}

impl SpiritBiasStructure {
    /// Calculate total bias (average of all bias types)
    pub fn total_bias(&self) -> Float {
        (self.discernment_bias
            + self.shadow_work_bias
            + self.light_descending_bias
            + self.polarity_bias
            + self.moonlight_bias)
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

#[derive(Debug, Clone)]
pub struct ExperienceSpiritArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,
    // A18-specific fields
    pub discernment: Float, // How well is truth discerned from falsity (0.0-1.0)
    pub shadow_work_effectiveness: Float, // How effectively are unhappy experiences worked with (0.0-1.0)
    pub light_descending_effectiveness: Float, // How effectively does light descend (0.0-1.0)
    pub polarity_development: Float,      // How well is polarity developed (0.0-1.0)
    pub moonlight_discernment: Float,     // Discernment in moonlight (0.0-1.0)
    pub positive_illumination: Float,     // Positive or service-to-others illumination (0.0-1.0)
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
    pub polarity: Polarity,
    // NEW: Spirit bias structure
    pub bias_structure: SpiritBiasStructure,
    pub bias_formation_rate: Float,
    pub bias_strength_evolution: Vec<Float>,
}

impl ExperienceSpiritArchetype {
    pub fn new() -> Self {
        // Initialize spirit bias structure with neutral values
        let bias_structure = SpiritBiasStructure {
            active: true,
            discernment_bias: 0.0,
            shadow_work_bias: 0.0,
            light_descending_bias: 0.0,
            polarity_bias: 0.0,
            moonlight_bias: 0.0,
        };

        ExperienceSpiritArchetype {
            archetype_id: 18,
            lambda: LambdaMeasurement::new(0.65, LambdaMeasurementType::ExperienceDepth),
            tarot_correlation: TarotCorrelation::new(format!("The Moon (XVIII): Illusion, fear, anxiety, subconscious, intuition, deception, hidden things, shadow work")),
            discernment: 0.65,                   // Healthy discernment level
            shadow_work_effectiveness: 0.60,     // Good shadow work effectiveness
            light_descending_effectiveness: 0.65, // Good light descending
            polarity_development: 0.60,          // Developing polarity
            moonlight_discernment: 0.55,         // Moderate moonlight discernment
            positive_illumination: 0.60,         // Positive illumination developing
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O1, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels: {
                let mut levels = HashMap::new();
                levels.insert(Rung::R1, 0.7);
                levels.insert(Rung::R2, 0.75);
                levels.insert(Rung::R3, 0.8);
                levels.insert(Rung::R4, 0.65);
                levels
            },
            description: "Experience of Spirit - Called the Moon. More manifest of influences upon polarity of adept. Even the most unhappy experiences may be worked with until light descends. Moonlight offers either true picture or chimera. Shadow work, polarity determination, discernment.".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.65,
            polarity: Polarity::STS, // Male polarity assignment after veiling
            bias_structure,
            bias_formation_rate: 0.3,
            bias_strength_evolution: Vec::new(),
        }
    }

    // Calculate shadow navigation score
    pub fn calculate_shadow_navigation_score(&self) -> Float {
        let discernment_component = self.discernment * 0.35;
        let light_descending_component = self.light_descending_effectiveness * 0.35;
        let shadow_work_component = self.shadow_work_effectiveness * 0.30;
        discernment_component + light_descending_component + shadow_work_component
    }

    // Calculate polarity development score
    pub fn calculate_polarity_development_score(&self) -> Float {
        let polarity_component = self.polarity_development * 0.4;
        let illumination_component = self.positive_illumination * 0.3;
        let discernment_component = self.moonlight_discernment * 0.3;
        polarity_component + illumination_component + discernment_component
    }

    // Update discernment based on moonlight and shadow work
    pub fn update_discernment(&mut self, catalyst_input: Float) {
        // Discernment improves with moonlight work and shadow processing
        let moonlight_factor = self.moonlight_discernment * 0.4;
        let shadow_work_factor = self.shadow_work_effectiveness * 0.35;
        let catalyst_factor = catalyst_input * 0.25;

        let target_discernment = moonlight_factor + shadow_work_factor + catalyst_factor;
        self.discernment = self.discernment * 0.7 + target_discernment * 0.3;
        self.discernment = self.discernment.max(0.0).min(1.0);
    }

    // Update shadow work effectiveness
    pub fn update_shadow_work_effectiveness(&mut self, unhappy_experiences: Float) {
        // Shadow work effectiveness improves with practice and discrimination
        let practice_factor = self.shadow_work_effectiveness * 0.3;
        let discrimination_factor = self.discernment * 0.4;
        let experience_factor = unhappy_experiences * 0.3;

        let target_effectiveness = practice_factor + discrimination_factor + experience_factor;
        self.shadow_work_effectiveness =
            self.shadow_work_effectiveness * 0.65 + target_effectiveness * 0.35;
        self.shadow_work_effectiveness = self.shadow_work_effectiveness.max(0.0).min(1.0);
    }

    // Update light descending effectiveness
    pub fn update_light_descending_effectiveness(&mut self, illumination_factor: Float) {
        // Light descending improves with positive illumination and polarity development
        let illumination_component = self.positive_illumination * 0.4;
        let polarity_component = self.polarity_development * 0.35;
        let input_factor = illumination_factor * 0.25;

        let target_effectiveness = illumination_component + polarity_component + input_factor;
        self.light_descending_effectiveness =
            self.light_descending_effectiveness * 0.7 + target_effectiveness * 0.3;
        self.light_descending_effectiveness = self.light_descending_effectiveness.max(0.0).min(1.0);
    }

    // Update polarity development
    pub fn update_polarity_development(&mut self, catalyst_input: Float) {
        // Polarity development requires shadow work and light descending
        let shadow_work_component = self.shadow_work_effectiveness * 0.35;
        let light_component = self.light_descending_effectiveness * 0.35;
        let catalyst_component = catalyst_input * 0.30;

        let target_development = shadow_work_component + light_component + catalyst_component;
        self.polarity_development = self.polarity_development * 0.68 + target_development * 0.32;
        self.polarity_development = self.polarity_development.max(0.0).min(1.0);
    }

    // Update moonlight discernment
    pub fn update_moonlight_discernment(&mut self, deception_level: Float) {
        // Moonlight discernment requires navigating deception
        let base_discernment = self.discernment * 0.4;
        let deception_navigated = (1.0 - deception_level) * 0.35;
        let shadow_component = self.shadow_work_effectiveness * 0.25;

        let target_discernment = base_discernment + deception_navigated + shadow_component;
        self.moonlight_discernment = self.moonlight_discernment * 0.65 + target_discernment * 0.35;
        self.moonlight_discernment = self.moonlight_discernment.max(0.0).min(1.0);
    }

    // Update positive illumination
    pub fn update_positive_illumination(&mut self, light_descending: Float) {
        // Positive illumination requires light descending and polarity development
        let light_component = light_descending * 0.45;
        let polarity_component = self.polarity_development * 0.35;
        let shadow_component = self.shadow_work_effectiveness * 0.20;

        let target_illumination = light_component + polarity_component + shadow_component;
        self.positive_illumination = self.positive_illumination * 0.7 + target_illumination * 0.3;
        self.positive_illumination = self.positive_illumination.max(0.0).min(1.0);
    }

    // Get diagnostic information
    pub fn get_diagnostic_info(&self) -> String {
        format!(
            "Experience of Spirit (A18) Diagnostic:\n\
             - Discernment: {:.2}\n\
             - Shadow Work Effectiveness: {:.2}\n\
             - Light Descending Effectiveness: {:.2}\n\
             - Polarity Development: {:.2}\n\
             - Moonlight Discernment: {:.2}\n\
             - Positive Illumination: {:.2}\n\
             - Shadow Navigation Score: {:.2}\n\
             - Polarity Development Score: {:.2}\n\
             - Lambda: {:.2}\n\
             - Health Status: {:?}",
            self.discernment,
            self.shadow_work_effectiveness,
            self.light_descending_effectiveness,
            self.polarity_development,
            self.moonlight_discernment,
            self.positive_illumination,
            self.calculate_shadow_navigation_score(),
            self.calculate_polarity_development_score(),
            self.lambda.value,
            self.lambda.health_status()
        )
    }

    // Check if adept is groping in moonlight (pathological state)
    pub fn is_groping_in_moonlight(&self) -> bool {
        self.discernment < 0.5 && self.moonlight_discernment < 0.5
    }

    // Check if adept is satisfied with shadows (negative path)
    pub fn is_satisfied_with_shadows(&self) -> bool {
        self.shadow_work_effectiveness < 0.4 && self.positive_illumination < 0.4
    }

    // Check if adept has grasped light of sun (positive path)
    pub fn has_grasped_light_of_sun(&self) -> bool {
        self.positive_illumination >= 0.7 && self.light_descending_effectiveness >= 0.7
    }

    // Update integration capacity based on state
    pub fn update_integration_capacity(&mut self) {
        // Integration capacity based on shadow navigation and polarity development
        let shadow_nav = self.calculate_shadow_navigation_score() * 0.5;
        let polarity_dev = self.calculate_polarity_development_score() * 0.5;
        self.integration_capacity = shadow_nav + polarity_dev;
        self.integration_capacity = self.integration_capacity.max(0.0).min(1.0);
    }

    // ============================================================================
    // PHASE 6: SPIRIT BIAS STRUCTURE METHODS
    // ============================================================================

    /// Form continuing bias from spirit experience
    ///
    /// Bias formation = experience depth × bias formation rate
    /// Updates all spirit bias dimensions based on experience
    pub fn form_continuing_bias(&mut self, _experience: Float) {
        // Bias formation = polarity development × bias formation rate
        let bias_formation = self.polarity_development * self.bias_formation_rate;

        // Update bias structure based on experience
        self.bias_structure.discernment_bias =
            (self.bias_structure.discernment_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.shadow_work_bias =
            (self.bias_structure.shadow_work_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.light_descending_bias =
            (self.bias_structure.light_descending_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.polarity_bias =
            (self.bias_structure.polarity_bias + bias_formation * 0.2).clamp(-1.0, 1.0);
        self.bias_structure.moonlight_bias =
            (self.bias_structure.moonlight_bias + bias_formation * 0.2).clamp(-1.0, 1.0);

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
            "discernment" => self.bias_structure.discernment_bias,
            "shadow_work" => self.bias_structure.shadow_work_bias,
            "light_descending" => self.bias_structure.light_descending_bias,
            "polarity" => self.bias_structure.polarity_bias,
            "moonlight" => self.bias_structure.moonlight_bias,
            _ => 0.0,
        }
    }

    /// Reset bias structure to neutral
    pub fn reset_bias(&mut self) {
        self.bias_structure.discernment_bias = 0.0;
        self.bias_structure.shadow_work_bias = 0.0;
        self.bias_structure.light_descending_bias = 0.0;
        self.bias_structure.polarity_bias = 0.0;
        self.bias_structure.moonlight_bias = 0.0;
    }
}

impl LambdaMeasurable for ExperienceSpiritArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        // Lambda = Discernment × Shadow Work × Light Descending
        let discernment_component = self.discernment * 0.35;
        let shadow_work_component = self.shadow_work_effectiveness * 0.35;
        let light_descending_component = self.light_descending_effectiveness * 0.30;

        discernment_component + shadow_work_component + light_descending_component
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.discernment < 0.5 {
            indicators.push("Low discernment - deception likely".to_string());
        }
        if self.shadow_work_effectiveness < 0.5 {
            indicators.push("Poor shadow work - stuck in shadows".to_string());
        }
        if self.light_descending_effectiveness < 0.5 {
            indicators.push("Light not descending - no illumination".to_string());
        }
        if self.polarity_development < 0.5 {
            indicators.push("Poor polarity development".to_string());
        }
        if self.moonlight_discernment < 0.5 {
            indicators.push("Low moonlight discernment - groping in moonlight".to_string());
        }
        if self.positive_illumination < 0.5 {
            indicators.push("Low positive illumination - negative satisfaction".to_string());
        }

        if indicators.is_empty() {
            indicators.push("All indicators within healthy range".to_string());
        }

        indicators
    }
}

impl Developmental for ExperienceSpiritArchetype {
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

impl Paired for ExperienceSpiritArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(17) // Paired with A17 Catalyst of Spirit
    }

    fn get_pair(&self) -> Option<u8> {
        Some(17)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, paired_archetype: &dyn Paired) -> Float {
        let paired_lambda = paired_archetype.lambda().value;
        // Tension = difference between experience and catalyst
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

impl Holonic for ExperienceSpiritArchetype {
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
        // Transcend to higher holonic level requires strong shadow navigation and polarity development
        let shadow_nav = self.calculate_shadow_navigation_score();
        let polarity_dev = self.calculate_polarity_development_score();

        if shadow_nav >= 0.75 && polarity_dev >= 0.75 {
            match self.holonic_level {
                HolonicLevel::Micro => {
                    self.holonic_level = HolonicLevel::Meso;
                    true
                }
                HolonicLevel::Meso => {
                    self.holonic_level = HolonicLevel::Macro;
                    true
                }
                HolonicLevel::Macro => {
                    self.holonic_level = HolonicLevel::Meta;
                    true
                }
                HolonicLevel::Meta => false, // Already at highest level
            }
        } else {
            false
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include lower level by integrating its experience
        let lower_capacity = lower_level.integration_capacity();
        let current_capacity = self.integration_capacity;

        // Integration improves when including lower levels
        let improvement = lower_capacity * 0.2;
        self.integration_capacity = (current_capacity + improvement).min(1.0);

        // Update A18-specific fields based on integration
        self.shadow_work_effectiveness =
            (self.shadow_work_effectiveness + improvement * 0.3).min(1.0);
        self.light_descending_effectiveness =
            (self.light_descending_effectiveness + improvement * 0.3).min(1.0);
        self.polarity_development = (self.polarity_development + improvement * 0.4).min(1.0);

        true
    }
}

// ============================================================================
// EXPERIENCE ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl ExperienceArchetypeTrait for ExperienceSpiritArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_experience_depth(&self) -> Float {
        // Map discernment to experience_depth
        self.discernment
    }

    fn get_integration_quality(&self) -> Float {
        // Map shadow_work_effectiveness to integration_quality
        self.shadow_work_effectiveness
    }

    fn get_continuing_bias_strength(&self) -> Float {
        // Calculate from bias_structure
        self.bias_structure.total_bias().abs()
    }

    // Core setters
    fn set_experience_depth(&mut self, value: Float) {
        self.discernment = value.clamp(0.0, 1.0);
        self.lambda.value = (value.clamp(0.5, 0.8) + value.clamp(0.5, 0.8)) / 2.0;
    }

    fn set_integration_quality(&mut self, value: Float) {
        self.shadow_work_effectiveness = value.clamp(0.0, 1.0);
    }

    fn set_continuing_bias_strength(&mut self, value: Float) {
        // Update bias structure to reflect new strength
        let current_total = self.bias_structure.total_bias();
        if current_total > 0.0 {
            let scale = value / current_total;
            self.bias_structure.discernment_bias *= scale;
            self.bias_structure.shadow_work_bias *= scale;
            self.bias_structure.light_descending_bias *= scale;
            self.bias_structure.polarity_bias *= scale;
            self.bias_structure.moonlight_bias *= scale;
        }
    }

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position.clone()
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

    // Bias structure methods (already exist)
    fn form_continuing_bias(&mut self, experience: Float) {
        ExperienceSpiritArchetype::form_continuing_bias(self, experience);
    }

    fn get_bias_direction(&self) -> Float {
        ExperienceSpiritArchetype::get_bias_direction(self)
    }

    fn apply_bias_to_matrix(&self, matrix: &mut Box<dyn MatrixArchetypeTrait>) {
        // Apply discernment bias to structural permeability
        let permeability_modifier = 1.0 + self.bias_structure.discernment_bias * 0.1;
        matrix.set_structural_permeability(
            matrix.get_structural_permeability() * permeability_modifier,
        );

        // Apply shadow work bias to structural tension
        let _tension_modifier = 1.0 + self.bias_structure.shadow_work_bias * 0.1;
        // Note: MatrixArchetypeTrait doesn't have set_structural_tension, but might need this
        // For now, we'll skip this modification

        // Apply light descending bias to integration capacity
        let capacity_modifier = 1.0 + self.bias_structure.light_descending_bias * 0.1;
        matrix.set_integration_capacity(matrix.get_integration_capacity() * capacity_modifier);

        // Apply polarity bias to resource access
        let access_modifier = 1.0 + self.bias_structure.polarity_bias * 0.1;
        matrix.set_resource_access(matrix.get_resource_access() * access_modifier);

        // Apply moonlight bias to conscious coherence
        let coherence_modifier = 1.0 + self.bias_structure.moonlight_bias * 0.1;
        matrix.set_conscious_coherence(matrix.get_conscious_coherence() * coherence_modifier);
    }

    fn apply_bias_to_potentiator(&self, potentiator: &mut Box<dyn PotentiatorArchetypeTrait>) {
        // Positive bias opens new Potentiator access
        if self.get_bias_direction() > 0.0 {
            let access_increase = self.get_continuing_bias_strength() * 0.1;
            potentiator.increase_resource_depth(access_increase);
        }
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        // Map common HealthStatus to complex HealthStatus
        let common_health = ExperienceSpiritArchetype::health_status(self);
        match common_health {
            HealthStatus::Healthy => HealthStatus::Healthy,
            HealthStatus::PathologicalLow => HealthStatus::Degraded,
            HealthStatus::PathologicalHigh => HealthStatus::Pathological,
            _ => HealthStatus::Warning,
        }
    }

    // Experience management methods (used by complex.rs)
    fn store_experiences(&mut self, _experiences: Vec<crate::complex::Experience>) {
        // Experiences are managed internally through experience_depth
        // This is a no-op for this implementation
    }

    fn get_bias(&self, rung: Rung) -> Float {
        // Return bias based on bias_structure and rung level
        let rung_level = rung.value() as Float / 7.0;
        // Calculate overall bias strength from bias_structure
        let bias_strength = (self.bias_structure.discernment_bias.abs()
            + self.bias_structure.shadow_work_bias.abs()
            + self.bias_structure.light_descending_bias.abs()
            + self.bias_structure.polarity_bias.abs()
            + self.bias_structure.moonlight_bias.abs())
            / 5.0;
        bias_strength * rung_level
    }
}

impl ArchetypeTrait for ExperienceSpiritArchetype {
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
        ArchetypeRole::Experience
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        let position_value = position.rung_level() as Float / 7.0;
        self.lambda.adjust(catalyst * position_value);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaA
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation().clone()
    }

    fn update_lambda(&mut self, delta: Float) {
        self.lambda.adjust(delta);
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Experience of Spirit"
    }

    fn description(&self) -> &str {
        "Experience of Spirit - Experience storage in Spirit complex"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::ExperiencePair
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
    struct MockCatalystSpirit {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockCatalystSpirit {
        fn new(lambda_value: Float) -> Self {
            MockCatalystSpirit {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::CatalystProcessingRate,
                ),
                tarot: TarotCorrelation::new("The Star".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockCatalystSpirit {
        fn archetype_id(&self) -> u8 {
            17
        }

        fn name(&self) -> &str {
            "Mock Catalyst Spirit"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
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
            SigmaAxis::SigmaC
        }

        fn tarot_correlation(&self) -> TarotCorrelation {
            self.tarot_correlation().clone()
        }

        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.clamp(0.0, 1.0);
        }

        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::ProcessPair
        }
    }
}
