// A15: The Matrix of the Spirit (Night of the Soul / The Devil)
// Input container - Night of the Soul, Primeval Darkness, extremely receptive matrix

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

/// Matrix of Spirit Archetype - Night of the Soul / The Devil
///
/// The Matrix of the Spirit is what you may call the Night of the Soul or Primeval Darkness -
/// that which is not capable of movement or work; the potential power of this extremely receptive
/// matrix is such that the potentiator may be seen as Lightning.
///
/// **Key Characteristics:**
/// - Night of the Soul or Primeval Darkness
/// - Not capable of movement or work (similar to Matrix of Mind)
/// - Extremely receptive matrix capable of receiving potentiating influence
/// - Potential power such that potentiator may be seen as Lightning
/// - Infinitely subtle nature of spirit
/// - Fructifying influence of light upon great darkness often not as apparent as darkness itself
///
/// **Lambda Measurement:** Spirit matrix receptivity and darkness depth (0.5 - 0.8 healthy range)
/// - < 0.5: Blindness, stuck in darkness, confusion
/// - 0.5 - 0.8: Deep receptivity with lightning activation and moonlight discernment
/// - > 0.8: Over-sensitivity, overwhelmed by darkness, loss of stability
#[derive(Debug, Clone)]
pub struct MatrixSpiritArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,

    // Activation state
    pub active: bool,

    // A15-specific fields
    /// Matrix receptivity - how receptive is matrix to lightning (0.0-1.0)
    pub matrix_receptivity: Float,
    /// Darkness depth - how deep is night of the soul (0.0-1.0)
    pub darkness_depth: Float,
    /// Lightning activation - how effectively does potentiator activate matrix (0.0-1.0)
    pub lightning_activation: Float,
    /// Moonlight discernment - how well can adept discern truth from falsity (0.0-1.0)
    pub moonlight_discernment: Float,
    /// Infinite potential - holds infinite potential (0.0-1.0)
    pub infinite_potential: Float,
    /// Spiritual subtlety - understanding of infinitely subtle nature (0.0-1.0)
    pub spiritual_subtlety: Float,

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
}

impl MatrixSpiritArchetype {
    /// Create a new Matrix of Spirit archetype with healthy initial values
    pub fn new() -> Self {
        let mut lambda = LambdaMeasurement::new(0.65, LambdaMeasurementType::MatrixRigidity);
        lambda.healthy_min = 0.5;
        lambda.healthy_max = 0.8;

        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.4);
        activation_levels.insert(Rung::R2, 0.5);
        activation_levels.insert(Rung::R3, 0.6);
        activation_levels.insert(Rung::R4, 0.7);
        activation_levels.insert(Rung::R5, 0.6);
        activation_levels.insert(Rung::R6, 0.5);
        activation_levels.insert(Rung::R7, 0.4);

        MatrixSpiritArchetype {
            archetype_id: 15,
            active: true,
            lambda,
            tarot_correlation: TarotCorrelation::new("The Devil (XV): Materialism, bondage, shadow work, liberation through facing darkness".to_string()),

            // A15-specific fields - healthy initial values
            matrix_receptivity: 0.65,
            darkness_depth: 0.65,
            lightning_activation: 0.65,
            moonlight_discernment: 0.65,
            infinite_potential: 1.0, // Infinite potential is always present
            spiritual_subtlety: 0.65,

            // Developmental tracking
            developmental_position: DevelopmentalPosition::new_with_octant_rung(Octant::O1, 4),
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,

            // Holonic integration
            description: "The Matrix of the Spirit is the Night of the Soul or Primeval Darkness - that which is not capable of movement or work; the potential power of this extremely receptive matrix is such that the potentiator may be seen as Lightning.".to_string(),
            holonic_level: HolonicLevel::Micro,
            integration_capacity: 0.65,

            // Polarity (female after veiling)
            polarity: Polarity::STO,

            // Phase 4: Initialize capacity metrics
            structural_capacity: 0.65,
            current_load: 0.0,
            processing_efficiency: 0.7,
            accumulation_rate: 0.0,
            structural_complexity: 0.6,
            structural_adaptability: 0.65,
            structural_history: Vec::new(),
        }
    }

    /// Calculate spirit matrix harmony based on receptivity and darkness
    pub fn calculate_spirit_matrix_harmony(&self) -> Float {
        let receptivity_factor = self.matrix_receptivity;
        let darkness_factor = self.darkness_depth;
        let lightning_factor = self.lightning_activation;

        // Harmony balances receptivity, darkness, and lightning activation
        receptivity_factor * 0.4 + darkness_factor * 0.3 + lightning_factor * 0.3
    }

    /// Update matrix receptivity based on potentiator influence
    pub fn update_matrix_receptivity(&mut self, potentiator_influence: Float) {
        // Receptivity increases with potentiator influence
        let influence = potentiator_influence.clamp(0.0, 1.0);
        self.matrix_receptivity = (self.matrix_receptivity * 0.7 + influence * 0.3)
            .clamp(0.0, 1.0);
    }

    /// Update darkness depth based on night of soul experience
    pub fn update_darkness_depth(&mut self, night_experience: Float) {
        // Darkness deepens with night of soul experience
        let experience = night_experience.clamp(0.0, 1.0);
        self.darkness_depth = (self.darkness_depth * 0.8 + experience * 0.2)
            .clamp(0.0, 1.0);
    }

    /// Update lightning activation based on potentiator lightning
    pub fn update_lightning_activation(&mut self, lightning_intensity: Float) {
        // Lightning activation increases with potentiator intensity
        let intensity = lightning_intensity.clamp(0.0, 1.0);
        self.lightning_activation = (self.lightning_activation * 0.6 + intensity * 0.4)
            .clamp(0.0, 1.0);
    }

    /// Update moonlight discernment based on adept navigation
    pub fn update_moonlight_discernment(&mut self, navigation_skill: Float) {
        // Discernment improves with navigation skill
        let skill = navigation_skill.clamp(0.0, 1.0);
        self.moonlight_discernment = (self.moonlight_discernment * 0.7 + skill * 0.3)
            .clamp(0.0, 1.0);
    }

    /// Update spiritual subtlety based on understanding
    pub fn update_spiritual_subtlety(&mut self, understanding: Float) {
        // Subtlety improves with understanding
        let understanding = understanding.clamp(0.0, 1.0);
        self.spiritual_subtlety = (self.spiritual_subtlety * 0.7 + understanding * 0.3)
            .clamp(0.0, 1.0);
    }

    /// Get diagnostic information about the spirit matrix
    pub fn get_diagnostic_info(&self) -> String {
        let harmony = self.calculate_spirit_matrix_harmony();
        let health = self.lambda.health_status();

        format!(
            "Matrix of Spirit Diagnostics:\n\
             - Harmony: {:.2}\n\
             - Receptivity: {:.2}\n\
             - Darkness Depth: {:.2}\n\
             - Lightning Activation: {:.2}\n\
             - Moonlight Discernment: {:.2}\n\
             - Spiritual Subtlety: {:.2}\n\
             - Health Status: {}",
            harmony,
            self.matrix_receptivity,
            self.darkness_depth,
            self.lightning_activation,
            self.moonlight_discernment,
            self.spiritual_subtlety,
            health
        )
    }

    /// Calculate Matrix's receptivity to illumination (Spirit complex)
    /// Spirit: Potentiator ILLUMINATES Matrix (lightning striking primeval darkness)
    pub fn calculate_illumination_receptivity(&self) -> Float {
        // Receptivity = integration_capacity × (1 - darkness_level)
        let darkness_level = self.darkness_depth;
        self.integration_capacity * (1.0 - darkness_level)
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
            permeability: self.matrix_receptivity,
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

    /// Get structural permeability (Spirit uses matrix_receptivity)
    pub fn get_structural_permeability(&self) -> Float {
        self.matrix_receptivity
    }

    /// Set structural permeability (Spirit uses matrix_receptivity)
    pub fn set_structural_permeability(&mut self, value: Float) {
        self.matrix_receptivity = value.clamp(0.0, 1.0);
    }

    /// Get resource access (Spirit uses darkness_depth)
    pub fn get_resource_access(&self) -> Float {
        self.darkness_depth
    }

    /// Set resource access (Spirit uses darkness_depth)
    pub fn set_resource_access(&mut self, value: Float) {
        self.darkness_depth = value.clamp(0.0, 1.0);
    }

    /// Get willful integration (Spirit uses lightning_activation)
    pub fn get_willful_integration(&self) -> Float {
        self.lightning_activation
    }

    /// Set willful integration (Spirit uses lightning_activation)
    pub fn set_willful_integration(&mut self, value: Float) {
        self.lightning_activation = value.clamp(0.0, 1.0);
    }

    /// Get conscious coherence (Spirit uses moonlight_discernment)
    pub fn get_conscious_coherence(&self) -> Float {
        self.moonlight_discernment
    }

    /// Set conscious coherence (Spirit uses moonlight_discernment)
    pub fn set_conscious_coherence(&mut self, value: Float) {
        self.moonlight_discernment = value.clamp(0.0, 1.0);
    }

    /// Get structural tension (calculated from Spirit-specific dynamics)
    pub fn get_structural_tension(&self) -> Float {
        // Spirit tension = |matrix_receptivity - darkness_depth|
        (self.matrix_receptivity - self.darkness_depth).abs()
    }

    /// Set structural tension (not directly used in Spirit, but required by trait)
    pub fn set_structural_tension(&mut self, _value: Float) {
        // Spirit doesn't store structural tension directly
        // This is a no-op for Spirit
    }

    /// Calculate reaching intensity (Spirit: not applicable, returns 0.0)
    pub fn calculate_reaching_intensity(&self) -> Float {
        // Spirit complex: Potentiator illuminates Matrix, not the other way around
        0.0
    }

    /// Calculate regulatory susceptibility (Spirit: not applicable, returns 0.0)
    pub fn calculate_regulatory_susceptibility(&self) -> Float {
        // Spirit complex: Potentiator illuminates Matrix, not regulates
        0.0
    }

    /// Calculate structural transformation (STO path for Spirit)
    pub fn calculate_structural_transformation(&self) -> Float {
        // Structural transformation = matrix_receptivity × moonlight_discernment
        self.matrix_receptivity * self.moonlight_discernment
    }

    /// Calculate state transformation (STS path for Spirit)
    pub fn calculate_state_transformation(&self) -> Float {
        // State transformation = lightning_activation × darkness_depth
        self.lightning_activation * self.darkness_depth
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

impl ArchetypeTrait for MatrixSpiritArchetype {
    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "Matrix of Spirit"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Spirit
    }

    fn role(&self) -> ArchetypeRole {
        ArchetypeRole::Matrix
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaC
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::StructurePair
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        // Update lambda based on spirit matrix harmony
        let harmony = self.calculate_spirit_matrix_harmony();
        self.update_lambda(harmony);

        // Update developmental position based on activation
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
            let current_level = self.activation_levels.get(&rung).copied().unwrap_or(0.0);
            let new_level = (current_level * 0.95 + harmony * 0.05).clamp(0.0, 1.0);
            self.activation_levels.insert(rung, new_level);
        }

        // Update integration capacity
        self.integration_capacity = harmony;
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn activate(&mut self, intensity: Float) {
        self.active = true;
        // Adjust lambda based on intensity
        let new_value = (self.lambda.value + intensity * 0.1).clamp(0.0, 1.0);
        self.update_lambda(new_value);
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn is_active(&self) -> bool {
        self.active
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
}

impl LambdaMeasurable for MatrixSpiritArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        // Lambda is the spirit matrix harmony
        self.calculate_spirit_matrix_harmony()
    }

    fn healthy_range(&self) -> (Float, Float) {
        (0.5, 0.8)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.value < 0.5 {
            indicators.push("Blindness - unable to receive lightning".to_string());
            indicators.push("Stuck in darkness - trapped in primeval darkness".to_string());
            indicators.push("Confusion - confused path attempting to use catalyst".to_string());
        }

        if self.lambda.value > 0.8 {
            indicators.push("Over-sensitivity - overwhelmed by darkness".to_string());
            indicators.push("Loss of stability - overwhelmed by infinite potential".to_string());
        }

        if self.moonlight_discernment < 0.5 {
            indicators
                .push("Deception by moonlight - cannot discern truth from falsity".to_string());
        }

        if self.spiritual_subtlety < 0.5 {
            indicators.push("Limited understanding of infinitely subtle nature".to_string());
        }

        indicators
    }
}

impl Developmental for MatrixSpiritArchetype {
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

impl Paired for MatrixSpiritArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(16) // Paired with A16: Potentiator of Spirit
    }

    fn get_pair(&self) -> Option<u8> {
        Some(16)
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

impl Holonic for MatrixSpiritArchetype {
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
        // Transcend to next level if integration capacity is high
        if self.integration_capacity > 0.8 {
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
                HolonicLevel::Meta => false,
            }
        } else {
            false
        }
    }

    fn include(&mut self, lower_level: &dyn Holonic) -> bool {
        // Include lower level by increasing integration capacity
        let lower_capacity = lower_level.integration_capacity();
        self.integration_capacity = (self.integration_capacity + lower_capacity * 0.5).min(1.0);
        true
    }
}

// ============================================================================
// PHASE 10: MATRIX ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl super::archetype_traits::MatrixArchetypeTrait for MatrixSpiritArchetype {
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
        // Spirit Matrix health status based on lambda value
        if self.lambda.value >= 0.5 && self.lambda.value <= 0.8 {
            HealthStatus::Healthy
        } else if self.lambda.value < 0.3 || self.lambda.value > 0.9 {
            HealthStatus::Pathological
        } else {
            HealthStatus::Warning
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock archetype for testing paired relationships
    struct MockPotentiatorSpirit {
        lambda: LambdaMeasurement,
        tarot: TarotCorrelation,
    }

    impl MockPotentiatorSpirit {
        fn new(lambda_value: Float) -> Self {
            MockPotentiatorSpirit {
                lambda: LambdaMeasurement::new(
                    lambda_value,
                    LambdaMeasurementType::PotentiatorAccessibility,
                ),
                tarot: TarotCorrelation::new("The Tower".to_string()),
            }
        }
    }

    impl ArchetypeTrait for MockPotentiatorSpirit {
        fn archetype_id(&self) -> u8 {
            16
        }
        fn name(&self) -> &str {
            "Potentiator of Spirit"
        }
        fn complex(&self) -> ArchetypeComplex {
            ArchetypeComplex::Spirit
        }
        fn role(&self) -> ArchetypeRole {
            ArchetypeRole::Potentiator
        }
        fn lambda(&self) -> &LambdaMeasurement {
            &self.lambda
        }
        fn update_lambda(&mut self, value: Float) {
            self.lambda.value = value.clamp(0.0, 1.0);
        }
        fn tarot_correlation(&self) -> TarotCorrelation {
            self.tarot.clone()
        }
        fn sigma_axis(&self) -> SigmaAxis {
            SigmaAxis::SigmaC
        }
        fn functional_pair(&self) -> FunctionalPair {
            FunctionalPair::StructurePair
        }
        fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {}
        fn description(&self) -> &str {
            "Potentiator of Spirit - infinite potential"
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

    impl Paired for MockPotentiatorSpirit {
        fn get_pair(&self) -> Option<u8> {
            None
        }
        fn paired_archetype_id(&self) -> Option<u8> {
            Some(15)
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
    fn test_new_archetype() {
        let archetype = MatrixSpiritArchetype::new();

        assert_eq!(archetype.archetype_id, 15);
        assert_eq!(archetype.name(), "Matrix of Spirit");
        assert!(archetype.is_healthy());
        assert_eq!(archetype.complex(), ArchetypeComplex::Spirit);
        assert_eq!(archetype.role(), ArchetypeRole::Matrix);
        assert_eq!(archetype.sigma_axis(), SigmaAxis::SigmaC);
        assert_eq!(archetype.functional_pair(), FunctionalPair::StructurePair);
        assert!(archetype.tarot_correlation.card.contains("The Devil"));
        assert!(archetype.tarot_correlation.card.contains("XV"));
    }

    #[test]
    fn test_initial_values() {
        let archetype = MatrixSpiritArchetype::new();

        // Lambda should be in healthy range
        assert!(archetype.lambda.value >= 0.5);
        assert!(archetype.lambda.value <= 0.8);

        // Specific fields should have healthy initial values
        assert!((archetype.matrix_receptivity - 0.65).abs() < 0.01);
        assert!((archetype.darkness_depth - 0.65).abs() < 0.01);
        assert!((archetype.lightning_activation - 0.65).abs() < 0.01);
        assert!((archetype.moonlight_discernment - 0.65).abs() < 0.01);
        assert_eq!(archetype.infinite_potential, 1.0);
        assert!((archetype.spiritual_subtlety - 0.65).abs() < 0.01);
    }

    #[test]
    fn test_calculate_spirit_matrix_harmony() {
        let archetype = MatrixSpiritArchetype::new();

        let harmony = archetype.calculate_spirit_matrix_harmony();

        // Harmony should be within valid range
        assert!(harmony >= 0.0);
        assert!(harmony <= 1.0);

        // Harmony should be close to lambda value
        assert!((harmony - archetype.lambda.value).abs() < 0.1);
    }

    #[test]
    fn test_update_matrix_receptivity() {
        let mut archetype = MatrixSpiritArchetype::new();

        let initial_receptivity = archetype.matrix_receptivity;
        archetype.update_matrix_receptivity(0.9);

        // Receptivity should increase
        assert!(archetype.matrix_receptivity > initial_receptivity);
        assert!(archetype.matrix_receptivity <= 1.0);

        // Test with low value
        archetype.update_matrix_receptivity(0.1);
        assert!(archetype.matrix_receptivity < 1.0);
    }

    #[test]
    fn test_update_darkness_depth() {
        let mut archetype = MatrixSpiritArchetype::new();

        let initial_depth = archetype.darkness_depth;
        archetype.update_darkness_depth(0.9);

        // Darkness should deepen
        assert!(archetype.darkness_depth >= initial_depth);
        assert!(archetype.darkness_depth <= 1.0);
    }

    #[test]
    fn test_update_lightning_activation() {
        let mut archetype = MatrixSpiritArchetype::new();

        let initial_activation = archetype.lightning_activation;
        archetype.update_lightning_activation(0.9);

        // Activation should increase
        assert!(archetype.lightning_activation > initial_activation);
        assert!(archetype.lightning_activation <= 1.0);
    }

    #[test]
    fn test_update_moonlight_discernment() {
        let mut archetype = MatrixSpiritArchetype::new();

        let initial_discernment = archetype.moonlight_discernment;
        archetype.update_moonlight_discernment(0.9);

        // Discernment should improve
        assert!(archetype.moonlight_discernment > initial_discernment);
        assert!(archetype.moonlight_discernment <= 1.0);
    }

    #[test]
    fn test_update_spiritual_subtlety() {
        let mut archetype = MatrixSpiritArchetype::new();

        let initial_subtlety = archetype.spiritual_subtlety;
        archetype.update_spiritual_subtlety(0.9);

        // Subtlety should improve
        assert!(archetype.spiritual_subtlety > initial_subtlety);
        assert!(archetype.spiritual_subtlety <= 1.0);
    }

    #[test]
    fn test_lambda_measurement() {
        let archetype = MatrixSpiritArchetype::new();

        // Lambda should be in healthy range
        assert!(archetype.lambda.is_healthy());

        // Healthy range should be correct
        let (min, max) = archetype.healthy_range();
        assert!((min - 0.5).abs() < 0.01);
        assert!((max - 0.8).abs() < 0.01);

        // Measurement type should be MatrixRigidity
        assert_eq!(
            archetype.lambda.measurement_type,
            LambdaMeasurementType::MatrixRigidity
        );
    }

    #[test]
    fn test_pathological_low() {
        let mut archetype = MatrixSpiritArchetype::new();

        // Set lambda to pathological low
        archetype.update_lambda(0.3);

        assert!(!archetype.is_healthy());
        assert_eq!(archetype.health_status(), HealthStatus::PathologicalLow);

        let indicators = archetype.pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Blindness")));
        assert!(indicators.iter().any(|i| i.contains("Stuck in darkness")));
    }

    #[test]
    fn test_pathological_high() {
        let mut archetype = MatrixSpiritArchetype::new();

        // Set lambda to pathological high
        archetype.update_lambda(0.9);

        assert!(!archetype.is_healthy());
        assert_eq!(archetype.health_status(), HealthStatus::PathologicalHigh);

        let indicators = archetype.pathological_indicators();
        assert!(!indicators.is_empty());
        assert!(indicators.iter().any(|i| i.contains("Over-sensitivity")));
    }

    #[test]
    fn test_process_method() {
        let mut archetype = MatrixSpiritArchetype::new();

        let _initial_lambda = archetype.lambda.value;
        archetype.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Lambda should be updated based on harmony
        assert!(archetype.lambda.value > 0.0);
        assert!(archetype.lambda.value <= 1.0);

        // Integration capacity should be updated
        assert!(archetype.integration_capacity > 0.0);
    }

    #[test]
    fn test_developmental_tracking() {
        let archetype = MatrixSpiritArchetype::new();

        // Should have developmental position
        let position = archetype.developmental_position();
        // DevelopmentalPosition is an enum, compare directly
        assert_eq!(position, DevelopmentalPosition::Input);

        // Should have activated rungs
        let activated_rungs = archetype.activated_rungs();
        assert!(!activated_rungs.is_empty());

        // Should have activation levels
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
            let level = archetype.rung_activation_level(rung);
            assert!(level >= 0.0);
            assert!(level <= 1.0);
        }
    }

    #[test]
    fn test_paired_relationship() {
        let archetype = MatrixSpiritArchetype::new();
        let paired = MockPotentiatorSpirit::new(0.55);

        // Paired archetype ID should be correct
        assert_eq!(archetype.paired_archetype_id(), Some(16));

        // Should calculate pair tension
        let tension = archetype.calculate_pair_tension(&paired);
        assert!(tension >= 0.0);
        assert!(tension <= 1.0);

        // Should calculate pair balance
        let balance = archetype.calculate_pair_balance(&paired);
        assert!(balance >= 0.0);
        assert!(balance <= 1.0);
    }

    #[test]
    fn test_holonic_integration() {
        let mut archetype = MatrixSpiritArchetype::new();

        // Should start at Micro level
        assert_eq!(archetype.holonic_level(), HolonicLevel::Micro);

        // Should have integration capacity
        assert!(archetype.integration_capacity() > 0.0);

        // Transcend should work when integration capacity is high
        archetype.integration_capacity = 0.9;
        let transcended = archetype.transcend();
        assert!(transcended);
        assert_eq!(archetype.holonic_level(), HolonicLevel::Meso);
    }

    #[test]
    fn test_include_lower_level() {
        let mut archetype = MatrixSpiritArchetype::new();
        let lower = MatrixSpiritArchetype::new();

        let initial_capacity = archetype.integration_capacity;
        let included = archetype.include(&lower);

        assert!(included);
        assert!(archetype.integration_capacity > initial_capacity);
    }

    #[test]
    fn test_polarity() {
        let archetype = MatrixSpiritArchetype::new();

        // Should have STO polarity (female after veiling)
        assert_eq!(archetype.polarity, Polarity::STO);
    }

    #[test]
    fn test_diagnostic_info() {
        let archetype = MatrixSpiritArchetype::new();

        let info = archetype.get_diagnostic_info();

        assert!(info.contains("Matrix of Spirit Diagnostics"));
        assert!(info.contains("Harmony"));
        assert!(info.contains("Receptivity"));
        assert!(info.contains("Darkness Depth"));
        assert!(info.contains("Lightning Activation"));
        assert!(info.contains("Moonlight Discernment"));
        assert!(info.contains("Spiritual Subtlety"));
        assert!(info.contains("Health Status"));
    }

    #[test]
    fn test_infinite_potential() {
        let archetype = MatrixSpiritArchetype::new();

        // Infinite potential should always be 1.0
        assert_eq!(archetype.infinite_potential, 1.0);
    }

    #[test]
    fn test_boundary_conditions() {
        let mut archetype = MatrixSpiritArchetype::new();

        // Test upper bound
        archetype.update_matrix_receptivity(1.5);
        assert!(archetype.matrix_receptivity <= 1.0);

        // Test lower bound
        archetype.update_matrix_receptivity(-0.5);
        assert!(archetype.matrix_receptivity >= 0.0);

        // Test lambda bounds
        archetype.update_lambda(2.0);
        assert!(archetype.lambda.value <= 1.0);

        archetype.update_lambda(-1.0);
        assert!(archetype.lambda.value >= 0.0);
    }

    #[test]
    fn test_functional_pair_structure() {
        let archetype = MatrixSpiritArchetype::new();

        // Should be part of Structure Pair
        assert_eq!(archetype.functional_pair(), FunctionalPair::StructurePair);

        // Paired with Potentiator (A16)
        assert_eq!(archetype.paired_archetype_id(), Some(16));
    }

    #[test]
    fn test_tarot_correlation() {
        let archetype = MatrixSpiritArchetype::new();

        // Tarot correlation should be correct
        assert!(archetype.tarot_correlation.card.contains("The Devil"));
        assert!(archetype.tarot_correlation.card.contains("XV"));
        assert!(archetype.tarot_correlation.card.contains("Materialism"));
    }

    #[test]
    fn test_sigma_axis() {
        let archetype = MatrixSpiritArchetype::new();

        // Should operate through SigmaC (Spirit Capacity)
        assert_eq!(archetype.sigma_axis(), SigmaAxis::SigmaC);
    }

    // ============================================================================
    // PHASE 4: MATRIX CAPACITY MECHANICS TESTS
    // ============================================================================

    #[test]
    fn test_available_capacity_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.structural_capacity = 0.8;
        archetype.current_load = 0.3;

        assert_eq!(archetype.available_capacity(), 0.5);
    }

    #[test]
    fn test_is_overloaded_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.structural_capacity = 0.8;
        archetype.current_load = 0.75; // 93.75% of capacity

        assert!(archetype.is_overloaded());
    }

    #[test]
    fn test_calculate_accumulation_rate_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.structural_capacity = 0.8;
        archetype.processing_efficiency = 0.7;
        archetype.current_load = 0.7;

        // Processing capacity = 0.8 * 0.7 = 0.56
        // Accumulation = (0.7 - 0.56) / 0.8 = 0.14 / 0.8 = 0.175
        let accumulation = archetype.calculate_accumulation_rate();
        assert!((accumulation - 0.175).abs() < 0.01);
    }

    #[test]
    fn test_expand_capacity_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        let initial_capacity = archetype.structural_capacity;
        let initial_efficiency = archetype.processing_efficiency;

        archetype.expand_capacity(0.6);

        assert!(archetype.structural_capacity > initial_capacity);
        assert!(archetype.processing_efficiency > initial_efficiency);
    }

    #[test]
    fn test_increase_load_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.structural_capacity = 0.8;
        archetype.current_load = 0.4;

        archetype.increase_load(0.2);

        assert!((archetype.current_load - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_decrease_load_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.current_load = 0.7;

        archetype.decrease_load(0.3);

        assert!((archetype.current_load - 0.4).abs() < 0.001);
    }

    #[test]
    fn test_record_structural_state_spirit() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.structural_capacity = 0.8;
        archetype.current_load = 0.5;
        archetype.structural_complexity = 0.6;
        archetype.matrix_receptivity = 0.7;
        archetype.integration_capacity = 0.8;

        archetype.record_structural_state(75.0);

        assert_eq!(archetype.structural_history.len(), 1);
        let state = &archetype.structural_history[0];
        assert_eq!(state.timestamp, 75.0);
        assert_eq!(state.capacity, 0.8);
        assert_eq!(state.load, 0.5);
    }

    #[test]
    fn test_capacity_metrics_initialization_spirit() {
        let archetype = MatrixSpiritArchetype::new();

        assert!(archetype.structural_capacity >= 0.0 && archetype.structural_capacity <= 1.0);
        assert!(archetype.current_load >= 0.0 && archetype.current_load <= 1.0);
        assert!(archetype.processing_efficiency >= 0.0 && archetype.processing_efficiency <= 1.0);
        assert!(archetype.accumulation_rate >= 0.0 && archetype.accumulation_rate <= 1.0);
        assert!(archetype.structural_complexity >= 0.0 && archetype.structural_complexity <= 1.0);
        assert!(
            archetype.structural_adaptability >= 0.0 && archetype.structural_adaptability <= 1.0
        );
    }

    #[test]
    fn test_accumulation_rate_zero_when_processing_exceeds_load() {
        let mut archetype = MatrixSpiritArchetype::new();
        archetype.structural_capacity = 0.8;
        archetype.processing_efficiency = 0.9;
        archetype.current_load = 0.5;

        // Processing capacity = 0.8 * 0.9 = 0.72
        // Accumulation = (0.5 - 0.72) / 0.8 = negative, clamped to 0.0
        assert_eq!(archetype.calculate_accumulation_rate(), 0.0);
    }
}
