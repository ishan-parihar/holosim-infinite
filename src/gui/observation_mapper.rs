//! ObservationMapper — Maps consciousness state to ObservableProperties.
//!
//! This is the core bridge: archetype interference drives behavior,
//! consciousness drives appearance, density drives capabilities.

use crate::gui::observable_properties::{BehaviorState, EntityShape, ObservableProperties};

pub struct ObservationMapper;

impl ObservationMapper {
    /// Map consciousness state to ObservableProperties.
    pub fn map(
        archetype_activations: &[f64; 22],
        consciousness_level: f64,
        density: u8,
        spectrum_position: f64,
        polarization: f64,
        entity_index: usize,
    ) -> ObservableProperties {
        let mut props = ObservableProperties::default();
        props.color = Self::archetypes_to_color(archetype_activations);
        props.size = Self::density_to_size(density);
        props.shape = Self::density_to_shape(density);
        props.glow = consciousness_level as f32;
        props.behavior_state = Self::archetypes_to_behavior(archetype_activations);
        props.intelligence = consciousness_level as f32;
        props.service_orientation = polarization as f32;
        props.density = density;
        props.top_archetypes = Self::top_n_archetypes(archetype_activations, 3);
        props.position = Self::initial_position(entity_index, 100);
        props.velocity = Self::behavior_to_velocity(&props.behavior_state, spectrum_position);
        props
    }

    /// Mind archetypes (0-6) → Red, Body (7-13) → Green, Spirit (14-20) → Blue, Choice (21) → brightness
    fn archetypes_to_color(activations: &[f64; 22]) -> [f32; 3] {
        let mind_avg: f64 = activations[0..7].iter().sum::<f64>() / 7.0;
        let body_avg: f64 = activations[7..14].iter().sum::<f64>() / 7.0;
        let spirit_avg: f64 = activations[14..21].iter().sum::<f64>() / 7.0;
        let choice = activations[21];
        let brightness = 0.5 + choice * 0.5;
        [
            (mind_avg * brightness) as f32,
            (body_avg * brightness) as f32,
            (spirit_avg * brightness) as f32,
        ]
    }

    fn density_to_size(density: u8) -> f32 {
        match density {
            1 => 0.5,
            2 => 0.7,
            3 => 1.0,
            4 => 1.2,
            5 => 1.4,
            6 => 1.6,
            7 => 1.8,
            8 => 2.0,
            _ => 1.0,
        }
    }

    fn density_to_shape(density: u8) -> EntityShape {
        match density {
            1..=2 => EntityShape::Circle,
            3 => EntityShape::Triangle,
            4 => EntityShape::Square,
            5 => EntityShape::Diamond,
            6 => EntityShape::Hexagon,
            7..=8 => EntityShape::Star,
            _ => EntityShape::Circle,
        }
    }

    /// Map dominant archetype to behavior:
    /// Catalyst (3,10,17)→Seeking, Tower(11)→Fleeing, Lovers(6)/Hierophant(5)→Socializing,
    /// Hermit(9)/HighPriestess(2)→Contemplating, Emperor(4)/Chariot(7)→Leading
    fn archetypes_to_behavior(activations: &[f64; 22]) -> BehaviorState {
        let max_idx = activations
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);
        match max_idx {
            3 | 10 | 17 => BehaviorState::Seeking,
            11 => BehaviorState::Fleeing,
            6 | 5 => BehaviorState::Socializing,
            9 | 2 => BehaviorState::Contemplating,
            4 | 7 => BehaviorState::Leading,
            _ => BehaviorState::Idle,
        }
    }

    fn top_n_archetypes(activations: &[f64; 22], n: usize) -> [u8; 3] {
        let mut indexed: Vec<(usize, f64)> = activations
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut result = [0u8; 3];
        for i in 0..n.min(3) {
            result[i] = indexed[i].0 as u8;
        }
        result
    }

    fn initial_position(index: usize, radius: usize) -> [f32; 2] {
        let count = (index as f64 + 1.0).max(1.0);
        let angle = (count * 0.3) % std::f64::consts::TAU;
        let r = (radius as f64) * 0.3;
        [(angle.cos() * r) as f32, (angle.sin() * r) as f32]
    }

    fn behavior_to_velocity(behavior: &BehaviorState, spectrum_position: f64) -> [f32; 2] {
        let speed = match behavior {
            BehaviorState::Seeking => 2.0,
            BehaviorState::Fleeing => 3.0,
            BehaviorState::Socializing => 1.0,
            BehaviorState::Contemplating => 0.2,
            BehaviorState::Leading => 1.5,
            _ => 0.0,
        };
        let angle = spectrum_position * std::f64::consts::TAU;
        let randomness = spectrum_position as f32 * 0.5;
        [
            (angle.cos() as f32 * speed) + randomness - 0.25,
            (angle.sin() as f32 * speed) + randomness - 0.25,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_mind_dominant() {
        let mut acts = [0.0f64; 22];
        for i in 0..7 {
            acts[i] = 0.9;
        }
        let color = ObservationMapper::archetypes_to_color(&acts);
        assert!(
            color[0] > color[2],
            "Red channel should dominate for mind-heavy"
        );
    }

    #[test]
    fn test_color_from_spirit_dominant() {
        let mut acts = [0.0f64; 22];
        for i in 14..21 {
            acts[i] = 0.9;
        }
        let color = ObservationMapper::archetypes_to_color(&acts);
        assert!(
            color[2] > color[0],
            "Blue channel should dominate for spirit-heavy"
        );
    }

    #[test]
    fn test_density_to_size() {
        assert_eq!(ObservationMapper::density_to_size(1), 0.5);
        assert_eq!(ObservationMapper::density_to_size(3), 1.0);
        assert_eq!(ObservationMapper::density_to_size(8), 2.0);
    }

    #[test]
    fn test_density_to_shape() {
        assert_eq!(
            ObservationMapper::density_to_shape(3),
            EntityShape::Triangle
        );
        assert_eq!(ObservationMapper::density_to_shape(5), EntityShape::Diamond);
        assert_eq!(ObservationMapper::density_to_shape(7), EntityShape::Star);
        assert_eq!(ObservationMapper::density_to_shape(1), EntityShape::Circle);
    }

    #[test]
    fn test_behavior_from_catalyst() {
        let mut acts = [0.0f64; 22];
        acts[3] = 0.95;
        assert_eq!(
            ObservationMapper::archetypes_to_behavior(&acts),
            BehaviorState::Seeking
        );
    }

    #[test]
    fn test_behavior_from_tower() {
        let mut acts = [0.0f64; 22];
        acts[11] = 0.95;
        assert_eq!(
            ObservationMapper::archetypes_to_behavior(&acts),
            BehaviorState::Fleeing
        );
    }

    #[test]
    fn test_behavior_from_hermit() {
        let mut acts = [0.0f64; 22];
        acts[9] = 0.95;
        assert_eq!(
            ObservationMapper::archetypes_to_behavior(&acts),
            BehaviorState::Contemplating
        );
    }

    #[test]
    fn test_top_archetypes() {
        let mut acts = [0.0f64; 22];
        acts[5] = 0.9;
        acts[3] = 0.8;
        acts[10] = 0.7;
        let top = ObservationMapper::top_n_archetypes(&acts, 3);
        assert_eq!(top[0], 5);
        assert_eq!(top[1], 3);
        assert_eq!(top[2], 10);
    }

    #[test]
    fn test_map_full_integration() {
        let mut acts = [0.1f64; 22];
        acts[3] = 0.9;
        let props = ObservationMapper::map(&acts, 0.75, 3, 0.4, 0.5, 0);
        assert!(props.glow > 0.7);
        assert_eq!(props.behavior_state, BehaviorState::Seeking);
        assert_eq!(props.density, 3);
        assert!(props.position[0] != 0.0 || props.position[1] != 0.0);
    }

    #[test]
    fn test_map_handles_zero_density() {
        let acts = [0.1f64; 22];
        let props = ObservationMapper::map(&acts, 0.5, 0, 0.5, 0.0, 5);
        assert_eq!(props.shape, EntityShape::Circle);
    }
}
