//!
//! Collision Detection System
//!
//! From MASTER_R&D_ROADMAP.md Phase 4, Week 61-64: "Implement interference-based collision,
//! spectrum-based collision rules, density-based collision exceptions, observer effect on collision."
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.2: "Each entity contains within it all densities
//! and sub-densities of the octave. The difference is resolution, not substance."
//!
//! Collision detection in holographic physics:
//! - Interference-based: Collisions detected through pattern interference, not geometric bounds
//! - Spectrum-based: Collision rules depend on spectrum ratio (Space/Time vs Time/Space)
//! - Density-based: Different densities have different collision behavior
//! - Observer effect: Observation affects collision outcome

use super::holographic_physics::{HolographicEntity, SpectrumRatio};
use super::scale_specific_physics::ScaleSpecificPhysics;
use crate::types::{Density, Float};

pub const INTERFERENCE_COLLISION_THRESHOLD: Float = 0.5;
pub const SPECTRUM_COLLISION_MODIFIER: Float = 0.3;
pub const DENSITY_COLLISION_EXCEPTION_THRESHOLD: Float = 0.8;
pub const OBSERVER_COLLISION_INFLUENCE: Float = 0.2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    Physical,
    Metaphysical,
    Quantum,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CollisionEvent {
    pub collision_type: CollisionType,
    pub entity_a_id: u64,
    pub entity_b_id: u64,
    pub collision_point: [Float; 3],
    pub collision_normal: [Float; 3],
    pub impulse: Float,
    pub interference_strength: Float,
    pub spectrum_ratio: SpectrumRatio,
    pub observer_influence: Float,
}

impl Default for CollisionEvent {
    fn default() -> Self {
        CollisionEvent {
            collision_type: CollisionType::Void,
            entity_a_id: 0,
            entity_b_id: 0,
            collision_point: [0.0, 0.0, 0.0],
            collision_normal: [0.0, 0.0, 1.0],
            impulse: 0.0,
            interference_strength: 0.0,
            spectrum_ratio: SpectrumRatio::new(1.0, 1.0),
            observer_influence: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterferencePattern {
    pub amplitude: Float,
    pub phase: Float,
    pub frequency: Float,
    pub spatial_components: [Float; 3],
    pub temporal_components: [Float; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumCollisionRules {
    pub space_time_collision_elasticity: Float,
    pub time_space_collision_elasticity: Float,
    pub quantum_collision_probability: Float,
    pub spectrum_transition_collision: Float,
}

impl Default for SpectrumCollisionRules {
    fn default() -> Self {
        SpectrumCollisionRules {
            space_time_collision_elasticity: 0.9,
            time_space_collision_elasticity: 0.1,
            quantum_collision_probability: 0.5,
            spectrum_transition_collision: 0.5,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DensityCollisionExceptions {
    pub density_level: Density,
    pub collision_exceptions: Vec<DensityCollisionException>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DensityCollisionException {
    pub exception_type: CollisionExceptionType,
    pub target_density: Density,
    pub exception_strength: Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionExceptionType {
    PassThrough,
    Merge,
    Absorb,
    Reflect,
    Entangle,
}

#[derive(Debug, Clone)]
pub struct HolographicCollisionSystem {
    pub spectrum_rules: SpectrumCollisionRules,
    pub density_exceptions: Vec<DensityCollisionExceptions>,
    pub physics: ScaleSpecificPhysics,
    pub collision_cache: Vec<CollisionEvent>,
}

impl Default for HolographicCollisionSystem {
    fn default() -> Self {
        HolographicCollisionSystem {
            spectrum_rules: SpectrumCollisionRules::default(),
            density_exceptions: Vec::new(),
            physics: ScaleSpecificPhysics::default(),
            collision_cache: Vec::new(),
        }
    }
}

impl HolographicCollisionSystem {
    pub fn new(spectrum_rules: SpectrumCollisionRules) -> Self {
        HolographicCollisionSystem {
            spectrum_rules,
            density_exceptions: Vec::new(),
            physics: ScaleSpecificPhysics::default(),
            collision_cache: Vec::new(),
        }
    }

    pub fn with_density_exceptions(mut self, exceptions: Vec<DensityCollisionExceptions>) -> Self {
        self.density_exceptions = exceptions;
        self
    }

    pub fn compute_interference_pattern(&self, entity: &HolographicEntity) -> InterferencePattern {
        let mut amplitude = 0.0;
        let mut phase = 0.0;
        let mut spatial_components = [0.0; 3];
        let mut temporal_components = [0.0; 3];

        for i in 0..22 {
            let sig = entity.holographic_signature[i];
            amplitude += sig.abs();
            phase += sig.atan2(1.0);

            spatial_components[i % 3] += sig * entity.position[i % 3];
            temporal_components[i % 3] += sig * entity.velocity[i % 3];
        }

        amplitude /= 22.0;
        phase /= 22.0;

        let spatial_norm = spatial_components
            .iter()
            .map(|c| c * c)
            .sum::<Float>()
            .sqrt();
        let temporal_norm = temporal_components
            .iter()
            .map(|c| c * c)
            .sum::<Float>()
            .sqrt();

        if spatial_norm > 0.0 {
            spatial_components = spatial_components.map(|c| c / spatial_norm);
        }
        if temporal_norm > 0.0 {
            temporal_components = temporal_components.map(|c| c / temporal_norm);
        }

        let frequency = amplitude * entity.energy;

        InterferencePattern {
            amplitude,
            phase,
            frequency,
            spatial_components,
            temporal_components,
        }
    }

    pub fn detect_interference_collision(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> bool {
        let pattern_a = self.compute_interference_pattern(entity_a);
        let pattern_b = self.compute_interference_pattern(entity_b);

        let interference_strength = self.compute_interference_strength(&pattern_a, &pattern_b);

        interference_strength > INTERFERENCE_COLLISION_THRESHOLD
    }

    pub fn compute_interference_strength(
        &self,
        pattern_a: &InterferencePattern,
        pattern_b: &InterferencePattern,
    ) -> Float {
        let amplitude_diff = (pattern_a.amplitude - pattern_b.amplitude).abs();
        let phase_diff = (pattern_a.phase - pattern_b.phase).abs();
        let frequency_diff = (pattern_a.frequency - pattern_b.frequency).abs();

        let spatial_dot = pattern_a
            .spatial_components
            .iter()
            .zip(pattern_b.spatial_components.iter())
            .map(|(a, b)| a * b)
            .sum::<Float>();

        let temporal_dot = pattern_a
            .temporal_components
            .iter()
            .zip(pattern_b.temporal_components.iter())
            .map(|(a, b)| a * b)
            .sum::<Float>();

        let amplitude_factor =
            1.0 - amplitude_diff / (pattern_a.amplitude + pattern_b.amplitude + 1e-10);
        let phase_factor = (phase_diff.cos() + 1.0) / 2.0;
        let frequency_factor =
            1.0 - frequency_diff / (pattern_a.frequency + pattern_b.frequency + 1e-10);
        let spatial_factor = (spatial_dot + 1.0) / 2.0;
        let temporal_factor = (temporal_dot + 1.0) / 2.0;

        amplitude_factor * phase_factor * frequency_factor * spatial_factor * temporal_factor
    }

    pub fn apply_spectrum_collision_rules(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Float {
        let spectrum_ratio = SpectrumRatio::new(
            entity_a.spectrum_ratio.space_time_ratio + entity_b.spectrum_ratio.space_time_ratio,
            entity_a.spectrum_ratio.time_space_ratio + entity_b.spectrum_ratio.time_space_ratio,
        );

        if spectrum_ratio.is_space_time_dominant() {
            self.spectrum_rules.space_time_collision_elasticity
        } else if spectrum_ratio.is_time_space_dominant() {
            self.spectrum_rules.time_space_collision_elasticity
        } else {
            self.spectrum_rules.quantum_collision_probability
        }
    }

    pub fn check_density_collision_exception(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Option<CollisionExceptionType> {
        let pattern_a = self.compute_interference_pattern(entity_a);
        let pattern_b = self.compute_interference_pattern(entity_b);

        for density_exception in &self.density_exceptions {
            for exception in &density_exception.collision_exceptions {
                let exception_match = (exception.target_density == Density::First
                    || exception.target_density == Density::Second)
                    && (pattern_a.amplitude + pattern_b.amplitude)
                        > DENSITY_COLLISION_EXCEPTION_THRESHOLD;

                if exception_match {
                    return Some(exception.exception_type);
                }
            }
        }

        None
    }

    pub fn apply_observer_effect_on_collision(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        collision: &mut CollisionEvent,
    ) {
        let observer_count_a = entity_a.observed_by.len() as Float;
        let observer_count_b = entity_b.observed_by.len() as Float;
        let total_observers = observer_count_a + observer_count_b;

        if total_observers > 0.0 {
            let observer_influence = (total_observers * OBSERVER_COLLISION_INFLUENCE).min(1.0);
            collision.observer_influence = observer_influence;

            collision.impulse *= (1.0 + observer_influence).min(2.0);
            collision.interference_strength *= (1.0 + observer_influence * 0.5).min(1.5);
        }
    }

    pub fn compute_collision_event(
        &mut self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Option<CollisionEvent> {
        if !self.detect_interference_collision(entity_a, entity_b) {
            return None;
        }

        let exception_type = self.check_density_collision_exception(entity_a, entity_b);

        if let Some(exception) = exception_type {
            return self.handle_collision_exception(entity_a, entity_b, exception);
        }

        let pattern_a = self.compute_interference_pattern(entity_a);
        let pattern_b = self.compute_interference_pattern(entity_b);

        let interference_strength = self.compute_interference_strength(&pattern_a, &pattern_b);

        let spectrum_elasticity = self.apply_spectrum_collision_rules(entity_a, entity_b);

        let collision_type =
            self.determine_collision_type(entity_a, entity_b, interference_strength);

        let collision_point = self.compute_collision_point(entity_a, entity_b);
        let collision_normal = self.compute_collision_normal(entity_a, entity_b, &collision_point);

        let impulse = self.compute_impulse(
            entity_a,
            entity_b,
            interference_strength,
            spectrum_elasticity,
        );

        let mut collision = CollisionEvent {
            collision_type,
            entity_a_id: entity_a.entity_id,
            entity_b_id: entity_b.entity_id,
            collision_point,
            collision_normal,
            impulse,
            interference_strength,
            spectrum_ratio: SpectrumRatio::new(
                (entity_a.spectrum_ratio.space_time_ratio
                    + entity_b.spectrum_ratio.space_time_ratio)
                    / 2.0,
                (entity_a.spectrum_ratio.time_space_ratio
                    + entity_b.spectrum_ratio.time_space_ratio)
                    / 2.0,
            ),
            observer_influence: 0.0,
        };

        self.apply_observer_effect_on_collision(entity_a, entity_b, &mut collision);

        self.collision_cache.push(collision.clone());

        Some(collision)
    }

    pub fn determine_collision_type(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        interference_strength: Float,
    ) -> CollisionType {
        let combined_spectrum = SpectrumRatio::new(
            (entity_a.spectrum_ratio.space_time_ratio + entity_b.spectrum_ratio.space_time_ratio)
                / 2.0,
            (entity_a.spectrum_ratio.time_space_ratio + entity_b.spectrum_ratio.time_space_ratio)
                / 2.0,
        );

        if combined_spectrum.is_quantum() || interference_strength > 0.9 {
            CollisionType::Quantum
        } else if combined_spectrum.is_time_space_dominant() {
            CollisionType::Metaphysical
        } else if interference_strength > 0.5 {
            CollisionType::Physical
        } else {
            CollisionType::Void
        }
    }

    pub fn compute_collision_point(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> [Float; 3] {
        let total_mass = entity_a.mass + entity_b.mass;
        let ratio_a = entity_a.mass / total_mass;
        let ratio_b = entity_b.mass / total_mass;

        [
            entity_a.position[0] * ratio_a + entity_b.position[0] * ratio_b,
            entity_a.position[1] * ratio_a + entity_b.position[1] * ratio_b,
            entity_a.position[2] * ratio_a + entity_b.position[2] * ratio_b,
        ]
    }

    pub fn compute_collision_normal(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        collision_point: &[Float; 3],
    ) -> [Float; 3] {
        let direction_a = [
            entity_a.position[0] - collision_point[0],
            entity_a.position[1] - collision_point[1],
            entity_a.position[2] - collision_point[2],
        ];

        let norm = direction_a.iter().map(|c| c * c).sum::<Float>().sqrt();

        if norm > 0.0 {
            [
                direction_a[0] / norm,
                direction_a[1] / norm,
                direction_a[2] / norm,
            ]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    pub fn compute_impulse(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        interference_strength: Float,
        spectrum_elasticity: Float,
    ) -> Float {
        let relative_velocity = [
            entity_a.velocity[0] - entity_b.velocity[0],
            entity_a.velocity[1] - entity_b.velocity[1],
            entity_a.velocity[2] - entity_b.velocity[2],
        ];

        let velocity_magnitude = relative_velocity
            .iter()
            .map(|c| c * c)
            .sum::<Float>()
            .sqrt();

        let mass_factor = (entity_a.mass * entity_b.mass) / (entity_a.mass + entity_b.mass);

        velocity_magnitude * mass_factor * interference_strength * spectrum_elasticity
    }

    pub fn handle_collision_exception(
        &mut self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        exception_type: CollisionExceptionType,
    ) -> Option<CollisionEvent> {
        let collision_point = self.compute_collision_point(entity_a, entity_b);
        let collision_normal = self.compute_collision_normal(entity_a, entity_b, &collision_point);

        let impulse = match exception_type {
            CollisionExceptionType::PassThrough => 0.0,
            CollisionExceptionType::Merge => entity_a.mass + entity_b.mass,
            CollisionExceptionType::Absorb => entity_a.mass * 0.5,
            CollisionExceptionType::Reflect => entity_a.mass * 2.0,
            CollisionExceptionType::Entangle => entity_a.mass * 0.1,
        };

        let collision = CollisionEvent {
            collision_type: CollisionType::Metaphysical,
            entity_a_id: entity_a.entity_id,
            entity_b_id: entity_b.entity_id,
            collision_point,
            collision_normal,
            impulse,
            interference_strength: DENSITY_COLLISION_EXCEPTION_THRESHOLD,
            spectrum_ratio: SpectrumRatio::new(1.0, 1.0),
            observer_influence: 0.0,
        };

        self.collision_cache.push(collision.clone());

        Some(collision)
    }

    pub fn clear_collision_cache(&mut self) {
        self.collision_cache.clear();
    }

    pub fn get_collision_count(&self) -> usize {
        self.collision_cache.len()
    }

    pub fn get_recent_collisions(&self, limit: usize) -> Vec<CollisionEvent> {
        let start = if self.collision_cache.len() > limit {
            self.collision_cache.len() - limit
        } else {
            0
        };

        self.collision_cache[start..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_event_default() {
        let collision = CollisionEvent::default();
        assert_eq!(collision.collision_type, CollisionType::Void);
        assert_eq!(collision.entity_a_id, 0);
        assert_eq!(collision.entity_b_id, 0);
        assert_eq!(collision.impulse, 0.0);
    }

    #[test]
    fn test_interference_pattern_computation() {
        let system = HolographicCollisionSystem::default();
        let entity = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));

        let pattern = system.compute_interference_pattern(&entity);

        assert!(pattern.amplitude >= 0.0);
        assert!(pattern.frequency >= 0.0);
    }

    #[test]
    fn test_interference_strength_computation() {
        let system = HolographicCollisionSystem::default();

        let pattern_a = InterferencePattern {
            amplitude: 1.0,
            phase: 0.0,
            frequency: 1.0,
            spatial_components: [1.0, 0.0, 0.0],
            temporal_components: [1.0, 0.0, 0.0],
        };

        let pattern_b = InterferencePattern {
            amplitude: 1.0,
            phase: 0.0,
            frequency: 1.0,
            spatial_components: [1.0, 0.0, 0.0],
            temporal_components: [1.0, 0.0, 0.0],
        };

        let strength = system.compute_interference_strength(&pattern_a, &pattern_b);

        assert!(strength > 0.8);
    }

    #[test]
    fn test_interference_collision_detection() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [0.0, 0.0, 0.0];

        let collision = system.detect_interference_collision(&entity_a, &entity_b);

        assert!(collision);
    }

    #[test]
    fn test_no_collision_when_far_apart() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1000.0, 1000.0, 1000.0];

        let collision = system.detect_interference_collision(&entity_a, &entity_b);

        assert!(!collision);
    }

    #[test]
    fn test_spectrum_collision_rules_space_time() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(2.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(2.0, 1.0));

        let elasticity = system.apply_spectrum_collision_rules(&entity_a, &entity_b);

        assert_eq!(
            elasticity,
            system.spectrum_rules.space_time_collision_elasticity
        );
    }

    #[test]
    fn test_spectrum_collision_rules_time_space() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 2.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 2.0));

        let elasticity = system.apply_spectrum_collision_rules(&entity_a, &entity_b);

        assert_eq!(
            elasticity,
            system.spectrum_rules.time_space_collision_elasticity
        );
    }

    #[test]
    fn test_spectrum_collision_rules_quantum() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        let elasticity = system.apply_spectrum_collision_rules(&entity_a, &entity_b);

        assert_eq!(
            elasticity,
            system.spectrum_rules.quantum_collision_probability
        );
    }

    #[test]
    fn test_collision_point_computation() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [2.0, 0.0, 0.0];

        let point = system.compute_collision_point(&entity_a, &entity_b);

        assert_eq!(point[0], 1.0);
        assert_eq!(point[1], 0.0);
        assert_eq!(point[2], 0.0);
    }

    #[test]
    fn test_collision_normal_computation() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [2.0, 0.0, 0.0];

        let collision_point = [1.0, 0.0, 0.0];
        let normal = system.compute_collision_normal(entity_a, entity_b, &collision_point);

        assert_eq!(normal[0], -1.0);
        assert_eq!(normal[1], 0.0);
        assert_eq!(normal[2], 0.0);
    }

    #[test]
    fn test_impulse_computation() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.velocity = [1.0, 0.0, 0.0];
        entity_b.velocity = [-1.0, 0.0, 0.0];
        entity_a.mass = 1.0;
        entity_b.mass = 1.0;

        let impulse = system.compute_impulse(&entity_a, &entity_b, 1.0, 1.0);

        assert!(impulse > 0.0);
    }

    #[test]
    fn test_observer_effect_on_collision() {
        let system = HolographicCollisionSystem::default();

        let mut entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let mut entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.observed_by = vec![10, 11, 12];
        entity_b.observed_by = vec![20, 21];

        let mut collision = CollisionEvent {
            impulse: 1.0,
            interference_strength: 1.0,
            ..Default::default()
        };

        system.apply_observer_effect_on_collision(&entity_a, &entity_b, &mut collision);

        assert!(collision.observer_influence > 0.0);
        assert!(collision.impulse > 1.0);
    }

    #[test]
    fn test_complete_collision_event() {
        let mut system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [0.0, 0.0, 0.0];
        entity_a.velocity = [1.0, 0.0, 0.0];
        entity_b.velocity = [-1.0, 0.0, 0.0];

        let collision = system.compute_collision_event(&entity_a, &entity_b);

        assert!(collision.is_some());
        assert_eq!(system.get_collision_count(), 1);
    }

    #[test]
    fn test_collision_cache_operations() {
        let mut system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [0.0, 0.0, 0.0];

        system.compute_collision_event(&entity_a, &entity_b);
        system.compute_collision_event(&entity_a, &entity_b);

        assert_eq!(system.get_collision_count(), 2);

        let recent = system.get_recent_collisions(1);
        assert_eq!(recent.len(), 1);

        system.clear_collision_cache();
        assert_eq!(system.get_collision_count(), 0);
    }

    #[test]
    fn test_collision_exception_types() {
        let mut system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        let passthrough = system.handle_collision_exception(
            &entity_a,
            &entity_b,
            CollisionExceptionType::PassThrough,
        );
        assert_eq!(passthrough.unwrap().impulse, 0.0);

        let merge =
            system.handle_collision_exception(&entity_a, &entity_b, CollisionExceptionType::Merge);
        assert_eq!(merge.unwrap().impulse, 2.0);

        let absorb =
            system.handle_collision_exception(&entity_a, &entity_b, CollisionExceptionType::Absorb);
        assert_eq!(absorb.unwrap().impulse, 0.5);

        let reflect = system.handle_collision_exception(
            &entity_a,
            &entity_b,
            CollisionExceptionType::Reflect,
        );
        assert_eq!(reflect.unwrap().impulse, 2.0);

        let entangle = system.handle_collision_exception(
            &entity_a,
            &entity_b,
            CollisionExceptionType::Entangle,
        );
        assert_eq!(entangle.unwrap().impulse, 0.1);
    }

    #[test]
    fn test_collision_type_determination() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        let collision_type = system.determine_collision_type(&entity_a, &entity_b, 0.95);
        assert_eq!(collision_type, CollisionType::Quantum);

        let collision_type = system.determine_collision_type(&entity_a, &entity_b, 0.7);
        assert_eq!(collision_type, CollisionType::Physical);

        let collision_type = system.determine_collision_type(&entity_a, &entity_b, 0.3);
        assert_eq!(collision_type, CollisionType::Void);
    }

    #[test]
    fn test_spectrum_collision_rules_default() {
        let rules = SpectrumCollisionRules::default();

        assert_eq!(rules.space_time_collision_elasticity, 0.9);
        assert_eq!(rules.time_space_collision_elasticity, 0.1);
        assert_eq!(rules.quantum_collision_probability, 0.5);
        assert_eq!(rules.spectrum_transition_collision, 0.5);
    }

    #[test]
    fn test_density_collision_exception_structure() {
        let exception = DensityCollisionException {
            exception_type: CollisionExceptionType::PassThrough,
            target_density: Density::First,
            exception_strength: 0.8,
        };

        let density_exceptions = DensityCollisionExceptions {
            density_level: Density::First,
            collision_exceptions: vec![exception],
        };

        assert_eq!(density_exceptions.collision_exceptions.len(), 1);
    }

    #[test]
    fn test_collision_system_builder() {
        let rules = SpectrumCollisionRules {
            space_time_collision_elasticity: 0.8,
            ..Default::default()
        };

        let system = HolographicCollisionSystem::new(rules);

        assert_eq!(system.spectrum_rules.space_time_collision_elasticity, 0.8);
    }

    #[test]
    fn test_collision_system_with_density_exceptions() {
        let rules = SpectrumCollisionRules::default();

        let exception = DensityCollisionException {
            exception_type: CollisionExceptionType::PassThrough,
            target_density: Density::First,
            exception_strength: 0.8,
        };

        let density_exceptions = DensityCollisionExceptions {
            density_level: Density::First,
            collision_exceptions: vec![exception],
        };

        let system = HolographicCollisionSystem::new(rules)
            .with_density_exceptions(vec![density_exceptions]);

        assert_eq!(system.density_exceptions.len(), 1);
    }

    #[test]
    fn test_interference_pattern_phase_computation() {
        let system = HolographicCollisionSystem::default();
        let entity = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));

        entity.holographic_signature[0] = 1.0;

        let pattern = system.compute_interference_pattern(&entity);

        assert!(pattern.phase > 0.0);
    }

    #[test]
    fn test_interference_pattern_frequency_computation() {
        let system = HolographicCollisionSystem::default();
        let entity = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));

        entity.holographic_signature[0] = 1.0;
        entity.energy = 2.0;

        let pattern = system.compute_interference_pattern(&entity);

        assert!(pattern.frequency > 0.0);
    }

    #[test]
    fn test_collision_normal_default_case() {
        let system = HolographicCollisionSystem::default();

        let entity_a = HolographicEntity::new(1, SpectrumRatio::new(1.0, 1.0));
        let entity_b = HolographicEntity::new(2, SpectrumRatio::new(1.0, 1.0));

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [0.0, 0.0, 0.0];

        let collision_point = [0.0, 0.0, 0.0];
        let normal = system.compute_collision_normal(entity_a, entity_b, &collision_point);

        assert_eq!(normal, [0.0, 0.0, 1.0]);
    }
}
