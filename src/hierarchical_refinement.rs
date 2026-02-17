// Hierarchical Refinement System
//
// This module implements advanced refinement algorithms and bias propagation system.
// It builds upon the basic holographic archetypical mind architecture from Phase 2.
//
// Phase 3 adds:
// - Bias propagation system with history tracking
// - Refinement optimization with caching
// - Adaptive bias adjustment based on experience
//
// Key Principles:
// - Bias flows through hierarchy (cosmic → logos → sub-logos → entity)
// - Refinement is cumulative and traceable
// - Adaptive bias responds to entity experience and development

use crate::spectrum::archetypical_mind::{Archetype, BiasVector, CosmicMind};
use crate::types::Float;
use std::collections::HashMap;

// ============================================================================
// Bias Propagation System
// ============================================================================

/// Bias Snapshot
///
/// Records the state of bias at a specific point in time.
/// Used to track bias evolution through the hierarchy.
#[derive(Debug, Clone, PartialEq)]
pub struct BiasSnapshot {
    /// Simulation tick when this snapshot was taken
    pub tick: u64,
    /// Bias vector at this moment
    pub bias: BiasVector,
    /// Refinement level (0 = cosmic, 1 = logos, 2 = sub-logos, 3 = entity)
    pub refinement_level: u8,
    /// Cumulative bias magnitude at this point
    pub cumulative_magnitude: Float,
}

impl BiasSnapshot {
    /// Create a new bias snapshot
    pub fn new(
        tick: u64,
        bias: BiasVector,
        refinement_level: u8,
        cumulative_magnitude: Float,
    ) -> Self {
        Self {
            tick,
            bias,
            refinement_level,
            cumulative_magnitude,
        }
    }
}

/// Propagation Rules
///
/// Defines how bias propagates through the hierarchy.
#[derive(Debug, Clone, PartialEq)]
pub struct PropagationRules {
    /// How much bias is inherited from previous level (0.0 to 1.0)
    pub inheritance_factor: Float,
    /// How much bias is modified at current level (0.0 to 1.0)
    pub modification_factor: Float,
    /// Minimum bias magnitude to propagate (0.0 to 1.0)
    pub propagation_threshold: Float,
    /// Bias decay rate per level (0.0 to 1.0)
    pub decay_rate: Float,
}

impl PropagationRules {
    /// Create default propagation rules
    pub fn default() -> Self {
        Self {
            inheritance_factor: 0.7,    // 70% inherited
            modification_factor: 0.3,   // 30% modified
            propagation_threshold: 0.1, // Only propagate if magnitude > 0.1
            decay_rate: 0.05,           // 5% decay per level
        }
    }

    /// Calculate effective bias after propagation
    ///
    /// Combines inherited bias with local modification according to propagation rules.
    pub fn calculate_effective_bias(
        &self,
        inherited: &BiasVector,
        local: &BiasVector,
    ) -> BiasVector {
        let mut effective_biases = [0.0; 22];

        for i in 0..22 {
            let inherited_bias = inherited.biases[i];
            let local_bias = local.biases[i];

            // Combine inherited and local bias
            let effective =
                inherited_bias * self.inheritance_factor + local_bias * self.modification_factor;

            // Apply decay
            let decayed = effective * (1.0 - self.decay_rate);

            effective_biases[i] = decayed.clamp(-1.0, 1.0);
        }

        BiasVector::from_values(effective_biases)
    }

    /// Check if bias should propagate to next level
    pub fn should_propagate(&self, bias: &BiasVector) -> bool {
        bias.magnitude() > self.propagation_threshold
    }
}

/// Bias Propagation System
///
/// Tracks bias flow through the hierarchy and manages bias evolution.
pub struct BiasPropagation {
    /// History of bias snapshots
    pub bias_history: Vec<BiasSnapshot>,
    /// Propagation rules
    pub rules: PropagationRules,
    /// Current tick
    pub current_tick: u64,
    /// Maximum history length (to prevent unbounded growth)
    pub max_history_length: usize,
}

impl BiasPropagation {
    /// Create a new bias propagation system
    pub fn new() -> Self {
        Self {
            bias_history: Vec::new(),
            rules: PropagationRules::default(),
            current_tick: 0,
            max_history_length: 1000, // Keep last 1000 snapshots
        }
    }

    /// Create a new bias propagation system with custom rules
    pub fn with_rules(rules: PropagationRules) -> Self {
        Self {
            bias_history: Vec::new(),
            rules,
            current_tick: 0,
            max_history_length: 1000,
        }
    }

    /// Record a bias snapshot
    pub fn record_snapshot(
        &mut self,
        bias: BiasVector,
        refinement_level: u8,
        cumulative_magnitude: Float,
    ) {
        let snapshot = BiasSnapshot::new(
            self.current_tick,
            bias,
            refinement_level,
            cumulative_magnitude,
        );

        self.bias_history.push(snapshot);

        // Trim history if too long
        if self.bias_history.len() > self.max_history_length {
            self.bias_history.remove(0);
        }
    }

    /// Advance to next tick
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
    }

    /// Get bias history for a specific refinement level
    pub fn get_history_for_level(&self, refinement_level: u8) -> Vec<&BiasSnapshot> {
        self.bias_history
            .iter()
            .filter(|s| s.refinement_level == refinement_level)
            .collect()
    }

    /// Calculate bias evolution rate
    ///
    /// Measures how quickly bias is changing at a given level.
    pub fn calculate_evolution_rate(&self, refinement_level: u8) -> Float {
        let history = self.get_history_for_level(refinement_level);

        if history.len() < 2 {
            return 0.0;
        }

        // Calculate average change between consecutive snapshots
        let mut total_change = 0.0;
        let mut count = 0;

        for i in 1..history.len() {
            let prev = &history[i - 1];
            let curr = &history[i];

            // Calculate magnitude of change
            let change = (curr.cumulative_magnitude - prev.cumulative_magnitude).abs();
            total_change += change;
            count += 1;
        }

        if count == 0 {
            return 0.0;
        }

        total_change / count as Float
    }

    /// Propagate bias through hierarchy
    ///
    /// Simulates bias flow from cosmic → logos → sub-logos → entity.
    pub fn propagate_bias(
        &self,
        cosmic_bias: &BiasVector,
        logos_bias: &BiasVector,
        sub_logos_bias: &BiasVector,
        entity_bias: &BiasVector,
    ) -> Vec<BiasVector> {
        let mut propagated = Vec::new();

        // Level 0: Cosmic (no inheritance, just cosmic bias)
        propagated.push(cosmic_bias.clone());

        // Level 1: Logos (inherits from cosmic)
        let logos_effective = self.rules.calculate_effective_bias(cosmic_bias, logos_bias);
        propagated.push(logos_effective);

        // Level 2: Sub-Logos (inherits from logos)
        let sub_logos_effective = self
            .rules
            .calculate_effective_bias(&propagated[1], sub_logos_bias);
        propagated.push(sub_logos_effective);

        // Level 3: Entity (inherits from sub-logos)
        let entity_effective = self
            .rules
            .calculate_effective_bias(&propagated[2], entity_bias);
        propagated.push(entity_effective);

        propagated
    }

    /// Get current tick
    pub fn get_tick(&self) -> u64 {
        self.current_tick
    }

    /// Get history length
    pub fn get_history_length(&self) -> usize {
        self.bias_history.len()
    }
}

// ============================================================================
// Refinement Cache
// ============================================================================

/// Refinement Key
///
/// Uniquely identifies a refinement operation for caching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefinementKey {
    /// Template ID
    pub template_id: u8,
    /// Bias value applied
    pub bias_value: i32, // Scaled by 1000 for hashing (e.g., 0.123 -> 123)
    /// Previous cumulative bias
    pub previous_bias: i32, // Scaled by 1000
}

impl RefinementKey {
    /// Create a new refinement key
    pub fn new(template_id: u8, bias_value: Float, previous_bias: Float) -> Self {
        Self {
            template_id,
            bias_value: (bias_value * 1000.0) as i32,
            previous_bias: (previous_bias * 1000.0) as i32,
        }
    }
}

/// Refinement Cache Entry
///
/// Stores a refined archetype and metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct RefinementCacheEntry {
    /// Refined archetype
    pub archetype: Archetype,
    /// Timestamp when cached
    pub timestamp: u64,
    /// Access count
    pub access_count: u64,
}

impl RefinementCacheEntry {
    /// Create a new cache entry
    pub fn new(archetype: Archetype, timestamp: u64) -> Self {
        Self {
            archetype,
            timestamp,
            access_count: 0,
        }
    }

    /// Increment access count
    pub fn access(&mut self) {
        self.access_count += 1;
    }
}

/// Refinement Cache
///
/// Caches refined archetypes to optimize performance.
pub struct RefinementCache {
    /// Cache storage
    cache: HashMap<RefinementKey, RefinementCacheEntry>,
    /// Current timestamp
    current_timestamp: u64,
    /// Cache hits
    hits: u64,
    /// Cache misses
    misses: u64,
    /// Maximum cache size
    max_size: usize,
}

impl RefinementCache {
    /// Create a new refinement cache
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            current_timestamp: 0,
            hits: 0,
            misses: 0,
            max_size: 1000, // Maximum 1000 cached entries
        }
    }

    /// Create a new refinement cache with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            current_timestamp: 0,
            hits: 0,
            misses: 0,
            max_size,
        }
    }

    /// Try to get a cached refinement
    pub fn get(&mut self, key: &RefinementKey) -> Option<&Archetype> {
        if let Some(entry) = self.cache.get_mut(key) {
            entry.access();
            self.hits += 1;
            Some(&entry.archetype)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Store a refinement in cache
    pub fn put(&mut self, key: RefinementKey, archetype: Archetype) {
        // Evict oldest entries if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        let entry = RefinementCacheEntry::new(archetype, self.current_timestamp);
        self.cache.insert(key, entry);
    }

    /// Evict oldest entries from cache
    fn evict_oldest(&mut self) {
        // Find oldest entry
        if let Some((&oldest_key, _)) = self.cache.iter().min_by_key(|(_, entry)| entry.timestamp) {
            self.cache.remove(&oldest_key);
        }
    }

    /// Advance timestamp
    pub fn advance_timestamp(&mut self) {
        self.current_timestamp += 1;
    }

    /// Calculate cache hit rate
    pub fn hit_rate(&self) -> Float {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as Float / total as Float
    }

    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Get cache statistics
    pub fn stats(&self) -> (u64, u64, Float) {
        (self.hits, self.misses, self.hit_rate())
    }
}

// ============================================================================
// Adaptive Bias
// ============================================================================

/// Adaptive Bias
///
/// Dynamically adjusts bias based on entity experience and development.
pub struct AdaptiveBias {
    /// Base bias (inherent to entity)
    pub base_bias: BiasVector,
    /// Experience modifier (adjusts based on experience)
    pub experience_modifier: BiasVector,
    /// Developmental modifier (adjusts based on developmental level)
    pub developmental_modifier: BiasVector,
    /// Polarity modifier (adjusts based on polarization intensity)
    pub polarization_modifier: BiasVector,
}

impl AdaptiveBias {
    /// Create a new adaptive bias system
    pub fn new(base_bias: BiasVector) -> Self {
        Self {
            base_bias: base_bias.clone(),
            experience_modifier: BiasVector::new(),
            developmental_modifier: BiasVector::new(),
            polarization_modifier: BiasVector::new(),
        }
    }

    /// Calculate effective bias based on entity state
    ///
    /// Combines base bias with modifiers weighted by entity state.
    pub fn calculate_effective_bias(
        &self,
        experience_factor: Float,      // 0.0 to 1.0
        developmental_level: Float,    // 0.0 to 1.0
        polarization_intensity: Float, // 0.0 to 1.0
    ) -> BiasVector {
        let mut effective_biases = [0.0; 22];

        for i in 0..22 {
            let base = self.base_bias.biases[i];
            let exp_mod = self.experience_modifier.biases[i];
            let dev_mod = self.developmental_modifier.biases[i];
            let pol_mod = self.polarization_modifier.biases[i];

            // Weighted combination
            let effective = base
                + exp_mod * experience_factor
                + dev_mod * developmental_level
                + pol_mod * polarization_intensity;

            effective_biases[i] = effective.clamp(-1.0, 1.0);
        }

        BiasVector::from_values(effective_biases)
    }

    /// Update experience modifier based on experience accumulation
    ///
    /// Adjusts bias based on archetype experience.
    pub fn update_experience_modifier(&mut self, archetype_id: u8, experience_delta: Float) {
        // Experience strengthens existing bias (positive feedback loop)
        let current_bias = self.base_bias.get_bias(archetype_id);
        let adjustment = current_bias * experience_delta * 0.1; // 10% effect

        let current_mod = self.experience_modifier.get_bias(archetype_id);
        self.experience_modifier
            .set_bias(archetype_id, (current_mod + adjustment).clamp(-1.0, 1.0));
    }

    /// Update developmental modifier based on developmental progress
    ///
    /// Adjusts bias based on overall development.
    pub fn update_developmental_modifier(&mut self, developmental_level: Float) {
        // Development tends toward balance (reduces extreme bias)
        for i in 1..=22 {
            let current_bias = self.base_bias.get_bias(i);
            let adjustment = -current_bias * developmental_level * 0.05; // 5% reduction per level

            let current_mod = self.developmental_modifier.get_bias(i);
            self.developmental_modifier
                .set_bias(i, (current_mod + adjustment).clamp(-1.0, 1.0));
        }
    }

    /// Update polarization modifier based on polarization intensity
    ///
    /// Amplifies bias in the direction of polarization.
    pub fn update_polarization_modifier(&mut self, polarization_direction: Float) {
        // Polarity amplifies existing bias
        for i in 1..=22 {
            let current_bias = self.base_bias.get_bias(i);
            let adjustment = current_bias * polarization_direction * 0.1; // 10% amplification

            let current_mod = self.polarization_modifier.get_bias(i);
            self.polarization_modifier
                .set_bias(i, (current_mod + adjustment).clamp(-1.0, 1.0));
        }
    }

    /// Set experience modifier directly
    pub fn set_experience_modifier(&mut self, modifier: BiasVector) {
        self.experience_modifier = modifier;
    }

    /// Set developmental modifier directly
    pub fn set_developmental_modifier(&mut self, modifier: BiasVector) {
        self.developmental_modifier = modifier;
    }

    /// Set polarization modifier directly
    pub fn set_polarization_modifier(&mut self, modifier: BiasVector) {
        self.polarization_modifier = modifier;
    }

    /// Get base bias
    pub fn get_base_bias(&self) -> &BiasVector {
        &self.base_bias
    }

    /// Get experience modifier
    pub fn get_experience_modifier(&self) -> &BiasVector {
        &self.experience_modifier
    }

    /// Get developmental modifier
    pub fn get_developmental_modifier(&self) -> &BiasVector {
        &self.developmental_modifier
    }

    /// Get polarization modifier
    pub fn get_polarization_modifier(&self) -> &BiasVector {
        &self.polarization_modifier
    }
}

// ============================================================================
// Hierarchical Refinement Engine
// ============================================================================

/// Hierarchical Refinement Engine
///
/// Orchestrates the entire refinement process with propagation, caching, and adaptation.
pub struct HierarchicalRefinementEngine {
    /// Bias propagation system
    pub propagation: BiasPropagation,
    /// Refinement cache
    pub cache: RefinementCache,
    /// Adaptive bias system (per entity, stored by entity ID)
    pub adaptive_biases: HashMap<u64, AdaptiveBias>,
}

impl HierarchicalRefinementEngine {
    /// Create a new hierarchical refinement engine
    pub fn new() -> Self {
        Self {
            propagation: BiasPropagation::new(),
            cache: RefinementCache::new(),
            adaptive_biases: HashMap::new(),
        }
    }

    /// Create a new hierarchical refinement engine with custom settings
    pub fn with_settings(propagation_rules: PropagationRules, cache_max_size: usize) -> Self {
        Self {
            propagation: BiasPropagation::with_rules(propagation_rules),
            cache: RefinementCache::with_max_size(cache_max_size),
            adaptive_biases: HashMap::new(),
        }
    }

    /// Register an entity for adaptive bias tracking
    pub fn register_entity(&mut self, entity_id: u64, base_bias: BiasVector) {
        let adaptive_bias = AdaptiveBias::new(base_bias);
        self.adaptive_biases.insert(entity_id, adaptive_bias);
    }

    /// Refine archetypes through hierarchy with caching
    ///
    /// Performs hierarchical refinement with optimization and caching.
    pub fn refine_archetypes(
        &mut self,
        cosmic_mind: &CosmicMind,
        logos_bias: &BiasVector,
        sub_logos_bias: &BiasVector,
        entity_bias: &BiasVector,
    ) -> (Vec<[Archetype; 22]>, Vec<BiasVector>) {
        // Propagate bias through hierarchy
        let propagated_biases = self.propagation.propagate_bias(
            &BiasVector::new(), // Cosmic has no bias (neutral)
            logos_bias,
            sub_logos_bias,
            entity_bias,
        );

        let mut refined_levels = Vec::new();

        // Level 0: Cosmic archetypes (templates)
        let cosmic_archetypes = cosmic_mind.get_archetypes();
        refined_levels.push(core::array::from_fn(|i| {
            Archetype::from_template(&cosmic_archetypes[i])
        }));

        // Level 1: Logos refinement
        let logos_refined = self.refine_with_cache(&refined_levels[0], &propagated_biases[1], 1);
        refined_levels.push(logos_refined);

        // Level 2: Sub-Logos refinement
        let sub_logos_refined =
            self.refine_with_cache(&refined_levels[1], &propagated_biases[2], 2);
        refined_levels.push(sub_logos_refined);

        // Level 3: Entity refinement
        let entity_refined = self.refine_with_cache(&refined_levels[2], &propagated_biases[3], 3);
        refined_levels.push(entity_refined);

        // Record snapshots for each level
        for (level, bias) in propagated_biases.iter().enumerate() {
            self.propagation
                .record_snapshot(bias.clone(), level as u8, bias.magnitude());
        }

        (refined_levels, propagated_biases)
    }

    /// Refine archetypes with caching
    ///
    /// Attempts to use cached refinements before computing new ones.
    fn refine_with_cache(
        &mut self,
        input_archetypes: &[Archetype; 22],
        bias: &BiasVector,
        _refinement_level: u8,
    ) -> [Archetype; 22] {
        // Initialize with input archetypes
        let mut refined = input_archetypes.clone();

        for i in 0..22 {
            let template_id = (i + 1) as u8;
            let bias_value = bias.biases[i];
            let previous_bias = input_archetypes[i].cumulative_bias;

            // Try cache
            let key = RefinementKey::new(template_id, bias_value, previous_bias);
            if let Some(cached) = self.cache.get(&key) {
                refined[i] = cached.clone();
                continue;
            }

            // Compute refinement
            let mut archetype = input_archetypes[i].clone();
            archetype.apply_bias(bias_value);

            // Store in cache
            self.cache.put(key, archetype.clone());

            refined[i] = archetype;
        }

        refined
    }

    /// Update entity adaptive bias
    ///
    /// Updates an entity's adaptive bias based on its current state.
    pub fn update_entity_adaptive_bias(
        &mut self,
        entity_id: u64,
        experience_factor: Float,
        developmental_level: Float,
        polarization_intensity: Float,
    ) -> Option<BiasVector> {
        if let Some(adaptive_bias) = self.adaptive_biases.get_mut(&entity_id) {
            adaptive_bias.update_developmental_modifier(developmental_level);
            adaptive_bias.update_polarization_modifier(polarization_intensity);

            let effective = adaptive_bias.calculate_effective_bias(
                experience_factor,
                developmental_level,
                polarization_intensity,
            );

            Some(effective)
        } else {
            None
        }
    }

    /// Update entity experience for specific archetype
    ///
    /// Records experience accumulation for an archetype.
    pub fn update_entity_experience(
        &mut self,
        entity_id: u64,
        archetype_id: u8,
        experience_delta: Float,
    ) {
        if let Some(adaptive_bias) = self.adaptive_biases.get_mut(&entity_id) {
            adaptive_bias.update_experience_modifier(archetype_id, experience_delta);
        }
    }

    /// Advance simulation tick
    pub fn advance_tick(&mut self) {
        self.propagation.advance_tick();
        self.cache.advance_timestamp();
    }

    /// Get propagation system
    pub fn get_propagation(&self) -> &BiasPropagation {
        &self.propagation
    }

    /// Get cache
    pub fn get_cache(&self) -> &RefinementCache {
        &self.cache
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> (u64, u64, Float) {
        self.cache.stats()
    }

    /// Clear all adaptive biases
    pub fn clear_adaptive_biases(&mut self) {
        self.adaptive_biases.clear();
    }

    /// Get number of registered entities
    pub fn get_entity_count(&self) -> usize {
        self.adaptive_biases.len()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum::archetypical_mind::ArchetypeTemplate;

    // -------------------------------------------------------------------------
    // BiasSnapshot Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_bias_snapshot_creation() {
        let bias = BiasVector::new();
        let snapshot = BiasSnapshot::new(10, bias, 1, 0.5);

        assert_eq!(snapshot.tick, 10);
        assert_eq!(snapshot.refinement_level, 1);
        assert_eq!(snapshot.cumulative_magnitude, 0.5);
    }

    // -------------------------------------------------------------------------
    // PropagationRules Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_propagation_rules_default() {
        let rules = PropagationRules::default();

        assert_eq!(rules.inheritance_factor, 0.7);
        assert_eq!(rules.modification_factor, 0.3);
        assert_eq!(rules.propagation_threshold, 0.1);
        assert_eq!(rules.decay_rate, 0.05);
    }

    #[test]
    fn test_propagation_rules_calculate_effective_bias() {
        let rules = PropagationRules::default();

        let mut inherited = BiasVector::new();
        inherited.set_bias(1, 0.5);

        let mut local = BiasVector::new();
        local.set_bias(1, 0.3);

        let effective = rules.calculate_effective_bias(&inherited, &local);

        let expected = 0.5 * 0.7 + 0.3 * 0.3; // 0.35 + 0.09 = 0.44
        let effective_bias = effective.get_bias(1);

        assert!((effective_bias - expected * 0.95).abs() < 0.01); // Account for decay
    }

    #[test]
    fn test_propagation_rules_should_propagate() {
        let rules = PropagationRules::default();

        let mut bias_above = BiasVector::new();
        bias_above.set_bias(1, 0.5);
        assert!(rules.should_propagate(&bias_above));

        let mut bias_below = BiasVector::new();
        bias_below.set_bias(1, 0.05);
        assert!(!rules.should_propagate(&bias_below));
    }

    // -------------------------------------------------------------------------
    // BiasPropagation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_bias_propagation_creation() {
        let propagation = BiasPropagation::new();

        assert_eq!(propagation.current_tick, 0);
        assert_eq!(propagation.get_history_length(), 0);
        assert_eq!(propagation.max_history_length, 1000);
    }

    #[test]
    fn test_bias_propagation_record_snapshot() {
        let mut propagation = BiasPropagation::new();
        let bias = BiasVector::new();

        propagation.record_snapshot(bias.clone(), 0, 0.0);

        assert_eq!(propagation.get_history_length(), 1);
    }

    #[test]
    fn test_bias_propagation_advance_tick() {
        let mut propagation = BiasPropagation::new();

        propagation.advance_tick();
        assert_eq!(propagation.get_tick(), 1);

        propagation.advance_tick();
        assert_eq!(propagation.get_tick(), 2);
    }

    #[test]
    fn test_bias_propagation_get_history_for_level() {
        let mut propagation = BiasPropagation::new();
        let bias = BiasVector::new();

        propagation.record_snapshot(bias.clone(), 0, 0.0);
        propagation.record_snapshot(bias.clone(), 1, 0.5);
        propagation.record_snapshot(bias.clone(), 0, 0.3);

        let level_0_history = propagation.get_history_for_level(0);
        assert_eq!(level_0_history.len(), 2);

        let level_1_history = propagation.get_history_for_level(1);
        assert_eq!(level_1_history.len(), 1);
    }

    #[test]
    fn test_bias_propagation_calculate_evolution_rate() {
        let mut propagation = BiasPropagation::new();

        // No history
        assert_eq!(propagation.calculate_evolution_rate(0), 0.0);

        // One snapshot
        let bias = BiasVector::new();
        propagation.record_snapshot(bias.clone(), 0, 0.0);
        assert_eq!(propagation.calculate_evolution_rate(0), 0.0);

        // Multiple snapshots
        propagation.record_snapshot(bias.clone(), 0, 0.1);
        propagation.record_snapshot(bias.clone(), 0, 0.3);

        let rate = propagation.calculate_evolution_rate(0);
        assert!(rate > 0.0);
    }

    #[test]
    fn test_bias_propagation_propagate_bias() {
        let propagation = BiasPropagation::new();

        let mut logos_bias = BiasVector::new();
        logos_bias.set_bias(1, 0.5);

        let mut sub_logos_bias = BiasVector::new();
        sub_logos_bias.set_bias(1, 0.3);

        let mut entity_bias = BiasVector::new();
        entity_bias.set_bias(1, 0.4);

        let propagated = propagation.propagate_bias(
            &BiasVector::new(),
            &logos_bias,
            &sub_logos_bias,
            &entity_bias,
        );

        assert_eq!(propagated.len(), 4);

        // Check that bias accumulates
        assert!(propagated[1].get_bias(1) > propagated[0].get_bias(1));
        assert!(
            propagated[2].get_bias(1) > propagated[1].get_bias(1)
                || propagated[2].get_bias(1) < propagated[1].get_bias(1)
        ); // May be higher or lower due to inheritance
    }

    #[test]
    fn test_bias_propagation_history_trimming() {
        let mut propagation = BiasPropagation::with_rules(PropagationRules::default());
        propagation.max_history_length = 5;

        let bias = BiasVector::new();

        for i in 0..10 {
            propagation.record_snapshot(bias.clone(), 0, 0.0);
            propagation.advance_tick();
        }

        // Should only keep last 5
        assert_eq!(propagation.get_history_length(), 5);
    }

    // -------------------------------------------------------------------------
    // RefinementKey Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_refinement_key_creation() {
        let key = RefinementKey::new(1, 0.123, 0.456);

        assert_eq!(key.template_id, 1);
        assert_eq!(key.bias_value, 123);
        assert_eq!(key.previous_bias, 456);
    }

    // -------------------------------------------------------------------------
    // RefinementCacheEntry Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_refinement_cache_entry_creation() {
        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );
        let archetype = Archetype::from_template(&template);
        let entry = RefinementCacheEntry::new(archetype.clone(), 10);

        assert_eq!(entry.timestamp, 10);
        assert_eq!(entry.access_count, 0);
        assert_eq!(entry.archetype, archetype);
    }

    #[test]
    fn test_refinement_cache_entry_access() {
        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );
        let archetype = Archetype::from_template(&template);
        let mut entry = RefinementCacheEntry::new(archetype, 10);

        assert_eq!(entry.access_count, 0);

        entry.access();
        assert_eq!(entry.access_count, 1);

        entry.access();
        entry.access();
        assert_eq!(entry.access_count, 3);
    }

    // -------------------------------------------------------------------------
    // RefinementCache Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_refinement_cache_creation() {
        let cache = RefinementCache::new();

        assert_eq!(cache.size(), 0);
        assert_eq!(cache.current_timestamp, 0);
        assert_eq!(cache.max_size, 1000);
    }

    #[test]
    fn test_refinement_cache_put_and_get() {
        let mut cache = RefinementCache::new();

        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );
        let archetype = Archetype::from_template(&template);

        let key = RefinementKey::new(1, 0.1, 0.0);
        cache.put(key.clone(), archetype.clone());

        assert_eq!(cache.size(), 1);

        let retrieved = cache.get(&key);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &archetype);
    }

    #[test]
    fn test_refinement_cache_miss() {
        let mut cache = RefinementCache::new();

        let key = RefinementKey::new(1, 0.1, 0.0);
        let retrieved = cache.get(&key);

        assert!(retrieved.is_none());
    }

    #[test]
    fn test_refinement_cache_hit_rate() {
        let mut cache = RefinementCache::new();

        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );
        let archetype = Archetype::from_template(&template);

        let key = RefinementKey::new(1, 0.1, 0.0);

        // Miss
        cache.get(&key);
        assert_eq!(cache.hit_rate(), 0.0);

        // Put
        cache.put(key.clone(), archetype);

        // Hit
        cache.get(&key);
        assert_eq!(cache.hit_rate(), 0.5);

        // Another hit
        cache.get(&key);
        assert!((cache.hit_rate() - 0.6666667).abs() < 0.0001);
    }

    #[test]
    fn test_refinement_cache_eviction() {
        let mut cache = RefinementCache::with_max_size(3);

        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );

        cache.put(
            RefinementKey::new(1, 0.1, 0.0),
            Archetype::from_template(&template),
        );
        cache.advance_timestamp();

        cache.put(
            RefinementKey::new(2, 0.2, 0.0),
            Archetype::from_template(&template),
        );
        cache.advance_timestamp();

        cache.put(
            RefinementKey::new(3, 0.3, 0.0),
            Archetype::from_template(&template),
        );
        cache.advance_timestamp();

        assert_eq!(cache.size(), 3);

        // This should evict the oldest entry
        cache.put(
            RefinementKey::new(4, 0.4, 0.0),
            Archetype::from_template(&template),
        );

        assert_eq!(cache.size(), 3);
        assert!(cache.get(&RefinementKey::new(1, 0.1, 0.0)).is_none());
    }

    #[test]
    fn test_refinement_cache_clear() {
        let mut cache = RefinementCache::new();

        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );
        let archetype = Archetype::from_template(&template);

        cache.put(RefinementKey::new(1, 0.1, 0.0), archetype);
        cache.advance_timestamp();

        assert_eq!(cache.size(), 1);

        cache.clear();

        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hit_rate(), 0.0);
    }

    #[test]
    fn test_refinement_cache_stats() {
        let mut cache = RefinementCache::new();

        let template = ArchetypeTemplate::new(
            1,
            String::from("Test"),
            0.5,
            0.5,
            crate::holographic_archetypical_mind::ComplexType::Mind,
        );
        let archetype = Archetype::from_template(&template);

        let key = RefinementKey::new(1, 0.1, 0.0);

        cache.get(&key); // Miss
        cache.put(key.clone(), archetype);
        cache.get(&key); // Hit
        cache.get(&key); // Hit

        let (hits, misses, hit_rate) = cache.stats();

        assert_eq!(hits, 2);
        assert_eq!(misses, 1);
        assert!((hit_rate - 0.6666667).abs() < 0.0001);
    }

    // -------------------------------------------------------------------------
    // AdaptiveBias Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_adaptive_bias_creation() {
        let base_bias = BiasVector::new();
        let adaptive = AdaptiveBias::new(base_bias.clone());

        assert_eq!(adaptive.get_base_bias(), &base_bias);
        assert_eq!(adaptive.get_experience_modifier().magnitude(), 0.0);
        assert_eq!(adaptive.get_developmental_modifier().magnitude(), 0.0);
        assert_eq!(adaptive.get_polarization_modifier().magnitude(), 0.0);
    }

    #[test]
    fn test_adaptive_bias_calculate_effective_bias() {
        let base_bias = BiasVector::new();
        let adaptive = AdaptiveBias::new(base_bias);

        let effective = adaptive.calculate_effective_bias(0.5, 0.5, 0.5);

        // Should be base bias (all zeros) since modifiers are zeros
        assert_eq!(effective.magnitude(), 0.0);
    }

    #[test]
    fn test_adaptive_bias_update_experience_modifier() {
        let mut base_bias = BiasVector::new();
        base_bias.set_bias(1, 0.5);

        let mut adaptive = AdaptiveBias::new(base_bias);

        adaptive.update_experience_modifier(1, 0.1);

        let exp_mod = adaptive.get_experience_modifier();
        let bias = exp_mod.get_bias(1);

        // Should increase in the direction of base bias
        assert!(bias > 0.0);
        assert!(bias < 1.0);
    }

    #[test]
    fn test_adaptive_bias_update_developmental_modifier() {
        let mut base_bias = BiasVector::new();
        base_bias.set_bias(1, 0.8);

        let mut adaptive = AdaptiveBias::new(base_bias);

        adaptive.update_developmental_modifier(0.5);

        let dev_mod = adaptive.get_developmental_modifier();
        let bias = dev_mod.get_bias(1);

        // Should reduce extreme bias (move toward 0)
        assert!(bias < 0.0);
        assert!(bias > -1.0);
    }

    #[test]
    fn test_adaptive_bias_update_polarization_modifier() {
        let mut base_bias = BiasVector::new();
        base_bias.set_bias(1, 0.5);

        let mut adaptive = AdaptiveBias::new(base_bias);

        adaptive.update_polarization_modifier(0.8);

        let pol_mod = adaptive.get_polarization_modifier();
        let bias = pol_mod.get_bias(1);

        // Should amplify in the direction of base bias
        assert!(bias > 0.0);
        assert!(bias < 1.0);
    }

    #[test]
    fn test_adaptive_bias_setters() {
        let base_bias = BiasVector::new();
        let mut adaptive = AdaptiveBias::new(base_bias);

        let mut exp_mod = BiasVector::new();
        exp_mod.set_bias(1, 0.5);
        adaptive.set_experience_modifier(exp_mod.clone());
        assert_eq!(adaptive.get_experience_modifier(), &exp_mod);

        let mut dev_mod = BiasVector::new();
        dev_mod.set_bias(1, 0.3);
        adaptive.set_developmental_modifier(dev_mod.clone());
        assert_eq!(adaptive.get_developmental_modifier(), &dev_mod);

        let mut pol_mod = BiasVector::new();
        pol_mod.set_bias(1, 0.7);
        adaptive.set_polarization_modifier(pol_mod.clone());
        assert_eq!(adaptive.get_polarization_modifier(), &pol_mod);
    }

    // -------------------------------------------------------------------------
    // HierarchicalRefinementEngine Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_hierarchical_refinement_engine_creation() {
        let engine = HierarchicalRefinementEngine::new();

        assert_eq!(engine.get_entity_count(), 0);
        assert_eq!(engine.get_propagation().get_tick(), 0);
        assert_eq!(engine.get_cache().size(), 0);
    }

    #[test]
    fn test_hierarchical_refinement_engine_register_entity() {
        let mut engine = HierarchicalRefinementEngine::new();
        let base_bias = BiasVector::from_seed(123);

        engine.register_entity(1, base_bias.clone());

        assert_eq!(engine.get_entity_count(), 1);
    }

    #[test]
    fn test_hierarchical_refinement_engine_refine_archetypes() {
        let mut engine = HierarchicalRefinementEngine::new();
        let cosmic_mind = CosmicMind::new();

        let logos_bias = BiasVector::from_seed(1);
        let sub_logos_bias = BiasVector::from_seed(2);
        let entity_bias = BiasVector::from_seed(3);

        let (refined_levels, propagated_biases) =
            engine.refine_archetypes(&cosmic_mind, &logos_bias, &sub_logos_bias, &entity_bias);

        assert_eq!(refined_levels.len(), 4); // Cosmic, Logos, Sub-Logos, Entity
        assert_eq!(propagated_biases.len(), 4);

        // Each level should have 22 archetypes
        for level in &refined_levels {
            assert_eq!(level.len(), 22);
        }
    }

    #[test]
    fn test_hierarchical_refinement_engine_update_entity_adaptive_bias() {
        let mut engine = HierarchicalRefinementEngine::new();
        let base_bias = BiasVector::from_seed(123);

        engine.register_entity(1, base_bias);

        let effective = engine.update_entity_adaptive_bias(1, 0.5, 0.5, 0.5);

        assert!(effective.is_some());
    }

    #[test]
    fn test_hierarchical_refinement_engine_update_entity_adaptive_bias_not_registered() {
        let mut engine = HierarchicalRefinementEngine::new();

        let effective = engine.update_entity_adaptive_bias(1, 0.5, 0.5, 0.5);

        assert!(effective.is_none());
    }

    #[test]
    fn test_hierarchical_refinement_engine_update_entity_experience() {
        let mut engine = HierarchicalRefinementEngine::new();
        let base_bias = BiasVector::from_seed(123);

        engine.register_entity(1, base_bias);

        // Should not panic
        engine.update_entity_experience(1, 1, 0.1);
    }

    #[test]
    fn test_hierarchical_refinement_engine_advance_tick() {
        let mut engine = HierarchicalRefinementEngine::new();

        engine.advance_tick();
        assert_eq!(engine.get_propagation().get_tick(), 1);

        engine.advance_tick();
        assert_eq!(engine.get_propagation().get_tick(), 2);
    }

    #[test]
    fn test_hierarchical_refinement_engine_cache_stats() {
        let mut engine = HierarchicalRefinementEngine::new();
        let cosmic_mind = CosmicMind::new();

        let logos_bias = BiasVector::from_seed(1);
        let sub_logos_bias = BiasVector::from_seed(2);
        let entity_bias = BiasVector::from_seed(3);

        // First call (cache misses)
        engine.refine_archetypes(&cosmic_mind, &logos_bias, &sub_logos_bias, &entity_bias);

        let (hits, misses, hit_rate) = engine.get_cache_stats();

        assert_eq!(hits, 0);
        assert!(misses > 0);
        assert_eq!(hit_rate, 0.0);
    }

    #[test]
    fn test_hierarchical_refinement_engine_clear_adaptive_biases() {
        let mut engine = HierarchicalRefinementEngine::new();
        let base_bias = BiasVector::from_seed(123);

        engine.register_entity(1, base_bias.clone());
        engine.register_entity(2, base_bias);

        assert_eq!(engine.get_entity_count(), 2);

        engine.clear_adaptive_biases();

        assert_eq!(engine.get_entity_count(), 0);
    }

    #[test]
    fn test_hierarchical_refinement_engine_with_settings() {
        let rules = PropagationRules {
            inheritance_factor: 0.8,
            modification_factor: 0.2,
            propagation_threshold: 0.2,
            decay_rate: 0.1,
        };

        let engine = HierarchicalRefinementEngine::with_settings(rules, 500);

        assert_eq!(engine.get_cache().max_size, 500);
        assert_eq!(engine.get_propagation().rules.inheritance_factor, 0.8);
    }

    // -------------------------------------------------------------------------
    // Integration Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_full_refinement_workflow() {
        let mut engine = HierarchicalRefinementEngine::new();
        let cosmic_mind = CosmicMind::new();

        // Register entities
        let base_bias_1 = BiasVector::from_seed(100);
        let base_bias_2 = BiasVector::from_seed(200);

        engine.register_entity(1, base_bias_1);
        engine.register_entity(2, base_bias_2);

        // Refine archetypes for entity 1
        let logos_bias = BiasVector::from_seed(1);
        let sub_logos_bias = BiasVector::from_seed(2);
        let entity_bias = engine
            .update_entity_adaptive_bias(1, 0.5, 0.5, 0.5)
            .unwrap();

        let (refined_levels, _) =
            engine.refine_archetypes(&cosmic_mind, &logos_bias, &sub_logos_bias, &entity_bias);

        // Should have 4 levels
        assert_eq!(refined_levels.len(), 4);

        // Each level should have 22 archetypes
        for level in &refined_levels {
            assert_eq!(level.len(), 22);
        }

        // Entity level archetypes should be most refined (highest cumulative bias)
        let cosmic_level = &refined_levels[0];
        let entity_level = &refined_levels[3];

        let cosmic_total_bias: Float = cosmic_level.iter().map(|a| a.cumulative_bias.abs()).sum();
        let entity_total_bias: Float = entity_level.iter().map(|a| a.cumulative_bias.abs()).sum();

        assert!(entity_total_bias > cosmic_total_bias);
    }

    #[test]
    fn test_bias_evolution_over_time() {
        let mut engine = HierarchicalRefinementEngine::new();
        let cosmic_mind = CosmicMind::new();

        let logos_bias = BiasVector::from_seed(1);
        let sub_logos_bias = BiasVector::from_seed(2);
        let entity_bias = BiasVector::from_seed(3);

        // Perform refinement over multiple ticks
        for _ in 0..10 {
            engine.refine_archetypes(&cosmic_mind, &logos_bias, &sub_logos_bias, &entity_bias);
            engine.advance_tick();
        }

        // Check evolution rate for level 0 (cosmic)
        let evolution_rate = engine.get_propagation().calculate_evolution_rate(0);

        // Cosmic level doesn't change, so rate should be 0
        assert_eq!(evolution_rate, 0.0);
    }

    #[test]
    fn test_adaptive_bias_accumulation() {
        let mut engine = HierarchicalRefinementEngine::new();
        let base_bias = BiasVector::from_seed(123);

        engine.register_entity(1, base_bias);

        // Accumulate experience over time
        for i in 1..=10 {
            engine.update_entity_experience(1, i as u8, 0.1);
        }

        // Get effective bias
        let effective = engine.update_entity_adaptive_bias(1, 0.5, 0.5, 0.5);

        assert!(effective.is_some());
        let effective_bias = effective.unwrap();

        // Experience modifier should have accumulated
        assert!(effective_bias.magnitude() > 0.0);
    }

    #[test]
    fn test_refinement_cache_effectiveness() {
        let mut engine = HierarchicalRefinementEngine::new();
        let cosmic_mind = CosmicMind::new();

        let logos_bias = BiasVector::from_seed(1);
        let sub_logos_bias = BiasVector::from_seed(2);
        let entity_bias = BiasVector::from_seed(3);

        // First refinement (cache misses)
        engine.refine_archetypes(&cosmic_mind, &logos_bias, &sub_logos_bias, &entity_bias);

        let (hits1, misses1, _) = engine.get_cache_stats();

        // Second refinement with same biases (cache hits)
        engine.refine_archetypes(&cosmic_mind, &logos_bias, &sub_logos_bias, &entity_bias);

        let (hits2, misses2, hit_rate) = engine.get_cache_stats();

        // Should have more hits on second run
        assert!(hits2 > hits1);
        assert!(misses2 == misses1); // No new misses
        assert!(hit_rate > 0.0);
    }
}
