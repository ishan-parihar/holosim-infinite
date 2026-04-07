//! Physics Feedback Loop (Phase 6.6)
//!
//! Bidirectional feedback between physics state and the holographic field.
//! Physical events (collisions, velocity changes, resonance) feed back to
//! modify archetype activations, closing the loop: physics → consciousness.
//!
//! From cosmological architecture: "Physics IS consciousness at the densest resolution."
//! Observed physical states feed back to modify the consciousness simulation.
//!
//! Feedback rules:
//! 1. Collision → Free Will Catalyst (A21 perturbation proportional to impact force)
//! 2. High acceleration → Body awareness boost (A7-A13)
//! 3. Low velocity → Spirit contemplation boost (A14-A20)
//! 4. Proximity resonance → Collective spirit boost for similar nearby entities
//! 5. All modifications capped at MAX_MODIFICATION_PER_TICK for stability

use std::collections::HashMap;

use crate::physics::observation_sync::CollisionEvent;
use crate::physics::physics_world::{EntityPhysicsState, PhysicsWorld};

/// Maximum archetype modification per tick to maintain simulation stability.
pub const MAX_MODIFICATION_PER_TICK: f64 = 0.01;

/// Index range for body archetypes (A7–A13, exclusive end).
pub const BODY_START: usize = 7;
pub const BODY_END: usize = 14;

/// Index range for spirit archetypes (A14–A20, exclusive end).
pub const SPIRIT_START: usize = 14;
pub const SPIRIT_END: usize = 21;

/// Index of the Free Will archetype (A21).
pub const FREE_WILL_INDEX: usize = 21;

/// Velocity magnitude threshold below which an entity is considered "still"
/// and receives a spirit archetype boost (contemplative state).
pub const LOW_VELOCITY_THRESHOLD: f64 = 0.1;

/// Acceleration threshold above which body archetypes get boosted.
pub const HIGH_ACCELERATION_THRESHOLD: f64 = 0.5;

/// Default resonance distance threshold for proximity-based collective boost.
pub const DEFAULT_RESONANCE_THRESHOLD: f64 = 2.0;

/// Type alias for archetype profiles: 22 coefficients indexed 0–21.
pub type ArchetypeProfile = [f64; 22];

/// Trigger for a feedback event, encoding the physical cause.
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackTrigger {
    /// Entity collision detected — acts as a "free will catalyst."
    Collision {
        impact_force: f64,
        relative_velocity: f64,
    },
    /// Entity velocity changed significantly.
    VelocityChange { delta_v: f64, acceleration: f64 },
    /// Entity entered resonance zone (close to similar entity).
    Resonance {
        proximity: f64,
        archetype_similarity: f64,
    },
    /// Entity crossed a spectrum boundary.
    SpectrumCrossing {
        from_spectrum: f64,
        to_spectrum: f64,
    },
}

/// A single feedback event recording what was modified and why.
#[derive(Debug, Clone, PartialEq)]
pub struct FeedbackEvent {
    pub entity_id: u64,
    pub trigger: FeedbackTrigger,
    pub archetype_deltas: ArchetypeProfile,
    pub timestamp: u64,
}

/// Bidirectional feedback loop between physics state and archetype activations.
///
/// Processes physical events (collisions, velocity changes, proximity resonance)
/// and produces small perturbations to archetype profiles, which are then applied
/// to the holographic field.
#[derive(Debug, Clone)]
pub struct PhysicsFeedback {
    /// History of archetype modifications made by feedback.
    pub modification_log: Vec<FeedbackEvent>,
}

impl PhysicsFeedback {
    /// Create a new, empty feedback processor.
    pub fn new() -> Self {
        Self {
            modification_log: Vec::new(),
        }
    }

    /// Process collision events and generate archetype modifications.
    ///
    /// Collisions act as "free will catalyst" events — sudden, stochastic
    /// perturbations to A21 proportional to impact force. Both colliding
    /// entities receive the perturbation.
    pub fn process_collisions(
        &mut self,
        collision_events: &[CollisionEvent],
        current_archetypes: &HashMap<u64, ArchetypeProfile>,
        current_tick: u64,
    ) -> HashMap<u64, ArchetypeProfile> {
        let mut modified: HashMap<u64, ArchetypeProfile> = HashMap::new();

        for event in collision_events {
            let impact_scale = (event.impact_force / (event.impact_force + 1.0)).clamp(0.0, 1.0);
            let perturbation = impact_scale * MAX_MODIFICATION_PER_TICK;

            for &entity_id in &[event.entity_a, event.entity_b] {
                let profile = current_archetypes
                    .get(&entity_id)
                    .copied()
                    .unwrap_or([0.0; 22]);
                let mut deltas: ArchetypeProfile = [0.0; 22];

                // Boost Free Will (A21) proportionally to impact
                let fw_boost = perturbation;
                deltas[FREE_WILL_INDEX] = fw_boost;

                let mut new_profile = profile;
                new_profile[FREE_WILL_INDEX] =
                    (new_profile[FREE_WILL_INDEX] + fw_boost).clamp(0.0, 1.0);
                new_profile = normalize_archetype_profile(&new_profile);

                self.modification_log.push(FeedbackEvent {
                    entity_id,
                    trigger: FeedbackTrigger::Collision {
                        impact_force: event.impact_force,
                        relative_velocity: event.relative_velocity,
                    },
                    archetype_deltas: deltas,
                    timestamp: current_tick,
                });

                modified.insert(entity_id, new_profile);
            }
        }

        modified
    }

    /// Process velocity changes and modify archetype activations.
    ///
    /// High acceleration → increased body archetype activation (physical awareness).
    /// Low velocity → increased spirit archetype activation (contemplative state).
    pub fn process_velocity_changes(
        &mut self,
        entity_states: &HashMap<u64, EntityPhysicsState>,
        previous_states: &HashMap<u64, EntityPhysicsState>,
        current_archetypes: &HashMap<u64, ArchetypeProfile>,
        current_tick: u64,
    ) -> HashMap<u64, ArchetypeProfile> {
        let mut modified: HashMap<u64, ArchetypeProfile> = HashMap::new();

        for (&entity_id, current_state) in entity_states {
            let prev_state = match previous_states.get(&entity_id) {
                Some(s) => s,
                None => continue,
            };

            let profile = match current_archetypes.get(&entity_id) {
                Some(p) => *p,
                None => continue,
            };

            let velocity_mag = vec3_magnitude(&current_state.velocity);
            let accel_mag = vec3_magnitude(&current_state.linear_acceleration);

            let dv_x = current_state.velocity[0] - prev_state.velocity[0];
            let dv_y = current_state.velocity[1] - prev_state.velocity[1];
            let dv_z = current_state.velocity[2] - prev_state.velocity[2];
            let delta_v = (dv_x * dv_x + dv_y * dv_y + dv_z * dv_z).sqrt();

            let mut new_profile = profile;
            let mut deltas: ArchetypeProfile = [0.0; 22];
            let mut triggered = false;

            // High acceleration → boost body archetypes
            if accel_mag > HIGH_ACCELERATION_THRESHOLD {
                let body_boost = (accel_mag / (accel_mag + 1.0)) * MAX_MODIFICATION_PER_TICK;
                let body_count = (BODY_END - BODY_START) as f64;
                for i in BODY_START..BODY_END {
                    let boost = body_boost / body_count;
                    deltas[i] = boost;
                    new_profile[i] = (new_profile[i] + boost).clamp(0.0, 1.0);
                }
                triggered = true;
            }

            // Low velocity → boost spirit archetypes
            if velocity_mag < LOW_VELOCITY_THRESHOLD {
                let spirit_factor = 1.0 - (velocity_mag / LOW_VELOCITY_THRESHOLD);
                let spirit_boost = spirit_factor * MAX_MODIFICATION_PER_TICK;
                let spirit_count = (SPIRIT_END - SPIRIT_START) as f64;
                for i in SPIRIT_START..SPIRIT_END {
                    let boost = spirit_boost / spirit_count;
                    deltas[i] += boost;
                    new_profile[i] = (new_profile[i] + boost).clamp(0.0, 1.0);
                }
                triggered = true;
            }

            if triggered {
                new_profile = normalize_archetype_profile(&new_profile);
                modified.insert(entity_id, new_profile);

                self.modification_log.push(FeedbackEvent {
                    entity_id,
                    trigger: FeedbackTrigger::VelocityChange {
                        delta_v,
                        acceleration: accel_mag,
                    },
                    archetype_deltas: deltas,
                    timestamp: current_tick,
                });
            }
        }

        modified
    }

    /// Process proximity/resonance events.
    ///
    /// Entities within `resonance_threshold` of similar entities get a spirit
    /// archetype boost proportional to their archetype similarity and inverse
    /// distance.
    pub fn process_resonance(
        &mut self,
        entity_positions: &HashMap<u64, [f64; 3]>,
        current_archetypes: &HashMap<u64, ArchetypeProfile>,
        resonance_threshold: f64,
        current_tick: u64,
    ) -> HashMap<u64, ArchetypeProfile> {
        let mut modified: HashMap<u64, ArchetypeProfile> = HashMap::new();
        let entity_ids: Vec<u64> = entity_positions.keys().copied().collect();

        for (i, &id_a) in entity_ids.iter().enumerate() {
            let profile_a = match current_archetypes.get(&id_a) {
                Some(p) => *p,
                None => continue,
            };
            let pos_a = &entity_positions[&id_a];

            let mut total_spirit_boost: f64 = 0.0;
            let mut resonance_count: u32 = 0;
            let mut best_proximity: f64 = f64::MAX;
            let mut best_similarity: f64 = 0.0;

            for &id_b in &entity_ids[i + 1..] {
                let profile_b = match current_archetypes.get(&id_b) {
                    Some(p) => *p,
                    None => continue,
                };
                let pos_b = &entity_positions[&id_b];

                let dist = distance_3d(pos_a, pos_b);
                if dist < resonance_threshold && dist > 1e-10 {
                    let similarity = archetype_similarity(&profile_a, &profile_b);
                    let proximity_factor = 1.0 - (dist / resonance_threshold);
                    let boost = proximity_factor * similarity * MAX_MODIFICATION_PER_TICK;

                    total_spirit_boost += boost;
                    resonance_count += 1;

                    if dist < best_proximity {
                        best_proximity = dist;
                        best_similarity = similarity;
                    }
                }
            }

            if resonance_count > 0 {
                // Average the boost across all resonant partners
                let avg_boost = total_spirit_boost / resonance_count as f64;
                let clamped_boost = avg_boost.min(MAX_MODIFICATION_PER_TICK);

                let mut new_profile = profile_a;
                let mut deltas: ArchetypeProfile = [0.0; 22];
                let spirit_count = (SPIRIT_END - SPIRIT_START) as f64;

                for i in SPIRIT_START..SPIRIT_END {
                    let boost = clamped_boost / spirit_count;
                    deltas[i] = boost;
                    new_profile[i] = (new_profile[i] + boost).clamp(0.0, 1.0);
                }

                new_profile = normalize_archetype_profile(&new_profile);
                modified.insert(id_a, new_profile);

                self.modification_log.push(FeedbackEvent {
                    entity_id: id_a,
                    trigger: FeedbackTrigger::Resonance {
                        proximity: best_proximity,
                        archetype_similarity: best_similarity,
                    },
                    archetype_deltas: deltas,
                    timestamp: current_tick,
                });
            }
        }

        modified
    }

    /// Apply ALL feedback modifications to archetype profiles in one call.
    ///
    /// Combines collision, velocity, and resonance feedback into a single
    /// pass. Returns the fully modified archetype profiles.
    pub fn apply_all_feedback(
        &mut self,
        physics_world: &PhysicsWorld,
        current_archetypes: &HashMap<u64, ArchetypeProfile>,
        collision_events: &[CollisionEvent],
        current_tick: u64,
    ) -> HashMap<u64, ArchetypeProfile> {
        // Start with collision feedback
        let collision_modified =
            self.process_collisions(collision_events, current_archetypes, current_tick);

        // Build current state maps for velocity processing
        let mut current_states: HashMap<u64, EntityPhysicsState> = HashMap::new();
        for &entity_id in current_archetypes.keys() {
            if let Some(state) = physics_world.get_entity_state(entity_id) {
                current_states.insert(entity_id, state);
            }
        }

        // Previous states are approximated by current states minus one step
        // (In production, these would be cached from the previous tick)
        let previous_states: HashMap<u64, EntityPhysicsState> = current_states
            .iter()
            .map(|(&k, v)| {
                (
                    k,
                    EntityPhysicsState {
                        position: v.position,
                        velocity: [
                            v.velocity[0] * 0.9,
                            v.velocity[1] * 0.9,
                            v.velocity[2] * 0.9,
                        ],
                        linear_acceleration: v.linear_acceleration,
                    },
                )
            })
            .collect();

        let velocity_modified = self.process_velocity_changes(
            &current_states,
            &previous_states,
            &merge_profiles(current_archetypes, &collision_modified),
            current_tick,
        );

        // Build position map for resonance
        let positions: HashMap<u64, [f64; 3]> = current_states
            .iter()
            .map(|(&k, v)| (k, v.position))
            .collect();

        let resonance_modified = self.process_resonance(
            &positions,
            &merge_profiles(
                &merge_profiles(current_archetypes, &collision_modified),
                &velocity_modified,
            ),
            DEFAULT_RESONANCE_THRESHOLD,
            current_tick,
        );

        // Merge all modifications
        merge_profiles(
            &merge_profiles(&collision_modified, &velocity_modified),
            &resonance_modified,
        )
    }

    /// Clear the modification log.
    pub fn clear_log(&mut self) {
        self.modification_log.clear();
    }
}

impl Default for PhysicsFeedback {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Normalize an archetype profile so the sum equals the original sum
/// while keeping all values in [0, 1].
fn normalize_archetype_profile(profile: &ArchetypeProfile) -> ArchetypeProfile {
    let original_sum: f64 = profile.iter().sum();
    if original_sum < 1e-10 {
        return [0.0; 22];
    }

    let sum: f64 = profile.iter().sum();
    if sum < 1e-10 {
        return [0.0; 22];
    }

    let target = original_sum.min(22.0); // Max possible: all 1.0
    let scale = target / sum;

    let mut normalized: ArchetypeProfile = [0.0; 22];
    for i in 0..22 {
        normalized[i] = (profile[i] * scale).clamp(0.0, 1.0);
    }

    // Final adjustment to ensure sum matches target
    let current_sum: f64 = normalized.iter().sum();
    if current_sum > 1e-10 && (current_sum - target).abs() > 1e-10 {
        let diff = (target - current_sum) / 22.0;
        for i in 0..22 {
            normalized[i] = (normalized[i] + diff).clamp(0.0, 1.0);
        }
    }

    normalized
}

/// Merge two profile maps: later values override earlier ones.
fn merge_profiles(
    base: &HashMap<u64, ArchetypeProfile>,
    overrides: &HashMap<u64, ArchetypeProfile>,
) -> HashMap<u64, ArchetypeProfile> {
    let mut merged = base.clone();
    for (k, v) in overrides {
        merged.insert(*k, *v);
    }
    merged
}

/// Euclidean distance between two 3D points.
fn distance_3d(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Magnitude of a 3D vector.
fn vec3_magnitude(v: &[f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

/// Cosine similarity between two archetype profiles.
fn archetype_similarity(a: &ArchetypeProfile, b: &ArchetypeProfile) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if mag_a < 1e-10 || mag_b < 1e-10 {
        return 0.0;
    }

    (dot / (mag_a * mag_b)).clamp(0.0, 1.0)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_profile(values: &[(usize, f64)]) -> ArchetypeProfile {
        let mut p = [0.0; 22];
        for &(idx, val) in values {
            p[idx] = val;
        }
        p
    }

    fn uniform_profile(v: f64) -> ArchetypeProfile {
        [v; 22]
    }

    fn make_collision_event(
        a: u64,
        b: u64,
        impact: f64,
        rel_vel: f64,
        tick: u64,
    ) -> CollisionEvent {
        CollisionEvent {
            entity_a: a,
            entity_b: b,
            impact_force: impact,
            relative_velocity: rel_vel,
            timestamp: tick,
        }
    }

    fn make_physics_state(pos: [f64; 3], vel: [f64; 3], accel: [f64; 3]) -> EntityPhysicsState {
        EntityPhysicsState {
            position: pos,
            velocity: vel,
            linear_acceleration: accel,
        }
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_collision_boosts_free_will() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);
        archetypes.insert(2, profile);

        let events = vec![make_collision_event(1, 2, 5.0, 3.0, 1)];
        let modified = feedback.process_collisions(&events, &archetypes, 1);

        for &id in &[1, 2] {
            let new_profile = modified
                .get(&id)
                .expect(&format!("entity {} should be modified", id));
            assert!(
                new_profile[FREE_WILL_INDEX] > profile[FREE_WILL_INDEX],
                "A21 should increase after collision: {} > {}",
                new_profile[FREE_WILL_INDEX],
                profile[FREE_WILL_INDEX]
            );
        }

        assert_eq!(feedback.modification_log.len(), 2);
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_acceleration_boosts_body() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);

        let current_states: HashMap<u64, EntityPhysicsState> = [(
            1,
            make_physics_state(
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [5.0, 0.0, 0.0], // High acceleration
            ),
        )]
        .into_iter()
        .collect();

        let previous_states: HashMap<u64, EntityPhysicsState> = [(
            1,
            make_physics_state([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        )]
        .into_iter()
        .collect();

        let modified =
            feedback.process_velocity_changes(&current_states, &previous_states, &archetypes, 1);

        let new_profile = modified.get(&1).expect("entity 1 should be modified");
        for i in BODY_START..BODY_END {
            assert!(
                new_profile[i] >= profile[i],
                "Body archetype {} should not decrease: {} >= {}",
                i,
                new_profile[i],
                profile[i]
            );
        }

        let body_increase: f64 = (BODY_START..BODY_END)
            .map(|i| new_profile[i] - profile[i])
            .sum();
        assert!(
            body_increase > 0.0,
            "Total body increase should be positive: {}",
            body_increase
        );
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_low_velocity_boosts_spirit() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);

        let current_states: HashMap<u64, EntityPhysicsState> = [(
            1,
            make_physics_state(
                [0.0, 0.0, 0.0],
                [0.01, 0.01, 0.01], // Very low velocity
                [0.0, 0.0, 0.0],
            ),
        )]
        .into_iter()
        .collect();

        let previous_states: HashMap<u64, EntityPhysicsState> = [(
            1,
            make_physics_state([0.0, 0.0, 0.0], [0.01, 0.01, 0.01], [0.0, 0.0, 0.0]),
        )]
        .into_iter()
        .collect();

        let modified =
            feedback.process_velocity_changes(&current_states, &previous_states, &archetypes, 1);

        let new_profile = modified.get(&1).expect("entity 1 should be modified");
        for i in SPIRIT_START..SPIRIT_END {
            assert!(
                new_profile[i] >= profile[i],
                "Spirit archetype {} should not decrease when low velocity: {} >= {}",
                i,
                new_profile[i],
                profile[i]
            );
        }

        let spirit_increase: f64 = (SPIRIT_START..SPIRIT_END)
            .map(|i| new_profile[i] - profile[i])
            .sum();
        assert!(
            spirit_increase > 0.0,
            "Total spirit increase should be positive: {}",
            spirit_increase
        );
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_resonance_boosts_collective() {
        let mut feedback = PhysicsFeedback::new();
        let profile_a = make_profile(&[(14, 0.5), (15, 0.5)]);
        let profile_b = make_profile(&[(14, 0.5), (15, 0.5)]);

        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile_a);
        archetypes.insert(2, profile_b);

        let mut positions: HashMap<u64, [f64; 3]> = HashMap::new();
        positions.insert(1, [0.0, 0.0, 0.0]);
        positions.insert(2, [0.5, 0.0, 0.0]); // Close enough for resonance

        let modified =
            feedback.process_resonance(&positions, &archetypes, DEFAULT_RESONANCE_THRESHOLD, 1);

        assert!(
            !modified.is_empty(),
            "Resonance should produce modifications for nearby similar entities"
        );

        for (&id, new_profile) in &modified {
            let orig = archetypes[&id];
            let spirit_increase: f64 = (SPIRIT_START..SPIRIT_END)
                .map(|i| new_profile[i] - orig[i])
                .sum();
            assert!(
                spirit_increase > 0.0,
                "Entity {} spirit should increase from resonance: {}",
                id,
                spirit_increase
            );
        }
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_modification_capped_at_max() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);
        archetypes.insert(2, profile);

        // Large impact force to test capping
        let events = vec![make_collision_event(1, 2, 1000.0, 500.0, 1)];
        let modified = feedback.process_collisions(&events, &archetypes, 1);

        for &id in &[1, 2] {
            let new_profile = modified.get(&id).unwrap();
            let fw_delta = new_profile[FREE_WILL_INDEX] - profile[FREE_WILL_INDEX];
            assert!(
                fw_delta <= MAX_MODIFICATION_PER_TICK + 1e-10,
                "Free Will delta should be capped at {}: got {}",
                MAX_MODIFICATION_PER_TICK,
                fw_delta
            );
        }
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_apply_all_feedback_combines() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);
        archetypes.insert(2, profile);
        archetypes.insert(3, profile);

        let mut world = PhysicsWorld::new();
        world.set_gravity(0.0, 0.0, 0.0);
        let props = crate::physics::archetype_physics::PhysicalProperties {
            mass: 1.0,
            charge: 0.0,
            spin: 0.0,
            damping: 0.0,
            moment_of_inertia: 0.4,
            force_vector: crate::physics::archetype_physics::Force3D::new(0.0, 0.0, 0.0),
        };
        world.add_entity(1, [0.0, 0.0, 0.0], &props);
        world.add_entity(2, [0.5, 0.0, 0.0], &props);
        world.add_entity(3, [100.0, 100.0, 100.0], &props);

        let collisions = vec![make_collision_event(1, 2, 2.0, 1.5, 1)];
        let modified = feedback.apply_all_feedback(&world, &archetypes, &collisions, 1);

        assert!(
            !modified.is_empty(),
            "apply_all_feedback should produce modifications"
        );
        assert!(
            feedback.modification_log.len() >= 2,
            "Should log at least collision events: got {}",
            feedback.modification_log.len()
        );
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_clear_log() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);
        archetypes.insert(2, profile);

        let events = vec![make_collision_event(1, 2, 1.0, 1.0, 1)];
        feedback.process_collisions(&events, &archetypes, 1);

        assert!(
            !feedback.modification_log.is_empty(),
            "Log should have entries after processing"
        );

        feedback.clear_log();
        assert!(
            feedback.modification_log.is_empty(),
            "Log should be empty after clear"
        );
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_archetype_normalization() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let original_sum: f64 = profile.iter().sum();
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);
        archetypes.insert(2, profile);

        let events = vec![make_collision_event(1, 2, 5.0, 3.0, 1)];
        let modified = feedback.process_collisions(&events, &archetypes, 1);

        for (_, new_profile) in &modified {
            let new_sum: f64 = new_profile.iter().sum();
            assert!(
                (new_sum - original_sum).abs() < 0.1,
                "Profile sum should remain approximately the same after normalization: original={}, after={}",
                original_sum,
                new_sum
            );
            // All values should still be in [0, 1]
            for &val in new_profile.iter() {
                assert!(
                    val >= 0.0 && val <= 1.0,
                    "Archetype values must be in [0, 1]: got {}",
                    val
                );
            }
        }
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_normalize_zero_profile() {
        let profile: ArchetypeProfile = [0.0; 22];
        let normalized = normalize_archetype_profile(&profile);
        let sum: f64 = normalized.iter().sum();
        assert!(sum < 1e-10, "Zero profile should remain zero: sum={}", sum);
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_resonance_far_entities_no_boost() {
        let mut feedback = PhysicsFeedback::new();
        let profile = uniform_profile(0.3);
        let mut archetypes: HashMap<u64, ArchetypeProfile> = HashMap::new();
        archetypes.insert(1, profile);
        archetypes.insert(2, profile);

        let mut positions: HashMap<u64, [f64; 3]> = HashMap::new();
        positions.insert(1, [0.0, 0.0, 0.0]);
        positions.insert(2, [100.0, 100.0, 100.0]); // Very far apart

        let modified =
            feedback.process_resonance(&positions, &archetypes, DEFAULT_RESONANCE_THRESHOLD, 1);

        assert!(
            modified.is_empty(),
            "Far entities should not trigger resonance"
        );
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_feedback_trigger_equality() {
        let t1 = FeedbackTrigger::Collision {
            impact_force: 5.0,
            relative_velocity: 3.0,
        };
        let t2 = FeedbackTrigger::Collision {
            impact_force: 5.0,
            relative_velocity: 3.0,
        };
        assert_eq!(t1, t2);

        let t3 = FeedbackTrigger::Resonance {
            proximity: 0.5,
            archetype_similarity: 0.8,
        };
        assert_ne!(t1, t3);
    }

    // ------------------------------------------------------------------
    #[test]
    fn test_feedback_event_clone_and_debug() {
        let event = FeedbackEvent {
            entity_id: 42,
            trigger: FeedbackTrigger::VelocityChange {
                delta_v: 1.5,
                acceleration: 3.0,
            },
            archetype_deltas: [0.0; 22],
            timestamp: 100,
        };

        let cloned = event.clone();
        assert_eq!(event, cloned);
        let _ = format!("{:?}", event); // Ensure Debug works
    }
}
