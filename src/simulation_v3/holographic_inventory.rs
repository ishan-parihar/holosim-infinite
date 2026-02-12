//! Holographic Inventory System
//!
//! Implements inventory as holographic memory, not slot-based storage.
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 8:
//! > "Inventory as holographic memory - what you remember/resonate with"
//! > "Resonance capacity - carry what resonates with you"
//! > "Archetypical item signatures - each item has unique archetypical pattern"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.6:
//! > "Each step INCLUDES all previous development and TRANSCENDS by adding new development."
//!
//! Key Concepts:
//! - **Holographic Inventory**: Items stored as holographic memory, not physical slots
//! - **Resonance Capacity**: Capacity measured by resonance, not slot count
//! - **Archetypical Signatures**: Each item has unique 22-archetype pattern
//! - **Holographic Blueprints**: Items can be reconstructed from archetypical blueprints
//! - **Resonance-Based Carry**: Can only carry items that resonate with your soul stream

use crate::simulation_v3::{
    holographic_memory::{
        HolographicMemoryError, HolographicMemorySystem, HolographicSignature, MemoryEntry,
        MemoryKey, MemoryType, SoulStreamId,
    },
    holographic_physics::SpectrumRatio,
};
use crate::types::Float;
use std::collections::HashMap;

pub type ItemId = u64;

pub const DEFAULT_RESONANCE_CAPACITY: Float = 100.0;
pub const MIN_RESONANCE_COST: Float = 1.0;
pub const MAX_INVENTORY_SIZE: usize = 1000;
pub const RESONANCE_DECAY_RATE: Float = 0.001;
pub const RESONANCE_REGEN_RATE: Float = 0.01;

#[derive(Debug, Clone, PartialEq)]
pub enum InventoryError {
    ItemNotFound(ItemId),
    ResonanceCapacityExceeded {
        current: Float,
        required: Float,
        capacity: Float,
    },
    InvalidItemSignature,
    SoulStreamNotFound(SoulStreamId),
    MemoryError(HolographicMemoryError),
    BlueprintNotFound(ItemId),
    ResonanceInsufficient {
        required: Float,
        available: Float,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Tool,
    Weapon,
    Material,
    Component,
    Blueprint,
    Artifact,
    Catalyst,
    Essence,
    Consumable,
    Key,
}

#[derive(Debug, Clone)]
pub struct ArchetypicalItemSignature {
    pub archetype_pattern: [Float; 22],
    pub density_affinity: Float,
    pub polarity_bias: Float,
    pub resonance_frequency: Float,
}

impl PartialEq for ArchetypicalItemSignature {
    fn eq(&self, other: &Self) -> bool {
        self.density_affinity == other.density_affinity
            && self.polarity_bias == other.polarity_bias
            && self.resonance_frequency == other.resonance_frequency
            && self
                .archetype_pattern
                .iter()
                .zip(other.archetype_pattern.iter())
                .all(|(a, b)| a == b)
    }
}

impl ArchetypicalItemSignature {
    pub fn new() -> Self {
        Self {
            archetype_pattern: [0.0; 22],
            density_affinity: 0.5,
            polarity_bias: 0.0,
            resonance_frequency: 440.0,
        }
    }

    pub fn from_holographic_signature(signature: &HolographicSignature) -> Self {
        Self {
            archetype_pattern: signature.interference_pattern,
            density_affinity: signature.coherence_level,
            polarity_bias: (signature.resonance_frequency - 440.0) / 880.0,
            resonance_frequency: signature.resonance_frequency,
        }
    }

    pub fn compute_resonance_cost(&self) -> Float {
        let pattern_magnitude: Float = self.archetype_pattern.iter().map(|x| x.abs()).sum();
        let base_cost = pattern_magnitude / 22.0;
        let density_factor = self.density_affinity.abs();
        let resonance_factor = (self.resonance_frequency - 440.0).abs() / 880.0;
        (base_cost + density_factor + resonance_factor) * MIN_RESONANCE_COST
    }

    pub fn compute_resonance_with_soul_stream(
        &self,
        soul_signature: &HolographicSignature,
    ) -> Float {
        let mut pattern_alignment = 0.0;
        for i in 0..22 {
            pattern_alignment +=
                1.0 - (self.archetype_pattern[i] - soul_signature.interference_pattern[i]).abs();
        }
        pattern_alignment /= 22.0;
        pattern_alignment
            * (0.5 + self.density_affinity * 0.3 + (1.0 - self.polarity_bias.abs()) * 0.2)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResonancePattern {
    pub pattern: [Float; 8],
    pub stability: Float,
    pub phase: Float,
}

impl ResonancePattern {
    pub fn new() -> Self {
        Self {
            pattern: [0.0; 8],
            stability: 1.0,
            phase: 0.0,
        }
    }

    pub fn from_spectrum(spectrum: &SpectrumRatio) -> Self {
        let base_ratio =
            spectrum.space_time_ratio / (spectrum.space_time_ratio + spectrum.time_space_ratio);
        let mut pattern = [0.0; 8];
        for i in 0..8 {
            pattern[i] = base_ratio * (1.0 + (i as Float / 8.0) * 0.5).min(1.0);
        }
        Self {
            pattern,
            stability: 0.8 + base_ratio * 0.2,
            phase: base_ratio * std::f64::consts::PI * 2.0,
        }
    }

    pub fn compute_interference(&self, other: &ResonancePattern) -> Float {
        let mut interference = 0.0;
        for i in 0..8 {
            interference += (self.pattern[i] - other.pattern[i]).abs();
        }
        interference /= 8.0;
        1.0 - interference
    }
}

#[derive(Debug, Clone)]
pub struct HolographicItem {
    pub item_id: ItemId,
    pub name: String,
    pub category: ItemCategory,
    pub archetype_signature: ArchetypicalItemSignature,
    pub resonance_pattern: ResonancePattern,
    pub holographic_blueprint: Option<HolographicItemBlueprint>,
    pub resonance_cost: Float,
    pub quantity: u64,
    pub memory_key: Option<MemoryKey>,
}

impl HolographicItem {
    pub fn new(item_id: ItemId, name: String, category: ItemCategory) -> Self {
        let archetype_signature = ArchetypicalItemSignature::new();
        let resonance_cost = archetype_signature.compute_resonance_cost();
        Self {
            item_id,
            name,
            category,
            archetype_signature,
            resonance_pattern: ResonancePattern::new(),
            holographic_blueprint: None,
            resonance_cost,
            quantity: 1,
            memory_key: None,
        }
    }

    pub fn with_archetype_signature(mut self, signature: ArchetypicalItemSignature) -> Self {
        let resonance_cost = signature.compute_resonance_cost();
        self.archetype_signature = signature;
        self.resonance_cost = resonance_cost;
        self
    }

    pub fn with_resonance_pattern(mut self, pattern: ResonancePattern) -> Self {
        self.resonance_pattern = pattern;
        self
    }

    pub fn with_blueprint(mut self, blueprint: HolographicItemBlueprint) -> Self {
        self.holographic_blueprint = Some(blueprint);
        self
    }

    pub fn with_quantity(mut self, quantity: u64) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn total_resonance_cost(&self) -> Float {
        self.resonance_cost * self.quantity as Float
    }
}

#[derive(Debug, Clone)]
pub struct HolographicItemBlueprint {
    pub blueprint_id: ItemId,
    pub base_signature: ArchetypicalItemSignature,
    pub material_components: Vec<ItemId>,
    pub archetypical_components: Vec<[Float; 22]>,
    pub craft_difficulty: Float,
    pub required_density_level: Float,
}

impl HolographicItemBlueprint {
    pub fn new(blueprint_id: ItemId, base_signature: ArchetypicalItemSignature) -> Self {
        Self {
            blueprint_id,
            base_signature,
            material_components: Vec::new(),
            archetypical_components: Vec::new(),
            craft_difficulty: 0.5,
            required_density_level: 0.0,
        }
    }

    pub fn compute_crafting_resonance(&self) -> Float {
        let base_resonance = self.base_signature.compute_resonance_cost();
        let component_resonance: Float = self
            .archetypical_components
            .iter()
            .map(|comp| comp.iter().map(|x| x.abs()).sum::<Float>() / 22.0)
            .sum();
        base_resonance + component_resonance * self.craft_difficulty
    }
}

#[derive(Debug, Clone)]
pub struct HolographicInventory {
    pub inventory_id: u64,
    pub soul_stream_id: SoulStreamId,
    pub items: HashMap<ItemId, HolographicItem>,
    pub resonance_capacity: Float,
    pub current_resonance_usage: Float,
    pub available_resonance: Float,
    pub inventory_signature: HolographicSignature,
    pub max_items: usize,
}

impl HolographicInventory {
    pub fn new(inventory_id: u64, soul_stream_id: SoulStreamId) -> Self {
        Self {
            inventory_id,
            soul_stream_id,
            items: HashMap::new(),
            resonance_capacity: DEFAULT_RESONANCE_CAPACITY,
            current_resonance_usage: 0.0,
            available_resonance: DEFAULT_RESONANCE_CAPACITY,
            inventory_signature: HolographicSignature::new(),
            max_items: MAX_INVENTORY_SIZE,
        }
    }

    pub fn with_capacity(mut self, capacity: Float) -> Self {
        self.resonance_capacity = capacity;
        self.available_resonance = capacity - self.current_resonance_usage;
        self
    }

    pub fn with_max_items(mut self, max_items: usize) -> Self {
        self.max_items = max_items;
        self
    }

    pub fn add_item(
        &mut self,
        item: HolographicItem,
        memory_system: &mut HolographicMemorySystem,
    ) -> Result<(), InventoryError> {
        if self.items.len() >= self.max_items {
            return Err(InventoryError::ResonanceCapacityExceeded {
                current: self.current_resonance_usage,
                required: item.total_resonance_cost(),
                capacity: self.resonance_capacity,
            });
        }

        let total_cost = item.total_resonance_cost();
        if self.available_resonance < total_cost {
            return Err(InventoryError::ResonanceCapacityExceeded {
                current: self.current_resonance_usage,
                required: total_cost,
                capacity: self.resonance_capacity,
            });
        }

        let memory_entry = Self::create_item_memory_entry(&item)?;
        let memory_key = memory_system
            .store_memory(self.soul_stream_id, memory_entry)
            .map_err(InventoryError::MemoryError)?;

        let mut item = item;
        item.memory_key = Some(memory_key);
        self.items.insert(item.item_id, item);
        self.current_resonance_usage += total_cost;
        self.available_resonance = self.resonance_capacity - self.current_resonance_usage;
        self.update_inventory_signature(memory_system);

        Ok(())
    }

    pub fn remove_item(&mut self, item_id: ItemId) -> Result<HolographicItem, InventoryError> {
        let item = self
            .items
            .remove(&item_id)
            .ok_or(InventoryError::ItemNotFound(item_id))?;

        let resonance_released = item.total_resonance_cost();
        self.current_resonance_usage = (self.current_resonance_usage - resonance_released).max(0.0);
        self.available_resonance = self.resonance_capacity - self.current_resonance_usage;

        Ok(item)
    }

    pub fn get_item(&self, item_id: ItemId) -> Option<&HolographicItem> {
        self.items.get(&item_id)
    }

    pub fn get_item_mut(&mut self, item_id: ItemId) -> Option<&mut HolographicItem> {
        self.items.get_mut(&item_id)
    }

    pub fn has_item(&self, item_id: ItemId) -> bool {
        self.items.contains_key(&item_id)
    }

    pub fn query_items_by_resonance(&self, min_resonance: Float) -> Vec<&HolographicItem> {
        self.items
            .values()
            .filter(|item| item.resonance_cost >= min_resonance)
            .collect()
    }

    pub fn query_items_by_category(&self, category: ItemCategory) -> Vec<&HolographicItem> {
        self.items
            .values()
            .filter(|item| item.category == category)
            .collect()
    }

    pub fn query_items_by_signature(
        &self,
        reference_signature: &ArchetypicalItemSignature,
        min_similarity: Float,
    ) -> Vec<&HolographicItem> {
        self.items
            .values()
            .filter(|item| {
                let similarity = item.archetype_signature.compute_resonance_with_soul_stream(
                    &HolographicSignature {
                        interference_pattern: reference_signature.archetype_pattern,
                        coherence_level: reference_signature.density_affinity,
                        entropy_level: 0.5,
                        resonance_frequency: reference_signature.resonance_frequency,
                    },
                );
                similarity >= min_similarity
            })
            .collect()
    }

    pub fn compute_total_resonance_usage(&self) -> Float {
        self.items
            .values()
            .map(|item| item.total_resonance_cost())
            .sum()
    }

    pub fn compute_resonance_efficiency(&self) -> Float {
        if self.resonance_capacity == 0.0 {
            return 0.0;
        }
        self.current_resonance_usage / self.resonance_capacity
    }

    pub fn regenerate_resonance(&mut self, delta_time: Float) {
        let regen = delta_time * RESONANCE_REGEN_RATE * self.resonance_capacity;
        self.current_resonance_usage = (self.current_resonance_usage - regen).max(0.0);
        self.available_resonance = self.resonance_capacity - self.current_resonance_usage;
    }

    pub fn decay_resonance(&mut self, delta_time: Float) {
        let decay = delta_time * RESONANCE_DECAY_RATE * self.current_resonance_usage;
        self.current_resonance_usage = (self.current_resonance_usage - decay).max(0.0);
        self.available_resonance = self.resonance_capacity - self.current_resonance_usage;
    }

    pub fn create_item_memory_entry(item: &HolographicItem) -> Result<MemoryEntry, InventoryError> {
        let signature = HolographicSignature {
            interference_pattern: item.archetype_signature.archetype_pattern,
            coherence_level: item.archetype_signature.density_affinity,
            entropy_level: 0.5,
            resonance_frequency: item.archetype_signature.resonance_frequency,
        };

        Ok(MemoryEntry::new(
            item.item_id,
            crate::types::Density::Third,
            MemoryType::Blueprint,
            signature,
        ))
    }

    pub fn update_inventory_signature(&mut self, memory_system: &HolographicMemorySystem) {
        if let Some(stream) = memory_system.get_soul_stream(self.soul_stream_id) {
            self.inventory_signature = stream.holographic_signature.clone();
        }
    }

    pub fn get_statistics(&self) -> InventoryStatistics {
        let item_count = self.items.len();
        let total_quantity: u64 = self.items.values().map(|item| item.quantity).sum();
        let category_counts: HashMap<ItemCategory, usize> = {
            let mut counts = HashMap::new();
            for item in self.items.values() {
                *counts.entry(item.category).or_insert(0) += 1;
            }
            counts
        };

        InventoryStatistics {
            inventory_id: self.inventory_id,
            soul_stream_id: self.soul_stream_id,
            item_count,
            total_quantity,
            resonance_capacity: self.resonance_capacity,
            current_resonance_usage: self.current_resonance_usage,
            available_resonance: self.available_resonance,
            resonance_efficiency: self.compute_resonance_efficiency(),
            category_counts,
        }
    }
}

impl Default for HolographicInventory {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug, Clone)]
pub struct InventoryStatistics {
    pub inventory_id: u64,
    pub soul_stream_id: SoulStreamId,
    pub item_count: usize,
    pub total_quantity: u64,
    pub resonance_capacity: Float,
    pub current_resonance_usage: Float,
    pub available_resonance: Float,
    pub resonance_efficiency: Float,
    pub category_counts: HashMap<ItemCategory, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holographic_inventory_creation() {
        let inventory = HolographicInventory::new(1, 100);
        assert_eq!(inventory.inventory_id, 1);
        assert_eq!(inventory.soul_stream_id, 100);
        assert_eq!(inventory.items.len(), 0);
        assert_eq!(inventory.resonance_capacity, DEFAULT_RESONANCE_CAPACITY);
    }

    #[test]
    fn test_holographic_item_creation() {
        let item = HolographicItem::new(1, "Test Item".to_string(), ItemCategory::Tool);
        assert_eq!(item.item_id, 1);
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.category, ItemCategory::Tool);
        assert_eq!(item.quantity, 1);
    }

    #[test]
    fn test_archetypical_item_signature_creation() {
        let signature = ArchetypicalItemSignature::new();
        assert_eq!(signature.density_affinity, 0.5);
        assert_eq!(signature.polarity_bias, 0.0);
        assert_eq!(signature.resonance_frequency, 440.0);
    }

    #[test]
    fn test_archetypical_item_signature_resonance_cost() {
        let signature = ArchetypicalItemSignature::new();
        let cost = signature.compute_resonance_cost();
        assert!(cost >= MIN_RESONANCE_COST);
    }

    #[test]
    fn test_resonance_pattern_creation() {
        let pattern = ResonancePattern::new();
        assert_eq!(pattern.stability, 1.0);
        assert_eq!(pattern.phase, 0.0);
    }

    #[test]
    fn test_resonance_pattern_from_spectrum() {
        let spectrum = SpectrumRatio::new(1.0, 1.0);
        let pattern = ResonancePattern::from_spectrum(&spectrum);
        assert!(pattern.stability > 0.0);
        assert!(pattern.stability <= 1.0);
    }

    #[test]
    fn test_holographic_item_blueprint_creation() {
        let signature = ArchetypicalItemSignature::new();
        let blueprint = HolographicItemBlueprint::new(1, signature);
        assert_eq!(blueprint.blueprint_id, 1);
        assert_eq!(blueprint.craft_difficulty, 0.5);
    }

    #[test]
    fn test_holographic_item_with_builder() {
        let signature = ArchetypicalItemSignature::new();
        let pattern = ResonancePattern::new();
        let blueprint = HolographicItemBlueprint::new(1, signature.clone());

        let item = HolographicItem::new(1, "Builder Item".to_string(), ItemCategory::Weapon)
            .with_archetype_signature(signature)
            .with_resonance_pattern(pattern)
            .with_blueprint(blueprint)
            .with_quantity(5);

        assert_eq!(item.quantity, 5);
        assert!(item.holographic_blueprint.is_some());
    }

    #[test]
    fn test_inventory_with_capacity() {
        let inventory = HolographicInventory::new(1, 100).with_capacity(200.0);
        assert_eq!(inventory.resonance_capacity, 200.0);
        assert_eq!(inventory.available_resonance, 200.0);
    }

    #[test]
    fn test_holographic_item_total_resonance_cost() {
        let mut item = HolographicItem::new(1, "Test Item".to_string(), ItemCategory::Tool);
        item.resonance_cost = 5.0;
        item.quantity = 3;
        assert_eq!(item.total_resonance_cost(), 15.0);
    }

    #[test]
    fn test_resonance_pattern_interference() {
        let pattern1 = ResonancePattern::new();
        let mut pattern2 = ResonancePattern::new();
        pattern2.pattern[0] = 0.5;

        let interference = pattern1.compute_interference(&pattern2);
        assert!(interference >= 0.0 && interference <= 1.0);
    }

    #[test]
    fn test_inventory_statistics() {
        let inventory = HolographicInventory::new(1, 100);
        let stats = inventory.get_statistics();
        assert_eq!(stats.inventory_id, 1);
        assert_eq!(stats.soul_stream_id, 100);
        assert_eq!(stats.item_count, 0);
    }

    #[test]
    fn test_resonance_regeneration() {
        let mut inventory = HolographicInventory::new(1, 100).with_capacity(100.0);
        inventory.current_resonance_usage = 50.0;
        inventory.available_resonance = 50.0;

        inventory.regenerate_resonance(1.0);
        assert!(inventory.current_resonance_usage < 50.0);
        assert!(inventory.available_resonance > 50.0);
    }

    #[test]
    fn test_resonance_decay() {
        let mut inventory = HolographicInventory::new(1, 100).with_capacity(100.0);
        inventory.current_resonance_usage = 50.0;
        inventory.available_resonance = 50.0;

        inventory.decay_resonance(1.0);
        assert!(inventory.current_resonance_usage < 50.0);
    }

    #[test]
    fn test_holographic_blueprint_crafting_resonance() {
        let signature = ArchetypicalItemSignature::new();
        let mut blueprint = HolographicItemBlueprint::new(1, signature);
        blueprint.archetypical_components.push([0.5; 22]);

        let resonance = blueprint.compute_crafting_resonance();
        assert!(resonance > 0.0);
    }

    #[test]
    fn test_inventory_efficiency() {
        let mut inventory = HolographicInventory::new(1, 100).with_capacity(100.0);
        assert_eq!(inventory.compute_resonance_efficiency(), 0.0);

        inventory.current_resonance_usage = 50.0;
        assert_eq!(inventory.compute_resonance_efficiency(), 0.5);
    }
}
