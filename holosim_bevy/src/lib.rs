//! # HoloSim Bevy Plugin
//!
//! Bridges HoloSim Infinite's consciousness-first simulation to Bevy ECS.
//!
//! ## Architecture
//!
//! ```text
//! HoloSim Engine (simulation_v3)
//!   ↓ (Observation Layer)
//! EntityObservation (physical + behavioral + environmental)
//!   ↓ (sync_holosim_state system)
//! Bevy ECS Components (HoloSimEntity, DerivedPhysics, DerivedBehavior)
//!   ↓ (game logic systems)
//! Rendering, Input, Audio, UI (Bevy standard)
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use bevy::prelude::*;
//! use holosim_bevy::HoloSimPlugin;
//!
//! App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugins(HoloSimPlugin::default())
//!     .run();
//! ```

use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use holonic_realms::simulation_v3::observation_layer::{
    EntityObservation, ObservationLayer, TerrainType,
};
use holonic_realms::types::Float;
use std::collections::HashMap;

// ─── Input Events ────────────────────────────────────────────────────────────

/// Type of player choice to influence entity Free Will.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceType {
    /// Encourage exploration — boosts Mind archetypes (A1-A7).
    Explore,
    /// Encourage social interaction — boosts Spirit archetypes (A15-A21).
    Socialize,
    /// Encourage resting — boosts Body archetypes (A8-A14).
    Rest,
    /// Encourage resource gathering — boosts Body+Mind archetypes.
    Gather,
}

/// Event sent when the player makes a choice to influence entities.
///
/// When `target_entity_id` is 0, the choice is broadcast to all entities.
#[derive(Event, Clone)]
pub struct PlayerChoiceEvent {
    pub target_entity_id: u64,
    pub choice_type: ChoiceType,
    pub position: Vec3,
}

// ─── Player Input State ──────────────────────────────────────────────────────

/// Bevy resource tracking player input state and pending choice events.
#[derive(Resource, Default)]
pub struct PlayerInputState {
    pub pending_choices: Vec<PlayerChoiceEvent>,
    pub mouse_position: Option<Vec2>,
    pub selected_entity: Option<u64>,
}

// ─── Resources ───────────────────────────────────────────────────────────────

/// The HoloSim engine, wrapped for Bevy ECS access.
///
/// Insert this resource before running the simulation. The plugin reads from
/// it every tick and syncs observation data to Bevy entities.
#[derive(Resource)]
pub struct HoloSimEngine {
    observation_layer: ObservationLayer,
    step_count: u64,
}

impl HoloSimEngine {
    pub fn new() -> Self {
        Self {
            observation_layer: ObservationLayer::new(),
            step_count: 0,
        }
    }

    pub fn observation_layer(&self) -> &ObservationLayer {
        &self.observation_layer
    }

    pub fn observation_layer_mut(&mut self) -> &mut ObservationLayer {
        &mut self.observation_layer
    }

    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    pub fn advance(&mut self) {
        self.step_count += 1;
    }

    pub fn observe_entity(
        &mut self,
        entity_id: u64,
        archetype_profile: &[Float; 22],
        spectrum_position: Float,
        density: u8,
        position: [Float; 3],
        velocity: [Float; 3],
        mass: Float,
        energy: Float,
        coherence: Float,
    ) {
        self.observation_layer.observe_entity(
            entity_id,
            archetype_profile,
            spectrum_position,
            density,
            position,
            velocity,
            mass,
            energy,
            coherence,
        );
    }

    pub fn get_archetype_profile(&self, entity_id: u64) -> Option<[Float; 22]> {
        self.observation_layer
            .get_all_observations()
            .get(&entity_id)
            .map(|obs| obs.behavioral.personality_profile)
    }

    pub fn apply_player_choice(&mut self, entity_id: u64, choice_type: ChoiceType) {
        let obs = self
            .observation_layer
            .get_all_observations()
            .get(&entity_id)
            .cloned();

        if let Some(observation) = obs {
            let mut profile = observation.behavioral.personality_profile;
            modify_archetype_for_choice(&mut profile, choice_type);

            let physical = &observation.physical;
            self.observation_layer.observe_entity(
                entity_id,
                &profile,
                0.5,
                3,
                physical.position,
                physical.velocity,
                physical.mass,
                physical.energy_level,
                physical.health,
            );
        }
    }
}

impl Default for HoloSimEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Components ──────────────────────────────────────────────────────────────

/// Marker component for entities driven by HoloSim simulation.
///
/// Carries the entity's holographic identity: archetype profile, density,
/// and spectrum position. Game logic systems read these to determine
/// entity behavior.
#[derive(Component, Debug, Clone)]
pub struct HoloSimEntity {
    pub entity_id: u64,
    pub archetype_profile: [Float; 22],
    pub density: u8,
    pub spectrum_position: Float,
    pub coherence: Float,
}

impl HoloSimEntity {
    /// Get the dominant archetype index (highest activation).
    pub fn dominant_archetype(&self) -> usize {
        self.archetype_profile
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get activation level of a specific archetype.
    pub fn archetype_activation(&self, index: usize) -> Float {
        self.archetype_profile.get(index).copied().unwrap_or(0.0)
    }
}

/// Physical properties derived from holographic simulation.
///
/// These values are computed by the Observation Layer from archetype
/// coefficients and spectrum position. Game physics systems can use
/// these directly.
#[derive(Component, Debug, Clone, Copy)]
pub struct DerivedPhysics {
    pub position: [Float; 3],
    pub velocity: [Float; 3],
    pub mass: Float,
    pub temperature: Float,
    pub charge: Float,
    pub health: Float,
    pub energy_level: Float,
}

/// Behavioral state derived from archetype interference.
///
/// Represents the entity's internal drive state. Game AI systems
/// can use these to determine entity actions without behavior trees.
#[derive(Component, Debug, Clone)]
pub struct DerivedBehavior {
    pub needs: [Float; 5],
    pub mood: Float,
    pub current_goal: Option<String>,
    pub personality_profile: [Float; 22],
    pub stability: Float,
    pub novelty: Float,
}

impl DerivedBehavior {
    pub const NEED_HUNGER: usize = 0;
    pub const NEED_SOCIAL: usize = 1;
    pub const NEED_REST: usize = 2;
    pub const NEED_EXPLORATION: usize = 3;
    pub const NEED_GROWTH: usize = 4;

    pub fn most_urgent_need(&self) -> (usize, Float) {
        self.needs
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, &v)| (i, v))
            .unwrap_or((0, 0.0))
    }

    pub fn is_need_critical(&self, threshold: Float) -> Option<usize> {
        self.needs.iter().position(|&n| n > threshold)
    }
}

/// Environmental context for the entity's current region.
#[derive(Component, Debug, Clone, Copy)]
pub struct DerivedEnvironment {
    pub terrain: TerrainType,
    pub weather_intensity: Float,
    pub resource_density: Float,
    pub danger_level: Float,
    pub social_density: Float,
}

// ─── System Sets ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct HoloSimSet;

// ─── Plugin ──────────────────────────────────────────────────────────────────

/// Bevy plugin that syncs HoloSim simulation state to ECS components.
///
/// Every tick, reads the Observation Layer and creates/updates Bevy entities
/// with `HoloSimEntity`, `DerivedPhysics`, `DerivedBehavior`, and
/// `DerivedEnvironment` components.
pub struct HoloSimPlugin;

impl Plugin for HoloSimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<HoloSimEngine>()
            .init_resource::<PlayerInputState>()
            .configure_sets(Update, HoloSimSet)
            .add_systems(
                Update,
                (
                    handle_player_input,
                    apply_player_choices,
                    sync_holosim_state,
                    cleanup_departed_entities,
                )
                    .chain()
                    .in_set(HoloSimSet),
            );
    }
}

// ─── Systems ─────────────────────────────────────────────────────────────────

/// Syncs HoloSim observation data to Bevy ECS components.
///
/// This is the primary bridge: every observation in the Observation Layer
/// becomes (or updates) a Bevy entity with holographic components.
pub fn sync_holosim_state(
    mut commands: bevy::prelude::Commands,
    mut engine: bevy::prelude::ResMut<HoloSimEngine>,
    existing: bevy::prelude::Query<(Entity, &HoloSimEntity)>,
) {
    engine.advance();

    let observations: Vec<(u64, EntityObservation)> = {
        let layer = engine.observation_layer();
        layer
            .get_all_observations()
            .iter()
            .map(|(id, obs)| (*id, obs.clone()))
            .collect()
    };

    let existing_map: HashMap<u64, Entity> = existing
        .iter()
        .map(|(entity, holo)| (holo.entity_id, entity))
        .collect();

    for (entity_id, observation) in &observations {
        if let Some(&existing_entity) = existing_map.get(entity_id) {
            commands.entity(existing_entity).insert((
                DerivedPhysics {
                    position: observation.physical.position,
                    velocity: observation.physical.velocity,
                    mass: observation.physical.mass,
                    temperature: observation.physical.temperature,
                    charge: observation.physical.charge,
                    health: observation.physical.health,
                    energy_level: observation.physical.energy_level,
                },
                DerivedBehavior {
                    needs: observation.behavioral.needs,
                    mood: observation.behavioral.mood,
                    current_goal: observation.behavioral.current_goal.clone(),
                    personality_profile: observation.behavioral.personality_profile,
                    stability: observation.behavioral.stability,
                    novelty: observation.behavioral.novelty,
                },
                DerivedEnvironment {
                    terrain: observation.environment.terrain_type,
                    weather_intensity: observation.environment.weather_intensity,
                    resource_density: observation.environment.resource_density,
                    danger_level: observation.environment.danger_level,
                    social_density: observation.environment.social_density,
                },
            ));
        } else {
            commands.spawn((
                HoloSimEntity {
                    entity_id: *entity_id,
                    archetype_profile: observation.behavioral.personality_profile,
                    density: 3,
                    spectrum_position: 0.5,
                    coherence: observation.physical.health,
                },
                DerivedPhysics {
                    position: observation.physical.position,
                    velocity: observation.physical.velocity,
                    mass: observation.physical.mass,
                    temperature: observation.physical.temperature,
                    charge: observation.physical.charge,
                    health: observation.physical.health,
                    energy_level: observation.physical.energy_level,
                },
                DerivedBehavior {
                    needs: observation.behavioral.needs,
                    mood: observation.behavioral.mood,
                    current_goal: observation.behavioral.current_goal.clone(),
                    personality_profile: observation.behavioral.personality_profile,
                    stability: observation.behavioral.stability,
                    novelty: observation.behavioral.novelty,
                },
                DerivedEnvironment {
                    terrain: observation.environment.terrain_type,
                    weather_intensity: observation.environment.weather_intensity,
                    resource_density: observation.environment.resource_density,
                    danger_level: observation.environment.danger_level,
                    social_density: observation.environment.social_density,
                },
            ));
        }
    }
}

/// Removes Bevy entities whose HoloSim observation no longer exists.
pub fn cleanup_departed_entities(
    mut commands: bevy::prelude::Commands,
    engine: bevy::prelude::Res<HoloSimEngine>,
    entities: bevy::prelude::Query<(Entity, &HoloSimEntity)>,
) {
    let observations = engine.observation_layer().get_all_observations();
    for (entity, holo) in &entities {
        if !observations.contains_key(&holo.entity_id) {
            commands.entity(entity).despawn();
        }
    }
}

// ─── Player Input Systems ────────────────────────────────────────────────────

const ENTITY_SELECTION_RADIUS: f32 = 30.0;
const EXPLORE_BOOST: Float = 0.05;
const SOCIALIZE_BOOST: Float = 0.05;
const REST_BOOST: Float = 0.05;
const GATHER_BOOST: Float = 0.03;

/// Handles player keyboard and mouse input, generating PlayerChoiceEvent events.
///
/// Keyboard shortcuts: E=Explore, S=Socialize, R=Rest, G=Gather.
/// Left-click selects the nearest entity within the selection radius.
pub fn handle_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_input: ResMut<PlayerInputState>,
    entities: Query<(&HoloSimEntity, &Transform)>,
) {
    if let Ok(window) = windows.single() {
        if let Some(pos) = window.cursor_position() {
            player_input.mouse_position = Some(pos);
        }
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let (Ok((camera, camera_transform)), Some(screen_pos)) =
            (camera.single(), player_input.mouse_position)
        {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                let click_pos = Vec2::new(world_pos.x as f32, world_pos.y as f32);
                let mut nearest_id: Option<u64> = None;
                let mut nearest_dist = ENTITY_SELECTION_RADIUS;

                for (holo, transform) in &entities {
                    let entity_pos = Vec2::new(transform.translation.x, transform.translation.y);
                    let dist = entity_pos.distance(click_pos);
                    if dist < nearest_dist {
                        nearest_dist = dist;
                        nearest_id = Some(holo.entity_id);
                    }
                }

                player_input.selected_entity = nearest_id;
            }
        }
    }

    let choice_type = if keys.just_pressed(KeyCode::KeyE) {
        Some(ChoiceType::Explore)
    } else if keys.just_pressed(KeyCode::KeyS) {
        Some(ChoiceType::Socialize)
    } else if keys.just_pressed(KeyCode::KeyR) {
        Some(ChoiceType::Rest)
    } else if keys.just_pressed(KeyCode::KeyG) {
        Some(ChoiceType::Gather)
    } else {
        None
    };

    if let Some(ct) = choice_type {
        let target_id = player_input.selected_entity.unwrap_or(0);
        let pos = player_input
            .mouse_position
            .map(|p| Vec3::new(p.x, p.y, 0.0))
            .unwrap_or(Vec3::ZERO);

        let event = PlayerChoiceEvent {
            target_entity_id: target_id,
            choice_type: ct,
            position: pos,
        };
        player_input.pending_choices.push(event);
    }
}

/// Applies pending player choice events to entity archetype profiles.
///
/// Boosts archetype groups based on choice type, clamped to 0.0–1.0.
/// Pending choices are drained after processing.
pub fn apply_player_choices(
    mut player_input: ResMut<PlayerInputState>,
    mut engine: ResMut<HoloSimEngine>,
    mut entities: Query<&mut HoloSimEntity>,
) {
    if player_input.pending_choices.is_empty() {
        return;
    }

    let choices: Vec<PlayerChoiceEvent> = player_input.pending_choices.drain(..).collect();

    for mut holo in &mut entities {
        for choice in &choices {
            if choice.target_entity_id == 0 || choice.target_entity_id == holo.entity_id {
                modify_archetype_for_choice(&mut holo.archetype_profile, choice.choice_type);
            }
        }
    }

    for choice in &choices {
        if choice.target_entity_id == 0 {
            let entity_ids: Vec<u64> = engine
                .observation_layer()
                .get_all_observations()
                .keys()
                .copied()
                .collect();
            for eid in entity_ids {
                engine.apply_player_choice(eid, choice.choice_type);
            }
        } else {
            engine.apply_player_choice(choice.target_entity_id, choice.choice_type);
        }
    }
}

fn modify_archetype_for_choice(profile: &mut [Float; 22], choice_type: ChoiceType) {
    match choice_type {
        ChoiceType::Explore => {
            for i in 0..=6 {
                profile[i] = (profile[i] + EXPLORE_BOOST).min(1.0);
            }
        }
        ChoiceType::Socialize => {
            for i in 14..=20 {
                profile[i] = (profile[i] + SOCIALIZE_BOOST).min(1.0);
            }
        }
        ChoiceType::Rest => {
            for i in 7..=13 {
                profile[i] = (profile[i] + REST_BOOST).min(1.0);
            }
        }
        ChoiceType::Gather => {
            for i in 0..=3 {
                profile[i] = (profile[i] + GATHER_BOOST).min(1.0);
            }
            for i in 7..=13 {
                profile[i] = (profile[i] + GATHER_BOOST).min(1.0);
            }
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holosim_entity_dominant_archetype() {
        let mut profile = [0.0; 22];
        profile[7] = 0.9;
        let entity = HoloSimEntity {
            entity_id: 1,
            archetype_profile: profile,
            density: 3,
            spectrum_position: 0.5,
            coherence: 0.8,
        };
        assert_eq!(entity.dominant_archetype(), 7);
    }

    #[test]
    fn test_derived_behavior_most_urgent_need() {
        let behavior = DerivedBehavior {
            needs: [0.2, 0.8, 0.3, 0.1, 0.5],
            mood: 0.0,
            current_goal: None,
            personality_profile: [0.0; 22],
            stability: 0.5,
            novelty: 0.5,
        };
        assert_eq!(behavior.most_urgent_need(), (1, 0.8));
    }

    #[test]
    fn test_derived_behavior_critical_need() {
        let behavior = DerivedBehavior {
            needs: [0.2, 0.95, 0.3, 0.1, 0.5],
            mood: 0.0,
            current_goal: None,
            personality_profile: [0.0; 22],
            stability: 0.5,
            novelty: 0.5,
        };
        assert_eq!(behavior.is_need_critical(0.9), Some(1));
        assert_eq!(behavior.is_need_critical(0.99), None);
    }

    #[test]
    fn test_holosim_engine_advance() {
        let mut engine = HoloSimEngine::new();
        assert_eq!(engine.step_count(), 0);
        engine.advance();
        engine.advance();
        assert_eq!(engine.step_count(), 2);
    }

    #[test]
    fn test_choice_type_modifies_archetype_profile() {
        let mut profile = [0.5; 22];

        modify_archetype_for_choice(&mut profile, ChoiceType::Explore);
        for i in 0..=6 {
            assert!(
                (profile[i] - 0.55).abs() < 1e-10,
                "Mind archetype {} should be boosted",
                i
            );
        }
        for i in 7..=21 {
            assert!(
                (profile[i] - 0.5).abs() < 1e-10,
                "Non-Mind archetype {} should be unchanged",
                i
            );
        }

        let mut profile2 = [0.5; 22];
        modify_archetype_for_choice(&mut profile2, ChoiceType::Socialize);
        for i in 14..=20 {
            assert!(
                (profile2[i] - 0.55).abs() < 1e-10,
                "Spirit archetype {} should be boosted",
                i
            );
        }

        let mut profile3 = [0.5; 22];
        modify_archetype_for_choice(&mut profile3, ChoiceType::Rest);
        for i in 7..=13 {
            assert!(
                (profile3[i] - 0.55).abs() < 1e-10,
                "Body archetype {} should be boosted",
                i
            );
        }

        let mut profile4 = [0.5; 22];
        modify_archetype_for_choice(&mut profile4, ChoiceType::Gather);
        for i in 0..=3 {
            assert!(
                (profile4[i] - 0.53).abs() < 1e-10,
                "Mind archetype {} should be boosted by gather",
                i
            );
        }
        for i in 7..=13 {
            assert!(
                (profile4[i] - 0.53).abs() < 1e-10,
                "Body archetype {} should be boosted by gather",
                i
            );
        }
    }

    #[test]
    fn test_broadcast_choice_affects_all_entities() {
        let mut profiles: Vec<[Float; 22]> = vec![[0.3; 22], [0.4; 22], [0.2; 22]];

        for profile in &mut profiles {
            modify_archetype_for_choice(profile, ChoiceType::Explore);
        }

        assert!((profiles[0][0] - 0.35).abs() < 1e-10);
        assert!((profiles[1][0] - 0.45).abs() < 1e-10);
        assert!((profiles[2][0] - 0.25).abs() < 1e-10);
        assert!((profiles[0][14] - 0.3).abs() < 1e-10);
        assert!((profiles[1][14] - 0.4).abs() < 1e-10);
        assert!((profiles[2][14] - 0.2).abs() < 1e-10);
    }
}
