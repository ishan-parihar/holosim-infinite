//! Rapier 3D Physics World Wrapper (Phase 6.3)
//!
//! Wraps `rapier3d` to simulate physical properties derived from archetype
//! activations as rigid bodies. Part of the physics unification pipeline:
//! archetype-derived forces → Rapier rigid-body physics → WGPU rendering.
//!
//! From `.sisyphus/plans/physics-unification.md`:
//! "Connect archetype-derived forces → Rapier rigid-body physics → WGPU rendering"

use std::collections::HashMap;

use rapier3d::prelude::*;

use crate::physics::archetype_physics::{ArchetypePhysicsMapper, Force3D, PhysicalProperties};

#[derive(Debug)]
pub struct PhysicsEntity {
    pub entity_id: u64,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityPhysicsState {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub linear_acceleration: [f64; 3],
}

pub struct PhysicsWorld {
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub mapper: ArchetypePhysicsMapper,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    entity_handles: HashMap<u64, RigidBodyHandle>,
    collider_handles: HashMap<u64, ColliderHandle>,
    entity_charges: HashMap<u64, f64>,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let gravity = Vector::new(0.0, -9.81, 0.0);
        let integration_parameters = IntegrationParameters::default();

        Self {
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            mapper: ArchetypePhysicsMapper::new(),
            gravity,
            integration_parameters,
            entity_handles: HashMap::new(),
            collider_handles: HashMap::new(),
            entity_charges: HashMap::new(),
        }
    }

    pub fn add_entity(
        &mut self,
        entity_id: u64,
        position: [f64; 3],
        properties: &PhysicalProperties,
    ) -> PhysicsEntity {
        let pos_f32 = Vector::new(position[0] as f32, position[1] as f32, position[2] as f32);

        let mass = properties.mass.max(1e-6) as f32;
        let radius = 0.5 * mass.powf(1.0 / 3.0);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(pos_f32)
            .linvel(Vector::zeros())
            .linear_damping(properties.damping as f32)
            .additional_mass(mass)
            .build();

        let rb_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(radius).mass(mass).build();
        let collider_handle =
            self.collider_set
                .insert_with_parent(collider, rb_handle, &mut self.rigid_body_set);

        self.entity_handles.insert(entity_id, rb_handle);
        self.collider_handles.insert(entity_id, collider_handle);
        self.entity_charges.insert(entity_id, properties.charge);

        PhysicsEntity {
            entity_id,
            rigid_body_handle: rb_handle,
            collider_handle,
        }
    }

    pub fn remove_entity(&mut self, entity_id: u64) {
        if let Some(&rb_handle) = self.entity_handles.get(&entity_id) {
            if let Some(&collider_handle) = self.collider_handles.get(&entity_id) {
                self.collider_set.remove(
                    collider_handle,
                    &mut self.island_manager,
                    &mut self.rigid_body_set,
                    true,
                );
                self.collider_handles.remove(&entity_id);
            }

            self.rigid_body_set.remove(
                rb_handle,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
            self.entity_handles.remove(&entity_id);
            self.entity_charges.remove(&entity_id);
        }
    }

    pub fn apply_archetype_force(&mut self, entity_id: u64, force: &Force3D) {
        if let Some(&handle) = self.entity_handles.get(&entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle) {
                body.add_force(
                    Vector::new(force.fx as f32, force.fy as f32, force.fz as f32),
                    true,
                );
            }
        }
    }

    /// Apply electromagnetic force between two entities (Coulomb's law).
    ///
    /// F = k * q₁ * q₂ / r² along the axis connecting the two bodies.
    /// Positive force = repulsion, negative = attraction.
    pub fn apply_electromagnetic_force(&mut self, entity_id_a: u64, entity_id_b: u64) {
        let ha = match self.entity_handles.get(&entity_id_a) {
            Some(h) => *h,
            None => return,
        };
        let hb = match self.entity_handles.get(&entity_id_b) {
            Some(h) => *h,
            None => return,
        };
        let charge_a = match self.entity_charges.get(&entity_id_a) {
            Some(c) => *c,
            None => return,
        };
        let charge_b = match self.entity_charges.get(&entity_id_b) {
            Some(c) => *c,
            None => return,
        };

        let (pos_a, pos_b) = {
            let body_a = match self.rigid_body_set.get(ha) {
                Some(b) => b,
                None => return,
            };
            let pa = [
                body_a.translation().x as f64,
                body_a.translation().y as f64,
                body_a.translation().z as f64,
            ];
            let body_b = match self.rigid_body_set.get(hb) {
                Some(b) => b,
                None => return,
            };
            let pb = [
                body_b.translation().x as f64,
                body_b.translation().y as f64,
                body_b.translation().z as f64,
            ];
            (pa, pb)
        };

        let dx = pos_b[0] - pos_a[0];
        let dy = pos_b[1] - pos_a[1];
        let dz = pos_b[2] - pos_a[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        let dist = dist_sq.sqrt();

        if dist < 1e-10 {
            return;
        }

        let force_magnitude = self.mapper.electromagnetic_force(charge_a, charge_b, dist);

        let ux = dx / dist;
        let uy = dy / dist;
        let uz = dz / dist;

        let f_a = Vector::new(
            -(force_magnitude as f32 * ux as f32),
            -(force_magnitude as f32 * uy as f32),
            -(force_magnitude as f32 * uz as f32),
        );

        let f_b = Vector::new(
            force_magnitude as f32 * ux as f32,
            force_magnitude as f32 * uy as f32,
            force_magnitude as f32 * uz as f32,
        );

        if let Some(body_a) = self
            .rigid_body_set
            .get_mut(self.entity_handles[&entity_id_a])
        {
            body_a.add_force(f_a, true);
        }
        if let Some(body_b) = self
            .rigid_body_set
            .get_mut(self.entity_handles[&entity_id_b])
        {
            body_b.add_force(f_b, true);
        }
    }

    /// Apply gravitational force between two entities (Newton's law).
    ///
    /// F = G * m₁ * m₂ / r² along the axis connecting the two bodies.
    /// Always attractive.
    pub fn apply_gravitational_force(&mut self, entity_id_a: u64, entity_id_b: u64) {
        let ha = match self.entity_handles.get(&entity_id_a) {
            Some(h) => *h,
            None => return,
        };
        let hb = match self.entity_handles.get(&entity_id_b) {
            Some(h) => *h,
            None => return,
        };

        let (pos_a, pos_b, mass_a, mass_b) = {
            let body_a = match self.rigid_body_set.get(ha) {
                Some(b) => b,
                None => return,
            };
            let ma = body_a.mass();
            let pa = [
                body_a.translation().x as f64,
                body_a.translation().y as f64,
                body_a.translation().z as f64,
            ];
            let body_b = match self.rigid_body_set.get(hb) {
                Some(b) => b,
                None => return,
            };
            let mb = body_b.mass();
            let pb = [
                body_b.translation().x as f64,
                body_b.translation().y as f64,
                body_b.translation().z as f64,
            ];
            (pa, pb, ma as f64, mb as f64)
        };

        let dx = pos_b[0] - pos_a[0];
        let dy = pos_b[1] - pos_a[1];
        let dz = pos_b[2] - pos_a[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        let dist = dist_sq.sqrt();

        if dist < 1e-10 {
            return;
        }

        let force_magnitude = self.mapper.gravitational_force(mass_a, mass_b, dist);

        let ux = dx / dist;
        let uy = dy / dist;
        let uz = dz / dist;

        let f_a = Vector::new(
            (force_magnitude as f32) * ux as f32,
            (force_magnitude as f32) * uy as f32,
            (force_magnitude as f32) * uz as f32,
        );
        let f_b = Vector::new(
            -(force_magnitude as f32) * ux as f32,
            -(force_magnitude as f32) * uy as f32,
            -(force_magnitude as f32) * uz as f32,
        );

        if let Some(body_a) = self
            .rigid_body_set
            .get_mut(self.entity_handles[&entity_id_a])
        {
            body_a.add_force(f_a, true);
        }
        if let Some(body_b) = self
            .rigid_body_set
            .get_mut(self.entity_handles[&entity_id_b])
        {
            body_b.add_force(f_b, true);
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.integration_parameters.dt = dt as f32;

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }

    pub fn get_entity_state(&self, entity_id: u64) -> Option<EntityPhysicsState> {
        let handle = self.entity_handles.get(&entity_id)?;
        let body = self.rigid_body_set.get(*handle)?;

        let position = [
            body.translation().x as f64,
            body.translation().y as f64,
            body.translation().z as f64,
        ];

        let velocity = [
            body.linvel().x as f64,
            body.linvel().y as f64,
            body.linvel().z as f64,
        ];

        let mass = body.mass();
        let net_force = body.user_force();
        let linear_acceleration = if mass > 0.0 {
            [
                (net_force.x as f64) / mass as f64,
                (net_force.y as f64) / mass as f64,
                (net_force.z as f64) / mass as f64,
            ]
        } else {
            [0.0, 0.0, 0.0]
        };

        Some(EntityPhysicsState {
            position,
            velocity,
            linear_acceleration,
        })
    }

    pub fn entity_count(&self) -> usize {
        self.entity_handles.len()
    }

    /// Returns all entity IDs managed by this physics world.
    pub fn entity_ids(&self) -> impl Iterator<Item = &u64> {
        self.entity_handles.keys()
    }

    /// Access to the narrow phase for collision detection.
    pub fn narrow_phase(&self) -> &NarrowPhase {
        &self.narrow_phase
    }

    /// Returns the collider handle for a given entity ID, if it exists.
    pub fn get_collider_handle(&self, entity_id: u64) -> Option<ColliderHandle> {
        self.collider_handles.get(&entity_id).copied()
    }

    /// Compute reduced mass between two entities: (m₁·m₂)/(m₁+m₂).
    pub fn get_reduced_mass(&self, entity_a: u64, entity_b: u64) -> f64 {
        let mass_a = self
            .entity_handles
            .get(&entity_a)
            .and_then(|&h| self.rigid_body_set.get(h))
            .map(|b| b.mass() as f64)
            .unwrap_or(1.0);
        let mass_b = self
            .entity_handles
            .get(&entity_b)
            .and_then(|&h| self.rigid_body_set.get(h))
            .map(|b| b.mass() as f64)
            .unwrap_or(1.0);
        let sum = mass_a + mass_b;
        if sum > 0.0 {
            (mass_a * mass_b) / sum
        } else {
            0.0
        }
    }

    /// Apply velocity to an entity's rigid body (for test setup).
    pub fn set_entity_velocity(&mut self, entity_id: u64, velocity: [f64; 3]) {
        if let Some(&handle) = self.entity_handles.get(&entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle) {
                body.set_linvel(
                    Vector::new(velocity[0] as f32, velocity[1] as f32, velocity[2] as f32),
                    true,
                );
            }
        }
    }

    /// Set gravity (useful for tests and zero-g scenarios).
    pub fn set_gravity(&mut self, x: f32, y: f32, z: f32) {
        self.gravity = Vector::new(x, y, z);
    }

    pub fn get_entity_properties(&self, entity_id: u64) -> Option<PhysicalProperties> {
        let handle = self.entity_handles.get(&entity_id)?;
        let body = self.rigid_body_set.get(*handle)?;

        let mass = body.mass() as f64;
        let charge = self.entity_charges.get(&entity_id).copied().unwrap_or(0.0);
        let damping = body.linear_damping() as f64;

        let moment_of_inertia = 0.4 * mass;
        let spin = 0.0;

        let net_force = body.user_force();
        let force_vector = Force3D::new(net_force.x as f64, net_force.y as f64, net_force.z as f64);

        Some(PhysicalProperties {
            mass,
            charge,
            spin,
            damping,
            moment_of_inertia,
            force_vector,
        })
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_properties() -> PhysicalProperties {
        PhysicalProperties {
            mass: 1.0,
            charge: 0.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 0.4,
            force_vector: Force3D::new(0.0, 0.0, 0.0),
        }
    }

    #[test]
    fn test_create_world() {
        let world = PhysicsWorld::new();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut world = PhysicsWorld::new();
        let props = test_properties();

        let entity = world.add_entity(1, [0.0, 0.0, 0.0], &props);
        assert_eq!(entity.entity_id, 1);
        assert_eq!(world.entity_count(), 1);

        let state = world.get_entity_state(1);
        assert!(state.is_some());
        let state = state.unwrap();
        assert!((state.position[0] - 0.0).abs() < 1e-6);
        assert!((state.position[1] - 0.0).abs() < 1e-6);
        assert!((state.position[2] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_remove_entity() {
        let mut world = PhysicsWorld::new();
        let props = test_properties();

        world.add_entity(1, [0.0, 0.0, 0.0], &props);
        assert_eq!(world.entity_count(), 1);

        world.remove_entity(1);
        assert_eq!(world.entity_count(), 0);

        assert!(world.get_entity_state(1).is_none());
    }

    #[test]
    fn test_step_advances_time() {
        let mut world = PhysicsWorld::new();
        let props = PhysicalProperties {
            mass: 1.0,
            charge: 0.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 0.4,
            force_vector: Force3D::new(0.0, 0.0, 0.0),
        };

        world.add_entity(1, [0.0, 10.0, 0.0], &props);

        let handle = world.entity_handles[&1];
        if let Some(body) = world.rigid_body_set.get_mut(handle) {
            body.set_linvel(Vector::new(0.0, -5.0, 0.0), true);
        }

        let state_before = world.get_entity_state(1).unwrap();
        let y_before = state_before.position[1];

        world.step(0.1);

        let state_after = world.get_entity_state(1).unwrap();
        let y_after = state_after.position[1];

        assert!(
            (y_after - y_before).abs() > 1e-6,
            "Position should change after step: before={}, after={}",
            y_before,
            y_after
        );
    }

    #[test]
    fn test_apply_force_changes_velocity() {
        let mut world = PhysicsWorld::new();
        let props = test_properties();

        world.add_entity(1, [0.0, 0.0, 0.0], &props);

        let state_before = world.get_entity_state(1).unwrap();
        let vx_before = state_before.velocity[0];

        world.apply_archetype_force(1, &Force3D::new(10.0, 0.0, 0.0));

        world.step(0.016);

        let state_after = world.get_entity_state(1).unwrap();
        let vx_after = state_after.velocity[0];

        assert!(
            (vx_after - vx_before).abs() > 1e-6,
            "Velocity should change after applying force: before={}, after={}",
            vx_before,
            vx_after
        );
    }

    #[test]
    fn test_gravitational_attraction() {
        let mut world = PhysicsWorld::new();
        world.gravity = Vector::zeros();
        world.mapper.gravitational_constant = 1.0;

        let props_a = PhysicalProperties {
            mass: 100.0,
            charge: 0.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 40.0,
            force_vector: Force3D::new(0.0, 0.0, 0.0),
        };
        let props_b = props_a;

        world.add_entity(1, [0.0, 0.0, 0.0], &props_a);
        world.add_entity(2, [10.0, 0.0, 0.0], &props_b);

        let pos_a_before = world.get_entity_state(1).unwrap().position[0];
        let pos_b_before = world.get_entity_state(2).unwrap().position[0];

        for _ in 0..100 {
            world.apply_gravitational_force(1, 2);
            world.step(0.1);
        }

        let pos_a_after = world.get_entity_state(1).unwrap().position[0];
        let pos_b_after = world.get_entity_state(2).unwrap().position[0];

        assert!(
            pos_a_after > pos_a_before,
            "Entity A should move toward B: {} > {}",
            pos_a_after,
            pos_a_before
        );
        assert!(
            pos_b_after < pos_b_before,
            "Entity B should move toward A: {} < {}",
            pos_b_after,
            pos_b_before
        );
    }

    #[test]
    fn test_electromagnetic_repulsion() {
        let mut world = PhysicsWorld::new();
        world.gravity = Vector::zeros();

        let props_a = PhysicalProperties {
            mass: 1.0,
            charge: 1.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 0.4,
            force_vector: Force3D::new(0.0, 0.0, 0.0),
        };
        let props_b = PhysicalProperties {
            mass: 1.0,
            charge: 1.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 0.4,
            force_vector: Force3D::new(0.0, 0.0, 0.0),
        };

        world.add_entity(1, [0.0, 0.0, 0.0], &props_a);
        world.add_entity(2, [1.0, 0.0, 0.0], &props_b);

        let pos_a_before = world.get_entity_state(1).unwrap().position[0];
        let pos_b_before = world.get_entity_state(2).unwrap().position[0];

        for _ in 0..50 {
            world.apply_electromagnetic_force(1, 2);
            world.step(0.001);
        }

        let pos_a_after = world.get_entity_state(1).unwrap().position[0];
        let pos_b_after = world.get_entity_state(2).unwrap().position[0];

        assert!(
            pos_a_after < pos_a_before,
            "Entity A should move away from B (repulsion): {} < {}",
            pos_a_after,
            pos_a_before
        );
        assert!(
            pos_b_after > pos_b_before,
            "Entity B should move away from A (repulsion): {} > {}",
            pos_b_after,
            pos_b_before
        );
    }
}
