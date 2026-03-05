//! Trophic Coupling Through Field Coherence
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Trophic levels = field coherence hierarchy
//!  Energy flow = field amplitude transfer"
//!
//! # Key Insight
//!
//! Trophic relationships are NOT just feeding relationships, but field
//! coupling dynamics where:
//! - Energy flows from lower to higher trophic levels via field amplitude transfer
//! - Trophic efficiency = field coherence transfer efficiency
//! - Food web stability = field network stability

use crate::holographic_foundation::ecosystem_dynamics::species_field::{
    SpeciesId, SpeciesInteraction,
};
use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrophicNodeId(pub u64);

impl TrophicNodeId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrophicLinkId(pub u64);

impl TrophicLinkId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrophicLevel {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Apex = 5,
}

impl TrophicLevel {
    pub fn from_int(level: usize) -> Self {
        match level {
            0 => TrophicLevel::Level0,
            1 => TrophicLevel::Level1,
            2 => TrophicLevel::Level2,
            3 => TrophicLevel::Level3,
            4 => TrophicLevel::Level4,
            _ => TrophicLevel::Apex,
        }
    }

    pub fn efficiency(&self) -> Float {
        match self {
            TrophicLevel::Level0 => 1.0,
            TrophicLevel::Level1 => 0.10,
            TrophicLevel::Level2 => 0.10,
            TrophicLevel::Level3 => 0.10,
            TrophicLevel::Level4 => 0.08,
            TrophicLevel::Apex => 0.05,
        }
    }

    pub fn field_coherence_threshold(&self) -> Float {
        match self {
            TrophicLevel::Level0 => 0.3,
            TrophicLevel::Level1 => 0.5,
            TrophicLevel::Level2 => 0.65,
            TrophicLevel::Level3 => 0.75,
            TrophicLevel::Level4 => 0.85,
            TrophicLevel::Apex => 0.95,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TrophicLevel::Level0 => "Producers (autotrophs)",
            TrophicLevel::Level1 => "Primary consumers (herbivores)",
            TrophicLevel::Level2 => "Secondary consumers (carnivores)",
            TrophicLevel::Level3 => "Tertiary consumers",
            TrophicLevel::Level4 => "Quaternary consumers",
            TrophicLevel::Apex => "Apex predators",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrophicNode {
    pub id: TrophicNodeId,
    pub species_id: SpeciesId,
    pub level: TrophicLevel,
    pub biomass: Float,
    pub field_amplitude: Float,
    pub coherence: Float,
    pub outgoing_links: Vec<TrophicLinkId>,
    pub incoming_links: Vec<TrophicLinkId>,
}

impl TrophicNode {
    pub fn new(species_id: SpeciesId, level: TrophicLevel, initial_biomass: Float) -> Self {
        let field_amplitude = initial_biomass.sqrt();
        let coherence = level.field_coherence_threshold();

        Self {
            id: TrophicNodeId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            species_id,
            level,
            biomass: initial_biomass,
            field_amplitude,
            coherence,
            outgoing_links: Vec::new(),
            incoming_links: Vec::new(),
        }
    }

    pub fn available_energy(&self) -> Float {
        self.biomass * self.coherence * self.level.efficiency()
    }

    pub fn field_strength(&self) -> Float {
        self.field_amplitude * self.coherence
    }

    pub fn update(&mut self, _dt: Float) {
        self.field_amplitude = self.biomass.sqrt();
    }
}

#[derive(Debug, Clone)]
pub struct TrophicLink {
    pub id: TrophicLinkId,
    pub source_id: TrophicNodeId,
    pub target_id: TrophicNodeId,
    pub interaction_type: SpeciesInteraction,
    pub strength: Float,
    pub energy_transfer_rate: Float,
    pub coupling_coefficient: Float,
}

impl TrophicLink {
    pub fn new(
        source_id: TrophicNodeId,
        target_id: TrophicNodeId,
        interaction_type: SpeciesInteraction,
        strength: Float,
    ) -> Self {
        let energy_transfer_rate = match interaction_type {
            SpeciesInteraction::Predation => 0.1 * strength.abs(),
            SpeciesInteraction::Mutualism => 0.05 * strength,
            _ => 0.08 * strength.abs(),
        };

        let coupling_coefficient = interaction_type.field_coupling_sign() * strength.abs();

        Self {
            id: TrophicLinkId::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0),
            ),
            source_id,
            target_id,
            interaction_type,
            strength,
            energy_transfer_rate,
            coupling_coefficient,
        }
    }

    pub fn energy_flow(&self, source_biomass: Float) -> Float {
        source_biomass * self.energy_transfer_rate
    }

    pub fn is_predation(&self) -> bool {
        matches!(self.interaction_type, SpeciesInteraction::Predation)
    }

    pub fn is_mutualism(&self) -> bool {
        matches!(self.interaction_type, SpeciesInteraction::Mutualism)
    }
}

#[derive(Debug, Clone)]
pub struct EnergyFlow {
    pub source_id: TrophicNodeId,
    pub target_id: TrophicNodeId,
    pub amount: Float,
    pub efficiency: Float,
    pub pathway: Vec<TrophicNodeId>,
}

impl EnergyFlow {
    pub fn new(source_id: TrophicNodeId, target_id: TrophicNodeId, amount: Float) -> Self {
        Self {
            source_id,
            target_id,
            amount,
            efficiency: 0.1,
            pathway: vec![source_id, target_id],
        }
    }

    pub fn net_energy(&self) -> Float {
        self.amount * self.efficiency
    }
}

#[derive(Debug, Clone)]
pub struct TrophicCoupling {
    pub node: TrophicNode,
    pub prey_nodes: Vec<TrophicNodeId>,
    pub predator_nodes: Vec<TrophicNodeId>,
    pub mutualist_nodes: Vec<TrophicNodeId>,
    pub competitor_nodes: Vec<TrophicNodeId>,
    pub coupling_strength: Float,
}

impl TrophicCoupling {
    pub fn new(node: TrophicNode) -> Self {
        let coupling_strength = node.coherence;
        Self {
            node,
            prey_nodes: Vec::new(),
            predator_nodes: Vec::new(),
            mutualist_nodes: Vec::new(),
            competitor_nodes: Vec::new(),
            coupling_strength,
        }
    }

    pub fn add_prey(&mut self, prey_id: TrophicNodeId) {
        if !self.prey_nodes.contains(&prey_id) {
            self.prey_nodes.push(prey_id);
        }
    }

    pub fn add_predator(&mut self, predator_id: TrophicNodeId) {
        if !self.predator_nodes.contains(&predator_id) {
            self.predator_nodes.push(predator_id);
        }
    }

    pub fn add_mutualist(&mut self, mutualist_id: TrophicNodeId) {
        if !self.mutualist_nodes.contains(&mutualist_id) {
            self.mutualist_nodes.push(mutualist_id);
        }
    }

    pub fn add_competitor(&mut self, competitor_id: TrophicNodeId) {
        if !self.competitor_nodes.contains(&competitor_id) {
            self.competitor_nodes.push(competitor_id);
        }
    }

    pub fn total_trophic_pressure(&self) -> Float {
        self.predator_nodes.len() as Float * 0.2 + self.competitor_nodes.len() as Float * 0.1
            - self.mutualist_nodes.len() as Float * 0.1
    }

    pub fn total_support(&self) -> Float {
        self.prey_nodes.len() as Float * 0.3 + self.mutualist_nodes.len() as Float * 0.2
    }

    pub fn net_coupling(&self) -> Float {
        self.total_support() - self.total_trophic_pressure()
    }
}

#[derive(Debug, Clone)]
pub struct TrophicNetwork {
    pub nodes: HashMap<TrophicNodeId, TrophicNode>,
    pub links: HashMap<TrophicLinkId, TrophicLink>,
    pub couplings: HashMap<TrophicNodeId, TrophicCoupling>,
    pub total_biomass: Float,
    pub network_coherence: Float,
    pub level_biomasses: [Float; 6],
}

impl TrophicNetwork {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            couplings: HashMap::new(),
            total_biomass: 0.0,
            network_coherence: 0.8,
            level_biomasses: [0.0; 6],
        }
    }

    pub fn add_node(&mut self, node: TrophicNode) -> TrophicNodeId {
        let id = node.id;
        let level_idx = node.level as usize;
        self.level_biomasses[level_idx] += node.biomass;
        self.total_biomass += node.biomass;

        let coupling = TrophicCoupling::new(node.clone());
        self.couplings.insert(id, coupling);
        self.nodes.insert(id, node);

        id
    }

    pub fn add_link(&mut self, link: TrophicLink) -> TrophicLinkId {
        let id = link.id;
        let source_id = link.source_id;
        let target_id = link.target_id;

        if let Some(source) = self.nodes.get_mut(&source_id) {
            source.outgoing_links.push(id);
        }
        if let Some(target) = self.nodes.get_mut(&target_id) {
            target.incoming_links.push(id);
        }

        if let Some(target_coupling) = self.couplings.get_mut(&target_id) {
            match link.interaction_type {
                SpeciesInteraction::Predation => {
                    target_coupling.add_prey(source_id);
                }
                SpeciesInteraction::Mutualism => {
                    target_coupling.add_mutualist(source_id);
                }
                SpeciesInteraction::Competition => {
                    target_coupling.add_competitor(source_id);
                }
                _ => {}
            }
        }

        if let Some(source_coupling) = self.couplings.get_mut(&source_id) {
            match link.interaction_type {
                SpeciesInteraction::Predation => {
                    source_coupling.add_predator(target_id);
                }
                SpeciesInteraction::Mutualism => {
                    source_coupling.add_mutualist(target_id);
                }
                SpeciesInteraction::Competition => {
                    source_coupling.add_competitor(target_id);
                }
                _ => {}
            }
        }

        self.links.insert(id, link);
        id
    }

    pub fn calculate_energy_flow(&self) -> Vec<EnergyFlow> {
        let mut flows = Vec::new();

        for link in self.links.values() {
            if let Some(source) = self.nodes.get(&link.source_id) {
                let amount = link.energy_flow(source.biomass);
                flows.push(EnergyFlow::new(link.source_id, link.target_id, amount));
            }
        }

        flows
    }

    pub fn update(&mut self, dt: Float) {
        let energy_flows = self.calculate_energy_flow();

        let mut biomass_changes: HashMap<TrophicNodeId, Float> = HashMap::new();
        for node_id in self.nodes.keys() {
            biomass_changes.insert(*node_id, 0.0);
        }

        for flow in &energy_flows {
            if flow.amount > 0.0 {
                if let Some(source_change) = biomass_changes.get_mut(&flow.source_id) {
                    *source_change -= flow.amount;
                }
                if let Some(target_change) = biomass_changes.get_mut(&flow.target_id) {
                    *target_change += flow.net_energy();
                }
            }
        }

        for (id, change) in biomass_changes {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.biomass = (node.biomass + change * dt).max(0.001);
                node.update(dt);
            }
        }

        self.total_biomass = self.nodes.values().map(|n| n.biomass).sum();

        for level in 0..6 {
            self.level_biomasses[level] = self
                .nodes
                .values()
                .filter(|n| n.level as usize == level)
                .map(|n| n.biomass)
                .sum();
        }

        self.network_coherence = self.nodes.values().map(|n| n.coherence).sum::<Float>()
            / self.nodes.len().max(1) as Float;
    }

    pub fn stability_index(&self) -> Float {
        if self.nodes.is_empty() {
            return 1.0;
        }

        let biomass_variance = self.calculate_biomass_variance();
        let connectivity = self.connectivity();

        let stability = 1.0 - (biomass_variance * 0.5).min(0.9) + connectivity * 0.1;

        stability.clamp(0.0, 1.0)
    }

    fn calculate_biomass_variance(&self) -> Float {
        if self.nodes.is_empty() {
            return 0.0;
        }

        let mean = self.total_biomass / self.nodes.len() as Float;
        let variance: Float = self
            .nodes
            .values()
            .map(|n| (n.biomass - mean).powi(2))
            .sum::<Float>()
            / self.nodes.len() as Float;

        variance / (mean.powi(2) + 0.001)
    }

    fn connectivity(&self) -> Float {
        if self.nodes.len() < 2 {
            return 1.0;
        }

        let max_links = self.nodes.len() * (self.nodes.len() - 1);
        self.links.len() as Float / max_links as Float
    }

    pub fn trophic_efficiency(&self) -> Float {
        let mut total_efficiency = 0.0;
        let mut count = 0;

        for level in 1..6 {
            if self.level_biomasses[level] > 0.0 && self.level_biomasses[level - 1] > 0.0 {
                total_efficiency += self.level_biomasses[level] / self.level_biomasses[level - 1];
                count += 1;
            }
        }

        if count > 0 {
            total_efficiency / count as Float
        } else {
            0.1
        }
    }
}

impl Default for TrophicNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trophic_level_efficiency() {
        assert_eq!(TrophicLevel::Level0.efficiency(), 1.0);
        assert!(TrophicLevel::Level1.efficiency() < 1.0);
    }

    #[test]
    fn test_trophic_level_coherence_threshold() {
        assert!(
            TrophicLevel::Apex.field_coherence_threshold()
                > TrophicLevel::Level0.field_coherence_threshold()
        );
    }

    #[test]
    fn test_trophic_node_creation() {
        let node = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level1, 100.0);
        assert_eq!(node.level, TrophicLevel::Level1);
        assert!(node.biomass > 0.0);
    }

    #[test]
    fn test_trophic_node_available_energy() {
        let node = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level1, 100.0);
        let energy = node.available_energy();
        assert!(energy > 0.0);
        assert!(energy < node.biomass);
    }

    #[test]
    fn test_trophic_link_creation() {
        let link = TrophicLink::new(
            TrophicNodeId::new(1),
            TrophicNodeId::new(2),
            SpeciesInteraction::Predation,
            0.8,
        );
        assert!(link.is_predation());
        assert!(!link.is_mutualism());
    }

    #[test]
    fn test_trophic_link_energy_flow() {
        let link = TrophicLink::new(
            TrophicNodeId::new(1),
            TrophicNodeId::new(2),
            SpeciesInteraction::Predation,
            0.8,
        );
        let flow = link.energy_flow(100.0);
        assert!(flow > 0.0);
    }

    #[test]
    fn test_trophic_coupling_creation() {
        let node = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level1, 100.0);
        let coupling = TrophicCoupling::new(node);
        assert!(coupling.coupling_strength > 0.0);
    }

    #[test]
    fn test_trophic_coupling_pressure_support() {
        let node = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level1, 100.0);
        let mut coupling = TrophicCoupling::new(node);
        coupling.add_predator(TrophicNodeId::new(2));
        coupling.add_prey(TrophicNodeId::new(3));

        assert!(coupling.total_trophic_pressure() > 0.0);
        assert!(coupling.total_support() > 0.0);
    }

    #[test]
    fn test_trophic_network_creation() {
        let network = TrophicNetwork::new();
        assert!(network.nodes.is_empty());
        assert!(network.links.is_empty());
    }

    #[test]
    fn test_trophic_network_add_node() {
        let mut network = TrophicNetwork::new();
        let node = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level0, 1000.0);
        network.add_node(node);

        assert_eq!(network.nodes.len(), 1);
        assert!(network.total_biomass > 0.0);
    }

    #[test]
    fn test_trophic_network_add_link() {
        let mut network = TrophicNetwork::new();

        let node1 = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level0, 1000.0);
        let id1 = network.add_node(node1);

        let node2 = TrophicNode::new(SpeciesId::new(2), TrophicLevel::Level1, 100.0);
        let id2 = network.add_node(node2);

        let link = TrophicLink::new(id1, id2, SpeciesInteraction::Predation, 0.8);
        network.add_link(link);

        assert_eq!(network.links.len(), 1);
    }

    #[test]
    fn test_trophic_network_energy_flow() {
        let mut network = TrophicNetwork::new();

        let node1 = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level0, 1000.0);
        let id1 = network.add_node(node1);

        let node2 = TrophicNode::new(SpeciesId::new(2), TrophicLevel::Level1, 100.0);
        let id2 = network.add_node(node2);

        let link = TrophicLink::new(id1, id2, SpeciesInteraction::Predation, 0.8);
        network.add_link(link);

        let flows = network.calculate_energy_flow();
        assert!(!flows.is_empty());
    }

    #[test]
    fn test_trophic_network_stability() {
        let mut network = TrophicNetwork::new();

        let node1 = TrophicNode::new(SpeciesId::new(1), TrophicLevel::Level0, 1000.0);
        network.add_node(node1);

        let stability = network.stability_index();
        assert!((0.0..=1.0).contains(&stability));
    }
}
