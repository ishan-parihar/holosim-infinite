// Consciousness Module
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "The Entity's Free Will is NOT an emergent property—it is the KERNEL of the simulation,
// inherited from Layer 1 (Indigo)."
//
// This module implements:
// 1. Free Will as kernel of entity consciousness
// 2. Archetype 22 as the choice operator
// 3. Possibility space generation
// 4. Non-deterministic selection from possibility space
// 5. Unified ConsciousnessKernel (Phase 5.1)

pub mod archetype22;
pub mod choice_operator;
pub mod free_will;
pub mod kernel;
pub mod memory_access;
pub mod possibility_space;

// Re-export main types for convenience
pub use free_will::{ChoiceRecord, ConsciousSelection, FreeWillKernel};

// Re-export unified ConsciousnessKernel types (Phase 5.1)
pub use kernel::{
    ConsciousSelection as KernelConsciousSelection, ConsciousnessAmplification,
    ConsciousnessKernel, ConsciousnessSignal, Experience, KernelChoiceContext, Memory, MemoryQuery,
    Possibility,
};

// Re-export Memory as Spectrum Access types (Phase 5.3)
pub use memory_access::{
    MemoryAccessRecord, MemoryAccessResult, MemoryStorageResult, MemorySystem,
    MemorySystemStatistics, MemoryType, SpectrumExperience, SpectrumMemory, SpectrumMemoryQuery,
};

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::consciousness::archetype22::Archetype22;
    use crate::consciousness::choice_operator::ChoiceContext;
    use crate::consciousness::choice_operator::{ChoiceOperator, PolarityPreference};
    use crate::consciousness::possibility_space::PossibilitySpace;
    use crate::entity_layer7::layer7::{PolarityState as EntityPolarityState, VibrationalState};
    use crate::foundation::indigo_realm::IntelligentInfinity;

    fn create_test_entity_state(
        consciousness_level: f64,
    ) -> crate::entity_layer7::layer7::EntityState {
        use crate::evolution_density_octave::density_octave::{Density, Density1SubLevel};
        crate::entity_layer7::layer7::EntityState {
            vibrational_state: VibrationalState {
                frequency: 0.5,
                amplitude: 0.5,
                coherence: 0.7,
                density: Density::First(Density1SubLevel::Quantum),
                potential_energy: 0.5,
                kinetic_energy: 0.5,
            },
            polarity_state: EntityPolarityState {
                polarity_bias: 0.3,
                polarization_strength: 0.5,
            },
            consciousness_level,
            experience_accumulation: 0.0,
            learning_progress: 0.0,
        }
    }

    #[test]
    fn test_free_will_kernel_basic_functionality() {
        // Create Archetype 22 from IntelligentInfinity
        let _intelligent_infinity = IntelligentInfinity::new();
        let foundation_archetype22 = crate::foundation::indigo_realm::Archetype22::new();

        // Create Free Will kernel with Archetype 22
        let mut free_will_kernel = FreeWillKernel::new(foundation_archetype22);

        // Create entity state
        let entity_state = create_test_entity_state(0.5);

        // Create choice context (using free_will::ChoiceContext)
        let context = crate::consciousness::free_will::ChoiceContext {
            polarity_preference: crate::consciousness::free_will::PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Exercise Free Will
        let result = free_will_kernel.exercise_free_will(&entity_state, &context, 0.5, 0.5);

        // Verify the result
        assert!(!result.possibility_space.possibilities.is_empty());
        assert!(result.choice.confidence >= 0.0);
        assert!(result.choice.confidence <= 1.0);
        assert!(!free_will_kernel.choice_history.is_empty());
    }

    #[test]
    fn test_archetype22_possibility_space_integration() {
        // Create Archetype 22
        let intelligent_infinity = IntelligentInfinity::new();
        let mut archetype22 = Archetype22::new(intelligent_infinity);

        // Create entity state
        let entity_state = create_test_entity_state(0.5);

        // Generate possibility space through Archetype 22
        let possibility_space = archetype22.generate_possibility_space(&entity_state);

        // Verify the possibility space
        assert!(!possibility_space.possibilities.is_empty());

        // Make a choice
        let choice = archetype22.make_choice(&possibility_space, &entity_state);

        // Verify the choice
        assert!(choice.selected_index < possibility_space.possibilities.len());
        assert!(choice.intensity >= 0.0);
        assert!(choice.intensity <= 1.0);
    }

    #[test]
    fn test_possibility_space_choice_operator_integration() {
        // Create entity state
        let entity_state = create_test_entity_state(0.5);

        // Generate possibility space
        let possibility_space = PossibilitySpace::from_entity_state(&entity_state);

        // Create choice operator
        let mut choice_operator = ChoiceOperator::new();

        // Create choice context
        let context = crate::consciousness::choice_operator::ChoiceContext {
            polarity_preference: crate::consciousness::choice_operator::PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
            consciousness_level: 0.5,
        };

        // Make a choice
        let result = choice_operator.make_choice(&possibility_space, &entity_state, &context);

        // Verify the result
        assert!(result.confidence >= 0.0);
        assert!(result.confidence <= 1.0);
        assert!(result.sto_alignment >= -1.0);
        assert!(result.sto_alignment <= 1.0);
    }

    #[test]
    fn test_complete_consciousness_flow() {
        // 1. Create IntelligentInfinity (Layer 1)
        let _intelligent_infinity = IntelligentInfinity::new();

        // 2. Create Archetype 22 from IntelligentInfinity
        let foundation_archetype22 = crate::foundation::indigo_realm::Archetype22::new();

        // 3. Create Free Will kernel with Archetype 22
        let mut free_will_kernel = FreeWillKernel::new(foundation_archetype22);

        // 4. Create entity state (Layer 7)
        let entity_state = create_test_entity_state(0.5);

        // 5. Create choice context
        let context = crate::consciousness::free_will::ChoiceContext {
            polarity_preference: crate::consciousness::free_will::PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // 6. Exercise Free Will
        let result = free_will_kernel.exercise_free_will(&entity_state, &context, 0.5, 0.5);

        // 7. Verify complete flow
        assert!(!result.possibility_space.possibilities.is_empty());
        assert!(result.effectiveness > 0.0);

        // 8. Check that consciousness grows through choice
        let initial_consciousness = free_will_kernel.consciousness_level;
        for _ in 0..5 {
            free_will_kernel.exercise_free_will(&entity_state, &context, 0.5, 0.5);
        }
        assert!(free_will_kernel.consciousness_level > initial_consciousness);
    }

    #[test]
    fn test_polarity_development_through_choices() {
        // Create Archetype 22
        let intelligent_infinity = IntelligentInfinity::new();
        let mut archetype22 = Archetype22::new(intelligent_infinity);

        // Create entity state
        let entity_state = create_test_entity_state(0.5);

        // Make choices with STO preference
        for _ in 0..10 {
            let possibility_space = archetype22.generate_possibility_space(&entity_state);
            archetype22.make_choice(&possibility_space, &entity_state);
        }

        // Check polarity development
        let stats = archetype22.get_polarity_statistics();
        assert!(stats.total_choices > 0);
        assert!(stats.sto_potential > 0.0 || stats.sts_potential > 0.0);
    }

    #[test]
    fn test_non_deterministic_behavior_across_systems() {
        // Create entity state
        let entity_state = create_test_entity_state(0.5);

        // Test Free Will kernel non-determinism
        let _intelligent_infinity = IntelligentInfinity::new();
        let foundation_archetype22 = crate::foundation::indigo_realm::Archetype22::new();
        let mut free_will_kernel = FreeWillKernel::new(foundation_archetype22);

        let context = crate::consciousness::free_will::ChoiceContext {
            polarity_preference: crate::consciousness::free_will::PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        let mut free_will_selections = Vec::new();
        for _ in 0..20 {
            let result = free_will_kernel.exercise_free_will(&entity_state, &context, 0.5, 0.5);
            free_will_selections.push(result.choice.selected_index);
        }

        // Test Archetype 22 non-determinism
        let mut archetype22 = Archetype22::new(IntelligentInfinity::new());
        let mut archetype22_selections = Vec::new();
        for _ in 0..20 {
            let possibility_space = archetype22.generate_possibility_space(&entity_state);
            let choice = archetype22.make_choice(&possibility_space, &entity_state);
            archetype22_selections.push(choice.selected_index);
        }

        // Test Choice Operator non-determinism
        let possibility_space = PossibilitySpace::from_entity_state(&entity_state);
        let mut choice_operator = ChoiceOperator::new();
        let choice_context = ChoiceContext {
            polarity_preference: PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
            consciousness_level: 0.5,
        };

        let mut choice_operator_selections = Vec::new();
        for _ in 0..20 {
            let result =
                choice_operator.make_choice(&possibility_space, &entity_state, &choice_context);
            choice_operator_selections.push(result.record.selected_index);
        }

        // All systems should show non-deterministic behavior
        let free_will_unique: std::collections::HashSet<_> =
            free_will_selections.into_iter().collect();
        let archetype22_unique: std::collections::HashSet<_> =
            archetype22_selections.into_iter().collect();
        let choice_operator_unique: std::collections::HashSet<_> =
            choice_operator_selections.into_iter().collect();

        assert!(
            free_will_unique.len() > 1,
            "Free Will should be non-deterministic"
        );
        assert!(
            archetype22_unique.len() > 1,
            "Archetype 22 should be non-deterministic"
        );
        assert!(
            choice_operator_unique.len() > 1,
            "Choice Operator should be non-deterministic"
        );
    }

    #[test]
    fn test_consciousness_growth_through_free_will() {
        // Create Free Will kernel
        let _intelligent_infinity = IntelligentInfinity::new();
        let foundation_archetype22 = crate::foundation::indigo_realm::Archetype22::new();
        let mut free_will_kernel = FreeWillKernel::new(foundation_archetype22);

        // Create entity state
        let entity_state = create_test_entity_state(0.5);

        // Create choice context
        let context = crate::consciousness::free_will::ChoiceContext {
            polarity_preference: crate::consciousness::free_will::PolarityPreference::Neutral,
            environmental_constraints: Vec::new(),
            experience_bias: 0.0,
        };

        // Get initial statistics
        let initial_stats = free_will_kernel.get_statistics();

        // Make choices
        for _ in 0..10 {
            free_will_kernel.exercise_free_will(&entity_state, &context, 0.5, 0.5);
        }

        // Get final statistics
        let final_stats = free_will_kernel.get_statistics();

        // Consciousness should have grown
        assert!(final_stats.consciousness_level > initial_stats.consciousness_level);
        assert!(final_stats.activation_level > initial_stats.activation_level);
    }
}
