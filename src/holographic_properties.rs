// Holographic Properties Module
// Layer 8: Holographic properties - any portion contains the whole
//
// MIGRATION 9: HOLOGRAPHIC PROPERTIES
// Migrated to: src/entity_layer7/holographic_blueprint.rs
// Date: 2026-02-05
//
// This module has been migrated to the entity_layer7 module as part of the
// directory structure refactoring (PHASE4_MODULE_MIGRATION).
//
// All types and implementations have been moved to:
// - entity_layer7::holographic_blueprint::{HolographicProperties, ArchetypicalMindSummary, ...}
//
// The following types are now available from the new location:
// - ArchetypicalMindSummary
// - HolographicProperties
// - EntityOctaveContainment
// - EnvironmentArchetypicalReflection
// - HolographicCoherence
// - HolographicState
//
// For backward compatibility, this module re-exports all types from the new location.
// After all imports are updated, this file can be removed.

// Re-export all types for backward compatibility
pub use crate::entity_layer7::holographic_blueprint::{
    ArchetypicalMindSummary, EntityOctaveContainment, EnvironmentArchetypicalReflection,
    HolographicCoherence, HolographicProperties, HolographicState,
};

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetypes::ArchetypeRole;
    // NOTE: entities and environments modules don't exist
    // use crate::entities::{EntityEmergence, PotentialEntity};
    // use crate::environments::{EnvironmentFormation, EnvironmentScale};

    /// Helper function to convert u8 to ArchetypeRole in tests
    fn u8_to_archetype_id(id: u8) -> ArchetypeRole {
        match id {
            1 => ArchetypeRole::Matrix,
            2 => ArchetypeRole::Potentiator,
            3 => ArchetypeRole::Catalyst,
            4 => ArchetypeRole::Experience,
            5 => ArchetypeRole::Significator,
            6 => ArchetypeRole::Transformation,
            7 => ArchetypeRole::GreatWay,
            _ => panic!("Invalid archetype role ID: {}", id),
        }
    }

    #[test]
    fn test_holographic_properties_creation() {
        let holographic = HolographicProperties::new();
        let state = holographic.get_state_summary();
        assert_eq!(state.total_entities, 0);
        assert_eq!(state.total_environments, 0);
        assert_eq!(state.overall_holographic_coherence, 0.0);
    }

    // NOTE: This test is disabled because EntityEmergence and PotentialEntity don't exist
    /*
    #[test]
    fn test_entity_octave_containment() {
        let mut holographic = HolographicProperties::new();
        let mut emergence = EntityEmergence::new();

        // Create an entity
        let potential = PotentialEntity::new(0, 0.8, 0.6, 0.5, 0.5, 0.5);
        emergence.process(vec![potential]);

        let entities = emergence.get_entities();
        let archetypical_mind_summary = ArchetypicalMindSummary {
            coherence: 0.5,
            total_activation: 0.5,
            dominant_archetype: 0,
        };
        let creation_engine = crate::creation_engine::CreationEngine::new();
        let environments = std::collections::HashMap::new();

        holographic.process(
            entities,
            &environments,
            &archetypical_mind_summary,
            &creation_engine,
        );

        let containment = holographic.get_entity_octave_containment(0);
        assert!(containment.is_some());
        assert_eq!(containment.unwrap().entity_id, 0);
        assert!(containment.unwrap().octave_containment >= 0.0);
        assert!(containment.unwrap().octave_containment <= 1.0);
    }
    */

    // NOTE: This test is disabled because EnvironmentScale, EnvironmentID, etc. don't exist in stub
    /*
    #[test]
    fn test_environment_archetypical_reflection() {
        let mut holographic = HolographicProperties::new();
        let archetypical_mind_summary = ArchetypicalMindSummary {
            coherence: 0.5,
            total_activation: 0.5,
            dominant_archetype: 0,
        };
        let creation_engine = crate::creation_engine::CreationEngine::new();
        let entities = std::collections::HashMap::new();

        // Create an environment
        let mut environments = std::collections::HashMap::new();
        let env = crate::environments::Environment {
            id: crate::environments::EnvironmentID::new(0),
            scale: EnvironmentScale::Planetary,
            archetypical_resonance: {
                let mut resonance = std::collections::HashMap::new();
                for id in 1..=22u8 {
                    resonance.insert(u8_to_archetype_id(id), 0.5);
                }
                resonance
            },
            realm_composition: crate::environments::RealmComposition {
                space_time_portion: 0.5,
                time_space_portion: 0.5,
                veil_transparency: 0.5,
            },
            complexity: 0.7,
            stability: 0.6,
            catalyst_flow: 0.5,
            evolutionary_potential: 0.5,
            energy_input: 0.5,
            energy_output: 0.5,
            emerged: true,
            emergence_age: 100,
        };
        let env_id = crate::environments::EnvironmentID::new(0);
        environments.insert(env_id, env);

        holographic.process(
            &entities,
            &environments,
            &archetypical_mind_summary,
            &creation_engine,
        );

        let reflection =
            holographic.get_environment_reflection(&crate::environments::EnvironmentID::new(0));
        assert!(reflection.is_some());
        assert_eq!(reflection.unwrap().environment_id, 0);
        assert_eq!(reflection.unwrap().archetype_reflection.len(), 22);
        assert!(reflection.unwrap().overall_reflection >= 0.0);
        assert!(reflection.unwrap().overall_reflection <= 1.0);
    }
    */

    // NOTE: This test is disabled because EntityEmergence and PotentialEntity don't exist
    /*
    #[test]
    fn test_holographic_coherence() {
        let mut holographic = HolographicProperties::new();
        let mut emergence = EntityEmergence::new();
        let archetypical_mind_summary = ArchetypicalMindSummary {
            coherence: 0.5,
            total_activation: 0.5,
            dominant_archetype: 0,
        };
        let creation_engine = crate::creation_engine::CreationEngine::new();

        // Create environments
        let mut environments = std::collections::HashMap::new();
        let env = crate::environments::Environment {
            id: crate::environments::EnvironmentID::new(0),
            scale: EnvironmentScale::Planetary,
            archetypical_resonance: {
                let mut resonance = std::collections::HashMap::new();
                for id in 1..=22u8 {
                    resonance.insert(u8_to_archetype_id(id), 0.5);
                }
                resonance
            },
            realm_composition: crate::environments::RealmComposition {
                space_time_portion: 0.5,
                time_space_portion: 0.5,
                veil_transparency: 0.5,
            },
            complexity: 0.7,
            stability: 0.6,
            catalyst_flow: 0.5,
            evolutionary_potential: 0.5,
            energy_input: 0.5,
            energy_output: 0.5,
            emerged: true,
            emergence_age: 100,
        };
        let env_id = crate::environments::EnvironmentID::new(0);
        environments.insert(env_id, env);

        // Create entities
        let potential = PotentialEntity::new(0, 0.8, 0.6, 0.5, 0.5, 0.5);
        emergence.process(vec![potential]);

        let entities = emergence.get_entities();

        holographic.process(
            entities,
            &environments,
            &archetypical_mind_summary,
            &creation_engine,
        );

        let coherence = holographic.get_holographic_coherence();
        assert!(coherence.overall_coherence >= 0.0);
        assert!(coherence.overall_coherence <= 1.0);
    }
    */

    #[test]
    fn test_empty_entities_and_environments() {
        let mut holographic = HolographicProperties::new();
        let archetypical_mind_summary = ArchetypicalMindSummary {
            coherence: 0.5,
            total_activation: 0.5,
            dominant_archetype: 0,
        };
        let creation_engine = crate::creation_engine::CreationEngine::new(1);
        let entities = std::collections::HashMap::new();
        let environments = std::collections::HashMap::new();

        let state = holographic.process(
            &entities,
            &environments,
            &archetypical_mind_summary,
            &creation_engine,
        );
        assert_eq!(state.total_entities, 0);
        assert_eq!(state.total_environments, 0);
        assert_eq!(state.overall_holographic_coherence, 0.0);
    }

    // NOTE: This test is disabled because EntityEmergence and PotentialEntity don't exist
    /*
    #[test]
    fn test_multiple_entities() {
        let mut holographic = HolographicProperties::new();
        let mut emergence = EntityEmergence::new();
        let archetypical_mind_summary = ArchetypicalMindSummary {
            coherence: 0.5,
            total_activation: 0.5,
            dominant_archetype: 0,
        };
        let creation_engine = crate::creation_engine::CreationEngine::new();
        let environments = std::collections::HashMap::new();

        // Create multiple entities
        let potentials = vec![
            PotentialEntity::new(0, 0.8, 0.6, 0.5, 0.5, 0.5),
            PotentialEntity::new(1, 0.75, 0.55, 0.45, 0.45, 0.45),
            PotentialEntity::new(2, 0.7, 0.5, 0.4, 0.4, 0.4),
        ];
        emergence.process(potentials);

        let entities = emergence.get_entities();

        holographic.process(
            entities,
            &environments,
            &archetypical_mind_summary,
            &creation_engine,
        );

        let state = holographic.get_state_summary();
        assert_eq!(state.total_entities, 3);
        assert!(state.avg_octave_containment >= 0.0);
    }
    */

    // NOTE: This test is disabled because EnvironmentScale, EnvironmentID, etc. don't exist in stub
    /*
    #[test]
    fn test_multiple_environments() {
        let mut holographic = HolographicProperties::new();
        let archetypical_mind_summary = ArchetypicalMindSummary {
            coherence: 0.5,
            total_activation: 0.5,
            dominant_archetype: 0,
        };
        let creation_engine = crate::creation_engine::CreationEngine::new();
        let entities = std::collections::HashMap::new();

        // Create multiple environments
        let mut environments = std::collections::HashMap::new();
        for (i, scale) in [
            EnvironmentScale::Atomic,
            EnvironmentScale::Cellular,
            EnvironmentScale::Organismic,
        ]
        .iter()
        .enumerate()
        {
            let env = crate::environments::Environment {
                id: crate::environments::EnvironmentID::new(i as u64),
                scale: scale.clone(),
                archetypical_resonance: {
                    let mut resonance = std::collections::HashMap::new();
                    for id in 1..=22u8 {
                        resonance.insert(u8_to_archetype_id(id), 0.5);
                    }
                    resonance
                },
                realm_composition: crate::environments::RealmComposition {
                    space_time_portion: 0.5,
                    time_space_portion: 0.5,
                    veil_transparency: 0.5,
                },
                complexity: 0.7,
                stability: 0.6,
                catalyst_flow: 0.5,
                evolutionary_potential: 0.5,
                energy_input: 0.5,
                energy_output: 0.5,
                emerged: true,
                emergence_age: 100,
            };
            let env_id = crate::environments::EnvironmentID::new(i as u64);
            environments.insert(env_id, env);
        }

        holographic.process(
            &entities,
            &environments,
            &archetypical_mind_summary,
            &creation_engine,
        );

        let state = holographic.get_state_summary();
        assert_eq!(state.total_environments, 3);
        assert!(state.avg_archetypical_reflection >= 0.0);
    }
    */
}
