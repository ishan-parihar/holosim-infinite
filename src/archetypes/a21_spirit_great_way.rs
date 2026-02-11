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

/// A21: The Great Way of Spirit - Environment Container
///
/// Represents contact with intelligent infinity and intelligent energy (the Universe or World).
/// Contact with intelligent energy is energy of Logos which heals, builds, removes, destroys,
/// and transforms all other-selves as well as self.
///
/// # Core Characteristics
///
/// - **Contact with Intelligent Infinity**: Direct contact with intelligent infinity
/// - **Contact with Intelligent Energy**: Logos energy (heals, builds, removes, destroys, transforms)
/// - **Unspeakable Joy**: Contact produces unspeakable joy
/// - **Cycle Completion**: Completion of archetypical cycle (21st archetype)
/// - **Transcends Veil**: Unlike Significator, transcends veiling process
///
/// # Tarot Correlation
///
/// The World (XXI) - Represents contact with intelligent energy which is Universe or World,
/// symbolizing completion, wholeness, and integration of all aspects.
///
/// # Lambda Measurement
///
/// Type: GreatWayClarity
/// - Healthy Range: 0.5 - 0.8 (direct intelligent infinity contact with effective Logos energy)
/// - Pathological Low: < 0.5 (separation, no intelligent infinity contact, no joy)
/// - Pathological High: > 0.8 (premature transcendence, bypassing necessary integration, loss of grounding)
///
/// # Mathematical Formulation
///
/// ```text
/// Λ_GreatWay_Spirit = Completion × IntelligentInfinityContact × Joy
///
/// Completion Score = (IntelligentInfinityContact × LogosEnergy) / ||GreatWay||
///
/// LogosEnergy = Heals × Builds × Removes × Destroys × Transforms
/// ```
#[derive(Debug, Clone)]
pub struct GreatWaySpiritArchetype {
    pub archetype_id: u8,
    pub active: bool,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,
    // A21-specific fields
    pub intelligent_infinity_contact: Float,
    pub logos_energy: Float,
    pub logos_heals: Float,
    pub logos_builds: Float,
    pub logos_removes: Float,
    pub logos_destroys: Float,
    pub logos_transforms: Float,
    pub unspeakable_joy: Float,
    pub cycle_completion: Float,
    pub veil_transcendence: Float,
    pub environment_time_space: Float,
    pub significator_difference: Float,
    // Developmental fields
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
    pub polarity: Polarity,

    // Phase 7: Current milieu for Lesser Cycle
    pub current_milieu: Milieu,
}

impl ArchetypeTrait for GreatWaySpiritArchetype {
    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "Great Way of Spirit"
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn role(&self) -> ArchetypeRole {
        ArchetypeRole::GreatWay
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::TransformationSingleton
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SpiritCapacity
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation().clone()
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn is_healthy(&self) -> bool {
        self.lambda.is_healthy()
    }

    fn health_status(&self) -> HealthStatus {
        if self.lambda.is_healthy() && self.intelligent_infinity_contact > 0.5 {
            HealthStatus::Healthy
        } else if self.lambda.is_pathological_low() {
            HealthStatus::Degraded
        } else if self.lambda.is_pathological_high() {
            HealthStatus::PathologicalHigh
        } else {
            HealthStatus::Warning
        }
    }

    fn activate(&mut self, intensity: Float) {
        self.lambda.adjust(intensity * 0.1);
    }

    fn deactivate(&mut self) {
        self.lambda.adjust(-0.1);
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        // Process catalyst based on developmental position
        let rung_level = position.rung_level() as Float / 7.0;
        let processing_capacity = self.unspeakable_joy * rung_level;

        // Update lambda based on catalyst
        self.lambda.value = (self.lambda.value + catalyst * processing_capacity * 0.1).min(1.0);
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
    }
}

impl GreatWaySpiritArchetype {
    /// Creates a new Great Way of Spirit archetype with healthy initial values
    ///
    /// # Initial Values
    ///
    /// - Lambda: 0.65 (within healthy range 0.5-0.8)
    /// - Intelligent Infinity Contact: 0.65 (direct contact)
    /// - Logos Energy: 0.65 (effective functioning)
    /// - Unspeakable Joy: 0.65 (producing joy)
    /// - Cycle Completion: 0.65 (approaching completion)
    /// - Developmental Position: Octant O1, Rung R6 (Transpersonal, Indigo)
    pub fn new() -> Self {
        let initial_lambda = 0.65;
        let healthy_min = 0.5;
        let healthy_max = 0.8;

        GreatWaySpiritArchetype {
            archetype_id: 21,
            active: true,
            lambda: LambdaMeasurement::new(initial_lambda, LambdaMeasurementType::GreatWayClarity),
            tarot_correlation: TarotCorrelation::new(format!("The World (XXI): Completion, wholeness, integration, contact with intelligent infinity")),
            intelligent_infinity_contact: 0.65,
            logos_energy: 0.65,
            logos_heals: 0.65,
            logos_builds: 0.65,
            logos_removes: 0.65,
            logos_destroys: 0.65,
            logos_transforms: 0.65,
            unspeakable_joy: 0.65,
            cycle_completion: 0.65,
            veil_transcendence: 0.65,
            environment_time_space: 0.65,
            significator_difference: 0.7,
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O1, 6),
            activated_rungs: vec![Rung::R4, Rung::R5, Rung::R6],
            activation_levels: {
                let mut levels = HashMap::new();
                levels.insert(Rung::R4, 0.5);
                levels.insert(Rung::R5, 0.6);
                levels.insert(Rung::R6, 0.65);
                levels
            },
            description: String::from(
                "Great Way of Spirit - Contact with intelligent infinity and intelligent energy. \
                 Logos energy heals, builds, removes, destroys, and transforms all other-selves \
                 as well as self. Contact produces unspeakable joy. Completion of archetypical cycle.",
            ),
            holonic_level: HolonicLevel::Macro,
            integration_capacity: 0.65,
            polarity: Polarity::STO,

            // Phase 7: Initialize default milieu
            current_milieu: Milieu::new(),
        }
    }

    // ===== Calculation Methods =====

    /// Calculates intelligent infinity contact based on veil transcendence
    pub fn calculate_intelligent_infinity_contact(&self) -> Float {
        // Direct contact increases with veil transcendence
        let veil_factor = self.veil_transcendence * 0.6;
        let energy_factor = self.logos_energy * 0.4;
        veil_factor + energy_factor
    }

    /// Calculates Logos energy based on its five functions
    pub fn calculate_logos_energy(&self) -> Float {
        let functions = vec![
            self.logos_heals,
            self.logos_builds,
            self.logos_removes,
            self.logos_destroys,
            self.logos_transforms,
        ];
        let sum: Float = functions.iter().sum();
        sum / functions.len() as Float
    }

    /// calculates unspeakable joy based on intelligent infinity contact
    pub fn calculate_unspeakable_joy(&self) -> Float {
        // Joy is most likely to be produced by intelligent infinity contact
        let contact_factor = self.intelligent_infinity_contact * 0.7;
        let completion_factor = self.cycle_completion * 0.3;
        contact_factor + completion_factor
    }

    /// Calculates cycle completion based on integration of all aspects
    pub fn calculate_cycle_completion(&self) -> Float {
        // Completion increases with intelligent infinity contact and Logos energy
        let contact_factor = self.intelligent_infinity_contact * 0.4;
        let logos_factor = self.logos_energy * 0.3;
        let joy_factor = self.unspeakable_joy * 0.3;
        contact_factor + logos_factor + joy_factor
    }

    /// Calculates veil transcendence based on difference from Significator
    pub fn calculate_veil_transcendence(&self) -> Float {
        // Veil transcendence increases with difference from Significator
        let diff_factor = self.significator_difference * 0.7;
        let contact_factor = self.intelligent_infinity_contact * 0.3;
        diff_factor + contact_factor
    }

    /// Calculates environment drawing from time/space
    pub fn calculate_environment_time_space(&self) -> Float {
        // Environment drawing increases with cycle completion
        let completion_factor = self.cycle_completion * 0.6;
        let logos_factor = self.logos_energy * 0.4;
        completion_factor + logos_factor
    }

    /// Calculates difference from Significator
    pub fn calculate_significator_difference(&self) -> Float {
        // Difference increases with veil transcendence and intelligent infinity contact
        let veil_factor = self.veil_transcendence * 0.6;
        let contact_factor = self.intelligent_infinity_contact * 0.4;
        veil_factor + contact_factor
    }

    /// Check if the archetype is healthy
    pub fn is_healthy(&self) -> bool {
        self.lambda.is_healthy()
    }

    /// Process catalyst
    pub fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        // Process catalyst through the archetype
        self.lambda.adjust(0.05);
    }

    /// Calculates lambda based on cycle completion, intelligent infinity contact, and joy
    pub fn calculate_lambda_from_state(&self) -> Float {
        let completion_factor = self.cycle_completion * 0.4;
        let contact_factor = self.intelligent_infinity_contact * 0.35;
        let joy_factor = self.unspeakable_joy * 0.25;
        completion_factor + contact_factor + joy_factor
    }

    // ===== Update Methods =====

    /// Updates intelligent infinity contact
    pub fn update_intelligent_infinity_contact(&mut self, value: Float) {
        self.intelligent_infinity_contact = value.max(0.0).min(1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates Logos energy
    pub fn update_logos_energy(&mut self, value: Float) {
        self.logos_energy = value.max(0.0).min(1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates Logos heals function
    pub fn update_logos_heals(&mut self, value: Float) {
        self.logos_heals = value.max(0.0).min(1.0);
        self.logos_energy = self.calculate_logos_energy();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates Logos builds function
    pub fn update_logos_builds(&mut self, value: Float) {
        self.logos_builds = value.max(0.0).min(1.0);
        self.logos_energy = self.calculate_logos_energy();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates Logos removes function
    pub fn update_logos_removes(&mut self, value: Float) {
        self.logos_removes = value.max(0.0).min(1.0);
        self.logos_energy = self.calculate_logos_energy();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates Logos destroys function
    pub fn update_logos_destroys(&mut self, value: Float) {
        self.logos_destroys = value.max(0.0).min(1.0);
        self.logos_energy = self.calculate_logos_energy();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates Logos transforms function
    pub fn update_logos_transforms(&mut self, value: Float) {
        self.logos_transforms = value.max(0.0).min(1.0);
        self.logos_energy = self.calculate_logos_energy();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates unspeakable joy
    pub fn update_unspeakable_joy(&mut self, value: Float) {
        self.unspeakable_joy = value.max(0.0).min(1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates cycle completion
    pub fn update_cycle_completion(&mut self, value: Float) {
        self.cycle_completion = value.max(0.0).min(1.0);
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates veil transcendence
    pub fn update_veil_transcendence(&mut self, value: Float) {
        self.veil_transcendence = value.max(0.0).min(1.0);
        self.intelligent_infinity_contact = self.calculate_intelligent_infinity_contact();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Updates environment drawing from time/space
    pub fn update_environment_time_space(&mut self, value: Float) {
        self.environment_time_space = value.max(0.0).min(1.0);
    }

    /// Updates difference from Significator
    pub fn update_significator_difference(&mut self, value: Float) {
        self.significator_difference = value.max(0.0).min(1.0);
        self.veil_transcendence = self.calculate_veil_transcendence();
        self.lambda.value = self.calculate_lambda_from_state();
    }

    /// Recalculates all fields based on current state
    pub fn recalculate_all_fields(&mut self) {
        self.logos_energy = self.calculate_logos_energy();
        self.intelligent_infinity_contact = self.calculate_intelligent_infinity_contact();
        self.unspeakable_joy = self.calculate_unspeakable_joy();
        self.cycle_completion = self.calculate_cycle_completion();
        self.veil_transcendence = self.calculate_veil_transcendence();
        self.environment_time_space = self.calculate_environment_time_space();
        self.significator_difference = self.calculate_significator_difference();
        self.lambda.value = self.calculate_lambda_from_state();
        self.integration_capacity = self.calculate_integration_capacity_from_state();
    }

    // ===== Query Methods =====

    /// Checks if Great Way has direct intelligent infinity contact
    pub fn has_direct_intelligent_infinity_contact(&self) -> bool {
        self.intelligent_infinity_contact >= 0.5
    }

    /// Checks if Logos energy is functioning effectively
    pub fn has_effective_logos_energy(&self) -> bool {
        self.logos_energy >= 0.5
    }

    /// Checks if producing unspeakable joy
    pub fn is_producing_unspeakable_joy(&self) -> bool {
        self.unspeakable_joy >= 0.5
    }

    /// Checks if cycle is approaching completion
    pub fn is_approaching_completion(&self) -> bool {
        self.cycle_completion >= 0.5
    }

    /// Checks if transcending veil
    pub fn is_transcending_veil(&self) -> bool {
        self.veil_transcendence >= 0.5
    }

    /// Checks if different from Significator
    pub fn is_different_from_significator(&self) -> bool {
        self.significator_difference >= 0.5
    }

    /// Gets Logos energy function effectiveness
    pub fn get_logos_functions(&self) -> (Float, Float, Float, Float, Float) {
        (
            self.logos_heals,
            self.logos_builds,
            self.logos_removes,
            self.logos_destroys,
            self.logos_transforms,
        )
    }

    /// Gets completion status
    pub fn get_completion_status(&self) -> String {
        if self.cycle_completion >= 0.8 {
            String::from("Near Complete")
        } else if self.cycle_completion >= 0.5 {
            String::from("In Progress")
        } else {
            String::from("Early Stage")
        }
    }

    /// Gets contact status
    pub fn get_contact_status(&self) -> String {
        if self.intelligent_infinity_contact >= 0.8 {
            String::from("Direct Contact")
        } else if self.intelligent_infinity_contact >= 0.5 {
            String::from("Approaching Contact")
        } else {
            String::from("No Contact")
        }
    }

    /// Gets joy status
    pub fn get_joy_status(&self) -> String {
        if self.unspeakable_joy >= 0.8 {
            String::from("Unspeakable Joy")
        } else if self.unspeakable_joy >= 0.5 {
            String::from("Increasing Joy")
        } else {
            String::from("No Joy")
        }
    }

    /// Calculates integration capacity from state
    pub fn calculate_integration_capacity_from_state(&self) -> Float {
        let contact_factor = self.intelligent_infinity_contact * 0.35;
        let logos_factor = self.logos_energy * 0.3;
        let joy_factor = self.unspeakable_joy * 0.2;
        let completion_factor = self.cycle_completion * 0.15;
        contact_factor + logos_factor + joy_factor + completion_factor
    }

    // ============================================================================
    // PHASE 7: MILIEU DEFINITION AND APPLICATION (Greater Cycle Regulation)
    // ============================================================================

    /// Create milieu based on intelligent infinity contact and Logos energy
    ///
    /// Great Way defines the MILIEU (time/space/entropy) for Lesser Cycle
    pub fn create_milieu(&self) -> Milieu {
        // Milieu factors based on intelligent infinity contact and Logos energy
        let time_factor = 1.0 + (self.veil_transcendence - 0.5) * 0.4;
        let space_factor = 1.0 + (self.cycle_completion - 0.5) * 0.4;
        let entropy_factor = 1.0 + (self.logos_energy - 0.5) * 0.3;
        let creativity_factor = 1.0 + (self.intelligent_infinity_contact - 0.5) * 0.5;
        let stability_factor = 1.0 + (self.unspeakable_joy - 0.5) * 0.3;

        Milieu::with_factors(
            time_factor,
            space_factor,
            entropy_factor,
            creativity_factor,
            stability_factor,
        )
    }

    /// Update current milieu
    pub fn update_milieu(&mut self) {
        self.current_milieu = self.create_milieu();
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

impl Developmental for GreatWaySpiritArchetype {
    fn develop(&mut self, catalyst: Float) {
        self.lambda.adjust(catalyst * 0.1);
    }

    fn regress(&mut self, resistance: Float) {
        self.lambda.adjust(-resistance * 0.1);
    }

    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    fn rung_activation_level(&self, rung: Rung) -> Float {
        self.activation_levels.get(&rung).copied().unwrap_or(0.0)
    }

    fn activated_rungs(&self) -> Vec<Rung> {
        self.activated_rungs.clone()
    }

    fn developmental_position(&self) -> DevelopmentalPosition {
        self.developmental_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock archetype for testing paired relationships
    struct MockArchetype {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockArchetype {
        fn new(lambda_value: Float) -> Self {
            MockArchetype {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::SignificatorCoherence,
                ),
                tarot: TarotCorrelation::new("The Sun".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockArchetype {
        fn archetype_id(&self) -> u8 {
            19
        }

        fn name(&self) -> &str {
            "Mock Transformation Spirit"
        }

        fn description(&self) -> &str {
            "Mock archetype for testing"
        }

        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
        }

        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Transformation
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
