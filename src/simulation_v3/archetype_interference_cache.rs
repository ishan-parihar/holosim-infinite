//! Archetypical Interference Caching - O(1) access for archetype profile interference
//!
//! From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
//! "Cache interference patterns for archetype profiles to enable O(1) access
//! for similar profiles"
//!
//! This module provides:
//! - ArchetypeInterferenceCache: Cache interference patterns by profile
//! - ArchetypeKey: Quantized profile key for cache lookup
//! - ArchetypicalInterference: Interference pattern result
//! - Profile quantization for cache clustering
//! - Cache hit/miss statistics
//! - O(1) access for similar profiles

use crate::simulation_v3::archetype_basis::{ArchetypeActivationProfile, NUM_ARCHETYPES};
use crate::types::Float;
use std::collections::HashMap;

/// Quantization levels for profile coefficients
pub const QUANTIZATION_LEVELS: u8 = 64;

/// Maximum cache size (number of entries)
pub const MAX_CACHE_SIZE: usize = 10000;

/// Cache key for archetype profiles (quantized coefficients)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArchetypeKey {
    /// Quantized coefficients (22 values, 0-63 each)
    quantized: [u8; NUM_ARCHETYPES],
}

impl ArchetypeKey {
    /// Create a new cache key from quantized coefficients
    pub fn new(quantized: [u8; NUM_ARCHETYPES]) -> Self {
        ArchetypeKey { quantized }
    }

    /// Create cache key from bytes
    pub fn from_bytes(bytes: [u8; NUM_ARCHETYPES]) -> Self {
        ArchetypeKey { quantized: bytes }
    }

    /// Get quantized coefficients
    pub fn quantized(&self) -> &[u8; NUM_ARCHETYPES] {
        &self.quantized
    }

    /// Get as u128 hash (for compact storage)
    pub fn to_u128(&self) -> u128 {
        let mut hash: u128 = 0;
        for (i, &val) in self.quantized.iter().enumerate() {
            hash |= (val as u128) << (i * 6);
        }
        hash
    }

    /// Compute Hamming distance between two keys (similarity metric)
    pub fn hamming_distance(&self, other: &ArchetypeKey) -> usize {
        self.quantized
            .iter()
            .zip(other.quantized.iter())
            .filter(|(a, b)| a != b)
            .count()
    }
}

/// Archetypical interference pattern result
#[derive(Debug, Clone, PartialEq)]
pub struct ArchetypicalInterference {
    /// Interference pattern components
    pattern: Vec<Float>,
    /// Interference magnitude
    magnitude: Float,
    /// Dominant interference frequency
    dominant_frequency: Float,
    /// Interference phase
    phase: Float,
}

impl ArchetypicalInterference {
    /// Create a new interference pattern
    pub fn new(pattern: Vec<Float>) -> Self {
        let magnitude = pattern.iter().map(|x| x * x).sum::<Float>().sqrt();
        let dominant_frequency = Self::compute_dominant_frequency(&pattern);
        let phase = Self::compute_phase(&pattern);

        ArchetypicalInterference {
            pattern,
            magnitude,
            dominant_frequency,
            phase,
        }
    }

    /// Create zero interference pattern
    pub fn zero(dimension: usize) -> Self {
        ArchetypicalInterference {
            pattern: vec![0.0; dimension],
            magnitude: 0.0,
            dominant_frequency: 0.0,
            phase: 0.0,
        }
    }

    /// Get reference to pattern components
    pub fn pattern(&self) -> &[Float] {
        &self.pattern
    }

    /// Get interference magnitude
    pub fn magnitude(&self) -> Float {
        self.magnitude
    }

    /// Get dominant frequency
    pub fn dominant_frequency(&self) -> Float {
        self.dominant_frequency
    }

    /// Get interference phase
    pub fn phase(&self) -> Float {
        self.phase
    }

    /// Compute dominant frequency from pattern
    fn compute_dominant_frequency(pattern: &[Float]) -> Float {
        if pattern.is_empty() {
            return 0.0;
        }

        let n = pattern.len();
        let mut max_freq = 0.0;
        let mut max_power = 0.0;

        for k in 1..=n.min(10) {
            let freq = k as Float;
            let mut real_sum = 0.0;
            let mut imag_sum = 0.0;
            for (i, &val) in pattern.iter().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * freq * (i as Float) / (n as Float);
                real_sum += val * angle.cos();
                imag_sum += val * angle.sin();
            }
            // Compute magnitude of complex sum
            let power = (real_sum * real_sum + imag_sum * imag_sum).sqrt() / n as Float;
            if power > max_power {
                max_power = power;
                max_freq = freq;
            }
        }

        max_freq
    }

    /// Compute phase from pattern
    fn compute_phase(pattern: &[Float]) -> Float {
        if pattern.is_empty() {
            return 0.0;
        }

        let n = pattern.len();
        let mut real_sum = 0.0;
        let mut imag_sum = 0.0;

        for (i, &val) in pattern.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * (i as Float) / (n as Float);
            real_sum += val * angle.cos();
            imag_sum += val * angle.sin();
        }

        imag_sum.atan2(real_sum)
    }

    /// Add interference patterns (for accumulation)
    pub fn add(&mut self, other: &ArchetypicalInterference) {
        assert_eq!(
            self.pattern.len(),
            other.pattern.len(),
            "Pattern dimensions must match"
        );
        for (a, b) in self.pattern.iter_mut().zip(other.pattern.iter()) {
            *a += b;
        }
        self.magnitude = self.pattern.iter().map(|x| x * x).sum::<Float>().sqrt();
    }
}

/// Error types for archetype interference cache
#[derive(Debug, Clone, PartialEq)]
pub enum ArchetypeCacheError {
    InvalidProfile,
    CacheFull,
    QuantizationError,
}

impl std::fmt::Display for ArchetypeCacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchetypeCacheError::InvalidProfile => write!(f, "Invalid profile"),
            ArchetypeCacheError::CacheFull => write!(f, "Cache is full"),
            ArchetypeCacheError::QuantizationError => write!(f, "Quantization error"),
        }
    }
}

impl std::error::Error for ArchetypeCacheError {}

/// Statistics for archetype interference cache
#[derive(Debug, Clone, Default)]
pub struct ArchetypeCacheStatistics {
    /// Total cache lookups
    pub total_lookups: usize,
    /// Cache hits
    pub cache_hits: usize,
    /// Cache misses
    pub cache_misses: usize,
    /// Hit rate (0.0-1.0)
    pub hit_rate: Float,
    /// Current cache size
    pub current_size: usize,
    /// Evictions performed
    pub evictions: usize,
    /// Total computation time (microseconds)
    pub total_computation_time_us: Float,
    /// Average computation time (microseconds)
    pub average_computation_time_us: Float,
}

impl ArchetypeCacheStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_hit(&mut self) {
        self.cache_hits += 1;
        self.total_lookups += 1;
        self.update_hit_rate();
    }

    pub fn record_miss(&mut self) {
        self.cache_misses += 1;
        self.total_lookups += 1;
        self.update_hit_rate();
    }

    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    pub fn record_computation(&mut self, duration_us: Float) {
        self.total_computation_time_us += duration_us;
        if self.cache_misses > 0 {
            self.average_computation_time_us =
                self.total_computation_time_us / self.cache_misses as Float;
        }
    }

    pub fn update_size(&mut self, size: usize) {
        self.current_size = size;
    }

    fn update_hit_rate(&mut self) {
        if self.total_lookups > 0 {
            self.hit_rate = self.cache_hits as Float / self.total_lookups as Float;
        }
    }
}

/// Archetype interference cache - O(1) access for similar profiles
///
/// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
/// "First access: O(1) compute + cache"
/// "Subsequent access: O(1) cache hit"
/// "Clustering benefit: Entities with similar profiles share cache entries"
#[derive(Debug, Clone)]
pub struct ArchetypeInterferenceCache {
    /// Cache entries
    entries: HashMap<ArchetypeKey, ArchetypicalInterference>,
    /// Cache statistics
    statistics: ArchetypeCacheStatistics,
    /// Maximum cache size
    max_size: usize,
    /// Pattern dimension
    pattern_dimension: usize,
}

impl ArchetypeInterferenceCache {
    /// Create a new interference cache
    pub fn new(max_size: usize, pattern_dimension: usize) -> Self {
        ArchetypeInterferenceCache {
            entries: HashMap::new(),
            statistics: ArchetypeCacheStatistics::new(),
            max_size: max_size.min(MAX_CACHE_SIZE),
            pattern_dimension,
        }
    }

    /// Create cache with default settings
    pub fn with_default_dimension(pattern_dimension: usize) -> Self {
        Self::new(MAX_CACHE_SIZE, pattern_dimension)
    }

    /// Get or compute interference pattern for a profile
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "First access: O(1) compute + cache"
    /// "Subsequent access: O(1) cache hit"
    pub fn get(&mut self, profile: &ArchetypeActivationProfile) -> ArchetypicalInterference {
        let key = self.quantize_key(profile);

        if let Some(cached) = self.entries.get(&key) {
            self.statistics.record_hit();
            return cached.clone();
        }

        self.statistics.record_miss();

        let start = std::time::Instant::now();
        let interference = self.compute_interference(profile);
        let duration = start.elapsed().as_micros() as Float;
        self.statistics.record_computation(duration);

        if self.entries.len() >= self.max_size {
            self.evict_oldest();
        }

        self.entries.insert(key, interference.clone());
        self.statistics.update_size(self.entries.len());

        interference
    }

    /// Check if profile has cached interference
    pub fn contains(&self, profile: &ArchetypeActivationProfile) -> bool {
        let key = self.quantize_key(profile);
        self.entries.contains_key(&key)
    }

    /// Get interference pattern without computing (if cached)
    pub fn get_cached(
        &self,
        profile: &ArchetypeActivationProfile,
    ) -> Option<ArchetypicalInterference> {
        let key = self.quantize_key(profile);
        self.entries.get(&key).cloned()
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.statistics = ArchetypeCacheStatistics::new();
    }

    /// Get cache statistics
    pub fn statistics(&self) -> &ArchetypeCacheStatistics {
        &self.statistics
    }

    /// Get current cache size
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Get maximum cache size
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// Evict oldest entries (LRU-style, using first key)
    fn evict_oldest(&mut self) {
        if let Some(key) = self.entries.keys().next().cloned() {
            self.entries.remove(&key);
            self.statistics.record_eviction();
        }
    }

    /// Quantize profile to create cache key
    ///
    /// From HOLOGRAPHIC_OPTIMIZATION_FRAMEWORK.md:
    /// "Quantize coefficients to create cache keys"
    /// "Similar profiles will map to same key"
    fn quantize_key(&self, profile: &ArchetypeActivationProfile) -> ArchetypeKey {
        let coefficients = profile.coefficients();

        let mut quantized = [0u8; NUM_ARCHETYPES];
        for (i, &coeff) in coefficients.iter().enumerate() {
            let normalized = coeff.clamp(0.0, 1.0);
            // Use rounding instead of truncation for better quantization accuracy
            quantized[i] = (normalized * (QUANTIZATION_LEVELS - 1) as Float).round() as u8;
        }

        ArchetypeKey::new(quantized)
    }

    /// Compute interference pattern from profile
    ///
    /// This creates an interference pattern by combining the archetype
    /// coefficients with sinusoidal interference functions
    fn compute_interference(
        &self,
        profile: &ArchetypeActivationProfile,
    ) -> ArchetypicalInterference {
        let coefficients = profile.coefficients();
        let pattern: Vec<Float> = (0..self.pattern_dimension)
            .map(|i| {
                coefficients
                    .iter()
                    .enumerate()
                    .map(|(j, &coeff)| {
                        let freq = (j + 1) as Float;
                        let phase = (j * 7) as Float;
                        let angle = 2.0 * std::f64::consts::PI * freq * (i as Float)
                            / (self.pattern_dimension as Float)
                            + phase;
                        coeff * angle.sin()
                    })
                    .sum()
            })
            .collect();

        ArchetypicalInterference::new(pattern)
    }

    /// Compute similarity between two profiles (0.0-1.0)
    pub fn similarity(
        &self,
        profile_a: &ArchetypeActivationProfile,
        profile_b: &ArchetypeActivationProfile,
    ) -> Float {
        let key_a = self.quantize_key(profile_a);
        let key_b = self.quantize_key(profile_b);

        let distance = key_a.hamming_distance(&key_b);
        let max_distance = NUM_ARCHETYPES;

        1.0 - (distance as Float / max_distance as Float)
    }

    /// Get all cache keys
    pub fn keys(&self) -> Vec<ArchetypeKey> {
        self.entries.keys().cloned().collect()
    }

    /// Get cache entry count for specific quantized value (bucket analysis)
    pub fn bucket_count(&self, archetype_index: usize, quantized_value: u8) -> usize {
        self.entries
            .keys()
            .filter(|key| key.quantized()[archetype_index] == quantized_value)
            .count()
    }
}

impl Default for ArchetypeInterferenceCache {
    fn default() -> Self {
        Self::with_default_dimension(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_profile() -> ArchetypeActivationProfile {
        let mut coefficients = [0.0; NUM_ARCHETYPES];
        coefficients[0] = 0.5;
        coefficients[5] = 0.3;
        coefficients[10] = 0.2;
        ArchetypeActivationProfile::new(coefficients)
    }

    #[test]
    fn test_archetype_key_creation() {
        let quantized = [0u8; NUM_ARCHETYPES];
        let key = ArchetypeKey::new(quantized);
        assert_eq!(key.quantized().len(), NUM_ARCHETYPES);
    }

    #[test]
    fn test_archetype_key_from_bytes() {
        let bytes = [42u8; NUM_ARCHETYPES];
        let key = ArchetypeKey::from_bytes(bytes);
        assert_eq!(key.quantized(), &bytes);
    }

    #[test]
    fn test_archetype_key_to_u128() {
        let mut bytes = [0u8; NUM_ARCHETYPES];
        bytes[0] = 1;
        bytes[1] = 2;
        let key = ArchetypeKey::from_bytes(bytes);
        let hash = key.to_u128();
        assert!(hash > 0);
    }

    #[test]
    fn test_archetype_key_hamming_distance() {
        let key1 = ArchetypeKey::from_bytes([0u8; NUM_ARCHETYPES]);
        let mut bytes = [0u8; NUM_ARCHETYPES];
        bytes[5] = 1;
        bytes[10] = 2;
        let key2 = ArchetypeKey::from_bytes(bytes);
        let distance = key1.hamming_distance(&key2);
        assert_eq!(distance, 2);
    }

    #[test]
    fn test_archetypical_interference_creation() {
        let pattern = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let interference = ArchetypicalInterference::new(pattern.clone());
        assert_eq!(interference.pattern(), &pattern);
        assert!(interference.magnitude() > 0.0);
    }

    #[test]
    fn test_archetypical_interference_zero() {
        let interference = ArchetypicalInterference::zero(100);
        assert_eq!(interference.magnitude(), 0.0);
        assert_eq!(interference.pattern().len(), 100);
    }

    #[test]
    fn test_archetypical_interference_add() {
        let mut interference1 = ArchetypicalInterference::new(vec![1.0, 2.0, 3.0]);
        let interference2 = ArchetypicalInterference::new(vec![4.0, 5.0, 6.0]);
        interference1.add(&interference2);
        assert_eq!(interference1.pattern(), &[5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_archetype_cache_creation() {
        let cache = ArchetypeInterferenceCache::new(100, 50);
        assert_eq!(cache.max_size(), 100);
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_archetype_cache_get_computes_on_first_access() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();
        let interference = cache.get(&profile);
        assert_eq!(interference.pattern().len(), 100);
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.statistics().cache_misses, 1);
    }

    #[test]
    fn test_archetype_cache_get_hits_on_second_access() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();
        cache.get(&profile);
        cache.get(&profile);
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.statistics().cache_hits, 1);
        assert_eq!(cache.statistics().cache_misses, 1);
    }

    #[test]
    fn test_archetype_cache_contains() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();
        assert!(!cache.contains(&profile));
        cache.get(&profile);
        assert!(cache.contains(&profile));
    }

    #[test]
    fn test_archetype_cache_get_cached() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();
        assert!(cache.get_cached(&profile).is_none());
        cache.get(&profile);
        assert!(cache.get_cached(&profile).is_some());
    }

    #[test]
    fn test_archetype_cache_clear() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();
        cache.get(&profile);
        assert_eq!(cache.size(), 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.statistics().total_lookups, 0);
    }

    #[test]
    fn test_archetype_cache_similarity() {
        let cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile1 = create_test_profile();
        let profile2 = create_test_profile();
        let similarity = cache.similarity(&profile1, &profile2);
        assert_eq!(similarity, 1.0);
    }

    #[test]
    fn test_archetype_cache_quantization() {
        let cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let mut profile = create_test_profile();
        profile.set(0, 0.5).unwrap();
        let key1 = cache.quantize_key(&profile);

        profile.set(0, 0.51).unwrap();
        let key2 = cache.quantize_key(&profile);

        profile.set(0, 0.99).unwrap();
        let key3 = cache.quantize_key(&profile);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_archetype_cache_statistics() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();

        cache.get(&profile);
        cache.get(&profile);
        cache.get(&profile);

        let stats = cache.statistics();
        assert_eq!(stats.total_lookups, 3);
        assert_eq!(stats.cache_hits, 2);
        assert_eq!(stats.cache_misses, 1);
        assert!((stats.hit_rate - 0.6667).abs() < 0.01);
    }

    #[test]
    fn test_archetype_cache_bucket_count() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let mut profile = create_test_profile();
        profile.set(0, 0.5).unwrap();
        cache.get(&profile);

        // Create a new profile with different archetype value but same archetype 0 value
        let mut profile2 = create_test_profile();
        profile2.set(0, 0.51).unwrap(); // Quantizes to 32
        profile2.set(5, 0.6).unwrap(); // Different from 0.3, creates different cache key
        cache.get(&profile2);

        let count = cache.bucket_count(0, 32);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_archetype_cache_computation_time() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile = create_test_profile();
        cache.get(&profile);

        let stats = cache.statistics();
        assert!(stats.total_computation_time_us > 0.0);
        assert!(stats.average_computation_time_us > 0.0);
    }

    #[test]
    fn test_archetype_cache_eviction() {
        let mut cache = ArchetypeInterferenceCache::new(3, 100);
        let mut profile = create_test_profile();

        for i in 0..5 {
            profile.set(0, (i as Float) / 4.0).unwrap();
            cache.get(&profile);
        }

        assert_eq!(cache.size(), 3);
        assert_eq!(cache.statistics().evictions, 2);
    }

    #[test]
    fn test_archetype_cache_default() {
        let cache = ArchetypeInterferenceCache::default();
        assert_eq!(cache.max_size(), MAX_CACHE_SIZE);
        assert_eq!(cache.pattern_dimension, 100);
    }

    #[test]
    fn test_archetype_cache_statistics_new() {
        let stats = ArchetypeCacheStatistics::new();
        assert_eq!(stats.total_lookups, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
        assert_eq!(stats.hit_rate, 0.0);
    }

    #[test]
    fn test_archetype_cache_statistics_update() {
        let mut stats = ArchetypeCacheStatistics::new();
        stats.record_hit();
        stats.record_hit();
        stats.record_miss();
        stats.record_computation(100.0);
        stats.record_computation(200.0);
        stats.update_size(5);

        assert_eq!(stats.total_lookups, 3);
        assert_eq!(stats.cache_hits, 2);
        assert_eq!(stats.cache_misses, 1);
        assert!((stats.hit_rate - 0.6667).abs() < 0.01);
        assert_eq!(stats.total_computation_time_us, 300.0);
        assert_eq!(stats.average_computation_time_us, 300.0);
        assert_eq!(stats.current_size, 5);
    }

    #[test]
    fn test_archetype_cache_error_display() {
        let err = ArchetypeCacheError::InvalidProfile;
        assert_eq!(err.to_string(), "Invalid profile");

        let err = ArchetypeCacheError::CacheFull;
        assert_eq!(err.to_string(), "Cache is full");

        let err = ArchetypeCacheError::QuantizationError;
        assert_eq!(err.to_string(), "Quantization error");
    }

    #[test]
    fn test_archetype_cache_keys() {
        let mut cache = ArchetypeInterferenceCache::with_default_dimension(100);
        let profile1 = create_test_profile();
        let profile2 = create_test_profile();

        cache.get(&profile1);
        cache.get(&profile2);

        let keys = cache.keys();
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn test_archetypical_interference_phase() {
        let pattern = vec![0.0, 1.0, 0.0, -1.0];
        let interference = ArchetypicalInterference::new(pattern);
        let phase = interference.phase();
        assert!((-std::f64::consts::PI..=std::f64::consts::PI).contains(&phase));
    }

    #[test]
    fn test_archetypical_interference_dominant_frequency() {
        let n = 100;
        let mut pattern = vec![0.0; n];
        for (i, item) in pattern.iter_mut().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * 3.0 * (i as Float) / (n as Float);
            *item = angle.sin();
        }
        let interference = ArchetypicalInterference::new(pattern);
        let freq = interference.dominant_frequency();
        assert!((freq - 3.0).abs() < 1.0);
    }
}
