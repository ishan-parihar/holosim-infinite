// A20: The Transformation of Spirit
// The Sarcophagus - Material world transformed by spirit into infinite and eternal

use crate::archetypes::archetype_traits::TransformationArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// The Transformation of Spirit - mutation event for Spirit complex
///
/// The Transformation of the Spirit is represented by the Sarcophagus - the material world
/// is transformed by the spirit into that which is infinite and eternal; the infinity of the
/// spirit is an even greater realization than the infinity of consciousness. Consciousness
/// disciplined by will and faith may contact intelligent infinity directly. Many things fall
/// away in many steps of adepthood.
#[derive(Debug, Clone)]
pub struct TransformationSpiritArchetype {
    pub archetype_id: u8,
    pub active: bool,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // A20-specific fields
    /// How effectively is material world transformed by spirit (0.0-1.0)
    pub material_transmutation: Float,

    /// How much infinite and eternal realization achieved (0.0-1.0)
    pub infinite_realization: Float,

    /// How direct is contact with intelligent infinity (0.0-1.0)
    pub intelligent_infinity_contact: Float,

    /// How many things falling away in steps of adepthood (0.0-1.0)
    pub adepthood_progress: Float,

    /// Overall velocity of transformation (0.0-1.0)
    pub transformation_velocity: Float,

    /// How disciplined is consciousness by will and faith (0.0-1.0)
    pub consciousness_discipline: Float,

    /// State of material world transformation (0.0-1.0)
    pub sarcophagus_state: Float,

    /// Depth of spirit infinity beyond consciousness (0.0-1.0)
    pub spirit_infinity_depth: Float,

    // Developmental fields
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    #[allow(dead_code)]
    rung_activation: HashMap<Rung, Float>,
    pub activation_levels: HashMap<Rung, Float>,

    // Holonic fields
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,

    // Polarity
    pub polarity: Polarity,

    /// Progress toward chosen polarization (0.0 - 1.0)
    pub polarization_progress: Float,

    /// Whether choice betwixt light and dark has been made
    pub choice_made: bool,

    // Description
    pub description: String,

    // Phase 7: Parameter modification capabilities
    pub can_modify_veil: bool,
    pub can_modify_capacity: bool,
    pub can_modify_efficiency: bool,
    pub can_modify_tolerance: bool,
}

impl Default for TransformationSpiritArchetype {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformationSpiritArchetype {
    /// Create a new Transformation of Spirit archetype with healthy initial values
    pub fn new() -> Self {
        let lambda = LambdaMeasurement::new(0.6, LambdaMeasurementType::TransformationVelocity);

        let tarot = TarotCorrelation::new("Judgment (XX): Resurrection, awakening, transformation, spiritual rebirth".to_string());

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
        // Activate initial rungs
        activation_levels.insert(Rung::R1, 0.4);
        activation_levels.insert(Rung::R2, 0.5);
        activation_levels.insert(Rung::R3, 0.6);
        activation_levels.insert(Rung::R4, 0.7);

        let mut archetype = TransformationSpiritArchetype {
            archetype_id: 20,
            active: true,
            lambda,
            tarot_correlation: tarot,

            material_transmutation: 0.6,
            infinite_realization: 0.6,
            intelligent_infinity_contact: 0.5,
            adepthood_progress: 0.5,
            transformation_velocity: 0.6,
            consciousness_discipline: 0.6,
            sarcophagus_state: 0.6,
            spirit_infinity_depth: 0.6,

            developmental_position,
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
                        rung_activation: HashMap::new(),
activation_levels,

            holonic_level: HolonicLevel::Micro,
            integration_capacity: 0.6,

            polarity: Polarity::STO, // Female polarity assignment after veiling
            polarization_progress: 0.0,
            choice_made: false,

            description: String::from(
                "The Transformation of Spirit (The Sarcophagus) - Material world transformed by spirit \
                 into infinite and eternal. Spirit infinity is greater than consciousness infinity. \
                 Consciousness disciplined by will and faith may contact intelligent infinity directly. \
                 Many things fall away in many steps of adepthood.",
            ),

            // Phase 7: Initialize parameter modification capabilities
            can_modify_veil: true,
            can_modify_capacity: true,
            can_modify_efficiency: true,
            can_modify_tolerance: true,
        };

        // Recalculate lambda based on initial state
        archetype.recalculate_lambda();

        archetype
    }

    // =========================================================================
    // A20-SPECIFIC CALCULATION METHODS
    // =========================================================================

    /// Calculate transformation velocity score
    pub fn calculate_transformation_velocity_score(&self) -> Float {
        // Transformation velocity is weighted combination of:
        // - Material transmutation (40%)
        // - Infinite realization (35%)
        // - Consciousness discipline (15%)
        // - Adepthood progress (10%)
        let transmutation_component = self.material_transmutation * 0.4;
        let infinite_component = self.infinite_realization * 0.35;
        let discipline_component = self.consciousness_discipline * 0.15;
        let adepthood_component = self.adepthood_progress * 0.1;

        transmutation_component + infinite_component + discipline_component + adepthood_component
    }

    /// Calculate material transmutation score
    pub fn calculate_material_transmutation_score(&self) -> Float {
        // Material transmutation depends on:
        // - Consciousness discipline (40%)
        // - Infinite realization (35%)
        // - Sarcophagus state (25%)
        let discipline_component = self.consciousness_discipline * 0.4;
        let infinite_component = self.infinite_realization * 0.35;
        let sarcophagus_component = self.sarcophagus_state * 0.25;

        discipline_component + infinite_component + sarcophagus_component
    }

    /// Calculate infinite realization score
    pub fn calculate_infinite_realization_score(&self) -> Float {
        // Infinite realization depends on:
        // - Material transmutation (40%)
        // - Spirit infinity depth (35%)
        // - Intelligent infinity contact (25%)
        let transmutation_component = self.material_transmutation * 0.4;
        let infinity_component = self.spirit_infinity_depth * 0.35;
        let contact_component = self.intelligent_infinity_contact * 0.25;

        transmutation_component + infinity_component + contact_component
    }

    /// Calculate intelligent infinity contact score
    pub fn calculate_intelligent_infinity_contact_score(&self) -> Float {
        // Intelligent infinity contact depends on:
        // - Consciousness discipline (50%)
        // - Spirit infinity depth (30%)
        // - Infinite realization (20%)
        let discipline_component = self.consciousness_discipline * 0.5;
        let infinity_component = self.spirit_infinity_depth * 0.3;
        let infinite_component = self.infinite_realization * 0.2;

        discipline_component + infinity_component + infinite_component
    }

    /// Calculate adepthood progress score
    pub fn calculate_adepthood_progress_score(&self) -> Float {
        // Adepthood progress depends on:
        // - Material transmutation (40%)
        // - Transformation velocity (35%)
        // - Infinite realization (25%)
        let transmutation_component = self.material_transmutation * 0.4;
        let velocity_component = self.transformation_velocity * 0.35;
        let infinite_component = self.infinite_realization * 0.25;

        transmutation_component + velocity_component + infinite_component
    }

    /// Calculate consciousness discipline score
    pub fn calculate_consciousness_discipline_score(&self) -> Float {
        // Consciousness discipline depends on:
        // - Adepthood progress (40%)
        // - Infinite realization (35%)
        // - Transformation velocity (25%)
        let adepthood_component = self.adepthood_progress * 0.4;
        let infinite_component = self.infinite_realization * 0.35;
        let velocity_component = self.transformation_velocity * 0.25;

        adepthood_component + infinite_component + velocity_component
    }

    /// Calculate sarcophagus state score
    pub fn calculate_sarcophagus_state_score(&self) -> Float {
        // Sarcophagus state depends on:
        // - Material transmutation (50%)
        // - Infinite realization (30%)
        // - Spirit infinity depth (20%)
        let transmutation_component = self.material_transmutation * 0.5;
        let infinite_component = self.infinite_realization * 0.3;
        let infinity_component = self.spirit_infinity_depth * 0.2;

        transmutation_component + infinite_component + infinity_component
    }

    /// Calculate spirit infinity depth score
    pub fn calculate_spirit_infinity_depth_score(&self) -> Float {
        // Spirit infinity depth depends on:
        // - Infinite realization (50%)
        // - Intelligent infinity contact (30%)
        // - Material transmutation (20%)
        let infinite_component = self.infinite_realization * 0.5;
        let contact_component = self.intelligent_infinity_contact * 0.3;
        let transmutation_component = self.material_transmutation * 0.2;

        infinite_component + contact_component + transmutation_component
    }

    // =========================================================================
    // A20-SPECIFIC UPDATE METHODS
    // =========================================================================

    /// Update material transmutation
    pub fn update_material_transmutation(&mut self, value: Float) {
        self.material_transmutation = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update infinite realization
    pub fn update_infinite_realization(&mut self, value: Float) {
        self.infinite_realization = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update intelligent infinity contact
    pub fn update_intelligent_infinity_contact(&mut self, value: Float) {
        self.intelligent_infinity_contact = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update adepthood progress
    pub fn update_adepthood_progress(&mut self, value: Float) {
        self.adepthood_progress = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update transformation velocity
    pub fn update_transformation_velocity(&mut self, value: Float) {
        self.transformation_velocity = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update consciousness discipline
    pub fn update_consciousness_discipline(&mut self, value: Float) {
        self.consciousness_discipline = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update sarcophagus state
    pub fn update_sarcophagus_state(&mut self, value: Float) {
        self.sarcophagus_state = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Update spirit infinity depth
    pub fn update_spirit_infinity_depth(&mut self, value: Float) {
        self.spirit_infinity_depth = value.clamp(0.0, 1.0);
        self.recalculate_lambda();
    }

    /// Recalculate lambda based on current state
    pub fn recalculate_lambda(&mut self) {
        let new_lambda = self.calculate_lambda();
        self.lambda.value = new_lambda;
    }

    // =========================================================================
    // A20-SPECIFIC QUERY METHODS
    // =========================================================================

    /// Check if transformation is effective
    pub fn is_transformation_effective(&self) -> bool {
        self.material_transmutation >= 0.5 && self.infinite_realization >= 0.5
    }

    /// Check if stuck in material (pathological low)
    pub fn is_stuck_in_material(&self) -> bool {
        self.material_transmutation < 0.5 && self.infinite_realization < 0.5
    }

    /// Check if premature transcendence (pathological high)
    pub fn is_premature_transcendence(&self) -> bool {
        self.lambda.value > 0.8 && self.adepthood_progress < 0.5
    }

    /// Check if has intelligent infinity contact
    pub fn has_intelligent_infinity_contact(&self) -> bool {
        self.intelligent_infinity_contact >= 0.7
    }

    /// Check if consciousness is disciplined
    pub fn is_consciousness_disciplined(&self) -> bool {
        self.consciousness_discipline >= 0.6
    }

    /// Check if making adepthood progress
    pub fn is_making_adepthood_progress(&self) -> bool {
        self.adepthood_progress >= 0.5
    }

    /// Check if spirit infinity is greater than consciousness infinity
    pub fn is_spirit_infinity_greater(&self) -> bool {
        self.spirit_infinity_depth > self.infinite_realization
    }

    /// Check if transformation is ongoing
    pub fn is_transformation_ongoing(&self) -> bool {
        self.transformation_velocity >= 0.4 && self.transformation_velocity <= 0.8
    }

    /// Get diagnostic information
    pub fn get_diagnostic_info(&self) -> String {
        format!(
            "Transformation of Spirit (A20) Diagnostic:\n\
             - Lambda: {:.2} ({})\n\
             - Material Transmutation: {:.2}\n\
             - Infinite Realization: {:.2}\n\
             - Intelligent Infinity Contact: {:.2}\n\
             - Adepthood Progress: {:.2}\n\
             - Transformation Velocity: {:.2}\n\
             - Consciousness Discipline: {:.2}\n\
             - Sarcophagus State: {:.2}\n\
             - Spirit Infinity Depth: {:.2}\n\
             - Health Status: {}",
            self.lambda.value,
            self.lambda.health_status(),
            self.material_transmutation,
            self.infinite_realization,
            self.intelligent_infinity_contact,
            self.adepthood_progress,
            self.transformation_velocity,
            self.consciousness_discipline,
            self.sarcophagus_state,
            self.spirit_infinity_depth,
            self.lambda.health_status()
        )
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

        // Higher transformation velocity → veil becomes more permeable
        let modification = self.transformation_velocity * 0.03;
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

impl LambdaMeasurable for TransformationSpiritArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        let transmutation = self.material_transmutation;
        let infinite = self.infinite_realization;
        let contact = self.intelligent_infinity_contact;
        let velocity = self.transformation_velocity;

        (transmutation * 0.3 + infinite * 0.3 + contact * 0.2 + velocity * 0.2).min(1.0)
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.is_pathological_low() {
            indicators.push("Stuck in material - no spiritual progress".to_string());
            indicators.push("Lack of infinite realization".to_string());
        }

        if self.lambda.is_pathological_high() {
            indicators.push("Premature transcendence".to_string());
            indicators.push("Skipping necessary development".to_string());
        }

        indicators
    }
}

impl Developmental for TransformationSpiritArchetype {
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

impl Paired for TransformationSpiritArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(0)
    }

    fn get_pair(&self) -> Option<u8> {
        Some(0)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        0.0
    }

    fn calculate_pair_balance(&self, _paired_archetype: &dyn Paired) -> Float {
        0.0
    }
}

impl Holonic for TransformationSpiritArchetype {
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
        let readiness = self.calculate_lambda();

        if readiness >= 0.7 {
            match self.holonic_level {
                HolonicLevel::Micro => {
                    self.holonic_level = HolonicLevel::Meso;
                    self.integration_capacity = self.calculate_lambda() * 0.9;
                    true
                }
                HolonicLevel::Meso => {
                    self.holonic_level = HolonicLevel::Macro;
                    self.integration_capacity = self.calculate_lambda() * 0.8;
                    true
                }
                HolonicLevel::Macro | HolonicLevel::Meta => false,
            }
        } else {
            false
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        let lower_capacity = lower_level.integration_capacity();
        let current_capacity = self.integration_capacity;
        self.integration_capacity = (current_capacity * 0.7 + lower_capacity * 0.3).min(1.0);
        true
    }
}

impl TransformationArchetypeTrait for TransformationSpiritArchetype {
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

    fn set_transformation_velocity(&mut self, velocity: Float) {
        self.transformation_velocity = velocity.clamp(0.0, 1.0);
    }

    fn set_transformation_direction(&mut self, direction: Polarity) {
        self.polarity = direction;
    }

    fn set_polarization_progress(&mut self, progress: Float) {
        self.polarization_progress = progress.clamp(0.0, 1.0);
    }

    fn set_choice_made(&mut self, made: bool) {
        self.choice_made = made;
    }

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

    fn modify_veil_permeability(&self, current_permeability: Float) -> Float {
        (current_permeability * (1.0 + self.transformation_velocity * 0.2)).min(1.0)
    }

    fn modify_processing_capacity(&self, current_capacity: Float) -> Float {
        (current_capacity * (1.0 + self.polarization_progress * 0.3)).min(1.0)
    }

    fn modify_processing_efficiency(&self, current_efficiency: Float) -> Float {
        (current_efficiency
            * (1.0 + self.transformation_velocity * self.polarization_progress * 0.15))
            .min(1.0)
    }

    fn modify_catalyst_tolerance(&self, current_tolerance: Float) -> Float {
        (current_tolerance * (1.0 + self.transformation_velocity * 0.1)).min(1.0)
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn get_health_status(&self) -> HealthStatus {
        self.lambda.health_status()
    }
}

impl ArchetypeTrait for TransformationSpiritArchetype {
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
        ArchetypeRole::Transformation
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        let new_transmutation = self.calculate_material_transmutation_score();
        let new_infinite = self.calculate_infinite_realization_score();
        let new_contact = self.calculate_intelligent_infinity_contact_score();
        let new_velocity = self.calculate_transformation_velocity_score();

        self.material_transmutation = new_transmutation;
        self.infinite_realization = new_infinite;
        self.intelligent_infinity_contact = new_contact;
        self.transformation_velocity = new_velocity;

        self.recalculate_lambda();
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaC
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
        "Transformation of Spirit"
    }

    fn description(&self) -> &str {
        "Transformation of Spirit - The choice and abandonment of principle at the spiritual level"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::TransformationSingleton
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

    #[test]
    fn test_transformation_creation() {
        let transformation = TransformationSpiritArchetype::new();
        assert_eq!(transformation.archetype_id, 20);
        assert_eq!(transformation.complex(), ArchetypeComplex::Spirit);
        assert_eq!(transformation.role(), ArchetypeRole::Transformation);
    }
}
