//! Scale-Specific Physics
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 8: "Implement quantum physics (probability-based),
//! classical physics (holographic simplification), metaphysical physics (time/space),
//! physics mode transitions."
//!
//! Each scale has different physical laws:
//! - Quantum Scale (v ≈ 1): Probability-based, wave functions, superposition, entanglement
//! - Classical Scale (v = s/t): Space/Time dominant, gravitational/electromagnetic forces
//! - Metaphysical Scale (v = t/s): Time/Space dominant, archetype resonance, temporal forces
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.5: "The spectrum is a continuous range with
//! different ratios at different points. The entire spectrum exists simultaneously."

use super::holographic_physics::{
    HolographicEntity, HolographicPhysicsError, HolographicPhysicsMode, Interaction,
    InteractionType, SpectrumRatio,
};
use crate::types::Float;

pub const QUANTUM_SCALE_THRESHOLD: Float = 1e-15;
pub const CELLULAR_SCALE_THRESHOLD: Float = 1e-6;
pub const BIOLOGICAL_SCALE_THRESHOLD: Float = 1.0;
pub const PLANETARY_SCALE_THRESHOLD: Float = 1e7;
pub const STELLAR_SCALE_THRESHOLD: Float = 1e13;
pub const GALACTIC_SCALE_THRESHOLD: Float = 1e21;

pub const MODE_TRANSITION_SMOOTHING: Float = 0.1;
pub const PROBABILITY_COLLAPSE_THRESHOLD: Float = 0.95;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaleLevel {
    Quantum,    // 10^-35 to 10^-15 m
    Cellular,   // 10^-12 to 10^-6 m
    Biological, // 10^-5 to 10^0 m
    Planetary,  // 10^0 to 10^7 m
    Stellar,    // 10^8 to 10^13 m
    Galactic,   // 10^14 to 10^21 m
    Cosmic,     // 10^22 to 10^26 m
}

impl ScaleLevel {
    pub fn from_meters(size: Float) -> Self {
        if size < QUANTUM_SCALE_THRESHOLD {
            ScaleLevel::Quantum
        } else if size < CELLULAR_SCALE_THRESHOLD {
            ScaleLevel::Cellular
        } else if size < BIOLOGICAL_SCALE_THRESHOLD {
            ScaleLevel::Biological
        } else if size < PLANETARY_SCALE_THRESHOLD {
            ScaleLevel::Planetary
        } else if size < STELLAR_SCALE_THRESHOLD {
            ScaleLevel::Stellar
        } else if size < GALACTIC_SCALE_THRESHOLD {
            ScaleLevel::Galactic
        } else {
            ScaleLevel::Cosmic
        }
    }

    pub fn typical_physics_mode(&self) -> HolographicPhysicsMode {
        match self {
            ScaleLevel::Quantum => HolographicPhysicsMode::Quantum,
            ScaleLevel::Cellular => HolographicPhysicsMode::Quantum,
            ScaleLevel::Biological => HolographicPhysicsMode::SpaceTime,
            ScaleLevel::Planetary => HolographicPhysicsMode::SpaceTime,
            ScaleLevel::Stellar => HolographicPhysicsMode::SpaceTime,
            ScaleLevel::Galactic => HolographicPhysicsMode::TimeSpace,
            ScaleLevel::Cosmic => HolographicPhysicsMode::TimeSpace,
        }
    }

    pub fn scale_factor(&self) -> Float {
        match self {
            ScaleLevel::Quantum => 1e-35,
            ScaleLevel::Cellular => 1e-12,
            ScaleLevel::Biological => 1e-5,
            ScaleLevel::Planetary => 1e3,
            ScaleLevel::Stellar => 1e10,
            ScaleLevel::Galactic => 1e17,
            ScaleLevel::Cosmic => 1e24,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumPhysicsMode {
    pub superposition_enabled: bool,
    pub entanglement_threshold: Float,
    pub wave_function_collapse_rate: Float,
    pub uncertainty_principle_factor: Float,
    pub quantum_tunneling_probability: Float,
}

impl Default for QuantumPhysicsMode {
    fn default() -> Self {
        QuantumPhysicsMode {
            superposition_enabled: true,
            entanglement_threshold: 0.5,
            wave_function_collapse_rate: 0.5,
            uncertainty_principle_factor: 1.0,
            quantum_tunneling_probability: 0.1,
        }
    }
}

impl QuantumPhysicsMode {
    pub fn new(
        superposition_enabled: bool,
        entanglement_threshold: Float,
        wave_function_collapse_rate: Float,
    ) -> Self {
        QuantumPhysicsMode {
            superposition_enabled,
            entanglement_threshold,
            wave_function_collapse_rate,
            uncertainty_principle_factor: 1.0,
            quantum_tunneling_probability: 0.1,
        }
    }

    pub fn compute_wave_function(&self, entity: &HolographicEntity) -> WaveFunction {
        let mut amplitudes = Vec::with_capacity(22);
        let mut phases = Vec::with_capacity(22);

        for i in 0..22 {
            let sig = entity.holographic_signature[i];
            let amplitude = sig.abs();
            let phase = sig.atan2(1.0);

            amplitudes.push(amplitude);
            phases.push(phase);
        }

        let normalization = amplitudes.iter().map(|a| a * a).sum::<Float>().sqrt();
        let normalized_amplitudes: Vec<Float> =
            amplitudes.iter().map(|a| a / normalization).collect();

        WaveFunction {
            amplitudes: normalized_amplitudes.clone(),
            phases,
            superposition_coefficients: if self.superposition_enabled {
                normalized_amplitudes
            } else {
                vec![1.0_f64; 22]
            },
            collapsed: false,
            collapsed_state: None,
        }
    }

    pub fn collapse_wave_function(
        &self,
        wave_function: &mut WaveFunction,
        observer_strength: Float,
    ) -> usize {
        if wave_function.collapsed {
            return wave_function.collapsed_state.unwrap_or(0);
        }

        let collapse_prob = self.wave_function_collapse_rate * observer_strength;
        if collapse_prob < PROBABILITY_COLLAPSE_THRESHOLD {
            return 0;
        }

        let total_prob: Float = wave_function.amplitudes.iter().map(|a| a * a).sum();
        if total_prob < 1e-10 {
            wave_function.collapsed = true;
            wave_function.collapsed_state = Some(0);
            return 0;
        }

        let mut max_prob = 0.0;
        let mut max_index = 0;

        for (i, amplitude) in wave_function.amplitudes.iter().enumerate() {
            let prob = amplitude * amplitude;
            if prob > max_prob {
                max_prob = prob;
                max_index = i;
            }
        }

        wave_function.collapsed = true;
        wave_function.collapsed_state = Some(max_index);
        max_index
    }

    pub fn compute_uncertainty(&self, entity: &HolographicEntity) -> (Float, Float) {
        let position_uncertainty = self.uncertainty_principle_factor / entity.mass;
        let momentum_uncertainty = self.uncertainty_principle_factor * entity.mass;
        (position_uncertainty, momentum_uncertainty)
    }

    pub fn check_entanglement(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> bool {
        let mut correlation = 0.0;

        for i in 0..22 {
            correlation +=
                (entity_a.holographic_signature[i] - entity_b.holographic_signature[i]).abs();
        }

        correlation / 22.0 < (1.0 - self.entanglement_threshold)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaveFunction {
    pub amplitudes: Vec<Float>,
    pub phases: Vec<Float>,
    pub superposition_coefficients: Vec<Float>,
    pub collapsed: bool,
    pub collapsed_state: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassicalPhysicsMode {
    pub gravitational_constant: Float,
    pub electromagnetic_constant: Float,
    pub friction_coefficient: Float,
    pub elasticity: Float,
    pub momentum_conservation: bool,
    pub energy_conservation: bool,
}

impl Default for ClassicalPhysicsMode {
    fn default() -> Self {
        ClassicalPhysicsMode {
            gravitational_constant: 6.674e-11,
            electromagnetic_constant: 8.987e9,
            friction_coefficient: 0.01,
            elasticity: 0.8,
            momentum_conservation: true,
            energy_conservation: true,
        }
    }
}

impl ClassicalPhysicsMode {
    pub fn new(gravitational_constant: Float, electromagnetic_constant: Float) -> Self {
        ClassicalPhysicsMode {
            gravitational_constant,
            electromagnetic_constant,
            friction_coefficient: 0.01,
            elasticity: 0.8,
            momentum_conservation: true,
            energy_conservation: true,
        }
    }

    pub fn compute_gravitational_force(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> [Float; 3] {
        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt().max(1e-10);

        let force_magnitude =
            self.gravitational_constant * entity_a.mass * entity_b.mass / distance_sq;

        let inv_distance = 1.0 / distance;
        [
            force_magnitude * dx * inv_distance,
            force_magnitude * dy * inv_distance,
            force_magnitude * dz * inv_distance,
        ]
    }

    pub fn compute_electromagnetic_force(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> [Float; 3] {
        let charge_a = entity_a.holographic_signature[0].abs();
        let charge_b = entity_b.holographic_signature[0].abs();

        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt().max(1e-10);

        let force_magnitude = self.electromagnetic_constant * charge_a * charge_b / distance_sq;

        let inv_distance = 1.0 / distance;
        [
            force_magnitude * dx * inv_distance,
            force_magnitude * dy * inv_distance,
            force_magnitude * dz * inv_distance,
        ]
    }

    pub fn apply_friction(&self, velocity: [Float; 3]) -> [Float; 3] {
        [
            velocity[0] * (1.0 - self.friction_coefficient),
            velocity[1] * (1.0 - self.friction_coefficient),
            velocity[2] * (1.0 - self.friction_coefficient),
        ]
    }

    pub fn compute_elastic_collision(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> ([Float; 3], [Float; 3]) {
        if !self.momentum_conservation {
            return (entity_a.velocity, entity_b.velocity);
        }

        let m1 = entity_a.mass;
        let m2 = entity_b.mass;
        let v1 = entity_a.velocity;
        let v2 = entity_b.velocity;

        let v1_new = [
            ((m1 - m2) * v1[0] + 2.0 * m2 * v2[0]) / (m1 + m2) * self.elasticity,
            ((m1 - m2) * v1[1] + 2.0 * m2 * v2[1]) / (m1 + m2) * self.elasticity,
            ((m1 - m2) * v1[2] + 2.0 * m2 * v2[2]) / (m1 + m2) * self.elasticity,
        ];

        let v2_new = [
            ((m2 - m1) * v2[0] + 2.0 * m1 * v1[0]) / (m1 + m2) * self.elasticity,
            ((m2 - m1) * v2[1] + 2.0 * m1 * v1[1]) / (m1 + m2) * self.elasticity,
            ((m2 - m1) * v2[2] + 2.0 * m1 * v1[2]) / (m1 + m2) * self.elasticity,
        ];

        (v1_new, v2_new)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaphysicalPhysicsMode {
    pub archetype_resonance_weight: Float,
    pub temporal_coupling_factor: Float,
    pub consciousness_influence: Float,
    pub telepathic_range: Float,
    pub unity_threshold: Float,
}

impl Default for MetaphysicalPhysicsMode {
    fn default() -> Self {
        MetaphysicalPhysicsMode {
            archetype_resonance_weight: 1.0,
            temporal_coupling_factor: 0.5,
            consciousness_influence: 0.7,
            telepathic_range: 1e6,
            unity_threshold: 0.9,
        }
    }
}

impl MetaphysicalPhysicsMode {
    pub fn new(archetype_resonance_weight: Float, temporal_coupling_factor: Float) -> Self {
        MetaphysicalPhysicsMode {
            archetype_resonance_weight,
            temporal_coupling_factor,
            consciousness_influence: 0.7,
            telepathic_range: 1e6,
            unity_threshold: 0.9,
        }
    }

    pub fn compute_archetype_resonance(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Float {
        let mut resonance = 0.0;

        for i in 0..22 {
            let sig_a = entity_a.holographic_signature[i];
            let sig_b = entity_b.holographic_signature[i];
            let similarity = 1.0 - (sig_a - sig_b).abs().min(1.0);
            resonance += similarity * self.archetype_resonance_weight;
        }

        resonance / 22.0
    }

    pub fn compute_temporal_force(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> [Float; 3] {
        let archetype_resonance = self.compute_archetype_resonance(entity_a, entity_b);
        let temporal_coupling = archetype_resonance * self.temporal_coupling_factor;

        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt().max(1.0);

        let force_magnitude = temporal_coupling * entity_a.energy * entity_b.energy / distance;

        let inv_distance = 1.0 / distance;
        [
            force_magnitude * dx * inv_distance,
            force_magnitude * dy * inv_distance,
            force_magnitude * dz * inv_distance,
        ]
    }

    pub fn check_telepathic_connection(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> bool {
        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt();

        distance < self.telepathic_range
    }

    pub fn compute_unity_field(&self, entity: &HolographicEntity) -> Float {
        let coherence = entity
            .holographic_signature
            .iter()
            .map(|&s| s.abs())
            .sum::<Float>()
            / 22.0;
        coherence * self.consciousness_influence
    }
}

#[derive(Debug, Clone)]
pub struct ScaleSpecificPhysics {
    pub quantum_mode: QuantumPhysicsMode,
    pub classical_mode: ClassicalPhysicsMode,
    pub metaphysical_mode: MetaphysicalPhysicsMode,
    pub current_mode: HolographicPhysicsMode,
    pub current_scale: ScaleLevel,
    pub mode_transition_progress: Float,
}

impl ScaleSpecificPhysics {
    pub fn new() -> Self {
        ScaleSpecificPhysics {
            quantum_mode: QuantumPhysicsMode::default(),
            classical_mode: ClassicalPhysicsMode::default(),
            metaphysical_mode: MetaphysicalPhysicsMode::default(),
            current_mode: HolographicPhysicsMode::Quantum,
            current_scale: ScaleLevel::Quantum,
            mode_transition_progress: 1.0,
        }
    }

    pub fn with_modes(
        quantum_mode: QuantumPhysicsMode,
        classical_mode: ClassicalPhysicsMode,
        metaphysical_mode: MetaphysicalPhysicsMode,
    ) -> Self {
        ScaleSpecificPhysics {
            quantum_mode,
            classical_mode,
            metaphysical_mode,
            current_mode: HolographicPhysicsMode::Quantum,
            current_scale: ScaleLevel::Quantum,
            mode_transition_progress: 1.0,
        }
    }

    pub fn set_scale(&mut self, scale: ScaleLevel) {
        self.current_scale = scale;
        let target_mode = scale.typical_physics_mode();
        self.transition_physics_mode(target_mode);
    }

    pub fn transition_physics_mode(&mut self, new_mode: HolographicPhysicsMode) {
        if self.current_mode == new_mode {
            self.mode_transition_progress = 1.0;
            return;
        }

        self.mode_transition_progress = 0.0;
        self.current_mode = new_mode;
    }

    pub fn update_transition(&mut self, delta_time: Float) {
        if self.mode_transition_progress < 1.0 {
            self.mode_transition_progress += delta_time * MODE_TRANSITION_SMOOTHING;
            self.mode_transition_progress = self.mode_transition_progress.min(1.0);
        }
    }

    pub fn compute_quantum_interaction(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let dx = entity_b.position[0] - entity_a.position[0];
        let dy = entity_b.position[1] - entity_a.position[1];
        let dz = entity_b.position[2] - entity_a.position[2];

        let distance_sq = dx * dx + dy * dy + dz * dz;
        let distance = distance_sq.sqrt().max(1e-35);

        let wave_function_a = self.quantum_mode.compute_wave_function(entity_a);
        let wave_function_b = self.quantum_mode.compute_wave_function(entity_b);

        let mut amplitude_product = 0.0;
        for (i, (amp_a, amp_b)) in wave_function_a
            .amplitudes
            .iter()
            .zip(wave_function_b.amplitudes.iter())
            .enumerate()
        {
            amplitude_product +=
                amp_a * amp_b * (wave_function_a.phases[i] - wave_function_b.phases[i]).cos();
        }

        let entangled = self.quantum_mode.check_entanglement(entity_a, entity_b);
        let entanglement_factor = if entangled { 2.0 } else { 1.0 };

        let mut strength = amplitude_product * entanglement_factor / (1.0 + distance);
        strength *= entity_a.energy * entity_b.energy;

        let uncertainty_a = self.quantum_mode.compute_uncertainty(entity_a);
        let uncertainty_b = self.quantum_mode.compute_uncertainty(entity_b);
        let uncertainty_factor = (uncertainty_a.0 + uncertainty_b.0) / 2.0;
        strength *= (1.0 + uncertainty_factor).min(2.0);

        let observer_influence =
            if !entity_a.observed_by.is_empty() || !entity_b.observed_by.is_empty() {
                0.5
            } else {
                0.0
            };

        let base_probability = amplitude_product.abs().min(1.0);
        let probability = (base_probability * (1.0 + observer_influence)).min(2.0);

        let direction = if distance > 1e-35 {
            let inv_distance = 1.0 / distance;
            [dx * inv_distance, dy * inv_distance, dz * inv_distance]
        } else {
            [0.0, 0.0, 0.0]
        };

        let interaction_type = if entangled {
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

    pub fn compute_classical_interaction(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let gravitational_force = self
            .classical_mode
            .compute_gravitational_force(entity_a, entity_b);
        let electromagnetic_force = self
            .classical_mode
            .compute_electromagnetic_force(entity_a, entity_b);

        let gravity_magnitude = (gravitational_force[0].powi(2)
            + gravitational_force[1].powi(2)
            + gravitational_force[2].powi(2))
        .sqrt();
        let em_magnitude = (electromagnetic_force[0].powi(2)
            + electromagnetic_force[1].powi(2)
            + electromagnetic_force[2].powi(2))
        .sqrt();

        let combined_force = [
            gravitational_force[0] + electromagnetic_force[0],
            gravitational_force[1] + electromagnetic_force[1],
            gravitational_force[2] + electromagnetic_force[2],
        ];

        let strength =
            (combined_force[0].powi(2) + combined_force[1].powi(2) + combined_force[2].powi(2))
                .sqrt();

        let probability = 1.0;
        let observer_influence = 0.0;

        let direction = [
            combined_force[0] / strength.max(1e-10),
            combined_force[1] / strength.max(1e-10),
            combined_force[2] / strength.max(1e-10),
        ];

        let interaction_type = if gravity_magnitude > em_magnitude {
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

    pub fn compute_metaphysical_interaction(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let archetype_resonance = self
            .metaphysical_mode
            .compute_archetype_resonance(entity_a, entity_b);
        let temporal_force = self
            .metaphysical_mode
            .compute_temporal_force(entity_a, entity_b);

        let telepathic = self
            .metaphysical_mode
            .check_telepathic_connection(entity_a, entity_b);
        let telepathic_factor = if telepathic { 2.0 } else { 1.0 };

        let mut strength = archetype_resonance * temporal_force[0].abs() * telepathic_factor;
        strength *= entity_a.energy * entity_b.energy;

        let unity_field_a = self.metaphysical_mode.compute_unity_field(entity_a);
        let unity_field_b = self.metaphysical_mode.compute_unity_field(entity_b);
        let unity_factor = (unity_field_a + unity_field_b) / 2.0;
        strength *= (1.0 + unity_factor).min(3.0);

        let mut probability = archetype_resonance * (1.0 + unity_factor).min(2.0);
        probability = probability.min(1.0);

        let observer_influence = 1.0;

        let direction = [
            temporal_force[0] / strength.max(1e-10),
            temporal_force[1] / strength.max(1e-10),
            temporal_force[2] / strength.max(1e-10),
        ];

        let interaction_type = if archetype_resonance > self.metaphysical_mode.unity_threshold {
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

    pub fn compute_scale_aware_interaction(
        &mut self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> Result<Interaction, HolographicPhysicsError> {
        let spectrum_mode = self.determine_physics_mode(entity_a, entity_b);

        match spectrum_mode {
            HolographicPhysicsMode::Quantum => self.compute_quantum_interaction(entity_a, entity_b),
            HolographicPhysicsMode::SpaceTime => {
                self.compute_classical_interaction(entity_a, entity_b)
            }
            HolographicPhysicsMode::TimeSpace => {
                self.compute_metaphysical_interaction(entity_a, entity_b)
            }
        }
    }

    pub fn determine_physics_mode(
        &self,
        entity_a: &HolographicEntity,
        entity_b: &HolographicEntity,
    ) -> HolographicPhysicsMode {
        let ratio_a = entity_a.spectrum_ratio();
        let ratio_b = entity_b.spectrum_ratio();

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
            self.current_scale.typical_physics_mode()
        }
    }
}

impl Default for ScaleSpecificPhysics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_level_from_meters_quantum() {
        let scale = ScaleLevel::from_meters(1e-20);
        assert_eq!(scale, ScaleLevel::Quantum);
    }

    #[test]
    fn test_scale_level_from_meters_cellular() {
        let scale = ScaleLevel::from_meters(1e-9);
        assert_eq!(scale, ScaleLevel::Cellular);
    }

    #[test]
    fn test_scale_level_from_meters_biological() {
        let scale = ScaleLevel::from_meters(1e-2);
        assert_eq!(scale, ScaleLevel::Biological);
    }

    #[test]
    fn test_scale_level_from_meters_planetary() {
        let scale = ScaleLevel::from_meters(1e5);
        assert_eq!(scale, ScaleLevel::Planetary);
    }

    #[test]
    fn test_scale_level_from_meters_stellar() {
        let scale = ScaleLevel::from_meters(1e10);
        assert_eq!(scale, ScaleLevel::Stellar);
    }

    #[test]
    fn test_scale_level_from_meters_galactic() {
        let scale = ScaleLevel::from_meters(1e16);
        assert_eq!(scale, ScaleLevel::Galactic);
    }

    #[test]
    fn test_scale_level_from_meters_cosmic() {
        let scale = ScaleLevel::from_meters(1e23);
        assert_eq!(scale, ScaleLevel::Cosmic);
    }

    #[test]
    fn test_scale_physics_mode_quantum() {
        assert_eq!(
            ScaleLevel::Quantum.typical_physics_mode(),
            HolographicPhysicsMode::Quantum
        );
        assert_eq!(
            ScaleLevel::Cellular.typical_physics_mode(),
            HolographicPhysicsMode::Quantum
        );
    }

    #[test]
    fn test_scale_physics_mode_classical() {
        assert_eq!(
            ScaleLevel::Biological.typical_physics_mode(),
            HolographicPhysicsMode::SpaceTime
        );
        assert_eq!(
            ScaleLevel::Planetary.typical_physics_mode(),
            HolographicPhysicsMode::SpaceTime
        );
        assert_eq!(
            ScaleLevel::Stellar.typical_physics_mode(),
            HolographicPhysicsMode::SpaceTime
        );
    }

    #[test]
    fn test_scale_physics_mode_metaphysical() {
        assert_eq!(
            ScaleLevel::Galactic.typical_physics_mode(),
            HolographicPhysicsMode::TimeSpace
        );
        assert_eq!(
            ScaleLevel::Cosmic.typical_physics_mode(),
            HolographicPhysicsMode::TimeSpace
        );
    }

    #[test]
    fn test_scale_factor() {
        assert_eq!(ScaleLevel::Quantum.scale_factor(), 1e-35);
        assert_eq!(ScaleLevel::Cellular.scale_factor(), 1e-12);
        assert_eq!(ScaleLevel::Biological.scale_factor(), 1e-5);
        assert_eq!(ScaleLevel::Planetary.scale_factor(), 1e3);
        assert_eq!(ScaleLevel::Stellar.scale_factor(), 1e10);
        assert_eq!(ScaleLevel::Galactic.scale_factor(), 1e17);
        assert_eq!(ScaleLevel::Cosmic.scale_factor(), 1e24);
    }

    #[test]
    fn test_quantum_physics_mode_creation() {
        let quantum = QuantumPhysicsMode::default();
        assert!(quantum.superposition_enabled);
        assert_eq!(quantum.entanglement_threshold, 0.5);
        assert_eq!(quantum.wave_function_collapse_rate, 0.5);
    }

    #[test]
    fn test_quantum_physics_mode_new() {
        let quantum = QuantumPhysicsMode::new(true, 0.7, 0.8);
        assert!(quantum.superposition_enabled);
        assert_eq!(quantum.entanglement_threshold, 0.7);
        assert_eq!(quantum.wave_function_collapse_rate, 0.8);
    }

    #[test]
    fn test_wave_function_computation() {
        let quantum = QuantumPhysicsMode::default();
        let ratio = SpectrumRatio::new(1.0, 1.0);
        let mut entity = HolographicEntity::new(1, ratio);
        entity.holographic_signature[0] = 0.5;
        entity.holographic_signature[1] = 0.3;

        let wave_function = quantum.compute_wave_function(&entity);
        assert_eq!(wave_function.amplitudes.len(), 22);
        assert_eq!(wave_function.phases.len(), 22);
        assert!(!wave_function.collapsed);
    }

    #[test]
    fn test_wave_function_collapse() {
        let quantum = QuantumPhysicsMode::new(true, 0.5, 1.0);
        let ratio = SpectrumRatio::new(1.0, 1.0);
        let mut entity = HolographicEntity::new(1, ratio);
        entity.holographic_signature[0] = 0.8;
        entity.holographic_signature[1] = 0.2;

        let mut wave_function = quantum.compute_wave_function(&entity);
        let collapsed_state = quantum.collapse_wave_function(&mut wave_function, 1.0);

        assert!(wave_function.collapsed);
        assert!(wave_function.collapsed_state.is_some());
        assert_eq!(collapsed_state, wave_function.collapsed_state.unwrap());
    }

    #[test]
    fn test_entanglement_check() {
        let quantum = QuantumPhysicsMode::default();
        let ratio = SpectrumRatio::new(1.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        for i in 0..22 {
            entity_a.holographic_signature[i] = 0.5;
            entity_b.holographic_signature[i] = 0.5;
        }

        assert!(quantum.check_entanglement(&entity_a, &entity_b));
    }

    #[test]
    fn test_uncertainty_computation() {
        let quantum = QuantumPhysicsMode::default();
        let ratio = SpectrumRatio::new(1.0, 1.0);
        let mut entity = HolographicEntity::new(1, ratio);
        entity.mass = 10.0;

        let (position_uncertainty, momentum_uncertainty) = quantum.compute_uncertainty(&entity);
        assert!(position_uncertainty > 0.0);
        assert!(momentum_uncertainty > 0.0);
    }

    #[test]
    fn test_classical_physics_mode_creation() {
        let classical = ClassicalPhysicsMode::default();
        assert!(classical.momentum_conservation);
        assert!(classical.energy_conservation);
        assert_eq!(classical.gravitational_constant, 6.674e-11);
    }

    #[test]
    fn test_gravitational_force_computation() {
        let classical = ClassicalPhysicsMode::default();
        let ratio = SpectrumRatio::new(2.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_a.mass = 10.0;
        entity_b.mass = 20.0;

        let force = classical.compute_gravitational_force(&entity_a, &entity_b);
        assert!(force[0] > 0.0);
        assert_eq!(force[1], 0.0);
        assert_eq!(force[2], 0.0);
    }

    #[test]
    fn test_electromagnetic_force_computation() {
        let classical = ClassicalPhysicsMode::default();
        let ratio = SpectrumRatio::new(2.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_a.holographic_signature[0] = 1.0;
        entity_b.holographic_signature[0] = 1.0;

        let force = classical.compute_electromagnetic_force(&entity_a, &entity_b);
        assert!(force[0] > 0.0);
        assert_eq!(force[1], 0.0);
        assert_eq!(force[2], 0.0);
    }

    #[test]
    fn test_friction_application() {
        let classical = ClassicalPhysicsMode::default();
        let velocity = [10.0, 5.0, 0.0];

        let new_velocity = classical.apply_friction(velocity);
        assert!(new_velocity[0] < velocity[0]);
        assert!(new_velocity[1] < velocity[1]);
    }

    #[test]
    fn test_elastic_collision() {
        let classical = ClassicalPhysicsMode::default();
        let ratio = SpectrumRatio::new(2.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.velocity = [5.0, 0.0, 0.0];
        entity_b.velocity = [-3.0, 0.0, 0.0];
        entity_a.mass = 10.0;
        entity_b.mass = 10.0;

        let (v1_new, v2_new) = classical.compute_elastic_collision(&entity_a, &entity_b);
        assert_eq!(v1_new[0], -3.0 * classical.elasticity);
        assert_eq!(v2_new[0], 5.0 * classical.elasticity);
    }

    #[test]
    fn test_metaphysical_physics_mode_creation() {
        let metaphysical = MetaphysicalPhysicsMode::default();
        assert_eq!(metaphysical.archetype_resonance_weight, 1.0);
        assert_eq!(metaphysical.temporal_coupling_factor, 0.5);
        assert_eq!(metaphysical.consciousness_influence, 0.7);
    }

    #[test]
    fn test_archetype_resonance_computation() {
        let metaphysical = MetaphysicalPhysicsMode::default();
        let ratio = SpectrumRatio::new(1.0, 2.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        for i in 0..22 {
            entity_a.holographic_signature[i] = 0.5;
            entity_b.holographic_signature[i] = 0.5;
        }

        let resonance = metaphysical.compute_archetype_resonance(&entity_a, &entity_b);
        assert_eq!(resonance, 1.0);
    }

    #[test]
    fn test_temporal_force_computation() {
        let metaphysical = MetaphysicalPhysicsMode::default();
        let ratio = SpectrumRatio::new(1.0, 2.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1.0, 0.0, 0.0];
        entity_a.energy = 10.0;
        entity_b.energy = 20.0;
        entity_a.holographic_signature[0] = 0.5;
        entity_b.holographic_signature[0] = 0.5;

        let force = metaphysical.compute_temporal_force(&entity_a, &entity_b);
        assert!(force[0] > 0.0);
    }

    #[test]
    fn test_telepathic_connection_check() {
        let metaphysical = MetaphysicalPhysicsMode::default();
        let ratio = SpectrumRatio::new(1.0, 2.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [100.0, 0.0, 0.0];

        assert!(metaphysical.check_telepathic_connection(&entity_a, &entity_b));
    }

    #[test]
    fn test_scale_specific_physics_creation() {
        let physics = ScaleSpecificPhysics::new();
        assert_eq!(physics.current_mode, HolographicPhysicsMode::Quantum);
        assert_eq!(physics.current_scale, ScaleLevel::Quantum);
        assert_eq!(physics.mode_transition_progress, 1.0);
    }

    #[test]
    fn test_scale_transition() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.set_scale(ScaleLevel::Planetary);
        assert_eq!(physics.current_scale, ScaleLevel::Planetary);
        assert_eq!(physics.current_mode, HolographicPhysicsMode::SpaceTime);
    }

    #[test]
    fn test_physics_mode_transition() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.transition_physics_mode(HolographicPhysicsMode::SpaceTime);
        assert_eq!(physics.current_mode, HolographicPhysicsMode::SpaceTime);
        assert_eq!(physics.mode_transition_progress, 0.0);
    }

    #[test]
    fn test_transition_update() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.transition_physics_mode(HolographicPhysicsMode::SpaceTime);
        assert_eq!(physics.mode_transition_progress, 0.0);

        physics.update_transition(10.0);
        assert!(physics.mode_transition_progress > 0.0);
    }

    #[test]
    fn test_compute_quantum_interaction() {
        let physics = ScaleSpecificPhysics::new();
        let ratio = SpectrumRatio::new(1.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1e-20, 0.0, 0.0];
        entity_a.energy = 10.0;
        entity_b.energy = 20.0;
        entity_a.holographic_signature[0] = 0.5;
        entity_b.holographic_signature[0] = 0.5;

        let result = physics.compute_quantum_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());
        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_compute_classical_interaction() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.set_scale(ScaleLevel::Planetary);

        let ratio = SpectrumRatio::new(2.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1000.0, 0.0, 0.0];
        entity_a.mass = 10.0;
        entity_b.mass = 20.0;

        let result = physics.compute_classical_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());
        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::SpaceTime);
    }

    #[test]
    fn test_compute_metaphysical_interaction() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.set_scale(ScaleLevel::Galactic);

        let ratio = SpectrumRatio::new(1.0, 2.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1000.0, 0.0, 0.0];
        entity_a.energy = 10.0;
        entity_b.energy = 20.0;
        entity_a.holographic_signature[0] = 0.5;
        entity_b.holographic_signature[0] = 0.5;

        let result = physics.compute_metaphysical_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());
        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::TimeSpace);
    }

    #[test]
    fn test_compute_scale_aware_interaction_quantum() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.set_scale(ScaleLevel::Quantum);

        let ratio = SpectrumRatio::new(1.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1e-20, 0.0, 0.0];
        entity_a.energy = 10.0;
        entity_b.energy = 20.0;

        let result = physics.compute_scale_aware_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());
        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::Quantum);
    }

    #[test]
    fn test_compute_scale_aware_interaction_classical() {
        let mut physics = ScaleSpecificPhysics::new();
        physics.set_scale(ScaleLevel::Planetary);

        let ratio = SpectrumRatio::new(2.0, 1.0);

        let mut entity_a = HolographicEntity::new(1, ratio);
        let mut entity_b = HolographicEntity::new(2, ratio);

        entity_a.position = [0.0, 0.0, 0.0];
        entity_b.position = [1000.0, 0.0, 0.0];
        entity_a.mass = 10.0;
        entity_b.mass = 20.0;

        let result = physics.compute_scale_aware_interaction(&entity_a, &entity_b);
        assert!(result.is_ok());
        let interaction = result.unwrap();
        assert_eq!(interaction.spectrum_mode, HolographicPhysicsMode::SpaceTime);
    }

    #[test]
    fn test_determine_physics_mode() {
        let physics = ScaleSpecificPhysics::new();

        let ratio_quantum = SpectrumRatio::new(1.0, 1.0);
        let ratio_space_time = SpectrumRatio::new(2.0, 1.0);
        let _ratio_time_space = SpectrumRatio::new(1.0, 2.0);

        let entity_a = HolographicEntity::new(1, ratio_quantum);
        let entity_b = HolographicEntity::new(2, ratio_space_time);

        let mode = physics.determine_physics_mode(&entity_a, &entity_b);
        assert!(
            mode == HolographicPhysicsMode::Quantum || mode == HolographicPhysicsMode::SpaceTime
        );
    }
}
