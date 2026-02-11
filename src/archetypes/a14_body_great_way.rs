// A14: The Great Way of the Body (The Alchemist)
// Environment container - mirror image of mind activity, providing athanor for alchemical transformation

use crate::archetypes::archetype_traits::GreatWayArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

// Use Mind Great Way's Milieu type for consistency
use crate::archetypes::a7_mind_great_way::Milieu;

/// Great Way of Body Archetype - The Alchemist
///
/// The Great Way of the Body is called the Alchemist - must be seen as mirror image of thrust
/// of activity of mind, for body is creature of mind and instrument of manifestation for fruits
/// of mind and spirit, providing athanor through which alchemist manifests gold.
///
/// **Key Characteristics:**
/// - Mirror image of thrust of activity of mind
/// - Body is creature of mind
/// - Instrument of manifestation for fruits of mind and spirit
/// - Provides athanor through which alchemist manifests gold
/// - Draws environment from veiling process dipped in time/space
/// - Infinity of time for various bodies to learn lessons
///
/// **Lambda Measurement:** Great Way clarity (0.5 - 0.8 healthy range)
/// - < 0.5: Distorted mirror, poor athanor, weak time/space connection
/// - 0.5 - 0.8: Harmonious mirror with effective athanor and time/space connection
/// - > 0.8: Over-identification with mind, loss of body autonomy
#[derive(Debug, Clone)]
pub struct GreatWayBodyArchetype {
    pub archetype_id: u8,
    pub active: bool,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // A14-specific fields
    /// Clarity of the great way - how clear is the perception of the field (0.0-1.0)
    pub great_way_clarity: Float,
    /// Mirror quality - how well does body mirror mind activity (0.0-1.0)
    pub mirror_quality: Float,
    /// Athanor effectiveness - how effective is athanor for alchemical transformation (0.0-1.0)
    pub athanor_effectiveness: Float,
    /// Time/space connection - how connected is body to time/space (0.0-1.0)
    pub time_space_connection: Float,
    /// Manifestation capacity - capacity for manifestation of fruits of mind and spirit (0.0-1.0)
    pub manifestation_capacity: Float,
    /// Environmental conditions - conditions of environment drawn from veiling (0.0-1.0)
    pub environmental_conditions: Float,

    // Developmental tracking
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,

    // Holonic integration
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,

    // Polarity (female after veiling)
    pub polarity: Polarity,

    // Phase 7: Current milieu for Lesser Cycle
    pub current_milieu: Milieu,
}

impl GreatWayBodyArchetype {
    /// Create a new Great Way of Body archetype with healthy initial values
    pub fn new() -> Self {
        let initial_lambda = LambdaMeasurement::new(0.65, LambdaMeasurementType::GreatWayClarity);

        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.4);
        activation_levels.insert(Rung::R2, 0.5);
        activation_levels.insert(Rung::R3, 0.6);
        activation_levels.insert(Rung::R4, 0.7);
        activation_levels.insert(Rung::R5, 0.6);
        activation_levels.insert(Rung::R6, 0.5);
        activation_levels.insert(Rung::R7, 0.4);

        GreatWayBodyArchetype {
            archetype_id: 14,
            active: true,
            lambda: initial_lambda,
            tarot_correlation: TarotCorrelation::new(
                "Temperance (XIV): Balance, harmony, integration, alchemical transformation".to_string(),
            ),

            // A14-specific fields - healthy initial values
            great_way_clarity: 0.65,
            mirror_quality: 0.65,
            athanor_effectiveness: 0.65,
            time_space_connection: 0.65,
            manifestation_capacity: 0.65,
            environmental_conditions: 0.65,

            // Developmental tracking
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O4, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,

            // Holonic integration
            description: "The Great Way of the Body is called the Alchemist - mirror image of mind activity, providing athanor for alchemical transformation of fruits of mind and spirit".to_string(),
            holonic_level: HolonicLevel::Meso,
            integration_capacity: 0.65,

            // Polarity (female after veiling)
            polarity: Polarity::STO,

            // Phase 7: Initialize default milieu
            current_milieu: crate::archetypes::a7_mind_great_way::Milieu {
                time_factor: 1.0,
                creativity_factor: 1.0,
                entropy_factor: 0.5,
                space_factor: 1.0,
                stability_factor: 0.7,
            },
        }
    }

    /// Calculate great way harmony score
    pub fn calculate_great_way_harmony(&self) -> Float {
        (self.great_way_clarity * 0.3
            + self.mirror_quality * 0.2
            + self.athanor_effectiveness * 0.2
            + self.time_space_connection * 0.2
            + self.manifestation_capacity * 0.1)
            .max(0.0)
            .min(1.0)
    }

    /// Update mirror quality based on mind activity
    pub fn update_mirror_quality(&mut self, mind_activity: Float) {
        // Mirror quality improves when mind activity is balanced
        let target = (mind_activity * 0.7 + 0.3).max(0.0).min(1.0);
        self.mirror_quality = (self.mirror_quality * 0.8 + target * 0.2).max(0.0).min(1.0);
        self.update_lambda(self.calculate_great_way_harmony());
    }

    /// Update athanor effectiveness based on transformation needs
    pub fn update_athanor_effectiveness(&mut self, transformation_needs: Float) {
        // Athanor effectiveness responds to transformation needs
        let target = (transformation_needs * 0.8 + 0.2).max(0.0).min(1.0);
        self.athanor_effectiveness = (self.athanor_effectiveness * 0.85 + target * 0.15)
            .max(0.0)
            .min(1.0);
        self.update_lambda(self.calculate_great_way_harmony());
    }

    /// Update time/space connection based on spiritual connection
    pub fn update_time_space_connection(&mut self, spiritual_connection: Float) {
        // Time/space connection improves with spiritual connection
        let target = (spiritual_connection * 0.75 + 0.25).max(0.0).min(1.0);
        self.time_space_connection = (self.time_space_connection * 0.9 + target * 0.1)
            .max(0.0)
            .min(1.0);
        self.update_lambda(self.calculate_great_way_harmony());
    }

    /// Update manifestation capacity
    pub fn update_manifestation_capacity(&mut self, mind_fruits: Float, spirit_fruits: Float) {
        // Manifestation capacity depends on fruits of mind and spirit
        let target = (mind_fruits * 0.5 + spirit_fruits * 0.5).max(0.0).min(1.0);
        self.manifestation_capacity = (self.manifestation_capacity * 0.85 + target * 0.15)
            .max(0.0)
            .min(1.0);
        self.update_lambda(self.calculate_great_way_harmony());
    }

    /// Update environmental conditions from veiling process
    pub fn update_environmental_conditions(&mut self, veiling_conditions: Float) {
        // Environmental conditions drawn from veiling process
        let target = veiling_conditions.max(0.0).min(1.0);
        self.environmental_conditions = (self.environmental_conditions * 0.9 + target * 0.1)
            .max(0.0)
            .min(1.0);
        self.update_lambda(self.calculate_great_way_harmony());
    }

    /// Get diagnostic information
    pub fn get_diagnostic_info(&self) -> Vec<String> {
        let mut info = vec![];

        info.push(format!("Great Way Clarity: {:.2}", self.great_way_clarity));
        info.push(format!("Mirror Quality: {:.2}", self.mirror_quality));
        info.push(format!(
            "Athanor Effectiveness: {:.2}",
            self.athanor_effectiveness
        ));
        info.push(format!(
            "Time/Space Connection: {:.2}",
            self.time_space_connection
        ));
        info.push(format!(
            "Manifestation Capacity: {:.2}",
            self.manifestation_capacity
        ));
        info.push(format!(
            "Environmental Conditions: {:.2}",
            self.environmental_conditions
        ));

        // Health indicators
        if self.lambda.is_healthy() {
            info.push("Status: Healthy - Harmonious mirror with effective athanor".to_string());
        } else if self.lambda.is_pathological_low() {
            info.push("Status: Pathological - Distorted mirror, poor athanor".to_string());
        } else {
            info.push("Status: Pathological - Over-identification with mind".to_string());
        }

        info
    }

    // ============================================================================
    // PHASE 7: MILIEU DEFINITION AND APPLICATION (Greater Cycle Regulation)
    // ============================================================================

    /// Create milieu based on mirror quality and athanor effectiveness
    ///
    /// Great Way defines the MILIEU (time/space/entropy) for Lesser Cycle
    pub fn create_milieu(&self, _mind_activity: Float) -> Milieu {
        // Milieu factors based on mirror quality and athanor effectiveness
        let time_factor = 1.0 + (self.time_space_connection - 0.5) * 0.4;
        let space_factor = 1.0 + (self.manifestation_capacity - 0.5) * 0.4;
        let entropy_factor = 1.0 + (self.athanor_effectiveness - 0.5) * 0.3;
        let creativity_factor = 1.0 + (self.mirror_quality - 0.5) * 0.5;
        let stability_factor = 1.0 + (self.great_way_clarity - 0.5) * 0.3;

        Milieu::with_factors(
            time_factor,
            space_factor,
            entropy_factor,
            creativity_factor,
            stability_factor,
        )
    }

    /// Update current milieu
    pub fn update_milieu(&mut self, mind_activity: Float) {
        self.current_milieu = self.create_milieu(mind_activity);
    }

    /// Get current milieu
    pub fn get_milieu(&self) -> &Milieu {
        &self.current_milieu
    }

    /// Apply milieu to processing parameters
    ///
    /// Great Way defines MILIEU for Lesser Cycle processing
    pub fn apply_milieu_to_processing(
        &self,
        processing_capacity: Float,
        processing_efficiency: Float,
        catalyst_tolerance: Float,
    ) -> (Float, Float, Float) {
        let modified_capacity = processing_capacity * self.current_milieu.time_factor;
        let modified_efficiency = processing_efficiency * self.current_milieu.creativity_factor;
        let modified_tolerance = catalyst_tolerance * self.current_milieu.stability_factor;

        (
            modified_capacity.clamp(0.1, 1.0),
            modified_efficiency.clamp(0.1, 1.0),
            modified_tolerance.clamp(0.1, 1.0),
        )
    }
}

impl ArchetypeTrait for GreatWayBodyArchetype {
    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Body
    }

    fn role(&self) -> ArchetypeRole {
        ArchetypeRole::GreatWay
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.max(0.0).min(1.0);
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaB
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::IdentityPair
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        // Great Way processes by maintaining mirror quality and athanor effectiveness
        let harmony = self.calculate_great_way_harmony();

        // Update clarity based on harmony
        self.great_way_clarity = (self.great_way_clarity * 0.9 + harmony * 0.1)
            .max(0.0)
            .min(1.0);

        // Update lambda
        self.update_lambda(harmony);

        // Update integration capacity based on harmony
        self.integration_capacity = (self.integration_capacity * 0.95 + harmony * 0.05)
            .max(0.0)
            .min(1.0);
    }

    fn archetype_id(&self) -> u8 {
        14
    }

    fn name(&self) -> &str {
        "The Great Way of the Body"
    }

    fn activate(&mut self, intensity: Float) {
        // Adjust lambda based on intensity
        let new_value = (self.lambda.value + intensity * 0.1).clamp(0.0, 1.0);
        self.update_lambda(new_value);
    }

    fn deactivate(&mut self) {
        // Deactivate by reducing lambda
        self.update_lambda(0.0);
    }

    fn is_active(&self) -> bool {
        self.lambda.value > 0.0
    }

    fn is_healthy(&self) -> bool {
        self.lambda.value >= self.lambda.healthy_min && self.lambda.value <= self.lambda.healthy_max
    }

    fn health_status(&self) -> HealthStatus {
        if self.is_healthy() {
            HealthStatus::Balanced
        } else if self.lambda.value < self.lambda.healthy_min {
            HealthStatus::PathologicalLow
        } else {
            HealthStatus::PathologicalHigh
        }
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl LambdaMeasurable for GreatWayBodyArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        self.calculate_great_way_harmony()
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 1.0)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = vec![];

        if self.lambda.is_pathological_low() {
            indicators
                .push("Distorted mirror - body does not mirror mind activity well".to_string());
            indicators.push("Poor athanor - ineffective for alchemical transformation".to_string());
            indicators.push(
                "Weak time/space connection - limited access to deeper dimensions".to_string(),
            );
            indicators.push(
                "Poor manifestation - not effective instrument for fruits of mind and spirit"
                    .to_string(),
            );
        } else if self.lambda.is_pathological_high() {
            indicators.push("Over-identification with mind - loss of body autonomy".to_string());
            indicators.push("Too rigid mirror - body loses its own nature".to_string());
            indicators
                .push("Athanor too intense - transformation becomes overwhelming".to_string());
        }

        indicators
    }
}

impl Developmental for GreatWayBodyArchetype {
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
        let rung_level = position.rung_level() as Float / 7.0;
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
            let current_level = self.activation_levels.get(&rung).unwrap_or(&0.0);
            let target_level = if rung.value() <= position.rung_level() {
                rung_level
            } else {
                *current_level * 0.95
            };
            self.activation_levels.insert(rung, target_level);
        }

        // Update activated rungs
        self.activated_rungs = vec![
            Rung::R1,
            Rung::R2,
            Rung::R3,
            Rung::R4,
            Rung::R5,
            Rung::R6,
            Rung::R7,
        ]
        .into_iter()
        .filter(|r| r.value() <= position.rung_level())
        .collect();
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        *self.activation_levels.get(&rung).unwrap_or(&0.0)
    }
}

impl Paired for GreatWayBodyArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(12) // Paired with A12 Significator of Body
    }

    fn get_pair(&self) -> Option<u8> {
        Some(12)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, paired_archetype: &dyn Paired) -> Float {
        // Identity pair tension: difference between identity (Significator) and environment (Great Way)
        let paired_lambda = paired_archetype.lambda().value;
        let tension = (self.lambda.value - paired_lambda).abs();
        tension
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        // Identity pair balance: harmonious integration of identity and environment
        let paired_lambda = paired_archetype.lambda().value;
        let sum = self.lambda.value + paired_lambda;
        if sum > 0.0 {
            1.0 - (self.lambda.value - paired_lambda).abs() / sum
        } else {
            0.0
        }
    }
}

impl Holonic for GreatWayBodyArchetype {
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
        // Transcend when integration capacity is high and lambda is healthy
        if self.integration_capacity > 0.7 && self.lambda.is_healthy() {
            // Upgrade holonic level
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
                _ => false,
            }
        } else {
            false
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include lower level by integrating its capacity
        let lower_capacity = lower_level.integration_capacity();

        // Integration is successful when there's room for inclusion
        if self.integration_capacity < 0.9 {
            self.integration_capacity = (self.integration_capacity + lower_capacity * 0.3)
                .max(0.0)
                .min(1.0);
            true
        } else {
            false
        }
    }
}

// ============================================================================
// GREAT WAY ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl GreatWayArchetypeTrait for GreatWayBodyArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement {
        self.lambda.clone()
    }

    fn get_framework_clarity(&self) -> Float {
        // Map great_way_clarity to framework_clarity
        self.great_way_clarity
    }

    fn get_framework_alignment(&self) -> Float {
        // Map mirror_quality to framework_alignment (how well body aligns with mind)
        self.mirror_quality
    }

    // Core setters
    fn set_framework_clarity(&mut self, value: Float) {
        // Map framework_clarity to great_way_clarity
        self.great_way_clarity = value.clamp(0.0, 1.0);
    }

    fn set_framework_alignment(&mut self, value: Float) {
        // Map framework_alignment to mirror_quality
        self.mirror_quality = value.clamp(0.0, 1.0);
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

    // Phase 7: Milieu definition
    fn get_milieu(&self) -> Milieu {
        self.current_milieu.clone()
    }

    fn set_milieu(&mut self, milieu: Milieu) {
        self.current_milieu = milieu;
    }

    fn update_coupling_coefficient(&self, current_coefficient: Float) -> Float {
        // Modify coupling based on great way clarity and mirror quality
        let clarity_factor = self.great_way_clarity;
        let alignment_factor = self.mirror_quality;
        let modification = (clarity_factor + alignment_factor) / 2.0;

        // Apply milieu factors
        let time_modification = self.current_milieu.time_factor;
        let creativity_modification = self.current_milieu.creativity_factor;

        current_coefficient * modification * time_modification * creativity_modification
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        // Calculate health based on multiple factors
        let clarity_health = 1.0 - (self.great_way_clarity - 0.65).abs() * 2.0;
        let mirror_health = self.mirror_quality;
        let athanor_health = self.athanor_effectiveness;
        let lambda_health = if self.lambda.is_healthy() { 1.0 } else { 0.0 };

        let overall_health =
            (clarity_health + mirror_health + athanor_health + lambda_health) / 4.0;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::common::{ArchetypeRole, HealthStatus, SigmaAxis};

    // Mock Significator for testing paired relationships
    struct MockSignificatorBody {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockSignificatorBody {
        fn new(lambda_value: Float) -> Self {
            MockSignificatorBody {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::SignificatorCoherence,
                ),
                tarot: TarotCorrelation::new("The Hanged Man".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockSignificatorBody {
        fn archetype_id(&self) -> u8 {
            12
        }

        fn name(&self) -> &str {
            "Significator of Body"
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

        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.max(0.0).min(1.0);
        }

        fn tarot_correlation(&self) -> TarotCorrelation {
            self.tarot.clone()
        }

        fn sigma_axis(&self) -> SigmaAxis {
            SigmaAxis::SigmaB
        }

        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::IdentityPair
        }

        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}

        fn description(&self) -> &str {
            "Significator of Body - identity agent"
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

    impl Paired for MockSignificatorBody {
        fn get_pair(&self) -> Option<u8> {
            None
        }
        fn paired_archetype_id(&self) -> Option<u8> {
            Some(14) // Paired with A14 Great Way of Body
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
    fn test_great_way_body_creation() {
        let great_way = GreatWayBodyArchetype::new();

        assert_eq!(great_way.archetype_id(), 14);
        assert_eq!(great_way.name(), "Great Way of Body (The Alchemist)");
        assert_eq!(great_way.complex(), ArchetypeComplex::Body);
        assert_eq!(great_way.role(), ArchetypeRole::GreatWay);
        assert_eq!(great_way.sigma_axis(), SigmaAxis::SigmaB);
        assert_eq!(great_way.functional_pair(), FunctionalPair::IdentityPair);
    }

    #[test]
    fn test_great_way_body_initial_values() {
        let great_way = GreatWayBodyArchetype::new();

        // Check lambda initial value
        assert!((great_way.lambda.value - 0.65).abs() < 0.01);
        assert_eq!(great_way.lambda.healthy_min, 0.5);
        assert_eq!(great_way.lambda.healthy_max, 0.8);

        // Check A14-specific fields
        assert!((great_way.great_way_clarity - 0.65).abs() < 0.01);
        assert!((great_way.mirror_quality - 0.65).abs() < 0.01);
        assert!((great_way.athanor_effectiveness - 0.65).abs() < 0.01);
        assert!((great_way.time_space_connection - 0.65).abs() < 0.01);
        assert!((great_way.manifestation_capacity - 0.65).abs() < 0.01);
        assert!((great_way.environmental_conditions - 0.65).abs() < 0.01);

        // Check polarity (female after veiling)
        assert_eq!(great_way.polarity, Polarity::STO);
    }

    #[test]
    fn test_great_way_body_tarot_correlation() {
        let great_way = GreatWayBodyArchetype::new();

        assert_eq!(
            great_way.tarot_correlation.card,
            "Temperance (XIV): Balance, harmony, moderation, and equilibrium"
        );
        assert!(great_way.tarot_correlation.card.contains("XIV"));
        assert!(great_way.tarot_correlation.card.contains("Balance"));
    }

    #[test]
    fn test_great_way_body_calculate_lambda() {
        let great_way = GreatWayBodyArchetype::new();
        let lambda = great_way.calculate_lambda();

        // Lambda should be within valid range
        assert!(lambda >= 0.0 && lambda <= 1.0);

        // Lambda should be close to initial harmony
        assert!((lambda - 0.65).abs() < 0.1);
    }

    #[test]
    fn test_great_way_body_health_status() {
        let mut great_way = GreatWayBodyArchetype::new();

        // Test healthy state
        assert_eq!(great_way.health_status(), HealthStatus::Healthy);
        assert!(great_way.is_healthy());

        // Test pathological low
        great_way.lambda.value = 0.4;
        assert_eq!(great_way.health_status(), HealthStatus::PathologicalLow);
        assert!(!great_way.is_healthy());

        // Test pathological high
        great_way.lambda.value = 0.9;
        assert_eq!(great_way.health_status(), HealthStatus::PathologicalHigh);
        assert!(!great_way.is_healthy());
    }

    #[test]
    fn test_great_way_body_healthy_range() {
        let great_way = GreatWayBodyArchetype::new();
        let (min, max) = great_way.healthy_range();

        assert_eq!(min, 0.5);
        assert_eq!(max, 0.8);
    }

    #[test]
    fn test_great_way_body_pathological_indicators() {
        let mut great_way = GreatWayBodyArchetype::new();

        // Test pathological low indicators
        great_way.lambda.value = 0.4;
        let indicators = great_way.pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Distorted mirror")));

        // Test pathological high indicators
        great_way.lambda.value = 0.9;
        let indicators = great_way.pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Over-identification")));
    }

    #[test]
    fn test_great_way_body_developmental_position() {
        let great_way = GreatWayBodyArchetype::new();
        let position = great_way.developmental_position();

        // DevelopmentalPosition is an enum, compare directly
        assert_eq!(position, DevelopmentalPosition::Significator); // Great Way archetypes are Significator position
    }

    #[test]
    fn test_great_way_body_activated_rungs() {
        let great_way = GreatWayBodyArchetype::new();
        let activated_rungs = great_way.activated_rungs();

        assert!(!activated_rungs.is_empty());
        assert!(activated_rungs.contains(&Rung::R1));
        assert!(activated_rungs.contains(&Rung::R2));
        assert!(activated_rungs.contains(&Rung::R3));
        assert!(activated_rungs.contains(&Rung::R4));
    }

    #[test]
    fn test_great_way_body_rung_activation_level() {
        let great_way = GreatWayBodyArchetype::new();

        // Test R4 activation (should be 0.7 from initialization)
        let r4_level = great_way.rung_activation_level(Rung::R4);
        assert!((r4_level - 0.7).abs() < 0.01);

        // Test R7 activation (should be 0.4 from initialization)
        let r7_level = great_way.rung_activation_level(Rung::R7);
        assert!((r7_level - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_great_way_body_update_developmental_position() {
        let mut great_way = GreatWayBodyArchetype::new();
        let new_position = DevelopmentalPosition::new_with_octant_rung(Octant::O7, 6);

        Developmental::update_developmental_position(&mut great_way, new_position);

        assert_eq!(
            great_way.developmental_position(),
            DevelopmentalPosition::Significator
        ); // rung 6 % 4 = 2 (Experience, but wraps to Significator)

        // Check that R6 is now activated
        assert!(great_way.activated_rungs().contains(&Rung::R6));
    }

    #[test]
    fn test_great_way_body_paired_archetype_id() {
        let great_way = GreatWayBodyArchetype::new();

        // Great Way is paired with Significator (A12)
        assert_eq!(great_way.paired_archetype_id(), Some(12));
    }

    #[test]
    fn test_great_way_body_pair_tension() {
        let great_way = GreatWayBodyArchetype::new();
        let significator = MockSignificatorBody::new(0.6);

        let tension = great_way.calculate_pair_tension(&significator);

        // Tension should be the absolute difference
        assert!((tension - 0.05).abs() < 0.01); // |0.65 - 0.60| ≈ 0.05
    }

    #[test]
    fn test_great_way_body_pair_balance() {
        let great_way = GreatWayBodyArchetype::new();
        let significator = MockSignificatorBody::new(0.6);

        let balance = great_way.calculate_pair_balance(&significator);

        // Balance should be high when values are close
        assert!(balance > 0.9);
    }

    #[test]
    fn test_great_way_body_holonic_level() {
        let great_way = GreatWayBodyArchetype::new();

        assert_eq!(great_way.holonic_level(), HolonicLevel::Meso);
    }

    #[test]
    fn test_great_way_body_integration_capacity() {
        let great_way = GreatWayBodyArchetype::new();

        // Integration capacity should be initialized
        assert!(great_way.integration_capacity() > 0.0);
        assert!(great_way.integration_capacity() <= 1.0);
    }

    #[test]
    fn test_great_way_body_transcend() {
        let mut great_way = GreatWayBodyArchetype::new();

        // Set high integration capacity for transcendence
        great_way.integration_capacity = 0.8;

        // Transcend from Meso to Macro
        let result = great_way.transcend();
        assert!(result);
        assert_eq!(great_way.holonic_level(), HolonicLevel::Macro);

        // Try to transcend again (should succeed to Meta)
        let result = great_way.transcend();
        assert!(result);
        assert_eq!(great_way.holonic_level(), HolonicLevel::Meta);

        // Try to transcend from Meta (should fail)
        let result = great_way.transcend();
        assert!(!result);
    }

    #[test]
    fn test_great_way_body_include() {
        let mut great_way = GreatWayBodyArchetype::new();

        // Create a mock lower level
        let lower_level = GreatWayBodyArchetype::new();

        // Include lower level
        let result = great_way.include(&lower_level);
        assert!(result);

        // Integration capacity should increase
        assert!(great_way.integration_capacity > lower_level.integration_capacity);
    }

    #[test]
    fn test_great_way_body_process() {
        let mut great_way = GreatWayBodyArchetype::new();

        let initial_clarity = great_way.great_way_clarity;
        let initial_lambda = great_way.lambda.value;

        great_way.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Process should update clarity and lambda
        assert!((great_way.great_way_clarity - initial_clarity).abs() < 0.1);
        assert!((great_way.lambda.value - initial_lambda).abs() < 0.1);
    }

    #[test]
    fn test_great_way_body_update_mirror_quality() {
        let mut great_way = GreatWayBodyArchetype::new();

        let initial_quality = great_way.mirror_quality;

        // Update mirror quality with high mind activity
        great_way.update_mirror_quality(0.8);

        // Mirror quality should increase
        assert!(great_way.mirror_quality > initial_quality);
        assert!(great_way.mirror_quality <= 1.0);
    }

    #[test]
    fn test_great_way_body_update_athanor_effectiveness() {
        let mut great_way = GreatWayBodyArchetype::new();

        let initial_athanor = great_way.athanor_effectiveness;

        // Update athanor effectiveness with high transformation needs
        great_way.update_athanor_effectiveness(0.9);

        // Athanor effectiveness should increase
        assert!(great_way.athanor_effectiveness > initial_athanor);
        assert!(great_way.athanor_effectiveness <= 1.0);
    }

    #[test]
    fn test_great_way_body_update_time_space_connection() {
        let mut great_way = GreatWayBodyArchetype::new();

        let initial_connection = great_way.time_space_connection;

        // Update time/space connection with high spiritual connection
        great_way.update_time_space_connection(0.85);

        // Time/space connection should increase
        assert!(great_way.time_space_connection > initial_connection);
        assert!(great_way.time_space_connection <= 1.0);
    }

    #[test]
    fn test_great_way_body_update_manifestation_capacity() {
        let mut great_way = GreatWayBodyArchetype::new();

        let initial_capacity = great_way.manifestation_capacity;

        // Update manifestation capacity with high fruits of mind and spirit
        great_way.update_manifestation_capacity(0.8, 0.9);

        // Manifestation capacity should increase
        assert!(great_way.manifestation_capacity > initial_capacity);
        assert!(great_way.manifestation_capacity <= 1.0);
    }

    #[test]
    fn test_great_way_body_update_environmental_conditions() {
        let mut great_way = GreatWayBodyArchetype::new();

        let initial_conditions = great_way.environmental_conditions;

        // Update environmental conditions
        great_way.update_environmental_conditions(0.75);

        // Environmental conditions should update
        assert!((great_way.environmental_conditions - initial_conditions).abs() < 0.1);
    }

    #[test]
    fn test_great_way_body_calculate_great_way_harmony() {
        let great_way = GreatWayBodyArchetype::new();
        let harmony = great_way.calculate_great_way_harmony();

        // Harmony should be within valid range
        assert!(harmony >= 0.0 && harmony <= 1.0);

        // Harmony should be close to initial values
        assert!((harmony - 0.65).abs() < 0.1);
    }

    #[test]
    fn test_great_way_body_get_diagnostic_info() {
        let great_way = GreatWayBodyArchetype::new();
        let info = great_way.get_diagnostic_info();

        // Should have diagnostic information
        assert!(!info.is_empty());
        assert!(info.len() >= 7); // At least 6 metrics + status

        // Check for specific metrics
        assert!(info.iter().any(|i| i.contains("Great Way Clarity")));
        assert!(info.iter().any(|i| i.contains("Mirror Quality")));
        assert!(info.iter().any(|i| i.contains("Athanor Effectiveness")));
        assert!(info.iter().any(|i| i.contains("Time/Space Connection")));
        assert!(info.iter().any(|i| i.contains("Manifestation Capacity")));
        assert!(info.iter().any(|i| i.contains("Environmental Conditions")));
    }
}
