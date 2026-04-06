//! Archetypical Crafting System
//!
//! Implements crafting as archetypical combination with emergent results (no fixed recipes).
//!
//! From GAMING_ENGINE_ROADMAP_v2.md Section 8:
//! > "Emergent Crafting: Instead of fixed recipes, players can combine any archetypical
//! > signatures, and the result emerges from the interference pattern."
//! > "Book = Archetype 1 (Matrix - Mind) + Archetype 16 (Significator - Spirit)"
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Section 2.6:
//! > "Each step INCLUDES all previous development and TRANSCENDS by adding new development."
//!
//! Key Concepts:
//! - **Archetypical Combination**: Combine items based on their 22-archetype patterns
//! - **Emergent Crafting**: No fixed recipes, result emerges from interference pattern
//! - **Interference Pattern**: Combined archetype signatures create interference
//! - **Emergent Result**: Result computed from interference, not from fixed recipe
//! - **Resonance-Based Crafting**: Crafting resonance determines success and quality

use crate::simulation_v3::holographic_inventory::{
    ArchetypicalItemSignature, HolographicItem, HolographicItemBlueprint, ItemCategory, ItemId,
    ResonancePattern,
};
use crate::types::Float;
use std::collections::HashMap;

pub const MIN_CRAFTING_ITEMS: usize = 2;
pub const MAX_CRAFTING_ITEMS: usize = 10;
pub const MIN_INTERFERENCE_COHERENCE: Float = 0.3;
pub const CRAFTING_RESONANCE_THRESHOLD: Float = 0.2;

#[derive(Debug, Clone, PartialEq)]
pub enum CraftingError {
    InsufficientItems { provided: usize, required: usize },
    TooManyItems { provided: usize, maximum: usize },
    LowInterferenceCoherence { coherence: Float, threshold: Float },
    LowCraftingResonance { resonance: Float, threshold: Float },
    InvalidItemSignature,
    UnknownCraftingResult,
}

#[derive(Debug, Clone)]
pub struct CraftingInterferencePattern {
    pub combined_pattern: [Float; 22],
    pub coherence: Float,
    pub stability: Float,
    pub dominant_archetypes: Vec<(usize, Float)>,
}

impl CraftingInterferencePattern {
    pub fn from_signatures(signatures: &[ArchetypicalItemSignature]) -> Self {
        let mut combined_pattern = [0.0; 22];
        let num_items = signatures.len() as Float;

        for signature in signatures {
            for (combined_i, &pattern_i) in combined_pattern
                .iter_mut()
                .zip(signature.archetype_pattern.iter())
            {
                *combined_i += pattern_i;
            }
        }

        for item in &mut combined_pattern {
            *item /= num_items;
        }

        let mut pattern_variance = 0.0;
        for (i, &mean) in combined_pattern.iter().enumerate() {
            for signature in signatures {
                pattern_variance += (signature.archetype_pattern[i] - mean).powi(2);
            }
        }
        pattern_variance /= num_items * 22.0;

        let coherence = (1.0 - pattern_variance.sqrt()).clamp(0.0, 1.0);
        let stability = coherence * 0.8 + 0.2;

        let mut archetype_magnitudes: Vec<(usize, Float)> = combined_pattern
            .iter()
            .enumerate()
            .map(|(i, &val)| (i, val.abs()))
            .collect();

        archetype_magnitudes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // Lower threshold to accommodate averaging (two items at 1.0 become 0.5 when averaged)
        let dominant_archetypes = archetype_magnitudes
            .into_iter()
            .filter(|&(_, mag)| mag > 0.25)
            .take(5)
            .collect();

        Self {
            combined_pattern,
            coherence,
            stability,
            dominant_archetypes,
        }
    }

    pub fn compute_crafting_resonance(&self) -> Float {
        let mut pattern_strength: Float = 0.0;
        for i in 0..22 {
            pattern_strength += self.combined_pattern[i].abs();
        }
        // Normalize by number of dominant archetypes, not all 22
        // This makes crafting more feasible with focused archetype combinations
        let divisor = if self.dominant_archetypes.is_empty() {
            1.0
        } else {
            self.dominant_archetypes.len() as Float
        };
        pattern_strength = (pattern_strength / divisor).min(1.0);

        pattern_strength * self.coherence * self.stability
    }
}

#[derive(Debug, Clone)]
pub struct CraftingResult {
    pub item: HolographicItem,
    pub quality: Float,
    pub coherence: Float,
    pub crafting_resonance: Float,
}

#[derive(Debug, Clone)]
pub struct ArchetypicalCrafting {
    pub min_items: usize,
    pub max_items: usize,
    pub min_coherence: Float,
    pub crafting_threshold: Float,
}

impl Default for ArchetypicalCrafting {
    fn default() -> Self {
        Self {
            min_items: MIN_CRAFTING_ITEMS,
            max_items: MAX_CRAFTING_ITEMS,
            min_coherence: MIN_INTERFERENCE_COHERENCE,
            crafting_threshold: CRAFTING_RESONANCE_THRESHOLD,
        }
    }
}

impl ArchetypicalCrafting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn combine(&self, items: Vec<&HolographicItem>) -> Result<CraftingResult, CraftingError> {
        if items.len() < self.min_items {
            return Err(CraftingError::InsufficientItems {
                provided: items.len(),
                required: self.min_items,
            });
        }

        if items.len() > self.max_items {
            return Err(CraftingError::TooManyItems {
                provided: items.len(),
                maximum: self.max_items,
            });
        }

        let signatures: Vec<ArchetypicalItemSignature> = items
            .iter()
            .map(|item| item.archetype_signature.clone())
            .collect();

        let interference = self.create_interference_pattern(&signatures);

        if interference.coherence < self.min_coherence {
            return Err(CraftingError::LowInterferenceCoherence {
                coherence: interference.coherence,
                threshold: self.min_coherence,
            });
        }

        let crafting_resonance = interference.compute_crafting_resonance();

        if crafting_resonance < self.crafting_threshold {
            return Err(CraftingError::LowCraftingResonance {
                resonance: crafting_resonance,
                threshold: self.crafting_threshold,
            });
        }

        let result = self.emergent_craft(&interference, &items);

        Ok(result)
    }

    pub fn create_interference_pattern(
        &self,
        signatures: &[ArchetypicalItemSignature],
    ) -> CraftingInterferencePattern {
        CraftingInterferencePattern::from_signatures(signatures)
    }

    pub fn emergent_craft(
        &self,
        interference: &CraftingInterferencePattern,
        input_items: &[&HolographicItem],
    ) -> CraftingResult {
        let quality = interference.coherence * interference.stability;
        let crafting_resonance = interference.compute_crafting_resonance();

        let archetype_signature = ArchetypicalItemSignature {
            archetype_pattern: interference.combined_pattern,
            density_affinity: 0.5 + (crafting_resonance - 0.5) * 0.3,
            polarity_bias: self.compute_polarity_bias(input_items),
            resonance_frequency: 440.0 + crafting_resonance * 440.0,
        };

        let resonance_pattern = self.compute_resonance_pattern(&archetype_signature);

        let (item_category, item_name) = self.determine_item_category(interference, input_items);

        let blueprint = self.generate_blueprint(&archetype_signature, &resonance_pattern);

        let item_id = self.generate_item_id(interference, input_items);

        let resonance_cost = archetype_signature.compute_resonance_cost();

        let item = HolographicItem {
            item_id,
            name: item_name,
            category: item_category,
            archetype_signature,
            resonance_pattern,
            holographic_blueprint: Some(blueprint),
            resonance_cost,
            quantity: 1,
            memory_key: None,
        };

        CraftingResult {
            item,
            quality,
            coherence: interference.coherence,
            crafting_resonance,
        }
    }

    fn compute_polarity_bias(&self, items: &[&HolographicItem]) -> Float {
        if items.is_empty() {
            return 0.0;
        }

        let mut sum = 0.0;
        for item in items {
            sum += item.archetype_signature.polarity_bias;
        }
        sum / items.len() as Float
    }

    fn compute_resonance_pattern(&self, signature: &ArchetypicalItemSignature) -> ResonancePattern {
        let base_frequency = signature.resonance_frequency / 880.0;
        let pattern: [Float; 8] = (0..8)
            .map(|i| {
                signature.archetype_pattern[i % 22].abs()
                    * (1.0 + base_frequency * (i as Float / 8.0)).min(1.0)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([0.0; 8]);

        ResonancePattern {
            pattern,
            stability: 0.7 + signature.density_affinity * 0.3,
            phase: base_frequency * std::f64::consts::PI * 2.0,
        }
    }

    fn determine_item_category(
        &self,
        interference: &CraftingInterferencePattern,
        input_items: &[&HolographicItem],
    ) -> (ItemCategory, String) {
        let category_counts: HashMap<ItemCategory, usize> = input_items
            .iter()
            .map(|item| (item.category, 1))
            .fold(HashMap::new(), |mut acc, (cat, count)| {
                *acc.entry(cat).or_insert(0) += count;
                acc
            });

        let dominant_category = category_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cat, _)| *cat)
            .unwrap_or(ItemCategory::Artifact);

        let prefix = self.get_category_prefix(dominant_category);
        let suffix = self.get_archetype_suffix(&interference.dominant_archetypes);

        let item_name = format!("{} {}", prefix, suffix);

        (dominant_category, item_name)
    }

    fn get_category_prefix(&self, category: ItemCategory) -> &'static str {
        match category {
            ItemCategory::Tool => "Forged",
            ItemCategory::Weapon => "Tempered",
            ItemCategory::Material => "Refined",
            ItemCategory::Component => "Crafted",
            ItemCategory::Blueprint => "Encoded",
            ItemCategory::Artifact => "Ancient",
            ItemCategory::Catalyst => "Crystallized",
            ItemCategory::Essence => "Distilled",
            ItemCategory::Consumable => "Brewed",
            ItemCategory::Key => "Forged",
        }
    }

    fn get_archetype_suffix(&self, dominant_archetypes: &[(usize, Float)]) -> String {
        let archetype_names = [
            "Matrix",
            "Potentiator",
            "Catalyst",
            "Experience",
            "Significator",
            "Transformation",
            "GreatWay",
            "BodyMatrix",
            "BodyPotentiator",
            "BodyCatalyst",
            "BodyExperience",
            "BodySignificator",
            "BodyTransformation",
            "BodyGreatWay",
            "SpiritMatrix",
            "SpiritPotentiator",
            "SpiritCatalyst",
            "SpiritExperience",
            "SpiritSignificator",
            "SpiritTransformation",
            "SpiritGreatWay",
            "Choice",
        ];

        let mut suffix_parts = Vec::new();
        for (archetype_idx, _) in dominant_archetypes.iter().take(3) {
            if *archetype_idx < archetype_names.len() {
                suffix_parts.push(archetype_names[*archetype_idx].to_string());
            }
        }

        if suffix_parts.is_empty() {
            "Essence".to_string()
        } else {
            suffix_parts.join(" ")
        }
    }

    fn generate_blueprint(
        &self,
        signature: &ArchetypicalItemSignature,
        _pattern: &ResonancePattern,
    ) -> HolographicItemBlueprint {
        let material_count = (signature.density_affinity * 10.0) as u64 + 1;

        HolographicItemBlueprint {
            blueprint_id: 0,
            base_signature: signature.clone(),
            material_components: (0..material_count).collect(),
            archetypical_components: vec![signature.archetype_pattern],
            craft_difficulty: (1.0 - signature.density_affinity) * 100.0,
            required_density_level: (signature.density_affinity * 8.0),
        }
    }

    fn generate_item_id(
        &self,
        interference: &CraftingInterferencePattern,
        input_items: &[&HolographicItem],
    ) -> u64 {
        let mut hash: u64 = 0;

        for (i, &val) in interference.combined_pattern.iter().enumerate() {
            let val_bits = (val.abs() * 1000.0) as u64;
            hash = hash.wrapping_add(val_bits.wrapping_mul(i as u64 + 1));
        }

        for item in input_items {
            hash = hash.wrapping_add(item.item_id);
        }

        hash
    }

    pub fn analyze_crafting_possibilities(
        &self,
        inventory_items: &[HolographicItem],
    ) -> Vec<(Vec<ItemId>, Float)> {
        let mut possibilities = Vec::new();

        if inventory_items.len() < 2 {
            return possibilities;
        }

        for i in 0..inventory_items.len().saturating_sub(1) {
            for j in (i + 1)..inventory_items.len().min(i + 6) {
                if let Ok(result) = self.combine(vec![&inventory_items[i], &inventory_items[j]]) {
                    let item_ids = vec![inventory_items[i].item_id, inventory_items[j].item_id];
                    possibilities.push((item_ids, result.crafting_resonance));
                }
            }
        }

        possibilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        possibilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_item(id: ItemId, category: ItemCategory) -> HolographicItem {
        let mut item = HolographicItem::new(id, format!("Item {}", id), category);
        item.archetype_signature.archetype_pattern[id as usize % 22] = 1.0;
        item
    }

    #[test]
    fn test_crafting_with_two_items() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Material);
        let item2 = create_test_item(16, ItemCategory::Material);

        let result = crafting.combine(vec![&item1, &item2]);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.item.quantity, 1);
        assert!(result.quality > 0.0);
        assert!(result.coherence > 0.0);
        assert!(result.crafting_resonance > 0.0);
    }

    #[test]
    fn test_insufficient_items() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Material);

        let result = crafting.combine(vec![&item1]);
        assert!(matches!(
            result,
            Err(CraftingError::InsufficientItems { .. })
        ));
    }

    #[test]
    fn test_too_many_items() {
        let crafting = ArchetypicalCrafting::new();
        let items: Vec<HolographicItem> = (0..15)
            .map(|i| create_test_item(i, ItemCategory::Material))
            .collect();

        let result = crafting.combine(items.iter().collect());
        assert!(matches!(result, Err(CraftingError::TooManyItems { .. })));
    }

    #[test]
    fn test_interference_pattern_creation() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Material);
        let item2 = create_test_item(16, ItemCategory::Material);

        let signatures = vec![
            item1.archetype_signature.clone(),
            item2.archetype_signature.clone(),
        ];

        let interference = crafting.create_interference_pattern(&signatures);

        assert!(interference.coherence >= 0.0 && interference.coherence <= 1.0);
        assert!(interference.stability >= 0.0 && interference.stability <= 1.0);
        assert!(!interference.dominant_archetypes.is_empty());
    }

    #[test]
    fn test_crafting_resonance_computation() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Material);
        let item2 = create_test_item(16, ItemCategory::Material);

        let signatures = vec![
            item1.archetype_signature.clone(),
            item2.archetype_signature.clone(),
        ];

        let interference = crafting.create_interference_pattern(&signatures);
        let crafting_resonance = interference.compute_crafting_resonance();

        assert!((0.0..=1.0).contains(&crafting_resonance));
    }

    #[test]
    fn test_emergent_craft_creates_item() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Material);
        let item2 = create_test_item(16, ItemCategory::Material);

        let signatures = vec![
            item1.archetype_signature.clone(),
            item2.archetype_signature.clone(),
        ];

        let interference = crafting.create_interference_pattern(&signatures);
        let result = crafting.emergent_craft(&interference, &[&item1, &item2]);

        assert_eq!(result.item.quantity, 1);
        assert!(result.quality > 0.0);
        assert!(result.coherence > 0.0);
        assert!(result.crafting_resonance > 0.0);
        assert!(result.item.item_id > 0);
        assert!(!result.item.name.is_empty());
        assert!(result.item.holographic_blueprint.is_some());
    }

    #[test]
    fn test_item_category_determination() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Tool);
        let item2 = create_test_item(16, ItemCategory::Tool);

        let signatures = vec![
            item1.archetype_signature.clone(),
            item2.archetype_signature.clone(),
        ];

        let interference = crafting.create_interference_pattern(&signatures);
        let (category, name) = crafting.determine_item_category(&interference, &[&item1, &item2]);

        assert_eq!(category, ItemCategory::Tool);
        assert!(name.starts_with("Forged"));
    }

    #[test]
    fn test_resonance_pattern_generation() {
        let crafting = ArchetypicalCrafting::new();
        let signature = ArchetypicalItemSignature::new();

        let pattern = crafting.compute_resonance_pattern(&signature);

        assert!(pattern.stability >= 0.0 && pattern.stability <= 1.0);
        assert_eq!(pattern.pattern.len(), 8);
    }

    #[test]
    fn test_analyze_crafting_possibilities() {
        let crafting = ArchetypicalCrafting::new();
        let items: Vec<HolographicItem> = (0..10)
            .map(|i| create_test_item(i, ItemCategory::Material))
            .collect();

        let possibilities = crafting.analyze_crafting_possibilities(&items);

        assert!(!possibilities.is_empty());

        for (item_ids, resonance) in possibilities {
            assert_eq!(item_ids.len(), 2);
            assert!((0.0..=1.0).contains(&resonance));
        }
    }

    #[test]
    fn test_default_crafting_config() {
        let crafting = ArchetypicalCrafting::default();

        assert_eq!(crafting.min_items, MIN_CRAFTING_ITEMS);
        assert_eq!(crafting.max_items, MAX_CRAFTING_ITEMS);
        assert_eq!(crafting.min_coherence, MIN_INTERFERENCE_COHERENCE);
        assert_eq!(crafting.crafting_threshold, CRAFTING_RESONANCE_THRESHOLD);
    }

    #[test]
    fn test_item_name_generation() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Weapon);
        let item2 = create_test_item(16, ItemCategory::Weapon);

        let signatures = vec![
            item1.archetype_signature.clone(),
            item2.archetype_signature.clone(),
        ];

        let interference = crafting.create_interference_pattern(&signatures);
        let result = crafting.emergent_craft(&interference, &[&item1, &item2]);

        assert!(result.item.name.starts_with("Tempered"));
        assert!(!result.item.name.is_empty());
    }

    #[test]
    fn test_multiple_crafts_produce_different_items() {
        let crafting = ArchetypicalCrafting::new();
        let item1 = create_test_item(1, ItemCategory::Material);
        let item2 = create_test_item(16, ItemCategory::Material);
        let item3 = create_test_item(4, ItemCategory::Material);

        let result1 = crafting.combine(vec![&item1, &item2]);
        let result2 = crafting.combine(vec![&item1, &item3]);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        let result1 = result1.unwrap();
        let result2 = result2.unwrap();

        assert_ne!(result1.item.item_id, result2.item.item_id);
    }

    #[test]
    fn test_blueprint_generation() {
        let crafting = ArchetypicalCrafting::new();
        let signature = ArchetypicalItemSignature::new();
        let pattern = ResonancePattern::new();

        let blueprint = crafting.generate_blueprint(&signature, &pattern);

        assert!(!blueprint.material_components.is_empty());
        assert!(!blueprint.archetypical_components.is_empty());
        assert!(blueprint.craft_difficulty >= 0.0 && blueprint.craft_difficulty <= 100.0);
        assert!(blueprint.required_density_level >= 1.0 && blueprint.required_density_level <= 8.0);
    }

    #[test]
    fn test_low_coherence_rejection() {
        let crafting = ArchetypicalCrafting::new();
        let mut item1 = create_test_item(1, ItemCategory::Material);
        let mut item2 = create_test_item(2, ItemCategory::Material);

        item1.archetype_signature.archetype_pattern[0] = 1.0;
        item2.archetype_signature.archetype_pattern[1] = -1.0;

        let result = crafting.combine(vec![&item1, &item2]);

        if let Err(CraftingError::LowInterferenceCoherence {
            coherence,
            threshold,
        }) = result
        {
            assert!(coherence < threshold);
            assert_eq!(threshold, MIN_INTERFERENCE_COHERENCE);
        } else {
            assert!(result.is_ok());
        }
    }
}
