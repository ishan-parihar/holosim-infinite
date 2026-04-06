//! Holographic Physics Engine
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.5: "The Larson Reciprocal Framework"
//! "Space and time are not independent variables but fundamentally reciprocal aspects
//! of a unified progression expressed through motion."
//!
//! Physics Modes:
//! - SpaceTime: v = s/t (3D space, 1D time) - Many-ness dominant
//! - TimeSpace: v = t/s (1D space, 3D time) - Oneness dominant
//! - Quantum: v ≈ 1 (the Veil) - transition point

use crate::types::Float;

pub const SPACE_TIME_THRESHOLD: Float = 1.1;
pub const TIME_SPACE_THRESHOLD: Float = 1.1;
pub const QUANTUM_TOLERANCE: Float = 0.1;

pub const MAX_INTERACTION_DISTANCE: Float = 1e26;
pub const MIN_INTERACTION_DISTANCE: Float = 1e-35;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolographicPhysicsMode {
    SpaceTime,
    TimeSpace,
    Quantum,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractionType {
    Gravitational,
    Electromagnetic,
    QuantumEntanglement,
    Metaphysical,
    Resonance,
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectrumRatio {
    pub space_time_ratio: Float,
    pub time_space_ratio: Float,
    pub spectrum_position: Float,
}

impl SpectrumRatio {
    pub fn new(space_time_ratio: Float, time_space_ratio: Float) -> Self {
        let spectrum_position = space_time_ratio / (space_time_ratio + time_space_ratio);
        SpectrumRatio {
            space_time_ratio,
            time_space_ratio,
            spectrum_position,
        }
    }

    pub fn compute_from_position(spectrum_position: Float) -> Self {
        if spectrum_position < 0.5 {
            let st_ratio = spectrum_position * 2.0;
            let ts_ratio = 2.0 - st_ratio;
            SpectrumRatio::new(st_ratio, ts_ratio)
        } else {
            let ts_ratio = (spectrum_position - 0.5) * 2.0;
            let st_ratio = 2.0 - ts_ratio;
            SpectrumRatio::new(st_ratio, ts_ratio)
        }
    }

    pub fn is_space_time_dominant(&self) -> bool {
        self.space_time_ratio > self.time_space_ratio * SPACE_TIME_THRESHOLD
    }

    pub fn is_time_space_dominant(&self) -> bool {
        self.time_space_ratio > self.space_time_ratio * TIME_SPACE_THRESHOLD
    }

    pub fn is_quantum(&self) -> bool {
        let ratio = self.space_time_ratio / self.time_space_ratio;
        (ratio - 1.0).abs() < QUANTUM_TOLERANCE
    }
}

#[derive(Debug, Clone)]
pub struct Interaction {
    pub interaction_type: InteractionType,
    pub strength: Float,
    pub direction: [Float; 3],
    pub probability: Float,
    pub observer_influence: Float,
    pub spectrum_mode: HolographicPhysicsMode,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction {
            interaction_type: InteractionType::Void,
            strength: 0.0,
            direction: [0.0, 0.0, 0.0],
            probability: 0.0,
            observer_influence: 0.0,
            spectrum_mode: HolographicPhysicsMode::Quantum,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpectrumThresholds {
    pub space_time_threshold: Float,
    pub time_space_threshold: Float,
    pub quantum_tolerance: Float,
}

impl Default for SpectrumThresholds {
    fn default() -> Self {
        SpectrumThresholds {
            space_time_threshold: SPACE_TIME_THRESHOLD,
            time_space_threshold: TIME_SPACE_THRESHOLD,
            quantum_tolerance: QUANTUM_TOLERANCE,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicsInterferencePattern {
    pub amplitude: Float,
    pub phase: Float,
    pub frequency: Float,
    pub spatial_components: [Float; 3],
    pub temporal_components: [Float; 3],
}

#[derive(Debug, Clone)]
pub struct HolographicEntity {
    pub entity_id: u64,
    pub spectrum_ratio: SpectrumRatio,
    pub holographic_signature: [Float; 22],
    pub position: [Float; 3],
    pub velocity: [Float; 3],
    pub mass: Float,
    pub energy: Float,
    pub observed_by: Vec<u64>,
}

impl HolographicEntity {
    pub fn new(entity_id: u64, spectrum_ratio: SpectrumRatio) -> Self {
        HolographicEntity {
            entity_id,
            spectrum_ratio,
            holographic_signature: [0.0; 22],
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            mass: 1.0,
            energy: 1.0,
            observed_by: Vec::new(),
        }
    }

    pub fn spectrum_ratio(&self) -> &SpectrumRatio {
        &self.spectrum_ratio
    }
}

#[derive(Debug, Clone)]
pub enum HolographicPhysicsError {
    InvalidSpectrumRatio(String),
    InvalidEntity(String),
    ComputationFailed(String),
    ModeTransitionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PhysicsEngineStatistics {
    pub total_interactions_computed: u64,
    pub space_time_interactions: u64,
    pub time_space_interactions: u64,
    pub quantum_interactions: u64,
    pub average_computation_time_ns: u64,
}

pub struct HolographicPhysicsEngine {
    current_mode: HolographicPhysicsMode,
    #[allow(dead_code)]
    spectrum_thresholds: SpectrumThresholds,
    statistics: PhysicsEngineStatistics,
}

impl HolographicPhysicsEngine {
    pub fn new() -> Self {
        HolographicPhysicsEngine {
            current_mode: HolographicPhysicsMode::Quantum,
            spectrum_thresholds: SpectrumThresholds::default(),
            statistics: PhysicsEngineStatistics::default(),
        }
    }

    pub fn with_thresholds(spectrum_thresholds: SpectrumThresholds) -> Self {
        HolographicPhysicsEngine {
            current_mode: HolographicPhysicsMode::Quantum,
            spectrum_thresholds,
            statistics: PhysicsEngineStatistics::default(),
        }
    }

    pub fn get_statistics(&self) -> &PhysicsEngineStatistics {
        &self.statistics
    }

    pub fn reset_statistics(&mut self) {
        self.statistics = PhysicsEngineStatistics::default();
    }

    pub fn determine_physics_mode(
        &self,
        ratio_a: &SpectrumRatio,
        ratio_b: &SpectrumRatio,
    ) -> HolographicPhysicsMode {
        let combined_ratio = SpectrumRatio::new(
            (ratio_a.space_time_ratio + ratio_b.space_time_ratio) / 2.0,
            (ratio_a.time_space_ratio + ratio_b.time_space_ratio) / 2.0,
        );

        if combined_ratio.is_quantum() {
            HolographicPhysicsMode::Quantum
        } else if combined_ratio.is_space_time_dominant() {
            HolographicPhysicsMode::SpaceTime
        } else if combined_ratio.is_time_space_dominant() {
            HolographicPhysicsMode::TimeSpace
        } else {
            HolographicPhysicsMode::Quantum
        }
    }

    pub fn set_physics_mode(&mut self, mode: HolographicPhysicsMode) {
        self.current_mode = mode;
    }

    pub fn get_current_mode(&self) -> HolographicPhysicsMode {
        self.current_mode
    }

    pub fn compute_interaction(
        &mut self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        if entity_a.entity_id == entity_b.entity_id {
            return Err(HolographicPhysicsError::InvalidEntity(
                "Cannot compute interaction with itself".to_string(),
            ));
        }

        let mode =
            self.determine_physics_mode(entity_a.spectrum_ratio(), entity_b.spectrum_ratio());

        let interaction = match mode {
            HolographicPhysicsMode::SpaceTime => {
                self.statistics.space_time_interactions += 1;
                self.compute_space_time_interaction(entity_a, entity_b)?
            }
            HolographicPhysicsMode::TimeSpace => {
                self.statistics.time_space_interactions += 1;
                self.compute_time_space_interaction(entity_a, entity_b)?
            }
            HolographicPhysicsMode::Quantum => {
                self.statistics.quantum_interactions += 1;
                self.compute_quantum_interaction(entity_a, entity_b)?
            }
        };

        self.statistics.total_interactions_computed += 1;

        Ok(interaction)
    }

    fn compute_space_time_interaction(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;

        if distance_sq < MIN_INTERACTION_DISTANCE * MIN_INTERACTION_DISTANCE {
            return Err(HolographicPhysicsError::ComputationFailed(
                "Distance below minimum threshold".to_string(),
            ));
        }

        let distance = distance_sq.sqrt();
        let mut strength = (entity_a.mass * entity_b.mass) / distance_sq;

        let gravitational_strength = strength;
        let electromagnetic_strength =
            self.compute_electromagnetic_component(entity_a, entity_b, distance);
        strength = gravitational_strength + electromagnetic_strength;

        if distance > MAX_INTERACTION_DISTANCE {
            strength *= 1e-20;
        }

        let mut probability = 1.0;
        let observer_influence = self.compute_observer_influence(entity_a, entity_b);
        probability *= (1.0 + observer_influence).min(2.0);

        let direction = if distance > 0.0 {
            let inv_distance = 1.0 / distance;
            [-dx * inv_distance, -dy * inv_distance, -dz * inv_distance]
        } else {
            [0.0, 0.0, 0.0]
        };

        let interaction_type = if gravitational_strength > electromagnetic_strength {
            InteractionType::Gravitational
        } else {
            InteractionType::Electromagnetic
        };

        Ok(Interaction {
            interaction_type,
            strength,
            direction,
            probability,
            observer_influence,
            spectrum_mode: HolographicPhysicsMode::SpaceTime,
        })
    }

    fn compute_time_space_interaction(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt().max(1.0);

        let archetype_resonance = self.compute_archetype_resonance(entity_a, entity_b);
        let temporal_proximity = self.compute_temporal_proximity(entity_a, entity_b);

        let mut strength = archetype_resonance * temporal_proximity;
        strength = strength * entity_a.energy * entity_b.energy;

        let observer_influence = self.compute_observer_influence(entity_a, entity_b);
        strength *= (1.0 + observer_influence).min(10.0);

        let mut probability = archetype_resonance;
        probability *= temporal_proximity;
        probability *= (1.0 + observer_influence).min(2.0);
        probability = probability.clamp(0.0, 1.0);

        let direction = if distance > 0.0 {
            let inv_distance = 1.0 / distance;
            [dx * inv_distance, dy * inv_distance, dz * inv_distance]
        } else {
            [0.0, 0.0, 0.0]
        };

        let interaction_type = if archetype_resonance > 0.7 {
            InteractionType::Resonance
        } else {
            InteractionType::Metaphysical
        };

        Ok(Interaction {
            interaction_type,
            strength,
            direction,
            probability,
            observer_influence,
            spectrum_mode: HolographicPhysicsMode::TimeSpace,
        })
    }

    fn compute_quantum_interaction(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt().max(1e-15);

        let interference = self.create_interference_pattern(entity_a, entity_b);

        let phase_factor = (interference.phase * 2.0 * std::f64::consts::PI).cos();
        let amplitude_factor = interference.amplitude;
        let frequency_factor = 1.0 / (1.0 + interference.frequency.abs());

        let mut strength = amplitude_factor * phase_factor * frequency_factor;
        strength *= entity_a.energy * entity_b.energy;
        strength *= 1.0 / (1.0 + distance);

        let entanglement_strength =
            self.compute_entanglement_strength(entity_a, entity_b, distance);

        let observer_influence = self.compute_observer_influence(entity_a, entity_b);
        let collapse_factor = (1.0 + observer_influence).min(2.0);
        strength *= collapse_factor;

        let base_probability = amplitude_factor * 0.5 + 0.5;
        let mut probability = base_probability * phase_factor.abs() * collapse_factor;
        probability = probability.clamp(0.0, 1.0);

        let direction = if distance > 1e-15 {
            let inv_distance = 1.0 / distance;
            let spatial_x = interference.spatial_components[0] * inv_distance;
            let spatial_y = interference.spatial_components[1] * inv_distance;
            let spatial_z = interference.spatial_components[2] * inv_distance;
            [spatial_x, spatial_y, spatial_z]
        } else {
            [0.0, 0.0, 0.0]
        };

        let interaction_type = if entanglement_strength > 0.5 {
            InteractionType::QuantumEntanglement
        } else {
            InteractionType::Resonance
        };

        Ok(Interaction {
            interaction_type,
            strength,
            direction,
            probability,
            observer_influence,
            spectrum_mode: HolographicPhysicsMode::Quantum,
        })
    }

    fn create_interference_pattern(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> PhysicsInterferencePattern {
        let mut amplitude = 0.0;
        let mut phase = 0.0;
        let mut frequency = 0.0;
        let mut spatial_components = [0.0; 3];
        let mut temporal_components = [0.0; 3];

        for i in 0..22 {
            let sig_a = entity_a.holographic_signature[i];
            let sig_b = entity_b.holographic_signature[i];

            amplitude += (sig_a + sig_b).abs() / 2.0;
            phase += (sig_a - sig_b).atan2(sig_b - sig_a) / 22.0;
            frequency += (sig_a * sig_b).abs();

            if i < 3 {
                spatial_components[i] = (sig_a - sig_b) / 2.0;
            } else if i < 6 {
                temporal_components[i - 3] = (sig_a + sig_b) / 2.0;
            }
        }

        amplitude /= 22.0;
        frequency /= 22.0;

        PhysicsInterferencePattern {
            amplitude,
            phase,
            frequency,
            spatial_components,
            temporal_components,
        }
    }

    fn compute_electromagnetic_component(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        distance: Float,
    ) -> Float {
        let charge_a = entity_a.holographic_signature[0].abs();
        let charge_b = entity_b.holographic_signature[0].abs();

        (charge_a * charge_b) / (distance * distance)
    }

    fn compute_archetype_resonance(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Float {
        let mut resonance = 0.0;

        for i in 0..22 {
            let diff =
                (entity_a.holographic_signature[i] - entity_b.holographic_signature[i]).abs();
            resonance += (1.0 - diff).max(0.0);
        }

        resonance / 22.0
    }

    fn compute_temporal_proximity(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Float {
        let mut proximity = 0.0;

        for i in 0..3.min(entity_a.holographic_signature.len()) {
            proximity +=
                (entity_a.holographic_signature[i] * entity_b.holographic_signature[i]).abs();
        }

        proximity / 3.0
    }

    fn compute_entanglement_strength(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
        distance: Float,
    ) -> Float {
        let mut strength = 0.0;

        for i in 0..22 {
            let product = entity_a.holographic_signature[i] * entity_b.holographic_signature[i];
            strength += product.abs();
        }

        strength /= 22.0;

        let distance_factor = 1.0 / (1.0 + distance * distance);
        strength * distance_factor
    }

    fn compute_observer_influence(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Float {
        let a_observed = !entity_a.observed_by.is_empty();
        let b_observed = !entity_b.observed_by.is_empty();

        let mut influence: Float = 0.0;

        if a_observed {
            influence += 0.5;
        }
        if b_observed {
            influence += 0.5;
        }

        influence.min(1.0)
    }
}

impl Default for HolographicPhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_ratio_creation() {
        let ratio = SpectrumRatio::new(2.0, 1.0);
        assert_eq!(ratio.space_time_ratio, 2.0);
        assert_eq!(ratio.time_space_ratio, 1.0);
        assert!((ratio.spectrum_position - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_spectrum_ratio_from_position() {
        let ratio = SpectrumRatio::compute_from_position(0.3);
        assert!(ratio.space_time_ratio < ratio.time_space_ratio);
    }

    #[test]
    fn test_spectrum_ratio_space_time_dominant() {
        let ratio = SpectrumRatio::new(2.0, 1.0);
        assert!(ratio.is_space_time_dominant());
        assert!(!ratio.is_time_space_dominant());
    }

    #[test]
    fn test_spectrum_ratio_time_space_dominant() {
        let ratio = SpectrumRatio::new(1.0, 2.0);
        assert!(!ratio.is_space_time_dominant());
        assert!(ratio.is_time_space_dominant());
    }

    #[test]
    fn test_spectrum_ratio_quantum() {
        let ratio = SpectrumRatio::new(1.0, 1.0);
        assert!(ratio.is_quantum());
    }

    #[test]
    fn test_physics_engine_creation() {
        let engine = HolographicPhysicsEngine::new();
        assert_eq!(engine.get_current_mode(), HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_physics_engine_with_thresholds() {
        let thresholds = SpectrumThresholds {
            space_time_threshold: 1.5,
            time_space_threshold: 1.5,
            quantum_tolerance: 0.2,
        };
        let engine = HolographicPhysicsEngine::with_thresholds(thresholds);
        assert_eq!(engine.get_current_mode(), HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_determine_physics_mode_space_time() {
        let engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(2.0, 1.0);
        let ratio_b = SpectrumRatio::new(2.0, 1.0);
        let mode = engine.determine_physics_mode(&ratio_a, &ratio_b);
        assert_eq!(mode, HolographicPhysicsMode::SpaceTime);
    }

    #[test]
    fn test_determine_physics_mode_time_space() {
        let engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 2.0);
        let ratio_b = SpectrumRatio::new(1.0, 2.0);
        let mode = engine.determine_physics_mode(&ratio_a, &ratio_b);
        assert_eq!(mode, HolographicPhysicsMode::TimeSpace);
    }

    #[test]
    fn test_determine_physics_mode_quantum() {
        let engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 1.0);
        let mode = engine.determine_physics_mode(&ratio_a, &ratio_b);
        assert_eq!(mode, HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_set_physics_mode() {
        let mut engine = HolographicPhysicsEngine::new();
        engine.set_physics_mode(HolographicPhysicsMode::SpaceTime);
        assert_eq!(engine.get_current_mode(), HolographicPhysicsMode::SpaceTime);
    }

    #[test]
    fn test_compute_space_time_interaction() {
        let mut engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(2.0, 1.0);
        let ratio_b = SpectrumRatio::new(2.0, 1.0);
        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);
        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_a.mass = 10.0;
        entity_b.mass = 20.0;

        let result = engine.compute_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());

        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::SpaceTime);
        assert!(interaction.strength > 0.0);
    }

    #[test]
    fn test_compute_time_space_interaction() {
        let mut engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 2.0);
        let ratio_b = SpectrumRatio::new(1.0, 2.0);
        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);
        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_a.energy = 10.0;
        entity_b.energy = 20.0;
        entity_a.holographic_signature[0] = 0.5;
        entity_b.holographic_signature[0] = 0.5;

        let result = engine.compute_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());

        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::TimeSpace);
    }

    #[test]
    fn test_compute_quantum_interaction() {
        let mut engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 1.0);
        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);
        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_a.energy = 10.0;
        entity_b.energy = 20.0;
        entity_a.holographic_signature[0] = 0.5;
        entity_b.holographic_signature[0] = 0.5;

        let result = engine.compute_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());

        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_self_interaction_error() {
        let mut engine = HolographicPhysicsEngine::new();
        let ratio = SpectrumRatio::new(1.0, 1.0);
        let entity = HolographicEntity::new(1, ratio);

        let result = engine.compute_interaction(&entity, &entity);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HolographicPhysicsError::InvalidEntity(_)
        ));
    }

    #[test]
    fn test_statistics_tracking() {
        let mut engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(2.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 2.0);
        let ratio_c = SpectrumRatio::new(1.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);
        let mut entity_c = HolographicEntity::new(3, ratio_c);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_c.position = [2.0, 0.0, 0.0];

        entity_a.mass = 10.0;
        entity_b.mass = 20.0;
        entity_c.mass = 30.0;

        let _ = engine.compute_interaction(&entity_a, &entity_b);
        let _ = engine.compute_interaction(&entity_b, &entity_c);
        let _ = engine.compute_interaction(&entity_a, &entity_c);

        let stats = engine.get_statistics();
        assert_eq!(stats.total_interactions_computed, 3);
        assert!(
            stats.space_time_interactions
                + stats.time_space_interactions
                + stats.quantum_interactions
                > 0
        );
    }

    #[test]
    fn test_reset_statistics() {
        let mut engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(2.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 2.0);

        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];

        let _ = engine.compute_interaction(&entity_a, &entity_b);
        engine.reset_statistics();

        let stats = engine.get_statistics();
        assert_eq!(stats.total_interactions_computed, 0);
    }

    #[test]
    fn test_interference_pattern_creation() {
        let engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 1.0);
        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);

        for i in 0..22 {
            entity_a.holographic_signature[i] = 0.5;
            entity_b.holographic_signature[i] = 0.5;
        }

        let pattern = engine.create_interference_pattern(&entity_a, &entity_b);
        assert!(pattern.amplitude >= 0.0);
    }

    #[test]
    fn test_archetype_resonance_computation() {
        let engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 1.0);
        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);

        for i in 0..22 {
            entity_a.holographic_signature[i] = 0.5;
            entity_b.holographic_signature[i] = 0.5;
        }

        let resonance = engine.compute_archetype_resonance(&entity_a, &entity_b);
        assert!((resonance - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_observer_influence() {
        let engine = HolographicPhysicsEngine::new();
        let ratio_a = SpectrumRatio::new(1.0, 1.0);
        let ratio_b = SpectrumRatio::new(1.0, 1.0);
        let mut entity_a = HolographicEntity::new(1, ratio_a);
        let mut entity_b = HolographicEntity::new(2, ratio_b);

        entity_a.observed_by.push(100);
        entity_b.observed_by.push(200);

        let influence = engine.compute_observer_influence(&entity_a, &entity_b);
        assert!(influence > 0.0);
    }

    #[test]
    fn test_interaction_default() {
        let interaction = Interaction::default();
        assert_eq!(interaction.interaction_type, InteractionType::Void);
        assert_eq!(interaction.strength, 0.0);
    }

    #[test]
    fn test_spectrum_thresholds_default() {
        let thresholds = SpectrumThresholds::default();
        assert_eq!(thresholds.space_time_threshold, SPACE_TIME_THRESHOLD);
        assert_eq!(thresholds.time_space_threshold, TIME_SPACE_THRESHOLD);
        assert_eq!(thresholds.quantum_tolerance, QUANTUM_TOLERANCE);
    }
}
