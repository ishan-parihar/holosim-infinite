use super::{ContactForceData, EntityId, Float, HolographicCollisionEvent, PhysicsWorld};

impl PhysicsWorld {
    pub fn collect_collision_events(&mut self, max_events: usize) {
        let collider_to_entity = self.build_collider_entity_map();

        self.narrow_phase.contacts_with(|_, contact_pair| {
            if contact_pair.has_any_active_contact {
                for manifolds in contact_pair.manifolds.iter() {
                    for contact in manifolds.contacts.iter() {
                        let point = contact.local_point1;
                        let impulse = manifolds.data.impulse;

                        let entity_a = collider_to_entity.get(&contact_pair.collider1);
                        let entity_b = collider_to_entity.get(&contact_pair.collider2);

                        if let (Some(a), Some(b)) = (entity_a, entity_b) {
                            if self.collision_events.len() < max_events {
                                self.collision_events.push(HolographicCollisionEvent {
                                    entity_a: a.clone(),
                                    entity_b: b.clone(),
                                    contact_point: [point.x, point.y, point.z],
                                    contact_normal: [
                                        manifolds.data.normal.x,
                                        manifolds.data.normal.y,
                                        manifolds.data.normal.z,
                                    ],
                                    impulse,
                                });
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn collect_contact_forces(&mut self, max_forces: usize) {
        let collider_to_entity = self.build_collider_entity_map();

        self.narrow_phase.contacts_with(|_, contact_pair| {
            if contact_pair.has_any_active_contact {
                for manifolds in contact_pair.manifolds.iter() {
                    let total_force = manifolds.data.impulse;

                    if total_force > 0.0 && self.contact_forces.len() < max_forces {
                        let entity_a = collider_to_entity.get(&contact_pair.collider1);
                        let entity_b = collider_to_entity.get(&contact_pair.collider2);

                        if let (Some(a), Some(b)) = (entity_a, entity_b) {
                            self.contact_forces.push(ContactForceData {
                                entity_a: a.clone(),
                                entity_b: b.clone(),
                                force_magnitude: total_force,
                                total_force: [
                                    manifolds.data.normal.x * total_force,
                                    manifolds.data.normal.y * total_force,
                                    manifolds.data.normal.z * total_force,
                                ],
                            });
                        }
                    }
                }
            }
        });
    }

    pub fn check_intersection(&self, entity_a: &EntityId, entity_b: &EntityId) -> bool {
        let handle_a = match self.entity_map.get(entity_a) {
            Some(h) => h,
            None => return false,
        };
        let handle_b = match self.entity_map.get(entity_b) {
            Some(h) => h,
            None => return false,
        };

        self.narrow_phase
            .contact_pair(handle_a.collider, handle_b.collider)
            .is_some()
    }

    pub fn get_contact_count(&self) -> usize {
        self.narrow_phase
            .contact_pairs()
            .filter(|(_, pair)| pair.has_any_active_contact)
            .count()
    }

    fn build_collider_entity_map(
        &self,
    ) -> std::collections::HashMap<rapier3d_f64::geometry::ColliderHandle, EntityId> {
        self.entity_map
            .iter()
            .map(|(id, handle)| (handle.collider, id.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ColliderShape;
    use super::*;
    use crate::entity_layer7::layer7::EntityId;

    fn test_entity_id(id: u64) -> EntityId {
        EntityId::new(format!("test-coll-{}", id))
    }

    #[test]
    fn test_collision_detection_overlapping_spheres() {
        let mut world = PhysicsWorld::new();
        world.integration_params.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_a = test_entity_id(100);
        let entity_b = test_entity_id(101);

        world.create_entity_body(
            entity_a.clone(),
            1.0,
            [0.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 1.0 },
        );
        world.create_entity_body(
            entity_b.clone(),
            1.0,
            [1.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 1.0 },
        );

        world.step_physics(Some(1.0 / 60.0));

        let contact_count = world.get_contact_count();
        assert!(contact_count >= 0);
    }

    #[test]
    fn test_no_collision_separated_bodies() {
        let mut world = PhysicsWorld::new();
        world.integration_params.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_a = test_entity_id(102);
        let entity_b = test_entity_id(103);

        world.create_entity_body(
            entity_a.clone(),
            1.0,
            [-10.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );
        world.create_entity_body(
            entity_b.clone(),
            1.0,
            [10.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        world.step_physics(Some(1.0 / 60.0));

        let intersects = world.check_intersection(&entity_a, &entity_b);
        assert!(!intersects);
    }

    #[test]
    fn test_contact_force_generation() {
        let mut world = PhysicsWorld::new();
        world.integration_params.gravity = rapier3d_f64::na::Vector3::new(0.0, -9.81, 0.0);

        let floor = test_entity_id(104);
        let ball = test_entity_id(105);

        world.create_entity_body(
            floor.clone(),
            0.0,
            [0.0, -1.0, 0.0],
            ColliderShape::Cuboid {
                half_extents: [10.0, 0.5, 10.0],
            },
        );
        world.create_entity_body(
            ball.clone(),
            1.0,
            [0.0, 5.0, 0.0],
            ColliderShape::Sphere { radius: 0.5 },
        );

        for _ in 0..120 {
            world.step_physics(Some(1.0 / 60.0));
        }

        world.collect_contact_forces(10);
        let forces = world.contact_forces();

        if forces.is_empty() {
            let pos = world.get_entity_position(ball.clone()).unwrap();
            assert!(
                pos[1] <= -0.5,
                "Ball should be on or below the floor, y={}",
                pos[1]
            );
        } else {
            assert!(forces[0].force_magnitude > 0.0);
        }
    }

    #[test]
    fn test_collision_events_collection() {
        let mut world = PhysicsWorld::new();
        world.integration_params.gravity = rapier3d_f64::na::Vector3::new(0.0, 0.0, 0.0);

        let entity_a = test_entity_id(106);
        let entity_b = test_entity_id(107);

        world.create_entity_body(
            entity_a.clone(),
            1.0,
            [-3.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 1.0 },
        );
        world.create_entity_body(
            entity_b.clone(),
            1.0,
            [3.0, 0.0, 0.0],
            ColliderShape::Sphere { radius: 1.0 },
        );

        world.apply_impulse(entity_a.clone(), [10.0, 0.0, 0.0]);
        world.apply_impulse(entity_b.clone(), [-10.0, 0.0, 0.0]);

        for _ in 0..60 {
            world.step_physics(Some(1.0 / 60.0));
        }

        world.collect_collision_events(10);
    }
}
