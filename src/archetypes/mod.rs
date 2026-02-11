// Archetype System Module
//
// This module contains the complete implementation of the 22 Logoic Archetypes
// from the Metaphysics framework. Each archetype is implemented in a separate
// file following its complete architecture from the Metaphysics documentation.
//
// The 22 archetypes provide the computational architecture for consciousness
// development, evolutionary dynamics, and the transcend-and-include mechanism.
//
// Documentation Reference: 01_Metaphysics/archetypes/00_OVERVIEW.md

pub mod common;

// Phase 10: Archetype Traits for Polymorphism
pub mod archetype_traits;

// Phase 5: Archetype Complexes (Mind/Body/Spirit)
pub mod complexes;

// Phase 5: Archetype Cycles (Lesser & Greater)
pub mod cycles;

// Phase 2.1: Archetypes as Computational Operations
pub mod computational_operations;

// Phase 2.1: Individual Archetype Operations
pub mod catalyst_operation;
pub mod experience_operation;
pub mod matrix_operation;
pub mod potentiator_operation;

// Phase 2.1: Computational Pipeline Demonstrations
pub mod lesser_cycle_pipeline;

// Mind Complex Archetypes (A1-A7)
pub mod a1_mind_matrix;
pub mod a2_mind_potentiator;
pub mod a3_mind_catalyst;
pub mod a4_mind_experience;
pub mod a5_mind_significator;
pub mod a6_mind_transformation;
pub mod a7_mind_great_way;

// Body Complex Archetypes (A8-A14)
pub mod a10_body_catalyst;
pub mod a11_body_experience;
pub mod a12_body_significator;
pub mod a13_body_transformation;
pub mod a14_body_great_way;
pub mod a8_body_matrix;
pub mod a9_body_potentiator;

// Spirit Complex Archetypes (A15-A21)
pub mod a15_spirit_matrix;
pub mod a16_spirit_potentiator;
pub mod a17_spirit_catalyst;
pub mod a18_spirit_experience;
pub mod a19_spirit_significator;
pub mod a20_spirit_transformation;
pub mod a21_spirit_great_way;

// Unified Archetype (A22)
pub mod a22_choice;

// Cycle Integration
// cycles.rs to be implemented

// Holonic Architecture
// holonic.rs to be implemented

// Re-exports for convenience
pub use crate::types::Polarity;
pub use common::{
    ArchetypeComplex, ArchetypeRole, ArchetypeTrait, Developmental, FunctionalPair,
    LambdaMeasurement, LambdaMeasurementType, SigmaAxis, TarotCorrelation,
};

// Re-export ExperienceMetadata from memory module
pub use crate::memory::holographic_memory::ExperienceMetadata;

pub use complexes::ArchetypeComplex as ComplexTrait;

// Cycles types are re-exported for library use (entity_state.rs, evolution_chain.rs, etc.)
// They generate unused import warnings in the binary but are required by library modules.
pub use cycles::{
    ArchetypeChoice, ChoiceContext, ComplexType, GreaterCycle, GreaterCycleResult, LesserCycle,
    LesserCycleResult,
};

// Phase 2.1: Computational Operations
pub use computational_operations::ArchetypeOperation;

pub use a10_body_catalyst::CatalystBodyArchetype;
pub use a11_body_experience::ExperienceBodyArchetype;
pub use a12_body_significator::SignificatorBodyArchetype;
pub use a13_body_transformation::TransformationBodyArchetype;
pub use a14_body_great_way::GreatWayBodyArchetype;
pub use a15_spirit_matrix::MatrixSpiritArchetype;
pub use a16_spirit_potentiator::PotentiatorSpiritArchetype;
pub use a17_spirit_catalyst::CatalystSpiritArchetype;
pub use a18_spirit_experience::ExperienceSpiritArchetype;
pub use a19_spirit_significator::SignificatorSpiritArchetype;
pub use a1_mind_matrix::MatrixMindArchetype;
pub use a20_spirit_transformation::TransformationSpiritArchetype;
pub use a21_spirit_great_way::GreatWaySpiritArchetype;
pub use a22_choice::ChoiceArchetype;
pub use a2_mind_potentiator::PotentiatorMindArchetype;
pub use a3_mind_catalyst::CatalystMindArchetype;
pub use a4_mind_experience::ExperienceMindArchetype;
pub use a5_mind_significator::SignificatorMindArchetype;
pub use a6_mind_transformation::TransformationMindArchetype;
pub use a7_mind_great_way::GreatWayMindArchetype;
pub use a8_body_matrix::MatrixBodyArchetype;
pub use a9_body_potentiator::PotentiatorBodyArchetype;

// Type aliases for backward compatibility with complex.rs
pub type MatrixArchetype = MatrixMindArchetype;
pub type PotentiatorArchetype = PotentiatorMindArchetype;
pub type CatalystArchetype = CatalystMindArchetype;
pub type ExperienceArchetype = ExperienceMindArchetype;
pub type SignificatorArchetype = SignificatorMindArchetype;
pub type TransformationArchetype = TransformationMindArchetype;
pub type GreatWayArchetype = GreatWayMindArchetype;

/// Archetype System
///
/// The complete 22-archetype system provides the computational architecture
/// for consciousness development.
///
/// Structure:
/// - Mind Complex (A1-A7): σA - Mind Capacity
/// - Body Complex (A8-A14): σB - Body Capacity
/// - Spirit Complex (A15-A21): σC - Spirit Capacity
/// - Unified Archetype (A22): Choice - Zero-point of polarization
///
/// Cycles:
/// - Lesser Cycle (A1→A2→A3→A4): Processing Engine
/// - Greater Cycle (A5→A22→A6→A7): Evolutionary Engine
///
/// Functional Pairs:
/// - Structure Pair: Matrix ↔ Potentiator
/// - Process Pair: Catalyst ↔ Experience
/// - Identity Pair: Significator ↔ Great Way
/// - Transformation Singleton: Transformation
#[derive(Debug, Clone)]
pub struct ArchetypeSystem {
    // Mind Complex (A1-A7)
    pub matrix: MatrixMindArchetype, // A1: The Matrix of Mind
    pub potentiator: PotentiatorMindArchetype, // A2: The Potentiator of Mind
    pub catalyst: CatalystMindArchetype, // A3: The Catalyst of Mind
    pub experience: ExperienceMindArchetype, // A4: The Experience of Mind
    pub significator: SignificatorMindArchetype, // A5: The Significator of Mind
    pub transformation: TransformationMindArchetype, // A6: The Transformation of Mind
    pub great_way: GreatWayMindArchetype, // A7: The Great Way of Mind

    // Body Complex (A8-A14)
    pub body_matrix: MatrixBodyArchetype, // A8: The Matrix of Body
    pub body_potentiator: PotentiatorBodyArchetype, // A9: The Potentiator of Body
    pub body_catalyst: CatalystBodyArchetype, // A10: The Catalyst of Body
    pub body_experience: ExperienceBodyArchetype, // A11: The Experience of Body
    pub body_significator: SignificatorBodyArchetype, // A12: The Significator of Body
    pub body_transformation: TransformationBodyArchetype, // A13: The Transformation of Body
    pub body_great_way: GreatWayBodyArchetype, // A14: The Great Way of Body

    // Spirit Complex (A15-A21)
    pub spirit_matrix: MatrixSpiritArchetype, // A15: The Matrix of Spirit
    pub spirit_potentiator: PotentiatorSpiritArchetype, // A16: The Potentiator of Spirit
    pub spirit_catalyst: CatalystSpiritArchetype, // A17: The Catalyst of Spirit
    pub spirit_experience: ExperienceSpiritArchetype, // A18: The Experience of Spirit
    pub spirit_significator: SignificatorSpiritArchetype, // A19: The Significator of Spirit
    pub spirit_transformation: TransformationSpiritArchetype, // A20: The Transformation of Spirit
    pub spirit_great_way: GreatWaySpiritArchetype, // A21: The Great Way of Spirit

    // Unified Archetype (A22)
    pub choice: ChoiceArchetype, // A22: The Choice - Unifying Archetype
}

impl ArchetypeSystem {
    /// Create a new archetype system with initialized mind complex archetypes
    pub fn new() -> Self {
        Self {
            matrix: MatrixMindArchetype::new(),
            potentiator: PotentiatorMindArchetype::new(),
            catalyst: CatalystMindArchetype::new(),
            experience: ExperienceMindArchetype::new(),
            significator: SignificatorMindArchetype::new(),
            transformation: TransformationMindArchetype::new(),
            great_way: GreatWayMindArchetype::new(),
            body_matrix: MatrixBodyArchetype::new(),
            body_potentiator: PotentiatorBodyArchetype::new(),
            body_catalyst: CatalystBodyArchetype::new(),
            body_experience: ExperienceBodyArchetype::new(),
            body_significator: SignificatorBodyArchetype::new(),
            body_transformation: TransformationBodyArchetype::new(),
            body_great_way: GreatWayBodyArchetype::new(),
            spirit_matrix: MatrixSpiritArchetype::new(),
            spirit_potentiator: PotentiatorSpiritArchetype::new(),
            spirit_catalyst: CatalystSpiritArchetype::new(),
            spirit_experience: ExperienceSpiritArchetype::new(),
            spirit_significator: SignificatorSpiritArchetype::new(),
            spirit_transformation: TransformationSpiritArchetype::new(),
            spirit_great_way: GreatWaySpiritArchetype::new(),
            choice: ChoiceArchetype::new(),
        }
    }

    /// Run the Lesser Cycle (A1→A2→A3→A4)
    ///
    /// The Lesser Cycle is the Processing Engine:
    /// - Matrix (A1) reaches into Potentiator (A2) for resources
    /// - Catalyst (A3) acts upon conscious mind
    /// - Experience (A4) stores processed catalyst in unconscious
    pub fn run_lesser_cycle(&mut self, catalyst_input: Float) -> Float {
        // Step 1: Matrix reaches into Potentiator for resources
        let _matrix_rigidity = self.matrix.lambda().value;
        let potentiator_accessibility = self.potentiator.lambda().value;

        // Step 2: Process catalyst through Catalyst archetype
        self.catalyst
            .process(catalyst_input, self.catalyst.developmental_position());

        // Step 3: Process experience through Experience archetype
        self.experience
            .process(catalyst_input, self.experience.developmental_position());

        // Return processed experience (simplified for now)
        catalyst_input * potentiator_accessibility
    }

    /// Calculate Lesser Cycle efficiency
    ///
    /// Returns (Microcosmic Tension, Processing Efficiency)
    pub fn calculate_lesser_cycle_efficiency(&self) -> (Float, Float) {
        // Microcosmic Tension = |A1 - A2|
        let microcosmic_tension =
            (self.matrix.lambda().value - self.potentiator.lambda().value).abs();

        // Processing Efficiency = A4 / A3, clamped to [0, 1]
        let processing_efficiency = if self.catalyst.lambda().value > 0.0 {
            (self.experience.lambda().value / self.catalyst.lambda().value).min(1.0)
        } else {
            0.0
        };

        (microcosmic_tension, processing_efficiency)
    }

    /// Assess overall system health
    pub fn assess_system_health(&self) -> bool {
        // All archetypes must be healthy
        self.matrix.is_healthy()
            && self.potentiator.is_healthy()
            && self.catalyst.is_healthy()
            && self.experience.is_healthy()
            && self.significator.is_healthy()
            && self.transformation.is_healthy()
            && self.great_way.is_healthy()
            && self.body_matrix.is_healthy()
            && self.body_potentiator.is_healthy()
            && self.body_catalyst.is_healthy()
            && self.body_experience.is_healthy()
            && self.body_significator.is_healthy()
            && self.body_transformation.is_healthy()
            && self.body_great_way.is_healthy()
    }

    /// Get system health report
    pub fn get_health_report(&self) -> String {
        format!(
            "Archetype System Health Report:\n\
            Mind Complex (σA):\n\
            - Matrix (A1): {}\n\
            - Potentiator (A2): {}\n\
            - Catalyst (A3): {}\n\
            - Experience (A4): {}\n\
            - Significator (A5): {}\n\
            - Transformation (A6): {}\n\
            - Great Way (A7): {}\n\
            \n\
Body Complex (σB):\n\
             - Matrix of Body (A8): {}\n\
             - Potentiator of Body (A9): {}\n\
             - Catalyst of Body (A10): {}\n\
             - Experience of Body (A11): {}\n\
             - Significator of Body (A12): {}\n\
             - Transformation of Body (A13): {}\n\
             - Great Way of Body (A14): {}\n\
             \n\
             Spirit Complex (σC):\n\
             - Matrix of Spirit (A15): {}\n\
             - Potentiator of Spirit (A16): {}\n\
             - Catalyst of Spirit (A17): {}\n\
             - Experience of Spirit (A18): {}\n\
             - Significator of Spirit (A19): {}\n\
             \n\
            Lesser Cycle Efficiency:\n\
            - Microcosmic Tension: {:.3}\n\
            - Processing Efficiency: {:.3}",
            self.matrix.health_status(),
            self.potentiator.health_status(),
            self.catalyst.health_status(),
            self.experience.health_status(),
            self.significator.health_status(),
            self.transformation.health_status(),
            self.great_way.health_status(),
            self.body_matrix.health_status(),
            self.body_potentiator.health_status(),
            self.body_catalyst.health_status(),
            self.body_experience.health_status(),
            self.body_significator.health_status(),
            self.body_transformation.health_status(),
            self.body_great_way.health_status(),
            self.spirit_matrix.health_status(),
            self.spirit_potentiator.health_status(),
            self.spirit_catalyst.health_status(),
            self.spirit_experience.health_status(),
            self.spirit_significator.health_status(),
            self.calculate_lesser_cycle_efficiency().0,
            self.calculate_lesser_cycle_efficiency().1,
        )
    }

    /// Get all archetypes as a collection of references
    ///
    /// Returns a vector of boxed trait objects for all 22 archetypes
    /// This allows iteration over all archetypes for operations like
    /// archetype resonance calculation or attractor pattern matching
    pub fn get_all_archetypes(&self) -> Vec<Box<dyn ArchetypeTrait>> {
        vec![
            Box::new(self.matrix.clone()),
            Box::new(self.potentiator.clone()),
            Box::new(self.catalyst.clone()),
            Box::new(self.experience.clone()),
            Box::new(self.significator.clone()),
            Box::new(self.transformation.clone()),
            Box::new(self.great_way.clone()),
            Box::new(self.body_matrix.clone()),
            Box::new(self.body_potentiator.clone()),
            Box::new(self.body_catalyst.clone()),
            Box::new(self.body_experience.clone()),
            Box::new(self.body_significator.clone()),
            Box::new(self.body_transformation.clone()),
            Box::new(self.body_great_way.clone()),
            Box::new(self.spirit_matrix.clone()),
            Box::new(self.spirit_potentiator.clone()),
            Box::new(self.spirit_catalyst.clone()),
            Box::new(self.spirit_experience.clone()),
            Box::new(self.spirit_significator.clone()),
            Box::new(self.spirit_transformation.clone()),
            Box::new(self.spirit_great_way.clone()),
            Box::new(self.choice.clone()),
        ]
    }
}

// Import Float type
use crate::types::Float;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_system_creation() {
        let system = ArchetypeSystem::new();
        assert_eq!(system.matrix.archetype_id(), 1);
        assert_eq!(system.potentiator.archetype_id(), 2);
        assert_eq!(system.catalyst.archetype_id(), 3);
        assert_eq!(system.experience.archetype_id(), 4);
        assert_eq!(system.significator.archetype_id(), 5);
        assert_eq!(system.transformation.archetype_id(), 6);
        assert_eq!(system.great_way.archetype_id(), 7);
        assert_eq!(system.body_matrix.archetype_id(), 8);
        assert_eq!(system.body_potentiator.archetype_id(), 9);
        assert_eq!(system.body_catalyst.archetype_id(), 10);
        assert_eq!(system.body_experience.archetype_id(), 11);
        assert_eq!(system.body_significator.archetype_id(), 12);
        assert_eq!(system.body_transformation.archetype_id(), 13);
        assert_eq!(system.body_great_way.archetype_id(), 14);
    }

    #[test]
    fn test_lesser_cycle() {
        let mut system = ArchetypeSystem::new();
        let catalyst_input = 0.7;

        let result = system.run_lesser_cycle(catalyst_input);

        // Result should be processed experience
        assert!(result > 0.0);
        assert!(result <= 1.0);
    }

    #[test]
    fn test_lesser_cycle_efficiency() {
        let mut system = ArchetypeSystem::new();
        system.run_lesser_cycle(0.7);

        let (tension, efficiency) = system.calculate_lesser_cycle_efficiency();

        // Both values should be in valid ranges
        assert!(tension >= 0.0 && tension <= 1.0);
        assert!(efficiency >= 0.0 && efficiency <= 1.0);
    }

    #[test]
    fn test_system_health() {
        let system = ArchetypeSystem::new();

        // All archetypes should start healthy
        assert!(system.assess_system_health());
    }
}
