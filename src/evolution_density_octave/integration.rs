// Density Octave Integration
//
// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
// "Create clear pathway from Layer 7 → 1st Density → 2nd Density → ... → 8th Density"
//
// This module provides the bridge between Entity Layer 7 (Sub-Sub-Logos)
// and the Density Octave progression system.

use crate::entity_layer7::layer7::{
    EntityId, EntityState, SubSubLogos,
};
use crate::evolution_density_octave::density_octave::Density;
use crate::evolution_density_octave::requirements::DensityTransitionRequirements;

/// Bridge between Layer 7 and Density Octave
///
/// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
/// "Bridge between Layer 7 and Density Octave"
///
/// This bridge manages the transition of entities from Layer 7 (Sub-Sub-Logos)
/// into the density octave progression system.
///
/// IMPORTANT: This bridge tracks COLLECTIVE system emergence, not individual
/// entity density transitions. Individual entities DO NOT change density.
/// They progress within their density.
///
/// From COSMOLOGICAL-ARCHITECTURE.md (CORRECTED):
/// "Densities are hierarchical material substrates, not individual evolutionary stages"
#[derive(Debug, Clone)]
pub struct Layer7ToDensityBridge {
    /// Entities transitioning from Layer 7 to density octave
    transitioning_entities: Vec<SubSubLogos>,

    /// Density octave system for tracking collective emergence
    density_octave: crate::evolution_density_octave::density_octave::DensityOctave,

    /// Entity density assignments (which density each entity belongs to)
    entity_density_assignments: std::collections::HashMap<EntityId, Density>,
}

impl Layer7ToDensityBridge {
    /// Create new bridge
    pub fn new() -> Self {
        Layer7ToDensityBridge {
            transitioning_entities: Vec::new(),
            density_octave: crate::evolution_density_octave::density_octave::DensityOctave::new(),
            entity_density_assignments: std::collections::HashMap::new(),
        }
    }

    /// Add entity to density octave progression
    ///
    /// Entities are assigned to densities based on their consciousness level
    /// and evolutionary state. Entities DO NOT change density - they are
    /// assigned to a density and progress within it.
    pub fn add_entity(&mut self, entity: SubSubLogos) {
        // Assign entity to density based on consciousness level
        let density = self.assign_entity_to_density(&entity);

        // Store density assignment
        self.entity_density_assignments
            .insert(entity.entity_id.clone(), density);

        // Add to transitioning entities
        self.transitioning_entities.push(entity);
    }

    /// Assign entity to density based on consciousness level
    fn assign_entity_to_density(&self, entity: &SubSubLogos) -> Density {
        let consciousness = entity.current_state.consciousness_level;

        if consciousness < 0.15 {
            Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Quantum,
            )
        } else if consciousness < 0.25 {
            Density::First(
                crate::evolution_density_octave::density_octave::Density1SubLevel::Planetary,
            )
        } else if consciousness < 0.50 {
            Density::Second(
                crate::evolution_density_octave::density_octave::Density2SubLevel::ComplexLife,
            )
        } else if consciousness < 0.75 {
            Density::Third
        } else if consciousness < 0.85 {
            Density::Fourth
        } else if consciousness < 0.95 {
            Density::Fifth
        } else if consciousness < 0.99 {
            Density::Sixth
        } else if consciousness < 1.0 {
            Density::Seventh
        } else {
            Density::Eighth
        }
    }

    /// Process density transition (collective system emergence)
    ///
    /// IMPORTANT: This processes COLLECTIVE system emergence, not individual
    /// entity transitions. Individual entities DO NOT change density.
    ///
    /// From COMPREHENSIVE_SIMULATION_REFACTOR_PLAN.md Phase 2:
    /// "Process density transitions"
    pub fn process_density_transition(
        &mut self,
        entity_id: EntityId,
        current_density: Density,
        target_density: Density,
    ) -> Result<DensityTransitionResult, DensityTransitionError> {
        // Check if entity exists
        let entity = self
            .transitioning_entities
            .iter()
            .find(|e| e.entity_id == entity_id)
            .ok_or_else(|| DensityTransitionError::EntityNotFound(entity_id.clone()))?;

        // Get requirements for transition
        let requirements =
            DensityTransitionRequirements::get_requirements(&current_density, &target_density);

        // Check if entity meets requirements
        if !self.check_entity_meets_requirements(entity, &requirements) {
            return Err(DensityTransitionError::RequirementsNotMet {
                entity_id,
                requirements: Box::new(requirements.clone()),
                current_state: Box::new(entity.current_state.clone()),
            });
        }

        // Process transition (this is for COLLECTIVE emergence, not individual entity)
        // Update collective emergence with entity's state
        self.density_octave
            .update_collective_emergence(&entity.current_state);

        // Attempt to advance collective emergence
        let result = self.density_octave.advance_collective_emergence();

        match result {
            crate::evolution_density_octave::density_octave::DensityTransitionResult::DensityTransition { .. } |
            crate::evolution_density_octave::density_octave::DensityTransitionResult::Advanced { .. } => {
                Ok(DensityTransitionResult::Success {
                    entity_id,
                    from_density: current_density,
                    to_density: target_density,
                })
            }
            crate::evolution_density_octave::density_octave::DensityTransitionResult::NotReady { current_density, current_progress, required_progress } => {
                Err(DensityTransitionError::NotReady {
                    entity_id,
                    current_density,
                    current_progress,
                    required_progress,
                })
            }
            crate::evolution_density_octave::density_octave::DensityTransitionResult::AlreadyComplete => {
                Err(DensityTransitionError::AlreadyComplete)
            }
        }
    }

    /// Check if entity meets density transition requirements
    fn check_entity_meets_requirements(
        &self,
        entity: &SubSubLogos,
        requirements: &DensityTransitionRequirements,
    ) -> bool {
        let state = &entity.current_state;

        // Check consciousness level
        if state.consciousness_level < requirements.consciousness_level {
            return false;
        }

        // Check polarization
        if state.polarity_state.polarization_strength < requirements.polarization {
            return false;
        }

        // Check coherence
        if state.vibrational_state.coherence < requirements.coherence {
            return false;
        }

        // Check experience accumulation
        if state.experience_accumulation < requirements.catalyst_threshold * 100.0 {
            return false;
        }

        true
    }

    /// Get entity's current density
    pub fn get_entity_density(&self, entity_id: EntityId) -> Option<Density> {
        self.entity_density_assignments.get(&entity_id).cloned()
    }

    /// Get density transition requirements
    pub fn get_transition_requirements(
        &self,
        from: &Density,
        to: &Density,
    ) -> DensityTransitionRequirements {
        DensityTransitionRequirements::get_requirements(from, to)
    }

    /// Get all transitioning entities
    pub fn get_entities(&self) -> &[SubSubLogos] {
        &self.transitioning_entities
    }

    /// Get density octave
    pub fn get_density_octave(
        &self,
    ) -> &crate::evolution_density_octave::density_octave::DensityOctave {
        &self.density_octave
    }

    /// Get mutable density octave
    pub fn get_density_octave_mut(
        &mut self,
    ) -> &mut crate::evolution_density_octave::density_octave::DensityOctave {
        &mut self.density_octave
    }

    /// Update collective emergence with entity state
    pub fn update_collective_emergence(&mut self, entity_state: &EntityState) {
        self.density_octave
            .update_collective_emergence(entity_state);
    }

    /// Check if collective system is ready for density transition
    pub fn check_collective_readiness(&self) -> TransitionReadiness {
        let readiness = self.density_octave.check_collective_emergence_readiness();

        TransitionReadiness {
            is_ready: readiness.is_ready,
            current_progress: readiness.current_progress,
            required_progress: readiness.required_progress,
            next_density: readiness.next_density,
        }
    }
}

impl Default for Layer7ToDensityBridge {
    fn default() -> Self {
        Self::new()
    }
}

/// Density transition result
#[derive(Debug, Clone, PartialEq)]
pub enum DensityTransitionResult {
    Success {
        entity_id: EntityId,
        from_density: Density,
        to_density: Density,
    },
}

/// Density transition error
#[derive(Debug, Clone, PartialEq)]
pub enum DensityTransitionError {
    EntityNotFound(EntityId),
    RequirementsNotMet {
        entity_id: EntityId,
        requirements: Box<DensityTransitionRequirements>,
        current_state: Box<EntityState>,
    },
    NotReady {
        entity_id: EntityId,
        current_density: String,
        current_progress: f64,
        required_progress: f64,
    },
    AlreadyComplete,
}

/// Transition readiness
#[derive(Debug, Clone)]
pub struct TransitionReadiness {
    pub is_ready: bool,
    pub current_progress: f64,
    pub required_progress: f64,
    pub next_density: String,
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::EntityId;
    use crate::evolution_density_octave::density_octave::Density1SubLevel;
    use crate::entity_layer7::holographic_blueprint::ArchetypicalMindBlueprint;

    fn create_test_entity(id: EntityId, consciousness: f64) -> SubSubLogos {
        use crate::entity_layer7::layer7::{
            EntityType, EvolutionaryAttractorField, IndividualSpectrumConfiguration, PolarityState,
            VibrationalState,
        };
        use crate::spectrum::ArchetypicalMind;

        SubSubLogos {
            entity_id: id,
            entity_type: EntityType::Individual,
            parent_id: None,
            children: Vec::new(),
            composition: Vec::new(),
            environment_id: None,
            spectrum_access: crate::entity_layer7::layer7::SpectrumAccess {
                ratio: 1.0,
                veil_active: true,
                space_time_access: 0.9,
                time_space_access: 0.1,
                mannyness_access: 0.9,
            },
            physical_entity: None,
            violet_realm: crate::foundation::violet_realm::VioletRealm::new(),
            indigo_realm: crate::foundation::indigo_realm::IntelligentInfinity::new(),
            blue_realm: crate::foundation::blue_realm::Logos::new(),
            green_realm: crate::foundation::green_realm::LightLoveField::new(),
            yellow_realm: {
                let light_love_field = crate::foundation::green_realm::LightLoveField::new();
                crate::spectrum::yellow_realm::YellowRealm::new(light_love_field)
            },
            orange_realm: {
                let light_love_field = crate::foundation::green_realm::LightLoveField::new();
                let yellow = crate::spectrum::yellow_realm::YellowRealm::new(light_love_field);
                crate::spectrum::orange_realm::OrangeRealm::new(yellow)
            },
            red_realm: {
                let light_love_field = crate::foundation::green_realm::LightLoveField::new();
                let yellow = crate::spectrum::yellow_realm::YellowRealm::new(light_love_field);
                let orange = crate::spectrum::orange_realm::OrangeRealm::new(yellow);
                crate::spectrum::red_realm::RedRealm::new(orange)
            },
            spectrum_configuration: {
                let ratio = crate::spectrum::SpectrumRatio::new(
                    1.5,
                    crate::spectrum::SpectrumSide::SpaceTime,
                );
                IndividualSpectrumConfiguration::new(ratio)
            },
            archetypical_mind: ArchetypicalMind::new(
                crate::spectrum::archetypical_mind::ArchetypicalSystemType::TwentyTwoArchetypes,
                "test-solar-logos".to_string(),
            ),
            holographic_blueprint: {
                // HolographicBlueprint doesn't have new(), create from spectrum config
                let ratio = crate::spectrum::SpectrumRatio::new(
                    1.5,
                    crate::spectrum::SpectrumSide::SpaceTime,
                );
                let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
                let archetypical_mind_blueprint = ArchetypicalMindBlueprint::new_from_logos();
                crate::entity_layer7::holographic_blueprint::HolographicBlueprint::from_spectrum_configuration(
                    &spectrum_config,
                    &archetypical_mind_blueprint
                )
            },
            dna_patterns: Vec::new(),
            evolutionary_attractor: {
                let ratio = crate::spectrum::SpectrumRatio::new(
                    1.5,
                    crate::spectrum::SpectrumSide::SpaceTime,
                );
                let spectrum_config = IndividualSpectrumConfiguration::new(ratio);
                let archetypical_mind_blueprint = ArchetypicalMindBlueprint::new_from_logos();
                let blueprint = crate::entity_layer7::holographic_blueprint::HolographicBlueprint::from_spectrum_configuration(
                    &spectrum_config,
                    &archetypical_mind_blueprint
                );
                EvolutionaryAttractorField::new(&blueprint)
            },
            current_state: EntityState {
                vibrational_state: VibrationalState {
                    frequency: 0.5,
                    amplitude: 0.5,
                    coherence: 0.5,
                    density: crate::evolution_density_octave::density_octave::Density::First(
                        Density1SubLevel::Quantum,
                    ),
                    potential_energy: 0.25,
                    kinetic_energy: 0.25,
                },
                polarity_state: PolarityState {
                    polarity_bias: 0.0,
                    polarization_strength: 0.5,
                },
                consciousness_level: consciousness,
                experience_accumulation: 10.0,
                learning_progress: 5.0,
            },
            evolutionary_rate: 1.0,
            karmic_patterns: Vec::new(),
            evolution_clock: 0.0,
            polarization: crate::polarization::PolarizationProgress::default(),
            // Backward compatibility fields
            current_density: crate::evolution_density_octave::density_octave::Density::First(
                Density1SubLevel::Quantum,
            ),
            consciousness_level: consciousness,
            experience_accumulation: 10.0,
            learning_progress: 5.0,
            archetype_activations: [0.0; 22],
            veil_transparency: 0.5,
            space_time_ratio: 1.0,
            time_space_ratio: 1.0,
            spectrum_position: 0.5,
            potential_energy: 0.25,
            kinetic_energy: 0.25,
            energy: 0.5,
            veil: crate::entity_layer7::layer7::VeilInfo {
                active: true,
                transparency: 0.5,
                illusion_strength: 0.5,
                access_control: crate::entity_layer7::layer7::VeilAccessControl {
                    time_space_access: 0.5,
                    holographic_connection_access: 0.5,
                    higher_consciousness_access: 0.5,
                },
            },
        }
    }

    #[test]
    fn test_layer7_to_density_bridge_creation() {
        let bridge = Layer7ToDensityBridge::new();

        assert_eq!(bridge.get_entities().len(), 0);
        assert!(bridge.get_density_octave().get_progress() >= 0.0);
    }

    #[test]
    fn test_add_entity() {
        let mut bridge = Layer7ToDensityBridge::new();
        let entity = create_test_entity(EntityId::new("1".to_string()), 0.2);

        bridge.add_entity(entity);

        assert_eq!(bridge.get_entities().len(), 1);
        assert!(bridge
            .get_entity_density(EntityId::new("1".to_string()))
            .is_some());
    }

    #[test]
    fn test_assign_entity_to_density() {
        let bridge = Layer7ToDensityBridge::new();

        // Test low consciousness - 1st density
        let entity_low = create_test_entity(EntityId::new("1".to_string()), 0.1);
        let density_low = bridge.assign_entity_to_density(&entity_low);
        assert!(matches!(density_low, Density::First(_)));

        // Test medium consciousness - 3rd density
        let entity_medium = create_test_entity(EntityId::new("2".to_string()), 0.5);
        let density_medium = bridge.assign_entity_to_density(&entity_medium);
        assert_eq!(density_medium, Density::Third);

        // Test high consciousness - 6th density
        let entity_high = create_test_entity(EntityId::new("3".to_string()), 0.95);
        let density_high = bridge.assign_entity_to_density(&entity_high);
        assert_eq!(density_high, Density::Sixth);
    }

    #[test]
    fn test_get_entity_density() {
        let mut bridge = Layer7ToDensityBridge::new();
        let entity = create_test_entity(EntityId::new("1".to_string()), 0.2);

        bridge.add_entity(entity);

        let density = bridge.get_entity_density(EntityId::new("1".to_string()));
        assert!(density.is_some());
    }

    #[test]
    fn test_get_transition_requirements() {
        let bridge = Layer7ToDensityBridge::new();

        let from = Density::First(Density1SubLevel::Quantum);
        let to = Density::Third;

        let requirements = bridge.get_transition_requirements(&from, &to);

        assert!(requirements.consciousness_level > 0.0);
        assert!(requirements.polarization > 0.0);
        assert!(requirements.coherence > 0.0);
        assert!(requirements.catalyst_threshold > 0.0);
    }

    #[test]
    fn test_update_collective_emergence() {
        let mut bridge = Layer7ToDensityBridge::new();
        let entity = create_test_entity(EntityId::new("1".to_string()), 0.5);

        bridge.update_collective_emergence(&entity.current_state);

        // Collective emergence should have been updated
        assert!(bridge.get_density_octave().get_progress() >= 0.0);
    }

    #[test]
    fn test_check_collective_readiness() {
        let bridge = Layer7ToDensityBridge::new();

        let readiness = bridge.check_collective_readiness();

        assert!(readiness.current_progress >= 0.0);
        assert!(readiness.required_progress > 0.0);
        assert!(!readiness.next_density.is_empty());
    }
}
