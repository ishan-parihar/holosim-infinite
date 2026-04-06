use crate::entity_layer7::layer7::EntityId;
use crate::types::Float;
use rapier3d_f64::parry::na::{Isometry3, Vector3};
use rapier3d_f64::prelude::{
    BroadPhaseMultiSap, CCDSolver, ColliderBuilder, ColliderHandle, ColliderSet, ImpulseJointSet,
    IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline,
    QueryPipeline, RigidBodyBuilder, RigidBodyHandle, RigidBodySet, RigidBodyType,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColliderShape {
    Sphere { radius: Float },
    Cuboid { half_extents: [Float; 3] },
    Capsule { half_height: Float, radius: Float },
    Cylinder { half_height: Float, radius: Float },
}

#[derive(Debug, Clone, Copy)]
pub struct EntityPhysicsHandle {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
}

pub struct PhysicsWorld {
    pub broad_phase: BroadPhaseMultiSap,
    pub narrow_phase: NarrowPhase,
    pub island_manager: IslandManager,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub query_pipeline: QueryPipeline,
    pub ccd_solver: CCDSolver,
    pub physics_pipeline: PhysicsPipeline,
    pub integration_params: IntegrationParameters,
    pub gravity: Vector3<Float>,
    pub entity_map: HashMap<EntityId, EntityPhysicsHandle>,
    pub collision_events: Vec<HolographicCollisionEvent>,
    pub contact_forces: Vec<ContactForceData>,
}

#[derive(Debug, Clone)]
pub struct HolographicCollisionEvent {
    pub entity_a: EntityId,
    pub entity_b: EntityId,
    pub contact_point: [Float; 3],
    pub contact_normal: [Float; 3],
    pub impulse: Float,
}

#[derive(Debug, Clone)]
pub struct ContactForceData {
    pub entity_a: EntityId,
    pub entity_b: EntityId,
    pub force_magnitude: Float,
    pub total_force: [Float; 3],
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let mut integration_params = IntegrationParameters::default();
        integration_params.dt = 1.0 / 60.0;

        Self {
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            island_manager: IslandManager::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            query_pipeline: QueryPipeline::new(),
            ccd_solver: CCDSolver::new(),
            physics_pipeline: PhysicsPipeline::new(),
            integration_params,
            gravity: Vector3::zeros(),
            entity_map: HashMap::new(),
            collision_events: Vec::new(),
            contact_forces: Vec::new(),
        }
    }

    pub fn create_entity_body(
        &mut self,
        entity_id: EntityId,
        mass: Float,
        position: [Float; 3],
        shape: ColliderShape,
    ) -> Option<EntityPhysicsHandle> {
        if self.entity_map.contains_key(&entity_id) {
            return None;
        }

        let translation = Vector3::new(position[0], position[1], position[2]);
        let isometry = Isometry3::translation(translation.x, translation.y, translation.z);

        let body_status = if mass > 0.0 {
            RigidBodyType::Dynamic
        } else {
            RigidBodyType::Fixed
        };

        let body = RigidBodyBuilder::new(body_status)
            .position(isometry)
            .build();

        let collider = match shape {
            ColliderShape::Sphere { radius } => ColliderBuilder::ball(radius),
            ColliderShape::Cuboid { half_extents } => {
                ColliderBuilder::cuboid(half_extents[0], half_extents[1], half_extents[2])
            }
            ColliderShape::Capsule {
                half_height,
                radius,
            } => ColliderBuilder::capsule_y(half_height, radius),
            ColliderShape::Cylinder {
                half_height,
                radius,
            } => ColliderBuilder::cylinder(half_height, radius),
        };

        let collider = if mass > 0.0 {
            collider.mass(mass).build()
        } else {
            collider.build()
        };
        let body_handle = self.rigid_body_set.insert(body);
        let collider_handle =
            self.collider_set
                .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);

        let handle = EntityPhysicsHandle {
            rigid_body: body_handle,
            collider: collider_handle,
        };

        self.entity_map.insert(entity_id, handle);
        Some(handle)
    }

    pub fn remove_entity_body(&mut self, entity_id: &EntityId) {
        if let Some(handle) = self.entity_map.remove(entity_id) {
            self.rigid_body_set.remove(
                handle.rigid_body,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
        }
    }

    pub fn step_physics(&mut self, delta_time: Option<Float>) {
        if let Some(dt) = delta_time {
            self.integration_params.dt = dt;
        }

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_params,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );

        self.collision_events.clear();
        self.contact_forces.clear();
    }

    pub fn get_entity_position(&self, entity_id: &EntityId) -> Option<[Float; 3]> {
        self.entity_map.get(entity_id).and_then(|handle| {
            self.rigid_body_set.get(handle.rigid_body).map(|body| {
                let t = body.translation();
                [t.x, t.y, t.z]
            })
        })
    }

    pub fn set_entity_position(&mut self, entity_id: &EntityId, position: [Float; 3]) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.set_translation(Vector3::new(position[0], position[1], position[2]), true);
            }
        }
    }

    pub fn apply_impulse(&mut self, entity_id: &EntityId, impulse: [Float; 3]) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.apply_impulse(Vector3::new(impulse[0], impulse[1], impulse[2]), true);
            }
        }
    }

    pub fn apply_force(&mut self, entity_id: &EntityId, force: [Float; 3]) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.add_force(Vector3::new(force[0], force[1], force[2]), true);
            }
        }
    }

    pub fn get_entity_velocity(&self, entity_id: &EntityId) -> Option<[Float; 3]> {
        self.entity_map.get(entity_id).and_then(|handle| {
            self.rigid_body_set.get(handle.rigid_body).map(|body| {
                let v = body.linvel();
                [v.x, v.y, v.z]
            })
        })
    }

    pub fn entity_count(&self) -> usize {
        self.entity_map.len()
    }

    pub fn collision_events(&self) -> &[HolographicCollisionEvent] {
        &self.collision_events
    }

    pub fn contact_forces(&self) -> &[ContactForceData] {
        &self.contact_forces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_entity_id(id: u64) -> EntityId {
        EntityId::new(format!("test-entity-{}", id))
    }

    #[test]
    fn test_physics_world_creation() {
        let world = PhysicsWorld::new();
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn test_create_entity_body_with_mass() {
        let mut world = PhysicsWorld::new();
        let entity_id = test_entity_id(1);

        let handle = world.create_entity_body(
            entity_id,
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        assert!(handle.is_some());
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_create_entity_body_fixed() {
        let mut world = PhysicsWorld::new();
        let entity_id = test_entity_id(2);

        let handle = world.create_entity_body(
            entity_id,
            0.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Cuboid {
                half_extents: [1.0, 1.0, 1.0],
            },
        );

        assert!(handle.is_some());
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_remove_entity_body() {
        let mut world = PhysicsWorld::new();
        let entity_id = test_entity_id(3);

        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );
        assert_eq!(world.entity_count(), 1);

        world.remove_entity_body(&entity_id);
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn test_entity_position_set_and_get() {
        let mut world = PhysicsWorld::new();
        let entity_id = test_entity_id(4);

        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        let pos = world.get_entity_position(&entity_id);
        assert!(pos.is_some());
        let pos = pos.unwrap();
        assert!((pos[0] - 0.0).abs() < 1e-10);
        assert!((pos[1] - 0.0).abs() < 1e-10);
        assert!((pos[2] - 0.0).abs() < 1e-10);

        world.set_entity_position(&entity_id, [5.0, 3.0, -2.0]);
        let new_pos = world.get_entity_position(&entity_id).unwrap();
        assert!((new_pos[0] - 5.0).abs() < 1e-10);
        assert!((new_pos[1] - 3.0).abs() < 1e-10);
        assert!((new_pos[2] - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_gravity_simulation() {
        let mut world = PhysicsWorld::new();
        world.gravity = Vector3::new(0.0, -9.81, 0.0);

        let entity_id = test_entity_id(5);
        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 10.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.step_physics(Some(1.0 / 60.0));

        let pos = world.get_entity_position(&entity_id).unwrap();
        assert!(pos[1] < 10.0);
    }

    #[test]
    fn test_impulse_application() {
        let mut world = PhysicsWorld::new();
        let entity_id = test_entity_id(6);

        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.apply_impulse(&entity_id, [10.0, 0.0, 0.0]);
        world.step_physics(Some(1.0 / 60.0));

        let vel = world.get_entity_velocity(&entity_id).unwrap();
        assert!(vel[0] > 0.0);
    }

    #[test]
    fn test_multiple_collider_shapes() {
        let mut world = PhysicsWorld::new();

        let shapes = vec![
            ColliderShape::Sphere { radius: 1.0 },
            ColliderShape::Cuboid {
                half_extents: [1.0, 2.0, 0.5],
            },
            ColliderShape::Capsule {
                half_height: 1.0,
                radius: 0.5,
            },
            ColliderShape::Cylinder {
                half_height: 1.0,
                radius: 0.5,
            },
        ];

        for (i, shape) in shapes.into_iter().enumerate() {
            let entity_id = test_entity_id((i + 10) as u64);
            let handle = world.create_entity_body(entity_id, 1.0, [0.0, 0.0, 0.0], shape);
            assert!(handle.is_some());
        }

        assert_eq!(world.entity_count(), 4);
    }

    #[test]
    fn test_duplicate_entity_rejected() {
        let mut world = PhysicsWorld::new();
        let entity_id = test_entity_id(20);

        let first = world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );
        assert!(first.is_some());

        let second = world.create_entity_body(
            entity_id.clone(),
            2.0,
            [1.0, 1.0, 1.0],
            ColliderShape::Sphere { radius: 1.0 },
        );
        assert!(second.is_none());
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn test_collision_between_two_bodies() {
        let mut world = PhysicsWorld::new();
        world.gravity = Vector3::new(0.0, 0.0, 0.0);

        let entity_a = test_entity_id(30);
        let entity_b = test_entity_id(31);

        world.create_entity_body(
            entity_a.clone(),
            1.0,
            [-2.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 1.0 },
        );
        world.create_entity_body(
            entity_b.clone(),
            1.0,
            [2.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 1.0 },
        );

        world.apply_impulse(&entity_a, [5.0, 0.0, 0.0]);
        world.apply_impulse(&entity_b, [-5.0, 0.0, 0.0]);

        for _ in 0..60 {
            world.step_physics(Some(1.0 / 60.0));
        }

        let pos_a = world.get_entity_position(&entity_a).unwrap();
        let pos_b = world.get_entity_position(&entity_b).unwrap();

        let dist = ((pos_a[0] - pos_b[0]).powi(2)).sqrt();
        assert!(
            dist >= 1.8,
            "Bodies should be separated after collision, dist={}",
            dist
        );
    }
}
