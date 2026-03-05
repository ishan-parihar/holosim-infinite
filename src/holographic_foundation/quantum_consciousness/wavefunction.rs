//! Quantum Wavefunction from Holographic Field Amplitudes
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 7:
//! "Wavefunction = Field amplitude at quantum resolution (~10^-35m)"
//!
//! The wavefunction is NOT a separate entity - it emerges from the holographic
//! field at the Quantum scale level. This implements the key insight:
//!
//! **Quantum phenomena are holographic phenomena at the finest resolution.**
//!
//! Key principles:
//! - Wavefunction amplitude = FieldNode.amplitudes[ScaleLevel::Quantum]
//! - Probability density = |amplitude|^2 (Born rule preserved)
//! - Superposition = Multiple field nodes with non-zero amplitudes
//! - Phase = Coherence relationship between nodes

use crate::types::Float;
use std::collections::HashMap;

use super::super::field_state::{FieldAmplitude, FieldNode, HolographicFieldState, Position3D};
use super::super::scale_level::ScaleLevel;
use super::quantum_numbers::{ArchetypeToQuantumMapping, QuantumNumberSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumNodeId(u64);

impl QuantumNodeId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn from_position(pos: &Position3D) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        pos.x.to_bits().hash(&mut hasher);
        pos.y.to_bits().hash(&mut hasher);
        pos.z.to_bits().hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumNode {
    pub id: QuantumNodeId,
    pub position: Position3D,
    pub amplitude: FieldAmplitude,
    pub archetype_vector: [Float; 22],
    pub quantum_numbers: QuantumNumberSet,
}

impl QuantumNode {
    pub fn new(
        position: Position3D,
        amplitude: FieldAmplitude,
        archetype_vector: [Float; 22],
    ) -> Self {
        let id = QuantumNodeId::from_position(&position);
        let quantum_numbers = ArchetypeToQuantumMapping::derive_quantum_numbers(&archetype_vector);
        Self {
            id,
            position,
            amplitude,
            archetype_vector,
            quantum_numbers,
        }
    }

    pub fn from_field_node(node: &FieldNode) -> Option<Self> {
        let quantum_amp = node.get_amplitude_at_scale(ScaleLevel::Quantum);
        if quantum_amp.magnitude() > 1e-10 {
            Some(Self::new(
                node.position,
                *quantum_amp,
                node.archetype_vector,
            ))
        } else {
            None
        }
    }

    pub fn probability_density(&self) -> Float {
        self.amplitude.magnitude().powi(2)
    }

    pub fn phase(&self) -> Float {
        self.amplitude.phase()
    }

    pub fn is_entangled_with(&self, other: &QuantumNode, threshold: Float) -> bool {
        let phase_diff = (self.phase() - other.phase()).abs();
        let normalized_phase_diff = if phase_diff > std::f64::consts::PI {
            2.0 * std::f64::consts::PI - phase_diff
        } else {
            phase_diff
        };
        normalized_phase_diff < threshold
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum WavefunctionState {
    #[default]
    Pure,
    Superposition { num_components: usize },
    Mixed { purity: Float },
    Collapsed { basis_state: QuantumNumberSet },
}

#[derive(Debug, Clone)]
pub struct QuantumWavefunction {
    nodes: HashMap<QuantumNodeId, QuantumNode>,
    source_field_coherence: Float,
    state: WavefunctionState,
    total_probability: Float,
    normalization_applied: bool,
}

impl QuantumWavefunction {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            source_field_coherence: 0.0,
            state: WavefunctionState::Pure,
            total_probability: 0.0,
            normalization_applied: false,
        }
    }

    pub fn from_holographic_field(field: &HolographicFieldState) -> Self {
        let mut wavefunction = Self::new();
        wavefunction.extract_quantum_nodes(field);
        wavefunction.source_field_coherence = field.global_coherence();
        wavefunction.recalculate_state();
        wavefunction
    }

    pub fn add_node(&mut self, node: QuantumNode) {
        self.nodes.insert(node.id, node);
        self.recalculate_state();
    }

    fn extract_quantum_nodes(&mut self, field: &HolographicFieldState) {
        self.nodes.clear();
        field.for_each_node(&mut |node: &FieldNode| {
            if let Some(quantum_node) = QuantumNode::from_field_node(node) {
                self.nodes.insert(quantum_node.id, quantum_node);
            }
        });
        self.normalize();
    }

    pub fn normalize(&mut self) {
        let total: Float = self.nodes.values().map(|n| n.probability_density()).sum();
        if total > 1e-10 {
            let norm_factor = 1.0 / total.sqrt();
            for node in self.nodes.values_mut() {
                node.amplitude = FieldAmplitude::from_polar(
                    node.amplitude.magnitude() * norm_factor,
                    node.amplitude.phase(),
                );
            }
            self.total_probability = 1.0;
            self.normalization_applied = true;
        }
    }

    fn recalculate_state(&mut self) {
        let num_components = self.nodes.len();

        self.state = if num_components <= 1 {
            WavefunctionState::Pure
        } else {
            let total_prob: Float = self.nodes.values().map(|n| n.probability_density()).sum();
            let purity = if total_prob > 1e-10 {
                let mut sum_sq = 0.0;
                for node in self.nodes.values() {
                    let p = node.probability_density() / total_prob;
                    sum_sq += p * p;
                }
                sum_sq
            } else {
                1.0
            };

            if purity > 0.99 {
                WavefunctionState::Pure
            } else if purity > 0.5 {
                WavefunctionState::Superposition { num_components }
            } else {
                WavefunctionState::Mixed { purity }
            }
        };

        self.total_probability = self.nodes.values().map(|n| n.probability_density()).sum();
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn total_probability(&self) -> Float {
        self.total_probability
    }

    pub fn state(&self) -> &WavefunctionState {
        &self.state
    }

    pub fn source_coherence(&self) -> Float {
        self.source_field_coherence
    }

    pub fn is_normalized(&self) -> bool {
        self.normalization_applied && (self.total_probability - 1.0).abs() < 1e-6
    }

    pub fn get_node(&self, id: &QuantumNodeId) -> Option<&QuantumNode> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &QuantumNodeId) -> Option<&mut QuantumNode> {
        self.nodes.get_mut(id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &QuantumNode> {
        self.nodes.values()
    }

    pub fn superposition_components(&self) -> Vec<&QuantumNode> {
        self.nodes.values().collect()
    }

    pub fn most_probable_node(&self) -> Option<&QuantumNode> {
        self.nodes.values().max_by(|a, b| {
            a.probability_density()
                .partial_cmp(&b.probability_density())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn expectation_position(&self) -> Option<Position3D> {
        if self.nodes.is_empty() {
            return None;
        }

        let total_prob: Float = self.nodes.values().map(|n| n.probability_density()).sum();
        if total_prob < 1e-10 {
            return None;
        }

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for node in self.nodes.values() {
            let weight = node.probability_density() / total_prob;
            x += node.position.x * weight;
            y += node.position.y * weight;
            z += node.position.z * weight;
        }

        Some(Position3D::new(x, y, z))
    }

    pub fn position_uncertainty(&self) -> Option<Float> {
        let expected_pos = self.expectation_position()?;
        let total_prob: Float = self.nodes.values().map(|n| n.probability_density()).sum();

        let mut variance = 0.0;
        for node in self.nodes.values() {
            let weight = node.probability_density() / total_prob;
            let dx = node.position.x - expected_pos.x;
            let dy = node.position.y - expected_pos.y;
            let dz = node.position.z - expected_pos.z;
            variance += weight * (dx * dx + dy * dy + dz * dz);
        }

        Some(variance.sqrt())
    }

    pub fn apply_phase_shift(&mut self, delta_phase: Float) {
        for node in self.nodes.values_mut() {
            node.amplitude = FieldAmplitude::from_polar(
                node.amplitude.magnitude(),
                node.amplitude.phase() + delta_phase,
            );
        }
    }

    pub fn evolve(&mut self, dt: Float, hamiltonian_strength: Float) {
        for node in self.nodes.values_mut() {
            let energy_factor = node.quantum_numbers.energy_factor();
            let phase_evolution = hamiltonian_strength * dt * energy_factor;
            let magnitude_decay = 1.0 - 0.001 * dt;

            node.amplitude = FieldAmplitude::from_polar(
                node.amplitude.magnitude() * magnitude_decay,
                node.amplitude.phase() + phase_evolution,
            );
        }

        self.normalize();
        self.recalculate_state();
    }

    pub fn interference_with(&self, other: &QuantumWavefunction) -> InterferenceResult {
        let mut constructive = 0.0;
        let mut destructive = 0.0;
        let mut overlapping_nodes = 0;

        for (id, node) in &self.nodes {
            if let Some(other_node) = other.get_node(id) {
                let phase_diff = node.phase() - other_node.phase();
                let interference = phase_diff.cos();

                if interference > 0.0 {
                    constructive += interference * node.probability_density();
                } else {
                    destructive += interference.abs() * node.probability_density();
                }
                overlapping_nodes += 1;
            }
        }

        InterferenceResult {
            constructive_strength: constructive,
            destructive_strength: destructive,
            overlapping_nodes,
            total_nodes_self: self.nodes.len(),
            total_nodes_other: other.nodes.len(),
        }
    }

    pub fn coherence_measure(&self) -> Float {
        if self.nodes.len() < 2 {
            return 1.0;
        }

        let phases: Vec<Float> = self.nodes.values().map(|n| n.phase()).collect();
        let mean_phase: Float = phases.iter().copied().sum::<Float>() / phases.len() as Float;

        let variance: Float = phases
            .iter()
            .copied()
            .map(|p| {
                let diff = (p - mean_phase).abs();
                let normalized = if diff > std::f64::consts::PI {
                    2.0 * std::f64::consts::PI - diff
                } else {
                    diff
                };
                normalized * normalized
            })
            .sum::<Float>()
            / phases.len() as Float;

        1.0 - (variance / (std::f64::consts::PI * std::f64::consts::PI)).min(1.0)
    }

    pub fn mark_collapsed(&mut self, collapsed_node_id: &QuantumNodeId) {
        if let Some(collapsed_node) = self.nodes.get(collapsed_node_id).cloned() {
            let quantum_numbers = collapsed_node.quantum_numbers;
            self.nodes.clear();
            let mut new_node = collapsed_node;
            new_node.amplitude = FieldAmplitude::one();
            self.nodes.insert(new_node.id, new_node);

            self.state = WavefunctionState::Collapsed {
                basis_state: quantum_numbers,
            };
            self.total_probability = 1.0;
        }
    }

    pub fn statistics(&self) -> WavefunctionStatistics {
        let mut stats = WavefunctionStatistics {
            node_count: self.nodes.len(),
            total_probability: self.total_probability,
            coherence: self.coherence_measure(),
            source_field_coherence: self.source_field_coherence,
            is_normalized: self.normalization_applied,
            ..Default::default()
        };

        if let Some(pos) = self.expectation_position() {
            stats.expectation_position = Some(pos);
        }

        if let Some(unc) = self.position_uncertainty() {
            stats.position_uncertainty = unc;
        }

        stats
    }
}

impl Default for QuantumWavefunction {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterferenceResult {
    pub constructive_strength: Float,
    pub destructive_strength: Float,
    pub overlapping_nodes: usize,
    pub total_nodes_self: usize,
    pub total_nodes_other: usize,
}

impl InterferenceResult {
    pub fn net_interference(&self) -> Float {
        self.constructive_strength - self.destructive_strength
    }

    pub fn interference_ratio(&self) -> Float {
        let total = self.constructive_strength + self.destructive_strength;
        if total > 1e-10 {
            self.constructive_strength / total
        } else {
            0.5
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WavefunctionStatistics {
    pub node_count: usize,
    pub total_probability: Float,
    pub coherence: Float,
    pub source_field_coherence: Float,
    pub is_normalized: bool,
    pub expectation_position: Option<Position3D>,
    pub position_uncertainty: Float,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_field_with_quantum_amplitude() -> HolographicFieldState {
        let mut field = HolographicFieldState::new(1.0);
        let pos = Position3D::new(0.5, 0.5, 0.5);

        field.set_amplitude(
            &pos,
            ScaleLevel::Quantum,
            FieldAmplitude::from_polar(0.707, 0.0),
        );

        field
    }

    #[test]
    fn test_wavefunction_creation() {
        let wf = QuantumWavefunction::new();
        assert_eq!(wf.node_count(), 0);
        assert!(!wf.is_normalized());
    }

    #[test]
    fn test_wavefunction_from_field() {
        let field = create_test_field_with_quantum_amplitude();
        let wf = QuantumWavefunction::from_holographic_field(&field);

        assert!(wf.node_count() >= 1);
        assert!(wf.is_normalized());
    }

    #[test]
    fn test_quantum_node_probability() {
        let node = QuantumNode::new(
            Position3D::new(0.5, 0.5, 0.5),
            FieldAmplitude::from_polar(0.707, std::f64::consts::PI / 4.0),
            [0.5; 22],
        );

        let expected_prob = 0.707 * 0.707;
        assert!((node.probability_density() - expected_prob).abs() < 1e-3);
        assert!((node.phase() - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalization() {
        let mut wf = QuantumWavefunction::new();

        let node1 = QuantumNode::new(
            Position3D::new(0.25, 0.25, 0.25),
            FieldAmplitude::from_polar(1.0, 0.0),
            [0.5; 22],
        );
        let node2 = QuantumNode::new(
            Position3D::new(0.75, 0.75, 0.75),
            FieldAmplitude::from_polar(1.0, 0.0),
            [0.5; 22],
        );

        wf.add_node(node1);
        wf.add_node(node2);
        wf.normalize();

        assert!((wf.total_probability() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_expectation_position() {
        let mut wf = QuantumWavefunction::new();

        let node1 = QuantumNode::new(
            Position3D::new(0.0, 0.0, 0.0),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );
        let node2 = QuantumNode::new(
            Position3D::new(1.0, 1.0, 1.0),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );

        wf.add_node(node1);
        wf.add_node(node2);

        let expected = wf.expectation_position().unwrap();
        assert!((expected.x - 0.5).abs() < 1e-10);
        assert!((expected.y - 0.5).abs() < 1e-10);
        assert!((expected.z - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_phase_shift() {
        let mut wf = QuantumWavefunction::new();

        let node = QuantumNode::new(
            Position3D::new(0.5, 0.5, 0.5),
            FieldAmplitude::from_polar(1.0, 0.0),
            [0.5; 22],
        );
        wf.add_node(node);

        wf.apply_phase_shift(std::f64::consts::PI / 2.0);

        let shifted_node = wf.nodes().next().unwrap();
        assert!((shifted_node.phase() - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_evolution() {
        let field = create_test_field_with_quantum_amplitude();
        let mut wf = QuantumWavefunction::from_holographic_field(&field);

        let initial_phase = wf.nodes().next().map(|n| n.phase()).unwrap_or(0.0);
        wf.evolve(0.1, 1.0);
        let final_phase = wf.nodes().next().map(|n| n.phase()).unwrap_or(0.0);

        assert!(final_phase != initial_phase);
    }

    #[test]
    fn test_interference() {
        let mut wf1 = QuantumWavefunction::new();
        let mut wf2 = QuantumWavefunction::new();

        let node1 = QuantumNode::new(
            Position3D::new(0.5, 0.5, 0.5),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );
        let node2 = QuantumNode::new(
            Position3D::new(0.5, 0.5, 0.5),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );

        wf1.add_node(node1);
        wf2.add_node(node2);

        let result = wf1.interference_with(&wf2);
        assert!(result.constructive_strength > 0.0);
    }

    #[test]
    fn test_coherence_measure() {
        let mut wf = QuantumWavefunction::new();

        let node1 = QuantumNode::new(
            Position3D::new(0.25, 0.25, 0.25),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );
        let node2 = QuantumNode::new(
            Position3D::new(0.75, 0.75, 0.75),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );

        wf.add_node(node1);
        wf.add_node(node2);

        assert!(wf.coherence_measure() > 0.0);
    }

    #[test]
    fn test_collapse() {
        let mut wf = QuantumWavefunction::new();

        let node1 = QuantumNode::new(
            Position3D::new(0.25, 0.25, 0.25),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );
        let node2 = QuantumNode::new(
            Position3D::new(0.75, 0.75, 0.75),
            FieldAmplitude::from_polar(0.707, 0.0),
            [0.5; 22],
        );

        let collapse_id = node1.id;
        wf.add_node(node1);
        wf.add_node(node2);

        wf.mark_collapsed(&collapse_id);

        assert_eq!(wf.node_count(), 1);
        assert!(matches!(wf.state(), WavefunctionState::Collapsed { .. }));
    }
}
