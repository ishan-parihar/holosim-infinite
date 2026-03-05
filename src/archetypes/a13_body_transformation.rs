// A13: The Transformation of Body
// Archetype 13 - Mutation Event: Death and Rebirth
//
// The Transformation of the Body is called Death, for with death the body is
// transformed to a higher-vibration body for additional learning. Each moment
// and each diurnal period of bodily incarnation offers death and rebirth to one
// attempting to use catalyst offered.

use crate::archetypes::archetype_traits::TransformationArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// A13: The Transformation of Body
///
/// Mutation Event - Death and Rebirth
///
/// With death the body is transformed to a higher-vibration body for additional
/// learning. Each moment and each diurnal period of bodily incarnation offers death
/// and rebirth to one attempting to use catalyst offered.
#[derive(Debug, Clone)]
pub struct TransformationBodyArchetype {
    /// Archetype ID (13)
    pub archetype_id: u8,
    pub active: bool,
    /// Lambda measurement - transformation velocity
    pub lambda: LambdaMeasurement,
    /// Tarot correlation - Death (XIII)
    pub tarot_correlation: TarotCorrelation,
    /// Transformation velocity (0.0 - 1.0)
    pub transformation_velocity: Float,
    /// Death and rebirth cycle (0.0 - 1.0)
    pub death_rebirth_cycle: Float,
    /// Higher-vibration body access (0.0 - 1.0)
    pub higher_vibration_access: Float,
    /// Catalyst use effectiveness (0.0 - 1.0)
    pub catalyst_use_effectiveness: Float,
    /// Developmental position in sigma network
    pub developmental_position: DevelopmentalPosition,
    /// Activated rungs
    pub activated_rungs: Vec<Rung>,
    /// Activation levels for each rung
    pub activation_levels: HashMap<Rung, Float>,
    /// Archetype description
    pub description: String,
    /// Holonic level
    pub holonic_level: HolonicLevel,
    /// Integration capacity (0.0 - 1.0)
    pub integration_capacity: Float,
    /// Polarity (STS - male assignment after veiling)
    pub polarity: Polarity,

    /// Progress toward chosen polarization (0.0 - 1.0)
    pub polarization_progress: Float,

    /// Whether choice betwixt light and dark has been made
    pub choice_made: bool,

    // Phase 7: Parameter modification capabilities
    pub can_modify_veil: bool,
    pub can_modify_capacity: bool,
    pub can_modify_efficiency: bool,
    pub can_modify_tolerance: bool,
}

impl Default for TransformationBodyArchetype {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformationBodyArchetype {
    /// Create a new Transformation of Body archetype with healthy initial values
    pub fn new() -> Self {
        let healthy_lambda = 0.65; // Within healthy range (0.5 - 0.8)

        TransformationBodyArchetype {
            archetype_id: 13,
            active: true,
            lambda: LambdaMeasurement::new(
                healthy_lambda,
                LambdaMeasurementType::TransformationVelocity,
            ),
            tarot_correlation: TarotCorrelation::new("Death (XIII): Transformation, death and rebirth, change, transition".to_string()),
            transformation_velocity: healthy_lambda,
            death_rebirth_cycle: 0.65,
            higher_vibration_access: 0.65,
            catalyst_use_effectiveness: 0.65,
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels: {
                let mut levels = HashMap::new();
                levels.insert(Rung::R1, 0.7);
                levels.insert(Rung::R2, 0.65);
                levels.insert(Rung::R3, 0.6);
                levels.insert(Rung::R4, 0.65);
                levels.insert(Rung::R5, 0.0);
                levels.insert(Rung::R6, 0.0);
                levels.insert(Rung::R7, 0.0);
                levels
            },
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O4, 4),
            description: String::from(
                "The Transformation of the Body is called Death, for with death the body is \
                 transformed to a higher-vibration body for additional learning. Each moment \
                 and each diurnal period of bodily incarnation offers death and rebirth to one \
                 attempting to use catalyst offered.",
            ),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.65,
            polarity: Polarity::STS, // Male polarity assignment after veiling
            polarization_progress: 0.0,
            choice_made: false,

            // Phase 7: Initialize parameter modification capabilities
            can_modify_veil: true,
            can_modify_capacity: true,
            can_modify_efficiency: true,
            can_modify_tolerance: true,
        }
    }

    /// Calculate transformation capacity score
    pub fn calculate_transformation_capacity(&self) -> Float {
        let death = self.death_rebirth_cycle;
        let rebirth = self.transformation_velocity;
        let transformation = self.lambda.value;

        if transformation > 0.0 {
            (death * rebirth) / transformation
        } else {
            0.0
        }
    }

    /// Process death and rebirth cycle
    pub fn process_death_rebirth(&mut self, catalyst_intensity: Float) {
        // Death and rebirth cycle is enhanced by catalyst use
        let cycle_improvement = catalyst_intensity * 0.1;
        self.death_rebirth_cycle = (self.death_rebirth_cycle + cycle_improvement).min(1.0);

        // Update lambda based on death/rebirth cycle
        let target_lambda = (self.death_rebirth_cycle + self.transformation_velocity) / 2.0;
        self.lambda.value = target_lambda;
    }

    /// Access higher-vibration body state
    pub fn access_higher_vibration(&mut self) -> Float {
        // Higher-vibration access is based on transformation velocity
        self.higher_vibration_access = self.transformation_velocity * 0.9;

        // Update lambda to reflect higher-vibration access
        self.lambda.value = (self.lambda.value + self.higher_vibration_access) / 2.0;

        self.higher_vibration_access
    }

    /// Use catalyst for transformation
    pub fn use_catalyst(&mut self, catalyst_intensity: Float) {
        // Catalyst use effectiveness improves with active engagement
        let improvement = catalyst_intensity * 0.15;
        self.catalyst_use_effectiveness = (self.catalyst_use_effectiveness + improvement).min(1.0);

        // Transformation velocity is enhanced by catalyst use
        self.transformation_velocity =
            (self.transformation_velocity + catalyst_intensity * 0.1).min(1.0);

        // Update lambda
        self.lambda.value = (self.transformation_velocity + self.death_rebirth_cycle) / 2.0;
    }

    /// Check if transformation is continual
    pub fn is_continual(&self) -> bool {
        self.death_rebirth_cycle > 0.5 && self.transformation_velocity > 0.5
    }

    /// Check if there's fear of transformation
    pub fn has_fear_of_transformation(&self) -> bool {
        self.lambda.value < 0.5 || self.death_rebirth_cycle < 0.5
    }

    /// Get transformation status
    pub fn transformation_status(&self) -> String {
        if self.lambda.value < 0.5 {
            String::from("Stagnation - fear of transformation, ineffective rebirth")
        } else if self.lambda.value > 0.8 {
            String::from("Over-transformation - lack of stability, constant upheaval")
        } else {
            String::from("Continual transformation - effective rebirth, higher-vibration access")
        }
    }

    // ============================================================================
    // PHASE 7: PARAMETER MODIFICATION METHODS (Greater Cycle Regulation)
    // ============================================================================

    /// Modify veil permeability based on transformation velocity
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_veil_permeability(&self, current_permeability: Float) -> Float {
        if !self.can_modify_veil {
            return current_permeability;
        }

        // Transformation velocity affects veil permeability
        // Higher transformation velocity → veil lifts (permeability increases)
        let modification = self.transformation_velocity * 0.1;
        (current_permeability + modification).clamp(0.0, 1.0)
    }

    /// Modify processing capacity based on transformation velocity
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_processing_capacity(&self, current_capacity: Float) -> Float {
        if !self.can_modify_capacity {
            return current_capacity;
        }

        // Higher transformation velocity → capacity increases
        let modification = self.transformation_velocity * 0.05;
        (current_capacity + modification).clamp(0.1, 1.0)
    }

    /// Modify processing efficiency based on transformation velocity
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_processing_efficiency(&self, current_efficiency: Float) -> Float {
        if !self.can_modify_efficiency {
            return current_efficiency;
        }

        // Higher transformation velocity → efficiency increases
        let modification = self.transformation_velocity * 0.08;
        (current_efficiency + modification).clamp(0.1, 1.0)
    }

    /// Modify catalyst tolerance based on transformation velocity
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_catalyst_tolerance(&self, current_tolerance: Float) -> Float {
        if !self.can_modify_tolerance {
            return current_tolerance;
        }

        // Higher transformation velocity → tolerance increases
        let modification = self.transformation_velocity * 0.1;
        (current_tolerance + modification).clamp(0.1, 1.0)
    }

    /// Check if transformation can modify parameters
    pub fn can_modify_parameters(&self) -> bool {
        self.can_modify_veil
            || self.can_modify_capacity
            || self.can_modify_efficiency
            || self.can_modify_tolerance
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

impl LambdaMeasurable for TransformationBodyArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        let capacity = self.transformation_velocity;
        let rebirth = self.death_rebirth_cycle;
        let higher_vibration = self.higher_vibration_access;

        // Lambda is weighted average of transformation components
        (capacity * 0.4 + rebirth * 0.3 + higher_vibration * 0.3).min(1.0)
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.value < 0.5 {
            indicators.push(String::from("Stagnation - lack of transformation"));
            indicators.push(String::from("Fear of death/transformation"));
            indicators.push(String::from("Ineffective rebirth process"));
        }

        if self.lambda.value > 0.8 {
            indicators.push(String::from("Over-transformation - constant upheaval"));
            indicators.push(String::from("Lack of stability"));
            indicators.push(String::from("Excessive transformation velocity"));
        }

        if self.catalyst_use_effectiveness < 0.5 {
            indicators.push(String::from(
                "Passive catalyst use - less effective transformation",
            ));
        }

        if self.higher_vibration_access < 0.5 {
            indicators.push(String::from("Limited access to higher-vibration states"));
        }

        indicators
    }
}

impl Developmental for TransformationBodyArchetype {
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

        // Update activation levels based on new rung
        let rung_level = position.rung_level();
        for r in 1..=7u8 {
            let rung = match r {
                1 => Rung::R1,
                2 => Rung::R2,
                3 => Rung::R3,
                4 => Rung::R4,
                5 => Rung::R5,
                6 => Rung::R6,
                7 => Rung::R7,
                _ => continue,
            };

            let activation = if r <= rung_level {
                (r as Float) / (rung_level as Float)
            } else {
                0.0
            };

            self.activation_levels.insert(rung, activation);
        }

        // Update activated rungs
        self.activated_rungs = (1..=rung_level)
            .map(|r| match r {
                1 => Rung::R1,
                2 => Rung::R2,
                3 => Rung::R3,
                4 => Rung::R4,
                5 => Rung::R5,
                6 => Rung::R6,
                7 => Rung::R7,
                _ => Rung::R1,
            })
            .collect();
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        *self.activation_levels.get(&rung).unwrap_or(&0.0)
    }
}

impl Paired for TransformationBodyArchetype {
    fn get_pair(&self) -> Option<u8> {
        // Transformation singleton connects all pairs
        // Primary connection is to A6 (Transformation of Mind)
        Some(6)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn paired_archetype_id(&self) -> Option<u8> {
        // Transformation singleton connects all pairs
        // Primary connection is to A6 (Transformation of Mind)
        Some(6)
    }

    fn calculate_pair_tension(&self, paired_archetype: &dyn Paired) -> Float {
        let paired_lambda = paired_archetype.lambda().value;
        let self_lambda = self.lambda.value;

        // Tension is absolute difference between lambda values
        (self_lambda - paired_lambda).abs()
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        let paired_lambda = paired_archetype.lambda().value;
        let self_lambda = self.lambda.value;

        // Balance is ratio of lambda values
        if paired_lambda > 0.0 {
            self_lambda / paired_lambda
        } else {
            0.0
        }
    }
}

impl Holonic for TransformationBodyArchetype {
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
        // Transcend to next holonic level
        let current_level = self.holonic_level as u8;
        let next_level = current_level + 1;

        if next_level <= 3 {
            // Can transcend (Micro->Meso->Macro->Meta)
            self.holonic_level = match next_level {
                0 => HolonicLevel::Micro,
                1 => HolonicLevel::Meso,
                2 => HolonicLevel::Macro,
                3 => HolonicLevel::Meta,
                _ => HolonicLevel::Meta,
            };

            // Increase integration capacity
            self.integration_capacity = (self.integration_capacity + 0.1).min(1.0);

            true
        } else {
            false
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include lower holonic level
        let lower_integration = lower_level.integration_capacity();

        // Integration capacity is increased by including lower level
        self.integration_capacity = (self.integration_capacity + lower_integration * 0.5).min(1.0);

        true
    }
}

// ============================================================================
// TRANSFORMATION ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl TransformationArchetypeTrait for TransformationBodyArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement {
        self.lambda.clone()
    }

    fn get_transformation_velocity(&self) -> Float {
        self.transformation_velocity
    }

    fn get_transformation_direction(&self) -> Polarity {
        self.polarity
    }

    fn get_polarization_progress(&self) -> Float {
        self.polarization_progress
    }

    fn get_choice_made(&self) -> bool {
        self.choice_made
    }

    // Core setters
    fn set_transformation_velocity(&mut self, value: Float) {
        self.transformation_velocity = value.clamp(0.0, 1.0);
    }

    fn set_transformation_direction(&mut self, direction: Polarity) {
        self.polarity = direction;
    }

    fn set_polarization_progress(&mut self, value: Float) {
        self.polarization_progress = value.clamp(0.0, 1.0);
    }

    fn set_choice_made(&mut self, value: bool) {
        self.choice_made = value;
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

    // Phase 7: Parameter modification
    fn modify_veil_permeability(&self, current_permeability: Float) -> Float {
        if !self.can_modify_veil {
            return current_permeability;
        }

        // Transformation direction affects veil
        // STS (male) → veil lifts through state transformation
        let direction_factor = match self.polarity {
            Polarity::STS => 1.0,
            Polarity::STO => 1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.1;
        (current_permeability + modification).clamp(0.0, 1.0)
    }

    fn modify_processing_capacity(&self, current_capacity: Float) -> Float {
        if !self.can_modify_capacity {
            return current_capacity;
        }

        // STS transformation → capacity increases through state transformation
        // STO transformation → capacity increases through structural transformation
        let direction_factor = match self.polarity {
            Polarity::STS => 1.0,
            Polarity::STO => 1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.05;
        (current_capacity + modification).clamp(0.1, 1.0)
    }

    fn modify_processing_efficiency(&self, current_efficiency: Float) -> Float {
        if !self.can_modify_efficiency {
            return current_efficiency;
        }

        // STS transformation → efficiency increases through state transformation
        // STO transformation → efficiency increases through structural transformation
        let direction_factor = match self.polarity {
            Polarity::STS => 1.0,
            Polarity::STO => 1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.08;
        (current_efficiency + modification).clamp(0.1, 1.0)
    }

    fn modify_catalyst_tolerance(&self, current_tolerance: Float) -> Float {
        if !self.can_modify_tolerance {
            return current_tolerance;
        }

        // STS transformation → tolerance increases through state transformation
        // STO transformation → tolerance increases through structural transformation
        let direction_factor = match self.polarity {
            Polarity::STS => 1.0,
            Polarity::STO => 1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.1;
        (current_tolerance + modification).clamp(0.1, 1.0)
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        // Calculate health based on multiple factors
        let velocity_health = 1.0 - (self.transformation_velocity - 0.65).abs() * 2.0;
        let death_rebirth_health = self.death_rebirth_cycle;
        let catalyst_use_health = self.catalyst_use_effectiveness;

        let overall_health = (velocity_health + death_rebirth_health + catalyst_use_health) / 3.0;

        match overall_health {
            h if h >= 0.8 => HealthStatus::Healthy,
            h if h >= 0.6 => HealthStatus::Warning,
            h if h >= 0.4 => HealthStatus::Degraded,
            _ => HealthStatus::Pathological,
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

impl ArchetypeTrait for TransformationBodyArchetype {
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
        ArchetypeRole::Transformation
    }

    fn process(&mut self, catalyst: Float, _position: DevelopmentalPosition) {
        // Transformation processing
        let processing = catalyst * self.transformation_velocity;
        self.integration_capacity += processing * 0.01;
        self.integration_capacity = self.integration_capacity.clamp(0.0, 1.0);
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
        "The Transformation of Body"
    }

    fn description(&self) -> &str {
        "Transformation of Body - The transformative change and evolution"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Body
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::TransformationSingleton
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::common::{
        ArchetypeComplex, ArchetypeRole, FunctionalPair, HealthStatus, LambdaMeasurementType,
        SigmaAxis,
    };

    // Mock archetype for testing paired relationships
    struct MockArchetypeA13 {
        archetype_id: u8,
        lambda: LambdaMeasurement,
        tarot_correlation: TarotCorrelation,
    }

    impl MockArchetypeA13 {
        fn new(id: u8, lambda_value: Float) -> Self {
            MockArchetypeA13 {
                archetype_id: id,
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::TransformationVelocity,
                ),
                tarot_correlation: TarotCorrelation::new(format!("{} - {}", "Mock", "0")),
            }
        }
    }

    impl ArchetypeTrait for MockArchetypeA13 {
        fn archetype_id(&self) -> u8 {
            self.archetype_id
        }

        fn name(&self) -> &str {
            "Mock Significator Body"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Body
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Significator
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
            self.tarot_correlation.clone()
        }

        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.clamp(0.0, 1.0);
        }

        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::ProcessPair
        }
    }
}
