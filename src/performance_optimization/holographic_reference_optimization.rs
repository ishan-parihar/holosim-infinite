// ============================================================================
// TASK 2: OPTIMIZE HOLOGRAPHIC REFERENCE ACCESS
// ============================================================================
// Objective: Profile and optimize holographic reference access patterns.
//
// Tasks:
// - Profile reference access code
// - Ensure O(1) access
// - Optimize Arc cloning
// - Optimize reference counting
// - Implement reference pooling if beneficial
// ============================================================================

use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// HOLOGRAPHIC REFERENCE STATS
// ============================================================================

/// Statistics for holographic reference access performance
#[derive(Debug, Clone, Default)]
pub struct HolographicReferenceStats {
    /// Total number of reference accesses
    pub total_accesses: u64,
    /// Total time spent on reference access (in nanoseconds)
    pub total_access_time_ns: u64,
    /// Average time per access (in nanoseconds)
    pub avg_access_time_ns: u64,
    /// Number of Arc clones
    pub arc_clones: u64,
    /// Number of Arc drops
    pub arc_drops: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Current Arc reference count
    pub current_ref_count: u64,
    /// Peak Arc reference count
    pub peak_ref_count: u64,
}

impl HolographicReferenceStats {
    /// Calculate average access time
    pub fn calculate_avg_time(&mut self) {
        if self.total_accesses > 0 {
            self.avg_access_time_ns = self.total_access_time_ns / self.total_accesses;
        }
    }

    /// Record an access
    pub fn record_access(&mut self, duration: Duration) {
        self.total_accesses += 1;
        self.total_access_time_ns += duration.as_nanos() as u64;
        self.calculate_avg_time();
    }

    /// Record an Arc clone
    pub fn record_arc_clone(&mut self) {
        self.arc_clones += 1;
    }

    /// Record an Arc drop
    pub fn record_arc_drop(&mut self) {
        self.arc_drops += 1;
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    /// Update current reference count
    pub fn update_ref_count(&mut self, count: u64) {
        self.current_ref_count = count;
        if count > self.peak_ref_count {
            self.peak_ref_count = count;
        }
    }

    /// Get cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 0.0;
        }
        self.cache_hits as f64 / total as f64
    }

    /// Get access overhead as percentage of baseline
    pub fn access_overhead(&self, baseline_ns: u64) -> f64 {
        if baseline_ns == 0 {
            return 0.0;
        }
        ((self.avg_access_time_ns as f64 - baseline_ns as f64) / baseline_ns as f64) * 100.0
    }
}

// ============================================================================
// REFERENCE POOL
// ============================================================================

/// Pool for reusing holographic references to reduce Arc cloning overhead
#[derive(Debug)]
pub struct ReferencePool {
    /// Pool of available references
    pool: Vec<Arc<HolographicSeed>>,
    /// Maximum pool size
    max_size: usize,
    /// Statistics
    stats: HolographicReferenceStats,
    /// Pool access counter
    access_counter: AtomicU64,
}

impl ReferencePool {
    /// Create a new reference pool
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: Vec::with_capacity(max_size),
            max_size,
            stats: HolographicReferenceStats::default(),
            access_counter: AtomicU64::new(0),
        }
    }

    /// Get a reference from the pool or create a new one
    pub fn get_or_create(
        &mut self,
        seed_factory: impl FnOnce() -> Arc<HolographicSeed>,
    ) -> Arc<HolographicSeed> {
        let start = Instant::now();

        let reference = if let Some(ref_arc) = self.pool.pop() {
            self.stats.record_cache_hit();
            ref_arc
        } else {
            self.stats.record_cache_miss();
            seed_factory()
        };

        self.stats.record_access(start.elapsed());
        self.access_counter.fetch_add(1, Ordering::Relaxed);

        reference
    }

    /// Return a reference to the pool
    pub fn return_reference(&mut self, reference: Arc<HolographicSeed>) {
        if self.pool.len() < self.max_size {
            self.pool.push(reference);
        }
    }

    /// Clear the pool
    pub fn clear(&mut self) {
        self.pool.clear();
    }

    /// Get pool statistics
    pub fn stats(&self) -> &HolographicReferenceStats {
        &self.stats
    }

    /// Get pool size
    pub fn size(&self) -> usize {
        self.pool.len()
    }

    /// Get total accesses
    pub fn total_accesses(&self) -> u64 {
        self.access_counter.load(Ordering::Relaxed)
    }
}

// ============================================================================
// HOLOGRAPHIC REFERENCE CACHE
// ============================================================================

/// Cache for holographic reference access results
#[derive(Debug)]
pub struct HolographicReferenceCache {
    /// Cache of access results
    cache: HashMap<String, Arc<HolographicSeed>>,
    /// Maximum cache size
    max_size: usize,
    /// Statistics
    stats: HolographicReferenceStats,
}

impl HolographicReferenceCache {
    /// Create a new holographic reference cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
            stats: HolographicReferenceStats::default(),
        }
    }

    /// Get a cached reference
    pub fn get(&mut self, key: &str) -> Option<Arc<HolographicSeed>> {
        let start = Instant::now();

        let result = self.cache.get(key).cloned();

        if result.is_some() {
            self.stats.record_cache_hit();
        } else {
            self.stats.record_cache_miss();
        }

        self.stats.record_access(start.elapsed());
        result
    }

    /// Insert a reference into the cache
    pub fn insert(&mut self, key: String, reference: Arc<HolographicSeed>) {
        // Evict oldest entry if cache is full
        if self.cache.len() >= self.max_size {
            self.cache.clear(); // Simple eviction strategy - clear all
        }

        self.cache.insert(key, reference);
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> &HolographicReferenceStats {
        &self.stats
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

// ============================================================================
// HOLOGRAPHIC REFERENCE OPTIMIZER
// ============================================================================

/// Optimizer for holographic reference access
#[derive(Debug)]
pub struct HolographicReferenceOptimizer {
    /// Reference pool
    pool: ReferencePool,
    /// Reference cache
    cache: HolographicReferenceCache,
    /// Statistics
    stats: HolographicReferenceStats,
    /// Enable/disable optimization
    optimization_enabled: bool,
    /// Enable/disable pooling
    pooling_enabled: bool,
    /// Enable/disable caching
    caching_enabled: bool,
}

impl HolographicReferenceOptimizer {
    /// Create a new holographic reference optimizer
    pub fn new() -> Self {
        Self {
            pool: ReferencePool::new(100),
            cache: HolographicReferenceCache::new(100),
            stats: HolographicReferenceStats::default(),
            optimization_enabled: true,
            pooling_enabled: true,
            caching_enabled: true,
        }
    }

    /// Create a new optimizer with custom pool and cache sizes
    pub fn with_config(pool_size: usize, cache_size: usize) -> Self {
        Self {
            pool: ReferencePool::new(pool_size),
            cache: HolographicReferenceCache::new(cache_size),
            stats: HolographicReferenceStats::default(),
            optimization_enabled: true,
            pooling_enabled: true,
            caching_enabled: true,
        }
    }

    /// Enable or disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }

    /// Enable or disable pooling
    pub fn set_pooling_enabled(&mut self, enabled: bool) {
        self.pooling_enabled = enabled;
    }

    /// Enable or disable caching
    pub fn set_caching_enabled(&mut self, enabled: bool) {
        self.caching_enabled = enabled;
    }

    /// Get a holographic reference with optimization
    pub fn get_reference(&mut self, seed: Arc<HolographicSeed>) -> Arc<HolographicSeed> {
        let start = Instant::now();

        let reference = if self.optimization_enabled && self.caching_enabled {
            // Try to get from cache
            let cache_key = format!("{:p}", seed.as_ref());
            if let Some(cached) = self.cache.get(&cache_key) {
                cached
            } else {
                // Not in cache, use the provided reference
                self.cache.insert(cache_key, seed.clone());
                seed
            }
        } else {
            seed
        };

        // Update statistics
        let duration = start.elapsed();
        self.stats.record_access(duration);
        self.stats
            .update_ref_count(Arc::strong_count(&reference) as u64);

        reference
    }

    /// Clone a holographic reference with optimization
    pub fn clone_reference(&mut self, reference: &Arc<HolographicSeed>) -> Arc<HolographicSeed> {
        let start = Instant::now();

        let cloned = if self.optimization_enabled && self.pooling_enabled {
            // Try to get from pool
            self.pool.get_or_create(|| reference.clone())
        } else {
            // Direct clone
            self.stats.record_arc_clone();
            reference.clone()
        };

        // Update statistics
        let duration = start.elapsed();
        self.stats.record_access(duration);
        self.stats
            .update_ref_count(Arc::strong_count(&cloned) as u64);

        cloned
    }

    /// Drop a holographic reference with optimization
    pub fn drop_reference(&mut self, reference: Arc<HolographicSeed>) {
        let start = Instant::now();

        if self.optimization_enabled && self.pooling_enabled {
            // Return to pool
            self.pool.return_reference(reference);
        }

        // Update statistics
        self.stats.record_arc_drop();
        self.stats.record_access(start.elapsed());
    }

    /// Access holographic seed data with O(1) complexity
    pub fn access_seed_data<'a>(
        &'a self,
        reference: &'a Arc<HolographicSeed>,
    ) -> &'a HolographicSeed {
        // Direct access - O(1) complexity
        reference.as_ref()
    }

    /// Batch access multiple references
    pub fn batch_access(
        &mut self,
        references: &[Arc<HolographicSeed>],
    ) -> Vec<Arc<HolographicSeed>> {
        let start = Instant::now();
        let count = references.len() as u64;

        // Clone the references to avoid borrow conflicts
        let accessed: Vec<Arc<HolographicSeed>> = references.to_vec();

        // Update statistics
        let duration = start.elapsed();
        self.stats.total_accesses += count;
        self.stats.total_access_time_ns += duration.as_nanos() as u64;
        self.stats.calculate_avg_time();

        accessed
    }

    /// Get optimizer statistics
    pub fn stats(&self) -> &HolographicReferenceStats {
        &self.stats
    }

    /// Get pool statistics
    pub fn pool_stats(&self) -> &HolographicReferenceStats {
        self.pool.stats()
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> &HolographicReferenceStats {
        self.cache.stats()
    }

    /// Clear pool and cache
    pub fn clear(&mut self) {
        self.pool.clear();
        self.cache.clear();
    }

    /// Get total pool size
    pub fn pool_size(&self) -> usize {
        self.pool.size()
    }

    /// Get total cache size
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
    fn test_reference_pool_basic() {
        let mut pool = ReferencePool::new(10);

        let seed = Arc::new(HolographicSeed::new_from_source());

        // Return reference to pool
        pool.return_reference(seed.clone());

        // Get reference from pool
        let retrieved = pool.get_or_create(|| seed.clone());
        assert_eq!(pool.stats().cache_hits, 1);
    }

    #[test]
    fn test_reference_pool_miss() {
        let mut pool = ReferencePool::new(10);

        // Get reference when pool is empty
        let retrieved = pool.get_or_create(|| Arc::new(HolographicSeed::new_from_source()));
        assert_eq!(pool.stats().cache_misses, 1);
    }

    #[test]
    fn test_reference_pool_size_limit() {
        let mut pool = ReferencePool::new(2);

        let seed = Arc::new(HolographicSeed::new_from_source());

        // Return 3 references (exceeds max size)
        pool.return_reference(seed.clone());
        pool.return_reference(seed.clone());
        pool.return_reference(seed.clone());

        // Pool should only hold 2 references
        assert_eq!(pool.size(), 2);
    }

    #[test]
    fn test_holographic_reference_cache_basic() {
        let mut cache = HolographicReferenceCache::new(10);

        let seed = Arc::new(HolographicSeed::new_from_source());

        // Insert into cache
        cache.insert("test_key".to_string(), seed.clone());

        // Retrieve from cache
        let retrieved = cache.get("test_key");
        assert!(retrieved.is_some());
        assert_eq!(cache.stats().cache_hits, 1);
    }

    #[test]
    fn test_holographic_reference_cache_miss() {
        let mut cache = HolographicReferenceCache::new(10);

        // Try to retrieve non-existent entry
        let retrieved = cache.get("test_key");
        assert!(retrieved.is_none());
        assert_eq!(cache.stats().cache_misses, 1);
    }

    #[test]
    fn test_holographic_reference_optimizer_basic() {
        let mut optimizer = HolographicReferenceOptimizer::new();

        let seed = Arc::new(HolographicSeed::new_from_source());
        let reference = optimizer.get_reference(seed);

        assert_eq!(optimizer.stats().total_accesses, 1);
    }

    #[test]
    fn test_holographic_reference_optimizer_clone() {
        let mut optimizer = HolographicReferenceOptimizer::new();

        let seed = Arc::new(HolographicSeed::new_from_source());
        let cloned = optimizer.clone_reference(&seed);

        assert_eq!(optimizer.stats().arc_clones, 0); // Should use pool
        assert_eq!(Arc::strong_count(&seed), 2);
    }

    #[test]
    fn test_holographic_reference_optimizer_drop() {
        let mut optimizer = HolographicReferenceOptimizer::new();

        let seed = Arc::new(HolographicSeed::new_from_source());
        optimizer.drop_reference(seed);

        assert_eq!(optimizer.stats().arc_drops, 1);
    }

    #[test]
    fn test_holographic_reference_optimizer_access_o1() {
        let mut optimizer = HolographicReferenceOptimizer::new();

        let seed = Arc::new(HolographicSeed::new_from_source());

        // Access should be O(1) - fast
        let start = Instant::now();
        let _ = optimizer.access_seed_data(&seed);
        let duration = start.elapsed();

        assert!(duration.as_nanos() < 1000); // Should be < 1 microsecond
    }

    #[test]
    fn test_holographic_reference_optimizer_batch_access() {
        let mut optimizer = HolographicReferenceOptimizer::new();

        let seeds: Vec<Arc<HolographicSeed>> = vec![
            Arc::new(HolographicSeed::new_from_source()),
            Arc::new(HolographicSeed::new_from_source()),
            Arc::new(HolographicSeed::new_from_source()),
        ];

        let accessed = optimizer.batch_access(&seeds);
        assert_eq!(accessed.len(), 3);
        assert_eq!(optimizer.stats().total_accesses, 3);
    }

    #[test]
    fn test_holographic_reference_stats() {
        let mut stats = HolographicReferenceStats::default();

        stats.record_access(Duration::from_nanos(100));
        stats.record_access(Duration::from_nanos(200));
        stats.record_access(Duration::from_nanos(300));

        assert_eq!(stats.total_accesses, 3);
        assert_eq!(stats.total_access_time_ns, 600);
        assert_eq!(stats.avg_access_time_ns, 200);
    }

    #[test]
    fn test_holographic_reference_cache_hit_rate() {
        let mut stats = HolographicReferenceStats::default();

        stats.record_cache_hit();
        stats.record_cache_hit();
        stats.record_cache_hit();
        stats.record_cache_miss();

        assert_eq!(stats.cache_hits, 3);
        assert_eq!(stats.cache_misses, 1);
        assert!((stats.cache_hit_rate() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_holographic_reference_access_overhead() {
        let mut stats = HolographicReferenceStats::default();

        stats.record_access(Duration::from_nanos(150));
        stats.record_access(Duration::from_nanos(150));

        let overhead = stats.access_overhead(100);
        assert!((overhead - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_holographic_reference_ref_count_tracking() {
        let mut stats = HolographicReferenceStats::default();

        stats.update_ref_count(5);
        stats.update_ref_count(3);
        stats.update_ref_count(7);

        assert_eq!(stats.current_ref_count, 7);
        assert_eq!(stats.peak_ref_count, 7);
    }
}
