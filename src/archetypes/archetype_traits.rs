// Archetype Traits for Complex Refactoring (Phase 10)
//
// This file defines trait objects for all 7 archetype types to enable polymorphism
// between Mind, Body, and Spirit variants. This is essential for supporting
// complex-specific dynamics and the mirror principle.

use crate::archetypes::common::{DevelopmentalPosition, HealthStatus};
use crate::types::{Float, Polarity, Rung};

// ============================================================================
// MATRIX ARCHETYPE TRAIT
// ============================================================================

/// Trait for Matrix archetypes (A1, A8, A15)
///
/// Matrix is the input container - the conscious structure that receives
/// and organizes experience. Different complexes have different Matrix dynamics:
/// - Mind: Matrix REACHES toward Potentiator (active seeking passive)
/// - Body: Potentiator REGULATES Matrix activity (wisdom guiding motion)
/// - Spirit: Potentiator ILLUMINATES Matrix (lightning striking primeval darkness)
pub trait MatrixArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    // Phase 13: Add #[inline] to small getter methods for performance

    fn get_archetype_id(&self) -> u8;

    fn get_structural_permeability(&self) -> Float;

    fn get_resource_access(&self) -> Float;

    fn get_willful_integration(&self) -> Float;

    fn get_conscious_coherence(&self) -> Float;

    fn get_structural_tension(&self) -> Float;

    fn get_integration_capacity(&self) -> Float;

    // Core setters
    // Phase 13: Add #[inline] to small setter methods for performance

    fn set_structural_permeability(&mut self, value: Float);

    fn set_resource_access(&mut self, value: Float);

    fn set_willful_integration(&mut self, value: Float);

    fn set_conscious_coherence(&mut self, value: Float);

    fn set_structural_tension(&mut self, value: Float);

    fn set_integration_capacity(&mut self, value: Float);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Phase 4: Capacity mechanics
    // Phase 13: Add #[inline] to small capacity methods for performance

    fn get_structural_capacity(&self) -> Float;

    fn get_current_load(&self) -> Float;

    fn get_processing_efficiency(&self) -> Float;

    fn get_accumulation_rate(&self) -> Float;

    fn set_structural_capacity(&mut self, value: Float);

    fn set_current_load(&mut self, value: Float);

    fn set_processing_efficiency(&mut self, value: Float);

    fn set_accumulation_rate(&mut self, value: Float);
    fn available_capacity(&self) -> Float;

    fn is_overloaded(&self) -> bool;
    fn increase_load(&mut self, load: Float);
    fn decrease_load(&mut self, load: Float);
    fn expand_capacity(&mut self, experience: Float);

    // Complex-specific M-P dynamics (Phase 2)
    fn calculate_reaching_intensity(&self) -> Float;
    fn calculate_regulatory_susceptibility(&self) -> Float;
    fn calculate_illumination_receptivity(&self) -> Float;

    // Phase 3: Veil mechanics - transformation methods
    fn calculate_structural_transformation(&self) -> Float;
    fn calculate_state_transformation(&self) -> Float;

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;
}

// ============================================================================
// POTENTIATOR ARCHETYPE TRAIT
// ============================================================================

/// Trait for Potentiator archetypes (A2, A9, A16)
///
/// Potentiator is the resource cache - the unconscious potential.
/// Different complexes have different Potentiator dynamics:
/// - Mind: Potentiator provides RECEPTIVITY to Matrix's reaching
/// - Body: Potentiator provides REGULATORY INTENSITY to guide Matrix
/// - Spirit: Potentiator provides ILLUMINATION to strike Matrix
pub trait PotentiatorArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    // Phase 13: Add #[inline] to small getter methods for performance

    fn get_archetype_id(&self) -> u8;
    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement;

    fn get_resource_accessibility(&self) -> Float;

    fn get_resource_quality(&self) -> Float;

    fn get_resource_depth(&self) -> Float;

    // Core setters
    // Phase 13: Add #[inline] to small setter methods for performance

    fn set_resource_accessibility(&mut self, value: Float);

    fn set_resource_quality(&mut self, value: Float);

    fn set_resource_depth(&mut self, value: Float);
    fn increase_resource_depth(&mut self, increase: Float);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Complex-specific M-P dynamics (Phase 2)
    fn calculate_receptivity(&self) -> Float;
    fn calculate_regulatory_intensity(&self) -> Float;
    fn calculate_illumination_intensity(&self) -> Float;

    // Phase 5: Resource structure methods
    fn calculate_resource_availability(&self) -> Float;
    fn calculate_resource_diversity(&self) -> Float;

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;
}

// ============================================================================
// CATALYST ARCHETYPE TRAIT
// ============================================================================

/// Trait for Catalyst archetypes (A3, A10, A17)
///
/// Catalyst is the input friction - raw experience that acts upon
/// the conscious mind to change it.
pub trait CatalystArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    // Phase 13: Add #[inline] to small getter methods for performance

    fn get_archetype_id(&self) -> u8;

    fn get_processing_rate(&self) -> Float;

    fn get_catalyst_inflow(&self) -> Float;

    fn get_processing_capacity(&self) -> Float;

    fn get_accumulation_rate(&self) -> Float;

    fn get_processing_efficiency(&self) -> Float;

    fn get_catalyst_quality(&self) -> Float;

    // Core setters
    // Phase 13: Add #[inline] to small setter methods for performance

    fn set_processing_rate(&mut self, value: Float);

    fn set_catalyst_inflow(&mut self, value: Float);

    fn set_processing_capacity(&mut self, value: Float);

    fn set_accumulation_rate(&mut self, value: Float);

    fn set_processing_efficiency(&mut self, value: Float);

    fn set_catalyst_quality(&mut self, value: Float);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Veil mechanics

    fn get_veil_transparency(&self) -> Float;

    fn get_veil_filtering(&self) -> Float;

    fn set_veil_transparency(&mut self, value: Float);

    fn set_veil_filtering(&mut self, value: Float);

    // Polarity

    fn get_polarization_potential(&self) -> Float;
    fn get_current_polarization(&self) -> Option<Polarity>;
    fn set_current_polarization(&mut self, polarization: Option<Polarity>);

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;

    // Flow management methods (used by complex.rs)
    fn clear_flows(&mut self);
    fn add_flow(&mut self, rung: Rung, flow: Float);
    fn get_flow(&self, rung: Rung) -> Float;
}

// ============================================================================
// EXPERIENCE ARCHETYPE TRAIT
// ============================================================================

/// Trait for Experience archetypes (A4, A11, A18)
///
/// Experience is the processed output - integrated wisdom stored
/// in the unconscious which creates its continuing bias.
pub trait ExperienceArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    fn get_archetype_id(&self) -> u8;
    fn get_experience_depth(&self) -> Float;
    fn get_integration_quality(&self) -> Float;
    fn get_continuing_bias_strength(&self) -> Float;

    // Core setters
    fn set_experience_depth(&mut self, value: Float);
    fn set_integration_quality(&mut self, value: Float);
    fn set_continuing_bias_strength(&mut self, value: Float);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Phase 6: Bias structure
    fn form_continuing_bias(&mut self, experience: Float);
    fn get_bias_direction(&self) -> Float;
    fn apply_bias_to_matrix(&self, matrix: &mut Box<dyn MatrixArchetypeTrait>);
    fn apply_bias_to_potentiator(&self, potentiator: &mut Box<dyn PotentiatorArchetypeTrait>);

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;

    // Experience management methods (used by complex.rs)
    fn store_experiences(&mut self, experiences: Vec<crate::complex::Experience>);
    fn get_bias(&self, rung: Rung) -> Float;
}

// ============================================================================
// SIGNIFICATOR ARCHETYPE TRAIT
// ============================================================================

/// Trait for Significator archetypes (A5, A12, A19)
///
/// Significator is the identity agent - the choosing entity at the
/// heart of the mind complex that makes decisions and polarizes.
pub trait SignificatorArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    fn get_archetype_id(&self) -> u8;
    fn get_identity_coherence(&self) -> Float;
    fn get_choice_clarity(&self) -> Float;
    fn get_will_power(&self) -> Float;
    fn get_polarization_tendency(&self) -> Float;

    // Core setters
    fn set_identity_coherence(&mut self, value: Float);
    fn set_choice_clarity(&mut self, value: Float);
    fn set_will_power(&mut self, value: Float);
    fn set_polarization_tendency(&mut self, value: Float);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Choice mechanism (Phase 8)
    fn make_choice_with_state(
        &mut self,
        state: &crate::complex::LesserCycleState,
    ) -> crate::complex::Choice;

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;

    // Flow management methods (used by complex.rs)
    fn clear_flows(&mut self);
    fn add_flow(&mut self, rung: Rung, flow: Float);
    fn get_flow(&self, rung: Rung) -> Float;
}

// ============================================================================
// TRANSFORMATION ARCHETYPE TRAIT
// ============================================================================

/// Trait for Transformation archetypes (A6, A13, A20)
///
/// Transformation is where choice meets action - the zero-point of
/// polarization becomes actualized transformation.
pub trait TransformationArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    fn get_archetype_id(&self) -> u8;
    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement;
    fn get_transformation_velocity(&self) -> Float;
    fn get_transformation_direction(&self) -> Polarity;
    fn get_polarization_progress(&self) -> Float;
    fn get_choice_made(&self) -> bool;

    // Core setters
    fn set_transformation_velocity(&mut self, value: Float);
    fn set_transformation_direction(&mut self, direction: Polarity);
    fn set_polarization_progress(&mut self, value: Float);
    fn set_choice_made(&mut self, value: bool);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Phase 7: Parameter modification
    fn modify_veil_permeability(&self, current_permeability: Float) -> Float;
    fn modify_processing_capacity(&self, current_capacity: Float) -> Float;
    fn modify_processing_efficiency(&self, current_efficiency: Float) -> Float;
    fn modify_catalyst_tolerance(&self, current_tolerance: Float) -> Float;

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;
}

// ============================================================================
// GREAT WAY ARCHETYPE TRAIT
// ============================================================================

/// Trait for Great Way archetypes (A7, A14, A21)
///
/// Great Way denotes and configures the particular framework within
/// which the archetypes move - defining the MILIEU for processing.
pub trait GreatWayArchetypeTrait: std::fmt::Debug + Send + Sync {
    // Core getters
    fn get_archetype_id(&self) -> u8;
    fn get_lambda(&self) -> crate::archetypes::common::LambdaMeasurement;
    fn get_framework_clarity(&self) -> Float;
    fn get_framework_alignment(&self) -> Float;

    // Core setters
    fn set_framework_clarity(&mut self, value: Float);
    fn set_framework_alignment(&mut self, value: Float);

    // Developmental
    fn get_developmental_position(&self) -> DevelopmentalPosition;
    fn get_activated_rungs(&self) -> Vec<Rung>;
    fn get_activation_level(&self, rung: Rung) -> Float;
    fn set_activation_level(&mut self, rung: Rung, level: Float);

    // Phase 7: Milieu definition
    fn get_milieu(&self) -> crate::archetypes::a7_mind_great_way::Milieu;
    fn set_milieu(&mut self, milieu: crate::archetypes::a7_mind_great_way::Milieu);
    fn update_coupling_coefficient(&self, current_coefficient: Float) -> Float;

    // Common archetype methods
    fn update_developmental_position(&mut self, position: DevelopmentalPosition);
    fn get_health_status(&self) -> HealthStatus;
}
