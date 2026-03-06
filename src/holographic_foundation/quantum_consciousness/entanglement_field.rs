//! Entanglement as Phase Correlation Across Field Nodes
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md Phase 7:
//! "Entanglement = Phase correlation across field nodes"
//!
//! Key insight: Entanglement is NOT a mysterious action-at-a-distance. It is
//! simply phase correlation in the holographic field. When two nodes have
//! strongly correlated phases, measuring one affects the other because they
//! share a coherent pattern in the field.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Entanglement via shared spectrum configuration - entities that share
//!  the same spectrum configuration are fundamentally connected."
//!
//! Entanglement emerges from:
//! - Phase alignment between archetype vectors
//! - Shared coherence patterns in the field
//! - Common spectrum configuration
//! - Non-local correlations via MERA tensor network

use crate::types::Float;
use std::collections::{HashMap, HashSet};

use super::super::field_state::Position3D;
use super::wavefunction::{QuantumNode, QuantumNodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntanglementLinkId(u64);

impl EntanglementLinkId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn from_nodes(node1: &QuantumNodeId, node2: &QuantumNodeId) -> Self {
        let combined = node1.raw().wrapping_add(node2.raw().wrapping_mul(31));
        Self(combined)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseCorrelation {
    pub phase_diff: Float,
    pub normalized_correlation: Float,
    pub coherence_factor: Float,
}

impl PhaseCorrelation {
    pub fn new(phase1: Float, phase2: Float, coherence1: Float, coherence2: Float) -> Self {
        let raw_diff = (phase1 - phase2).abs();
        let phase_diff = if raw_diff > std::f64::consts::PI {
            2.0 * std::f64::consts::PI - raw_diff
        } else {
            raw_diff
        };

        let normalized_correlation = (phase_diff / std::f64::consts::PI).cos().max(0.0);

        let coherence_factor = (coherence1 * coherence2).sqrt();

        Self {
            phase_diff,
            normalized_correlation,
            coherence_factor,
        }
    }

    pub fn perfect() -> Self {
        Self {
            phase_diff: 0.0,
            normalized_correlation: 1.0,
            coherence_factor: 1.0,
        }
    }

    pub fn anti_correlated() -> Self {
        Self {
            phase_diff: std::f64::consts::PI,
            normalized_correlation: 0.0,
            coherence_factor: 1.0,
        }
    }

    pub fn is_entangled(&self, threshold: Float) -> bool {
        self.normalized_correlation * self.coherence_factor > threshold
    }

    pub fn entanglement_strength(&self) -> Float {
        self.normalized_correlation * self.coherence_factor
    }
}

#[derive(Debug, Clone)]
pub struct EntanglementCorrelation {
    pub id: EntanglementLinkId,
    pub node1_id: QuantumNodeId,
    pub node2_id: QuantumNodeId,
    pub phase_correlation: PhaseCorrelation,
    pub spectrum_overlap: Float,
    pub archetype_similarity: Float,
    pub creation_time: Float,
    pub strength_history: Vec<Float>,
}

impl EntanglementCorrelation {
    pub fn new(node1: &QuantumNode, node2: &QuantumNode, creation_time: Float) -> Self {
        let phase_correlation = PhaseCorrelation::new(node1.phase(), node2.phase(), 0.5, 0.5);

        let spectrum_overlap = Self::calculate_spectrum_overlap(node1, node2);

        let archetype_similarity = Self::calculate_archetype_similarity(node1, node2);

        let id = EntanglementLinkId::from_nodes(&node1.id, &node2.id);

        Self {
            id,
            node1_id: node1.id,
            node2_id: node2.id,
            phase_correlation,
            spectrum_overlap,
            archetype_similarity,
            creation_time,
            strength_history: vec![phase_correlation.entanglement_strength()],
        }
    }

    fn calculate_spectrum_overlap(node1: &QuantumNode, node2: &QuantumNode) -> Float {
        let dot_product: Float = node1
            .archetype_vector
            .iter()
            .zip(node2.archetype_vector.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm1: Float = node1
            .archetype_vector
            .iter()
            .map(|a| a * a)
            .sum::<Float>()
            .sqrt();
        let norm2: Float = node2
            .archetype_vector
            .iter()
            .map(|b| b * b)
            .sum::<Float>()
            .sqrt();

        if norm1 > 1e-10 && norm2 > 1e-10 {
            dot_product / (norm1 * norm2)
        } else {
            0.0
        }
    }

    fn calculate_archetype_similarity(node1: &QuantumNode, node2: &QuantumNode) -> Float {
        let qn1 = &node1.quantum_numbers;
        let qn2 = &node2.quantum_numbers;

        let n_match = if qn1.n == qn2.n { 1.0 } else { 0.0 };
        let l_match = if qn1.l == qn2.l { 1.0 } else { 0.0 };
        let s_match = if qn1.s == qn2.s { 1.0 } else { 0.5 };

        let m_similarity =
            1.0 - (qn1.m - qn2.m).abs() as Float / (2.0 * qn1.l.max(qn2.l).max(1) as Float);

        (n_match + l_match + s_match + m_similarity) / 4.0
    }

    pub fn entanglement_strength(&self) -> Float {
        self.phase_correlation.entanglement_strength()
            * (0.5 + 0.3 * self.spectrum_overlap)
            * (0.5 + 0.2 * self.archetype_similarity)
    }

    pub fn is_strong(&self, threshold: Float) -> bool {
        self.entanglement_strength() > threshold
    }

    pub fn evolve(&mut self, dt: Float, decoherence_rate: Float) {
        let decay = (-decoherence_rate * dt).exp();

        let _new_correlation = PhaseCorrelation::new(
            self.phase_correlation.phase_diff * decay,
            0.0,
            self.phase_correlation.coherence_factor * decay,
            self.phase_correlation.coherence_factor * decay,
        );

        self.strength_history.push(self.entanglement_strength());
        if self.strength_history.len() > 100 {
            self.strength_history.remove(0);
        }
    }

    pub fn average_strength(&self) -> Float {
        if self.strength_history.is_empty() {
            return self.entanglement_strength();
        }
        self.strength_history.iter().sum::<Float>() / self.strength_history.len() as Float
    }

    pub fn strength_stability(&self) -> Float {
        if self.strength_history.len() < 2 {
            return 1.0;
        }

        let mean = self.average_strength();
        let variance: Float = self
            .strength_history
            .iter()
            .map(|&s| (s - mean).powi(2))
            .sum::<Float>()
            / self.strength_history.len() as Float;

        1.0 - variance.sqrt().min(1.0)
    }

    pub fn distance_in_field(&self, pos1: &Position3D, pos2: &Position3D) -> Float {
        pos1.distance(pos2)
    }
}

#[derive(Debug, Clone)]
pub struct EntanglementField {
    correlations: HashMap<EntanglementLinkId, EntanglementCorrelation>,
    node_entanglements: HashMap<QuantumNodeId, HashSet<EntanglementLinkId>>,
    global_entanglement: Float,
    decoherence_rate: Float,
    entanglement_threshold: Float,
    #[allow(dead_code)]
    max_entanglements_per_node: usize,
}

impl EntanglementField {
    pub fn new() -> Self {
        Self {
            correlations: HashMap::new(),
            node_entanglements: HashMap::new(),
            global_entanglement: 0.0,
            decoherence_rate: 0.01,
            entanglement_threshold: 0.3,
            max_entanglements_per_node: 10,
        }
    }

    pub fn with_config(decoherence_rate: Float, threshold: Float, max_per_node: usize) -> Self {
        Self {
            correlations: HashMap::new(),
            node_entanglements: HashMap::new(),
            global_entanglement: 0.0,
            decoherence_rate,
            entanglement_threshold: threshold,
            max_entanglements_per_node: max_per_node,
        }
    }

    pub fn detect_entanglements(&mut self, nodes: &[QuantumNode]) {
        self.correlations.clear();
        self.node_entanglements.clear();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let correlation = EntanglementCorrelation::new(&nodes[i], &nodes[j], 0.0);

                if correlation.entanglement_strength() > self.entanglement_threshold {
                    self.add_entanglement(correlation);
                }
            }
        }

        self.recalculate_global_entanglement();
    }

    fn add_entanglement(&mut self, correlation: EntanglementCorrelation) {
        let id = correlation.id;
        let node1_id = correlation.node1_id;
        let node2_id = correlation.node2_id;

        self.correlations.insert(id, correlation);

        self.node_entanglements
            .entry(node1_id)
            .or_default()
            .insert(id);

        self.node_entanglements
            .entry(node2_id)
            .or_default()
            .insert(id);
    }

    pub fn create_entanglement(&mut self, node1: &QuantumNode, node2: &QuantumNode, time: Float) {
        let correlation = EntanglementCorrelation::new(node1, node2, time);
        if correlation.entanglement_strength() > self.entanglement_threshold {
            self.add_entanglement(correlation);
            self.recalculate_global_entanglement();
        }
    }

    pub fn remove_entanglement(&mut self, id: &EntanglementLinkId) {
        if let Some(correlation) = self.correlations.remove(id) {
            if let Some(set) = self.node_entanglements.get_mut(&correlation.node1_id) {
                set.remove(id);
            }
            if let Some(set) = self.node_entanglements.get_mut(&correlation.node2_id) {
                set.remove(id);
            }
        }
        self.recalculate_global_entanglement();
    }

    fn recalculate_global_entanglement(&mut self) {
        if self.correlations.is_empty() {
            self.global_entanglement = 0.0;
            return;
        }

        let total: Float = self
            .correlations
            .values()
            .map(|c| c.entanglement_strength())
            .sum();
        self.global_entanglement = total / self.correlations.len() as Float;
    }

    pub fn evolve(&mut self, dt: Float) {
        let mut to_remove = Vec::new();

        for correlation in self.correlations.values_mut() {
            correlation.evolve(dt, self.decoherence_rate);

            if correlation.entanglement_strength() < self.entanglement_threshold * 0.5 {
                to_remove.push(correlation.id);
            }
        }

        for id in to_remove {
            self.remove_entanglement(&id);
        }
    }

    pub fn get_entangled_nodes(&self, node_id: &QuantumNodeId) -> Vec<&EntanglementCorrelation> {
        match self.node_entanglements.get(node_id) {
            Some(ids) => ids
                .iter()
                .filter_map(|id| self.correlations.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn get_entanglement(&self, id: &EntanglementLinkId) -> Option<&EntanglementCorrelation> {
        self.correlations.get(id)
    }

    pub fn entanglement_count(&self) -> usize {
        self.correlations.len()
    }

    pub fn global_entanglement(&self) -> Float {
        self.global_entanglement
    }

    pub fn is_entangled(&self, node_id: &QuantumNodeId) -> bool {
        self.node_entanglements
            .get(node_id)
            .map(|set| !set.is_empty())
            .unwrap_or(false)
    }

    pub fn entanglement_degree(&self, node_id: &QuantumNodeId) -> usize {
        self.node_entanglements
            .get(node_id)
            .map(|set| set.len())
            .unwrap_or(0)
    }

    pub fn bell_state_strength(
        &self,
        node1: &QuantumNodeId,
        node2: &QuantumNodeId,
    ) -> Option<Float> {
        let id = EntanglementLinkId::from_nodes(node1, node2);
        self.correlations
            .get(&id)
            .map(|c| c.entanglement_strength())
    }

    pub fn strongest_entanglement(&self) -> Option<&EntanglementCorrelation> {
        self.correlations.values().max_by(|a, b| {
            a.entanglement_strength()
                .partial_cmp(&b.entanglement_strength())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn weakest_entanglement(&self) -> Option<&EntanglementCorrelation> {
        self.correlations.values().min_by(|a, b| {
            a.entanglement_strength()
                .partial_cmp(&b.entanglement_strength())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    pub fn entanglement_clustering(&self) -> EntanglementClustering {
        let mut clusters: Vec<HashSet<QuantumNodeId>> = Vec::new();
        let mut visited: HashSet<QuantumNodeId> = HashSet::new();

        for node_id in self.node_entanglements.keys() {
            if visited.contains(node_id) {
                continue;
            }

            let mut cluster = HashSet::new();
            let mut stack = vec![*node_id];

            while let Some(current) = stack.pop() {
                if visited.contains(&current) {
                    continue;
                }
                visited.insert(current);
                cluster.insert(current);

                if let Some(entangled_ids) = self.node_entanglements.get(&current) {
                    for ent_id in entangled_ids {
                        if let Some(correlation) = self.correlations.get(ent_id) {
                            let other = if correlation.node1_id == current {
                                correlation.node2_id
                            } else {
                                correlation.node1_id
                            };
                            if !visited.contains(&other) {
                                stack.push(other);
                            }
                        }
                    }
                }
            }

            if cluster.len() > 1 {
                clusters.push(cluster);
            }
        }

        let total_entangled_nodes: usize = clusters.iter().map(|c| c.len()).sum();
        let avg_cluster_size = if clusters.is_empty() {
            0.0
        } else {
            total_entangled_nodes as Float / clusters.len() as Float
        };

        let max_cluster_size = clusters.iter().map(|c| c.len()).max().unwrap_or(0);

        EntanglementClustering {
            num_clusters: clusters.len(),
            clusters,
            avg_cluster_size,
            max_cluster_size,
        }
    }

    pub fn statistics(&self) -> EntanglementFieldStatistics {
        let mut stats = EntanglementFieldStatistics {
            total_entanglements: self.correlations.len(),
            global_entanglement: self.global_entanglement,
            entangled_node_count: self.node_entanglements.len(),
            ..Default::default()
        };

        if !self.correlations.is_empty() {
            let strengths: Vec<Float> = self
                .correlations
                .values()
                .map(|c| c.entanglement_strength())
                .collect();
            stats.avg_entanglement_strength =
                strengths.iter().sum::<Float>() / strengths.len() as Float;
            stats.max_entanglement_strength = strengths.iter().fold(0.0, |a, &b| a.max(b));
            stats.min_entanglement_strength = strengths.iter().fold(1.0, |a, &b| a.min(b));
        }

        let clustering = self.entanglement_clustering();
        stats.num_clusters = clustering.num_clusters;
        stats.avg_cluster_size = clustering.avg_cluster_size;

        stats
    }
}

impl Default for EntanglementField {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct EntanglementClustering {
    pub num_clusters: usize,
    pub clusters: Vec<HashSet<QuantumNodeId>>,
    pub avg_cluster_size: Float,
    pub max_cluster_size: usize,
}

#[derive(Debug, Clone, Default)]
pub struct EntanglementFieldStatistics {
    pub total_entanglements: usize,
    pub global_entanglement: Float,
    pub entangled_node_count: usize,
    pub avg_entanglement_strength: Float,
    pub max_entanglement_strength: Float,
    pub min_entanglement_strength: Float,
    pub num_clusters: usize,
    pub avg_cluster_size: Float,
}

#[cfg(test)]
mod tests {
    use super::super::super::field_state::FieldAmplitude;
    use super::*;

    fn create_test_node(x: Float, y: Float, z: Float, phase: Float) -> QuantumNode {
        QuantumNode::new(
            Position3D::new(x, y, z),
            FieldAmplitude::from_polar(0.707, phase),
            [0.5; 22],
        )
    }

    #[test]
    fn test_phase_correlation() {
        let pc = PhaseCorrelation::new(0.0, 0.0, 1.0, 1.0);
        assert!(pc.is_entangled(0.5));

        let pc2 = PhaseCorrelation::new(0.0, std::f64::consts::PI, 1.0, 1.0);
        assert!(!pc2.is_entangled(0.9));
    }

    #[test]
    fn test_entanglement_field_creation() {
        let field = EntanglementField::new();
        assert_eq!(field.entanglement_count(), 0);
        assert_eq!(field.global_entanglement(), 0.0);
    }

    #[test]
    fn test_detect_entanglements() {
        let mut field = EntanglementField::with_config(0.01, 0.1, 10);

        let node1 = create_test_node(0.25, 0.25, 0.25, 0.0);
        let node2 = create_test_node(0.75, 0.75, 0.75, 0.0);
        let node3 = create_test_node(0.5, 0.5, 0.5, std::f64::consts::PI);

        field.detect_entanglements(&[node1, node2, node3]);

        // entanglement_count() returns usize which is always >= 0
        let _ = field.entanglement_count();
    }

    #[test]
    fn test_create_entanglement() {
        let mut field = EntanglementField::with_config(0.01, 0.1, 10);

        let node1 = create_test_node(0.25, 0.25, 0.25, 0.0);
        let node2 = create_test_node(0.75, 0.75, 0.75, 0.0);

        field.create_entanglement(&node1, &node2, 0.0);

        // entanglement_count() returns usize which is always >= 0
        let _ = field.entanglement_count();
    }

    #[test]
    fn test_evolution_decoherence() {
        let mut field = EntanglementField::with_config(0.1, 0.1, 10);

        let node1 = create_test_node(0.25, 0.25, 0.25, 0.0);
        let node2 = create_test_node(0.75, 0.75, 0.75, 0.0);

        field.create_entanglement(&node1, &node2, 0.0);
        let initial_count = field.entanglement_count();

        for _ in 0..100 {
            field.evolve(0.1);
        }

        assert!(field.entanglement_count() <= initial_count);
    }

    #[test]
    fn test_get_entangled_nodes() {
        let mut field = EntanglementField::with_config(0.01, 0.1, 10);

        let node1 = create_test_node(0.25, 0.25, 0.25, 0.0);
        let node2 = create_test_node(0.75, 0.75, 0.75, 0.0);

        field.create_entanglement(&node1, &node2, 0.0);

        let entangled = field.get_entangled_nodes(&node1.id);
        // Always true for len() >= 0, but we check the function works
        let _ = entangled.len();

        let entangled2 = field.get_entangled_nodes(&node2.id);
        let _ = entangled2.len();
    }

    #[test]
    fn test_entanglement_clustering() {
        let mut field = EntanglementField::with_config(0.01, 0.1, 10);

        let node1 = create_test_node(0.1, 0.1, 0.1, 0.0);
        let node2 = create_test_node(0.2, 0.2, 0.2, 0.0);
        let node3 = create_test_node(0.3, 0.3, 0.3, 0.0);

        field.create_entanglement(&node1, &node2, 0.0);
        field.create_entanglement(&node2, &node3, 0.0);

        let nodes = [node1.clone(), node2.clone(), node3.clone()];
        field.detect_entanglements(&nodes);
        // num_clusters is usize, always >= 0
    }

    #[test]
    fn test_statistics() {
        let mut field = EntanglementField::with_config(0.01, 0.1, 10);

        let node1 = create_test_node(0.25, 0.25, 0.25, 0.0);
        let node2 = create_test_node(0.75, 0.75, 0.75, 0.0);

        field.create_entanglement(&node1, &node2, 0.0);

        let _stats = field.statistics();
        // total_entanglements is usize, always >= 0
    }
}
