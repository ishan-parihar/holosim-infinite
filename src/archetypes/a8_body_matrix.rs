// A8: The Matrix of Body
// Physical structure - Body's foundational functioning and balanced working
// Documentation: 01_Metaphysics/archetypes/A8_BodyMatrix.md

use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

// ============================================================================
// PHASE 4: MATRIX CAPACITY MECHANICS
// ============================================================================

/// Structural State - tracks Matrix evolution over time
#[derive(Debug, Clone)]
pub struct StructuralState {
    pub active: bool,

    pub timestamp: Float,
    pub complexity: Float,
    pub capacity: Float,
    pub load: Float,
    pub permeability: Float,
    pub coherence: Float,
}

/// A8: The Matrix of Body struct
///
/// The Matrix of the Body may be seen as Balanced Working or Even Functioning -
/// a reflection in opposites of the mind, representing unrestricted motion,
/// always active with no means of being inactive. It represents the foundational
/// functioning of the physical complex and records the results of experiments in novelty.
#[derive(Debug, Clone)]
pub struct MatrixBodyArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // Activation state
    pub active: bool,

    // Core Properties
    /// Balanced Working - Even functioning as fundamental state
    pub balanced_working: Float,
    /// Unrestricted Motion - Continuous physical activity and potential
    pub unrestricted_motion: Float,
    /// Experience Recording - Records experiments in novelty
    pub experience_recording: Float,
    /// Potentiator Regulation - How well Potentiator regulates activity
    pub potentiator_regulation: Float,

    // Veil Effects
    /// Knowledge of body (lost in veiling)
    pub body_knowledge: Float,
    /// Control over body (lost in veiling)
    pub body_control: Float,
    /// Veil lifting progress (0.0 = fully veiled, 1.0 = fully unveiled)
    pub veil_lifting_progress: Float,

    // Polarity
    /// Polarity assignment (female after veiling)
    pub polarity: Option<Polarity>,

    // ============================================================================
    // PHASE 4: MATRIX CAPACITY MECHANICS
    // ============================================================================
    /// Structural capacity - how much can Matrix hold? (0.0 - 1.0)
    pub structural_capacity: Float,
    /// Current load - how much is currently held? (0.0 - 1.0)
    pub current_load: Float,
    /// Processing efficiency - how well does it process? (0.0 - 1.0)
    pub processing_efficiency: Float,
    /// Accumulation rate - how fast is it accumulating? (0.0 - 1.0)
    pub accumulation_rate: Float,
    /// Structural complexity - how complex is the structure? (0.0 - 1.0)
    pub structural_complexity: Float,
    /// Structural adaptability - how well does it adapt? (0.0 - 1.0)
    pub structural_adaptability: Float,
    /// Structural history - how has it evolved?
    pub structural_history: Vec<StructuralState>,

    // Developmental
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,

    // Metadata
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,
}

impl MatrixBodyArchetype {
    /// Create a new Matrix of Body archetype
    pub fn new() -> Self {
        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.3);
        activation_levels.insert(Rung::R2, 0.25);
        activation_levels.insert(Rung::R3, 0.2);
        activation_levels.insert(Rung::R4, 0.15);
        activation_levels.insert(Rung::R5, 0.1);
        activation_levels.insert(Rung::R6, 0.05);
        activation_levels.insert(Rung::R7, 0.0);

        MatrixBodyArchetype {
            archetype_id: 8,
            active: true,
            lambda: LambdaMeasurement {
                value: 0.6, // Initial healthy value
                healthy_min: 0.5,
                healthy_max: 0.8,
                measurement_type: LambdaMeasurementType::MatrixRigidity,
            },
            tarot_correlation: TarotCorrelation::new(format!("Strength (VIII): Balanced working, unrestricted motion, control over physical body")),
            balanced_working: 0.6,
            unrestricted_motion: 0.6,
            experience_recording: 0.5,
            potentiator_regulation: 0.5,
            body_knowledge: 0.2, // Low due to veiling
            body_control: 0.2,   // Low due to veiling
            veil_lifting_progress: 0.0,
            polarity: None, // Will be assigned after veiling

            // Phase 4: Initialize capacity metrics
            structural_capacity: 0.5,
            current_load: 0.0,
            processing_efficiency: 0.6,
            accumulation_rate: 0.0,
            structural_complexity: 0.4,
            structural_adaptability: 0.5,
            structural_history: Vec::new(),

            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O3, 1),
            activated_rungs: vec![
                Rung::R1,
                Rung::R2,
                Rung::R3,
                Rung::R4,
            ],
            activation_levels,
            description: "The Matrix of the Body represents Balanced Working or Even Functioning - a reflection in opposites of the mind, representing unrestricted motion, always active with no means of being inactive. It records the results of experiments in novelty at cellular and energetic levels.".to_string(),
            holonic_level: HolonicLevel::Micro,
            integration_capacity: 0.5,
        }
    }

    /// Calculate body balance score
    ///
    /// Body Balance Score = (Balanced Working × Unrestricted Motion) / ||Matrix||
    pub fn calculate_body_balance_score(&self) -> Float {
        let matrix_norm = (self.balanced_working.powi(2)
            + self.unrestricted_motion.powi(2)
            + self.experience_recording.powi(2))
        .sqrt();

        if matrix_norm > 0.0 {
            (self.balanced_working * self.unrestricted_motion) / matrix_norm
        } else {
            0.0
        }
    }

    /// Record experience in body matrix
    ///
    /// Records experiments in novelty at cellular and energetic levels
    pub fn record_experience(&mut self, experience_intensity: Float) {
        // Record experience at cellular and energetic levels
        self.experience_recording =
            (self.experience_recording + experience_intensity * 0.1).min(1.0);
    }

    /// Update veil lifting progress
    ///
    /// As veil lifts, body knowledge and control increase
    pub fn update_veil_lifting(&mut self, progress: Float) {
        self.veil_lifting_progress = progress.clamp(0.0, 1.0);

        // Knowledge and control increase with veil lifting
        self.body_knowledge = 0.2 + (progress * 0.8);
        self.body_control = 0.2 + (progress * 0.8);

        // Assign polarity after veiling (female for Matrix of Body)
        if progress > 0.5 && self.polarity.is_none() {
            self.polarity = Some(Polarity::SinkholeOfIndifference); // Neutral before polarization
        }
    }

    /// Check if always active
    ///
    /// Matrix is always active with no means of being inactive
    pub fn is_always_active(&self) -> bool {
        true // Matrix is always active by definition
    }

    /// Calculate reflection in opposites of mind
    ///
    /// Matrix of Body reflects Matrix of Mind in opposites:
    /// - Matrix of Mind: Unmoving yet activating
    /// - Matrix of Body: Unrestricted motion
    pub fn calculate_reflection_score(&self, mind_matrix_rigidity: Float) -> Float {
        // Reflection in opposites: body motion vs. mind stillness
        1.0 - (mind_matrix_rigidity * self.unrestricted_motion).abs()
    }

    /// Get matrix expression for a specific octant
    pub fn get_octant_expression(&self, octant: Octant) -> String {
        match octant {
            Octant::O3 => {
                "Physical matrix: Individual physical functioning and balance".to_string()
            }
            Octant::O4 => {
                "Biological matrix: Systemic biological functioning and homeostasis".to_string()
            }
            Octant::O7 => "Behavioral matrix: Behavioral patterns and physical habits".to_string(),
            Octant(0_u8) => {
                /* invalid octant */
                "Invalid octant".to_string()
            }
            Octant(9_u8..=u8::MAX) => {
                /* invalid octant */
                "Invalid octant".to_string()
            }
            _ => "Not primary archetype for this octant".to_string(),
        }
    }

    /// Get matrix development for a specific rung
    pub fn get_rung_development(&self, rung: Rung) -> String {
        match rung {
            Rung::R1 => "Survival matrix: Basic physical functioning for survival".to_string(),
            Rung::R2 => "Sensation matrix: Sensation and pleasure functioning".to_string(),
            Rung::R3 => "Coordination matrix: Motor coordination and physical skill".to_string(),
            Rung::R4 => "Health matrix: Health maintenance and vitality".to_string(),
            Rung::R5 => "Integration matrix: Mind-body integration and harmony".to_string(),
            Rung::R6 => "Energy matrix: Subtle energy body functioning".to_string(),
            Rung::R7 => {
                "Transcendent matrix: Transcendent body functioning and light body".to_string()
            }
            Rung(0_u8) => {
                /* invalid rung */
                "Invalid rung".to_string()
            }
            Rung(8_u8..=u8::MAX) => {
                /* invalid rung */
                "Invalid rung".to_string()
            }
        }
    }

    /// Assess health status based on lambda and core properties
    pub fn assess_health(&self) -> crate::archetypes::common::HealthStatus {
        let lambda_value = self.lambda.value;

        if lambda_value >= 0.5 && lambda_value <= 0.8 {
            crate::archetypes::common::HealthStatus::Healthy
        } else if lambda_value < 0.5 {
            crate::archetypes::common::HealthStatus::PathologicalLow
        } else {
            crate::archetypes::common::HealthStatus::PathologicalHigh
        }
    }

    /// Get pathological indicators
    pub fn get_pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.balanced_working < 0.5 {
            indicators.push("Low balanced working: Imbalance in body functioning".to_string());
        }

        if self.unrestricted_motion < 0.5 {
            indicators
                .push("Restricted motion: Limited physical movement and potential".to_string());
        }

        if self.experience_recording < 0.5 {
            indicators
                .push("Poor experience recording: Experiments not recorded properly".to_string());
        }

        if self.body_knowledge < 0.3 {
            indicators.push("Low body knowledge: Veil effect limiting body awareness".to_string());
        }

        if self.body_control < 0.3 {
            indicators.push("Low body control: Veil effect limiting body regulation".to_string());
        }

        indicators
    }

    /// Process matrix dynamics
    pub fn process_dynamics(&mut self) {
        // Calculate lambda
        self.lambda.value = self.calculate_lambda();

        // Update integration capacity
        self.integration_capacity = self.calculate_integration_score();
    }

    /// Calculate integration score
    pub fn calculate_integration_score(&self) -> Float {
        // Integration = Balance × Motion × Recording
        self.balanced_working * self.unrestricted_motion * self.experience_recording
    }

    /// Get health indicators
    pub fn get_health_indicators(&self) -> Vec<(String, Float, bool)> {
        vec![
            (
                "Balanced Working".to_string(),
                self.balanced_working,
                self.balanced_working >= 0.5 && self.balanced_working <= 0.8,
            ),
            (
                "Unrestricted Motion".to_string(),
                self.unrestricted_motion,
                self.unrestricted_motion >= 0.5 && self.unrestricted_motion <= 0.8,
            ),
            (
                "Experience Recording".to_string(),
                self.experience_recording,
                self.experience_recording >= 0.5,
            ),
            (
                "Potentiator Regulation".to_string(),
                self.potentiator_regulation,
                self.potentiator_regulation >= 0.4,
            ),
        ]
    }

    /// Calculate Matrix's susceptibility to regulation (Body complex)
    /// Body: Potentiator REGULATES Matrix activity (wisdom guiding motion)
    pub fn calculate_regulatory_susceptibility(&self) -> Float {
        // Susceptibility = potentiator_regulation × unrestricted_motion
        // Higher regulation and more motion = more susceptible to guidance
        self.potentiator_regulation * self.unrestricted_motion
    }

    // ============================================================================
    // PHASE 4: MATRIX CAPACITY MECHANICS
    // ============================================================================

    /// Calculate available capacity
    pub fn available_capacity(&self) -> Float {
        (self.structural_capacity - self.current_load).max(0.0)
    }

    /// Check if Matrix is overloaded
    pub fn is_overloaded(&self) -> bool {
        self.current_load > self.structural_capacity * 0.9
    }

    /// Calculate accumulation rate
    pub fn calculate_accumulation_rate(&self) -> Float {
        // Accumulation = (load - processing_capacity) / capacity
        let processing_capacity = self.structural_capacity * self.processing_efficiency;
        (self.current_load - processing_capacity).max(0.0) / self.structural_capacity
    }

    /// Expand capacity based on integrated experience
    pub fn expand_capacity(&mut self, experience: Float) {
        // Capacity expansion = experience × learning_rate × expansion_factor
        let expansion = experience * 0.05 * 0.1;
        self.structural_capacity = (self.structural_capacity + expansion).min(1.0);

        // Improve processing efficiency
        let efficiency_improvement = experience * 0.05 * 0.08;
        self.processing_efficiency = (self.processing_efficiency + efficiency_improvement).min(1.0);

        // Increase structural complexity
        let complexity_increase = experience * 0.05 * 0.12;
        self.structural_complexity = (self.structural_complexity + complexity_increase).min(1.0);

        // Increase structural adaptability
        let adaptability_increase = experience * 0.05 * 0.15;
        self.structural_adaptability =
            (self.structural_adaptability + adaptability_increase).min(1.0);
    }

    /// Increase current load (when catalyst enters)
    pub fn increase_load(&mut self, load: Float) {
        self.current_load = (self.current_load + load).min(1.0);
        self.accumulation_rate = self.calculate_accumulation_rate();
    }

    /// Decrease current load (when catalyst is processed)
    pub fn decrease_load(&mut self, load: Float) {
        self.current_load = (self.current_load - load).max(0.0);
        self.accumulation_rate = self.calculate_accumulation_rate();
    }

    /// Record structural state
    pub fn record_structural_state(&mut self, timestamp: Float) {
        let state = StructuralState {
            active: true,
            timestamp,
            complexity: self.structural_complexity,
            capacity: self.structural_capacity,
            load: self.current_load,
            permeability: self.balanced_working, // Body uses balanced_working as permeability proxy
            coherence: self.integration_capacity,
        };
        self.structural_history.push(state);

        // Limit history size to 1000 entries
        if self.structural_history.len() > 1000 {
            self.structural_history.remove(0);
        }
    }

    // ============================================================================
    // PHASE 10: ADDITIONAL METHODS FOR TRAIT IMPLEMENTATION
    // ============================================================================

    /// Get structural permeability (Body uses balanced_working)
    pub fn get_structural_permeability(&self) -> Float {
        self.balanced_working
    }

    /// Set structural permeability (Body uses balanced_working)
    pub fn set_structural_permeability(&mut self, value: Float) {
        self.balanced_working = value.clamp(0.0, 1.0);
    }

    /// Get resource access (Body uses unrestricted_motion)
    pub fn get_resource_access(&self) -> Float {
        self.unrestricted_motion
    }

    /// Set resource access (Body uses unrestricted_motion)
    pub fn set_resource_access(&mut self, value: Float) {
        self.unrestricted_motion = value.clamp(0.0, 1.0);
    }

    /// Get willful integration (Body uses potentiator_regulation)
    pub fn get_willful_integration(&self) -> Float {
        self.potentiator_regulation
    }

    /// Set willful integration (Body uses potentiator_regulation)
    pub fn set_willful_integration(&mut self, value: Float) {
        self.potentiator_regulation = value.clamp(0.0, 1.0);
    }

    /// Get conscious coherence (Body uses experience_recording)
    pub fn get_conscious_coherence(&self) -> Float {
        self.experience_recording
    }

    /// Set conscious coherence (Body uses experience_recording)
    pub fn set_conscious_coherence(&mut self, value: Float) {
        self.experience_recording = value.clamp(0.0, 1.0);
    }

    /// Get structural tension (calculated from Body-specific dynamics)
    pub fn get_structural_tension(&self) -> Float {
        // Body tension = |balanced_working - potentiator_regulation|
        (self.balanced_working - self.potentiator_regulation).abs()
    }

    /// Set structural tension (not directly used in Body, but required by trait)
    pub fn set_structural_tension(&mut self, _value: Float) {
        // Body doesn't store structural tension directly
        // This is a no-op for Body
    }

    /// Calculate reaching intensity (Body: not applicable, returns 0.0)
    pub fn calculate_reaching_intensity(&self) -> Float {
        // Body complex: Potentiator regulates Matrix, not the other way around
        0.0
    }

    /// Calculate illumination receptivity (Body: not applicable, returns 0.0)
    pub fn calculate_illumination_receptivity(&self) -> Float {
        // Body complex: Potentiator regulates Matrix, not illuminates
        0.0
    }

    /// Calculate structural transformation (STO path for Body)
    pub fn calculate_structural_transformation(&self) -> Float {
        // Structural transformation = balanced_working × unrestricted_motion
        self.balanced_working * self.unrestricted_motion
    }

    /// Calculate state transformation (STS path for Body)
    pub fn calculate_state_transformation(&self) -> Float {
        // State transformation = potentiator_regulation × experience_recording
        self.potentiator_regulation * self.experience_recording
    }

    /// Update developmental position
    pub fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.developmental_position = position;
    }

    /// Set structural capacity
    pub fn set_structural_capacity(&mut self, value: Float) {
        self.structural_capacity = value.clamp(0.0, 1.0);
    }

    /// Set current load
    pub fn set_current_load(&mut self, value: Float) {
        self.current_load = value.clamp(0.0, 1.0);
    }

    /// Set processing efficiency
    pub fn set_processing_efficiency(&mut self, value: Float) {
        self.processing_efficiency = value.clamp(0.0, 1.0);
    }

    /// Set accumulation rate
    pub fn set_accumulation_rate(&mut self, value: Float) {
        self.accumulation_rate = value.clamp(0.0, 1.0);
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

impl ArchetypeTrait for MatrixBodyArchetype {
    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Matrix of Body"
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
        FunctionalPair::StructurePair
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        self.process_dynamics();
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn activate(&mut self, intensity: Float) {
        self.active = true;
        // Adjust lambda based on intensity
        let lambda = ArchetypeTrait::lambda(self);
        let new_value = (lambda.value + intensity * 0.1).clamp(0.0, 1.0);
        self.update_lambda(new_value);
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn is_healthy(&self) -> bool {
        let lambda = ArchetypeTrait::lambda(self);
        lambda.value >= lambda.healthy_min && lambda.value <= lambda.healthy_max
    }

    fn health_status(&self) -> HealthStatus {
        if self.is_healthy() {
            HealthStatus::Balanced
        } else if ArchetypeTrait::lambda(self).value < ArchetypeTrait::lambda(self).healthy_min {
            HealthStatus::PathologicalLow
        } else {
            HealthStatus::PathologicalHigh
        }
    }
}

impl LambdaMeasurable for MatrixBodyArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        // Lambda = Balance × Functioning × Motion
        let balance = self.balanced_working;
        let functioning = self.potentiator_regulation;
        let motion = self.unrestricted_motion;

        // Weighted calculation
        let lambda = (balance * 0.4) + (functioning * 0.3) + (motion * 0.3);

        lambda.clamp(0.0, 1.0)
    }

    fn healthy_range(&self) -> (Float, Float) {
        (self.lambda.healthy_min, self.lambda.healthy_max)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        self.get_pathological_indicators()
    }
}

impl Developmental for MatrixBodyArchetype {
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

impl Paired for MatrixBodyArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(9) // A9: Potentiator of Body
    }

    fn get_pair(&self) -> Option<u8> {
        Some(9)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        // Structure Pair: Matrix ↔ Potentiator
        // Tension = |Matrix - Potentiator|
        let matrix_balance = self.calculate_lambda();

        // Simplified: use paired archetype's lambda
        let potentiator_accessibility = 0.6; // Would get from paired archetype

        (matrix_balance - potentiator_accessibility).abs()
    }

    fn calculate_pair_balance(&self, _paired_archetype: &dyn Paired) -> Float {
        // Balance = Matrix × Potentiator
        let matrix_balance = self.calculate_lambda();
        let potentiator_accessibility = 0.6; // Would get from paired archetype

        matrix_balance * potentiator_accessibility
    }
}

impl Holonic for MatrixBodyArchetype {
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
        // Try to transcend to next holonic level
        match self.holonic_level {
            HolonicLevel::Micro => {
                if self.integration_capacity > 0.6 {
                    self.holonic_level = HolonicLevel::Meso;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meso => {
                if self.integration_capacity > 0.7 {
                    self.holonic_level = HolonicLevel::Macro;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Macro => {
                if self.integration_capacity > 0.8 {
                    self.holonic_level = HolonicLevel::Meta;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meta => false, // Already at highest level
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include lower level holon into this level
        let lower_capacity = lower_level.integration_capacity();

        // Integrate the lower level's capacity
        self.integration_capacity = (self.integration_capacity + lower_capacity) / 2.0;

        true
    }
}

// ============================================================================
// PHASE 10: MATRIX ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl super::archetype_traits::MatrixArchetypeTrait for MatrixBodyArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_structural_permeability(&self) -> Float {
        self.get_structural_permeability()
    }

    fn get_resource_access(&self) -> Float {
        self.get_resource_access()
    }

    fn get_willful_integration(&self) -> Float {
        self.get_willful_integration()
    }

    fn get_conscious_coherence(&self) -> Float {
        self.get_conscious_coherence()
    }

    fn get_structural_tension(&self) -> Float {
        self.get_structural_tension()
    }

    fn get_integration_capacity(&self) -> Float {
        self.integration_capacity
    }

    // Core setters
    fn set_structural_permeability(&mut self, value: Float) {
        self.set_structural_permeability(value);
    }

    fn set_resource_access(&mut self, value: Float) {
        self.set_resource_access(value);
    }

    fn set_willful_integration(&mut self, value: Float) {
        self.set_willful_integration(value);
    }

    fn set_conscious_coherence(&mut self, value: Float) {
        self.set_conscious_coherence(value);
    }

    fn set_structural_tension(&mut self, value: Float) {
        self.set_structural_tension(value);
    }

    fn set_integration_capacity(&mut self, value: Float) {
        self.integration_capacity = value.clamp(0.0, 1.0);
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

    // Phase 4: Capacity mechanics
    fn get_structural_capacity(&self) -> Float {
        self.structural_capacity
    }

    fn get_current_load(&self) -> Float {
        self.current_load
    }

    fn get_processing_efficiency(&self) -> Float {
        self.processing_efficiency
    }

    fn get_accumulation_rate(&self) -> Float {
        self.accumulation_rate
    }

    fn set_structural_capacity(&mut self, value: Float) {
        self.set_structural_capacity(value);
    }

    fn set_current_load(&mut self, value: Float) {
        self.set_current_load(value);
    }

    fn set_processing_efficiency(&mut self, value: Float) {
        self.set_processing_efficiency(value);
    }

    fn set_accumulation_rate(&mut self, value: Float) {
        self.set_accumulation_rate(value);
    }

    fn available_capacity(&self) -> Float {
        self.available_capacity()
    }

    fn is_overloaded(&self) -> bool {
        self.is_overloaded()
    }

    fn increase_load(&mut self, load: Float) {
        self.increase_load(load);
    }

    fn decrease_load(&mut self, load: Float) {
        self.decrease_load(load);
    }

    fn expand_capacity(&mut self, experience: Float) {
        self.expand_capacity(experience);
    }

    // Complex-specific M-P dynamics (Phase 2)
    fn calculate_reaching_intensity(&self) -> Float {
        self.calculate_reaching_intensity()
    }

    fn calculate_regulatory_susceptibility(&self) -> Float {
        self.calculate_regulatory_susceptibility()
    }

    fn calculate_illumination_receptivity(&self) -> Float {
        self.calculate_illumination_receptivity()
    }

    // Phase 3: Veil mechanics - transformation methods
    fn calculate_structural_transformation(&self) -> Float {
        self.calculate_structural_transformation()
    }

    fn calculate_state_transformation(&self) -> Float {
        self.calculate_state_transformation()
    }

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition) {
        self.update_developmental_position(position);
    }

    fn get_health_status(&self) -> HealthStatus {
        // Body Matrix health status based on lambda value
        if self.lambda.value >= 0.5 && self.lambda.value <= 0.8 {
            HealthStatus::Healthy
        } else if self.lambda.value < 0.3 || self.lambda.value > 0.9 {
            HealthStatus::Pathological
        } else {
            HealthStatus::Warning
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_matrix() -> MatrixBodyArchetype {
        MatrixBodyArchetype::new()
    }

    #[test]
    fn test_matrix_creation() {
        let matrix = create_test_matrix();
        assert_eq!(matrix.archetype_id, 8);
        assert_eq!(matrix.name(), "The Matrix of Body");
        assert_eq!(matrix.role(), ArchetypeRole::Matrix);
        assert_eq!(matrix.complex(), ArchetypeComplex::Body);
    }

    #[test]
    fn test_tarot_correlation() {
        let matrix = create_test_matrix();
        let tarot = matrix.tarot_correlation();
        assert!(tarot.card.contains("Strength"));
        assert!(tarot.card.contains("VIII"));
    }

    #[test]
    fn test_sigma_axis() {
        let matrix = create_test_matrix();
        assert_eq!(matrix.sigma_axis(), SigmaAxis::SigmaB);
    }

    #[test]
    fn test_balanced_working() {
        let mut matrix = create_test_matrix();
        assert!(matrix.balanced_working >= 0.0 && matrix.balanced_working <= 1.0);

        matrix.balanced_working = 0.8;
        assert_eq!(matrix.balanced_working, 0.8);
    }

    #[test]
    fn test_unrestricted_motion() {
        let matrix = create_test_matrix();
        assert!(matrix.unrestricted_motion >= 0.0 && matrix.unrestricted_motion <= 1.0);
    }

    #[test]
    fn test_always_active() {
        let matrix = create_test_matrix();
        assert!(matrix.is_always_active());
    }

    #[test]
    fn test_experience_recording() {
        let mut matrix = create_test_matrix();
        let initial_recording = matrix.experience_recording;

        matrix.record_experience(0.5);
        assert!(matrix.experience_recording > initial_recording);
    }

    #[test]
    fn test_veil_lifting() {
        let mut matrix = create_test_matrix();

        // Initial state (fully veiled)
        assert_eq!(matrix.veil_lifting_progress, 0.0);
        assert_eq!(matrix.body_knowledge, 0.2);
        assert_eq!(matrix.body_control, 0.2);

        // Lift veil
        matrix.update_veil_lifting(0.5);
        assert!(
            (matrix.veil_lifting_progress - 0.5).abs() < 0.001,
            "Expected ≈0.5, got {}",
            matrix.veil_lifting_progress
        );
        assert!(
            (matrix.body_knowledge - 0.6).abs() < 0.001,
            "Expected ≈0.6, got {}",
            matrix.body_knowledge
        );
        assert!(
            (matrix.body_control - 0.6).abs() < 0.001,
            "Expected ≈0.6, got {}",
            matrix.body_control
        );

        // Full veil lifting
        matrix.update_veil_lifting(1.0);
        assert_eq!(matrix.veil_lifting_progress, 1.0);
        assert_eq!(matrix.body_knowledge, 1.0);
        assert_eq!(matrix.body_control, 1.0);
    }

    #[test]
    fn test_body_balance_score() {
        let matrix = create_test_matrix();
        let balance_score = matrix.calculate_body_balance_score();

        assert!(balance_score >= 0.0 && balance_score <= 1.0);
    }

    #[test]
    fn test_reflection_in_opposites() {
        let matrix = create_test_matrix();
        let mind_matrix_rigidity = 0.5;
        let reflection_score = matrix.calculate_reflection_score(mind_matrix_rigidity);

        assert!(reflection_score >= 0.0 && reflection_score <= 1.0);
    }

    #[test]
    fn test_octant_expression() {
        let matrix = create_test_matrix();

        assert_eq!(
            matrix.get_octant_expression(Octant::O3),
            "Physical matrix: Individual physical functioning and balance"
        );
        assert_eq!(
            matrix.get_octant_expression(Octant::O4),
            "Biological matrix: Systemic biological functioning and homeostasis"
        );
        assert_eq!(
            matrix.get_octant_expression(Octant::O7),
            "Behavioral matrix: Behavioral patterns and physical habits"
        );
    }

    #[test]
    fn test_rung_development() {
        let matrix = create_test_matrix();

        assert_eq!(
            matrix.get_rung_development(Rung::R1),
            "Survival matrix: Basic physical functioning for survival"
        );
        assert_eq!(
            matrix.get_rung_development(Rung::R2),
            "Sensation matrix: Sensation and pleasure functioning"
        );
        assert_eq!(
            matrix.get_rung_development(Rung::R3),
            "Coordination matrix: Motor coordination and physical skill"
        );
        assert_eq!(
            matrix.get_rung_development(Rung::R4),
            "Health matrix: Health maintenance and vitality"
        );
        assert_eq!(
            matrix.get_rung_development(Rung::R5),
            "Integration matrix: Mind-body integration and harmony"
        );
        assert_eq!(
            matrix.get_rung_development(Rung::R6),
            "Energy matrix: Subtle energy body functioning"
        );
        assert_eq!(
            matrix.get_rung_development(Rung::R7),
            "Transcendent matrix: Transcendent body functioning and light body"
        );
    }

    #[test]
    fn test_lambda_calculation() {
        let matrix = create_test_matrix();
        let lambda = matrix.calculate_lambda();

        assert!(lambda >= 0.0 && lambda <= 1.0);
    }

    #[test]
    fn test_lambda_healthy_range() {
        let mut matrix = create_test_matrix();

        // Healthy range
        matrix.balanced_working = 0.6;
        matrix.unrestricted_motion = 0.6;
        matrix.potentiator_regulation = 0.6;
        let lambda = matrix.calculate_lambda();
        assert!(lambda >= 0.5 && lambda <= 0.8);

        // Too low
        matrix.balanced_working = 0.3;
        matrix.unrestricted_motion = 0.3;
        matrix.potentiator_regulation = 0.3;
        let lambda = matrix.calculate_lambda();
        assert!(lambda < 0.5);

        // Too high
        matrix.balanced_working = 0.9;
        matrix.unrestricted_motion = 0.9;
        matrix.potentiator_regulation = 0.9;
        let lambda = matrix.calculate_lambda();
        assert!(lambda > 0.8);
    }

    #[test]
    fn test_health_status() {
        let mut matrix = create_test_matrix();

        // Healthy
        matrix.lambda.value = 0.6;
        assert_eq!(matrix.health_status(), HealthStatus::Healthy);

        // Pathological - too low
        matrix.lambda.value = 0.3;
        assert_eq!(matrix.health_status(), HealthStatus::PathologicalLow);

        // Pathological - too high
        matrix.lambda.value = 0.9;
        assert_eq!(matrix.health_status(), HealthStatus::PathologicalHigh);
    }

    #[test]
    fn test_pathological_indicators() {
        let mut matrix = create_test_matrix();

        // Create pathology
        matrix.balanced_working = 0.3;
        matrix.unrestricted_motion = 0.3;
        matrix.experience_recording = 0.3;

        let indicators = matrix.get_pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators
            .iter()
            .any(|i| i.contains("Low balanced working")));
    }

    #[test]
    fn test_developmental_system() {
        let matrix = create_test_matrix();

        // Initially, Red rung should be activated
        assert!(matrix.activated_rungs().contains(&Rung::R1));

        let activation_level = matrix.rung_activation_level(Rung::R1);
        assert!(activation_level > 0.0);
    }

    #[test]
    fn test_functional_pair() {
        let matrix = create_test_matrix();

        assert_eq!(matrix.paired_archetype_id(), Some(9));
        assert_eq!(matrix.functional_pair(), FunctionalPair::StructurePair);
    }

    #[test]
    fn test_pair_dynamics() {
        let matrix = create_test_matrix();

        // Create a mock paired archetype for testing
        let tension = matrix.calculate_pair_tension(&matrix); // Using self as mock
        let balance = matrix.calculate_pair_balance(&matrix);

        assert!(tension >= 0.0 && tension <= 1.0);
        assert!(balance >= 0.0 && balance <= 1.0);
    }

    #[test]
    fn test_holonic_integration() {
        let matrix = create_test_matrix();
        let integration_score = matrix.calculate_integration_score();

        assert!(integration_score >= 0.0 && integration_score <= 1.0);
    }

    #[test]
    fn test_holonic_transcend() {
        let mut matrix = create_test_matrix();

        // Set high integration capacity
        matrix.integration_capacity = 0.7;

        let initial_level = matrix.holonic_level;
        matrix.transcend();

        // Should have transcended
        assert_ne!(matrix.holonic_level, initial_level);
    }

    #[test]
    fn test_holonic_include() {
        let mut matrix = create_test_matrix();
        let initial_integration = matrix.integration_capacity;

        // Include lower level - create a separate instance with higher capacity
        let mut lower_level = create_test_matrix();
        lower_level.integration_capacity = 0.8; // Higher capacity
        matrix.include(&lower_level);

        // Should have increased integration (averaging 0.5 and 0.8 = 0.65)
        assert!(matrix.integration_capacity > initial_integration);
    }

    #[test]
    fn test_archetype_trait_process() {
        let mut matrix = create_test_matrix();
        matrix.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Should have processed dynamics
        assert_eq!(matrix.lambda.value, matrix.calculate_lambda());
    }

    #[test]
    fn test_health_indicators() {
        let matrix = create_test_matrix();
        let indicators = matrix.get_health_indicators();

        assert_eq!(indicators.len(), 4);

        // Check structure
        for (name, value, _is_healthy) in indicators {
            assert!(!name.is_empty());
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn test_veil_clamping() {
        let mut matrix = create_test_matrix();

        // Test clamping at upper bound
        matrix.update_veil_lifting(1.5);
        assert_eq!(matrix.veil_lifting_progress, 1.0);

        // Test clamping at lower bound
        matrix.update_veil_lifting(-0.5);
        assert_eq!(matrix.veil_lifting_progress, 0.0);
    }

    #[test]
    fn test_experience_recording_clamping() {
        let mut matrix = create_test_matrix();

        // Test clamping at upper bound
        matrix.experience_recording = 0.9;
        matrix.record_experience(0.5);
        // 0.9 + (0.5 * 0.1) = 0.95
        assert!(
            (matrix.experience_recording - 0.95).abs() < 0.001,
            "Expected ≈0.95, got {}",
            matrix.experience_recording
        );

        // Test actual clamping - need more intensity to reach 1.0
        matrix.record_experience(0.6);
        // 0.95 + (0.6 * 0.1) = 1.01, clamped to 1.0
        assert!(
            (matrix.experience_recording - 1.0).abs() < 0.001,
            "Expected ≈1.0, got {}",
            matrix.experience_recording
        );
    }

    #[test]
    fn test_functional_pair_type() {
        let matrix = create_test_matrix();
        assert_eq!(matrix.functional_pair(), FunctionalPair::StructurePair);
    }

    #[test]
    fn test_octant_primary_expression() {
        let matrix = create_test_matrix();

        // Test primary octants for Body Matrix
        assert!(matrix
            .get_octant_expression(Octant::O3)
            .contains("Physical matrix"));
        assert!(matrix
            .get_octant_expression(Octant::O4)
            .contains("Biological matrix"));
        assert!(matrix
            .get_octant_expression(Octant::O7)
            .contains("Behavioral matrix"));
    }

    #[test]
    fn test_rung_development_progression() {
        let matrix = create_test_matrix();

        // Test that rung development follows progression
        let red = matrix.get_rung_development(Rung::R1);
        let orange = matrix.get_rung_development(Rung::R2);
        let yellow = matrix.get_rung_development(Rung::R3);
        let green = matrix.get_rung_development(Rung::R4);
        let blue = matrix.get_rung_development(Rung::R5);
        let indigo = matrix.get_rung_development(Rung::R6);
        let violet = matrix.get_rung_development(Rung::R7);

        assert!(red.contains("Survival"));
        assert!(orange.contains("Sensation"));
        assert!(yellow.contains("Coordination"));
        assert!(green.contains("Health"));
        assert!(blue.contains("Integration"));
        assert!(indigo.contains("Energy"));
        assert!(violet.contains("Transcendent"));
    }

    #[test]
    fn test_reflection_score_boundaries() {
        let matrix = create_test_matrix();

        // Test with extreme values
        let high_reflection = matrix.calculate_reflection_score(1.0);
        let low_reflection = matrix.calculate_reflection_score(0.0);

        assert!(high_reflection >= 0.0 && high_reflection <= 1.0);
        assert!(low_reflection >= 0.0 && low_reflection <= 1.0);
    }

    #[test]
    fn test_integration_capacity_bounds() {
        let mut matrix = create_test_matrix();

        // Test that integration capacity stays within bounds
        matrix.integration_capacity = 1.2;
        let lower_level = create_test_matrix();
        matrix.include(&lower_level); // Should clamp at 1.0
        assert!(matrix.integration_capacity <= 1.0);
    }

    #[test]
    fn test_multiple_transcend_attempts() {
        let mut matrix = create_test_matrix();

        // Set very high integration capacity
        matrix.integration_capacity = 0.95;

        // First transcend
        matrix.transcend();
        let level1 = matrix.holonic_level;

        // Second transcend
        matrix.transcend();
        let level2 = matrix.holonic_level;

        // Should have progressed
        assert_ne!(level1, level2);
    }

    #[test]
    fn test_transcend_failure() {
        let mut matrix = create_test_matrix();

        // Set low integration capacity
        matrix.integration_capacity = 0.3;

        let initial_level = matrix.holonic_level;
        let success = matrix.transcend();

        // Should fail to transcend
        assert!(!success);
        assert_eq!(matrix.holonic_level, initial_level);
    }

    #[test]
    fn test_pathological_indicators_comprehensive() {
        let mut matrix = create_test_matrix();

        // Create comprehensive pathology
        matrix.balanced_working = 0.2;
        matrix.unrestricted_motion = 0.2;
        matrix.experience_recording = 0.2;
        matrix.body_knowledge = 0.1;
        matrix.body_control = 0.1;

        let indicators = matrix.get_pathological_indicators();

        // Should detect multiple pathologies
        assert!(indicators.len() >= 3);
        assert!(indicators
            .iter()
            .any(|i| i.contains("Low balanced working")));
        assert!(indicators.iter().any(|i| i.contains("Restricted motion")));
        assert!(indicators
            .iter()
            .any(|i| i.contains("Poor experience recording")));
    }

    // ============================================================================
    // PHASE 4: MATRIX CAPACITY MECHANICS TESTS
    // ============================================================================

    #[test]
    fn test_available_capacity_body() {
        let mut matrix = create_test_matrix();
        matrix.structural_capacity = 0.7;
        matrix.current_load = 0.2;

        let capacity = matrix.available_capacity();
        assert!((capacity - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_is_overloaded_body() {
        let mut matrix = create_test_matrix();
        matrix.structural_capacity = 0.7;
        matrix.current_load = 0.65; // 92.8% of capacity

        assert!(matrix.is_overloaded());
    }

    #[test]
    fn test_calculate_accumulation_rate_body() {
        let mut matrix = create_test_matrix();
        matrix.structural_capacity = 0.7;
        matrix.processing_efficiency = 0.6;
        matrix.current_load = 0.6;

        // Processing capacity = 0.7 * 0.6 = 0.42
        // Accumulation = (0.6 - 0.42) / 0.7 = 0.18 / 0.7 ≈ 0.257
        let accumulation = matrix.calculate_accumulation_rate();
        assert!((accumulation - 0.257).abs() < 0.01);
    }

    #[test]
    fn test_expand_capacity_body() {
        let mut matrix = create_test_matrix();
        let initial_capacity = matrix.structural_capacity;
        let initial_efficiency = matrix.processing_efficiency;

        matrix.expand_capacity(0.4);

        assert!(matrix.structural_capacity > initial_capacity);
        assert!(matrix.processing_efficiency > initial_efficiency);
    }

    #[test]
    fn test_increase_load_body() {
        let mut matrix = create_test_matrix();
        matrix.structural_capacity = 0.7;
        matrix.current_load = 0.3;

        matrix.increase_load(0.2);

        assert_eq!(matrix.current_load, 0.5);
    }

    #[test]
    fn test_decrease_load_body() {
        let mut matrix = create_test_matrix();
        matrix.current_load = 0.6;

        matrix.decrease_load(0.3);

        assert_eq!(matrix.current_load, 0.3);
    }

    #[test]
    fn test_record_structural_state_body() {
        let mut matrix = create_test_matrix();
        matrix.structural_capacity = 0.7;
        matrix.current_load = 0.4;
        matrix.structural_complexity = 0.5;
        matrix.balanced_working = 0.6;
        matrix.integration_capacity = 0.7;

        matrix.record_structural_state(50.0);

        assert_eq!(matrix.structural_history.len(), 1);
        let state = &matrix.structural_history[0];
        assert_eq!(state.timestamp, 50.0);
        assert_eq!(state.capacity, 0.7);
        assert_eq!(state.load, 0.4);
    }

    #[test]
    fn test_capacity_metrics_initialization_body() {
        let matrix = create_test_matrix();

        assert!(matrix.structural_capacity >= 0.0 && matrix.structural_capacity <= 1.0);
        assert!(matrix.current_load >= 0.0 && matrix.current_load <= 1.0);
        assert!(matrix.processing_efficiency >= 0.0 && matrix.processing_efficiency <= 1.0);
        assert!(matrix.accumulation_rate >= 0.0 && matrix.accumulation_rate <= 1.0);
        assert!(matrix.structural_complexity >= 0.0 && matrix.structural_complexity <= 1.0);
        assert!(matrix.structural_adaptability >= 0.0 && matrix.structural_adaptability <= 1.0);
    }
}
