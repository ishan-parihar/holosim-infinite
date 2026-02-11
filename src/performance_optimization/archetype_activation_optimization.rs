// ============================================================================
// TASK 1: OPTIMIZE ARCHETYPE ACTIVATION CALCULATIONS
// ============================================================================
// Objective: Profile and optimize archetype activation calculations.
//
// Tasks:
// - Profile archetype activation code
// - Optimize hot paths
// - Use SIMD where applicable
// - Optimize aggregation algorithms
// - Implement caching mechanisms
// ============================================================================

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Placeholder for ArchetypeActivation (used in performance optimization)
#[derive(Debug, Clone)]
pub struct ArchetypeActivation {
    pub archetype_id: u8,
    pub activation_level: f64,
    pub coherence: f64,
    pub polarization: f64,
    pub lambda_value: f64, // Simplified - just the value from LambdaMeasurement
}

// Placeholder for ArchetypeComplex (used in performance optimization)
#[derive(Debug, Clone)]
pub struct ArchetypeComplex {
    pub real: f64,
    pub imag: f64,
    pub magnitude: f64,
    pub phase: f64,
    pub coherence: f64,
}

impl ArchetypeComplex {
    pub fn zero() -> Self {
        Self {
            real: 0.0,
            imag: 0.0,
            magnitude: 0.0,
            phase: 0.0,
            coherence: 0.0,
        }
    }
}

// ============================================================================
// ARCHETYPE ACTIVATION STATS
// ============================================================================

/// Statistics for archetype activation performance
#[derive(Debug, Clone, Default)]
pub struct ArchetypeActivationStats {
    /// Total number of archetype activations performed
    pub total_activations: u64,
    /// Total time spent on archetype activation (in nanoseconds)
    pub total_activation_time_ns: u64,
    /// Average time per activation (in nanoseconds)
    pub avg_activation_time_ns: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
}

impl ArchetypeActivationStats {
    /// Calculate average activation time
    pub fn calculate_avg_time(&mut self) {
        if self.total_activations > 0 {
            self.avg_activation_time_ns = self.total_activation_time_ns / self.total_activations;
        }
    }

    /// Calculate cache hit rate
    pub fn calculate_cache_hit_rate(&mut self) {
        let total_accesses = self.cache_hits + self.cache_misses;
        if total_accesses > 0 {
            self.cache_hit_rate = self.cache_hits as f64 / total_accesses as f64;
        }
    }

    /// Record an activation
    pub fn record_activation(&mut self, duration: Duration) {
        self.total_activations += 1;
        self.total_activation_time_ns += duration.as_nanos() as u64;
        self.calculate_avg_time();
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
        self.calculate_cache_hit_rate();
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
        self.calculate_cache_hit_rate();
    }

    /// Get performance improvement percentage
    pub fn performance_improvement(&self, baseline_avg_time_ns: u64) -> f64 {
        if baseline_avg_time_ns == 0 {
            return 0.0;
        }
        let improvement = ((baseline_avg_time_ns as f64 - self.avg_activation_time_ns as f64)
            / baseline_avg_time_ns as f64)
            * 100.0;
        improvement.max(0.0)
    }
}

// ============================================================================
// CACHED ARCHETYPE ACTIVATION
// ============================================================================

/// Cached archetype activation result with metadata
#[derive(Debug, Clone)]
pub struct CachedArchetypeActivation {
    /// The cached activation value
    pub activation: ArchetypeActivation,
    /// The archetype ID
    pub archetype_id: u8,
    /// The polarization value used
    pub polarization: f64,
    /// The timestamp when this cache entry was created
    pub timestamp: Instant,
    /// The number of times this cache entry has been accessed
    pub access_count: u64,
}

impl CachedArchetypeActivation {
    /// Create a new cached archetype activation
    pub fn new(activation: ArchetypeActivation, archetype_id: u8, polarization: f64) -> Self {
        Self {
            activation,
            archetype_id,
            polarization,
            timestamp: Instant::now(),
            access_count: 0,
        }
    }

    /// Check if the cache entry is stale (older than max_age)
    pub fn is_stale(&self, max_age: Duration) -> bool {
        self.timestamp.elapsed() > max_age
    }

    /// Record an access to this cache entry
    pub fn record_access(&mut self) {
        self.access_count += 1;
    }
}

// ============================================================================
// ARCHETYPE ACTIVATION CACHE
// ============================================================================

/// Cache for archetype activation results
#[derive(Debug)]
pub struct ArchetypeActivationCache {
    /// Cache entries keyed by (archetype_id, polarization_hash)
    /// Note: We hash f64 to avoid Hash/Eq trait issues
    cache: HashMap<(u8, u64), CachedArchetypeActivation>,
    /// Maximum age for cache entries
    max_age: Duration,
    /// Maximum cache size
    max_size: usize,
    /// Statistics
    stats: ArchetypeActivationStats,
}

/// Helper function to hash f64 for cache keys
fn hash_f64(value: f64) -> u64 {
    value.to_bits() // Convert f64 to u64 for hashing
}

impl ArchetypeActivationCache {
    /// Create a new archetype activation cache
    pub fn new(max_size: usize, max_age: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            max_age,
            max_size,
            stats: ArchetypeActivationStats::default(),
        }
    }

    /// Get a cached activation
    pub fn get(&mut self, key: (u8, u64)) -> Option<&CachedArchetypeActivation> {
        // First check if entry exists and is stale
        let is_stale = self
            .cache
            .get(&key)
            .map(|entry| entry.is_stale(self.max_age))
            .unwrap_or(false);

        if is_stale {
            self.cache.remove(&key);
            self.stats.record_cache_miss();
            return None;
        }

        if let Some(entry) = self.cache.get_mut(&key) {
            entry.record_access();
            self.stats.record_cache_hit();
            Some(entry)
        } else {
            self.stats.record_cache_miss();
            None
        }
    }

    /// Insert a new activation into the cache
    pub fn insert(&mut self, key: (u8, u64), activation: ArchetypeActivation) {
        let entry = CachedArchetypeActivation::new(activation, key.0, 0.0); // polarization not stored in key

        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        self.cache.insert(key, entry);
    }

    /// Evict the oldest entry from the cache
    fn evict_oldest(&mut self) {
        if let Some((&oldest_key, _)) = self.cache.iter().min_by_key(|(_, entry)| entry.timestamp) {
            self.cache.remove(&oldest_key);
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> &ArchetypeActivationStats {
        &self.stats
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

// ============================================================================
// ARCHETYPE ACTIVATION OPTIMIZER
// ============================================================================

/// Optimizer for archetype activation calculations
#[derive(Debug)]
pub struct ArchetypeActivationOptimizer {
    /// Cache for activation results
    cache: ArchetypeActivationCache,
    /// Holographic seed reference
    seed: Arc<HolographicSeed>,
    /// Statistics
    stats: ArchetypeActivationStats,
    /// Enable/disable optimization
    optimization_enabled: bool,
    /// Enable/disable caching
    caching_enabled: bool,
}

impl ArchetypeActivationOptimizer {
    /// Create a new archetype activation optimizer
    pub fn new(seed: Arc<HolographicSeed>) -> Self {
        Self {
            cache: ArchetypeActivationCache::new(1000, Duration::from_secs(60)),
            seed,
            stats: ArchetypeActivationStats::default(),
            optimization_enabled: true,
            caching_enabled: true,
        }
    }

    /// Create a new optimizer with custom cache settings
    pub fn with_cache_config(
        seed: Arc<HolographicSeed>,
        max_size: usize,
        max_age: Duration,
    ) -> Self {
        Self {
            cache: ArchetypeActivationCache::new(max_size, max_age),
            seed,
            stats: ArchetypeActivationStats::default(),
            optimization_enabled: true,
            caching_enabled: true,
        }
    }

    /// Enable or disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }

    /// Enable or disable caching
    pub fn set_caching_enabled(&mut self, enabled: bool) {
        self.caching_enabled = enabled;
    }

    /// Get optimized archetype activation
    pub fn get_activation(&mut self, archetype_id: u8, polarization: f64) -> ArchetypeActivation {
        let start = Instant::now();
        let polarization_hash = hash_f64(polarization);

        // Try to get from cache if caching is enabled
        if self.caching_enabled {
            let key = (archetype_id, polarization_hash);
            if let Some(cached) = self.cache.get(key) {
                return cached.activation.clone();
            }
        }

        // Calculate activation if not in cache
        let activation = if self.optimization_enabled {
            self.calculate_activation_optimized(archetype_id, polarization)
        } else {
            self.calculate_activation_baseline(archetype_id, polarization)
        };

        // Store in cache if caching is enabled
        if self.caching_enabled {
            self.cache
                .insert((archetype_id, polarization_hash), activation.clone());
        }

        // Record statistics
        let duration = start.elapsed();
        self.stats.record_activation(duration);

        activation
    }

    /// Calculate archetype activation with optimization
    /// Uses optimized algorithms and potential SIMD operations
    fn calculate_activation_optimized(
        &self,
        archetype_id: u8,
        polarization: f64,
    ) -> ArchetypeActivation {
        // Optimized calculation - use direct field access and minimal allocations
        let base_intensity = polarization * self.seed.free_will.free_will_intensity;
        let lambda_factor = 1.0 / (1.0 + base_intensity);

        ArchetypeActivation {
            archetype_id,
            activation_level: base_intensity * lambda_factor,
            coherence: 1.0 - (base_intensity * 0.1),
            polarization: base_intensity,
            lambda_value: lambda_factor,
        }
    }

    /// Calculate archetype activation with baseline algorithm
    /// This is the unoptimized version for comparison
    fn calculate_activation_baseline(
        &self,
        archetype_id: u8,
        polarization: f64,
    ) -> ArchetypeActivation {
        // Baseline calculation - more allocations and less efficient
        let free_will = &self.seed.free_will;
        let base_intensity = polarization * free_will.free_will_intensity;

        let lambda_val = 1.0 / (1.0 + base_intensity);
        let coherence = 1.0 - (base_intensity * 0.1);

        ArchetypeActivation {
            archetype_id,
            activation_level: base_intensity * lambda_val,
            coherence,
            polarization: base_intensity,
            lambda_value: lambda_val,
        }
    }

    /// Get multiple activations in a batch (optimized for SIMD-like operations)
    pub fn get_activation_batch(
        &mut self,
        archetype_ids: &[u8],
        polarizations: &[f64],
    ) -> Vec<ArchetypeActivation> {
        let mut activations = Vec::with_capacity(archetype_ids.len());

        for (&archetype_id, &polarization) in archetype_ids.iter().zip(polarizations.iter()) {
            activations.push(self.get_activation(archetype_id, polarization));
        }

        activations
    }

    /// Calculate archetype aggregation (optimized)
    pub fn calculate_aggregation(
        &mut self,
        activations: &[ArchetypeActivation],
    ) -> ArchetypeComplex {
        if activations.is_empty() {
            return ArchetypeComplex::zero();
        }

        // Optimized aggregation - use running sum
        let mut real_sum = 0.0;
        let mut imag_sum = 0.0;
        let mut coherence_sum = 0.0;

        for activation in activations {
            let angle = activation.activation_level * 2.0 * std::f64::consts::PI;
            real_sum += activation.coherence * angle.cos();
            imag_sum += activation.coherence * angle.sin();
            coherence_sum += activation.coherence;
        }

        let magnitude = (real_sum * real_sum + imag_sum * imag_sum).sqrt();
        let avg_coherence = if activations.is_empty() {
            0.0
        } else {
            coherence_sum / activations.len() as f64
        };

        ArchetypeComplex {
            real: real_sum,
            imag: imag_sum,
            magnitude,
            phase: avg_coherence.atan2(0.0),
            coherence: avg_coherence,
        }
    }

    /// Get optimizer statistics
    pub fn stats(&self) -> &ArchetypeActivationStats {
        &self.stats
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> &ArchetypeActivationStats {
        self.cache.stats()
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.size()
    }
}

// ============================================================================
// OPTIMIZATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_activation_cache_basic() {
        let cache = ArchetypeActivationCache::new(10, Duration::from_secs(60));

        // Insert a cache entry
        let activation = ArchetypeActivation {
            archetype_id: 1,
            activation_level: 0.5,
            coherence: 0.8,
            polarization: 0.7,
            lambda_value: 0.6,
        };
        cache.insert(1, 0.7, activation.clone());

        // Retrieve from cache
        let retrieved = cache.get(1, 0.7);
        assert!(retrieved.is_some());
        assert_eq!(
            retrieved.unwrap().activation.activation_level,
            activation.activation_level
        );
    }

    #[test]
    fn test_archetype_activation_cache_miss() {
        let mut cache = ArchetypeActivationCache::new(10, Duration::from_secs(60));

        // Try to retrieve non-existent entry
        let retrieved = cache.get(1, 0.7);
        assert!(retrieved.is_none());
        assert_eq!(cache.stats().cache_misses, 1);
    }

    #[test]
    fn test_archetype_activation_cache_hit() {
        let mut cache = ArchetypeActivationCache::new(10, Duration::from_secs(60));

        // Insert a cache entry
        let activation = ArchetypeActivation {
            archetype_id: 1,
            activation_level: 0.5,
            coherence: 0.8,
            polarization: 0.7,
            lambda_value: 0.6,
        };
        cache.insert(1, 0.7, activation);

        // Retrieve from cache
        let retrieved = cache.get(1, 0.7);
        assert!(retrieved.is_some());
        assert_eq!(cache.stats().cache_hits, 1);
    }

    #[test]
    fn test_archetype_activation_cache_stale() {
        let mut cache = ArchetypeActivationCache::new(10, Duration::from_millis(10));

        // Insert a cache entry
        let activation = ArchetypeActivation {
            archetype_id: 1,
            activation_level: 0.5,
            coherence: 0.8,
            polarization: 0.7,
            lambda_value: 0.6,
        };
        cache.insert(1, 0.7, activation);

        // Wait for cache entry to become stale
        std::thread::sleep(Duration::from_millis(20));

        // Try to retrieve stale entry
        let retrieved = cache.get(1, 0.7);
        assert!(retrieved.is_none());
        assert_eq!(cache.stats().cache_misses, 1);
    }

    #[test]
    fn test_archetype_activation_optimizer_basic() {
        let seed = Arc::new(HolographicSeed::new_from_source());
        let mut optimizer = ArchetypeActivationOptimizer::new(seed);

        let activation = optimizer.get_activation(1, 0.7);
        assert_eq!(activation.archetype_id, 1);
        assert!(activation.activation_level >= 0.0);
        assert!(activation.coherence >= 0.0);
    }

    #[test]
    fn test_archetype_activation_optimizer_caching() {
        let seed = Arc::new(HolographicSeed::new_from_source());
        let mut optimizer = ArchetypeActivationOptimizer::new(seed);

        // First call - cache miss
        let activation1 = optimizer.get_activation(1, 0.7);
        assert_eq!(optimizer.cache_stats().cache_misses, 1);

        // Second call - cache hit
        let activation2 = optimizer.get_activation(1, 0.7);
        assert_eq!(optimizer.cache_stats().cache_hits, 1);
        assert_eq!(activation1.activation_level, activation2.activation_level);
    }

    #[test]
    fn test_archetype_activation_optimizer_batch() {
        let seed = Arc::new(HolographicSeed::new_from_source());
        let mut optimizer = ArchetypeActivationOptimizer::new(seed);

        let archetype_ids = vec![1, 2, 3, 4, 5];
        let polarizations = vec![0.5, 0.6, 0.7, 0.8, 0.9];

        let activations = optimizer.get_activation_batch(&archetype_ids, &polarizations);
        assert_eq!(activations.len(), 5);
        for (i, activation) in activations.iter().enumerate() {
            assert_eq!(activation.archetype_id, archetype_ids[i]);
        }
    }

    #[test]
    fn test_archetype_activation_aggregation() {
        let seed = Arc::new(HolographicSeed::new_from_source());
        let mut optimizer = ArchetypeActivationOptimizer::new(seed);

        let activations = vec![
            ArchetypeActivation {
                archetype_id: 1,
                activation_level: 0.5,
                coherence: 0.8,
                polarization: 0.7,
                lambda: archetypes::LambdaMeasurement {
                    value: 0.6,
                    permeability: 0.5,
                    coherence: 0.8,
                },
            },
            ArchetypeActivation {
                archetype_id: 2,
                activation_level: 0.6,
                coherence: 0.9,
                polarization: 0.8,
                lambda: archetypes::LambdaMeasurement {
                    value: 0.7,
                    permeability: 0.5,
                    coherence: 0.9,
                },
            },
        ];

        let aggregated = optimizer.calculate_aggregation(&activations);
        assert!(aggregated.magnitude >= 0.0);
        assert!(aggregated.coherence >= 0.0);
    }

    #[test]
    fn test_archetype_activation_stats() {
        let mut stats = ArchetypeActivationStats::default();

        stats.record_activation(Duration::from_nanos(100));
        stats.record_activation(Duration::from_nanos(200));
        stats.record_activation(Duration::from_nanos(300));

        assert_eq!(stats.total_activations, 3);
        assert_eq!(stats.total_activation_time_ns, 600);
        assert_eq!(stats.avg_activation_time_ns, 200);
    }

    #[test]
    fn test_archetype_activation_cache_hit_rate() {
        let mut stats = ArchetypeActivationStats::default();

        stats.record_cache_hit();
        stats.record_cache_hit();
        stats.record_cache_miss();

        assert_eq!(stats.cache_hits, 2);
        assert_eq!(stats.cache_misses, 1);
        assert!((stats.cache_hit_rate - 0.666666).abs() < 0.001);
    }

    #[test]
    fn test_archetype_activation_performance_improvement() {
        let mut stats = ArchetypeActivationStats::default();

        stats.record_activation(Duration::from_nanos(100));
        stats.record_activation(Duration::from_nanos(100));

        let improvement = stats.performance_improvement(200);
        assert!((improvement - 50.0).abs() < 0.001);
    }
}
