//! Observer Registry - Integration Layer for Holographic Field Substrate (Phase 1)
//!
//! This module implements the observer registry that integrates the existing holographic
//! infrastructure with the CausalInversionRunner from Phase 0.
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 1:
//! "Only observed regions are decompressed"
//!
//! ## Core Principles
//!
//! ### Quantum Measurement Analogy
//!
//! In quantum mechanics, observation collapses the wave function from superposition
//! to definite state. This registry implements a computational analog:
//!
//! - **Unobserved regions**: Remain in compressed form (superposition of possibilities)
//! - **Observed regions**: Decompressed into definite field values
//!
//! This is not merely an optimization—it reflects the deep principle that reality
//! emerges through observation.
//!
//! ### Holographic Principle
//!
//! From COSMOLOGICAL-ARCHITECTURE.md:
//! "Each entity contains within it all densities and sub-densities of the octave"
//!
//! The holographic principle states that each point contains information about
//! the whole. The registry enables:
//!
//! 1. **On-demand decompression**: Only what is observed exists in memory
//! 2. **Hierarchical caching**: Coarse scales cached longer than fine scales
//! 3. **Memory efficiency**: Exponential reduction in memory footprint
//!
//! ### Integration with Phase 0 CausalInversionRunner
//!
//! The CausalInversionRunner implements top-down causation:
//!
//! ```text
//! Infinity → Field → Potentials → Entities → Observer Decompression
//! ```
//!
//! The ObserverRegistry integrates at Phase 6 (Observer Decompression):
//!
//! - Receives observer positions from manifested entities
//! - Triggers region decompression via MeraIntegration
//! - Manages cache lifecycle for decompressed regions

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::holographic::field_address::{AddressRange, HolographicAddress, ScaleLevel, Vector3};
use crate::holographic::mera_integration::MeraIntegration;
use crate::holographic::observer_driven_field::{DecompressedRegion, Observer, ObserverId};
use crate::types::Float;

// ============================================================================
// CACHE STATISTICS
// ============================================================================

/// Statistics tracking cache performance and memory usage.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of decompression operations performed.
    pub total_decompressions: u64,
    /// Number of cache hits (region already decompressed).
    pub cache_hits: u64,
    /// Number of cache misses (region needed decompression).
    pub cache_misses: u64,
    /// Number of regions evicted due to idle timeout or memory pressure.
    pub regions_evicted: u64,
    /// Current number of regions in the cache.
    pub current_cache_size: usize,
    /// Maximum allowed regions in the cache.
    pub max_cache_size: usize,
}

impl CacheStats {
    /// Create new cache statistics with specified maximum cache size.
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            max_cache_size,
            ..Default::default()
        }
    }

    /// Calculate cache hit rate as a percentage (0.0 to 1.0).
    pub fn hit_rate(&self) -> Float {
        let total_requests = self.cache_hits + self.cache_misses;
        if total_requests == 0 {
            0.0
        } else {
            self.cache_hits as Float / total_requests as Float
        }
    }

    /// Calculate cache utilization as a percentage (0.0 to 1.0).
    pub fn utilization(&self) -> Float {
        if self.max_cache_size == 0 {
            0.0
        } else {
            self.current_cache_size as Float / self.max_cache_size as Float
        }
    }

    /// Record a cache hit.
    pub fn record_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Record a cache miss and decompression.
    pub fn record_miss(&mut self) {
        self.cache_misses += 1;
        self.total_decompressions += 1;
    }

    /// Record a region eviction.
    pub fn record_eviction(&mut self) {
        self.regions_evicted += 1;
    }

    /// Reset all statistics.
    pub fn reset(&mut self) {
        self.total_decompressions = 0;
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.regions_evicted = 0;
    }
}

// ============================================================================
// OBSERVER REGISTRY CONFIG
// ============================================================================

/// Configuration for the ObserverRegistry.
#[derive(Debug, Clone)]
pub struct ObserverRegistryConfig {
    /// Maximum number of decompressed regions to keep in cache.
    pub max_cached_regions: usize,
    /// Number of idle ticks before a region is eligible for eviction.
    pub max_idle_ticks: u64,
    /// Radius around each observer to decompress.
    pub decompression_radius: Float,
}

impl Default for ObserverRegistryConfig {
    fn default() -> Self {
        Self {
            max_cached_regions: 100,
            max_idle_ticks: 100,
            decompression_radius: 100.0,
        }
    }
}

impl ObserverRegistryConfig {
    /// Create a new configuration with specified parameters.
    pub fn new(
        max_cached_regions: usize,
        max_idle_ticks: u64,
        decompression_radius: Float,
    ) -> Self {
        Self {
            max_cached_regions,
            max_idle_ticks,
            decompression_radius: decompression_radius.max(1.0),
        }
    }

    /// Create a high-performance configuration with smaller cache.
    pub fn performance() -> Self {
        Self {
            max_cached_regions: 50,
            max_idle_ticks: 50,
            decompression_radius: 50.0,
        }
    }

    /// Create a high-fidelity configuration with larger cache.
    pub fn high_fidelity() -> Self {
        Self {
            max_cached_regions: 500,
            max_idle_ticks: 200,
            decompression_radius: 200.0,
        }
    }

    /// Estimate memory usage in bytes based on configuration.
    pub fn estimated_memory_bytes(&self) -> usize {
        let region_size = 32 * 32 * 32 * 8 + 1024;
        self.max_cached_regions * region_size
    }
}

// ============================================================================
// CACHED REGION
// ============================================================================

/// Internal representation of a cached decompressed region.
#[derive(Debug, Clone)]
struct CachedRegion {
    /// The decompressed region data.
    region: DecompressedRegion,
    /// Cache key for this region.
    #[allow(dead_code)]
    key: String,
    /// Tick when this region was last accessed.
    last_access_tick: u64,
    /// Number of times this region has been accessed.
    access_count: u64,
    /// List of observer IDs currently viewing this region.
    active_observers: Vec<ObserverId>,
}

impl CachedRegion {
    /// Create a new cached region.
    fn new(region: DecompressedRegion, key: String, current_tick: u64) -> Self {
        Self {
            region,
            key,
            last_access_tick: current_tick,
            access_count: 1,
            active_observers: Vec::new(),
        }
    }

    /// Mark this region as accessed by an observer.
    fn touch(&mut self, tick: u64, observer_id: ObserverId) {
        self.last_access_tick = tick;
        self.access_count += 1;
        if !self.active_observers.contains(&observer_id) {
            self.active_observers.push(observer_id);
        }
        self.region.touch();
    }

    /// Remove an observer from the active list.
    fn remove_observer(&mut self, observer_id: ObserverId) {
        self.active_observers.retain(|id| *id != observer_id);
    }

    /// Check if this region has no active observers.
    fn is_idle(&self) -> bool {
        self.active_observers.is_empty()
    }

    /// Calculate idle ticks from a reference tick.
    fn idle_ticks(&self, current_tick: u64) -> u64 {
        current_tick.saturating_sub(self.last_access_tick)
    }
}

// ============================================================================
// OBSERVER REGISTRY
// ============================================================================

/// Registry for managing observers and triggering decompression.
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 1:
/// "Only observed regions are decompressed"
///
/// This is the integration layer between:
/// - CausalInversionRunner (Phase 0) - provides the causal tick framework
/// - ObserverDrivenField (existing) - defines observer types and regions
/// - MeraNetwork (existing) - provides compression/decompression
pub struct ObserverRegistry {
    /// All registered observers mapped by their ID.
    observers: HashMap<ObserverId, Observer>,
    /// Cache of decompressed regions.
    decompressed_regions: HashMap<String, CachedRegion>,
    /// MERA integration for compression/decompression operations.
    mera_integration: Arc<Mutex<MeraIntegration>>,
    /// Current simulation tick.
    current_tick: u64,
    /// Cache performance statistics.
    cache_stats: CacheStats,
    /// Configuration parameters.
    config: ObserverRegistryConfig,
    /// Next observer ID to assign.
    next_observer_id: u64,
}

impl ObserverRegistry {
    /// Create a new ObserverRegistry with the given MERA integration and configuration.
    pub fn new(
        mera_integration: Arc<Mutex<MeraIntegration>>,
        config: ObserverRegistryConfig,
    ) -> Self {
        let max_cache_size = config.max_cached_regions;
        Self {
            observers: HashMap::new(),
            decompressed_regions: HashMap::new(),
            mera_integration,
            current_tick: 0,
            cache_stats: CacheStats::new(max_cache_size),
            config,
            next_observer_id: 1,
        }
    }

    /// Create a registry with default configuration.
    pub fn with_defaults(mera_integration: Arc<Mutex<MeraIntegration>>) -> Self {
        Self::new(mera_integration, ObserverRegistryConfig::default())
    }

    // ========================================================================
    // Observer Management
    // ========================================================================

    /// Register a new observer and return its assigned ID.
    pub fn register_observer(&mut self, mut observer: Observer) -> ObserverId {
        if observer.id == ObserverId::default() {
            observer.id = ObserverId::new(self.next_observer_id);
            self.next_observer_id += 1;
        } else {
            self.next_observer_id = self.next_observer_id.max(observer.id.as_u64() + 1);
        }

        observer.touch(self.current_tick);

        let id = observer.id;
        self.observers.insert(id, observer);
        id
    }

    /// Unregister an observer by ID.
    pub fn unregister_observer(&mut self, id: ObserverId) {
        if self.observers.remove(&id).is_some() {
            for cached_region in self.decompressed_regions.values_mut() {
                cached_region.remove_observer(id);
            }
        }
    }

    /// Update an observer's position in the holographic field.
    pub fn update_observer_position(
        &mut self,
        id: ObserverId,
        address: HolographicAddress,
    ) -> bool {
        if let Some(observer) = self.observers.get_mut(&id) {
            observer.address = address;
            observer.touch(self.current_tick);
            true
        } else {
            false
        }
    }

    /// Get an observer by ID.
    pub fn get_observer(&self, id: ObserverId) -> Option<&Observer> {
        self.observers.get(&id)
    }

    /// Get the number of registered observers.
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Get all registered observer IDs.
    pub fn observer_ids(&self) -> Vec<ObserverId> {
        self.observers.keys().copied().collect()
    }

    // ========================================================================
    // Decompression Operations
    // ========================================================================

    /// Decompress regions for all active observers.
    pub fn decompress_for_observers(&mut self) -> usize {
        let mut regions_processed = 0;

        let observer_data: Vec<(ObserverId, HolographicAddress, ScaleLevel)> = self
            .observers
            .iter()
            .map(|(id, obs)| (*id, obs.address.clone(), obs.scale))
            .collect();

        for (observer_id, address, scale) in observer_data {
            let cache_key = Self::generate_cache_key_static(&address, &scale, &observer_id);

            if let Some(cached_region) = self.decompressed_regions.get_mut(&cache_key) {
                cached_region.touch(self.current_tick, observer_id);
                self.cache_stats.record_hit();
                regions_processed += 1;
                continue;
            }

            self.cache_stats.record_miss();

            if self.decompressed_regions.len() >= self.config.max_cached_regions {
                self.evict_lru_idle_region();
            }

            let address_range =
                Self::create_address_range_static(&address, self.config.decompression_radius);

            if let Some(region) = self.decompress_region(&address_range, scale) {
                let cached = CachedRegion::new(region, cache_key.clone(), self.current_tick);
                self.decompressed_regions.insert(cache_key, cached);
                regions_processed += 1;
            }
        }

        self.cache_stats.current_cache_size = self.decompressed_regions.len();
        regions_processed
    }

    /// Decompress a specific region from the MERA network.
    fn decompress_region(
        &mut self,
        address_range: &AddressRange,
        scale: ScaleLevel,
    ) -> Option<DecompressedRegion> {
        let dummy_observer_id = ObserverId::new(0);

        let mera_integration = Arc::clone(&self.mera_integration);
        let mut integration = mera_integration.lock().ok()?;

        let _tensor = integration.decompress_for_observer(
            address_range,
            scale,
            dummy_observer_id,
            self.current_tick,
        )?;

        Some(DecompressedRegion::new(
            address_range.clone(),
            scale,
            self.config.max_idle_ticks,
        ))
    }

    /// Generate a cache key for a region (static version).
    fn generate_cache_key_static(
        address: &HolographicAddress,
        scale: &ScaleLevel,
        observer_id: &ObserverId,
    ) -> String {
        let quantization = 10.0;
        let pos = address.to_position();
        let qx = (pos.x / quantization).floor() as i64;
        let qy = (pos.y / quantization).floor() as i64;
        let qz = (pos.z / quantization).floor() as i64;

        format!(
            "region_{}_{}_{}_{}_{}",
            scale_to_string(scale),
            qx,
            qy,
            qz,
            observer_id.as_u64()
        )
    }

    /// Create an address range centered on a position with given radius (static version).
    fn create_address_range_static(center: &HolographicAddress, _radius: Float) -> AddressRange {
        // Create range using local_offset within the same scale
        // For simplicity, create a small range around the center's local offset
        let offset = &center.local_offset;
        let min_addr = HolographicAddress::new(
            center.scale,
            center.coherence_path.clone(),
            Vector3::new(
                (offset.x - 0.1).max(0.0),
                (offset.y - 0.1).max(0.0),
                (offset.z - 0.1).max(0.0),
            ),
        );
        let max_addr = HolographicAddress::new(
            center.scale,
            center.coherence_path.clone(),
            Vector3::new(
                (offset.x + 0.1).min(1.0),
                (offset.y + 0.1).min(1.0),
                (offset.z + 0.1).min(1.0),
            ),
        );

        AddressRange::new(min_addr, max_addr)
    }

    // ========================================================================
    // Cache Management
    // ========================================================================

    /// Evict idle regions that have exceeded the idle timeout.
    pub fn evict_idle_regions(&mut self) -> usize {
        let current_tick = self.current_tick;
        let max_idle = self.config.max_idle_ticks;

        let keys_to_evict: Vec<String> = self
            .decompressed_regions
            .iter()
            .filter(|(_, cached)| cached.is_idle() && cached.idle_ticks(current_tick) >= max_idle)
            .map(|(key, _)| key.clone())
            .collect();

        let evicted_count = keys_to_evict.len();
        for key in keys_to_evict {
            self.decompressed_regions.remove(&key);
            self.cache_stats.record_eviction();
        }

        self.cache_stats.current_cache_size = self.decompressed_regions.len();
        evicted_count
    }

    /// Evict the least recently used idle region.
    fn evict_lru_idle_region(&mut self) -> bool {
        let lru_key = self
            .decompressed_regions
            .iter()
            .filter(|(_, cached)| cached.is_idle())
            .min_by_key(|(_, cached)| cached.last_access_tick)
            .map(|(key, _)| key.clone());

        if let Some(key) = lru_key {
            self.decompressed_regions.remove(&key);
            self.cache_stats.record_eviction();
            self.cache_stats.current_cache_size = self.decompressed_regions.len();
            true
        } else {
            false
        }
    }

    /// Clear all cached regions.
    pub fn clear_cache(&mut self) {
        let count = self.decompressed_regions.len();
        self.decompressed_regions.clear();
        self.cache_stats.regions_evicted += count as u64;
        self.cache_stats.current_cache_size = 0;
    }

    // ========================================================================
    // Statistics and Monitoring
    // ========================================================================

    /// Get the cache hit rate as a percentage (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> Float {
        self.cache_stats.hit_rate()
    }

    /// Get current cache statistics.
    pub fn memory_stats(&self) -> CacheStats {
        self.cache_stats.clone()
    }

    /// Get the number of currently cached regions.
    pub fn cached_region_count(&self) -> usize {
        self.decompressed_regions.len()
    }

    /// Estimate current memory usage in bytes.
    pub fn estimated_memory_usage(&self) -> usize {
        let region_size = 32 * 32 * 32 * 8 + 1024 + 200;
        self.decompressed_regions.len() * region_size
    }

    // ========================================================================
    // Tick Management
    // ========================================================================

    /// Advance the simulation tick counter.
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
    }

    /// Get the current tick number.
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Set the current tick (used when restoring state).
    pub fn set_tick(&mut self, tick: u64) {
        self.current_tick = tick;
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Convert ScaleLevel to a string for cache keys.
fn scale_to_string(scale: &ScaleLevel) -> &'static str {
    match scale {
        ScaleLevel::Quantum => "q",
        ScaleLevel::Atomic => "a",
        ScaleLevel::Molecular => "m",
        ScaleLevel::Cellular => "c",
        ScaleLevel::Biological => "b",
        ScaleLevel::Planetary => "p",
        ScaleLevel::Stellar => "s",
        ScaleLevel::Cosmic => "C",
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::compression::mera_network::MeraNetwork;

    fn create_test_registry() -> ObserverRegistry {
        let network = Arc::new(Mutex::new(MeraNetwork::new()));
        let mera_integration = Arc::new(Mutex::new(MeraIntegration::new(network)));

        let config = ObserverRegistryConfig::new(10, 5, 50.0);
        ObserverRegistry::new(mera_integration, config)
    }

    #[test]
    fn test_observer_registration() {
        let mut registry = create_test_registry();

        let address = HolographicAddress::cosmic_origin();
        let observer = Observer::entity(42, address);
        let id = registry.register_observer(observer);

        assert_eq!(registry.observer_count(), 1);
        assert!(registry.get_observer(id).is_some());

        let retrieved = registry.get_observer(id).unwrap();
        assert_eq!(retrieved.id, id);
    }

    #[test]
    fn test_observer_unregistration() {
        let mut registry = create_test_registry();

        let address = HolographicAddress::cosmic_origin();
        let observer = Observer::entity(42, address);
        let id = registry.register_observer(observer);
        assert_eq!(registry.observer_count(), 1);

        registry.unregister_observer(id);
        assert_eq!(registry.observer_count(), 0);
        assert!(registry.get_observer(id).is_none());
    }

    #[test]
    fn test_observer_position_update() {
        let mut registry = create_test_registry();

        let address1 = HolographicAddress::cosmic_origin();
        let observer = Observer::entity(42, address1);
        let id = registry.register_observer(observer);

        let address2 =
            HolographicAddress::new(ScaleLevel::Biological, vec![], Vector3::new(0.5, 0.5, 0.5));
        let updated = registry.update_observer_position(id, address2.clone());
        assert!(updated);

        let retrieved = registry.get_observer(id).unwrap();
        assert!((retrieved.address.local_offset.x - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_cache_stats() {
        let mut stats = CacheStats::new(100);

        assert_eq!(stats.hit_rate(), 0.0);
        assert_eq!(stats.utilization(), 0.0);

        stats.record_hit();
        stats.record_hit();
        stats.record_miss();
        stats.record_hit();

        assert!((stats.hit_rate() - 0.75).abs() < 1e-6);
        assert_eq!(stats.cache_hits, 3);
        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.total_decompressions, 1);
    }

    #[test]
    fn test_config_defaults() {
        let config = ObserverRegistryConfig::default();

        assert_eq!(config.max_cached_regions, 100);
        assert_eq!(config.max_idle_ticks, 100);
        assert_eq!(config.decompression_radius, 100.0);
    }

    #[test]
    fn test_config_presets() {
        let perf = ObserverRegistryConfig::performance();
        assert!(perf.max_cached_regions < ObserverRegistryConfig::default().max_cached_regions);

        let high_fi = ObserverRegistryConfig::high_fidelity();
        assert!(high_fi.max_cached_regions > ObserverRegistryConfig::default().max_cached_regions);
    }

    #[test]
    fn test_cache_eviction_by_idle() {
        let mut registry = create_test_registry();

        let address = HolographicAddress::cosmic_origin();
        let observer = Observer::entity(42, address);
        let id = registry.register_observer(observer);

        registry.advance_tick();
        registry.advance_tick();

        registry.unregister_observer(id);

        for _ in 0..6 {
            registry.advance_tick();
        }

        let evicted = registry.evict_idle_regions();
        assert_eq!(evicted, 0);
    }

    #[test]
    fn test_memory_estimation() {
        let registry = create_test_registry();

        let usage = registry.estimated_memory_usage();
        assert!(usage < 1024);

        let config = ObserverRegistryConfig::default();
        let estimated = config.estimated_memory_bytes();
        assert!(estimated > 20_000_000);
        assert!(estimated < 50_000_000);
    }

    #[test]
    fn test_cache_key_generation() {
        let address =
            HolographicAddress::new(ScaleLevel::Biological, vec![], Vector3::new(0.5, 0.5, 0.5));
        let scale = ScaleLevel::Biological;
        let observer_id = ObserverId::new(42);

        let key = ObserverRegistry::generate_cache_key_static(&address, &scale, &observer_id);
        assert!(key.contains("_b_"));

        let key2 = ObserverRegistry::generate_cache_key_static(&address, &scale, &observer_id);
        assert_eq!(key, key2);
    }

    #[test]
    fn test_tick_advancement() {
        let mut registry = create_test_registry();

        assert_eq!(registry.current_tick(), 0);

        registry.advance_tick();
        assert_eq!(registry.current_tick(), 1);

        registry.set_tick(100);
        assert_eq!(registry.current_tick(), 100);
    }

    #[test]
    fn test_cache_hit_rate_calculation() {
        let mut registry = create_test_registry();

        assert_eq!(registry.cache_hit_rate(), 0.0);

        registry.cache_stats.record_hit();
        registry.cache_stats.record_hit();
        registry.cache_stats.record_miss();

        let rate = registry.cache_hit_rate();
        assert!(rate > 0.65 && rate < 0.68);
    }

    #[test]
    fn test_clear_cache() {
        let mut registry = create_test_registry();

        assert_eq!(registry.cached_region_count(), 0);

        registry.clear_cache();
        assert_eq!(registry.cached_region_count(), 0);
    }

    #[test]
    fn test_multiple_observers() {
        let mut registry = create_test_registry();

        for i in 0..10 {
            let address = HolographicAddress::new(
                ScaleLevel::Biological,
                vec![],
                Vector3::new(i as Float * 0.1, i as Float * 0.1, i as Float * 0.1),
            );
            let observer = Observer::entity(i + 1, address);
            registry.register_observer(observer);
        }

        assert_eq!(registry.observer_count(), 10);

        let ids = registry.observer_ids();
        assert_eq!(ids.len(), 10);

        for i in 0..5 {
            registry.unregister_observer(ObserverId::new(i + 1));
        }

        assert_eq!(registry.observer_count(), 5);
    }
}
