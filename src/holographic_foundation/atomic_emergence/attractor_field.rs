//! Attractor Field: Stable Configuration in Field Space
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 8:
//! "Attractor fields are stable quantum states with specific energy levels.
//!  The periodic table is the map of these stable attractor fields."
//!
//! An attractor field is a STABLE configuration of the holographic field.
//! When field coherence reaches certain thresholds, stable configurations
//! emerge spontaneously - these ARE atoms, not objects that create atoms.
//!
//! Key Insight: Atoms don't "exist" as separate entities. They are the
//! stable attractor states that the field naturally falls into when
//! it has sufficient coherence at atomic resolution.

use crate::types::Float;
use std::collections::HashMap;

use super::super::archetype_profile::NUM_ARCHETYPES;
use super::super::field_state::{HolographicFieldState, Position3D};
use super::super::quantum_consciousness::quantum_numbers::QuantumNumberSet;
use super::super::scale_level::ScaleLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttractorId(u64);

impl AttractorId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn from_quantum_numbers(qn: &QuantumNumberSet) -> Self {
        Self(qn.hash())
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttractorStability {
    Unstable,
    Metastable,
    Stable,
    NobleGas,
}

impl AttractorStability {
    pub fn stability_factor(&self) -> Float {
        match self {
            AttractorStability::Unstable => 0.1,
            AttractorStability::Metastable => 0.5,
            AttractorStability::Stable => 0.8,
            AttractorStability::NobleGas => 1.0,
        }
    }

    pub fn from_coherence(coherence: Float, is_noble: bool) -> Self {
        if is_noble && coherence > 0.9 {
            AttractorStability::NobleGas
        } else if coherence > 0.8 {
            AttractorStability::Stable
        } else if coherence > 0.5 {
            AttractorStability::Metastable
        } else {
            AttractorStability::Unstable
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldConfiguration {
    pub archetype_vector: [Float; NUM_ARCHETYPES],
    pub quantum_numbers: QuantumNumberSet,
    pub energy_level: Float,
    pub coherence: Float,
    pub phase: Float,
}

impl FieldConfiguration {
    pub fn new(archetype_vector: [Float; NUM_ARCHETYPES]) -> Self {
        use super::super::quantum_consciousness::quantum_numbers::ArchetypeToQuantumMapping;

        let quantum_numbers = ArchetypeToQuantumMapping::derive_quantum_numbers(&archetype_vector);
        let energy_level = quantum_numbers.energy_factor();

        let mean: Float = archetype_vector.iter().sum::<Float>() / NUM_ARCHETYPES as Float;
        let variance: Float = archetype_vector
            .iter()
            .map(|c| (c - mean).powi(2))
            .sum::<Float>()
            / NUM_ARCHETYPES as Float;
        let coherence = 1.0 - variance.sqrt().min(1.0);

        let phase = archetype_vector.iter().sum::<Float>() % (2.0 * std::f64::consts::PI);

        Self {
            archetype_vector,
            quantum_numbers,
            energy_level,
            coherence,
            phase,
        }
    }

    pub fn from_quantum_numbers(qn: QuantumNumberSet) -> Self {
        use super::super::quantum_consciousness::quantum_numbers::ArchetypeToQuantumMapping;

        let archetype_vector = ArchetypeToQuantumMapping::archetype_from_quantum_state(&qn);
        Self::new(archetype_vector)
    }

    pub fn ground_state() -> Self {
        Self::from_quantum_numbers(QuantumNumberSet::ground_state())
    }

    pub fn hydrogen_configuration() -> Self {
        let mut archetype = [0.5; NUM_ARCHETYPES];
        archetype[0] = 0.6;
        archetype[2] = 0.55;
        Self::new(archetype)
    }

    pub fn quantum_numbers(&self) -> &QuantumNumberSet {
        &self.quantum_numbers
    }

    pub fn distance_to(&self, other: &FieldConfiguration) -> Float {
        let mut distance = 0.0;
        for i in 0..NUM_ARCHETYPES {
            distance += (self.archetype_vector[i] - other.archetype_vector[i]).powi(2);
        }
        distance.sqrt()
    }

    pub fn archetype_similarity(&self, other: &FieldConfiguration) -> Float {
        let dot: Float = self
            .archetype_vector
            .iter()
            .zip(other.archetype_vector.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_self: Float = self
            .archetype_vector
            .iter()
            .map(|a| a * a)
            .sum::<Float>()
            .sqrt();
        let norm_other: Float = other
            .archetype_vector
            .iter()
            .map(|b| b * b)
            .sum::<Float>()
            .sqrt();

        if norm_self > 1e-10 && norm_other > 1e-10 {
            dot / (norm_self * norm_other)
        } else {
            0.0
        }
    }

    pub fn is_noble_gas_configuration(&self) -> bool {
        let n = self.quantum_numbers.n;
        matches!(n, 1 | 2 | 10 | 18 | 36 | 54 | 86 | 118) || self.coherence > 0.95
    }
}

#[derive(Debug, Clone)]
pub struct AttractorBasin {
    pub center: FieldConfiguration,
    pub radius: Float,
    pub depth: Float,
    pub stability: AttractorStability,
    pub transition_barriers: HashMap<AttractorId, Float>,
}

impl AttractorBasin {
    pub fn new(center: FieldConfiguration, radius: Float, depth: Float) -> Self {
        let stability = AttractorStability::from_coherence(
            center.coherence,
            center.is_noble_gas_configuration(),
        );

        Self {
            center,
            radius,
            depth,
            stability,
            transition_barriers: HashMap::new(),
        }
    }

    pub fn contains(&self, config: &FieldConfiguration) -> bool {
        let distance = self.center.distance_to(config);
        distance < self.radius
    }

    pub fn attraction_strength(&self, config: &FieldConfiguration) -> Float {
        let distance = self.center.distance_to(config);
        if distance < 1e-10 {
            return self.depth;
        }

        let within_basin = if distance < self.radius { 1.0 } else { 0.0 };
        let depth_factor = self.depth / (1.0 + distance);

        depth_factor * within_basin + depth_factor * 0.1 * (1.0 - within_basin)
    }

    pub fn add_transition_barrier(&mut self, target_id: AttractorId, barrier_height: Float) {
        self.transition_barriers.insert(target_id, barrier_height);
    }

    pub fn can_transition_to(&self, target_id: &AttractorId, energy: Float) -> bool {
        match self.transition_barriers.get(target_id) {
            Some(&barrier) => energy >= barrier,
            None => true,
        }
    }

    pub fn stability_lifetime(&self) -> Float {
        self.stability.stability_factor() * self.depth * 1e15
    }
}

#[derive(Debug, Clone)]
pub struct AttractorField {
    id: AttractorId,
    configuration: FieldConfiguration,
    basin: AttractorBasin,
    current_energy: Float,
    manifestation_threshold: Float,
    is_manifested: bool,
}

impl AttractorField {
    pub fn new(configuration: FieldConfiguration) -> Self {
        let id = AttractorId::from_quantum_numbers(&configuration.quantum_numbers);
        let basin = AttractorBasin::new(configuration.clone(), 0.3, configuration.coherence);
        let manifestation_threshold = 0.3 + configuration.quantum_numbers.n as Float * 0.02;

        Self {
            id,
            configuration,
            basin,
            current_energy: 0.0,
            manifestation_threshold,
            is_manifested: false,
        }
    }

    pub fn from_field_state(field: &HolographicFieldState, position: &Position3D) -> Option<Self> {
        let node = field.get_node_at(position)?;
        let amplitude = node.get_amplitude_at_scale(ScaleLevel::Atomic);

        if amplitude.magnitude() < 0.1 {
            return None;
        }

        let configuration = FieldConfiguration::new(node.archetype_vector);
        Some(Self::new(configuration))
    }

    pub fn id(&self) -> &AttractorId {
        &self.id
    }

    pub fn configuration(&self) -> &FieldConfiguration {
        &self.configuration
    }

    pub fn basin(&self) -> &AttractorBasin {
        &self.basin
    }

    pub fn quantum_numbers(&self) -> &QuantumNumberSet {
        &self.configuration.quantum_numbers
    }

    pub fn energy(&self) -> Float {
        self.current_energy
    }

    pub fn add_energy(&mut self, energy: Float) {
        self.current_energy += energy;
        if self.current_energy >= self.manifestation_threshold && !self.is_manifested {
            self.is_manifested = true;
        }
    }

    pub fn is_manifested(&self) -> bool {
        self.is_manifested
    }

    pub fn manifestation_threshold(&self) -> Float {
        self.manifestation_threshold
    }

    pub fn decay(&mut self, dt: Float, decay_rate: Float) {
        if self.current_energy > 0.0 {
            self.current_energy *= (1.0 - decay_rate * dt).max(0.0);
            if self.current_energy < self.manifestation_threshold * 0.5 {
                self.is_manifested = false;
            }
        }
    }

    pub fn relax_toward_attractor(&mut self, dt: Float) {
        let relaxation_rate = 0.1;
        let target_energy = self.basin.depth * 0.5;

        self.current_energy += (target_energy - self.current_energy) * relaxation_rate * dt;
    }

    pub fn can_absorb(&self, other: &AttractorField) -> bool {
        let distance = self.configuration.distance_to(&other.configuration);
        distance < self.basin.radius * 0.5 && self.basin.depth > other.basin.depth
    }

    pub fn absorb(&mut self, other: &AttractorField) -> Float {
        let energy_gained = other.current_energy * other.basin.depth / self.basin.depth;
        self.add_energy(energy_gained);
        energy_gained
    }

    pub fn stability(&self) -> AttractorStability {
        self.basin.stability
    }

    pub fn is_stable(&self) -> bool {
        matches!(
            self.basin.stability,
            AttractorStability::Stable | AttractorStability::NobleGas
        )
    }
}

impl Default for AttractorField {
    fn default() -> Self {
        Self::new(FieldConfiguration::ground_state())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attractor_id_creation() {
        let id = AttractorId::new(42);
        assert_eq!(id.raw(), 42);
    }

    #[test]
    fn test_field_configuration_creation() {
        let config = FieldConfiguration::new([0.5; NUM_ARCHETYPES]);
        assert!(config.coherence >= 0.0 && config.coherence <= 1.0);
        assert!(config.energy_level > 0.0);
    }

    #[test]
    fn test_ground_state() {
        let config = FieldConfiguration::ground_state();
        assert!(config.quantum_numbers.n >= 1);
        assert!(config.quantum_numbers.l <= config.quantum_numbers.n);
    }

    #[test]
    fn test_attractor_basin_contains() {
        let center = FieldConfiguration::ground_state();
        let basin = AttractorBasin::new(center.clone(), 0.5, 0.8);

        assert!(basin.contains(&center));

        let mut far_config = center.clone();
        far_config.archetype_vector[0] = 0.99;
        assert!(!basin.contains(&far_config));
    }

    #[test]
    fn test_attractor_field_creation() {
        let field = AttractorField::new(FieldConfiguration::ground_state());

        assert!(!field.is_manifested());
        assert!(field.manifestation_threshold() > 0.0);
    }

    #[test]
    fn test_energy_addition() {
        let mut field = AttractorField::new(FieldConfiguration::ground_state());

        let threshold = field.manifestation_threshold();
        field.add_energy(threshold + 0.1);

        assert!(field.is_manifested());
    }

    #[test]
    fn test_energy_decay() {
        let mut field = AttractorField::new(FieldConfiguration::ground_state());
        field.add_energy(1.0);

        let initial_energy = field.energy();
        field.decay(1.0, 0.5);

        assert!(field.energy() < initial_energy);
    }

    #[test]
    fn test_attractor_stability() {
        let stable = AttractorStability::Stable;
        let unstable = AttractorStability::Unstable;

        assert!(stable.stability_factor() > unstable.stability_factor());
    }

    #[test]
    fn test_configuration_distance() {
        let config1 = FieldConfiguration::ground_state();
        let config2 = FieldConfiguration::hydrogen_configuration();

        let distance = config1.distance_to(&config2);
        assert!(distance > 0.0);
    }

    #[test]
    fn test_archetype_similarity() {
        let config1 = FieldConfiguration::new([0.5; NUM_ARCHETYPES]);
        let config2 = FieldConfiguration::new([0.5; NUM_ARCHETYPES]);

        let similarity = config1.archetype_similarity(&config2);
        assert!((similarity - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_relaxation() {
        let mut field = AttractorField::new(FieldConfiguration::ground_state());
        field.add_energy(2.0);

        let initial_energy = field.energy();
        field.relax_toward_attractor(1.0);

        assert!(field.energy() != initial_energy);
    }
}
