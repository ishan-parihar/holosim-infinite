//! Observer Effect as Cache Invalidation - Quantum Mechanics Implementation
//!
//! This module implements the observer effect as a cache invalidation mechanism
//! in the holographic simulation. Observation affects probability collapse,
//! and collapsed states are cached for re-observation optimization.
//!
//! From COSMOLOGICAL-ARCHITECTURE.md Layer 4 (The Great Mystery/Quantum Realm):
//! "The Veil forms as a structural feature...Quantum Realm begins...observation
//! affects probability collapse - 'the watched pot boils differently'."
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md Section 6.2:
//! "Observer Effect as Cache Invalidation: Unobserved: Keep as compressed possibility.
//! Observed: Cache collapsed state. Re-observe: Use cached state."

use crate::types::Float;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// Observer effect - the impact of observation on entity states
#[derive(Debug, Clone)]
pub struct ObserverEffect {
    observer_id: u64,
    target_id: u64,
    observation_timestamp: u64,
    observation_strength: Float,
}

impl ObserverEffect {
    pub fn new(observer_id: u64, target_id: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        ObserverEffect {
            observer_id,
            target_id,
            observation_timestamp: timestamp,
            observation_strength: 1.0,
        }
    }

    pub fn with_strength(mut self, strength: Float) -> Self {
        self.observation_strength = strength.clamp(0.0, 1.0);
        self
    }

    pub fn observer_id(&self) -> u64 {
        self.observer_id
    }

    pub fn target_id(&self) -> u64 {
        self.target_id
    }

    pub fn observation_timestamp(&self) -> u64 {
        self.observation_timestamp
    }

    pub fn observation_strength(&self) -> Float {
        self.observation_strength
    }

    pub fn is_strong_observation(&self) -> bool {
        self.observation_strength >= 0.5
    }
}

/// Key for observation cache - uniquely identifies an observation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverKey {
    observer_id: u64,
    target_id: u64,
    context_hash: u64,
}

impl ObserverKey {
    pub fn new(observer_id: u64, target_id: u64, context: &[u8]) -> Self {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        context.hash(&mut hasher);
        let context_hash = hasher.finish();

        ObserverKey {
            observer_id,
            target_id,
            context_hash,
        }
    }

    pub fn simple(observer_id: u64, target_id: u64) -> Self {
        ObserverKey {
            observer_id,
            target_id,
            context_hash: 0,
        }
    }
}

/// Collapsed state - the result of wave function collapse on observation
#[derive(Debug, Clone)]
pub struct CollapsedState {
    state_vector: Vec<Float>,
    collapse_timestamp: u64,
    confidence: Float,
    observer_influence: Float,
}

impl CollapsedState {
    pub fn new(state_vector: Vec<Float>, observer_effect: &ObserverEffect) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let confidence =
            Self::compute_confidence(&state_vector, observer_effect.observation_strength);

        CollapsedState {
            state_vector,
            collapse_timestamp: timestamp,
            confidence,
            observer_influence: observer_effect.observation_strength,
        }
    }
}

impl PartialEq for CollapsedState {
    fn eq(&self, other: &Self) -> bool {
        self.state_vector.len() == other.state_vector.len()
            && self.collapse_timestamp == other.collapse_timestamp
            && (self.confidence - other.confidence).abs() < 1e-9
            && (self.observer_influence - other.observer_influence).abs() < 1e-9
            && self
                .state_vector
                .iter()
                .zip(other.state_vector.iter())
                .all(|(a, b)| (a - b).abs() < 1e-9)
    }
}

impl CollapsedState {
    pub fn state_vector(&self) -> &[Float] {
        &self.state_vector
    }

    pub fn collapse_timestamp(&self) -> u64 {
        self.collapse_timestamp
    }

    pub fn confidence(&self) -> Float {
        self.confidence
    }

    pub fn observer_influence(&self) -> Float {
        self.observer_influence
    }

    pub fn is_fresh(&self, max_age_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now - self.collapse_timestamp < max_age_seconds
    }

    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.5
    }

    fn compute_confidence(state_vector: &[Float], observation_strength: Float) -> Float {
        if state_vector.is_empty() {
            return 0.0;
        }

        // From COSMOLOGICAL-ARCHITECTURE.md: Observation affects probability collapse
        // Confidence is based on the maximum probability amplitude in the collapsed state
        // Higher max amplitude = higher confidence in the collapsed state

        let max_amplitude = state_vector.iter().cloned().fold(0.0f64, f64::max);

        // Also consider observation strength - stronger observations yield higher confidence
        max_amplitude * observation_strength
    }
}

/// Observation cache - stores collapsed states for observed entities
#[derive(Debug, Clone)]
pub struct ObservationCache {
    collapsed_states: HashMap<ObserverKey, CollapsedState>,
    max_cache_size: usize,
    cache_hits: u64,
    cache_misses: u64,
    invalidations: u64,
}

impl ObservationCache {
    pub fn new(max_cache_size: usize) -> Self {
        ObservationCache {
            collapsed_states: HashMap::new(),
            max_cache_size,
            cache_hits: 0,
            cache_misses: 0,
            invalidations: 0,
        }
    }

    pub fn with_capacity(max_cache_size: usize, initial_capacity: usize) -> Self {
        ObservationCache {
            collapsed_states: HashMap::with_capacity(initial_capacity),
            max_cache_size,
            cache_hits: 0,
            cache_misses: 0,
            invalidations: 0,
        }
    }

    pub fn get(&mut self, key: &ObserverKey) -> Option<&CollapsedState> {
        if let Some(state) = self.collapsed_states.get(key) {
            self.cache_hits += 1;
            Some(state)
        } else {
            self.cache_misses += 1;
            None
        }
    }

    pub fn insert(&mut self, key: ObserverKey, state: CollapsedState) {
        if self.collapsed_states.len() >= self.max_cache_size {
            self.evict_oldest();
        }

        self.collapsed_states.insert(key, state);
    }

    pub fn invalidate(&mut self, key: &ObserverKey) -> bool {
        if self.collapsed_states.remove(key).is_some() {
            self.invalidations += 1;
            true
        } else {
            false
        }
    }

    pub fn invalidate_by_target(&mut self, target_id: u64) -> usize {
        let keys_to_remove: Vec<ObserverKey> = self
            .collapsed_states
            .keys()
            .filter(|k| k.target_id == target_id)
            .copied()
            .collect();

        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.collapsed_states.remove(&key);
            self.invalidations += 1;
        }

        count
    }

    pub fn invalidate_by_observer(&mut self, observer_id: u64) -> usize {
        let keys_to_remove: Vec<ObserverKey> = self
            .collapsed_states
            .keys()
            .filter(|k| k.observer_id == observer_id)
            .copied()
            .collect();

        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.collapsed_states.remove(&key);
            self.invalidations += 1;
        }

        count
    }

    pub fn invalidate_all(&mut self) {
        let count = self.collapsed_states.len();
        self.collapsed_states.clear();
        self.invalidations += count as u64;
    }

    pub fn invalidate_old(&mut self, max_age_seconds: u64) -> usize {
        let keys_to_remove: Vec<ObserverKey> = self
            .collapsed_states
            .iter()
            .filter(|(_, state)| !state.is_fresh(max_age_seconds))
            .map(|(key, _)| *key)
            .collect();

        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.collapsed_states.remove(&key);
            self.invalidations += 1;
        }

        count
    }

    pub fn size(&self) -> usize {
        self.collapsed_states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.collapsed_states.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.collapsed_states.len() >= self.max_cache_size
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self
            .collapsed_states
            .iter()
            .min_by_key(|(_, state)| state.collapse_timestamp)
            .map(|(key, _)| *key)
        {
            self.collapsed_states.remove(&oldest_key);
        }
    }

    pub fn hit_rate(&self) -> Float {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        self.cache_hits as Float / total as Float
    }

    pub fn statistics(&self) -> ObservationCacheStatistics {
        ObservationCacheStatistics {
            cache_size: self.collapsed_states.len(),
            max_cache_size: self.max_cache_size,
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            invalidations: self.invalidations,
            hit_rate: self.hit_rate(),
        }
    }
}

impl Default for ObservationCache {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Observation engine - orchestrates observation and collapse
#[derive(Debug, Clone)]
pub struct ObservationEngine {
    cache: ObservationCache,
    #[allow(dead_code)]
    collapse_threshold: Float,
}

impl ObservationEngine {
    pub fn new(cache_size: usize) -> Self {
        ObservationEngine {
            cache: ObservationCache::new(cache_size),
            collapse_threshold: 0.5,
        }
    }

    pub fn with_threshold(cache_size: usize, collapse_threshold: Float) -> Self {
        ObservationEngine {
            cache: ObservationCache::new(cache_size),
            collapse_threshold: collapse_threshold.clamp(0.0, 1.0),
        }
    }

    pub fn observe(
        &mut self,
        observer_effect: ObserverEffect,
        possibility_space: &[Float],
    ) -> Result<CollapsedState, ObserverEffectError> {
        if possibility_space.is_empty() {
            return Err(ObserverEffectError::EmptyPossibilitySpace);
        }

        let key = ObserverKey::simple(observer_effect.observer_id(), observer_effect.target_id());

        if let Some(cached) = self.cache.get(&key) {
            if cached.is_fresh(60) && cached.is_high_confidence() {
                return Ok(cached.clone());
            }
        }

        let collapsed = self.collapse_wavefunction(possibility_space, &observer_effect)?;
        self.cache.insert(key, collapsed.clone());
        Ok(collapsed)
    }

    pub fn collapse_wavefunction(
        &self,
        possibility_space: &[Float],
        observer_effect: &ObserverEffect,
    ) -> Result<CollapsedState, ObserverEffectError> {
        if possibility_space.is_empty() {
            return Err(ObserverEffectError::EmptyPossibilitySpace);
        }

        let total_probability: Float = possibility_space.iter().sum();

        if total_probability <= 0.0 {
            return Err(ObserverEffectError::InvalidProbabilitySpace);
        }

        let mut normalized_space: Vec<Float> = possibility_space
            .iter()
            .map(|&p| p / total_probability)
            .collect();

        self.apply_observer_influence(&mut normalized_space, observer_effect);

        Ok(CollapsedState::new(normalized_space, observer_effect))
    }

    pub fn apply_observation(&mut self, observer_effect: ObserverEffect) -> usize {
        self.cache.invalidate_by_target(observer_effect.target_id())
    }

    pub fn invalidate_observation(&mut self, observer_effect: ObserverEffect) -> bool {
        let key = ObserverKey::simple(observer_effect.observer_id(), observer_effect.target_id());
        self.cache.invalidate(&key)
    }

    pub fn invalidate_target(&mut self, target_id: u64) -> usize {
        self.cache.invalidate_by_target(target_id)
    }

    pub fn invalidate_observer(&mut self, observer_id: u64) -> usize {
        self.cache.invalidate_by_observer(observer_id)
    }

    pub fn invalidate_old_observations(&mut self, max_age_seconds: u64) -> usize {
        self.cache.invalidate_old(max_age_seconds)
    }

    pub fn clear_cache(&mut self) {
        self.cache.invalidate_all();
    }

    pub fn cache(&self) -> &ObservationCache {
        &self.cache
    }

    pub fn cache_mut(&mut self) -> &mut ObservationCache {
        &mut self.cache
    }

    fn apply_observer_influence(&self, state: &mut [Float], observer_effect: &ObserverEffect) {
        let influence = observer_effect.observation_strength();

        for val in state.iter_mut() {
            *val = *val * (1.0 - influence) + *val * influence;
        }

        let max_val = state.iter().cloned().fold(0.0f64, f64::max);
        if max_val > 0.0 {
            for val in state.iter_mut() {
                *val /= max_val;
            }
        }
    }
}

impl Default for ObservationEngine {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Statistics for observation cache
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationCacheStatistics {
    pub cache_size: usize,
    pub max_cache_size: usize,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub invalidations: u64,
    pub hit_rate: Float,
}

impl ObservationCacheStatistics {
    pub fn total_requests(&self) -> u64 {
        self.cache_hits + self.cache_misses
    }

    pub fn cache_utilization(&self) -> Float {
        if self.max_cache_size == 0 {
            return 0.0;
        }
        self.cache_size as Float / self.max_cache_size as Float
    }
}

/// Errors for observer effect operations
#[derive(Debug, Clone, PartialEq)]
pub enum ObserverEffectError {
    EmptyPossibilitySpace,
    InvalidProbabilitySpace,
    InvalidObserverEffect,
    CollapseFailed(String),
}

impl std::fmt::Display for ObserverEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObserverEffectError::EmptyPossibilitySpace => {
                write!(f, "Possibility space is empty")
            }
            ObserverEffectError::InvalidProbabilitySpace => {
                write!(f, "Probability space is invalid")
            }
            ObserverEffectError::InvalidObserverEffect => {
                write!(f, "Observer effect is invalid")
            }
            ObserverEffectError::CollapseFailed(msg) => {
                write!(f, "Wave function collapse failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for ObserverEffectError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_effect_creation() {
        let effect = ObserverEffect::new(1, 2);
        assert_eq!(effect.observer_id(), 1);
        assert_eq!(effect.target_id(), 2);
        assert_eq!(effect.observation_strength(), 1.0);
    }

    #[test]
    fn test_observer_effect_with_strength() {
        let effect = ObserverEffect::new(1, 2).with_strength(0.75);
        assert_eq!(effect.observation_strength(), 0.75);
    }

    #[test]
    fn test_observer_key_creation() {
        let key = ObserverKey::new(1, 2, b"context");
        assert_eq!(key.observer_id, 1);
        assert_eq!(key.target_id, 2);
    }

    #[test]
    fn test_observer_key_simple() {
        let key = ObserverKey::simple(1, 2);
        assert_eq!(key.observer_id, 1);
        assert_eq!(key.target_id, 2);
        assert_eq!(key.context_hash, 0);
    }

    #[test]
    fn test_collapsed_state_creation() {
        let effect = ObserverEffect::new(1, 2);
        let state = vec![0.5, 0.3, 0.2];
        let collapsed = CollapsedState::new(state.clone(), &effect);

        assert_eq!(collapsed.state_vector(), &state);
        assert!(collapsed.confidence() >= 0.0 && collapsed.confidence() <= 1.0);
        assert_eq!(collapsed.observer_influence(), 1.0);
    }

    #[test]
    fn test_collapsed_state_freshness() {
        let effect = ObserverEffect::new(1, 2);
        let state = vec![0.5, 0.3, 0.2];
        let collapsed = CollapsedState::new(state, &effect);

        assert!(collapsed.is_fresh(60));
        assert!(collapsed.is_fresh(100000));
    }

    #[test]
    fn test_collapsed_state_high_confidence() {
        let effect = ObserverEffect::new(1, 2);
        let state = vec![0.8, 0.6, 0.4];
        let collapsed = CollapsedState::new(state, &effect);

        assert!(collapsed.is_high_confidence());
    }

    #[test]
    fn test_observation_cache_creation() {
        let cache = ObservationCache::new(100);
        assert_eq!(cache.max_cache_size, 100);
        assert!(cache.is_empty());
        assert!(!cache.is_full());
    }

    #[test]
    fn test_observation_cache_insert_and_get() {
        let mut cache = ObservationCache::new(100);
        let key = ObserverKey::simple(1, 2);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(key, state.clone());
        assert_eq!(cache.size(), 1);

        let retrieved = cache.get(&key);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &state);
    }

    #[test]
    fn test_observation_cache_hit_rate() {
        let mut cache = ObservationCache::new(100);
        let key = ObserverKey::simple(1, 2);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(key, state);
        cache.get(&key);
        cache.get(&key);
        cache.get(&ObserverKey::simple(3, 4));

        assert_eq!(cache.cache_hits, 2);
        assert_eq!(cache.cache_misses, 1);
        assert!((cache.hit_rate() - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_observation_cache_invalidate() {
        let mut cache = ObservationCache::new(100);
        let key = ObserverKey::simple(1, 2);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(key, state);
        assert_eq!(cache.size(), 1);

        let invalidated = cache.invalidate(&key);
        assert!(invalidated);
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_observation_cache_invalidate_by_target() {
        let mut cache = ObservationCache::new(100);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(ObserverKey::simple(1, 2), state.clone());
        cache.insert(ObserverKey::simple(3, 2), state.clone());
        cache.insert(ObserverKey::simple(4, 5), state.clone());

        assert_eq!(cache.size(), 3);

        let count = cache.invalidate_by_target(2);
        assert_eq!(count, 2);
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_observation_cache_invalidate_by_observer() {
        let mut cache = ObservationCache::new(100);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(ObserverKey::simple(1, 2), state.clone());
        cache.insert(ObserverKey::simple(1, 3), state.clone());
        cache.insert(ObserverKey::simple(4, 5), state.clone());

        assert_eq!(cache.size(), 3);

        let count = cache.invalidate_by_observer(1);
        assert_eq!(count, 2);
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_observation_cache_eviction() {
        let mut cache = ObservationCache::new(2);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(ObserverKey::simple(1, 2), state.clone());
        cache.insert(ObserverKey::simple(3, 4), state.clone());
        assert_eq!(cache.size(), 2);
        assert!(cache.is_full());

        cache.insert(ObserverKey::simple(5, 6), state);
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn test_observation_engine_creation() {
        let engine = ObservationEngine::new(100);
        assert_eq!(engine.cache().max_cache_size, 100);
        assert_eq!(engine.collapse_threshold, 0.5);
    }

    #[test]
    fn test_observation_engine_collapse_wavefunction() {
        let engine = ObservationEngine::new(100);
        let effect = ObserverEffect::new(1, 2);
        let possibility = vec![0.3, 0.5, 0.2];

        let result = engine.collapse_wavefunction(&possibility, &effect);
        assert!(result.is_ok());

        let collapsed = result.unwrap();
        assert_eq!(collapsed.state_vector().len(), 3);
        assert!(collapsed.is_high_confidence());
    }

    #[test]
    fn test_observation_engine_collapse_empty_space() {
        let engine = ObservationEngine::new(100);
        let effect = ObserverEffect::new(1, 2);
        let possibility: Vec<Float> = vec![];

        let result = engine.collapse_wavefunction(&possibility, &effect);
        assert!(matches!(
            result,
            Err(ObserverEffectError::EmptyPossibilitySpace)
        ));
    }

    #[test]
    fn test_observation_engine_observe() {
        let mut engine = ObservationEngine::new(100);
        let effect = ObserverEffect::new(1, 2);
        let possibility = vec![0.3, 0.5, 0.2];

        let result = engine.observe(effect, &possibility);
        assert!(result.is_ok());

        let collapsed = result.unwrap();
        assert_eq!(engine.cache().size(), 1);
        assert_eq!(collapsed.state_vector().len(), 3);
    }

    #[test]
    fn test_observation_engine_observe_cached() {
        let mut engine = ObservationEngine::new(100);
        let effect = ObserverEffect::new(1, 2);
        let possibility = vec![0.3, 0.5, 0.2];

        let result1 = engine.observe(effect.clone(), &possibility);
        assert!(result1.is_ok());

        let result2 = engine.observe(effect, &possibility);
        assert!(result2.is_ok());

        assert_eq!(engine.cache().cache_hits, 1);
        assert_eq!(engine.cache().cache_misses, 1);
    }

    #[test]
    fn test_observation_engine_apply_observation() {
        let mut engine = ObservationEngine::new(100);
        let effect = ObserverEffect::new(1, 2);
        let possibility = vec![0.3, 0.5, 0.2];

        engine.observe(effect, &possibility).unwrap();
        assert_eq!(engine.cache().size(), 1);

        let invalidation_effect = ObserverEffect::new(3, 2);
        let count = engine.apply_observation(invalidation_effect);
        assert_eq!(count, 1);
        assert_eq!(engine.cache().size(), 0);
    }

    #[test]
    fn test_observation_engine_invalidate_target() {
        let mut engine = ObservationEngine::new(100);
        let _effect = ObserverEffect::new(1, 2);
        let possibility = vec![0.3, 0.5, 0.2];

        engine
            .observe(ObserverEffect::new(1, 2), &possibility)
            .unwrap();
        engine
            .observe(ObserverEffect::new(3, 2), &possibility)
            .unwrap();
        engine
            .observe(ObserverEffect::new(4, 5), &possibility)
            .unwrap();

        assert_eq!(engine.cache().size(), 3);

        let count = engine.invalidate_target(2);
        assert_eq!(count, 2);
        assert_eq!(engine.cache().size(), 1);
    }

    #[test]
    fn test_observation_cache_statistics() {
        let mut cache = ObservationCache::new(100);
        let key = ObserverKey::simple(1, 2);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(key, state);
        cache.get(&key);
        cache.get(&key);
        cache.get(&ObserverKey::simple(3, 4));

        let stats = cache.statistics();
        assert_eq!(stats.cache_size, 1);
        assert_eq!(stats.max_cache_size, 100);
        assert_eq!(stats.cache_hits, 2);
        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.invalidations, 0);
        assert!((stats.hit_rate - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_error_display() {
        let err = ObserverEffectError::EmptyPossibilitySpace;
        assert_eq!(err.to_string(), "Possibility space is empty");

        let err = ObserverEffectError::InvalidProbabilitySpace;
        assert_eq!(err.to_string(), "Probability space is invalid");

        let err = ObserverEffectError::InvalidObserverEffect;
        assert_eq!(err.to_string(), "Observer effect is invalid");

        let err = ObserverEffectError::CollapseFailed("test".to_string());
        assert_eq!(err.to_string(), "Wave function collapse failed: test");
    }

    #[test]
    fn test_observation_effect_strength_clamping() {
        let effect = ObserverEffect::new(1, 2).with_strength(1.5);
        assert_eq!(effect.observation_strength(), 1.0);

        let effect = ObserverEffect::new(1, 2).with_strength(-0.5);
        assert_eq!(effect.observation_strength(), 0.0);
    }

    #[test]
    fn test_observation_cache_with_capacity() {
        let cache = ObservationCache::with_capacity(100, 50);
        assert_eq!(cache.max_cache_size, 100);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_observation_engine_with_threshold() {
        let engine = ObservationEngine::with_threshold(100, 0.75);
        assert_eq!(engine.collapse_threshold, 0.75);
    }

    #[test]
    fn test_observation_cache_invalidate_all() {
        let mut cache = ObservationCache::new(100);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        cache.insert(ObserverKey::simple(1, 2), state.clone());
        cache.insert(ObserverKey::simple(3, 4), state.clone());

        assert_eq!(cache.size(), 2);

        cache.invalidate_all();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.invalidations, 2);
    }

    #[test]
    fn test_observation_engine_clear_cache() {
        let mut engine = ObservationEngine::new(100);
        let _effect = ObserverEffect::new(1, 2);
        let possibility = vec![0.3, 0.5, 0.2];

        engine
            .observe(ObserverEffect::new(1, 2), &possibility)
            .unwrap();
        engine
            .observe(ObserverEffect::new(3, 4), &possibility)
            .unwrap();

        assert_eq!(engine.cache().size(), 2);

        engine.clear_cache();
        assert_eq!(engine.cache().size(), 0);
    }

    #[test]
    fn test_observation_cache_statistics_utilization() {
        let mut cache = ObservationCache::new(10);
        let effect = ObserverEffect::new(1, 2);
        let state = CollapsedState::new(vec![0.5, 0.3, 0.2], &effect);

        for i in 0..5 {
            cache.insert(ObserverKey::simple(i, i + 1), state.clone());
        }

        let stats = cache.statistics();
        assert_eq!(stats.cache_size, 5);
        assert_eq!(stats.max_cache_size, 10);
        assert!((stats.cache_utilization() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_collapsed_state_confidence_computation() {
        let effect = ObserverEffect::new(1, 2);
        let high_magnitude = vec![0.9, 0.8, 0.7];
        let low_magnitude = vec![0.1, 0.1, 0.1];

        let high = CollapsedState::new(high_magnitude, &effect);
        let low = CollapsedState::new(low_magnitude, &effect);

        assert!(high.confidence() > low.confidence());
    }
}
