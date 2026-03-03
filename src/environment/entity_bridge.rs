//! Entity-Planet Coupling
//!
//! From HOLOSIM_INFINITE_COMPLETION_ROADMAP_V3.md Phase 4.2:
//! "Implement entity-planet coupling"
//! "Entities are IN environments, not floating in void"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! > "The entity contains within it all densities and sub-densities"
//! > "Entities are not separate from their environment - they are expressions of it"
//!
//! This module implements:
//! - Assigning entities to planets based on position
//! - Tracking entity-environment relationships
//! - Environmental state propagation to entities

use super::*;
use crate::hpo::planetary_emergence::Planet;
use crate::hpo::spatial_field::Position3D;
use std::collections::HashMap;

/// Configuration for entity-planet coupling
#[derive(Debug, Clone)]
pub struct EntityBridgeConfig {
    /// Maximum distance from planet center for assignment
    pub max_planet_distance: f64,

    /// Minimum coherence for planet detection
    pub planet_detection_threshold: f64,

    /// Enable environmental effects
    pub enable_environmental_effects: bool,

    /// Update frequency (in steps)
    pub update_frequency: usize,
}

impl Default for EntityBridgeConfig {
    fn default() -> Self {
        EntityBridgeConfig {
            max_planet_distance: 1000.0,
            planet_detection_threshold: 0.5,
            enable_environmental_effects: true,
            update_frequency: 1,
        }
    }
}

/// Entity-Planet Bridge
///
/// Couples entities to their planetary environments
#[derive(Debug, Clone)]
pub struct EntityBridge {
    config: EntityBridgeConfig,
    planets: HashMap<PlanetId, Planet>,
    entity_assignments: HashMap<u64, EntityPlanetAssignment>,
    entity_states: HashMap<u64, EntityEnvironmentState>,
    statistics: EntityBridgeStatistics,
}

#[derive(Debug, Clone)]
pub struct EntityPlanetAssignment {
    pub entity_id: u64,
    pub planet_id: PlanetId,
    pub assigned_position: Position3D,
    pub distance_from_center: f64,
    pub assignment_time: usize,
}

#[derive(Debug, Clone, Default)]
pub struct EntityBridgeStatistics {
    pub total_entities: usize,
    pub assigned_entities: usize,
    pub unassigned_entities: usize,
    pub planet_count: usize,
    pub average_planet_distance: f64,
}

impl EntityBridge {
    pub fn new() -> Self {
        EntityBridge {
            config: EntityBridgeConfig::default(),
            planets: HashMap::new(),
            entity_assignments: HashMap::new(),
            entity_states: HashMap::new(),
            statistics: EntityBridgeStatistics::default(),
        }
    }

    pub fn with_config(config: EntityBridgeConfig) -> Self {
        EntityBridge {
            config,
            planets: HashMap::new(),
            entity_assignments: HashMap::new(),
            entity_states: HashMap::new(),
            statistics: EntityBridgeStatistics::default(),
        }
    }

    /// Add a planet to the bridge
    pub fn add_planet(&mut self, planet_id: PlanetId, planet: Planet) {
        self.planets.insert(planet_id, planet);
    }

    /// Remove a planet from the bridge
    pub fn remove_planet(&mut self, planet_id: PlanetId) {
        self.planets.remove(&planet_id);
        self.entity_assignments
            .retain(|_, assignment| assignment.planet_id != planet_id);
    }

    /// Compute distance between position and planet
    fn compute_distance(&self, pos: Position3D, planet_pos: Position3D) -> f64 {
        let dx = pos.x - planet_pos.x;
        let dy = pos.y - planet_pos.y;
        let dz = pos.z - planet_pos.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Assign entity to planet based on position
    pub fn assign_to_planet(
        &mut self,
        entity_id: u64,
        position: Position3D,
        current_step: usize,
    ) -> Option<PlanetId> {
        // Find closest planet within range
        let mut closest_planet: Option<(PlanetId, f64)> = None;

        for (planet_id, planet) in &self.planets {
            let distance = self.compute_distance(position, planet.position);

            if distance <= self.config.max_planet_distance {
                match &closest_planet {
                    Some((_, closest_dist)) if distance < *closest_dist => {
                        closest_planet = Some((*planet_id, distance));
                    }
                    None => {
                        closest_planet = Some((*planet_id, distance));
                    }
                    _ => {}
                }
            }
        }

        if let Some((planet_id, distance)) = closest_planet {
            let assignment = EntityPlanetAssignment {
                entity_id,
                planet_id,
                assigned_position: position,
                distance_from_center: distance,
                assignment_time: current_step,
            };

            self.entity_assignments.insert(entity_id, assignment);
            self.initialize_entity_state(entity_id, planet_id);

            Some(planet_id)
        } else {
            None
        }
    }

    /// Initialize entity state from planet environment
    fn initialize_entity_state(&mut self, entity_id: u64, planet_id: PlanetId) {
        if let Some(planet) = self.planets.get(&planet_id) {
            let base_temp = 20.0;
            let temp_from_atmosphere = planet.atmosphere.temperature_gradient * 30.0 - 15.0;
            let temperature = base_temp + temp_from_atmosphere;
            let oxygen_level = 0.21;

            let state = EntityEnvironmentState {
                planet_id: Some(planet_id),
                environment_id: Some(EnvironmentId(entity_id)),
                temperature,
                oxygen_level,
                weather: WeatherPattern::Clear,
                terrain: EnvironmentTerrain::Plains,
                movement_cost: 1.0,
                energy_expenditure: 1.0,
            };

            self.entity_states.insert(entity_id, state);
        }
    }

    /// Derive terrain type from planet and position
    fn derive_terrain(&self, _planet: &Planet, _distance_from_center: f64) -> EnvironmentTerrain {
        EnvironmentTerrain::Plains
    }

    /// Update entity assignment when position changes
    pub fn update_assignment(
        &mut self,
        entity_id: u64,
        new_position: Position3D,
        current_step: usize,
    ) {
        // First check if entity is assigned
        let current_info = self
            .entity_assignments
            .get(&entity_id)
            .map(|a| (a.planet_id, a.distance_from_center));

        let needs_reassign = match current_info {
            Some((_, dist)) => dist > self.config.max_planet_distance * 1.5,
            None => true,
        };

        if needs_reassign {
            self.assign_to_planet(entity_id, new_position, current_step);
        } else if let Some((planet_id, _)) = current_info {
            let planet_position = self.planets.get(&planet_id).map(|p| p.position);

            if let Some(planet_pos) = planet_position {
                let new_dist = self.compute_distance(new_position, planet_pos);

                if let Some(assignment) = self.entity_assignments.get_mut(&entity_id) {
                    assignment.assigned_position = new_position;
                    assignment.distance_from_center = new_dist;
                }
            }
        }
    }

    /// Get planet for entity
    pub fn get_planet(&self, entity_id: u64) -> Option<&Planet> {
        self.entity_assignments
            .get(&entity_id)
            .and_then(|a| self.planets.get(&a.planet_id))
    }

    /// Get environment state for entity
    pub fn get_environment_state(&self, entity_id: u64) -> Option<&EntityEnvironmentState> {
        self.entity_states.get(&entity_id)
    }

    /// Get environment state for entity (mutable)
    pub fn get_environment_state_mut(
        &mut self,
        entity_id: u64,
    ) -> Option<&mut EntityEnvironmentState> {
        self.entity_states.get_mut(&entity_id)
    }

    /// Apply planet effects to entity state
    pub fn apply_planet_effects(&mut self, entity_id: u64) {
        // Get planet info first
        let planet_info = self
            .entity_assignments
            .get(&entity_id)
            .map(|a| (a.planet_id, a.distance_from_center));

        if let Some((planet_id, distance)) = planet_info {
            if let Some(planet) = self.planets.get(&planet_id) {
                // Compute terrain first (before mutable borrow)
                let terrain = self.derive_terrain(planet, distance);

                if let Some(state) = self.entity_states.get_mut(&entity_id) {
                    let distance_ratio = distance / planet.radius;
                    let base_temp = 20.0 + planet.atmosphere.temperature_gradient * 30.0 - 15.0;
                    state.temperature = base_temp * (1.0 - distance_ratio.min(1.0) * 0.5);
                    state.terrain = terrain;
                }
            }
        }
    }

    /// Check if entity is assigned to a planet
    pub fn is_assigned(&self, entity_id: u64) -> bool {
        self.entity_assignments.contains_key(&entity_id)
    }

    /// Remove entity from bridge
    pub fn remove_entity(&mut self, entity_id: u64) {
        self.entity_assignments.remove(&entity_id);
        self.entity_states.remove(&entity_id);
    }

    /// Update statistics
    pub fn update_statistics(&mut self) {
        self.statistics.total_entities = self.entity_assignments.len() + self.entity_states.len();
        self.statistics.assigned_entities = self.entity_assignments.len();
        self.statistics.unassigned_entities =
            self.statistics.total_entities - self.entity_assignments.len();
        self.statistics.planet_count = self.planets.len();

        if !self.entity_assignments.is_empty() {
            let total_distance: f64 = self
                .entity_assignments
                .values()
                .map(|a| a.distance_from_center)
                .sum();
            self.statistics.average_planet_distance =
                total_distance / self.entity_assignments.len() as f64;
        }
    }

    /// Get statistics
    pub fn get_statistics(&self) -> &EntityBridgeStatistics {
        &self.statistics
    }

    /// Get all entity IDs assigned to a planet
    pub fn get_entities_on_planet(&self, planet_id: PlanetId) -> Vec<u64> {
        self.entity_assignments
            .iter()
            .filter(|(_, a)| a.planet_id == planet_id)
            .map(|(id, _)| *id)
            .collect()
    }
}

impl Default for EntityBridge {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_assignment() {
        let mut bridge = EntityBridge::new();
        let planet = Planet::default();
        bridge.add_planet(PlanetId(1), planet);

        let position = Position3D::new(0.0, 0.0, 0.0);
        let result = bridge.assign_to_planet(1, position, 0);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), PlanetId(1));
    }

    #[test]
    fn test_environment_state() {
        let mut bridge = EntityBridge::new();
        let planet = Planet::default();
        bridge.add_planet(PlanetId(1), planet);

        let position = Position3D::new(0.0, 0.0, 0.0);
        bridge.assign_to_planet(1, position, 0);

        let state = bridge.get_environment_state(1);
        assert!(state.is_some());

        let state = state.unwrap();
        assert_eq!(state.planet_id, Some(PlanetId(1)));
    }
}
