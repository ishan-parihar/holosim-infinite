// Archetype Complexes Module
//
// This module implements the three complexes of the Archetypical Mind:
// - Mind Complex (A1-A7): σA - Mind Capacity
// - Body Complex (A8-A14): σB - Body Capacity
// - Spirit Complex (A15-A21): σC - Spirit Capacity
//
// Each complex has 7 archetypes following the same pattern:
// 1. Matrix - input container
// 2. Potentiator - resource cache
// 3. Catalyst - input friction
// 4. Experience - processed output
// 5. Significator - identity agent
// 6. Transformation - mutation event
// 7. Great Way - environment container
//
// Knowledge Base Reference: Archetypes/0. Archetypical Mind System.json

use crate::archetypes::{
    common::ArchetypeTrait, CatalystBodyArchetype, CatalystMindArchetype, CatalystSpiritArchetype,
    ExperienceBodyArchetype, ExperienceMindArchetype, ExperienceSpiritArchetype,
    GreatWayBodyArchetype, GreatWayMindArchetype, GreatWaySpiritArchetype, MatrixBodyArchetype,
    MatrixMindArchetype, MatrixSpiritArchetype, PotentiatorBodyArchetype, PotentiatorMindArchetype,
    PotentiatorSpiritArchetype, SignificatorBodyArchetype, SignificatorMindArchetype,
    SignificatorSpiritArchetype, TransformationBodyArchetype, TransformationMindArchetype,
    TransformationSpiritArchetype,
};
use crate::types::Float;
use std::fmt;

// ============================================================================
// MIND COMPLEX (A1-A7)
// ============================================================================

/// Mind Complex - σA (Mind Capacity)
///
/// The 7 archetypes of the Mind Complex represent the processing architecture
/// for conscious thought, reasoning, and intellectual development.
///
/// Knowledge Base Reference: Archetypes/1. Mind/
#[derive(Debug, Clone)]
pub struct MindComplex {
    /// A1: Matrix of the Mind - input container
    pub matrix: MatrixMindArchetype,

    /// A2: Potentiator of the Mind - resource cache
    pub potentiator: PotentiatorMindArchetype,

    /// A3: Catalyst of the Mind - input friction
    pub catalyst: CatalystMindArchetype,

    /// A4: Experience of the Mind - processed output
    pub experience: ExperienceMindArchetype,

    /// A5: Significator of the Mind - identity agent
    pub significator: SignificatorMindArchetype,

    /// A6: Transformation of the Mind - mutation event
    pub transformation: TransformationMindArchetype,

    /// A7: Great Way of the Mind - environment container
    pub great_way: GreatWayMindArchetype,
}

impl MindComplex {
    /// Create a new Mind Complex with initialized archetypes
    pub fn new() -> Self {
        Self {
            matrix: MatrixMindArchetype::new(),
            potentiator: PotentiatorMindArchetype::new(),
            catalyst: CatalystMindArchetype::new(),
            experience: ExperienceMindArchetype::new(),
            significator: SignificatorMindArchetype::new(),
            transformation: TransformationMindArchetype::new(),
            great_way: GreatWayMindArchetype::new(),
        }
    }

    /// Get Matrix (A1) - input container
    pub fn get_matrix(&self) -> &MatrixMindArchetype {
        &self.matrix
    }

    /// Get Potentiator (A2) - resource cache
    pub fn get_potentiator(&self) -> &PotentiatorMindArchetype {
        &self.potentiator
    }

    /// Get Catalyst (A3) - input friction
    pub fn get_catalyst(&self) -> &CatalystMindArchetype {
        &self.catalyst
    }

    /// Get Experience (A4) - processed output
    pub fn get_experience(&self) -> &ExperienceMindArchetype {
        &self.experience
    }

    /// Get Significator (A5) - identity agent
    pub fn get_significator(&self) -> &SignificatorMindArchetype {
        &self.significator
    }

    /// Get Transformation (A6) - mutation event
    pub fn get_transformation(&self) -> &TransformationMindArchetype {
        &self.transformation
    }

    /// Get Great Way (A7) - environment container
    pub fn get_great_way(&self) -> &GreatWayMindArchetype {
        &self.great_way
    }

    /// Calculate Mind Capacity (σA)
    ///
    /// Mind Capacity represents the overall processing capability of the Mind Complex.
    /// It is calculated as the average lambda value of all 7 archetypes.
    pub fn calculate_mind_capacity(&self) -> Float {
        let sum = self.matrix.lambda().value
            + self.potentiator.lambda().value
            + self.catalyst.lambda().value
            + self.experience.lambda().value
            + self.significator.lambda().value
            + self.transformation.lambda().value
            + self.great_way.lambda().value;
        sum / 7.0
    }

    /// Assess Mind Complex health
    pub fn assess_health(&self) -> bool {
        self.matrix.is_healthy()
            && self.potentiator.is_healthy()
            && self.catalyst.is_healthy()
            && self.experience.is_healthy()
            && self.significator.is_healthy()
            && self.transformation.is_healthy()
            && self.great_way.is_healthy()
    }
}

impl Default for MindComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MindComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mind Complex (σA):\n\
             - Matrix (A1): {:.3}\n\
             - Potentiator (A2): {:.3}\n\
             - Catalyst (A3): {:.3}\n\
             - Experience (A4): {:.3}\n\
- Significator (A5): {:.3}\n\
              - Transformation (A6): {:.3}\n\
              - Great Way (A7): {:.3}\n\
              - Mind Capacity (σA): {:.3}",
            self.matrix.lambda().value,
            self.potentiator.lambda().value,
            self.catalyst.lambda().value,
            self.experience.lambda().value,
            self.significator.lambda().value,
            self.transformation.lambda().value,
            self.great_way.lambda().value,
            self.calculate_mind_capacity(),
        )
    }
}

// ============================================================================
// BODY COMPLEX (A8-A14)
// ============================================================================

/// Body Complex - σB (Body Capacity)
///
/// The 7 archetypes of the Body Complex represent the processing architecture
/// for physical sensation, embodiment, and bodily wisdom.
///
/// Knowledge Base Reference: Archetypes/2. Body/
#[derive(Debug, Clone)]
pub struct BodyComplex {
    /// A8: Matrix of the Body - input container
    pub matrix: MatrixBodyArchetype,

    /// A9: Potentiator of the Body - resource cache
    pub potentiator: PotentiatorBodyArchetype,

    /// A10: Catalyst of the Body - input friction
    pub catalyst: CatalystBodyArchetype,

    /// A11: Experience of the Body - processed output
    pub experience: ExperienceBodyArchetype,

    /// A12: Significator of the Body - identity agent
    pub significator: SignificatorBodyArchetype,

    /// A13: Transformation of the Body - mutation event
    pub transformation: TransformationBodyArchetype,

    /// A14: Great Way of the Body - environment container
    pub great_way: GreatWayBodyArchetype,
}

impl BodyComplex {
    /// Create a new Body Complex with initialized archetypes
    pub fn new() -> Self {
        Self {
            matrix: MatrixBodyArchetype::new(),
            potentiator: PotentiatorBodyArchetype::new(),
            catalyst: CatalystBodyArchetype::new(),
            experience: ExperienceBodyArchetype::new(),
            significator: SignificatorBodyArchetype::new(),
            transformation: TransformationBodyArchetype::new(),
            great_way: GreatWayBodyArchetype::new(),
        }
    }

    /// Get Matrix (A8) - input container
    pub fn get_matrix(&self) -> &MatrixBodyArchetype {
        &self.matrix
    }

    /// Get Potentiator (A9) - resource cache
    pub fn get_potentiator(&self) -> &PotentiatorBodyArchetype {
        &self.potentiator
    }

    /// Get Catalyst (A10) - input friction
    pub fn get_catalyst(&self) -> &CatalystBodyArchetype {
        &self.catalyst
    }

    /// Get Experience (A11) - processed output
    pub fn get_experience(&self) -> &ExperienceBodyArchetype {
        &self.experience
    }

    /// Get Significator (A12) - identity agent
    pub fn get_significator(&self) -> &SignificatorBodyArchetype {
        &self.significator
    }

    /// Get Transformation (A13) - mutation event
    pub fn get_transformation(&self) -> &TransformationBodyArchetype {
        &self.transformation
    }

    /// Get Great Way (A14) - environment container
    pub fn get_great_way(&self) -> &GreatWayBodyArchetype {
        &self.great_way
    }

    /// Calculate Body Capacity (σB)
    ///
    /// Body Capacity represents the overall processing capability of the Body Complex.
    pub fn calculate_body_capacity(&self) -> Float {
        let sum = self.matrix.lambda.value
            + self.potentiator.lambda.value
            + self.catalyst.lambda.value
            + self.experience.lambda.value
            + self.significator.lambda.value
            + self.transformation.lambda.value
            + self.great_way.lambda.value;
        sum / 7.0
    }

    /// Assess Body Complex health
    pub fn assess_health(&self) -> bool {
        self.matrix.is_healthy()
            && self.potentiator.is_healthy()
            && self.catalyst.is_healthy()
            && self.experience.is_healthy()
            && self.significator.is_healthy()
            && self.transformation.is_healthy()
            && self.great_way.is_healthy()
    }
}

impl Default for BodyComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BodyComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Body Complex (σB):\n\
             - Matrix (A8): {:.3}\n\
             - Potentiator (A9): {:.3}\n\
             - Catalyst (A10): {:.3}\n\
             - Experience (A11): {:.3}\n\
             - Significator (A12): {:.3}\n\
             - Transformation (A13): {:.3}\n\
             - Great Way (A14): {:.3}\n\
             - Body Capacity (σB): {:.3}",
            self.matrix.lambda.value,
            self.potentiator.lambda.value,
            self.catalyst.lambda.value,
            self.experience.lambda.value,
            self.significator.lambda.value,
            self.transformation.lambda.value,
            self.great_way.lambda.value,
            self.calculate_body_capacity(),
        )
    }
}

// ============================================================================
// SPIRIT COMPLEX (A15-A21)
// ============================================================================

/// Spirit Complex - σC (Spirit Capacity)
///
/// The 7 archetypes of the Spirit Complex represent the processing architecture
/// for spiritual connection, intuition, and higher consciousness.
///
/// Knowledge Base Reference: Archetypes/3. Spirit/
#[derive(Debug, Clone)]
pub struct SpiritComplex {
    /// A15: Matrix of the Spirit - input container
    pub matrix: MatrixSpiritArchetype,

    /// A16: Potentiator of the Spirit - resource cache
    pub potentiator: PotentiatorSpiritArchetype,

    /// A17: Catalyst of the Spirit - input friction
    pub catalyst: CatalystSpiritArchetype,

    /// A18: Experience of the Spirit - processed output
    pub experience: ExperienceSpiritArchetype,

    /// A19: Significator of the Spirit - identity agent
    pub significator: SignificatorSpiritArchetype,

    /// A20: Transformation of the Spirit - mutation event
    pub transformation: TransformationSpiritArchetype,

    /// A21: Great Way of the Spirit - environment container
    pub great_way: GreatWaySpiritArchetype,
}

impl SpiritComplex {
    /// Create a new Spirit Complex with initialized archetypes
    pub fn new() -> Self {
        Self {
            matrix: MatrixSpiritArchetype::new(),
            potentiator: PotentiatorSpiritArchetype::new(),
            catalyst: CatalystSpiritArchetype::new(),
            experience: ExperienceSpiritArchetype::new(),
            significator: SignificatorSpiritArchetype::new(),
            transformation: TransformationSpiritArchetype::new(),
            great_way: GreatWaySpiritArchetype::new(),
        }
    }

    /// Get Matrix (A15) - input container
    pub fn get_matrix(&self) -> &MatrixSpiritArchetype {
        &self.matrix
    }

    /// Get Potentiator (A16) - resource cache
    pub fn get_potentiator(&self) -> &PotentiatorSpiritArchetype {
        &self.potentiator
    }

    /// Get Catalyst (A17) - input friction
    pub fn get_catalyst(&self) -> &CatalystSpiritArchetype {
        &self.catalyst
    }

    /// Get Experience (A18) - processed output
    pub fn get_experience(&self) -> &ExperienceSpiritArchetype {
        &self.experience
    }

    /// Get Significator (A19) - identity agent
    pub fn get_significator(&self) -> &SignificatorSpiritArchetype {
        &self.significator
    }

    /// Get Transformation (A20) - mutation event
    pub fn get_transformation(&self) -> &TransformationSpiritArchetype {
        &self.transformation
    }

    /// Get Great Way (A21) - environment container
    pub fn get_great_way(&self) -> &GreatWaySpiritArchetype {
        &self.great_way
    }

    /// Calculate Spirit Capacity (σC)
    ///
    /// Spirit Capacity represents the overall processing capability of the Spirit Complex.
    pub fn calculate_spirit_capacity(&self) -> Float {
        let sum = self.matrix.lambda.value
            + self.potentiator.lambda.value
            + self.catalyst.lambda.value
            + self.experience.lambda.value
            + self.significator.lambda.value
            + self.transformation.lambda.value
            + self.great_way.lambda.value;
        sum / 7.0
    }

    /// Assess Spirit Complex health
    pub fn assess_health(&self) -> bool {
        self.matrix.is_healthy()
            && self.potentiator.is_healthy()
            && self.catalyst.is_healthy()
            && self.experience.is_healthy()
            && self.significator.is_healthy()
            && self.transformation.is_healthy()
            && self.great_way.is_healthy()
    }
}

impl Default for SpiritComplex {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SpiritComplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Spirit Complex (σC):\n\
             - Matrix (A15): {:.3}\n\
             - Potentiator (A16): {:.3}\n\
             - Catalyst (A17): {:.3}\n\
             - Experience (A18): {:.3}\n\
             - Significator (A19): {:.3}\n\
             - Transformation (A20): {:.3}\n\
             - Great Way (A21): {:.3}\n\
             - Spirit Capacity (σC): {:.3}",
            self.matrix.lambda.value,
            self.potentiator.lambda.value,
            self.catalyst.lambda.value,
            self.experience.lambda.value,
            self.significator.lambda.value,
            self.transformation.lambda.value,
            self.great_way.lambda.value,
            self.calculate_spirit_capacity(),
        )
    }
}

// ============================================================================
// ARCHETYPE COMPLEX TRAIT
// ============================================================================

/// ArchetypeComplex trait for polymorphic access to all three complexes
///
/// This trait allows treating Mind, Body, and Spirit complexes uniformly
/// for operations like cycle processing and capacity calculation.
pub trait ArchetypeComplex {
    /// Get the complex's lambda value (average of all 7 archetypes)
    fn get_complex_lambda(&self) -> Float;

    /// Check if the complex is healthy
    fn is_healthy(&self) -> bool;

    /// Get a description of the complex
    fn describe(&self) -> String;
}

impl ArchetypeComplex for MindComplex {
    fn get_complex_lambda(&self) -> Float {
        self.calculate_mind_capacity()
    }

    fn is_healthy(&self) -> bool {
        self.assess_health()
    }

    fn describe(&self) -> String {
        format!("{}", self)
    }
}

impl ArchetypeComplex for BodyComplex {
    fn get_complex_lambda(&self) -> Float {
        self.calculate_body_capacity()
    }

    fn is_healthy(&self) -> bool {
        self.assess_health()
    }

    fn describe(&self) -> String {
        format!("{}", self)
    }
}

impl ArchetypeComplex for SpiritComplex {
    fn get_complex_lambda(&self) -> Float {
        self.calculate_spirit_capacity()
    }

    fn is_healthy(&self) -> bool {
        self.assess_health()
    }

    fn describe(&self) -> String {
        format!("{}", self)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mind_complex_creation() {
        let mind = MindComplex::new();
        assert_eq!(mind.matrix.archetype_id(), 1);
        assert_eq!(mind.potentiator.archetype_id(), 2);
        assert_eq!(mind.catalyst.archetype_id(), 3);
        assert_eq!(mind.experience.archetype_id(), 4);
        assert_eq!(mind.significator.archetype_id(), 5);
        assert_eq!(mind.transformation.archetype_id(), 6);
        assert_eq!(mind.great_way.archetype_id(), 7);
    }

    #[test]
    fn test_body_complex_creation() {
        let body = BodyComplex::new();
        assert_eq!(body.matrix.archetype_id(), 8);
        assert_eq!(body.potentiator.archetype_id(), 9);
        assert_eq!(body.catalyst.archetype_id(), 10);
        assert_eq!(body.experience.archetype_id(), 11);
        assert_eq!(body.significator.archetype_id(), 12);
        assert_eq!(body.transformation.archetype_id(), 13);
        assert_eq!(body.great_way.archetype_id(), 14);
    }

    #[test]
    fn test_spirit_complex_creation() {
        let spirit = SpiritComplex::new();
        assert_eq!(spirit.matrix.archetype_id(), 15);
        assert_eq!(spirit.potentiator.archetype_id(), 16);
        assert_eq!(spirit.catalyst.archetype_id(), 17);
        assert_eq!(spirit.experience.archetype_id(), 18);
        assert_eq!(spirit.significator.archetype_id(), 19);
        assert_eq!(spirit.transformation.archetype_id(), 20);
        assert_eq!(spirit.great_way.archetype_id(), 21);
    }

    #[test]
    fn test_mind_capacity_calculation() {
        let mind = MindComplex::new();
        let capacity = mind.calculate_mind_capacity();
        assert!((0.0..=1.0).contains(&capacity));
    }

    #[test]
    fn test_body_capacity_calculation() {
        let body = BodyComplex::new();
        let capacity = body.calculate_body_capacity();
        assert!((0.0..=1.0).contains(&capacity));
    }

    #[test]
    fn test_spirit_capacity_calculation() {
        let spirit = SpiritComplex::new();
        let capacity = spirit.calculate_spirit_capacity();
        assert!((0.0..=1.0).contains(&capacity));
    }

    #[test]
    fn test_all_complexes_healthy() {
        let mind = MindComplex::new();
        let body = BodyComplex::new();
        let spirit = SpiritComplex::new();

        assert!(mind.assess_health());
        assert!(body.assess_health());
        assert!(spirit.assess_health());
    }

    #[test]
    fn test_archetype_complex_trait() {
        let mind: Box<dyn ArchetypeComplex> = Box::new(MindComplex::new());
        let body: Box<dyn ArchetypeComplex> = Box::new(BodyComplex::new());
        let spirit: Box<dyn ArchetypeComplex> = Box::new(SpiritComplex::new());

        assert!(mind.is_healthy());
        assert!(body.is_healthy());
        assert!(spirit.is_healthy());

        assert!(mind.get_complex_lambda() >= 0.0);
        assert!(body.get_complex_lambda() >= 0.0);
        assert!(spirit.get_complex_lambda() >= 0.0);
    }
}
