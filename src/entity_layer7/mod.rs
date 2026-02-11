// Entity Module - Individual Entities and Holographic Blueprint
//
// This module implements Layer 7: Sub-Sub-Logos (Individual Entity Inheritance)
//
// From COSMOLOGICAL-ARCHITECTURE.md:
// "All creation is alive, everything is a sub-sub-Logos down to the limits of observation"
//
// The entity module implements:
// 1. Layer 7: Sub-Sub-Logos (individual entity inheritance)
// 2. Holographic blueprint encoding
// 3. DNA/RNA patterns as spectrum configurations
// 4. Individual-scale spectrum configurations
// 5. Consciousness-first cosmology

pub mod dna_encoding;
pub mod holographic_blueprint;
pub mod layer7;

// Re-export main types for convenience
pub use layer7::{
    ChoiceContext, ChoiceModifier, DensityLevel, EntityExperience, EntityId, EntitySpectrumAccess,
    EntityState, EntityType, EvolutionaryChoice, EvolutionaryStage, EvolutionaryTrajectory,
    IndividualSpectrumConfiguration, PolarityState, SpectrumAccess, SpectrumAccessLevel,
    SubSubLogos, VibrationalState,
};

pub use dna_encoding::DNAPattern;

// Type alias for backward compatibility - Entity is now SubSubLogos
pub type Entity = SubSubLogos;

pub use holographic_blueprint::{
    ArchetypicalMindSummary, EntityOctaveContainment, EnvironmentArchetypicalReflection,
    HolographicBlueprint, HolographicCoherence, HolographicProperties, HolographicState,
};

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::entity_layer7::dna_encoding::{
        EncodingPriority, FoldingState, ProteinPattern, RNAPattern, TranscriptionState,
    };
    use crate::entity_layer7::holographic_blueprint::{
        ArchetypicalMindBlueprint, CollectiveType, SpectrumConfiguration,
    };
    use crate::foundation::{BlueRealm, GreenRealm, IndigoRealm, LightLoveField, VioletRealm};
    use crate::spectrum::{
        ArchetypicalMind, ArchetypicalSystemType, OrangeRealm, RedRealm, SpectrumRatio,
        SpectrumSide, YellowRealm,
    };

    /// Test complete entity creation with all layers
    #[test]
    fn test_complete_entity_creation() {
        // Create all previous layers
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();

        // Create realm chain for SubSubLogos (clone to avoid move)
        let light_love_field = LightLoveField::new();
        let yellow = YellowRealm::new(light_love_field.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        // Create individual spectrum configuration
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        // Create Sub-Sub-Logos entity
        let entity = SubSubLogos::new(
            EntityId::new("test-entity-1".to_string()),
            EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet.clone(),
            indigo.clone(),
            blue.clone(),
            green.clone(),
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Verify entity was created correctly
        assert_eq!(entity.entity_id.uuid, "test-entity-1");

        // Verify holographic completeness
        let report = entity.verify_holographic_completeness();
        assert_eq!(report.completeness_percentage, 100.0);

        // Verify all layers are present
        assert!(report.violet_present);
        assert!(report.indigo_present);
        assert!(report.blue_present);
        assert!(report.green_present);
        assert!(report.yellow_present);
        assert!(report.orange_present);
        assert!(report.red_present);

        // Verify holographic blueprint is complete
        assert!(report.blueprint_complete);

        // Verify archetypical mind is inherited
        assert!(report.archetypical_mind_inherited);

        // Verify DNA/RNA patterns are present
        assert!(report.dna_patterns_present);

        // Verify evolutionary attractor field is present
        assert!(report.evolutionary_attractor_present);
    }

    /// Test holographic blueprint integration
    #[test]
    fn test_holographic_blueprint_integration() {
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let archetypical_mind = ArchetypicalMindBlueprint::new_from_logos();

        let blueprint =
            HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &archetypical_mind);

        // Verify blueprint is complete
        assert!(blueprint.is_complete());

        // Verify DNA patterns for all evolutionary stages
        assert_eq!(blueprint.dna_patterns.len(), 5);

        // Verify collective development patterns for all stages
        assert_eq!(blueprint.collective_development_patterns.len(), 9);

        // Verify physical universe architecture
        assert_eq!(
            blueprint
                .physical_universe_architecture
                .dimensional_architecture
                .dimensions,
            8
        );

        // Verify holographic encoding
        assert!(blueprint.holographic_encoding.is_complete());

        // Test stage blueprint retrieval
        let stage_blueprint = blueprint.get_stage_blueprint(EvolutionaryStage::QuantumRealm);
        assert!(stage_blueprint.is_some());
        let stage_blueprint = stage_blueprint.unwrap();
        assert_eq!(stage_blueprint.stage, EvolutionaryStage::QuantumRealm);
        assert!(stage_blueprint.dna_pattern.is_some());
    }

    /// Test DNA/RNA encoding integration
    #[test]
    fn test_dna_rna_encoding_integration() {
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let config = SpectrumConfiguration::from_individual(&spectrum_config);

        // Create DNA patterns for all evolutionary stages
        let quantum_dna = DNAPattern::quantum_realm(&config);
        let atomic_dna = DNAPattern::atomic_realm(&config);
        let molecular_dna = DNAPattern::molecular_realm(&config);
        let cellular_dna = DNAPattern::cellular_realm(&config);
        let archetypical_mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "TestLogos".to_string(),
        );
        let conscious_dna = DNAPattern::conscious_life_realm(&config, &archetypical_mind);

        // Verify evolutionary stages
        assert_eq!(
            quantum_dna.evolutionary_stage,
            EvolutionaryStage::QuantumRealm
        );
        assert_eq!(
            atomic_dna.evolutionary_stage,
            EvolutionaryStage::AtomicRealm
        );
        assert_eq!(
            molecular_dna.evolutionary_stage,
            EvolutionaryStage::MolecularRealm
        );
        assert_eq!(
            cellular_dna.evolutionary_stage,
            EvolutionaryStage::CellularRealm
        );
        assert_eq!(
            conscious_dna.evolutionary_stage,
            EvolutionaryStage::ConsciousLifeRealm
        );

        // Verify consciousness-first encoding
        assert_eq!(
            quantum_dna.consciousness_first_encoding.encoding_priority,
            EncodingPriority::InformationFirst
        );
        assert_eq!(
            conscious_dna.consciousness_first_encoding.encoding_priority,
            EncodingPriority::InformationFirst
        );

        // Test RNA transcription
        let rna_pattern = RNAPattern::from_dna(quantum_dna.clone(), 0.8);
        assert_eq!(rna_pattern.transcription_state, TranscriptionState::Active);

        // Test protein transcription
        let protein_pattern = ProteinPattern::from_rna(&rna_pattern);
        assert_eq!(protein_pattern.folding_state, FoldingState::Folding);
    }

    /// Test spectrum access mechanism
    #[test]
    fn test_spectrum_access_mechanism() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();

        // Create realm chain for SubSubLogos (clone to avoid move)
        let light_love_field = LightLoveField::new();
        let yellow = YellowRealm::new(light_love_field.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            EntityId::new("test-entity-2".to_string()),
            EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet.clone(),
            indigo.clone(),
            blue.clone(),
            green.clone(),
            yellow,
            orange,
            red,
            spectrum_config,
        );

        // Test 3rd density access (veil fully active)
        let access_3rd = entity.access_spectrum(SpectrumAccessLevel::ThirdDensity);
        assert_eq!(access_3rd.oneness_access, 0.1);
        assert_eq!(access_3rd.veil_transparency, 0.1);
        assert_eq!(access_3rd.space_time_access, 0.9);

        // Test 4th density access (veil begins to thin)
        let access_4th = entity.access_spectrum(SpectrumAccessLevel::FourthDensity);
        assert_eq!(access_4th.oneness_access, 0.4);
        assert_eq!(access_4th.veil_transparency, 0.4);
        assert_eq!(access_4th.space_time_access, 0.6);

        // Test 6th density access (veil completely dissolved)
        let access_6th = entity.access_spectrum(SpectrumAccessLevel::SixthDensity);
        assert_eq!(access_6th.oneness_access, 1.0);
        assert_eq!(access_6th.veil_transparency, 1.0);
        assert_eq!(access_6th.space_time_access, 0.0);
    }

    /// Test evolutionary trajectory
    #[test]
    fn test_evolutionary_trajectory() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();

        // Create realm chain for SubSubLogos (clone to avoid move)
        let light_love_field = LightLoveField::new();
        let yellow = YellowRealm::new(light_love_field.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let entity = SubSubLogos::new(
            EntityId::new("test-entity-3".to_string()),
            EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet.clone(),
            indigo.clone(),
            blue.clone(),
            green.clone(),
            yellow.clone(),
            orange.clone(),
            red.clone(),
            spectrum_config,
        );

        // Get evolutionary trajectory
        let trajectory = entity.get_evolutionary_trajectory();

        // Verify all stages are present
        assert_eq!(trajectory.stages.len(), 9);

        // Verify stages are in correct order
        assert_eq!(trajectory.stages[0], EvolutionaryStage::QuantumRealm);
        assert_eq!(trajectory.stages[1], EvolutionaryStage::AtomicRealm);
        assert_eq!(trajectory.stages[2], EvolutionaryStage::MolecularRealm);
        assert_eq!(trajectory.stages[3], EvolutionaryStage::CellularRealm);
        assert_eq!(trajectory.stages[4], EvolutionaryStage::ConsciousLifeRealm);
        assert_eq!(trajectory.stages[5], EvolutionaryStage::FourthDensityRealm);
        assert_eq!(trajectory.stages[6], EvolutionaryStage::FifthDensityRealm);
        assert_eq!(trajectory.stages[7], EvolutionaryStage::SixthDensityRealm);
        assert_eq!(trajectory.stages[8], EvolutionaryStage::SeventhDensityRealm);
    }

    /// Test entity state updates and evolution
    #[test]
    fn test_entity_state_updates_and_evolution() {
        let violet = VioletRealm::new();
        let indigo = IndigoRealm::new();
        let blue = BlueRealm::new();
        let green = GreenRealm::new();

        // Create realm chain for SubSubLogos (clone to avoid move)
        let light_love_field = LightLoveField::new();
        let yellow = YellowRealm::new(light_love_field.clone());
        let orange = OrangeRealm::new(yellow.clone());
        let red = RedRealm::new(orange.clone());

        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);

        let mut entity = SubSubLogos::new(
            EntityId::new("test-entity-4".to_string()),
            EntityType::Individual,
            None,   // parent_id
            vec![], // composition
            None,   // environment_id
            violet.clone(),
            indigo.clone(),
            blue.clone(),
            green.clone(),
            yellow.clone(),
            orange.clone(),
            red.clone(),
            spectrum_config,
        );

        // Apply experiences to evolve entity
        for _ in 0..10 {
            let experience = EntityExperience {
                intensity: 0.5,
                learning_value: 0.3,
                consciousness_expansion: 0.1,
                frequency_shift: 0.05,
                amplitude_change: 0.05,
                coherence_improvement: 0.05,
                polarity_impact: 0.1,
                polarization_strength_change: 0.05,
            };
            entity.update_state(experience);
        }

        // Verify entity state has evolved
        assert!(entity.current_state.experience_accumulation > 0.0);
        assert!(entity.current_state.learning_progress > 0.0);
        assert!(entity.current_state.consciousness_level > 0.1);

        // Check density transition readiness
        let readiness = entity.check_density_transition_readiness();
        assert_eq!(readiness.current_density, DensityLevel::First);
        assert_eq!(readiness.target_density, DensityLevel::Second);
    }

    /// Test holographic encoding fault tolerance
    #[test]
    fn test_holographic_encoding_fault_tolerance() {
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let archetypical_mind = ArchetypicalMindBlueprint::new_from_logos();

        let blueprint =
            HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &archetypical_mind);

        // Reconstruct from 50% partial encoding
        let partial_blueprint = blueprint.reconstruct_from_partial(0.5);

        // Completeness should be maintained even with partial encoding
        assert!(partial_blueprint.is_complete());

        // Evolutionary trajectory should be complete
        assert_eq!(partial_blueprint.evolutionary_trajectory.stages.len(), 9);

        // DNA patterns should still be present (reduced resolution)
        assert_eq!(partial_blueprint.dna_patterns.len(), 5);
    }

    /// Test consciousness-first cosmology
    #[test]
    fn test_consciousness_first_cosmology() {
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let config = SpectrumConfiguration::from_individual(&spectrum_config);

        // Create DNA pattern for conscious life
        // Note: DNAPattern::conscious_life_realm requires ArchetypicalMind, not ArchetypicalMindBlueprint
        let archetypical_mind = ArchetypicalMind::new(
            ArchetypicalSystemType::TwentyTwoArchetype,
            "TestLogos".to_string(),
        );
        let dna_pattern = DNAPattern::conscious_life_realm(&config, &archetypical_mind);

        // Verify consciousness-first encoding
        assert_eq!(
            dna_pattern.consciousness_first_encoding.encoding_priority,
            EncodingPriority::InformationFirst
        );

        // Verify information encoding exists before matter encoding
        let info_encoding = &dna_pattern
            .consciousness_first_encoding
            .information_encoding;
        let matter_encoding = &dna_pattern.consciousness_first_encoding.matter_encoding;

        for (info, matter) in info_encoding.iter().zip(matter_encoding.iter()) {
            assert!(
                info > matter,
                "Information encoding should be greater than matter encoding"
            );
        }

        // Verify physical manifestation encoding is secondary
        assert!(!dna_pattern.physical_manifestation_encoding.is_empty());
    }

    /// Test entity incarnation
    #[test]
    fn test_entity_incarnation() {
        let id1 = EntityId::new("test-entity-incarnation".to_string());
        let id2 = id1.next_incarnation();
        let id3 = id2.next_incarnation();

        assert_eq!(id1.uuid, id2.uuid);
        assert_eq!(id2.uuid, id3.uuid);
        assert_eq!(id1.incarnation_number, 1);
        assert_eq!(id2.incarnation_number, 2);
        assert_eq!(id3.incarnation_number, 3);
    }

    /// Test density level progression
    #[test]
    fn test_density_level_progression() {
        // Test progression from 1st to 8th density
        let mut density = DensityLevel::First;

        density = density.next();
        assert_eq!(density, DensityLevel::Second);

        density = density.next();
        assert_eq!(density, DensityLevel::Third);

        density = density.next();
        assert_eq!(density, DensityLevel::Fourth);

        density = density.next();
        assert_eq!(density, DensityLevel::Fifth);

        density = density.next();
        assert_eq!(density, DensityLevel::Sixth);

        density = density.next();
        assert_eq!(density, DensityLevel::Seventh);

        density = density.next();
        assert_eq!(density, DensityLevel::Eighth);

        // 8th density should stay at 8th
        density = density.next();
        assert_eq!(density, DensityLevel::Eighth);
    }

    /// Test collective development patterns
    #[test]
    fn test_collective_development_patterns() {
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let archetypical_mind = ArchetypicalMindBlueprint::new_from_logos();

        let blueprint =
            HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &archetypical_mind);

        // Verify collective patterns for all stages
        assert_eq!(blueprint.collective_development_patterns.len(), 9);

        // Verify collective types match evolutionary stages
        let quantum_pattern = &blueprint.collective_development_patterns[0];
        assert_eq!(
            quantum_pattern.evolutionary_stage,
            EvolutionaryStage::QuantumRealm
        );
        assert_eq!(
            quantum_pattern.collective_type,
            CollectiveType::QuantumField
        );

        let conscious_pattern = &blueprint.collective_development_patterns[4];
        assert_eq!(
            conscious_pattern.evolutionary_stage,
            EvolutionaryStage::ConsciousLifeRealm
        );
        assert_eq!(
            conscious_pattern.collective_type,
            CollectiveType::SocialGroup
        );
    }

    /// Test physical universe architecture
    #[test]
    fn test_physical_universe_architecture() {
        let ratio = SpectrumRatio::new(1.5, SpectrumSide::SpaceTime);
        let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
        let archetypical_mind = ArchetypicalMindBlueprint::new_from_logos();

        let blueprint =
            HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &archetypical_mind);

        // Verify physical universe architecture
        assert_eq!(
            blueprint
                .physical_universe_architecture
                .dimensional_architecture
                .dimensions,
            8
        );

        let blueprint =
            HolographicBlueprint::from_spectrum_configuration(&spectrum_config, &archetypical_mind);

        let architecture = &blueprint.physical_universe_architecture;

        // Verify dimensional architecture
        assert_eq!(architecture.dimensional_architecture.dimensions, 8);
        assert_eq!(architecture.dimensional_architecture.space_dimensions, 3);
        assert_eq!(architecture.dimensional_architecture.time_dimensions, 3);

        // Verify natural laws
        assert_eq!(architecture.natural_laws.len(), 3);

        // Verify force patterns
        assert_eq!(architecture.force_patterns.len(), 4);

        // Verify matter patterns
        assert_eq!(architecture.matter_patterns.len(), 5);
    }
}
