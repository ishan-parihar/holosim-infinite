//! Resource Extraction with Regeneration
//!
//! From HOLOSIM_INFINITE_COMPLETE_ROADMAP.md:
//! "Resource Extraction = Field amplitude modification at location"
//!
//! # Key Insight
//!
//! Resources are NOT independent objects but field amplitude concentrations.
//! Extraction modifies the field at that location, and regeneration is the
//! field naturally re-establishing its amplitude pattern.

use crate::types::Float;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Organic,
    Mineral,
    Energetic,
    Consciousness,
    Water,
    Air,
    Light,
    Essence,
    Void,
    Harmonic,
}

impl ResourceType {
    pub fn base_regeneration_rate(&self) -> Float {
        match self {
            ResourceType::Light => 0.5,
            ResourceType::Air => 0.4,
            ResourceType::Water => 0.2,
            ResourceType::Organic => 0.15,
            ResourceType::Energetic => 0.1,
            ResourceType::Consciousness => 0.08,
            ResourceType::Mineral => 0.02,
            ResourceType::Essence => 0.05,
            ResourceType::Void => 0.01,
            ResourceType::Harmonic => 0.03,
        }
    }

    pub fn field_coherence_cost(&self) -> Float {
        match self {
            ResourceType::Light => 0.01,
            ResourceType::Air => 0.02,
            ResourceType::Water => 0.05,
            ResourceType::Organic => 0.08,
            ResourceType::Energetic => 0.12,
            ResourceType::Consciousness => 0.15,
            ResourceType::Mineral => 0.2,
            ResourceType::Essence => 0.25,
            ResourceType::Void => 0.3,
            ResourceType::Harmonic => 0.1,
        }
    }

    pub fn max_concentration(&self) -> Float {
        match self {
            ResourceType::Light => 1000.0,
            ResourceType::Air => 800.0,
            ResourceType::Water => 500.0,
            ResourceType::Organic => 300.0,
            ResourceType::Energetic => 200.0,
            ResourceType::Consciousness => 150.0,
            ResourceType::Mineral => 400.0,
            ResourceType::Essence => 50.0,
            ResourceType::Void => 30.0,
            ResourceType::Harmonic => 100.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ResourceType::Organic => "Living matter derived from field condensation",
            ResourceType::Mineral => "Crystallized field patterns in solid form",
            ResourceType::Energetic => "Raw field energy in concentrated form",
            ResourceType::Consciousness => "Crystallized awareness from sentient beings",
            ResourceType::Water => "Liquid field manifestation of flow",
            ResourceType::Air => "Gaseous field manifestation of movement",
            ResourceType::Light => "Radiant field manifestation of illumination",
            ResourceType::Essence => "Pure consciousness field extract",
            ResourceType::Void => "Absence that creates presence",
            ResourceType::Harmonic => "Resonant field frequency patterns",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub concentration: Float,
    pub field_amplitude: Float,
    pub extraction_rate: Float,
    pub regeneration_rate: Float,
    pub depletion_level: Float,
    pub location_x: Float,
    pub location_y: Float,
    pub location_z: Float,
}

impl ResourceNode {
    pub fn new(resource_type: ResourceType, x: Float, y: Float, z: Float) -> Self {
        let concentration = resource_type.max_concentration() * 0.5;

        Self {
            resource_type,
            concentration,
            field_amplitude: concentration.sqrt(),
            extraction_rate: 0.1,
            regeneration_rate: resource_type.base_regeneration_rate(),
            depletion_level: 0.0,
            location_x: x,
            location_y: y,
            location_z: z,
        }
    }

    pub fn with_concentration(mut self, concentration: Float) -> Self {
        self.concentration = concentration.min(self.resource_type.max_concentration());
        self.field_amplitude = self.concentration.sqrt();
        self
    }

    pub fn update(&mut self, dt: Float) {
        if self.depletion_level > 0.0 {
            let recovery = self.regeneration_rate * dt * (1.0 - self.depletion_level * 0.5);
            self.depletion_level = (self.depletion_level - recovery).max(0.0);
        }

        if self.concentration < self.resource_type.max_concentration() {
            let regen = self.regeneration_rate * (1.0 - self.depletion_level) * dt;
            self.concentration =
                (self.concentration + regen).min(self.resource_type.max_concentration());
            self.field_amplitude = self.concentration.sqrt();
        }
    }

    pub fn extract(&mut self, amount: Float) -> Float {
        let available = self.concentration * (1.0 - self.depletion_level * 0.5);
        let extracted = amount.min(available);

        self.concentration -= extracted;
        self.field_amplitude = self.concentration.sqrt();
        self.depletion_level =
            (self.depletion_level + extracted / self.resource_type.max_concentration()).min(1.0);

        extracted
    }

    pub fn remaining_capacity(&self) -> Float {
        self.concentration * (1.0 - self.depletion_level * 0.5)
    }

    pub fn health_factor(&self) -> Float {
        1.0 - self.depletion_level
    }

    pub fn distance_to(&self, x: Float, y: Float, z: Float) -> Float {
        let dx = self.location_x - x;
        let dy = self.location_y - y;
        let dz = self.location_z - z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct ResourcePool {
    pub resources: HashMap<ResourceType, Float>,
    pub capacity: HashMap<ResourceType, Float>,
    pub decay_rates: HashMap<ResourceType, Float>,
}

impl ResourcePool {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            capacity: HashMap::new(),
            decay_rates: HashMap::new(),
        }
    }

    pub fn add_resource_type(&mut self, resource_type: ResourceType, capacity: Float) {
        self.resources.insert(resource_type, 0.0);
        self.capacity.insert(resource_type, capacity);
        self.decay_rates.insert(resource_type, 0.001);
    }

    pub fn deposit(&mut self, resource_type: ResourceType, amount: Float) -> Float {
        let current = self.resources.get(&resource_type).copied().unwrap_or(0.0);
        let capacity = self.capacity.get(&resource_type).copied().unwrap_or(0.0);
        let new_amount = (current + amount).min(capacity);
        let deposited = new_amount - current;

        self.resources.insert(resource_type, new_amount);
        deposited
    }

    pub fn withdraw(&mut self, resource_type: ResourceType, amount: Float) -> Float {
        let current = self.resources.get(&resource_type).copied().unwrap_or(0.0);
        let withdrawn = amount.min(current);
        self.resources.insert(resource_type, current - withdrawn);
        withdrawn
    }

    pub fn update(&mut self, dt: Float) {
        for (resource_type, amount) in self.resources.iter_mut() {
            if let Some(&decay_rate) = self.decay_rates.get(resource_type) {
                *amount = (*amount * (1.0 - decay_rate * dt)).max(0.0);
            }
        }
    }

    pub fn total_value(&self) -> Float {
        self.resources.values().sum()
    }

    pub fn fill_percentage(&self, resource_type: ResourceType) -> Float {
        let current = self.resources.get(&resource_type).copied().unwrap_or(0.0);
        let capacity = self.capacity.get(&resource_type).copied().unwrap_or(1.0);
        current / capacity
    }

    pub fn is_depleted(&self, resource_type: ResourceType) -> bool {
        self.resources.get(&resource_type).copied().unwrap_or(0.0) < 0.01
    }
}

impl Default for ResourcePool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ResourceAvailability {
    pub location_x: Float,
    pub location_y: Float,
    pub location_z: Float,
    pub nearby_nodes: Vec<(usize, Float)>,
    pub total_availability: HashMap<ResourceType, Float>,
    pub extraction_difficulty: HashMap<ResourceType, Float>,
}

impl ResourceAvailability {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            location_x: x,
            location_y: y,
            location_z: z,
            nearby_nodes: Vec::new(),
            total_availability: HashMap::new(),
            extraction_difficulty: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node_index: usize, distance: Float) {
        self.nearby_nodes.push((node_index, distance));
    }

    pub fn calculate_availability(&mut self, nodes: &[ResourceNode], extraction_radius: Float) {
        self.total_availability.clear();
        self.extraction_difficulty.clear();

        for &(node_idx, distance) in &self.nearby_nodes {
            if distance > extraction_radius {
                continue;
            }

            let node = &nodes[node_idx];
            let availability = node.remaining_capacity() * (1.0 - distance / extraction_radius);

            *self
                .total_availability
                .entry(node.resource_type)
                .or_insert(0.0) += availability;

            let difficulty = 0.5 + distance / extraction_radius * 0.5;
            let current_difficulty = self
                .extraction_difficulty
                .entry(node.resource_type)
                .or_insert(1.0);
            *current_difficulty = (*current_difficulty + difficulty) * 0.5;
        }
    }

    pub fn best_resource_type(&self) -> Option<ResourceType> {
        self.total_availability
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, _)| *k)
    }

    pub fn availability_of(&self, resource_type: ResourceType) -> Float {
        self.total_availability
            .get(&resource_type)
            .copied()
            .unwrap_or(0.0)
    }
}

#[derive(Debug, Clone)]
pub struct ExtractionImpact {
    pub resource_type: ResourceType,
    pub amount_extracted: Float,
    pub field_disruption: Float,
    pub coherence_cost: Float,
    pub regeneration_time: Float,
    pub nearby_entity_impact: Float,
}

impl ExtractionImpact {
    pub fn new(resource_type: ResourceType, amount: Float, node: &ResourceNode) -> Self {
        let coherence_cost = resource_type.field_coherence_cost() * amount;
        let field_disruption = amount / node.resource_type.max_concentration();
        let regeneration_time = amount / resource_type.base_regeneration_rate();

        Self {
            resource_type,
            amount_extracted: amount,
            field_disruption,
            coherence_cost,
            regeneration_time,
            nearby_entity_impact: field_disruption * 0.1,
        }
    }

    pub fn total_impact(&self) -> Float {
        self.field_disruption * 0.3
            + self.coherence_cost * 0.3
            + self.nearby_entity_impact * 0.2
            + (self.regeneration_time / 100.0).min(1.0) * 0.2
    }
}

#[derive(Debug, Clone)]
pub struct ResourceExtraction {
    pub entity_id: u64,
    pub resource_pool: ResourcePool,
    pub extraction_efficiency: Float,
    pub extraction_radius: Float,
    pub sustainable_mode: bool,
    pub extraction_history: Vec<ExtractionImpact>,
    pub total_extracted: HashMap<ResourceType, Float>,
}

impl ResourceExtraction {
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            resource_pool: ResourcePool::new(),
            extraction_efficiency: 0.5,
            extraction_radius: 50.0,
            sustainable_mode: true,
            extraction_history: Vec::new(),
            total_extracted: HashMap::new(),
        }
    }

    pub fn with_efficiency(mut self, efficiency: Float) -> Self {
        self.extraction_efficiency = efficiency.clamp(0.1, 1.0);
        self
    }

    pub fn with_radius(mut self, radius: Float) -> Self {
        self.extraction_radius = radius;
        self
    }

    pub fn extract_from_node(
        &mut self,
        node: &mut ResourceNode,
        desired_amount: Float,
    ) -> ExtractionImpact {
        let max_sustainable = if self.sustainable_mode {
            node.concentration * 0.1
        } else {
            desired_amount
        };

        let amount_to_extract = desired_amount.min(max_sustainable) * self.extraction_efficiency;
        let extracted = node.extract(amount_to_extract);

        let impact = ExtractionImpact::new(node.resource_type, extracted, node);
        self.extraction_history.push(impact.clone());

        self.resource_pool.deposit(node.resource_type, extracted);
        *self
            .total_extracted
            .entry(node.resource_type)
            .or_insert(0.0) += extracted;

        impact
    }

    pub fn update(&mut self, dt: Float) {
        self.resource_pool.update(dt);

        if self.sustainable_mode {
            let total_impact: Float = self
                .extraction_history
                .iter()
                .rev()
                .take(10)
                .map(|i| i.total_impact())
                .sum::<Float>()
                / 10.0;

            if total_impact > 0.5 {
                self.extraction_efficiency *= 0.99;
            } else {
                self.extraction_efficiency = (self.extraction_efficiency * 1.001).min(1.0);
            }
        }
    }

    pub fn toggle_sustainable_mode(&mut self) {
        self.sustainable_mode = !self.sustainable_mode;
    }

    pub fn total_resources_extracted(&self) -> Float {
        self.total_extracted.values().sum()
    }

    pub fn environmental_impact(&self) -> Float {
        if self.extraction_history.is_empty() {
            return 0.0;
        }

        let history_len = self.extraction_history.len() as Float;
        self.extraction_history
            .iter()
            .rev()
            .take(20)
            .map(|i| i.total_impact())
            .sum::<Float>()
            / 20.0_f64.min(history_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_type_regeneration_rates() {
        assert!(
            ResourceType::Light.base_regeneration_rate()
                > ResourceType::Mineral.base_regeneration_rate()
        );
    }

    #[test]
    fn test_resource_type_coherence_cost() {
        assert!(
            ResourceType::Light.field_coherence_cost() < ResourceType::Void.field_coherence_cost()
        );
    }

    #[test]
    fn test_resource_node_creation() {
        let node = ResourceNode::new(ResourceType::Water, 10.0, 20.0, 30.0);
        assert_eq!(node.resource_type, ResourceType::Water);
        assert!(node.concentration > 0.0);
    }

    #[test]
    fn test_resource_node_update() {
        let mut node = ResourceNode::new(ResourceType::Organic, 0.0, 0.0, 0.0);
        node.depletion_level = 0.5;
        node.update(1.0);
        assert!(node.depletion_level < 0.5);
    }

    #[test]
    fn test_resource_node_extract() {
        let mut node =
            ResourceNode::new(ResourceType::Water, 0.0, 0.0, 0.0).with_concentration(100.0);
        let extracted = node.extract(50.0);
        assert!(extracted > 0.0);
        assert!(node.concentration < 100.0);
    }

    #[test]
    fn test_resource_node_remaining_capacity() {
        let node = ResourceNode::new(ResourceType::Water, 0.0, 0.0, 0.0).with_concentration(100.0);
        assert!(node.remaining_capacity() > 0.0);
    }

    #[test]
    fn test_resource_pool_creation() {
        let mut pool = ResourcePool::new();
        pool.add_resource_type(ResourceType::Water, 100.0);
        assert!(!pool.resources.is_empty());
    }

    #[test]
    fn test_resource_pool_deposit() {
        let mut pool = ResourcePool::new();
        pool.add_resource_type(ResourceType::Water, 100.0);
        let deposited = pool.deposit(ResourceType::Water, 50.0);
        assert_eq!(deposited, 50.0);
    }

    #[test]
    fn test_resource_pool_withdraw() {
        let mut pool = ResourcePool::new();
        pool.add_resource_type(ResourceType::Water, 100.0);
        pool.deposit(ResourceType::Water, 50.0);
        let withdrawn = pool.withdraw(ResourceType::Water, 30.0);
        assert_eq!(withdrawn, 30.0);
    }

    #[test]
    fn test_resource_pool_capacity_limit() {
        let mut pool = ResourcePool::new();
        pool.add_resource_type(ResourceType::Water, 100.0);
        let deposited = pool.deposit(ResourceType::Water, 150.0);
        assert_eq!(deposited, 100.0);
    }

    #[test]
    fn test_resource_availability_creation() {
        let availability = ResourceAvailability::new(0.0, 0.0, 0.0);
        assert!(availability.nearby_nodes.is_empty());
    }

    #[test]
    fn test_resource_availability_add_node() {
        let mut availability = ResourceAvailability::new(0.0, 0.0, 0.0);
        availability.add_node(0, 10.0);
        assert_eq!(availability.nearby_nodes.len(), 1);
    }

    #[test]
    fn test_extraction_impact_creation() {
        let node = ResourceNode::new(ResourceType::Water, 0.0, 0.0, 0.0);
        let impact = ExtractionImpact::new(ResourceType::Water, 10.0, &node);
        assert!(impact.amount_extracted > 0.0);
    }

    #[test]
    fn test_resource_extraction_creation() {
        let extraction = ResourceExtraction::new(1);
        assert_eq!(extraction.entity_id, 1);
    }

    #[test]
    fn test_resource_extraction_extract_from_node() {
        let mut extraction = ResourceExtraction::new(1);
        let mut node =
            ResourceNode::new(ResourceType::Water, 0.0, 0.0, 0.0).with_concentration(100.0);

        let impact = extraction.extract_from_node(&mut node, 10.0);
        assert!(impact.amount_extracted > 0.0);
        assert!(!extraction.extraction_history.is_empty());
    }

    #[test]
    fn test_resource_extraction_sustainable_mode() {
        let mut extraction = ResourceExtraction::new(1);
        assert!(extraction.sustainable_mode);
        extraction.toggle_sustainable_mode();
        assert!(!extraction.sustainable_mode);
    }

    #[test]
    fn test_resource_extraction_update() {
        let mut extraction = ResourceExtraction::new(1);
        extraction.update(1.0);
        assert!(extraction.extraction_efficiency > 0.0);
    }

    #[test]
    fn test_resource_node_distance() {
        let node = ResourceNode::new(ResourceType::Water, 0.0, 0.0, 0.0);
        let distance = node.distance_to(3.0, 4.0, 0.0);
        assert!((distance - 5.0).abs() < 0.001);
    }
}
