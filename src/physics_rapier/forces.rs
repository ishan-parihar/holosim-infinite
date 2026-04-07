use super::{ColliderShape, EntityId, Float, PhysicsWorld};

impl PhysicsWorld {
    pub fn set_gravity(&mut self, gravity: [Float; 3]) {
        self.gravity = rapier3d_f64::na::Vector3::new(gravity[0], gravity[1], gravity[2]);
    }

    pub fn apply_gravity_to_entity(&mut self, entity_id: &EntityId, gravity_strength: Float) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                let mass = body.mass();
                if mass > 0.0 {
                    let force = mass * gravity_strength;
                    body.add_force(rapier3d_f64::na::Vector3::new(0.0, -force, 0.0), true);
                }
            }
        }
    }

    pub fn apply_custom_force(
        &mut self,
        entity_id: &EntityId,
        force: [Float; 3],
        at_point: Option<[Float; 3]>,
    ) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                let force_vec = rapier3d_f64::na::Vector3::new(force[0], force[1], force[2]);

                match at_point {
                    Some(point) => {
                        let point_vec = rapier3d_f64::na::Point3::new(point[0], point[1], point[2]);
                        body.add_force_at_point(force_vec, point_vec, true);
                    }
                    None => {
                        body.add_force(force_vec, true);
                    }
                }
            }
        }
    }

    pub fn apply_torque(&mut self, entity_id: &EntityId, torque: [Float; 3]) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.add_torque(
                    rapier3d_f64::na::Vector3::new(torque[0], torque[1], torque[2]),
                    true,
                );
            }
        }
    }

    pub fn set_entity_velocity(&mut self, entity_id: &EntityId, velocity: [Float; 3]) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.set_linvel(
                    rapier3d_f64::na::Vector3::new(velocity[0], velocity[1], velocity[2]),
                    true,
                );
            }
        }
    }

    pub fn set_entity_angular_velocity(&mut self, entity_id: &EntityId, angular_vel: [Float; 3]) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.set_angvel(
                    rapier3d_f64::na::Vector3::new(angular_vel[0], angular_vel[1], angular_vel[2]),
                    true,
                );
            }
        }
    }

    pub fn apply_damping(
        &mut self,
        entity_id: &EntityId,
        linear_damping: Float,
        angular_damping: Float,
    ) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.set_linear_damping(linear_damping);
                body.set_angular_damping(angular_damping);
            }
        }
    }

    pub fn get_entity_kinetic_energy(&self, entity_id: &EntityId) -> Option<Float> {
        self.entity_map.get(entity_id).and_then(|handle| {
            self.rigid_body_set.get(handle.rigid_body).map(|body| {
                let mass = body.mass();
                let vel = body.linvel();
                0.5 * mass * vel.norm_squared()
            })
        })
    }

    pub fn get_entity_momentum(&self, entity_id: &EntityId) -> Option<[Float; 3]> {
        self.entity_map.get(entity_id).and_then(|handle| {
            self.rigid_body_set.get(handle.rigid_body).map(|body| {
                let mass = body.mass();
                let vel = body.linvel();
                [mass * vel.x, mass * vel.y, mass * vel.z]
            })
        })
    }

    pub fn enable_ccd(&mut self, entity_id: &EntityId, enabled: bool) {
        if let Some(handle) = self.entity_map.get(entity_id) {
            if let Some(body) = self.rigid_body_set.get_mut(handle.rigid_body) {
                body.enable_ccd(enabled);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColliderShape;
    use super::*;
    use crate::entity_layer7::layer7::EntityId;

    fn test_entity_id(id: u64) -> EntityId {
        EntityId::new(format!("test-force-{}", id))
    }

    #[test]
    fn test_set_gravity() {
        let mut world = PhysicsWorld::new();
        world.set_gravity([0.0, -19.62, 0.0]);

        let entity_id = test_entity_id(200);
        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 10.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.step_physics(Some(1.0 / 60.0));

        let vel = world.get_entity_velocity(&entity_id).unwrap();
        assert!(vel[1] < 0.0);
    }

    #[test]
    fn test_apply_custom_force() {
        let mut world = PhysicsWorld::new();
        world.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_id = test_entity_id(201);
        world.create_entity_body(
            entity_id.clone(),
            2.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.apply_custom_force(&entity_id, [20.0, 0.0, 0.0], None);
        world.step_physics(Some(1.0 / 60.0));

        let vel = world.get_entity_velocity(&entity_id).unwrap();
        assert!(vel[0] > 0.0);
    }

    #[test]
    fn test_apply_force_at_point() {
        let mut world = PhysicsWorld::new();
        world.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_id = test_entity_id(202);
        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Cuboid {
                half_extents: [1.0, 1.0, 1.0],
            },
        );

        world.apply_custom_force(&entity_id, [10.0, 0.0, 0.0], Some([1.0, 0.0, 0.0]));
        world.step_physics(Some(1.0 / 60.0));

        let vel = world.get_entity_velocity(&entity_id).unwrap();
        assert!(vel[0] > 0.0);
    }

    #[test]
    fn test_damping_reduces_velocity() {
        let mut world = PhysicsWorld::new();
        world.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_id = test_entity_id(203);
        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.set_entity_velocity(&entity_id, [100.0, 0.0, 0.0]);
        world.apply_damping(&entity_id, 0.5, 0.0);

        for _ in 0..10 {
            world.step_physics(Some(1.0 / 60.0));
        }

        let vel = world.get_entity_velocity(&entity_id).unwrap();
        assert!(vel[0] < 100.0);
    }

    #[test]
    fn test_kinetic_energy_calculation() {
        let mut world = PhysicsWorld::new();
        world.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_id = test_entity_id(204);
        world.create_entity_body(
            entity_id.clone(),
            2.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.set_entity_velocity(&entity_id, [3.0, 4.0, 0.0]);

        let ke = world.get_entity_kinetic_energy(&entity_id).unwrap();
        let expected_ke = 0.5 * 2.0 * (3.0 * 3.0 + 4.0 * 4.0);
        assert!((ke - expected_ke).abs() < 0.01);
    }

    #[test]
    fn test_momentum_calculation() {
        let mut world = PhysicsWorld::new();
        world.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_id = test_entity_id(205);
        world.create_entity_body(
            entity_id.clone(),
            3.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.set_entity_velocity(&entity_id, [2.0, 0.0, 0.0]);

        let momentum = world.get_entity_momentum(&entity_id).unwrap();
        assert!((momentum[0] - 6.0).abs() < 0.01);
        assert!((momentum[1] - 0.0).abs() < 0.01);
        assert!((momentum[2] - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_torque_application() {
        let mut world = PhysicsWorld::new();
        world.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_id = test_entity_id(206);
        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Cuboid {
                half_extents: [1.0, 1.0, 1.0],
            },
        );

        world.apply_torque(&entity_id, [0.0, 10.0, 0.0]);
        world.step_physics(Some(1.0 / 60.0));
    }

    #[test]
    fn test_ccd_toggle() {
        let mut world = PhysicsWorld::new();

        let entity_id = test_entity_id(207);
        world.create_entity_body(
            entity_id.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.enable_ccd(&entity_id, true);
        world.enable_ccd(&entity_id, false);
    }
}
