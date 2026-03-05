// A7: The Great Way of Mind
// Environment container - field of possibility
// Documentation: 01_Metaphysics/archetypes/A7_GreatWay.md

use crate::archetypes::archetype_traits::GreatWayArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, Holonic, HolonicLevel, LambdaMeasurable, LambdaMeasurement,
    LambdaMeasurementType, Paired, SigmaAxis, TarotCorrelation,
};
use crate::types::{Float, Rung};
use std::collections::HashMap;

/// Milieu - Environment definition for Lesser Cycle processing
///
/// Great Way defines the MILIEU (time/space/entropy) for Lesser Cycle
#[derive(Debug, Clone)]
pub struct Milieu {
    /// Time dilation/compression factor (0.0 = compressed, 1.0 = normal, 2.0 = dilated)
    pub time_factor: Float,
    /// Space expansion/contraction factor (0.0 = contracted, 1.0 = normal, 2.0 = expanded)
    pub space_factor: Float,
    /// Entropy increase/decrease factor (0.0 = decrease, 1.0 = normal, 2.0 = increase)
    pub entropy_factor: Float,
    /// Creativity enhancement factor (0.0 = blocked, 1.0 = normal, 2.0 = enhanced)
    pub creativity_factor: Float,
    /// Stability maintenance factor (0.0 = unstable, 1.0 = normal, 2.0 = stable)
    pub stability_factor: Float,
}

impl Milieu {
    /// Create a new default Milieu
    pub fn new() -> Self {
        Milieu {
            time_factor: 1.0,
            space_factor: 1.0,
            entropy_factor: 1.0,
            creativity_factor: 1.0,
            stability_factor: 1.0,
        }
    }

    /// Create a Milieu with specific factors
    pub fn with_factors(
        time_factor: Float,
        space_factor: Float,
        entropy_factor: Float,
        creativity_factor: Float,
        stability_factor: Float,
    ) -> Self {
        Milieu {
            time_factor: time_factor.clamp(0.0, 2.0),
            space_factor: space_factor.clamp(0.0, 2.0),
            entropy_factor: entropy_factor.clamp(0.0, 2.0),
            creativity_factor: creativity_factor.clamp(0.0, 2.0),
            stability_factor: stability_factor.clamp(0.0, 2.0),
        }
    }

    /// Calculate overall milieu quality
    pub fn overall_quality(&self) -> Float {
        let factors = [self.time_factor,
            self.space_factor,
            self.entropy_factor,
            self.creativity_factor,
            self.stability_factor];
        factors.iter().sum::<Float>() / factors.len() as Float
    }

    /// Check if milieu is balanced
    pub fn is_balanced(&self) -> bool {
        let quality = self.overall_quality();
        (0.8..=1.2).contains(&quality)
    }
}

/// A7: The Great Way of Mind
///
/// The Great Way of the Mind denotes and configures the particular framework within
/// which the Mind archetypes move — drawing the environment which has been the new
/// architecture caused by the veiling process, dipped in the great, limitless current
/// of time/space. It represents the culmination and framework for Mind complex activity.
///
/// **Core Concept**: The Great Way of the Mind denotes and configures the particular
/// framework within which the Mind, Body, or Spirit archetypes move — drawing the
/// environment which has been the new architecture caused by the veiling process,
/// dipped in the great, limitless current of time/space.
///
/// **Lambda Measurement**: Great Way clarity and alignment
/// - Healthy range: 0.5 - 0.8 (clear perception with good alignment)
/// - Pathological: < 0.5 (misperception) or > 0.8 (over-identification)
#[derive(Debug, Clone)]
pub struct GreatWayMindArchetype {
    /// Archetype ID (A7)
    pub archetype_id: u8,

    /// Lambda measurement - Great Way clarity and alignment
    pub lambda: LambdaMeasurement,

    /// Tarot correlation
    pub tarot_correlation: TarotCorrelation,

    /// Framework clarity - how clearly is the framework perceived (0.0 - 1.0)
    pub framework_clarity: Float,

    /// Environment perception - how accurately is environment perceived (0.0 - 1.0)
    pub environment_perception: Float,

    /// Significator alignment - how well Significator navigates within field (0.0 - 1.0)
    pub significator_alignment: Float,

    /// Time/space connection - effectiveness of connection to time/space (0.0 - 1.0)
    pub time_space_connection: Float,

    /// Veil lifting progress - progress in lifting the great veil (0.0 - 1.0)
    pub veil_lifting_progress: Float,

    /// Framework configuration strength - how well framework is configured (0.0 - 1.0)
    pub framework_configuration: Float,

    /// Foundation integration - integration of Archetypes 1-6 (0.0 - 1.0)
    pub foundation_integration: Float,

    /// Developmental position in Sigma Network
    pub developmental_position: DevelopmentalPosition,

    /// Activated rungs (R1-R7)
    pub activated_rungs: Vec<Rung>,

    /// Activation levels for each rung (0.0 to 1.0)
    pub activation_levels: HashMap<Rung, Float>,

    /// Description
    pub description: String,

    /// Holonic level
    pub holonic_level: HolonicLevel,

    /// Integration capacity (0.0 to 1.0)
    pub integration_capacity: Float,

    /// Alignment score - Significator choice × Great Way perception (0.0 - 1.0)
    pub alignment_score: Float,

    // Phase 7: Current milieu for Lesser Cycle
    pub current_milieu: Milieu,
}

impl GreatWayMindArchetype {
    /// Create a new Great Way archetype with default healthy state
    pub fn new() -> Self {
        // Initialize with optimal lambda value (middle of healthy range)
        let mut lambda = LambdaMeasurement::new(0.5, LambdaMeasurementType::GreatWayClarity);
        lambda.healthy_min = 0.5;
        lambda.healthy_max = 0.8;

        let tarot_correlation = TarotCorrelation::new("The Chariot (VII): The path of the adept, the journey of transformation".to_string());

        let activation_levels = HashMap::new();

        GreatWayMindArchetype {
            archetype_id: 7,
            lambda,
            tarot_correlation,
            framework_clarity: 0.65,
            environment_perception: 0.70,
            significator_alignment: 0.65,
            time_space_connection: 0.60,
            veil_lifting_progress: 0.50,
            framework_configuration: 0.65,
            foundation_integration: 0.60,
            developmental_position: DevelopmentalPosition::Significator,
            activated_rungs: vec![Rung::R1, Rung::R2, Rung::R3, Rung::R4],
            activation_levels,
            description: "The Great Way of Mind denotes and configures the particular framework within which the Mind archetypes move — drawing the environment which has been the new architecture caused by the veiling process, dipped in the great, limitless current of time/space. It represents the culmination and framework for Mind complex activity.".to_string(),
            holonic_level: HolonicLevel::Macro,
            integration_capacity: 0.7,
            alignment_score: 0.65,

            // Phase 7: Initialize default milieu
            current_milieu: Milieu::new(),
        }
    }

    /// Calculate framework clarity
    ///
    /// Framework clarity increases with:
    /// - Higher environment perception
    /// - Higher veil lifting progress
    /// - Higher foundation integration
    ///
    /// Returns: Framework clarity (0.0 to 1.0)
    pub fn calculate_framework_clarity(&self) -> Float {
        (self.environment_perception * 0.3
            + self.veil_lifting_progress * 0.3
            + self.foundation_integration * 0.4)
            .clamp(0.0, 1.0)
    }

    /// Calculate environment perception
    ///
    /// Environment perception increases with:
    /// - Higher time/space connection
    /// - Higher veil lifting progress
    /// - Higher framework configuration
    ///
    /// Returns: Environment perception (0.0 to 1.0)
    pub fn calculate_environment_perception(&self) -> Float {
        (self.time_space_connection * 0.4
            + self.veil_lifting_progress * 0.3
            + self.framework_configuration * 0.3)
            .clamp(0.0, 1.0)
    }

    /// Calculate time/space connection
    ///
    /// Time/space connection increases with:
    /// - Higher veil lifting progress
    /// - Higher framework configuration
    ///
    /// Returns: Time/space connection (0.0 to 1.0)
    pub fn calculate_time_space_connection(&self) -> Float {
        (self.veil_lifting_progress * 0.6 + self.framework_configuration * 0.4)
            .clamp(0.0, 1.0)
    }

    /// Calculate veil lifting progress
    ///
    /// Veil lifting progress increases with:
    /// - Higher developmental position (rung level)
    /// - Higher integration capacity
    ///
    /// Returns: Veil lifting progress (0.0 to 1.0)
    pub fn calculate_veil_lifting_progress(&self) -> Float {
        let rung_level = self.developmental_position.rung_level() as Float / 7.0;
        (rung_level * 0.5 + self.integration_capacity * 0.5)
            .clamp(0.0, 1.0)
    }

    /// Calculate foundation integration
    ///
    /// Foundation integration represents the reflection and substantial summary
    /// of Archetypes One through Six
    ///
    /// Returns: Foundation integration (0.0 to 1.0)
    pub fn calculate_foundation_integration(&self) -> Float {
        // Foundation is reflection and substantial summary of A1-A6
        // This increases with integration capacity and developmental position
        let rung_level = self.developmental_position.rung_level() as Float / 7.0;
        (rung_level * 0.4 + self.integration_capacity * 0.6)
            .clamp(0.0, 1.0)
    }

    /// Calculate alignment score
    ///
    /// Alignment Score = (Significator Choice × Great Way Perception) / ||Great Way||
    ///
    /// Where:
    /// - Significator Choice: Choice between STO and STS paths
    /// - Great Way Perception: Perception of the field of possibility
    ///
    /// Returns: Alignment score (0.0 to 1.0)
    pub fn calculate_alignment_score(&self, significator_choice: Float) -> Float {
        let perception = self.lambda.value;
        
        (significator_choice * perception).clamp(0.0, 1.0)
    }

    /// Calculate framework configuration
    ///
    /// Framework = Configuration × Environment × Constraints
    ///
    /// Returns: Framework configuration (0.0 to 1.0)
    pub fn calculate_framework_configuration(&self) -> Float {
        (self.framework_clarity * 0.4
            + self.environment_perception * 0.3
            + self.foundation_integration * 0.3)
            .clamp(0.0, 1.0)
    }

    /// Process Great Way
    ///
    /// Updates all Great Way properties based on current state
    pub fn process_great_way(&mut self) {
        self.framework_clarity = self.calculate_framework_clarity();
        self.environment_perception = self.calculate_environment_perception();
        self.time_space_connection = self.calculate_time_space_connection();
        self.veil_lifting_progress = self.calculate_veil_lifting_progress();
        self.framework_configuration = self.calculate_framework_configuration();
        self.foundation_integration = self.calculate_foundation_integration();

        // Update lambda based on framework clarity and alignment
        self.lambda.value = (self.framework_clarity * 0.6 + self.significator_alignment * 0.4)
            .clamp(0.0, 1.0);
    }

    /// Get framework status
    ///
    /// Returns description of current framework state
    pub fn get_framework_status(&self) -> String {
        match self.lambda.health_status() {
            HealthStatus::Healthy => {
                if self.framework_clarity > 0.7 {
                    "Clear Framework: Excellent perception and navigation".to_string()
                } else if self.framework_clarity > 0.5 {
                    "Healthy Framework: Good perception, effective navigation".to_string()
                } else {
                    "Acceptable Framework: Adequate perception, workable navigation".to_string()
                }
            }
            HealthStatus::Balanced => "Balanced state".to_string(),
            HealthStatus::Imbalanced => "Imbalanced state".to_string(),
            HealthStatus::Warning => {
                "Warning: Framework clarity declining, perception issues emerging".to_string()
            }
            HealthStatus::Degraded => {
                "Degraded: Multiple framework issues, navigation significantly impaired".to_string()
            }
            HealthStatus::Pathological => {
                "Pathological: Severe framework dysfunction, perception and navigation broken"
                    .to_string()
            }
            HealthStatus::PathologicalLow => {
                if self.veil_lifting_progress < 0.3 {
                    "Pathological: Misperception, confusion, fighting against conditions"
                        .to_string()
                } else {
                    "Pathological: Poor framework clarity, ineffective navigation".to_string()
                }
            }
            HealthStatus::PathologicalHigh => {
                if self.significator_alignment > 0.8 {
                    "Pathological: Over-identification with environment, loss of agency".to_string()
                } else {
                    "Pathological: Excessive framework rigidity, reduced flexibility".to_string()
                }
            }
        }
    }

    /// Check if fighting against conditions
    ///
    /// Returns true if entity is resisting rather than navigating within Great Way
    pub fn is_fighting_conditions(&self) -> bool {
        // Fighting conditions occurs when perception is low but effort is high
        self.environment_perception < 0.4 && self.significator_alignment > 0.6
    }

    // ============================================================================
    // PHASE 7: MILIEU DEFINITION AND APPLICATION (Greater Cycle Regulation)
    // ============================================================================

    /// Create milieu based on Significator choice and Great Way perception
    ///
    /// Great Way defines the MILIEU (time/space/entropy) for Lesser Cycle
    pub fn create_milieu(&self, _significator_choice: Float) -> Milieu {
        // Milieu factors based on Great Way clarity and Significator alignment
        let time_factor = 1.0 + (self.time_space_connection - 0.5) * 0.4;
        let space_factor = 1.0 + (self.environment_perception - 0.5) * 0.4;
        let entropy_factor = 1.0 + (self.veil_lifting_progress - 0.5) * 0.3;
        let creativity_factor = 1.0 + (self.framework_clarity - 0.5) * 0.5;
        let stability_factor = 1.0 + (self.significator_alignment - 0.5) * 0.3;

        Milieu::with_factors(
            time_factor,
            space_factor,
            entropy_factor,
            creativity_factor,
            stability_factor,
        )
    }

    /// Update current milieu
    pub fn update_milieu(&mut self, significator_choice: Float) {
        self.current_milieu = self.create_milieu(significator_choice);
    }

    /// Get current milieu
    pub fn get_milieu(&self) -> &Milieu {
        &self.current_milieu
    }

    /// Apply milieu to processing parameters
    ///
    /// Greater Way defines MILIEU for Lesser Cycle processing
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

// Additional methods for archetype processing

impl GreatWayMindArchetype {
    /// Set veil permeability (actually sets veil_lifting_progress)
    pub fn set_veil_permeability(&mut self, permeability: Float) {
        self.veil_lifting_progress = permeability.clamp(0.0, 1.0);
    }
}

// Implement LambdaMeasurable
impl LambdaMeasurable for GreatWayMindArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        // Lambda = Clarity × Alignment
        self.framework_clarity * self.significator_alignment
    }

    fn healthy_range(&self) -> (Float, Float) {
        (self.lambda.healthy_min, self.lambda.healthy_max)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.is_pathological_low() {
            indicators.push("Misperception of framework or environment".to_string());
            indicators.push("Fighting against conditions rather than navigating".to_string());
            indicators.push("Lack of progressive veil lifting".to_string());
        }

        if self.lambda.is_pathological_high() {
            indicators.push("Over-identification with environment".to_string());
            indicators.push("Loss of agency within framework".to_string());
            indicators.push("Excessive framework rigidity".to_string());
        }

        indicators
    }
}

// Implement Developmental
impl Developmental for GreatWayMindArchetype {
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
        self.activation_levels.get(&rung).copied().unwrap_or(0.0)
    }
}

// Implement Paired
impl Paired for GreatWayMindArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        Some(5) // A5: The Significator of Mind
    }

    fn get_pair(&self) -> Option<u8> {
        Some(5)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        // Identity Pair tension: |Significator coherence - Great Way clarity|
        // This is a placeholder - actual calculation requires Significator reference
        
        (self.significator_alignment - self.framework_clarity).abs()
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        // Balance = 1.0 - |tension - 0.5|
        let tension = self.calculate_pair_tension(paired_archetype);
        1.0 - (tension - 0.5).abs()
    }
}

// Implement Holonic
impl Holonic for GreatWayMindArchetype {
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
        // Attempt to transcend to next holonic level
        match self.holonic_level {
            HolonicLevel::Micro => {
                if self.integration_capacity > 0.7 {
                    self.holonic_level = HolonicLevel::Meso;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meso => {
                if self.integration_capacity > 0.8 {
                    self.holonic_level = HolonicLevel::Macro;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Macro => {
                if self.integration_capacity > 0.9 {
                    self.holonic_level = HolonicLevel::Meta;
                    true
                } else {
                    false
                }
            }
            HolonicLevel::Meta => false, // Already at highest level
        }
    }

    fn include(&mut self, _lower_level: &dyn Holonic) -> bool {
        // Include lower level holonic properties
        self.integration_capacity = (self.integration_capacity + 0.1).min(1.0);
        true
    }
}

// ============================================================================
// GREAT WAY ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl GreatWayArchetypeTrait for GreatWayMindArchetype {
    // Core getters
    fn get_archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement {
        self.lambda.clone()
    }

    fn get_framework_clarity(&self) -> Float {
        self.framework_clarity
    }

    fn get_framework_alignment(&self) -> Float {
        self.significator_alignment
    }

    // Core setters
    fn set_framework_clarity(&mut self, value: Float) {
        self.framework_clarity = value.clamp(0.0, 1.0);
    }

    fn set_framework_alignment(&mut self, value: Float) {
        self.significator_alignment = value.clamp(0.0, 1.0);
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

    // Phase 7: Milieu definition
    fn get_milieu(&self) -> Milieu {
        self.current_milieu.clone()
    }

    fn set_milieu(&mut self, milieu: Milieu) {
        self.current_milieu = milieu;
    }

    fn update_coupling_coefficient(&self, current_coefficient: Float) -> Float {
        // Modify coupling based on framework clarity and alignment
        let clarity_factor = self.framework_clarity;
        let alignment_factor = self.significator_alignment;
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
        let clarity_health = 1.0 - (self.framework_clarity - 0.65).abs() * 2.0;
        let alignment_health = self.significator_alignment;
        let lambda_health = if self.lambda.is_healthy() { 1.0 } else { 0.0 };

        let overall_health = (clarity_health + alignment_health + lambda_health) / 3.0;

        match overall_health {
            h if h >= 0.8 => HealthStatus::Healthy,
            h if h >= 0.6 => HealthStatus::Warning,
            h if h >= 0.4 => HealthStatus::Degraded,
            _ => HealthStatus::Pathological,
        }
    }
}

impl ArchetypeTrait for GreatWayMindArchetype {
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
        ArchetypeRole::GreatWay
    }

    fn process(&mut self, _catalyst: Float, _position: DevelopmentalPosition) {
        self.process_great_way();
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaC
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
        self.framework_clarity = self.lambda.value;
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Great Way of Mind"
    }

    fn description(&self) -> &str {
        "Great Way of Mind - The framework that defines the Lesser Cycle processing"
    }

    fn complex(&self) -> ArchetypeComplex {
        ArchetypeComplex::Mind
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::IdentityPair
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_great_way_creation() {
        let great_way = GreatWayMindArchetype::new();
        assert_eq!(great_way.archetype_id, 7);
        assert_eq!(great_way.name(), "The Great Way of Mind");
        assert_eq!(great_way.complex(), ArchetypeComplex::Mind);
        assert_eq!(great_way.role(), ArchetypeRole::GreatWay);
    }

    #[test]
    fn test_great_way_lambda_initialization() {
        let great_way = GreatWayMindArchetype::new();
        assert_eq!(great_way.lambda.value, 0.5);
        assert_eq!(great_way.lambda.healthy_min, 0.5);
        assert_eq!(great_way.lambda.healthy_max, 0.8);
        assert!(great_way.is_healthy());
    }

    #[test]
    fn test_great_way_health_status() {
        let mut great_way = GreatWayMindArchetype::new();

        // Test healthy state
        great_way.update_lambda(0.65);
        assert_eq!(great_way.health_status(), HealthStatus::Healthy);
        assert!(great_way.is_healthy());

        // Test pathological low
        great_way.update_lambda(0.4);
        assert_eq!(great_way.health_status(), HealthStatus::PathologicalLow);
        assert!(!great_way.is_healthy());

        // Test pathological high
        great_way.update_lambda(0.85);
        assert_eq!(great_way.health_status(), HealthStatus::PathologicalHigh);
        assert!(!great_way.is_healthy());
    }

    #[test]
    fn test_great_way_processing() {
        let mut great_way = GreatWayMindArchetype::new();
        great_way.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        assert!(great_way.framework_clarity > 0.0);
        assert!(great_way.framework_clarity <= 1.0);
        assert!(great_way.environment_perception > 0.0);
        assert!(great_way.environment_perception <= 1.0);
        assert!(great_way.time_space_connection > 0.0);
        assert!(great_way.time_space_connection <= 1.0);
    }

    #[test]
    fn test_alignment_score() {
        let mut great_way = GreatWayMindArchetype::new();
        great_way.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Test with different significator choices
        let score_low = great_way.calculate_alignment_score(0.3);
        let score_mid = great_way.calculate_alignment_score(0.5);
        let score_high = great_way.calculate_alignment_score(0.8);

        assert!((0.0..=1.0).contains(&score_low));
        assert!((0.0..=1.0).contains(&score_mid));
        assert!((0.0..=1.0).contains(&score_high));
        assert!(score_high > score_mid);
        assert!(score_mid > score_low);
    }

    #[test]
    fn test_foundation_integration() {
        let mut great_way = GreatWayMindArchetype::new();
        great_way.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        // Foundation is reflection of A1-A6
        assert!(great_way.foundation_integration > 0.0);
        assert!(great_way.foundation_integration <= 1.0);
    }

    #[test]
    fn test_veil_lifting_progress() {
        let mut great_way = GreatWayMindArchetype::new();
        great_way.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);

        assert!(great_way.veil_lifting_progress > 0.0);
        assert!(great_way.veil_lifting_progress <= 1.0);
    }

    #[test]
    fn test_fighting_conditions() {
        let mut great_way = GreatWayMindArchetype::new();

        // Normal state - not fighting
        assert!(!great_way.is_fighting_conditions());

        // Fighting conditions state
        great_way.environment_perception = 0.3;
        great_way.significator_alignment = 0.7;
        assert!(great_way.is_fighting_conditions());
    }

    #[test]
    fn test_framework_status() {
        let mut great_way = GreatWayMindArchetype::new();

        // Healthy status
        great_way.process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);
        let status = great_way.get_framework_status();
        assert!(status.contains("Framework"));

        // Pathological low status
        great_way.update_lambda(0.4);
        let status = great_way.get_framework_status();
        assert!(status.contains("Pathological"));

        // Pathological high status
        great_way.update_lambda(0.85);
        let status = great_way.get_framework_status();
        assert!(status.contains("Pathological"));
    }

    #[test]
    fn test_pathological_indicators() {
        let mut great_way = GreatWayMindArchetype::new();

        // Test pathological low
        great_way.update_lambda(0.4);
        let indicators_low = great_way.pathological_indicators();
        assert!(!indicators_low.is_empty());
        assert!(indicators_low
            .iter()
            .any(|i| i.to_lowercase().contains("misperception")
                || i.to_lowercase().contains("fighting")));

        // Test pathological high
        great_way.update_lambda(0.85);
        let indicators_high = great_way.pathological_indicators();
        assert!(!indicators_high.is_empty());
        assert!(indicators_high
            .iter()
            .any(|i| i.to_lowercase().contains("over-identification")
                || i.to_lowercase().contains("loss of agency")));
    }

    #[test]
    fn test_functional_pair() {
        let great_way = GreatWayMindArchetype::new();
        assert_eq!(great_way.functional_pair(), FunctionalPair::IdentityPair);
        assert_eq!(great_way.paired_archetype_id(), Some(5));
    }

    #[test]
    fn test_holonic_transcendence() {
        let mut great_way = GreatWayMindArchetype::new();

        // Start at Macro level
        assert_eq!(great_way.holonic_level(), HolonicLevel::Macro);

        // Try to transcend without sufficient integration capacity
        let result = great_way.transcend();
        assert!(!result);
        assert_eq!(great_way.holonic_level(), HolonicLevel::Macro);

        // Increase integration capacity and transcend (need > 0.9 for Macro -> Meta)
        great_way.integration_capacity = 0.95;
        let result = great_way.transcend();
        assert!(result);
        assert_eq!(great_way.holonic_level(), HolonicLevel::Meta);
    }

    #[test]
    fn test_developmental_position() {
        let great_way = GreatWayMindArchetype::new();

        assert_eq!(
            great_way.developmental_position(),
            DevelopmentalPosition::Significator
        );

        let activated_rungs = great_way.activated_rungs();
        assert_eq!(activated_rungs.len(), 4);
        assert!(activated_rungs.contains(&Rung::R1));
        assert!(activated_rungs.contains(&Rung::R2));
        assert!(activated_rungs.contains(&Rung::R3));
        assert!(activated_rungs.contains(&Rung::R4));
    }
}
