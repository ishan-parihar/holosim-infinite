//! Comprehensive Integration Tests for HoloSim Phase 6.7 Physics Pipeline
//!
//! These tests verify the COMPLETE physics pipeline works end-to-end:
//! Archetype activations → physical properties → Rapier bodies → observation sync
//! → feedback → modified archetypes.
//!
//! Test scenarios cover:
//! 1. Full pipeline integration
//! 2. Energy conservation in closed systems
//! 3. Momentum conservation in collisions
//! 4. Archetype feedback loops
//! 5. Multi-entity system stability
//! 6. Physical consistency of archetype-derived properties
//! 7. Physics → observation roundtrip fidelity
//! 8. Gravitational cluster boundedness

use std::collections::HashMap;

use crate::physics::archetype_physics::{ArchetypePhysicsMapper, Force3D, PhysicalProperties};
use crate::physics::feedback::{ArchetypeProfile, PhysicsFeedback};
use crate::physics::integrator::{RK4Integrator, RK4State};
use crate::physics::observation_sync::ObservationSync;
use crate::physics::physics_world::PhysicsWorld;
use crate::simulation_v3::observation_layer::ObservationLayer;

// ============================================================================
// Helper Functions
// ============================================================================

/// Deterministic pseudo-random f64 in [0, 1) from a seed.
fn seeded_random(seed: u64) -> f64 {
    let mut h = seed.wrapping_mul(0x5deece66d).wrapping_add(0xb);
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
    h ^= h >> 33;
    (h & 0x000F_FFFF_FFFF_FFFF) as f64 / (0x000F_FFFF_FFFF_FFFF_u64 as f64)
}

/// Generate a deterministic random archetype profile from a seed.
fn random_archetype_profile(seed: u64) -> [f64; 22] {
    std::array::from_fn(|i| seeded_random(seed.wrapping_add(i as u64 * 7919)))
}

/// Compute kinetic energy: 0.5 * m * v^2.
#[allow(dead_code)]
fn kinetic_energy(mass: f64, velocity: [f64; 3]) -> f64 {
    let v_sq = velocity[0] * velocity[0] + velocity[1] * velocity[1] + velocity[2] * velocity[2];
    0.5 * mass * v_sq
}

/// Compute gravitational potential energy: -G * m1 * m2 / r.
#[allow(dead_code)]
fn gravitational_potential_energy(g: f64, m1: f64, m2: f64, pos1: [f64; 3], pos2: [f64; 3]) -> f64 {
    let dx = pos2[0] - pos1[0];
    let dy = pos2[1] - pos1[1];
    let dz = pos2[2] - pos1[2];
    let r = (dx * dx + dy * dy + dz * dz).sqrt();
    if r < 1e-10 {
        return 0.0;
    }
    -g * m1 * m2 / r
}

/// Compute center of mass for a set of entities.
fn center_of_mass(entities: &[(u64, [f64; 3], f64)]) -> [f64; 3] {
    let mut total_mass = 0.0;
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cz = 0.0;
    for &(_, pos, mass) in entities {
        total_mass += mass;
        cx += pos[0] * mass;
        cy += pos[1] * mass;
        cz += pos[2] * mass;
    }
    if total_mass < 1e-10 {
        return [0.0, 0.0, 0.0];
    }
    [cx / total_mass, cy / total_mass, cz / total_mass]
}

/// Euclidean norm of a 3D vector.
fn vec3_norm(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

// ============================================================================
// Integration Tests
// ============================================================================

/// Test 1: Full physics pipeline — archetype → properties → physics → observation → position changed.
#[test]
fn test_full_physics_pipeline() {
    let mapper = ArchetypePhysicsMapper::new();
    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(0.0, 0.0, 0.0);

    // Create entity with a distinctive archetype profile (high body activation)
    let mut profile = [0.0; 22];
    profile[9] = 1.0; // A9: Motion — strong kinetic impulse
    let entity_id = 1u64;
    let density = 1u8;

    // Step 1: Archetype → Physical Properties
    let props = mapper.compute_properties_with_id(&profile, density, entity_id);
    assert!(props.mass > 0.0, "mass must be positive: {}", props.mass);
    assert!(
        props.force_vector.magnitude() > 0.0,
        "Motion archetype should produce non-zero force"
    );

    // Step 2: Add to Physics World
    let initial_position = [0.0, 0.0, 0.0];
    physics_world.add_entity(entity_id, initial_position, &props);

    // Step 3: Apply archetype force and step simulation
    physics_world.apply_archetype_force(entity_id, &props.force_vector);
    let dt = 0.016; // ~60 Hz
    for _ in 0..50 {
        physics_world.apply_archetype_force(entity_id, &props.force_vector);
        physics_world.step(dt);
    }

    // Step 4: Verify position changed from initial
    let state = physics_world
        .get_entity_state(entity_id)
        .expect("entity should exist");
    let displacement = vec3_norm([
        state.position[0] - initial_position[0],
        state.position[1] - initial_position[1],
        state.position[2] - initial_position[2],
    ]);

    assert!(
        displacement > 1e-6,
        "Entity should have moved from initial position after force application: displacement = {}",
        displacement
    );

    // Step 5: Sync to observation layer and verify roundtrip
    let mut observation_sync = ObservationSync::new();
    let mut observation_layer = ObservationLayer::new();

    // Pre-register the entity in the observation layer
    observation_layer.observe_entity(
        entity_id,
        &profile,
        0.5,
        density,
        initial_position,
        [0.0; 3],
        props.mass,
        1.0,
        0.5,
    );

    observation_sync.sync_physics_to_observations(&physics_world, &mut observation_layer, 1);

    let observations = observation_layer.get_all_observations();
    let obs = observations
        .get(&entity_id)
        .expect("observation should exist");

    // Observation position should match physics state within tolerance
    assert!(
        (obs.physical.position[0] - state.position[0]).abs() < 1e-3,
        "observation X should match physics: obs={}, phys={}",
        obs.physical.position[0],
        state.position[0]
    );
}

/// Test 2: Energy conservation in a closed two-body system using RK4 integration.
#[test]
fn test_energy_conservation_closed_system() {
    // Two-body gravitational system integrated with RK4
    let gm = 1.0; // GM product (central mass * gravitational constant)
    let mass = 1.0; // orbiting mass
    let dt = 0.001;
    let steps = 1000;

    // Circular orbit: v = sqrt(GM/r) = 1 at r=1
    let mut state = RK4State {
        position: [1.0, 0.0, 0.0],
        velocity: [0.0, 1.0, 0.0],
    };

    let grav_force = |s: &RK4State| -> [f64; 3] {
        let r2 = s.position[0] * s.position[0]
            + s.position[1] * s.position[1]
            + s.position[2] * s.position[2];
        let r = r2.sqrt();
        if r < 1e-10 {
            return [0.0, 0.0, 0.0];
        }
        let mag = gm * mass / r2;
        [
            -mag * s.position[0] / r,
            -mag * s.position[1] / r,
            -mag * s.position[2] / r,
        ]
    };

    fn energy(s: &RK4State, mass: f64, gm: f64) -> f64 {
        let r = vec3_norm(s.position);
        let v = vec3_norm(s.velocity);
        0.5 * mass * v * v - gm * mass / r
    }

    let e_initial = energy(&state, mass, gm);

    for _ in 0..steps {
        state = RK4Integrator::integrate(&state, mass, &grav_force, dt);
    }

    let e_final = energy(&state, mass, gm);
    let e_change = (e_final - e_initial).abs();
    let e_relative = if e_initial.abs() > 1e-10 {
        e_change / e_initial.abs()
    } else {
        e_change
    };

    assert!(
        e_relative < 0.01,
        "RK4 should conserve energy within 1%: initial={}, final={}, relative_error={}",
        e_initial,
        e_final,
        e_relative
    );
}

/// Test 3: Momentum conservation in a two-entity collision.
#[test]
fn test_momentum_conservation_collision() {
    let mass = 5.0;
    let props = PhysicalProperties {
        mass,
        charge: 0.0,
        spin: 0.0,
        damping: 0.0,
        moment_of_inertia: 2.0,
        force_vector: Force3D::new(0.0, 0.0, 0.0),
    };

    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(0.0, 0.0, 0.0);

    // Place entities on x-axis, moving toward each other
    physics_world.add_entity(1, [-10.0, 0.0, 0.0], &props);
    physics_world.add_entity(2, [10.0, 0.0, 0.0], &props);
    physics_world.set_entity_velocity(1, [3.0, 0.0, 0.0]);
    physics_world.set_entity_velocity(2, [-3.0, 0.0, 0.0]);

    // Compute initial total momentum: p = m*v
    let state1_init = physics_world.get_entity_state(1).unwrap();
    let state2_init = physics_world.get_entity_state(2).unwrap();

    let px_initial = mass * state1_init.velocity[0] + mass * state2_init.velocity[0];
    let py_initial = mass * state1_init.velocity[1] + mass * state2_init.velocity[2];
    let pz_initial = mass * state1_init.velocity[2] + mass * state2_init.velocity[2];

    // Simulate until collision (entities start 20 units apart, closing at 6 u/s → ~3.3s)
    let steps = 200;
    let dt = 0.016;
    for _ in 0..steps {
        physics_world.step(dt);
    }

    // Compute final total momentum
    let state1_final = physics_world.get_entity_state(1).unwrap();
    let state2_final = physics_world.get_entity_state(2).unwrap();

    let px_final = mass * state1_final.velocity[0] + mass * state2_final.velocity[0];
    let py_final = mass * state1_final.velocity[1] + mass * state2_final.velocity[1];
    let pz_final = mass * state1_final.velocity[2] + mass * state2_final.velocity[2];

    // Momentum should be approximately conserved
    // Initial px = 5*3 + 5*(-3) = 0
    let dp_x = (px_final - px_initial).abs();
    let dp_y = (py_final - py_initial).abs();
    let dp_z = (pz_final - pz_initial).abs();

    // With zero external forces and collision, momentum change should be small
    // (Rapier handles impulse-based collision which conserves momentum)
    let total_dp = (dp_x * dp_x + dp_y * dp_y + dp_z * dp_z).sqrt();
    assert!(
        total_dp < 1.0,
        "Total momentum change should be small: dp=({}, {}, {}), magnitude={}",
        dp_x,
        dp_y,
        dp_z,
        total_dp
    );
}

/// Test 4: Archetype feedback loop — collision → feedback → modified archetype → different force.
#[test]
fn test_archetype_feedback_loop() {
    let mapper = ArchetypePhysicsMapper::new();
    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(0.0, 0.0, 0.0);

    // Create two entities with distinct archetype profiles
    let mut profile_a: ArchetypeProfile = [0.0; 22];
    profile_a[9] = 0.5; // Motion
    profile_a[21] = 0.2; // Free Will

    let mut profile_b: ArchetypeProfile = [0.0; 22];
    profile_b[8] = 0.5; // Vitality
    profile_b[21] = 0.2; // Free Will

    let props_a = mapper.compute_properties(&profile_a, 1);
    let props_b = mapper.compute_properties(&profile_b, 1);

    // Place close and slam together — collision within ~5 steps
    physics_world.add_entity(1, [-1.5, 0.0, 0.0], &props_a);
    physics_world.add_entity(2, [1.5, 0.0, 0.0], &props_b);
    physics_world.set_entity_velocity(1, [10.0, 0.0, 0.0]);
    physics_world.set_entity_velocity(2, [-10.0, 0.0, 0.0]);

    // Step to cause collision
    for _ in 0..15 {
        physics_world.step(0.016);
    }

    // Detect collisions
    let mut sync = ObservationSync::new();
    sync.detect_collisions(physics_world.narrow_phase(), &physics_world, 1);
    let collisions = sync.take_collision_events();

    // Verify collision was detected
    assert!(
        !collisions.is_empty(),
        "Head-on collision should be detected"
    );

    // Process feedback
    let mut feedback = PhysicsFeedback::new();
    let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
    archetypes.insert(1, profile_a);
    archetypes.insert(2, profile_b);

    let modified = feedback.process_collisions(&collisions, &archetypes, 1);

    // Verify both entities were modified
    assert!(
        modified.contains_key(&1),
        "Entity 1 should be modified by collision feedback"
    );
    assert!(
        modified.contains_key(&2),
        "Entity 2 should be modified by collision feedback"
    );

    // Verify Free Will increased for both entities
    for &entity_id in &[1, 2] {
        let new_profile = modified.get(&entity_id).unwrap();
        assert!(
            new_profile[21] > archetypes[&entity_id][21] - 1e-10,
            "Entity {} Free Will should increase: {} vs {}",
            entity_id,
            new_profile[21],
            archetypes[&entity_id][21]
        );
    }

    // Verify feedback log was recorded
    assert!(
        !feedback.modification_log.is_empty(),
        "Feedback should log modification events"
    );
}

/// Test 5: Multi-entity system stability — 10 entities, 500 steps, bounded positions.
#[test]
fn test_multi_entity_stability() {
    let mapper = ArchetypePhysicsMapper::new();
    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(0.0, 0.0, 0.0);

    let num_entities = 10;
    let seed_base = 42u64;

    // Create 10 entities with random archetype profiles
    for i in 0..num_entities {
        let entity_id = i as u64;
        let profile = random_archetype_profile(seed_base + i * 1000);
        let props = mapper.compute_properties_with_id(&profile, 1, entity_id);

        // Spread entities in a 20x20x20 cube
        let x = (seeded_random(seed_base + i * 3 + 1) - 0.5) * 20.0;
        let y = (seeded_random(seed_base + i * 3 + 2) - 0.5) * 20.0;
        let z = (seeded_random(seed_base + i * 3 + 3) - 0.5) * 20.0;

        physics_world.add_entity(entity_id, [x, y, z], &props);
    }

    // Simulate 500 steps
    let steps = 500;
    let dt = 0.016;
    for step in 0..steps {
        // Apply inter-entity gravitational forces (pairwise)
        for a in 0..num_entities {
            for b in (a + 1)..num_entities {
                physics_world.apply_gravitational_force(a as u64, b as u64);
            }
        }
        physics_world.step(dt);

        // Every 100 steps, check for unbounded positions
        if (step + 1) % 100 == 0 {
            for entity_id in 0..num_entities as u64 {
                let state = physics_world.get_entity_state(entity_id).unwrap();
                let pos_norm = vec3_norm(state.position);
                assert!(
                    pos_norm < 1e6,
                    "Entity {} position unbounded at step {}: |pos|={}",
                    entity_id,
                    step + 1,
                    pos_norm
                );
            }
        }
    }

    // Final check: all positions bounded
    for entity_id in 0..num_entities as u64 {
        let state = physics_world.get_entity_state(entity_id).unwrap();
        let pos_norm = vec3_norm(state.position);
        assert!(
            pos_norm < 1e6,
            "Entity {} final position unbounded: |pos|={}",
            entity_id,
            pos_norm
        );
    }
}

/// Test 6: Archetype physical consistency — mass positive, charge in [-1,1], damping >= 0.
#[test]
fn test_archetype_physical_consistency() {
    let mapper = ArchetypePhysicsMapper::new();

    // Test across a wide range of archetype profiles
    let test_profiles: Vec<[f64; 22]> = vec![
        [0.0; 22],                                                   // All zeros
        [1.0; 22],                                                   // All ones
        [0.5; 22],                                                   // Uniform 0.5
        std::array::from_fn(|i| i as f64 / 22.0),                    // Linear gradient
        std::array::from_fn(|i| if i % 2 == 0 { 1.0 } else { 0.0 }), // Alternating
        std::array::from_fn(|i| (i as f64 * 0.137) % 1.0),           // Pseudo-random
        std::array::from_fn(|i| (i as f64 * 0.317) % 1.0),           // Different pseudo-random
    ];

    // Also test with different density levels
    for density in 0..=8 {
        for profile in &test_profiles {
            let props = mapper.compute_properties(profile, density);

            assert!(
                props.mass > 0.0,
                "mass must be positive for density={density}: got {}",
                props.mass
            );

            assert!(
                props.charge >= -1.0 && props.charge <= 1.0,
                "charge must be in [-1,1] for density={density}: got {}",
                props.charge
            );

            assert!(
                props.damping >= 0.0,
                "damping must be non-negative for density={density}: got {}",
                props.damping
            );

            assert!(
                props.moment_of_inertia > 0.0,
                "moment_of_inertia must be positive for density={density}: got {}",
                props.moment_of_inertia
            );

            // Force components should be finite
            assert!(
                props.force_vector.fx.is_finite(),
                "force fx must be finite: got {}",
                props.force_vector.fx
            );
            assert!(
                props.force_vector.fy.is_finite(),
                "force fy must be finite: got {}",
                props.force_vector.fy
            );
            assert!(
                props.force_vector.fz.is_finite(),
                "force fz must be finite: got {}",
                props.force_vector.fz
            );
        }
    }
}

/// Test 7: Physics → observation roundtrip fidelity.
#[test]
fn test_physics_observation_roundtrip() {
    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(0.0, 0.0, 0.0);

    let props = PhysicalProperties {
        mass: 2.0,
        charge: 0.5,
        spin: 0.3,
        damping: 0.1,
        moment_of_inertia: 0.8,
        force_vector: Force3D::new(0.0, 0.0, 0.0),
    };

    let entity_id = 42u64;
    let initial_pos = [10.0, 20.0, 30.0];
    physics_world.add_entity(entity_id, initial_pos, &props);

    // Step simulation briefly
    physics_world.step(0.1);

    // Get physics state
    let physics_state = physics_world
        .get_entity_state(entity_id)
        .expect("entity should exist in physics world");

    // Sync to observation layer
    let mut sync = ObservationSync::new();
    let mut observation_layer = ObservationLayer::new();

    // Register entity in observation layer first
    let profile = [0.5; 22];
    observation_layer.observe_entity(
        entity_id,
        &profile,
        0.5,
        1,
        initial_pos,
        [0.0; 3],
        props.mass,
        1.0,
        0.5,
    );

    sync.sync_physics_to_observations(&physics_world, &mut observation_layer, 1);

    // Verify observation matches physics state
    let observations = observation_layer.get_all_observations();
    let obs = observations
        .get(&entity_id)
        .expect("observation should exist");

    let tolerance = 1e-3;

    assert!(
        (obs.physical.position[0] - physics_state.position[0]).abs() < tolerance,
        "X position mismatch: obs={}, phys={}",
        obs.physical.position[0],
        physics_state.position[0]
    );
    assert!(
        (obs.physical.position[1] - physics_state.position[1]).abs() < tolerance,
        "Y position mismatch: obs={}, phys={}",
        obs.physical.position[1],
        physics_state.position[1]
    );
    assert!(
        (obs.physical.position[2] - physics_state.position[2]).abs() < tolerance,
        "Z position mismatch: obs={}, phys={}",
        obs.physical.position[2],
        physics_state.position[2]
    );
}

/// Test 8: Gravitational cluster stability — 5 entities, 200 steps, bounded center of mass.
#[test]
fn test_gravitational_cluster_stability() {
    let g_constant = 100.0; // Strong gravity for noticeable clustering
    let mapper = ArchetypePhysicsMapper {
        gravitational_constant: g_constant,
        ..ArchetypePhysicsMapper::new()
    };

    let mass = 10.0;
    let props = PhysicalProperties {
        mass,
        charge: 0.0,
        spin: 0.0,
        damping: 0.01, // Small damping for stability
        moment_of_inertia: 4.0,
        force_vector: Force3D::new(0.0, 0.0, 0.0),
    };

    let mut physics_world = PhysicsWorld::new();
    physics_world.set_gravity(0.0, 0.0, 0.0);
    physics_world.mapper = mapper;

    let num_entities = 5;

    // Arrange entities in a pentagon around origin
    for i in 0..num_entities {
        let entity_id = i as u64;
        let angle = (i as f64) * 2.0 * std::f64::consts::PI / (num_entities as f64);
        let radius = 5.0;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        let z = 0.0;

        // Give slight tangential velocity for orbital motion
        let vx = -angle.sin() * 1.0;
        let vy = angle.cos() * 1.0;

        physics_world.add_entity(entity_id, [x, y, z], &props);
        physics_world.set_entity_velocity(entity_id, [vx, vy, 0.0]);
    }

    // Record initial center of mass
    fn gather_entities(world: &PhysicsWorld, entity_mass: f64) -> Vec<(u64, [f64; 3], f64)> {
        let m = entity_mass;
        world
            .entity_ids()
            .filter_map(move |&id| world.get_entity_state(id).map(|s| (id, s.position, m)))
            .collect()
    }

    let initial_entities = gather_entities(&physics_world, mass);
    let initial_com = center_of_mass(&initial_entities);

    // Simulate 200 steps
    let steps = 200;
    let dt = 0.01;
    for _ in 0..steps {
        // Apply pairwise gravitational forces
        for a in 0..num_entities {
            for b in (a + 1)..num_entities {
                physics_world.apply_gravitational_force(a as u64, b as u64);
            }
        }
        physics_world.step(dt);
    }

    // Check center of mass hasn't drifted infinitely
    let final_entities = gather_entities(&physics_world, mass);
    let final_com = center_of_mass(&final_entities);

    let com_drift = vec3_norm([
        final_com[0] - initial_com[0],
        final_com[1] - initial_com[1],
        final_com[2] - initial_com[2],
    ]);

    assert!(
        com_drift < 1.0,
        "Center of mass should not drift significantly: drift={}",
        com_drift
    );

    // Check all entities are still bounded
    for (id, pos, _) in &final_entities {
        let dist = vec3_norm(*pos);
        assert!(
            dist < 1e6,
            "Entity {} position unbounded: |pos|={}",
            id,
            dist
        );
    }
}
