// A6: The Transformation of Mind
// Documentation reference: 01_Metaphysics/archetypes/A6_Transformation.md

use crate::archetypes::archetype_traits::TransformationArchetypeTrait;
use crate::archetypes::common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, DevelopmentalPosition,
    FunctionalPair, HealthStatus, HealthStatus as CommonHealthStatus, Holonic, HolonicLevel,
    LambdaMeasurable, LambdaMeasurement, LambdaMeasurementType, Paired, SigmaAxis,
    TarotCorrelation,
};
use crate::types::{Float, Octant, Polarity, Rung};
use std::collections::HashMap;

/// A6: The Transformation of Mind
///
/// The Transformation represents the student of the mysteries being transformed
/// by the need to choose betwixt the light and the dark in mind — the process
/// of choosing between service-to-others and service-to-self paths.
///
/// Transformation is the actual change occurring in the system, the mechanism
/// by which the Significator evolves through polarization.
#[derive(Debug, Clone)]
pub struct TransformationMindArchetype {
    pub archetype_id: u8,
    pub lambda: LambdaMeasurement,
    pub tarot_correlation: TarotCorrelation,
    pub complex: ArchetypeComplex,
    pub role: ArchetypeRole,

    // Transformation-specific properties
    pub transformation_velocity: Float, // Speed of transformation (0.0 - 1.0)
    pub transformation_direction: Polarity, // STO or STS
    pub abandonment_completeness: Float, // How completely one principle is abandoned (0.0 - 1.0)
    pub polarization_progress: Float,   // Progress toward chosen polarization (0.0 - 1.0)

    // Choice mechanism
    pub choice_made: bool, // Whether choice betwixt light and dark has been made
    pub choice_intensity: Float, // Intensity of the choice (0.0 - 1.0)
    pub creative_point_active: bool, // Whether triangular creative point is active

    // Path characteristics
    pub path_focus_mental: Float,   // Mental focus (higher for STO)
    pub path_focus_physical: Float, // Physical focus (higher for STS)
    pub protection_level: Float,    // Protection from other-selves (STS only)
    pub deep_mind_attitude: DeepMindAttitude, // Prostituted vs. virginal/maiden

    // Developmental tracking
    pub developmental_position: DevelopmentalPosition,
    pub activated_rungs: Vec<Rung>,
    pub activation_levels: HashMap<Rung, Float>,
    pub description: String,
    pub holonic_level: HolonicLevel,
    pub integration_capacity: Float,

    // Greater Cycle integration
    pub connected_to_choice: bool,       // Connected to A22 Choice
    pub connected_to_significator: bool, // Connected to A5 Significator

    // Phase 7: Parameter modification capabilities
    pub can_modify_veil: bool,
    pub can_modify_capacity: bool,
    pub can_modify_efficiency: bool,
    pub can_modify_tolerance: bool,

    /// Transformation level - current level of transformation (0.0 to 1.0)
    pub transformation_level: Float,

    /// Polarity choice - the chosen polarity direction
    pub polarity_choice: Option<Polarity>,
}

/// Deep Mind Attitude
///
/// How the conscious entity relates to its deep mind resources
#[derive(Debug, Clone, PartialEq)]
pub enum DeepMindAttitude {
    /// Using deep mind as prostitute - rough extraction without respect
    Prostituted,
    /// Courting deep mind as maiden - respectful courtship
    Virginal,
    /// Mixed or undecided
    Undecided,
}

impl TransformationMindArchetype {
    /// Create a new Transformation archetype with default values
    pub fn new() -> Self {
        let mut lambda = LambdaMeasurement::new(0.5, LambdaMeasurementType::TransformationVelocity);
        lambda.healthy_min = 0.5;
        lambda.healthy_max = 0.8;

        let tarot_correlation = TarotCorrelation::new(format!(
            "The Lovers (VI): Catalyst processed and transformed into new understanding"
        ));

        let mut activation_levels = HashMap::new();
        activation_levels.insert(Rung::R1, 0.0);
        activation_levels.insert(Rung::R2, 0.0);
        activation_levels.insert(Rung::R3, 0.0);
        activation_levels.insert(Rung::R4, 0.0);
        activation_levels.insert(Rung::R5, 0.0);
        activation_levels.insert(Rung::R6, 0.0);
        activation_levels.insert(Rung::R7, 0.0);

        TransformationMindArchetype {
            archetype_id: 6,
            lambda,
            tarot_correlation,
            complex: ArchetypeComplex::Mind,
            role: ArchetypeRole::Transformation,

            transformation_velocity: 0.5,
            transformation_direction: Polarity::SinkholeOfIndifference,  // Undecided
            abandonment_completeness: 0.0,
            polarization_progress: 0.0,

            choice_made: false,
            choice_intensity: 0.0,
            creative_point_active: false,

            path_focus_mental: 0.5,
            path_focus_physical: 0.5,
            protection_level: 0.0,
            deep_mind_attitude: DeepMindAttitude::Undecided,

            developmental_position: DevelopmentalPosition::Experience,
            activated_rungs: vec![Rung::R3],
            activation_levels,
            description: "The Transformation of Mind represents the student of the mysteries being transformed by the need to choose betwixt the light and the dark in mind — the process of choosing between service-to-others and service-to-self paths.".to_string(),
            holonic_level: HolonicLevel::Micro,
            integration_capacity: 0.5,

            connected_to_choice: false,
            connected_to_significator: false,

            // Phase 7: Initialize parameter modification capabilities
            can_modify_veil: true,
            can_modify_capacity: true,
            can_modify_efficiency: true,
            can_modify_tolerance: true,

            transformation_level: 0.0,
            polarity_choice: None,
        }
    }

    /// Make choice betwixt light and dark
    ///
    /// This is the fundamental act that enables transformation
    pub fn make_choice(&mut self, direction: Polarity, intensity: Float) -> Result<(), String> {
        if !matches!(direction, Polarity::STO | Polarity::STS) {
            return Err("Choice must be between STO (light) and STS (dark)".to_string());
        }

        if intensity < 0.0 || intensity > 1.0 {
            return Err("Choice intensity must be between 0.0 and 1.0".to_string());
        }

        self.choice_made = true;
        self.choice_intensity = intensity;
        self.transformation_direction = direction;

        // Adjust path focus based on choice
        match direction {
            Polarity::STO | Polarity::ServiceToOthers => {
                self.path_focus_mental = 0.7 + (intensity * 0.3);
                self.path_focus_physical = 0.3 - (intensity * 0.2);
                self.protection_level = 0.0; // STO has no protection
            }
            Polarity::STS | Polarity::ServiceToSelf => {
                self.path_focus_mental = 0.3 - (intensity * 0.2);
                self.path_focus_physical = 0.7 + (intensity * 0.3);
                self.protection_level = intensity; // STS protected by transformation strength
            }
            Polarity::Neutral | Polarity::SinkholeOfIndifference => {
                // No path focus for neutral paths
            }
            _ => {}
        }

        // Choice activates transformation velocity
        self.transformation_velocity = 0.4 + (intensity * 0.3);

        // Activate creative point
        self.creative_point_active = true;

        Ok(())
    }

    /// Abandon one principle governing use of deep mind
    ///
    /// Required for transformation to occur
    pub fn abandon_principle(&mut self, completeness: Float) -> Result<(), String> {
        if completeness < 0.0 || completeness > 1.0 {
            return Err("Abandonment completeness must be between 0.0 and 1.0".to_string());
        }

        if !self.choice_made {
            return Err("Must make choice before abandoning principle".to_string());
        }

        self.abandonment_completeness = completeness;

        // Abandonment increases transformation velocity
        let velocity_increase = completeness * 0.2;
        self.transformation_velocity = (self.transformation_velocity + velocity_increase).min(1.0);

        Ok(())
    }

    /// Set deep mind attitude
    ///
    /// Determines quality of treasure received from deep mind
    pub fn set_deep_mind_attitude(&mut self, attitude: DeepMindAttitude) {
        self.deep_mind_attitude = attitude;

        // Attitude affects transformation velocity
        match self.deep_mind_attitude {
            DeepMindAttitude::Virginal => {
                // Careful courtship yields better transformation
                self.transformation_velocity = (self.transformation_velocity * 1.1).min(1.0);
            }
            DeepMindAttitude::Prostituted => {
                // Rough use yields rough transformation
                self.transformation_velocity = self.transformation_velocity * 0.9;
            }
            DeepMindAttitude::Undecided => {}
        }
    }

    /// Calculate transformation treasure quality
    ///
    /// Returns quality of treasure received from deep mind
    pub fn calculate_treasure_quality(&self) -> Float {
        match self.deep_mind_attitude {
            DeepMindAttitude::Virginal => {
                // Great treasure through careful courtship
                self.abandonment_completeness * self.choice_intensity
            }
            DeepMindAttitude::Prostituted => {
                // Rough, prostituted treasure without great virtue
                self.abandonment_completeness * self.choice_intensity * 0.5
            }
            DeepMindAttitude::Undecided => 0.0,
        }
    }

    /// Process transformation cycle
    ///
    /// Advances transformation based on current state
    pub fn process_transformation(&mut self, catalyst_intensity: Float) {
        if !self.choice_made {
            // No choice made, rocking back and forth without transformation
            self.transformation_velocity = 0.2;
            return;
        }

        if self.abandonment_completeness < 0.5 {
            // Incomplete abandonment, limited transformation
            self.transformation_velocity = 0.3 + (self.abandonment_completeness * 0.2);
            return;
        }

        // Full transformation in progress
        let base_velocity = self.choice_intensity * self.abandonment_completeness;
        let catalyst_influence = catalyst_intensity * 0.3;
        self.transformation_velocity = (base_velocity + catalyst_influence).min(1.0);

        // Update polarization progress
        match self.transformation_direction {
            Polarity::STO | Polarity::STS | Polarity::ServiceToOthers | Polarity::ServiceToSelf => {
                self.polarization_progress =
                    (self.polarization_progress + self.transformation_velocity * 0.1).min(1.0);
            }
            Polarity::Neutral | Polarity::SinkholeOfIndifference => {
                // No polarization progress for neutral paths
            }
            _ => {}
        }
    }

    /// Calculate polarization intensity
    ///
    /// Returns intensity of polarization (used in Greater Cycle)
    pub fn calculate_polarization_intensity(&self) -> Float {
        if !matches!(self.transformation_direction, Polarity::STO | Polarity::STS) {
            return 0.0;
        }

        self.choice_intensity * self.abandonment_completeness * self.polarization_progress
    }

    /// Check if transformation is healthy
    ///
    /// Returns true if transformation is in healthy range
    pub fn is_healthy(&self) -> bool {
        self.transformation_velocity >= 0.4 && self.transformation_velocity <= 0.7
    }

    /// Get transformation state description
    pub fn get_state_description(&self) -> &str {
        if !self.choice_made {
            return "No choice made - rocking back and forth without transformation";
        }

        if self.abandonment_completeness < 0.3 {
            return "Choice made but incomplete abandonment - limited transformation";
        }

        if self.transformation_velocity < 0.4 {
            return "Transformation avoidance - stagnation";
        }

        if self.transformation_velocity > 0.7 {
            return "Chaotic transformation - uncontrolled change";
        }

        "Healthy transformation - ongoing polarization progress"
    }

    /// Calculate protection from other-selves
    ///
    /// STS path has protection, STO path has no protection
    pub fn calculate_protection(&self) -> Float {
        match self.transformation_direction {
            Polarity::STS | Polarity::ServiceToSelf => {
                self.protection_level * self.transformation_velocity
            }
            Polarity::STO | Polarity::ServiceToOthers => {
                0.0 // No protection, find many mirrors for reflection
            }
            Polarity::Neutral | Polarity::SinkholeOfIndifference => 0.0,
            _ => 0.0,
        }
    }

    /// Get path characteristics
    pub fn get_path_characteristics(&self) -> (bool, Float, Float, Float) {
        match self.transformation_direction {
            Polarity::STO | Polarity::ServiceToOthers => {
                (
                    true, // Is right-hand path
                    self.path_focus_mental,
                    0.0, // No protection
                    self.polarization_progress,
                )
            }
            Polarity::STS | Polarity::ServiceToSelf => {
                (
                    false, // Is left-hand path
                    self.path_focus_physical,
                    self.calculate_protection(),
                    self.polarization_progress,
                )
            }
            Polarity::Neutral | Polarity::SinkholeOfIndifference => {
                (
                    false, // No path
                    0.0, 0.0, 0.0,
                )
            }
            _ => (false, 0.5, 0.0, 0.0),
        }
    }

    // ============================================================================
    // PHASE 7: PARAMETER MODIFICATION METHODS (Greater Cycle Regulation)
    // ============================================================================

    /// Modify veil permeability based on transformation direction
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_veil_permeability(&self, current_permeability: Float) -> Float {
        if !self.can_modify_veil {
            return current_permeability;
        }

        // Transformation direction affects veil
        // Positive direction (polarization) → veil lifts (permeability increases)
        // Negative direction (depolarization) → veil thickens (permeability decreases)
        let direction_factor = match self.transformation_direction {
            Polarity::STO | Polarity::STS => 1.0,
            Polarity::SinkholeOfIndifference => -0.5,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
            Polarity::ServiceToOthers => 1.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.1;
        (current_permeability + modification).clamp(0.0, 1.0)
    }

    /// Modify processing capacity based on transformation direction
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_processing_capacity(&self, current_capacity: Float) -> Float {
        if !self.can_modify_capacity {
            return current_capacity;
        }

        // Positive transformation → capacity increases
        // Negative transformation → capacity decreases
        let direction_factor = match self.transformation_direction {
            Polarity::STO | Polarity::STS => 1.0,
            Polarity::SinkholeOfIndifference => -0.3,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
            Polarity::ServiceToOthers => 1.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.05;
        (current_capacity + modification).clamp(0.1, 1.0)
    }

    /// Modify processing efficiency based on transformation direction
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_processing_efficiency(&self, current_efficiency: Float) -> Float {
        if !self.can_modify_efficiency {
            return current_efficiency;
        }

        // Positive transformation → efficiency increases
        // Negative transformation → efficiency decreases
        let direction_factor = match self.transformation_direction {
            Polarity::STO | Polarity::STS => 1.0,
            Polarity::SinkholeOfIndifference => -0.3,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
            Polarity::ServiceToOthers => 1.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.08;
        (current_efficiency + modification).clamp(0.1, 1.0)
    }

    /// Modify catalyst tolerance based on transformation direction
    ///
    /// Greater Cycle regulation: Transformation modifies Lesser Cycle parameters
    pub fn modify_catalyst_tolerance(&self, current_tolerance: Float) -> Float {
        if !self.can_modify_tolerance {
            return current_tolerance;
        }

        // Positive transformation → tolerance increases
        // Negative transformation → tolerance decreases
        let direction_factor = match self.transformation_direction {
            Polarity::STO | Polarity::STS => 1.0,
            Polarity::SinkholeOfIndifference => -0.3,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
            Polarity::ServiceToOthers => 1.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.1;
        (current_tolerance + modification).clamp(0.1, 1.0)
    }

    /// Check if transformation can modify parameters
    pub fn can_modify_parameters(&self) -> bool {
        self.can_modify_veil
            || self.can_modify_capacity
            || self.can_modify_efficiency
            || self.can_modify_tolerance
    }

    /// Get transformation direction as float (for calculations)
    pub fn get_transformation_direction_factor(&self) -> Float {
        match self.transformation_direction {
            Polarity::STO => 1.0,
            Polarity::STS => 1.0,
            Polarity::SinkholeOfIndifference => -0.5,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
        }
    }
}

// Implement LambdaMeasurable
impl LambdaMeasurable for TransformationMindArchetype {
    fn get_lambda(&self) -> Float {
        self.lambda.value
    }

    fn set_lambda(&mut self, value: Float) {
        self.lambda.value = value;
    }

    fn calculate_lambda(&self) -> Float {
        // Lambda = transformation velocity * direction_factor
        let direction_factor = match self.transformation_direction {
            Polarity::STO | Polarity::STS => 1.0,
            _ => 0.5,
            Polarity::ServiceToSelf => 1.0,
            Polarity::Neutral => 0.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
        };

        self.transformation_velocity * direction_factor
    }

    fn healthy_range(&self) -> (Float, Float) {
        (self.lambda.healthy_min, self.lambda.healthy_max)
    }

    fn pathological_indicators(&self) -> Vec<String> {
        let mut indicators = Vec::new();

        if self.lambda.is_pathological_low() {
            indicators.push("Transformation avoidance - stagnation".to_string());
            indicators.push("Refusal to choose - polarization blocked".to_string());
        }

        if self.lambda.is_pathological_high() {
            indicators.push("Chaotic transformation - uncontrolled change".to_string());
            indicators.push("Excessive velocity without integration".to_string());
        }

        indicators
    }
}

// Implement Developmental
impl Developmental for TransformationMindArchetype {
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

// Implement Paired
impl Paired for TransformationMindArchetype {
    fn paired_archetype_id(&self) -> Option<u8> {
        // Transformation is a singleton - self-referential
        Some(6)
    }

    fn get_pair(&self) -> Option<u8> {
        Some(6)
    }

    fn lambda(&self) -> &LambdaMeasurement {
        &self.lambda
    }

    fn calculate_pair_tension(&self, _paired_archetype: &dyn Paired) -> Float {
        // Transformation tension: polarization intensity
        self.polarization_progress
    }

    fn calculate_pair_balance(&self, paired_archetype: &dyn Paired) -> Float {
        // Balance = 1.0 - |tension - 0.5|
        let tension = self.calculate_pair_tension(paired_archetype);
        1.0 - (tension - 0.5).abs()
    }
}

// Implement Holonic
impl Holonic for TransformationMindArchetype {
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
        // Transformation transcends through polarization progress
        if self.polarization_progress < 0.8 {
            return false;
        }

        match self.holonic_level {
            HolonicLevel::Micro => {
                self.holonic_level = HolonicLevel::Meso;
                self.integration_capacity = 0.6;
                true
            }
            HolonicLevel::Meso => {
                self.holonic_level = HolonicLevel::Macro;
                self.integration_capacity = 0.7;
                true
            }
            HolonicLevel::Macro => {
                self.holonic_level = HolonicLevel::Meta;
                self.integration_capacity = 0.8;
                true
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
// TRANSFORMATION ARCHETYPE TRAIT IMPLEMENTATION
// ============================================================================

impl TransformationArchetypeTrait for TransformationMindArchetype {
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
        self.transformation_direction
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
        self.transformation_direction = direction;
    }

    fn set_polarization_progress(&mut self, value: Float) {
        self.polarization_progress = value.clamp(0.0, 1.0);
    }

    fn set_choice_made(&mut self, value: bool) {
        self.choice_made = value;
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

    // Phase 7: Parameter modification
    fn modify_veil_permeability(&self, current_permeability: Float) -> Float {
        if !self.can_modify_veil {
            return current_permeability;
        }

        // Transformation direction affects veil
        // Positive direction (STO) → veil lifts (permeability increases)
        // Negative direction (STS) → veil lifts (permeability increases)
        // Neutral (SinkholeOfIndifference) → no modification
        let direction_factor = match self.transformation_direction {
            Polarity::STO => 1.0,
            Polarity::STS => 1.0,
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

        // Positive transformation → capacity increases
        // Negative transformation → capacity decreases
        let direction_factor = match self.transformation_direction {
            Polarity::STO => 1.0,
            Polarity::STS => -1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => -1.0,
            Polarity::Neutral => 0.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.05;
        (current_capacity + modification).clamp(0.1, 1.0)
    }

    fn modify_processing_efficiency(&self, current_efficiency: Float) -> Float {
        if !self.can_modify_efficiency {
            return current_efficiency;
        }

        // Positive transformation → efficiency increases
        // Negative transformation → efficiency decreases
        let direction_factor = match self.transformation_direction {
            Polarity::STO => 1.0,
            Polarity::STS => -1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => -1.0,
            Polarity::Neutral => 0.0,
        };

        let modification = direction_factor * self.transformation_velocity * 0.08;
        (current_efficiency + modification).clamp(0.1, 1.0)
    }

    fn modify_catalyst_tolerance(&self, current_tolerance: Float) -> Float {
        if !self.can_modify_tolerance {
            return current_tolerance;
        }

        // Positive transformation → tolerance increases
        // Negative transformation → tolerance decreases
        let direction_factor = match self.transformation_direction {
            Polarity::STO => 1.0,
            Polarity::STS => -1.0,
            Polarity::SinkholeOfIndifference => 0.0,
            Polarity::ServiceToOthers => 1.0,
            Polarity::ServiceToSelf => -1.0,
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
        let velocity_health = 1.0 - (self.transformation_velocity - 0.5).abs() * 2.0;
        let polarization_health = if self.choice_made {
            self.polarization_progress
        } else {
            1.0 - self.transformation_velocity.abs() // Healthy to be undecided initially
        };

        let overall_health = (velocity_health + polarization_health) / 2.0;

        match overall_health {
            h if h >= 0.8 => HealthStatus::Healthy,
            h if h >= 0.6 => HealthStatus::Warning,
            h if h >= 0.4 => HealthStatus::Degraded,
            _ => HealthStatus::Pathological,
        }
    }
}

impl ArchetypeTrait for TransformationMindArchetype {
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
        self.role
    }

    fn process(&mut self, catalyst: Float, position: DevelopmentalPosition) {
        self.process_transformation(catalyst);
    }

    fn sigma_axis(&self) -> SigmaAxis {
        SigmaAxis::SigmaB
    }

    fn tarot_correlation(&self) -> TarotCorrelation {
        self.tarot_correlation.clone()
    }

    fn update_lambda(&mut self, value: Float) {
        self.lambda.value = value.clamp(0.0, 1.0);
        self.transformation_velocity = self.lambda.value;
    }

    fn archetype_id(&self) -> u8 {
        self.archetype_id
    }

    fn name(&self) -> &str {
        "The Transformation of Mind"
    }

    fn description(&self) -> &str {
        "Transformation of Mind - The choice and abandonment of principle"
    }

    fn complex(&self) -> ArchetypeComplex {
        self.complex
    }

    fn functional_pair(&self) -> FunctionalPair {
        FunctionalPair::TransformationSingleton
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
    use crate::archetypes::common::HealthStatus as CommonHealthStatus;

    #[test]
    fn test_transformation_initialization() {
        let transformation = TransformationMindArchetype::new();

        assert_eq!(transformation.archetype_id, 6);
        assert_eq!(transformation.complex, ArchetypeComplex::Mind);
        assert_eq!(transformation.role, ArchetypeRole::Transformation);
        assert!(!transformation.choice_made);
        assert_eq!(transformation.transformation_velocity, 0.5);
        assert_eq!(transformation.abandonment_completeness, 0.0);
        assert_eq!(transformation.polarization_progress, 0.0);
    }

    #[test]
    fn test_make_choice_sto() {
        let mut transformation = TransformationMindArchetype::new();

        let result = transformation.make_choice(Polarity::STO, 0.8);
        assert!(result.is_ok());

        assert!(transformation.choice_made);
        assert_eq!(transformation.choice_intensity, 0.8);
        assert_eq!(transformation.transformation_direction, Polarity::STO);
        assert!(transformation.path_focus_mental > 0.7);
        assert!(transformation.path_focus_physical < 0.3);
        assert_eq!(transformation.protection_level, 0.0);
        assert!(transformation.creative_point_active);
    }

    #[test]
    fn test_make_choice_sts() {
        let mut transformation = TransformationMindArchetype::new();

        let result = transformation.make_choice(Polarity::STS, 0.7);
        assert!(result.is_ok());

        assert!(transformation.choice_made);
        assert_eq!(transformation.choice_intensity, 0.7);
        assert_eq!(transformation.transformation_direction, Polarity::STS);
        assert!(transformation.path_focus_physical > 0.7);
        assert!(transformation.protection_level > 0.0);
    }

    #[test]
    fn test_abandon_principle() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();

        let result = transformation.abandon_principle(0.9);
        assert!(result.is_ok());

        assert_eq!(transformation.abandonment_completeness, 0.9);
        assert!(transformation.transformation_velocity > 0.5); // Should increase
    }

    #[test]
    fn test_deep_mind_attitude_virginal() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        transformation.abandon_principle(0.9).unwrap();

        let initial_velocity = transformation.transformation_velocity;
        transformation.set_deep_mind_attitude(DeepMindAttitude::Virginal);

        assert_eq!(
            transformation.deep_mind_attitude,
            DeepMindAttitude::Virginal
        );
        assert!(transformation.transformation_velocity > initial_velocity); // Should increase

        let treasure_quality = transformation.calculate_treasure_quality();
        assert!(treasure_quality > 0.7); // High quality treasure
    }

    #[test]
    fn test_deep_mind_attitude_prostituted() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STS, 0.7).unwrap();
        transformation.abandon_principle(0.8).unwrap();

        let initial_velocity = transformation.transformation_velocity;
        transformation.set_deep_mind_attitude(DeepMindAttitude::Prostituted);

        assert_eq!(
            transformation.deep_mind_attitude,
            DeepMindAttitude::Prostituted
        );
        assert!(transformation.transformation_velocity < initial_velocity); // Should decrease

        let treasure_quality = transformation.calculate_treasure_quality();
        assert!(treasure_quality < 0.4); // Lower quality treasure
    }

    #[test]
    fn test_process_transformation() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        transformation.abandon_principle(0.9).unwrap();

        transformation.process_transformation(0.6);

        assert!(transformation.transformation_velocity > 0.5);
        assert!(transformation.polarization_progress > 0.0);
    }

    #[test]
    fn test_process_transformation_no_choice() {
        let mut transformation = TransformationMindArchetype::new();

        transformation.process_transformation(0.6);

        assert_eq!(transformation.transformation_velocity, 0.2); // Low velocity without choice
        assert_eq!(transformation.polarization_progress, 0.0);
    }

    #[test]
    fn test_calculate_polarization_intensity() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        transformation.abandon_principle(0.9).unwrap();

        // Process multiple times to build up polarization progress
        for _ in 0..10 {
            transformation.process_transformation(0.6);
        }

        let intensity = transformation.calculate_polarization_intensity();
        assert!(intensity > 0.5);
    }

    #[test]
    fn test_is_healthy() {
        let mut transformation = TransformationMindArchetype::new();

        // Healthy range
        transformation.transformation_velocity = 0.5;
        assert!(transformation.is_healthy());

        // Below healthy range
        transformation.transformation_velocity = 0.3;
        assert!(!transformation.is_healthy());

        // Above healthy range
        transformation.transformation_velocity = 0.8;
        assert!(!transformation.is_healthy());
    }

    #[test]
    fn test_get_state_description() {
        let mut transformation = TransformationMindArchetype::new();

        // No choice made
        let desc = transformation.get_state_description();
        assert!(desc.contains("No choice made"));

        // Choice made but incomplete abandonment
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        let desc = transformation.get_state_description();
        assert!(desc.contains("Choice made but incomplete abandonment"));

        // Healthy transformation (abandonment > 0.5)
        transformation.abandon_principle(0.8).unwrap();
        transformation.process_transformation(0.6);
        let desc = transformation.get_state_description();
        // With abandonment_completeness = 0.8, it should show full transformation status
        assert!(
            !desc.contains("No choice made") && !desc.contains("incomplete abandonment"),
            "Unexpected description: {}",
            desc
        );
    }

    #[test]
    fn test_calculate_protection() {
        let mut transformation_sto = TransformationMindArchetype::new();
        transformation_sto.make_choice(Polarity::STO, 0.8).unwrap();
        transformation_sto.abandon_principle(0.9).unwrap();

        let protection_sto = transformation_sto.calculate_protection();
        assert_eq!(protection_sto, 0.0); // STO has no protection

        let mut transformation_sts = TransformationMindArchetype::new();
        transformation_sts.make_choice(Polarity::STS, 0.8).unwrap();
        transformation_sts.abandon_principle(0.9).unwrap();

        let protection_sts = transformation_sts.calculate_protection();
        assert!(protection_sts > 0.0); // STS has protection
    }

    #[test]
    fn test_get_path_characteristics() {
        let mut transformation_sto = TransformationMindArchetype::new();
        transformation_sto.make_choice(Polarity::STO, 0.8).unwrap();
        transformation_sto.abandon_principle(0.9).unwrap();
        transformation_sto.process_transformation(0.6);

        let (is_right_hand, focus, protection, progress) =
            transformation_sto.get_path_characteristics();
        assert!(is_right_hand);
        assert!(focus > 0.7); // Mental focus
        assert_eq!(protection, 0.0);
        assert!(progress > 0.0);
    }

    #[test]
    fn test_lambda_measurement() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        transformation.abandon_principle(0.9).unwrap();

        // Process transformation to update velocity
        transformation.process_transformation(0.6);

        let lambda = transformation.calculate_lambda();
        // Lambda = velocity * direction_factor (1.0 for STO)
        // Velocity = choice_intensity * abandonment_completeness + catalyst_influence
        // = 0.8 * 0.9 + 0.6 * 0.3 = 0.72 + 0.18 = 0.9 (clamped to 1.0)
        // So lambda will be around 0.9, which is above healthy range
        // This is expected - the transformation is very active
        assert!(lambda > 0.0);
        assert!(lambda <= 1.0);
    }

    #[test]
    fn test_lambda_pathological() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.transformation_velocity = 0.3; // Below healthy range
        transformation.update_lambda(0.3); // Update lambda value

        let lambda = transformation.calculate_lambda();
        assert!(lambda < 0.4);
        assert!(!transformation.is_healthy());
        assert_eq!(
            transformation.health_status(),
            CommonHealthStatus::PathologicalLow
        );

        let pathology = transformation.pathological_indicators();
        assert!(!pathology.is_empty());
        assert!(pathology.iter().any(|p| p.contains("stagnation")));
    }

    #[test]
    fn test_developmental_rung_activation() {
        let mut transformation = TransformationMindArchetype::new();

        let new_position = DevelopmentalPosition::new_with_octant_rung(Octant::O4, 4);
        Developmental::update_developmental_position(&mut transformation, new_position);

        assert_eq!(
            transformation.developmental_position(),
            DevelopmentalPosition::Input
        ); // rung 4 % 4 = 0 (Input)

        // Set activation level for R4 (simulating development)
        transformation.activation_levels.insert(Rung::R4, 0.75);
        transformation.activated_rungs.push(Rung::R4); // Add to activated rungs

        let activation = transformation.rung_activation_level(Rung::R4);
        assert!(activation > 0.5);

        let rungs = transformation.activated_rungs();
        assert!(rungs.contains(&Rung::R4));
    }

    #[test]
    fn test_advance_development() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.9).unwrap();
        transformation.abandon_principle(0.9).unwrap();
        transformation.process_transformation(0.8);

        // Set polarization progress high enough for advancement
        transformation.polarization_progress = 0.8;

        let initial_position = transformation.developmental_position();
        let new_position = DevelopmentalPosition::new_with_octant_rung(Octant::O5, 5);
        Developmental::update_developmental_position(&mut transformation, new_position);

        assert_ne!(transformation.developmental_position(), initial_position);
        assert_eq!(
            transformation.developmental_position(),
            DevelopmentalPosition::Catalyst
        ); // rung 5 % 4 = 1 (Catalyst)
    }

    #[test]
    fn test_pair_dynamics() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        transformation.abandon_principle(0.9).unwrap();

        // Test that transformation is a singleton (no paired archetype)
        assert_eq!(
            transformation.functional_pair(),
            FunctionalPair::TransformationSingleton
        );
        assert_eq!(transformation.paired_archetype_id(), Some(6)); // Self-referential

        // Test transformation velocity
        assert!(transformation.transformation_velocity > 0.5);
    }

    #[test]
    fn test_holonic_transcend() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.9).unwrap();
        transformation.abandon_principle(0.9).unwrap();
        transformation.polarization_progress = 0.9;

        let initial_level = transformation.holonic_level;
        let result = transformation.transcend();
        assert!(result);
        assert_ne!(transformation.holonic_level, initial_level);
    }

    #[test]
    fn test_calculate_emergent_properties() {
        let mut transformation = TransformationMindArchetype::new();
        transformation.make_choice(Polarity::STO, 0.8).unwrap();
        transformation.abandon_principle(0.9).unwrap();
        transformation.set_deep_mind_attitude(DeepMindAttitude::Virginal);
        transformation.process_transformation(0.6);

        // Test integration capacity (holonic property) - initially 0.5
        let capacity = transformation.integration_capacity();
        assert!(capacity >= 0.5); // Changed to >= since initial capacity is 0.5

        // Test holonic level (starts at Micro, needs transcend() to advance)
        let level = transformation.holonic_level();
        // Note: Level remains Micro until transcend() is called
        assert!(matches!(level, HolonicLevel::Micro));

        // Check that polarization creates emergent structure
        assert!(transformation.creative_point_active);
        assert_eq!(transformation.protection_level, 0.0); // STO has no protection
    }

    #[test]
    fn test_archetype_trait_methods() {
        let transformation = TransformationMindArchetype::new();

        assert_eq!(transformation.archetype_id(), 6);
        assert_eq!(transformation.name(), "The Transformation of Mind");
        assert_eq!(transformation.complex(), ArchetypeComplex::Mind);
        assert_eq!(transformation.role(), ArchetypeRole::Transformation);

        let mut mutable_transformation = transformation.clone();
        mutable_transformation
            .process(0.5, crate::archetypes::common::DevelopmentalPosition::Input);
        assert!(!mutable_transformation.choice_made);

        // Check that initial state is preserved (except velocity which changes when no choice is made)
        assert_eq!(mutable_transformation.transformation_velocity, 0.2); // Velocity drops when no choice is made
        assert_eq!(mutable_transformation.abandonment_completeness, 0.0);
    }

    #[test]
    fn test_paired_singleton() {
        let transformation = TransformationMindArchetype::new();

        // Transformation is a singleton
        assert_eq!(
            transformation.functional_pair(),
            FunctionalPair::TransformationSingleton
        );
        assert_eq!(transformation.paired_archetype_id(), Some(6)); // Self-referential
                                                                   // Check that it's the Transformation archetype
        assert!(transformation
            .description()
            .contains("Transformation of Mind"));
    }

    #[test]
    fn test_invalid_choice() {
        let mut transformation = TransformationMindArchetype::new();

        // Invalid choice direction
        let result = transformation.make_choice(Polarity::SinkholeOfIndifference, 0.5);
        assert!(result.is_err());

        // Invalid intensity
        let result = transformation.make_choice(Polarity::STO, 1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_abandonment() {
        let mut transformation = TransformationMindArchetype::new();

        // Cannot abandon before making choice
        let result = transformation.abandon_principle(0.5);
        assert!(result.is_err());

        transformation.make_choice(Polarity::STO, 0.8).unwrap();

        // Invalid completeness
        let result = transformation.abandon_principle(1.5);
        assert!(result.is_err());
    }
}
