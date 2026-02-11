//! Holographic Game Loop
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
//! "The holographic system is PRIMARY, ECS is SECONDARY"
//! "The game loop orchestrates holographic evolution, not entity updates"
//!
//! This module implements the holographic-first game loop architecture:
//! 1. Holographic Field is PRIMARY - it evolves first
//! 2. Entities are EXTRACTED from the field (not iterated)
//! 3. ECS is SECONDARY - wrapper for UI/inventory/game components
//! 4. No entity-centric update loop

use crate::entity_layer7::layer7::{EntityId, SubSubLogos};
use crate::simulation_v3::catalyst_system::CatalystManager;
use crate::simulation_v3::collective_dynamics::CollectiveDynamicsManager;
use crate::simulation_v3::emergent_behavior::EmergenceManager;
use crate::simulation_v3::entity_lifecycle::EntityLifecycleManager;
use crate::simulation_v3::environment::EnvironmentalInteractionManager;
use crate::simulation_v3::holographic_field::HolographicFieldManager;
use crate::simulation_v3::inter_scale_interactions::InterScaleInteractionManager;
use crate::simulation_v3::statistics::StatisticsTracker;
use crate::types::Float;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Holographic Game Loop
///
/// Orchestrates holographic evolution in the correct order:
/// 1. Evolve holographic field (PRIMARY)
/// 2. Extract entities from field
/// 3. Apply Free Will + Archetypical choices
/// 4. Update ECS wrapper (SECONDARY)
///
/// From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
/// "The holographic system is NOT a data source - it is the simulation engine itself"
pub struct HolographicGameLoop {
    /// Holographic field manager (PRIMARY)
    holographic_manager: HolographicFieldManager,

    /// Entity lifecycle manager
    lifecycle_manager: EntityLifecycleManager,

    /// Catalyst manager (Free Will choices)
    catalyst_manager: CatalystManager,

    /// Collective dynamics manager
    collective_dynamics_manager: CollectiveDynamicsManager,

    /// Environmental interaction manager
    environmental_manager: EnvironmentalInteractionManager,

    /// Inter-scale interaction manager
    inter_scale_interaction_manager: InterScaleInteractionManager,

    /// Emergence manager
    emergence_manager: EmergenceManager,

    /// Statistics tracker
    statistics_tracker: StatisticsTracker,

    /// Current simulation step
    current_step: u64,

    /// Current simulation time
    current_time: u64,

    /// Entities extracted from holographic field (SECONDARY)
    /// These are used for game-specific components (UI, inventory, etc.)
    entities: HashMap<EntityId, SubSubLogos>,
}

impl HolographicGameLoop {
    /// Create a new holographic game loop
    pub fn new(holographic_manager: HolographicFieldManager) -> Self {
        HolographicGameLoop {
            holographic_manager,
            lifecycle_manager: EntityLifecycleManager::new(),
            catalyst_manager: CatalystManager::new(),
            collective_dynamics_manager: CollectiveDynamicsManager::new(),
            environmental_manager: EnvironmentalInteractionManager::new(),
            inter_scale_interaction_manager: InterScaleInteractionManager::new(),
            emergence_manager: EmergenceManager::new(),
            statistics_tracker: StatisticsTracker::new(),
            current_step: 0,
            current_time: 0,
            entities: HashMap::new(),
        }
    }

    /// Run one holographic game loop step
    ///
    /// This is the CORRECT order of operations:
    /// 1. Evolve holographic field (PRIMARY)
    /// 2. Extract entities from field
    /// 3. Apply Free Will + Archetypical choices
    /// 4. Update game-specific components (SECONDARY)
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
    /// "Game Loop (Orchestrator) → ECS (Secondary - for game mechanics only)"
    pub fn run_step(&mut self, time_step_size: Float) -> GameLoopResult {
        let step_start = Instant::now();

        // Step 1: Evolve holographic field (PRIMARY)
        //
        // From COSMOLOGICAL-ARCHITECTURE.md:
        // "Reality is not constructed; it is Unfolded from a Pre-Existing Whole"
        //
        // The holographic field evolves first, entities emerge from it
        let holographic_evolution_start = Instant::now();
        self.evolve_holographic_field();
        let holographic_evolution_time = holographic_evolution_start.elapsed();

        // Step 2: Extract entities from holographic field
        //
        // From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
        // "Entities are EXTRACTED from the field (not iterated)"
        //
        // The field contains all entities, we extract them for game-specific operations
        let extraction_start = Instant::now();
        self.extract_entities_from_field();
        let extraction_time = extraction_start.elapsed();

        // Step 3: Apply Free Will + Archetypical choices
        //
        // From COSMOLOGICAL-ARCHITECTURE.md:
        // "Archetype 22 (The Choice): Creates polarity by choosing between STO and STS"
        //
        // Free Will choices are made based on archetype interference patterns
        let free_will_start = Instant::now();
        self.apply_free_will_choices();
        let free_will_time = free_will_start.elapsed();

        // Step 4: Update game-specific components (SECONDARY)
        //
        // From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
        // "ECS (Secondary - for game mechanics only)"
        //
        // Only update game-specific components (UI, inventory, etc.)
        // NOT the holographic field (already evolved in Step 1)
        let game_components_start = Instant::now();
        self.update_game_components();
        let game_components_time = game_components_start.elapsed();

        // Step 5: Update statistics
        self.update_statistics();

        // Advance time
        self.current_step += 1;
        self.current_time += time_step_size as u64;

        let total_time = step_start.elapsed();

        GameLoopResult {
            step: self.current_step,
            holographic_evolution_time,
            extraction_time,
            free_will_time,
            game_components_time,
            total_time,
            entities_count: self.entities.len(),
        }
    }

    /// Evolve holographic field (PRIMARY step)
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "The spectrum is configured at galactic and solar scales before physical matter exists"
    ///
    /// The holographic field evolves first, then entities emerge from it.
    fn evolve_holographic_field(&mut self) {
        // Update holographic field connections
        let _ = self.holographic_manager.update_field(1);

        // Calculate interference patterns
        self.holographic_manager.calculate_interference_patterns();

        // Track resonance for all entities
        self.holographic_manager.track_resonance_for_all_entities();

        // Update holographic field statistics
        self.holographic_manager.update_statistics();
    }

    /// Extract entities from holographic field
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
    /// "Entities are EXTRACTED from the field (not iterated)"
    ///
    /// The holographic field contains all entities, we extract them
    /// for game-specific operations (UI, inventory, etc.).
    fn extract_entities_from_field(&mut self) {
        // Get entities from holographic field
        let field_entities = self.holographic_manager.get_all_entities();

        // Update local entities map (used for game-specific components)
        for entity in field_entities {
            self.entities.insert(entity.entity_id.clone(), entity);
        }
    }

    /// Apply Free Will + Archetypical choices
    ///
    /// From COSMOLOGICAL-ARCHITECTURE.md:
    /// "Third density is the density of choice"
    /// "Archetype 22 (The Choice): Creates polarity by choosing between STO and STS"
    ///
    /// Free Will choices are made based on archetype interference patterns.
    /// This is NOT a behavior tree - it's emergent from holographic interference.
    fn apply_free_will_choices(&mut self) {
        // Collect entity states for catalyst application
        let mut entity_states: HashMap<EntityId, crate::entity_layer7::layer7::EntityState> = self
            .lifecycle_manager
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.current_state.clone()))
            .collect();

        // Collect entity densities
        let entity_densities: HashMap<
            EntityId,
            crate::evolution_density_octave::density_octave::Density,
        > = self
            .lifecycle_manager
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.current_density.clone()))
            .collect();

        // Collect entity types
        let entity_types: HashMap<EntityId, crate::entity_layer7::layer7::EntityType> = self
            .entities
            .iter()
            .map(|(id, data)| (id.clone(), data.entity_type.clone()))
            .collect();

        // Generate catalysts periodically
        if self.current_step % 5 == 0 {
            let num_catalysts = (self.lifecycle_manager.entities.len() / 15).max(1).min(10);
            self.catalyst_manager.generate_catalysts(
                &mut entity_states,
                &entity_types,
                &entity_densities,
                num_catalysts,
            );
        }

        // Apply catalysts with Free Will integration
        let catalyst_events = self.catalyst_manager.apply_catalysts(
            &mut entity_states,
            &entity_densities,
            &mut self.lifecycle_manager.free_will_kernels,
        );

        // Update entity states after Free Will choices
        for (entity_id, new_state) in entity_states {
            if let Some(lifecycle_data) = self.lifecycle_manager.entities.get_mut(&entity_id) {
                lifecycle_data.current_state = new_state;
            }
        }

        // Record catalyst events
        for event in catalyst_events {
            self.record_catalyst_event(event);
        }
    }

    /// Update game-specific components (SECONDARY step)
    ///
    /// From GAMING_ENGINE_ROADMAP_v2.md Section 2.2:
    /// "ECS (Secondary - for game mechanics only)"
    ///
    /// Only update game-specific components (UI, inventory, etc.).
    /// NOT the holographic field (already evolved in Step 1).
    fn update_game_components(&mut self) {
        // Evolve entities (lifecycle)
        let _evolution_result = self.lifecycle_manager.evolve_entities(1);

        // Update catalyst manager time
        self.catalyst_manager.update_time();

        // Update collective dynamics periodically
        if self.current_step % 5 == 0 {
            self.update_collective_dynamics();
        }

        // Update environmental interactions periodically
        if self.current_step % 5 == 0 {
            self.update_environmental_interactions();
        }

        // Update inter-scale interactions periodically
        if self.current_step % 5 == 0 {
            self.update_inter_scale_interactions();
        }

        // Update emergent behavior periodically
        if self.current_step % 10 == 0 {
            self.update_emergent_behavior();
        }
    }

    /// Update collective dynamics
    fn update_collective_dynamics(&mut self) {
        let entities_map: HashMap<_, _> = self
            .entities
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let empty_catalysts: Vec<crate::simulation_v3::catalyst_system::Catalyst> = Vec::new();
        self.collective_dynamics_manager
            .update_collective_dynamics(&entities_map, &empty_catalysts);
    }

    /// Update environmental interactions
    fn update_environmental_interactions(&mut self) {
        let entities_map: HashMap<_, _> = self
            .entities
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        self.environmental_manager.update_environment(&entities_map);
    }

    /// Update inter-scale interactions
    fn update_inter_scale_interactions(&mut self) {
        let entities_map: HashMap<_, _> = self
            .entities
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        self.inter_scale_interaction_manager
            .process_composition_interactions(&entities_map);
    }

    /// Update emergent behavior
    fn update_emergent_behavior(&mut self) {
        let entities_map: HashMap<_, _> = self
            .entities
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let system_emergence = self
            .emergence_manager
            .calculate_system_emergence(&entities_map);
        // Note: calculate_system_emergence already adds the measurement internally
    }

    /// Record catalyst event
    fn record_catalyst_event(
        &mut self,
        event: crate::simulation_v3::catalyst_system::CatalystEvent,
    ) {
        // Update entity's polarization based on catalyst choice
        let polarity_choice = match event.polarity_choice {
            crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToOthers => {
                crate::foundation::indigo_realm::PolarityChoice::ServiceToOthers
            }
            crate::simulation_v3::catalyst_system::PolarityChoice::ServiceToSelf => {
                crate::foundation::indigo_realm::PolarityChoice::ServiceToSelf
            }
            crate::simulation_v3::catalyst_system::PolarityChoice::Unpolarized => {
                crate::foundation::indigo_realm::PolarityChoice::Neutral
            }
        };

        if let Some(entity) = self.entities.get_mut(&event.entity_id) {
            entity.polarization.make_choice(polarity_choice);
        }
    }

    /// Update statistics
    fn update_statistics(&mut self) {
        // Record step statistics periodically
        if self.current_step % 10 == 0 || self.current_step == 1 {
            let lifecycle_stats = self.lifecycle_manager.get_statistics();

            // Build density distribution
            let mut density_distribution = HashMap::new();
            for (density, count) in &lifecycle_stats.entities_by_density {
                density_distribution.insert(density.clone(), *count);
            }

            // Build polarization distribution
            let polarization_distribution =
                crate::simulation_v3::statistics::PolarizationDistribution {
                    sto: lifecycle_stats.polarization_distribution.sto_count,
                    sts: lifecycle_stats.polarization_distribution.sts_count,
                    unpolarized: lifecycle_stats.polarization_distribution.unpolarized_count,
                    average_bias: lifecycle_stats.polarization_distribution.avg_sto_score
                        - lifecycle_stats.polarization_distribution.avg_sts_score,
                };

            // Calculate feature completion metrics
            let avg_development = lifecycle_stats.avg_developmental_level;
            let normalized_development = (avg_development / 10.0).clamp(0.0, 1.0);
            let total_tapped_energy =
                normalized_development * lifecycle_stats.total_entities as Float * 10.0;
            let average_tap_strength = if lifecycle_stats.total_entities > 0 {
                total_tapped_energy / lifecycle_stats.total_entities as Float
            } else {
                0.0
            };

            let spectrum_access_level = normalized_development.clamp(0.0, 1.0);
            let veil_thickness = (1.0 - normalized_development * 0.8).clamp(0.0, 1.0);
            let veil_transparency = (normalized_development * 0.9).clamp(0.0, 1.0);

            let polarization_intensity = if lifecycle_stats.total_entities > 0 {
                (lifecycle_stats.polarization_distribution.sto_count as Float
                    + lifecycle_stats.polarization_distribution.sts_count as Float)
                    / lifecycle_stats.total_entities as Float
            } else {
                0.0
            };
            let attractor_field_activation =
                ((normalized_development + polarization_intensity) / 2.0).clamp(0.0, 1.0);

            self.statistics_tracker.record_evolution_step(
                self.current_step,
                lifecycle_stats.total_entities,
                density_distribution,
                polarization_distribution,
                lifecycle_stats.total_transitions,
                total_tapped_energy,
                average_tap_strength,
                spectrum_access_level,
                veil_thickness,
                veil_transparency,
                attractor_field_activation,
            );
        }
    }

    /// Get entities (SECONDARY - for game-specific components only)
    pub fn get_entities(&self) -> &HashMap<EntityId, SubSubLogos> {
        &self.entities
    }

    /// Get holographic field manager (PRIMARY)
    pub fn get_holographic_manager(&self) -> &HolographicFieldManager {
        &self.holographic_manager
    }

    /// Get holographic field manager (mutable)
    pub fn get_holographic_manager_mut(&mut self) -> &mut HolographicFieldManager {
        &mut self.holographic_manager
    }

    /// Add entity to holographic field
    pub fn add_entity(&mut self, entity: SubSubLogos) -> Result<(), String> {
        // Add to holographic field (PRIMARY)
        let _ = self.holographic_manager.add_entity(entity.clone());

        // Add to lifecycle manager
        let entity_id = entity.entity_id.clone();
        let initial_state = entity.current_state.clone();
        let evolutionary_trajectory = entity.get_evolutionary_trajectory().clone();
        let free_will_kernel = crate::consciousness::free_will::FreeWillKernel::new(
            entity.indigo_realm.archetype22.clone(),
        );
        let initial_density = entity.current_density.clone();

        // Create EntitySpectrumAccess from SpectrumAccess
        let spectrum_configuration = crate::entity_layer7::layer7::EntitySpectrumAccess {
            space_time_access: entity.spectrum_access.space_time_access,
            oneness_access: entity.spectrum_access.time_space_access,
            veil_transparency: if entity.spectrum_access.veil_active {
                0.0
            } else {
                1.0
            },
            evolutionary_level: crate::entity_layer7::layer7::SpectrumAccessLevel::ThirdDensity,
            space_time_ratio: entity.spectrum_access.space_time_access,
            time_space_ratio: entity.spectrum_access.time_space_access,
            spectrum_position: entity.spectrum_access.space_time_access,
        };

        self.lifecycle_manager.add_entity(
            entity_id.clone(),
            initial_state,
            evolutionary_trajectory,
            free_will_kernel,
            initial_density,
            spectrum_configuration,
        );

        // Add to local entities map (SECONDARY)
        self.entities.insert(entity_id, entity);

        Ok(())
    }

    /// Get current step
    pub fn current_step(&self) -> u64 {
        self.current_step
    }

    /// Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Get statistics tracker
    pub fn get_statistics_tracker(&self) -> &StatisticsTracker {
        &self.statistics_tracker
    }

    /// Get statistics tracker (mutable)
    pub fn get_statistics_tracker_mut(&mut self) -> &mut StatisticsTracker {
        &mut self.statistics_tracker
    }
}

/// Game Loop Result
///
/// Contains performance metrics for one game loop step.
#[derive(Debug, Clone)]
pub struct GameLoopResult {
    /// Step number
    pub step: u64,

    /// Time spent evolving holographic field
    pub holographic_evolution_time: Duration,

    /// Time spent extracting entities from field
    pub extraction_time: Duration,

    /// Time spent applying Free Will choices
    pub free_will_time: Duration,

    /// Time spent updating game components
    pub game_components_time: Duration,

    /// Total time for this step
    pub total_time: Duration,

    /// Number of entities
    pub entities_count: usize,
}

impl GameLoopResult {
    /// Get summary string
    pub fn summary(&self) -> String {
        format!(
            "Step {}: {:.3}ms total (holographic: {:.3}ms, extraction: {:.3}ms, free_will: {:.3}ms, game: {:.3}ms) | {} entities",
            self.step,
            self.total_time.as_secs_f64() * 1000.0,
            self.holographic_evolution_time.as_secs_f64() * 1000.0,
            self.extraction_time.as_secs_f64() * 1000.0,
            self.free_will_time.as_secs_f64() * 1000.0,
            self.game_components_time.as_secs_f64() * 1000.0,
            self.entities_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_layer7::layer7::EntityId;

    #[test]
    fn test_holographic_game_loop_creation() {
        let holographic_manager = HolographicFieldManager::new();
        let game_loop = HolographicGameLoop::new(holographic_manager);

        assert_eq!(game_loop.current_step(), 0);
        assert_eq!(game_loop.current_time(), 0);
        assert_eq!(game_loop.get_entities().len(), 0);
    }

    #[test]
    fn test_holographic_game_loop_step() {
        let holographic_manager = HolographicFieldManager::new();
        let mut game_loop = HolographicGameLoop::new(holographic_manager);

        let result = game_loop.run_step(1.0);

        assert_eq!(result.step, 1);
        assert_eq!(game_loop.current_step(), 1);
        assert_eq!(game_loop.current_time(), 1);
    }

    #[test]
    fn test_holographic_game_loop_multiple_steps() {
        let holographic_manager = HolographicFieldManager::new();
        let mut game_loop = HolographicGameLoop::new(holographic_manager);

        for i in 1..=10 {
            let result = game_loop.run_step(1.0);
            assert_eq!(result.step, i);
            assert_eq!(game_loop.current_step(), i);
            assert_eq!(game_loop.current_time(), i as u64);
        }
    }
}
