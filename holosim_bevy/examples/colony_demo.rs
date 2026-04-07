//! Colony Demo — 60 entities with emergent need-driven behavior.
//!
//! Run with: `cargo run --example colony_demo --release -p holosim_bevy`

use bevy::prelude::*;
use bevy::time::Fixed;
use holonic_realms::simulation_v3::observation_layer::TerrainType;
use holonic_realms::types::Float;
use holosim_bevy::{
    apply_player_choices, handle_player_input, DerivedBehavior, DerivedEnvironment, DerivedPhysics,
    HoloSimEngine, HoloSimEntity, HoloSimPlugin, HoloSimSet, PlayerInputState,
};

#[derive(Component)]
struct ColonyEntity;

#[derive(Resource)]
struct WorldBounds {
    min: Vec2,
    max: Vec2,
}

#[derive(Resource, Clone, Copy)]
struct ColonyConfig {
    entity_count: usize,
    world_size: Vec2,
}

impl Default for ColonyConfig {
    fn default() -> Self {
        Self {
            entity_count: 60,
            world_size: Vec2::new(800.0, 600.0),
        }
    }
}

#[derive(Resource, Default)]
struct ColonyCentroid {
    center: Vec2,
    positions: Vec<(u64, Vec2)>,
}

#[derive(Resource)]
struct StatsTimer(Timer);

#[derive(Resource, Default)]
struct HudStats {
    entity_count: usize,
    avg_mood: f64,
    avg_hunger: f64,
    avg_social: f64,
    avg_rest: f64,
    avg_exploration: f64,
    step_count: u64,
}

// ─── Archetype Color Mapping ────────────────────────────────────────────────

/// Maps archetype index to a representative color.
/// Mind (A1-A7): Cyan → Purple, Body (A8-A14): Green → Orange,
/// Spirit (A15-A21): Blue → Pink, Choice (A22): Gold.
fn archetype_color(index: usize) -> [f32; 3] {
    match index {
        0..=2 => [0.0, 0.74, 0.83],
        3..=6 => [0.61, 0.15, 0.69],
        7..=10 => [0.30, 0.69, 0.31],
        11..=13 => [1.0, 0.60, 0.0],
        14..=17 => [0.13, 0.59, 0.95],
        18..=20 => [0.91, 0.12, 0.39],
        21 => [1.0, 0.84, 0.0],
        _ => [1.0, 1.0, 1.0],
    }
}

fn blend_archetype_colors(profile: &[Float; 22]) -> Color {
    let mut indices: Vec<usize> = (0..22).collect();
    indices.sort_by(|a, b| profile[*b].partial_cmp(&profile[*a]).unwrap());

    let top3: Vec<(usize, f32)> = indices[..3.min(22)]
        .iter()
        .map(|&i| (i, profile[i] as f32))
        .collect();

    let total: f32 = top3.iter().map(|(_, v)| v).sum();
    if total == 0.0 {
        return Color::WHITE;
    }

    let mut r = 0.0f32;
    let mut g = 0.0f32;
    let mut b = 0.0f32;

    for (idx, weight) in &top3 {
        let c = archetype_color(*idx);
        let w = weight / total;
        r += c[0] * w;
        g += c[1] * w;
        b += c[2] * w;
    }

    Color::srgb(r, g, b)
}

fn xorshift32(mut seed: u32) -> u32 {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    seed
}

fn xorshift_range(seed: u32, min: f64, max: f64) -> f64 {
    let x = xorshift32(seed) as f64 / u32::MAX as f64;
    min + x * (max - min)
}

fn spawn_colony(mut commands: Commands, config: Res<ColonyConfig>) {
    for i in 0..config.entity_count {
        let entity_id = (i + 1) as u64;
        let base_seed = entity_id as u32;

        let mut archetype_profile = [0.0f64; 22];
        for j in 0..22 {
            archetype_profile[j] = xorshift_range(
                base_seed.wrapping_mul(31).wrapping_add(j as u32),
                0.05,
                0.95,
            );
        }

        let x = xorshift_range(base_seed.wrapping_mul(37), 50.0, 750.0);
        let y = xorshift_range(base_seed.wrapping_mul(41), 50.0, 550.0);

        let pos = Vec3::new(x as f32, y as f32, 0.0);

        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            Transform::from_translation(pos),
            Visibility::default(),
            ColonyEntity,
            HoloSimEntity {
                entity_id,
                archetype_profile,
                density: 2,
                spectrum_position: 0.6,
                coherence: 0.8,
            },
            DerivedPhysics {
                position: [x, y, 0.0],
                velocity: [0.0; 3],
                mass: 1.0,
                temperature: 300.0,
                charge: 0.0,
                health: 1.0,
                energy_level: 0.8,
            },
            DerivedBehavior {
                needs: [0.0; 5],
                mood: 0.0,
                current_goal: None,
                personality_profile: archetype_profile,
                stability: 0.5,
                novelty: 0.5,
            },
            DerivedEnvironment {
                terrain: TerrainType::Planetary,
                weather_intensity: 0.0,
                resource_density: 0.5,
                danger_level: 0.0,
                social_density: 0.0,
            },
        ));
    }
}

fn initialize_observations(
    mut engine: ResMut<HoloSimEngine>,
    entities: Query<(&HoloSimEntity, &DerivedPhysics)>,
) {
    for (holo, physics) in &entities {
        engine.observe_entity(
            holo.entity_id,
            &holo.archetype_profile,
            holo.spectrum_position,
            holo.density,
            physics.position,
            physics.velocity,
            physics.mass,
            physics.energy_level,
            holo.coherence,
        );
    }
}

fn simulation_tick(
    mut engine: ResMut<HoloSimEngine>,
    entities: Query<(&HoloSimEntity, &DerivedPhysics)>,
) {
    for (holo, physics) in &entities {
        engine.observe_entity(
            holo.entity_id,
            &holo.archetype_profile,
            holo.spectrum_position,
            holo.density,
            physics.position,
            physics.velocity,
            physics.mass,
            physics.energy_level,
            holo.coherence,
        );
    }
    engine.advance();
}

fn compute_centroid(
    entities: Query<(&HoloSimEntity, &DerivedPhysics)>,
    mut centroid: ResMut<ColonyCentroid>,
) {
    centroid.positions.clear();
    let count = entities.iter().len() as f32;
    if count == 0.0 {
        centroid.center = Vec2::ZERO;
        return;
    }
    let mut sum = Vec2::ZERO;
    for (holo, physics) in &entities {
        let pos = Vec2::new(physics.position[0] as f32, physics.position[1] as f32);
        centroid.positions.push((holo.entity_id, pos));
        sum += pos;
    }
    centroid.center = sum / count;
}

fn need_driven_movement(
    bounds: Res<WorldBounds>,
    time: Res<Time>,
    centroid: Res<ColonyCentroid>,
    mut entities: Query<(
        &HoloSimEntity,
        &DerivedBehavior,
        &mut DerivedPhysics,
        &mut Transform,
    )>,
) {
    for (holo, behavior, mut physics, mut transform) in entities.iter_mut() {
        let current_pos = Vec2::new(physics.position[0] as f32, physics.position[1] as f32);
        let mut direction = Vec2::ZERO;
        let mut speed = 100.0;

        let hunger = behavior.needs[DerivedBehavior::NEED_HUNGER];
        let social = behavior.needs[DerivedBehavior::NEED_SOCIAL];
        let rest = behavior.needs[DerivedBehavior::NEED_REST];
        let exploration = behavior.needs[DerivedBehavior::NEED_EXPLORATION];

        if rest > 0.5 {
            speed = 20.0;
        }

        if hunger > 0.5 {
            let resource_zone = Vec2::new(400.0, 300.0);
            direction += (resource_zone - current_pos).normalize_or_zero();
        }

        if social > 0.5 {
            let mut nearest_dir = Vec2::ZERO;
            let mut nearest_dist = f32::MAX;
            for (other_id, other_pos) in &centroid.positions {
                if *other_id != holo.entity_id {
                    let diff = *other_pos - current_pos;
                    let dist = diff.length();
                    if dist < nearest_dist && dist > 0.001 {
                        nearest_dist = dist;
                        nearest_dir = diff.normalize();
                    }
                }
            }
            direction += nearest_dir;
        }

        if exploration > 0.5 {
            let seed = (holo.entity_id as u32).wrapping_mul(7919);
            let angle = xorshift_range(seed, 0.0, std::f64::consts::PI * 2.0) as f32;
            direction += Vec2::new(angle.cos(), angle.sin());
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            let delta = direction * speed * time.delta_secs();
            let new_pos = current_pos + delta;
            let clamped_pos = Vec2::new(
                new_pos.x.clamp(bounds.min.x, bounds.max.x),
                new_pos.y.clamp(bounds.min.y, bounds.max.y),
            );

            physics.position = [clamped_pos.x as Float, clamped_pos.y as Float, 0.0];
            transform.translation.x = clamped_pos.x;
            transform.translation.y = clamped_pos.y;
        }
    }
}

fn need_decay(mut entities: Query<&mut DerivedBehavior>) {
    for mut behavior in entities.iter_mut() {
        for need in &mut behavior.needs {
            *need = (*need + 0.02).clamp(0.0, 1.0);
        }
    }
}

fn print_colony_stats(
    time: Res<Time>,
    mut timer: ResMut<StatsTimer>,
    entities: Query<(&HoloSimEntity, &DerivedBehavior)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let count = entities.iter().len();
        if count == 0 {
            info!("Colony Stats — no entities");
            return;
        }

        let mut total_mood = 0.0f64;
        let mut need_sums = [0.0f64; 5];
        let mut need_counts = [0usize; 5];

        for (_, behavior) in &entities {
            total_mood += behavior.mood;
            for i in 0..5 {
                need_sums[i] += behavior.needs[i] as f64;
                if behavior.needs[i] > 0.5 {
                    need_counts[i] += 1;
                }
            }
        }

        let count_f = count as f64;
        let avg_mood = total_mood / count_f;

        info!(
            "=== Colony Stats — entities: {}, avg mood: {:.3} ===",
            count, avg_mood
        );
        info!(
            "  hunger:     avg={:.3}  urgent={}/{}",
            need_sums[0] / count_f,
            need_counts[0],
            count
        );
        info!(
            "  social:     avg={:.3}  urgent={}/{}",
            need_sums[1] / count_f,
            need_counts[1],
            count
        );
        info!(
            "  rest:       avg={:.3}  urgent={}/{}",
            need_sums[2] / count_f,
            need_counts[2],
            count
        );
        info!(
            "  exploration:avg={:.3}  urgent={}/{}",
            need_sums[3] / count_f,
            need_counts[3],
            count
        );
        info!(
            "  growth:     avg={:.3}  urgent={}/{}",
            need_sums[4] / count_f,
            need_counts[4],
            count
        );
    }
}

fn main() {
    let config = ColonyConfig::default();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1280, 720).into(),
                title: "HoloSim Colony Demo".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(HoloSimPlugin)
        .insert_resource(config)
        .insert_resource(WorldBounds {
            min: Vec2::ZERO,
            max: config.world_size,
        })
        .insert_resource(ColonyCentroid::default())
        .insert_resource(StatsTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(HudStats::default())
        .insert_resource(Time::<Fixed>::from_hz(4.0))
        .add_systems(
            Startup,
            (spawn_camera, spawn_colony, initialize_observations).chain(),
        )
        .add_systems(FixedUpdate, (simulation_tick, need_decay).chain())
        .add_systems(
            Update,
            (
                compute_centroid.after(HoloSimSet),
                need_driven_movement.after(compute_centroid),
                update_entity_colors,
                update_selection_highlight.after(update_entity_colors),
                handle_player_input,
                apply_player_choices.after(handle_player_input),
                (compute_hud_stats, render_hud)
                    .chain()
                    .after(need_driven_movement),
                print_colony_stats,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update_entity_colors(mut entities: Query<(&HoloSimEntity, &mut Sprite)>) {
    for (holo, mut sprite) in entities.iter_mut() {
        sprite.color = blend_archetype_colors(&holo.archetype_profile);
    }
}

fn update_selection_highlight(
    player_input: Res<PlayerInputState>,
    mut entities: Query<(&HoloSimEntity, &mut Sprite, &mut Transform)>,
) {
    for (holo, mut sprite, mut transform) in entities.iter_mut() {
        let is_selected = player_input.selected_entity == Some(holo.entity_id);
        let scale = if is_selected { 1.8 } else { 1.0 };
        transform.scale = Vec3::splat(scale);
        if is_selected {
            sprite.color = Color::WHITE;
        }
    }
}

fn choice_key_hints() -> String {
    "Keys: [E]xplore [S]ocialize [R]est [G]ather | Click to select".to_string()
}

fn compute_hud_stats(
    engine: Res<HoloSimEngine>,
    entities: Query<(&HoloSimEntity, &DerivedBehavior)>,
    mut hud: ResMut<HudStats>,
) {
    let count = entities.iter().len();
    if count == 0 {
        hud.entity_count = 0;
        return;
    }

    let mut total_mood = 0.0f64;
    let mut need_sums = [0.0f64; 5];
    let count_f = count as f64;

    for (_, behavior) in &entities {
        total_mood += behavior.mood;
        for i in 0..5 {
            need_sums[i] += behavior.needs[i] as f64;
        }
    }

    hud.entity_count = count;
    hud.avg_mood = total_mood / count_f;
    hud.avg_hunger = need_sums[0] / count_f;
    hud.avg_social = need_sums[1] / count_f;
    hud.avg_rest = need_sums[2] / count_f;
    hud.avg_exploration = need_sums[3] / count_f;
    hud.step_count = engine.step_count();
}

fn render_hud(
    hud: Res<HudStats>,
    player_input: Res<PlayerInputState>,
    mut commands: Commands,
    existing: Query<Entity, With<HeadHud>>,
) {
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    if hud.entity_count == 0 {
        return;
    }

    let need_bar = |label: &str, value: f64| -> String {
        let filled = (value * 10.0).round() as usize;
        let bar: String = (0..10)
            .map(|i| if i < filled { "#" } else { "." })
            .collect();
        format!("  {} [{:<10}] {:>3.0}%\n", label, bar, value * 100.0)
    };

    let selection_info = match player_input.selected_entity {
        Some(id) => format!("Selected: Entity #{}", id),
        None => "Selected: None (click entity)".to_string(),
    };

    let content = format!(
        "HOLOSIM COLONY — Step {}\n\
         Entities: {}\n\
         {}\n\
         Mood: {:.2}\n\
         \n{}\n{}\n{}\n{}\n{}\n\
         \n{}\n",
        hud.step_count,
        hud.entity_count,
        selection_info,
        hud.avg_mood,
        need_bar("Hunger", hud.avg_hunger),
        need_bar("Social", hud.avg_social),
        need_bar("Rest", hud.avg_rest),
        need_bar("Explore", hud.avg_exploration),
        need_bar("Growth", hud.avg_exploration),
        choice_key_hints(),
    );

    commands.spawn((
        Text::new(content),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.85, 0.7)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        HeadHud,
    ));
}

#[derive(Component)]
struct HeadHud;
