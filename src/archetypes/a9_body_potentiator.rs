// A9: The Potentiator of Body
// Instinct and body wisdom - Body complex resource regulator
// Documentation: 01_Metaphysics/archetypes/A9_BodyPotentiator.md

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, HealthStatus as CommonHealthStatus, Holonic, HolonicLevel,
    LambdaMeasurable, LambdaMeasurement, LambdaMeasurementType, Paired, SigmaAxis,
    TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// Resource structure for Body Potentiator archetypes
///
/// Represents different types of body resources available for regulation
#[derive(Debug, Clone)]
pub struct BodyResourceStructure {
    pub active: bool,

    /// Physical vitality
    pub physical_resources: Float,
    /// Sensory capacity
    pub sensory_resources: Float,
    /// Motor coordination
    pub motor_resources: Float,
    /// Health resilience
    pub health_resources: Float,
    /// Energy resources
    pub energy_resources: Float,
    /// Instinctual wisdom
    pub instinctual_resources: Float,
}

impl BodyResourceStructure {
    /// Calculate total resources (average of all resource types)
    pub fn total_resources(&self) -> Float {
        (self.physical_resources
            + self.sensory_resources
            + self.motor_resources
            + self.health_resources
            + self.energy_resources
            + self.instinctual_resources)
            / 6.0
    }

    /// Calculate resource diversity (lower standard deviation = more balanced)
    pub fn resource_diversity(&self) -> Float {
        let resources = vec![
            self.physical_resources,
            self.sensory_resources,
            self.motor_resources,
            self.health_resources,
            self.energy_resources,
            self.instinctual_resources,
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

/// A9: The Potentiator of Body struct
///
/// The Potentiator of the Body is that which, being informed, regulates activity -
/// may be called Wisdom for it is only through judgment that the unceasing activities
/// and proclivities of the body complex may be experienced in useful modes.
#[derive(Debug, Clone)]
pub struct PotentiatorBodyArchetype {
    /// Unique archetype identifier (A9)
    pub archetype_id: u8,

    /// Lambda measurement for body potentiator wisdom and regulation
    pub lambda: LambdaMeasurement,

    /// Tarot correlation: The Hermit (IX)
    pub tarot_correlation: TarotCorrelation,

    // A9-specific properties
    /// Wisdom function - capacity to make experiences useful through judgment
    pub wisdom: Float,

    /// Judgment capacity - ability to distinguish useful from useless modes
    pub judgment: Float,

    /// Regulatory capacity - effectiveness of activity regulation
    pub regulatory_capacity: Float,

    /// Questing involvement - level of involvement in seeking new experience
    pub questing_involvement: Float,

    /// Free will choice capacity - ability to make alterations in experiential continuum
    pub free_will_choice: Float,

    /// Current developmental position
    pub developmental_position: DevelopmentalPosition,

    /// Activated rungs (R1-R7)
    pub activated_rungs: Vec<Rung>,

    /// Activation levels for each rung
    pub activation_levels: HashMap<Rung, Float>,

    /// Description of the archetype
    pub description: String,

    /// Holonic level of integration
    pub holonic_level: HolonicLevel,

    /// Integration capacity (transcend and include)
    pub integration_capacity: Float,

    /// Polarity assignment (Male after veiling)
    pub polarity: Polarity,

    // NEW: Resource structure (Phase 5)
    pub resource_structure: BodyResourceStructure,
    pub resource_quality: Float, // Quality of resources (0.0 - 1.0)
    pub resource_depth: Float,   // Depth of resources (0.0 - 1.0)
}

impl PotentiatorBodyArchetype {
    /// Create a new Potentiator of Body archetype
    pub fn new() -> Self {
        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.4); // Survival wisdom
        activation_levels.insert(Rung::R2, 0.5); // Sensation wisdom
        activation_levels.insert(Rung::R3, 0.6); // Coordination wisdom
        activation_levels.insert(Rung::R4, 0.8); // Health wisdom
        activation_levels.insert(Rung::R5, 0.8); // Integration wisdom
        activation_levels.insert(Rung::R6, 0.75); // Energy wisdom
        activation_levels.insert(Rung::R7, 0.8); // Transcendent wisdom

        // Initialize resource structure
        let resource_structure = BodyResourceStructure {
            active: true,
            physical_resources: 0.8,
            sensory_resources: 0.75,
            motor_resources: 0.78,
            health_resources: 0.8,
            energy_resources: 0.76,
            instinctual_resources: 0.77,
        };

        let resource_quality = resource_structure.total_resources();
        let resource_depth = 0.6;

        PotentiatorBodyArchetype {
            archetype_id: 9,
            lambda: LambdaMeasurement {
                value: 0.56, // Initial healthy wisdom level (0.8 * 0.85 * 0.83 ≈ 0.56)
                healthy_min: 0.5,
                healthy_max: 0.8,
                measurement_type: LambdaMeasurementType::PotentiatorAccessibility,
            },
            tarot_correlation: TarotCorrelation::new(
                "The Hermit (IX): Inner wisdom, introspection, spiritual guidance".to_string()
            ),
            wisdom: 0.8,
            judgment: 0.85,
            regulatory_capacity: 0.83,
            questing_involvement: 0.6,
            free_will_choice: 0.5,
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O4, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,
            description: "The Potentiator of the Body is that which, being informed, regulates activity - may be called Wisdom for it is only through judgment that the unceasing activities and proclivities of the body complex may be experienced in useful modes.".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.6,
            polarity: Polarity::STS, // Service to Self - male polarity assignment after veiling
            resource_structure,
            resource_quality,
            resource_depth,
        }
    }

    /// Calculate wisdom score based on judgment and regulatory capacity
    pub fn calculate_wisdom_score(&self) -> Float {
        (self.judgment * self.regulatory_capacity * self.wisdom).sqrt()
    }

    /// Calculate useful experience transformation
    pub fn calculate_useful_experience(&self, chaos_level: Float) -> Float {
        // Wisdom transforms chaos into useful experience
        let wisdom_factor = self.calculate_wisdom_score();
        chaos_level * (1.0 - wisdom_factor) + wisdom_factor * 0.8
    }

    /// Calculate questing activity (involvement in seeking new experience)
    pub fn calculate_questing(&self) -> Float {
        self.questing_involvement * self.wisdom
    }

    /// Process body activities and apply wisdom regulation
    pub fn process_activities(&mut self, activities: Vec<Float>) -> Vec<Float> {
        let wisdom_score = self.calculate_wisdom_score();
        activities
            .into_iter()
            .map(|activity| {
                // Apply wisdom judgment to transform activities into useful modes
                let regulated = activity * self.regulatory_capacity;
                let judged = if regulated > 0.5 {
                    // Useful mode - enhance with wisdom
                    regulated.min(1.0)
                } else {
                    // Chaotic mode - regulate with wisdom
                    regulated * (1.0 + wisdom_score * 0.5)
                };
                judged.clamp(0.0, 1.0)
            })
            .collect()
    }

    /// Make free will alterations in experiential continuum
    pub fn make_alterations(&mut self, continuum: Float) -> Float {
        let choice_factor = self.free_will_choice * self.wisdom;
        continuum * (1.0 + choice_factor * 0.3)
    }

    /// Update wisdom based on experience
    pub fn update_wisdom_from_experience(&mut self, experience_quality: Float) {
        // Wisdom grows from high-quality experience
        let growth = (experience_quality - 0.5) * 0.1;
        self.wisdom = (self.wisdom + growth).clamp(0.0, 1.0);
        self.judgment = (self.judgment + growth * 0.5).clamp(0.0, 1.0);
    }

    /// Calculate regulatory effectiveness against Matrix activity
    pub fn calculate_regulatory_effectiveness(&self, matrix_activity: Float) -> Float {
        // Potentiator regulates Matrix activity through wisdom
        let wisdom_score = self.calculate_wisdom_score();
        if matrix_activity > 0.8 {
            // High activity - need strong regulation
            self.regulatory_capacity * wisdom_score
        } else {
            // Balanced activity - moderate regulation
            self.regulatory_capacity * (1.0 - (0.5 - matrix_activity).abs())
        }
    }

    /// Get wisdom level description
    pub fn get_wisdom_level(&self) -> &str {
        if self.wisdom < 0.4 {
            "Poor regulation, chaotic experience, lack of wisdom"
        } else if self.wisdom < 0.6 {
            "Developing wisdom, improving regulation"
        } else if self.wisdom <= 0.8 {
            "Wise regulation, useful experience, effective judgment"
        } else {
            "Over-regulation, rigidity, lack of flexibility"
        }
    }

    /// Check if archetype is healthy
    pub fn is_healthy(&self) -> bool {
        let lambda = self.calculate_lambda();
        let (healthy_min, healthy_max) = self.healthy_range();
        lambda >= healthy_min && lambda <= healthy_max
    }

    /// Get health status
    pub fn health_status(&self) -> CommonHealthStatus {
        if self.is_healthy() {
            CommonHealthStatus::Healthy
        } else {
            // Determine if pathological low or high based on lambda value
            let lambda = self.calculate_lambda();
            let (healthy_min, _healthy_max) = self.healthy_range();
            if lambda < healthy_min {
                CommonHealthStatus::PathologicalLow
            } else {
                CommonHealthStatus::PathologicalHigh
            }
        }
    }

    /// Calculate Potentiator's regulatory intensity (Body complex)
    /// Body: Potentiator REGULATES Matrix activity (wisdom guiding motion)
    pub fn calculate_regulatory_intensity(&self) -> Float {
        // Regulatory = wisdom × regulatory_capacity
        self.wisdom * self.regulatory_capacity
    }

    // NEW: Resource availability methods (Phase 5)

    /// Calculate resource availability based on body's needs
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
        self.resource_structure = BodyResourceStructure {
            active: true,
            physical_resources: self.wisdom * 0.9,
            sensory_resources: self.judgment * 0.85,
            motor_resources: self.regulatory_capacity * 0.9,
            health_resources: self.wisdom * self.regulatory_capacity,
            energy_resources: self.questing_involvement * 0.8,
            instinctual_resources: self.free_will_choice * 0.85,
        };

        // Calculate quality and depth
        self.resource_quality = self.resource_structure.total_resources();
        self.resource_depth = self.wisdom;
    }

    /// Increase resource depth (deeper access to body wisdom)
    pub fn increase_resource_depth(&mut self, increase: Float) {
        self.resource_depth = (self.resource_depth + increase).min(1.0);
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

// LambdaMeasurable, Developmental, Paired, Holonic, PotentiatorArchetypeTrait impls follow...

// ArchetypeTrait impl is at the end of file

// Implement LambdaMeasurable
impl LambdaMeasurable for PotentiatorBodyArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        // Lambda = Wisdom × Judgment × Regulation
        self.wisdom * self.judgment * self.regulatory_capacity
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.wisdom < 0.5 {
            indicators.push("Poor regulation of bodily activity".to_string());
            indicators.push("Lack of wisdom and judgment".to_string());
            indicators.push("Chaotic bodily experience".to_string());
        }

        if self.wisdom > 0.8 {
            indicators.push("Over-regulation and rigidity".to_string());
            indicators.push("Lack of flexibility in regulation".to_string());
        }

        if self.judgment < 0.4 {
            indicators
                .push("Ineffective judgment - cannot distinguish useful from useless".to_string());
        }

        if self.regulatory_capacity < 0.5 {
            indicators.push("Poor regulatory capacity".to_string());
        }

        if self.questing_involvement < 0.3 {
            indicators.push("Lack of questing for new experience".to_string());
        }

        if self.free_will_choice < 0.3 {
            indicators.push("Limited free will choice in experiential continuum".to_string());
        }

        indicators
    }
}

// Implement Developmental
impl Developmental for PotentiatorBodyArchetype {
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

        // Update activation levels based on position's rung
        match position.rung_level() {
            1 => {
                self.activation_levels.insert(Rung::R1, 0.8);
                self.wisdom = 0.4; // Survival wisdom
            }
            2 => {
                self.activation_levels.insert(Rung::R2, 0.8);
                self.wisdom = 0.5; // Sensation wisdom
            }
            3 => {
                self.activation_levels.insert(Rung::R3, 0.8);
                self.wisdom = 0.6; // Coordination wisdom
            }
            4 => {
                self.activation_levels.insert(Rung::R4, 0.8);
                self.wisdom = 0.65; // Health wisdom
            }
            5 => {
                self.activation_levels.insert(Rung::R5, 0.8);
                self.wisdom = 0.7; // Integration wisdom
            }
            6 => {
                self.activation_levels.insert(Rung::R6, 0.8);
                self.wisdom = 0.75; // Energy wisdom
            }
            7 => {
                self.activation_levels.insert(Rung::R7, 0.8);
                self.wisdom = 0.8; // Transcendent wisdom
            }
            _ => {}
        }

        // Update activated rungs
        self.activated_rungs = self
            .activation_levels
            .iter()
            .filter(|(_, &level)| level > 0.5)
            .map(|(&rung, _)| rung)
            .collect();
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        *self.activation_levels.get(&rung).unwrap_or(&0.0)
    }
}

// Implement Paired
impl Paired for PotentiatorBodyArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(8) // Paired with A8: Matrix of Body (Structure Pair)
    }

    fn get_pair(&self) -> Option<u8> {
        Some(8)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        // Calculate tension between Potentiator and Matrix
        // Tension = |Wisdom - Matrix Activity|
        let potentiator_wisdom = self.calculate_wisdom_score();

        // Get Matrix activity level (would need to access actual Matrix archetype)
        // For now, use a placeholder calculation
        let matrix_activity = 0.7; // Placeholder

        (potentiator_wisdom - matrix_activity).abs()
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        // Calculate balance between Potentiator and Matrix
        // Balance = 1.0 - |Wisdom - Matrix Activity|
        let tension = self.calculate_pair_tension(paired_archetype);
        1.0 - tension
    }
}

// Implement Holonic
impl Holonic for PotentiatorBodyArchetype {
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
        // Transcend current level of wisdom
        let current_wisdom = self.wisdom;

        if current_wisdom >= 0.7 {
            // Can transcend to higher level
            self.holonic_level = match self.holonic_level {
                HolonicLevel::Micro => HolonicLevel::Meso,
                HolonicLevel::Meso => HolonicLevel::Macro,
                HolonicLevel::Macro => HolonicLevel::Meta,
                HolonicLevel::Meta => HolonicLevel::Meta, // Already at highest
            };

            // Increase integration capacity
            self.integration_capacity = (self.integration_capacity + 0.1).clamp(0.0, 1.0);

            true
        } else {
            false // Not enough wisdom to transcend
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include wisdom from lower level
        let lower_capacity = lower_level.integration_capacity();

        if self.integration_capacity > lower_capacity {
            // Can include lower level
            self.integration_capacity =
                (self.integration_capacity + lower_capacity * 0.1).clamp(0.0, 1.0);
            true
        } else {
            false
        }
    }
}

// ============================================================================
// POTENTIATOR ARCHETYPE TRAIT IMPLEMENTATION (Phase 10)
// ============================================================================

impl crate::archetypes::archetype_traits::PotentiatorArchetypeTrait for PotentiatorBodyArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement {
        self.lambda.clone()
    }

    fn get_resource_accessibility(&self) -> Float {
        // Body uses wisdom and regulatory capacity as proxy for accessibility
        (self.wisdom + self.regulatory_capacity) / 2.0
    }

    fn get_resource_quality(&self) -> Float {
        self.resource_quality
    }

    fn get_resource_depth(&self) -> Float {
        self.resource_depth
    }

    // Core setters
    fn set_resource_accessibility(&mut self, value: Float) {
        // Body splits accessibility between wisdom and regulatory capacity
        let clamped_value = value.clamp(0.0, 1.0);
        self.wisdom = clamped_value;
        self.regulatory_capacity = clamped_value;
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
    // Body: Potentiator provides REGULATORY INTENSITY to guide Matrix
    fn calculate_receptivity(&self) -> Float {
        // Body complex doesn't use receptivity dynamics
        0.0
    }

    fn calculate_regulatory_intensity(&self) -> Float {
        self.calculate_regulatory_intensity()
    }

    fn calculate_illumination_intensity(&self) -> Float {
        // Body complex doesn't use illumination dynamics
        0.0
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

impl ArchetypeTrait for PotentiatorBodyArchetype {
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
        ArchetypeRole::Potentiator
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        let wisdom_score = self.calculate_wisdom_score();
        self.resource_depth = (self.resource_depth + wisdom_score * 0.05).min(1.0);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaB
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
        self.wisdom = (self.wisdom + (value - 0.65) * 0.5).clamp(0.0, 1.0);
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "Potentiator of Body"
    }

    fn description(&self) -> &str {
        "Potentiator of Body - The capacity for wisdom and judgment in body activities"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Body
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::StructurePair
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_potentiator_body_creation() {
        let potentiator = PotentiatorBodyArchetype::new();

        assert_eq!(potentiator.archetype_id, 9);
        assert_eq!(potentiator.name(), "Potentiator of Body");
        assert_eq!(potentiator.complex(), ArchetypeComplex::Body);
        assert_eq!(potentiator.role(), ArchetypeRole::Potentiator);
        assert_eq!(potentiator.functional_pair(), FunctionalPair::StructurePair);
        assert_eq!(potentiator.sigma_axis(), SigmaAxis::SigmaB);
        assert!(potentiator.tarot_correlation().card.contains("IX"));
        assert!(potentiator.tarot_correlation().card.contains("The Hermit"));
    }

    #[test]
    fn test_potentiator_body_wisdom_calculation() {
        let potentiator = PotentiatorBodyArchetype::new();

        let wisdom_score = potentiator.calculate_wisdom_score();
        assert!(wisdom_score >= 0.0 && wisdom_score <= 1.0);

        // Wisdom should be based on judgment, regulation, and wisdom
        assert!(wisdom_score > 0.5); // Should be healthy
    }

    #[test]
    fn test_potentiator_body_useful_experience() {
        let potentiator = PotentiatorBodyArchetype::new();

        // Test chaotic experience transformation
        let chaotic_experience = 0.3;
        let useful = potentiator.calculate_useful_experience(chaotic_experience);

        assert!(useful > chaotic_experience); // Wisdom should improve experience
        assert!(useful <= 1.0);
    }

    #[test]
    fn test_potentiator_body_questing() {
        let potentiator = PotentiatorBodyArchetype::new();

        let questing = potentiator.calculate_questing();
        assert!(questing >= 0.0 && questing <= 1.0);

        // Questing should be based on involvement and wisdom
        assert!(questing > 0.3);
    }

    #[test]
    fn test_potentiator_body_activity_processing() {
        let mut potentiator = PotentiatorBodyArchetype::new();

        let activities = vec![0.2, 0.5, 0.8, 0.9];
        let processed = potentiator.process_activities(activities);

        assert_eq!(processed.len(), 4);

        // All activities should be regulated
        for activity in &processed {
            assert!(*activity >= 0.0 && *activity <= 1.0);
        }
    }

    #[test]
    fn test_potentiator_body_free_will_alterations() {
        let mut potentiator = PotentiatorBodyArchetype::new();

        let continuum = 0.7;
        let altered = potentiator.make_alterations(continuum);

        assert!(altered >= continuum); // Free will should alter continuum
        assert!(altered <= 1.0);
    }

    #[test]
    fn test_potentiator_body_wisdom_update() {
        let mut potentiator = PotentiatorBodyArchetype::new();
        let initial_wisdom = potentiator.wisdom;

        // Update wisdom from high-quality experience
        potentiator.update_wisdom_from_experience(0.8);

        assert!(potentiator.wisdom >= initial_wisdom); // Should grow
        assert!(potentiator.wisdom <= 1.0);

        // Judgment should also grow
        assert!(potentiator.judgment >= 0.6);
    }

    #[test]
    fn test_potentiator_body_regulatory_effectiveness() {
        let potentiator = PotentiatorBodyArchetype::new();

        // Test with high Matrix activity
        let high_activity = 0.9;
        let effectiveness = potentiator.calculate_regulatory_effectiveness(high_activity);

        assert!(effectiveness >= 0.0 && effectiveness <= 1.0);
        assert!(effectiveness > 0.3); // Should provide some regulation
    }

    #[test]
    fn test_potentiator_body_wisdom_level() {
        let potentiator = PotentiatorBodyArchetype::new();

        let level = potentiator.get_wisdom_level();
        assert!(!level.is_empty());

        // With initial wisdom of 0.65, should be in healthy range
        assert!(level.contains("Wise regulation") || level.contains("Developing wisdom"));
    }

    #[test]
    fn test_potentiator_body_lambda_measurement() {
        let potentiator = PotentiatorBodyArchetype::new();

        let lambda = potentiator.calculate_lambda();
        assert!(lambda >= 0.0 && lambda <= 1.0);

        let (healthy_min, healthy_max) = potentiator.healthy_range();
        assert_eq!(healthy_min, 0.5);
        assert_eq!(healthy_max, 0.8);

        // Initial lambda should be in healthy range
        assert!(lambda >= healthy_min && lambda <= healthy_max);
    }

    #[test]
    fn test_potentiator_body_pathological_indicators() {
        let mut potentiator = PotentiatorBodyArchetype::new();

        // Initially healthy - should have few indicators
        let indicators = potentiator.pathological_indicators();
        assert!(indicators.is_empty());

        // Simulate pathological state (low wisdom)
        potentiator.wisdom = 0.3;
        potentiator.judgment = 0.3;
        potentiator.regulatory_capacity = 0.3;

        let indicators = potentiator.pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Poor regulation")));
    }

    #[test]
    fn test_potentiator_body_developmental_progression() {
        let mut potentiator = PotentiatorBodyArchetype::new();

        // Start at R4 (Experience position: 4 % 4 = 0 = Input)
        assert_eq!(
            potentiator.developmental_position(),
            DevelopmentalPosition::Input
        );

        // Progress to R5 (Catalyst position: 5 % 4 = 1 = Catalyst)
        potentiator.update_developmental_position(DevelopmentalPosition::new_with_octant_rung(
            Octant::O5,
            5,
        ));
        assert_eq!(
            potentiator.developmental_position(),
            DevelopmentalPosition::Catalyst
        );

        // Check that R1 is activated (based on Catalyst position rung_level = 1)
        assert!(potentiator.activated_rungs().contains(&Rung::R1));

        // Check activation level for R1
        let r1_level = potentiator.rung_activation_level(Rung::R1);
        assert_eq!(r1_level, 0.8);

        // Wisdom should be set to 0.4 for Catalyst position
        assert!((potentiator.wisdom - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_potentiator_body_pair_dynamics() {
        let potentiator = PotentiatorBodyArchetype::new();

        // Check paired archetype ID
        assert_eq!(potentiator.paired_archetype_id(), Some(8));

        // Create a mock paired archetype for testing
        // In real implementation, this would be the actual A8 Body Matrix
        struct MockMatrix {
            lambda: LambdaMeasurement,
            tarot: TarotCorrelation,
        }
        impl MockMatrix {
            fn new() -> Self {
                Self {
                    lambda: LambdaMeasurement::new(0.5, LambdaMeasurementType::MatrixRigidity),
                    tarot: TarotCorrelation::new(format!("{} - {}", "Strength", "VIII")),
                }
            }
        }
        impl ArchetypeTrait for MockMatrix {
            fn archetype_id(&self) -> u8 {
                8
            }

            fn name(&self) -> &str {
                "Mock Matrix Body"
            }

            fn description(&self) -> &str {
                "Mock archetype for testing"
            }

            fn complex(&self) -> ArchetypeComplex {
                ArchetypeComplex::Body
            }

            fn role(&self) -> ArchetypeRole {
                ArchetypeRole::Matrix
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
}
