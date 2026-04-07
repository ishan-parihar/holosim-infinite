//! Observation Sync — Bridge between Rapier physics state and the Observation Layer.
//!
//! Extracts positions/velocities from Rapier rigid bodies, updates
//! `PhysicalObservation` entries, and tracks collision events from the narrow phase.
//!
//! From Phase 6.4 physics unification: archetype-derived forces → Rapier → observation.

use std::collections::HashSet;

use rapier3d::prelude::*;

use crate::physics::physics_world::{EntityPhysicsState, PhysicsWorld};
use crate::simulation_v3::observation_layer::ObservationLayer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionEvent {
    pub entity_a: u64,
    pub entity_b: u64,
    pub impact_force: f64,
    pub relative_velocity: f64,
    pub timestamp: u64,
}

pub struct ObservationSync {
    pub synced_entities: HashSet<u64>,
    pub collision_events: Vec<CollisionEvent>,
}

impl ObservationSync {
    pub fn new() -> Self {
        Self {
            synced_entities: HashSet::new(),
            collision_events: Vec::new(),
        }
    }

    pub fn sync_physics_to_observations(
        &mut self,
        physics_world: &PhysicsWorld,
        observation_layer: &mut ObservationLayer,
        current_tick: u64,
    ) {
        self.synced_entities.clear();

        for &entity_id in physics_world.entity_ids() {
            if let Some(physics_state) = physics_world.get_entity_state(entity_id) {
                self.update_entity_physics(entity_id, &physics_state, observation_layer);
                self.synced_entities.insert(entity_id);
            }
        }

        self.detect_collisions(physics_world.narrow_phase(), physics_world, current_tick);
    }

    pub fn update_entity_physics(
        &self,
        entity_id: u64,
        physics_state: &EntityPhysicsState,
        observation_layer: &mut ObservationLayer,
    ) {
        observation_layer.update_physical_observation(
            entity_id,
            physics_state.position,
            physics_state.velocity,
            1.0,
            0.0,
            0.0,
            1.0,
            1.0,
        );
    }

    pub fn detect_collisions(
        &mut self,
        narrow_phase: &NarrowPhase,
        physics_world: &PhysicsWorld,
        current_tick: u64,
    ) {
        self.collision_events.clear();

        let collider_to_entity: std::collections::HashMap<ColliderHandle, u64> = physics_world
            .entity_ids()
            .filter_map(|&eid| physics_world.get_collider_handle(eid).map(|h| (h, eid)))
            .collect();

        for contact_pair in narrow_phase.contact_pairs() {
            let handle1 = contact_pair.collider1;
            let handle2 = contact_pair.collider2;

            let entity_a = match collider_to_entity.get(&handle1) {
                Some(&eid) => eid,
                None => continue,
            };
            let entity_b = match collider_to_entity.get(&handle2) {
                Some(&eid) => eid,
                None => continue,
            };

            let (state_a, state_b) = match (
                physics_world.get_entity_state(entity_a),
                physics_world.get_entity_state(entity_b),
            ) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };

            let rel_vel = relative_velocity_3d(&state_a.velocity, &state_b.velocity);
            let reduced_mass = physics_world.get_reduced_mass(entity_a, entity_b);
            let impact_force = rel_vel * reduced_mass;

            self.collision_events.push(CollisionEvent {
                entity_a,
                entity_b,
                impact_force,
                relative_velocity: rel_vel,
                timestamp: current_tick,
            });
        }
    }

    pub fn take_collision_events(&mut self) -> Vec<CollisionEvent> {
        std::mem::take(&mut self.collision_events)
    }
}

impl Default for ObservationSync {
    fn default() -> Self {
        Self::new()
    }
}

fn relative_velocity_3d(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::archetype_physics::PhysicalProperties;

    fn test_properties() -> PhysicalProperties {
        PhysicalProperties {
            mass: 1.0,
            charge: 0.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 0.4,
            force_vector: crate::physics::archetype_physics::Force3D::new(0.0, 0.0, 0.0),
        }
    }

    #[test]
    fn test_sync_updates_position() {
        let mut physics_world = PhysicsWorld::new();
        let mut observation_layer = ObservationLayer::new();
        let mut sync = ObservationSync::new();

        let props = test_properties();
        physics_world.add_entity(1, [5.0, 10.0, 15.0], &props);

        let profile = [0.5; 22];
        observation_layer.observe_entity(1, &profile, 0.5, 1, [0.0; 3], [0.0; 3], 1.0, 0.5, 0.5);

        sync.sync_physics_to_observations(&physics_world, &mut observation_layer, 1);

        let obs = observation_layer.get_all_observations();
        let entity_obs = obs.get(&1).expect("entity 1 should exist");

        assert!(
            (entity_obs.physical.position[0] - 5.0).abs() < 1e-3,
            "X position should match physics: got {}",
            entity_obs.physical.position[0]
        );
        assert!(
            (entity_obs.physical.position[1] - 10.0).abs() < 1e-3,
            "Y position should match physics: got {}",
            entity_obs.physical.position[1]
        );
        assert!(
            (entity_obs.physical.position[2] - 15.0).abs() < 1e-3,
            "Z position should match physics: got {}",
            entity_obs.physical.position[2]
        );
    }

    #[test]
    fn test_collision_detection() {
        let mut physics_world = PhysicsWorld::new();
        physics_world.set_gravity(0.0, 0.0, 0.0);

        let props = PhysicalProperties {
            mass: 10.0,
            charge: 0.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 4.0,
            force_vector: crate::physics::archetype_physics::Force3D::new(0.0, 0.0, 0.0),
        };

        physics_world.add_entity(1, [0.0, 0.0, 0.0], &props);
        physics_world.add_entity(2, [0.8, 0.0, 0.0], &props);

        physics_world.set_entity_velocity(1, [5.0, 0.0, 0.0]);

        for _ in 0..20 {
            physics_world.step(0.016);
        }

        let mut sync = ObservationSync::new();
        sync.detect_collisions(physics_world.narrow_phase(), &physics_world, 1);

        assert!(
            !sync.collision_events.is_empty(),
            "Entities close enough should generate collision events"
        );
    }

    #[test]
    fn test_take_events_clears() {
        let mut sync = ObservationSync::new();
        sync.collision_events.push(CollisionEvent {
            entity_a: 1,
            entity_b: 2,
            impact_force: 5.0,
            relative_velocity: 5.0,
            timestamp: 1,
        });

        let events = sync.take_collision_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].entity_a, 1);
        assert_eq!(events[0].entity_b, 2);
        assert!(
            sync.collision_events.is_empty(),
            "take_collision_events should clear the buffer"
        );
    }

    #[test]
    fn test_no_collision_no_events() {
        let mut physics_world = PhysicsWorld::new();
        physics_world.set_gravity(0.0, 0.0, 0.0);

        let props = test_properties();
        physics_world.add_entity(1, [0.0, 0.0, 0.0], &props);
        physics_world.add_entity(2, [100.0, 100.0, 100.0], &props);

        let mut sync = ObservationSync::new();
        sync.detect_collisions(physics_world.narrow_phase(), &physics_world, 1);

        assert!(
            sync.collision_events.is_empty(),
            "Distant entities should not generate collision events"
        );
    }

    #[test]
    fn test_sync_skips_missing_entities() {
        let mut physics_world = PhysicsWorld::new();
        let mut observation_layer = ObservationLayer::new();
        let mut sync = ObservationSync::new();

        let profile = [0.5; 22];
        observation_layer.observe_entity(999, &profile, 0.5, 1, [0.0; 3], [0.0; 3], 1.0, 0.5, 0.5);

        sync.sync_physics_to_observations(&physics_world, &mut observation_layer, 1);

        assert!(
            !sync.synced_entities.contains(&999),
            "Entity 999 should not be synced (not in physics world)"
        );
    }

    #[test]
    fn test_update_entity_physics_graceful_missing() {
        let mut observation_layer = ObservationLayer::new();
        let sync = ObservationSync::new();

        let physics_state = EntityPhysicsState {
            position: [1.0, 2.0, 3.0],
            velocity: [0.1, 0.2, 0.3],
            linear_acceleration: [0.0, 0.0, 0.0],
        };

        let result = observation_layer.update_physical_observation(
            42,
            physics_state.position,
            physics_state.velocity,
            1.0,
            0.0,
            0.0,
            1.0,
            1.0,
        );

        assert!(!result, "Updating non-existent entity should return false");
    }

    #[test]
    fn test_collision_event_fields() {
        let evt = CollisionEvent {
            entity_a: 10,
            entity_b: 20,
            impact_force: 15.5,
            relative_velocity: 3.1,
            timestamp: 42,
        };

        assert_eq!(evt.entity_a, 10);
        assert_eq!(evt.entity_b, 20);
        assert!((evt.impact_force - 15.5).abs() < 1e-6);
        assert!((evt.relative_velocity - 3.1).abs() < 1e-6);
        assert_eq!(evt.timestamp, 42);
    }

    #[test]
    fn test_relative_velocity_3d() {
        let a = [3.0, 0.0, 0.0];
        let b = [0.0, 4.0, 0.0];
        let rel = relative_velocity_3d(&a, &b);
        assert!((rel - 5.0).abs() < 1e-6, "3-4-5 triangle: got {}", rel);

        let same = [1.0, 2.0, 3.0];
        assert!(
            (relative_velocity_3d(&same, &same) - 0.0).abs() < 1e-10,
            "Same velocity should yield zero relative velocity"
        );
    }
}
