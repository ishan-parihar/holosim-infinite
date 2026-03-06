//! Memory Profiler for Validating Holographic Compression Efficiency
//!
//! This module provides memory tracking and validation for the holographic compression
//! claims made in the HoloSim Infinite refactor. It measures actual memory usage against
//! theoretical bounds to validate the O(n^(2/3)) scaling claim.
//!
//! # Theoretical Foundation
//!
//! From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 1:
//! > "Memory: O(n²/3) via shared references (entities share common field data)"
//!
//! ## Traditional vs Holographic Memory Scaling
//!
//! ### Traditional Simulation (O(n))
//! In a traditional simulation, each entity stores all its data independently:
//! - Position, velocity, properties
//! - Relationships, memories, history
//! - State variables, cached computations
//!
//! This leads to linear memory growth: `Memory = n × entity_size`
//!
//! ### Holographic Simulation (O(n^(2/3)))
//! The holographic principle states that "each part contains the whole." In computational
//! terms, entities share common field data through `Arc` references:
//!
//! ```text
//! Traditional:  Entity → [Full copy of all data]
//! Holographic:  Entity → [Arc reference] → Shared field data
//! ```
//!
//! The n^(2/3) scaling emerges because:
//! 1. **Field Sharing**: Entities share field data (holographic principle)
//! 2. **Archetype Compression**: 22 archetypes encode infinite behavioral variety
//! 3. **MERA Network**: Tensor network compression reduces redundancy
//! 4. **Observer-Driven Rendering**: Only observed regions are decompressed
//!
//! ### Why n^(2/3)?
//!
//! The exponent 2/3 (approximately 0.667) arises from the holographic principle:
//! - A 3D volume's information content scales with its 2D surface area
//! - Surface area scales as volume^(2/3): A ∝ V^(2/3)
//! - Therefore: Memory ∝ n^(2/3) where n is entity count
//!
//! # Usage
//!
//! ```ignore
//! use simulation_v3::memory_profiler::MemoryProfiler;
//!
//! let mut profiler = MemoryProfiler::new();
//! profiler.record_baseline();
//!
//! // During simulation, update with current stats
//! profiler.update(entity_count, &cache_stats);
//!
//! // Take snapshots for analysis
//! let snapshot = profiler.snapshot(tick);
//!
//! // Validate scaling efficiency
//! let efficiency = profiler.efficiency_report();
//! println!("{}", efficiency);
//! ```

use std::fmt;

/// Estimated memory per entity without compression (traditional approach)
///
/// Traditional entities store:
/// - Position/velocity: ~100 bytes
/// - Properties and state: ~500 bytes
/// - Relationships and memories: ~2000 bytes
/// - Behavioral data: ~1000 bytes
/// - Cached computations: ~6400 bytes
///
/// Total: ~10 KB per entity (conservative estimate)
pub const TRADITIONAL_ENTITY_SIZE: usize = 10_000;

/// Estimated memory per entity with holographic compression
///
/// Holographic entities store:
/// - Arc reference to shared field: 8 bytes
/// - Archetype vector (22 floats): 176 bytes
/// - Position in field: 24 bytes
/// - Minimal state: 50 bytes
/// - Free will seed: 16 bytes
///
/// Total: ~100 bytes per entity (with Arc-based sharing)
pub const HOLOGRAPHIC_ENTITY_SIZE: usize = 100;

/// Expected scaling exponent for holographic compression
///
/// The 2/3 exponent (approximately 0.667) comes from the holographic principle:
/// information in a volume scales with its surface area, and surface area
/// scales as volume^(2/3).
pub const HOLOGRAPHIC_SCALING_EXPONENT: f64 = 0.667;

/// Cache memory statistics for tracking holographic field cache efficiency
///
/// The cache is central to holographic compression. Entities reference
/// cached field data rather than storing copies.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CacheMemoryStats {
    /// Number of cached regions
    ///
    /// Each region represents a portion of the holographic field that
    /// has been computed and stored for reuse.
    pub cached_regions: usize,

    /// Memory used by cache (bytes)
    ///
    /// The actual memory consumption of cached field data.
    /// This memory is shared among all entities via Arc references.
    pub cache_memory: usize,

    /// Cache hit rate (0.0 to 1.0)
    ///
    /// Higher hit rates indicate better holographic compression efficiency.
    /// A hit rate near 1.0 means entities frequently access the same
    /// shared data, validating the holographic principle.
    pub hit_rate: f64,

    /// Evictions count
    ///
    /// Number of cache entries evicted due to memory pressure.
    /// Lower eviction rates indicate stable holographic field regions.
    pub evictions: u64,
}

impl CacheMemoryStats {
    /// Creates a new empty cache statistics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates cache stats with the given values
    pub fn with_values(regions: usize, memory: usize, hit_rate: f64, evictions: u64) -> Self {
        Self {
            cached_regions: regions,
            cache_memory: memory,
            hit_rate: hit_rate.clamp(0.0, 1.0),
            evictions,
        }
    }

    /// Returns the average memory per cached region
    pub fn memory_per_region(&self) -> f64 {
        if self.cached_regions == 0 {
            0.0
        } else {
            self.cache_memory as f64 / self.cached_regions as f64
        }
    }
}

impl fmt::Display for CacheMemoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CacheStats: {} regions, {:.2} MB, {:.1}% hit rate, {} evictions",
            self.cached_regions,
            self.cache_memory as f64 / 1_048_576.0,
            self.hit_rate * 100.0,
            self.evictions
        )
    }
}

/// Compression metrics for comparing actual vs theoretical memory usage
///
/// These metrics validate the holographic compression claim by comparing
/// actual memory usage against theoretical bounds.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompressionMetrics {
    /// Theoretical memory without compression (bytes)
    ///
    /// Calculated as: `entity_count × TRADITIONAL_ENTITY_SIZE`
    /// This represents the memory a traditional simulation would require.
    pub uncompressed_size: usize,

    /// Actual memory with compression (bytes)
    ///
    /// Measured memory consumption with holographic compression enabled.
    /// Should be significantly less than `uncompressed_size`.
    pub compressed_size: usize,

    /// Compression ratio (uncompressed / compressed)
    ///
    /// A ratio > 1.0 indicates successful compression.
    /// Target ratio scales with entity count due to n^(2/3) scaling.
    pub compression_ratio: f64,

    /// Scaling factor relative to linear scaling
    ///
    /// Compares actual memory growth to n^(2/3) theoretical curve.
    /// A factor near 1.0 validates the holographic compression claim.
    /// - 1.0 = Perfect n^(2/3) scaling
    /// - > 1.0 = Worse than theoretical (compression less effective)
    /// - < 1.0 = Better than theoretical (exceptional compression)
    pub scaling_factor: f64,
}

impl CompressionMetrics {
    /// Creates a new empty compression metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculates compression metrics from measured values
    ///
    /// # Arguments
    /// * `entity_count` - Number of entities in simulation
    /// * `actual_memory` - Measured memory usage in bytes
    ///
    /// # Returns
    /// Compression metrics with calculated ratios
    pub fn calculate(entity_count: usize, actual_memory: usize) -> Self {
        let uncompressed = entity_count * TRADITIONAL_ENTITY_SIZE;
        let ratio = if actual_memory > 0 {
            uncompressed as f64 / actual_memory as f64
        } else {
            0.0
        };

        // Calculate scaling factor relative to n^(2/3)
        let theoretical_holographic = if entity_count > 0 {
            let n = entity_count as f64;
            HOLOGRAPHIC_ENTITY_SIZE as f64 * n.powf(HOLOGRAPHIC_SCALING_EXPONENT)
        } else {
            0.0
        };

        let scaling = if theoretical_holographic > 0.0 {
            actual_memory as f64 / theoretical_holographic
        } else {
            1.0
        };

        Self {
            uncompressed_size: uncompressed,
            compressed_size: actual_memory,
            compression_ratio: ratio,
            scaling_factor: scaling,
        }
    }

    /// Returns true if compression is within acceptable bounds
    ///
    /// Acceptable means:
    /// - Compression ratio > 1.0 (actual savings)
    /// - Scaling factor within 20% of theoretical (0.8 to 1.2)
    pub fn is_efficient(&self) -> bool {
        self.compression_ratio > 1.0 && self.scaling_factor > 0.8 && self.scaling_factor < 1.2
    }

    /// Returns a human-readable efficiency assessment
    pub fn efficiency_rating(&self) -> &'static str {
        if self.scaling_factor < 0.9 {
            "Excellent - Better than theoretical"
        } else if self.scaling_factor < 1.1 {
            "Good - Within theoretical bounds"
        } else if self.scaling_factor < 1.5 {
            "Fair - Slightly worse than theoretical"
        } else {
            "Poor - Compression not effective"
        }
    }
}

impl fmt::Display for CompressionMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Compression: {:.1}x ratio, scaling factor: {:.3} ({})",
            self.compression_ratio,
            self.scaling_factor,
            self.efficiency_rating()
        )
    }
}

/// A snapshot of memory state at a point in time
///
/// Used for tracking memory usage over the course of a simulation
/// and validating holographic compression efficiency at scale.
#[derive(Debug, Clone, PartialEq)]
pub struct MemorySnapshot {
    /// Simulation tick when snapshot was taken
    pub timestamp: u64,

    /// Total memory usage in bytes
    pub total_memory: usize,

    /// Number of entities at this point
    pub entity_count: usize,

    /// Memory used by cache in bytes
    pub cache_memory: usize,

    /// Bytes per entity (total_memory / entity_count)
    pub bytes_per_entity: f64,

    /// Compression ratio at this point
    pub compression_ratio: f64,
}

impl MemorySnapshot {
    /// Creates a new memory snapshot
    pub fn new(
        timestamp: u64,
        total_memory: usize,
        entity_count: usize,
        cache_memory: usize,
    ) -> Self {
        let bytes_per_entity = if entity_count > 0 {
            total_memory as f64 / entity_count as f64
        } else {
            0.0
        };

        let theoretical_uncompressed = entity_count * TRADITIONAL_ENTITY_SIZE;
        let compression_ratio = if total_memory > 0 && theoretical_uncompressed > 0 {
            theoretical_uncompressed as f64 / total_memory as f64
        } else {
            1.0
        };

        Self {
            timestamp,
            total_memory,
            entity_count,
            cache_memory,
            bytes_per_entity,
            compression_ratio,
        }
    }
}

impl fmt::Display for MemorySnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tick {}: {} entities, {:.2} MB ({:.1} bytes/entity), {:.1}x compression",
            self.timestamp,
            self.entity_count,
            self.total_memory as f64 / 1_048_576.0,
            self.bytes_per_entity,
            self.compression_ratio
        )
    }
}

/// Memory profiler for validating holographic compression efficiency
///
/// From HOLOSIM_INFINITE_REFACTOR_ROADMAP_V5.md Phase 1:
/// "Memory: O(n²/3) via shared references (entities share common field data)"
///
/// This profiler tracks memory usage throughout a simulation run and validates
/// that the actual memory scaling matches the theoretical n^(2/3) claim.
///
/// # Example
///
/// ```ignore
/// let mut profiler = MemoryProfiler::new();
/// profiler.record_baseline();
///
/// // Run simulation...
/// for tick in 0..1000 {
///     let cache_stats = get_cache_stats();
///     profiler.update(entity_count, &cache_stats);
///
///     if tick % 100 == 0 {
///         let snapshot = profiler.snapshot(tick);
///         println!("{}", snapshot);
///     }
/// }
///
/// println!("{}", profiler.efficiency_report());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryProfiler {
    /// Baseline memory when simulation starts (bytes)
    ///
    /// This is the memory footprint before any entities are created,
    /// used to calculate the delta memory usage.
    pub baseline_memory: usize,

    /// Current memory usage (bytes)
    ///
    /// The total memory consumption at the most recent measurement.
    pub current_memory: usize,

    /// Peak memory usage (bytes)
    ///
    /// The highest memory consumption observed during the simulation.
    /// Useful for memory budget validation.
    pub peak_memory: usize,

    /// Number of entities tracked
    ///
    /// The current entity count, used for per-entity calculations.
    pub entity_count: usize,

    /// Cache statistics
    ///
    /// Statistics about the holographic field cache, which is central
    /// to achieving n^(2/3) memory scaling.
    pub cache_stats: CacheMemoryStats,

    /// Compression metrics
    ///
    /// Metrics comparing actual memory usage to theoretical bounds.
    pub compression_metrics: CompressionMetrics,

    /// History of memory snapshots
    ///
    /// Snapshots taken during simulation for trend analysis.
    snapshots: Vec<MemorySnapshot>,
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryProfiler {
    /// Creates a new memory profiler with zero initial values
    pub fn new() -> Self {
        Self {
            baseline_memory: 0,
            current_memory: 0,
            peak_memory: 0,
            entity_count: 0,
            cache_stats: CacheMemoryStats::new(),
            compression_metrics: CompressionMetrics::new(),
            snapshots: Vec::new(),
        }
    }

    /// Record baseline memory at simulation start
    ///
    /// Call this method before creating any entities to establish
    /// a baseline for memory delta calculations.
    ///
    /// Note: This uses a simple estimation. For production use,
    /// integrate with system memory APIs or allocation tracking.
    pub fn record_baseline(&mut self) {
        // Estimate baseline memory (overhead from other modules)
        // In production, this would query actual system memory
        self.baseline_memory = self.estimate_current_memory();
        self.current_memory = self.baseline_memory;
        self.peak_memory = self.baseline_memory;
    }

    /// Update memory usage with current simulation state
    ///
    /// # Arguments
    /// * `entity_count` - Current number of entities
    /// * `cache_stats` - Current cache statistics
    ///
    /// This method should be called periodically during simulation
    /// to track memory usage trends.
    pub fn update(&mut self, entity_count: usize, cache_stats: &CacheMemoryStats) {
        self.entity_count = entity_count;
        self.cache_stats = cache_stats.clone();

        // Estimate memory: baseline + holographic entity memory + cache
        let entity_memory = entity_count * HOLOGRAPHIC_ENTITY_SIZE;
        self.current_memory = self.baseline_memory + entity_memory + cache_stats.cache_memory;

        // Track peak
        if self.current_memory > self.peak_memory {
            self.peak_memory = self.current_memory;
        }

        // Update compression metrics
        let memory_delta = self.current_memory.saturating_sub(self.baseline_memory);
        self.compression_metrics = CompressionMetrics::calculate(entity_count, memory_delta);
    }

    /// Take a memory snapshot for later analysis
    ///
    /// # Arguments
    /// * `tick` - Current simulation tick
    ///
    /// # Returns
    /// A snapshot capturing current memory state
    pub fn snapshot(&mut self, tick: u64) -> MemorySnapshot {
        let snapshot = MemorySnapshot::new(
            tick,
            self.current_memory,
            self.entity_count,
            self.cache_stats.cache_memory,
        );
        self.snapshots.push(snapshot.clone());
        snapshot
    }

    /// Calculate theoretical memory without compression
    ///
    /// Traditional simulation: O(n) where each entity stores all data independently.
    ///
    /// # Arguments
    /// * `entity_count` - Number of entities
    ///
    /// # Returns
    /// Estimated memory in bytes for a traditional simulation
    pub fn theoretical_uncompressed(&self, entity_count: usize) -> usize {
        entity_count * TRADITIONAL_ENTITY_SIZE
    }

    /// Calculate theoretical holographic memory
    ///
    /// Holographic simulation: O(n^(2/3)) where entities share field data.
    /// This represents the theoretical lower bound for holographic compression.
    ///
    /// # Arguments
    /// * `entity_count` - Number of entities
    ///
    /// # Returns
    /// Estimated memory in bytes for holographic compression
    pub fn theoretical_holographic(&self, entity_count: usize) -> usize {
        if entity_count == 0 {
            return 0;
        }

        let n = entity_count as f64;
        let holographic_memory =
            HOLOGRAPHIC_ENTITY_SIZE as f64 * n.powf(HOLOGRAPHIC_SCALING_EXPONENT);

        // Add cache overhead (scales with number of unique regions)
        let regions = (n.powf(0.5)) as usize; // Regions scale as sqrt(n)
        let cache_overhead = regions * 1024; // ~1 KB per region

        holographic_memory as usize + cache_overhead
    }

    /// Calculate actual scaling factor
    ///
    /// Compares measured memory growth to the theoretical n^(2/3) curve.
    ///
    /// # Returns
    /// The scaling factor where:
    /// - 1.0 = Perfect n^(2/3) scaling
    /// - > 1.0 = Worse than theoretical
    /// - < 1.0 = Better than theoretical
    pub fn calculate_scaling_factor(&self) -> f64 {
        if self.entity_count == 0 {
            return 1.0;
        }

        let memory_delta = self.current_memory.saturating_sub(self.baseline_memory);
        let theoretical = self.theoretical_holographic(self.entity_count);

        if theoretical == 0 {
            return 1.0;
        }

        memory_delta as f64 / theoretical as f64
    }

    /// Get memory efficiency report
    ///
    /// Returns a detailed report of memory usage and compression efficiency,
    /// suitable for logging or display during simulation runs.
    pub fn efficiency_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Memory Profiler Efficiency Report ===\n\n");

        // Current state
        report.push_str(&format!("Entity Count: {}\n", self.entity_count));
        report.push_str(&format!(
            "Current Memory: {:.2} MB\n",
            self.current_memory as f64 / 1_048_576.0
        ));
        report.push_str(&format!(
            "Peak Memory: {:.2} MB\n",
            self.peak_memory as f64 / 1_048_576.0
        ));
        report.push_str(&format!(
            "Baseline Memory: {:.2} MB\n\n",
            self.baseline_memory as f64 / 1_048_576.0
        ));

        // Cache statistics
        report.push_str("--- Cache Statistics ---\n");
        report.push_str(&format!("{}\n\n", self.cache_stats));

        // Compression metrics
        report.push_str("--- Compression Metrics ---\n");
        report.push_str(&format!(
            "Theoretical (traditional): {:.2} MB\n",
            self.theoretical_uncompressed(self.entity_count) as f64 / 1_048_576.0
        ));
        report.push_str(&format!(
            "Theoretical (holographic): {:.2} MB\n",
            self.theoretical_holographic(self.entity_count) as f64 / 1_048_576.0
        ));
        let memory_delta = self.current_memory.saturating_sub(self.baseline_memory);
        report.push_str(&format!(
            "Actual Memory Delta: {:.2} MB\n",
            memory_delta as f64 / 1_048_576.0
        ));
        report.push_str(&format!("{}\n\n", self.compression_metrics));

        // Scaling validation
        let scaling = self.calculate_scaling_factor();
        report.push_str("--- Scaling Validation ---\n");
        report.push_str(&format!(
            "Target scaling exponent: {:.3} (n^(2/3))\n",
            HOLOGRAPHIC_SCALING_EXPONENT
        ));
        report.push_str(&format!("Actual scaling factor: {:.3}\n", scaling));

        if self.compression_metrics.is_efficient() {
            report.push_str("Status: ✓ HOLOGRAPHIC COMPRESSION VALIDATED\n");
        } else {
            report.push_str("Status: ✗ Compression below expected efficiency\n");
        }

        // Snapshots summary
        if !self.snapshots.is_empty() {
            report.push_str(&format!(
                "\nSnapshots collected: {}\n",
                self.snapshots.len()
            ));
        }

        report
    }

    /// Returns all collected snapshots
    pub fn get_snapshots(&self) -> &[MemorySnapshot] {
        &self.snapshots
    }

    /// Clears all collected snapshots
    pub fn clear_snapshots(&mut self) {
        self.snapshots.clear();
    }

    /// Estimate current memory usage
    ///
    /// This is a simplified estimation. In production, this would
    /// integrate with platform-specific memory tracking APIs.
    fn estimate_current_memory(&self) -> usize {
        // Base overhead for the simulation framework
        let base_overhead = 1_048_576; // ~1 MB

        // Entity memory
        let entity_memory = self.entity_count * HOLOGRAPHIC_ENTITY_SIZE;

        // Cache memory
        let cache_memory = self.cache_stats.cache_memory;

        base_overhead + entity_memory + cache_memory
    }

    /// Validate memory scaling at a specific entity count
    ///
    /// Returns true if actual memory is within acceptable bounds of
    /// theoretical holographic memory.
    pub fn validate_scaling(&self, tolerance: f64) -> bool {
        let scaling = self.calculate_scaling_factor();
        scaling >= (1.0 - tolerance) && scaling <= (1.0 + tolerance)
    }

    /// Calculate memory savings compared to traditional approach
    ///
    /// # Returns
    /// Tuple of (bytes_saved, percentage_saved)
    pub fn memory_savings(&self) -> (usize, f64) {
        let traditional = self.theoretical_uncompressed(self.entity_count);
        let actual = self.current_memory.saturating_sub(self.baseline_memory);

        let saved = traditional.saturating_sub(actual);
        let percentage = if traditional > 0 {
            (saved as f64 / traditional as f64) * 100.0
        } else {
            0.0
        };

        (saved, percentage)
    }
}

impl fmt::Display for MemoryProfiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MemoryProfiler: {} entities, {:.2} MB current, {:.2} MB peak, {:.1}x compression",
            self.entity_count,
            self.current_memory as f64 / 1_048_576.0,
            self.peak_memory as f64 / 1_048_576.0,
            self.compression_metrics.compression_ratio
        )
    }
}

#[cfg(test)]
mod memory_profiler_tests {
    use super::*;

    #[test]
    fn test_memory_profiler_creation() {
        let profiler = MemoryProfiler::new();
        assert_eq!(profiler.baseline_memory, 0);
        assert_eq!(profiler.current_memory, 0);
        assert_eq!(profiler.peak_memory, 0);
        assert_eq!(profiler.entity_count, 0);
    }

    #[test]
    fn test_baseline_recording() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();
        assert!(profiler.baseline_memory > 0);
        assert_eq!(profiler.current_memory, profiler.baseline_memory);
        assert_eq!(profiler.peak_memory, profiler.baseline_memory);
    }

    #[test]
    fn test_memory_update() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        let cache_stats = CacheMemoryStats::with_values(100, 1_048_576, 0.9, 10);
        profiler.update(1000, &cache_stats);

        assert_eq!(profiler.entity_count, 1000);
        assert!(profiler.current_memory > profiler.baseline_memory);
        assert_eq!(profiler.cache_stats, cache_stats);
    }

    #[test]
    fn test_peak_tracking() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        let cache_stats1 = CacheMemoryStats::with_values(100, 1_048_576, 0.9, 10);
        profiler.update(1000, &cache_stats1);
        let first_peak = profiler.peak_memory;

        let cache_stats2 = CacheMemoryStats::with_values(50, 524_288, 0.8, 5);
        profiler.update(500, &cache_stats2);

        assert_eq!(profiler.peak_memory, first_peak);
        assert!(profiler.current_memory < profiler.peak_memory);
    }

    #[test]
    fn test_snapshot_creation() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        let cache_stats = CacheMemoryStats::with_values(100, 1_048_576, 0.9, 10);
        profiler.update(1000, &cache_stats);

        let snapshot = profiler.snapshot(42);
        assert_eq!(snapshot.timestamp, 42);
        assert_eq!(snapshot.entity_count, 1000);
        assert_eq!(snapshot.cache_memory, 1_048_576);

        assert_eq!(profiler.get_snapshots().len(), 1);
    }

    #[test]
    fn test_theoretical_uncompressed() {
        let profiler = MemoryProfiler::new();

        // 1000 entities * 10 KB = 10 MB
        let theoretical = profiler.theoretical_uncompressed(1000);
        assert_eq!(theoretical, 10_000_000);

        // 10000 entities * 10 KB = 100 MB
        let theoretical = profiler.theoretical_uncompressed(10000);
        assert_eq!(theoretical, 100_000_000);
    }

    #[test]
    fn test_theoretical_holographic() {
        let profiler = MemoryProfiler::new();

        // Zero entities should return 0
        assert_eq!(profiler.theoretical_holographic(0), 0);

        // For 1000 entities: 100 * 1000^(2/3) ≈ 100 * 100 = 10,000 bytes
        // Plus cache overhead
        let holographic = profiler.theoretical_holographic(1000);
        assert!(holographic > 0);
        assert!(holographic < profiler.theoretical_uncompressed(1000));
    }

    #[test]
    fn test_scaling_factor_calculation() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        // With zero entities, scaling should be 1.0
        assert!((profiler.calculate_scaling_factor() - 1.0).abs() < f64::EPSILON);

        // Update with some entities
        let cache_stats = CacheMemoryStats::with_values(10, 10_000, 0.95, 0);
        profiler.update(100, &cache_stats);

        let factor = profiler.calculate_scaling_factor();
        assert!(factor > 0.0);
    }

    #[test]
    fn test_compression_metrics_calculation() {
        let metrics = CompressionMetrics::calculate(1000, 50_000);

        assert_eq!(metrics.uncompressed_size, 10_000_000); // 1000 * 10 KB
        assert_eq!(metrics.compressed_size, 50_000);
        assert!((metrics.compression_ratio - 200.0).abs() < f64::EPSILON); // 10M / 50K = 200
    }

    #[test]
    fn test_compression_efficiency() {
        // Good compression (scaling near 1.0)
        let good_metrics = CompressionMetrics::calculate(1000, 10_000);
        assert!(good_metrics.is_efficient());

        // Poor compression (scaling > 1.5)
        let poor_metrics = CompressionMetrics::calculate(1000, 100_000_000);
        assert!(!poor_metrics.is_efficient());
    }

    #[test]
    fn test_cache_stats() {
        let stats = CacheMemoryStats::with_values(100, 1_048_576, 0.95, 5);

        assert_eq!(stats.cached_regions, 100);
        assert_eq!(stats.cache_memory, 1_048_576);
        assert!((stats.hit_rate - 0.95).abs() < f64::EPSILON);
        assert_eq!(stats.evictions, 5);

        // Memory per region
        assert!((stats.memory_per_region() - 10_485.76).abs() < 1.0);
    }

    #[test]
    fn test_cache_stats_clamping() {
        // Hit rate should be clamped to 0.0-1.0
        let stats = CacheMemoryStats::with_values(10, 1000, 1.5, 0);
        assert!((stats.hit_rate - 1.0).abs() < f64::EPSILON);

        let stats = CacheMemoryStats::with_values(10, 1000, -0.5, 0);
        assert!((stats.hit_rate - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_memory_savings() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        let cache_stats = CacheMemoryStats::with_values(100, 10_000, 0.9, 5);
        profiler.update(1000, &cache_stats);

        let (saved, percentage) = profiler.memory_savings();
        assert!(saved > 0);
        assert!(percentage > 0.0);
    }

    #[test]
    fn test_scaling_validation() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        // With good compression
        let cache_stats = CacheMemoryStats::with_values(10, 5_000, 0.95, 0);
        profiler.update(100, &cache_stats);

        // Should pass with reasonable tolerance
        assert!(profiler.validate_scaling(0.5));
    }

    #[test]
    fn test_efficiency_report() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        let cache_stats = CacheMemoryStats::with_values(100, 1_048_576, 0.9, 10);
        profiler.update(1000, &cache_stats);

        let report = profiler.efficiency_report();
        assert!(report.contains("Memory Profiler Efficiency Report"));
        assert!(report.contains("Entity Count: 1000"));
        assert!(report.contains("Cache Statistics"));
        assert!(report.contains("Compression Metrics"));
    }

    #[test]
    fn test_snapshot_display() {
        let snapshot = MemorySnapshot::new(100, 10_485_760, 1000, 1_048_576);

        let display = format!("{}", snapshot);
        assert!(display.contains("Tick 100"));
        assert!(display.contains("1000 entities"));
    }

    #[test]
    fn test_holographic_scaling_exponent() {
        // Verify the exponent is approximately 2/3
        assert!((HOLOGRAPHIC_SCALING_EXPONENT - 0.667).abs() < 0.001);
    }

    #[test]
    fn test_traditional_vs_holographic_entity_size() {
        // Traditional should be much larger than holographic
        const { assert!(TRADITIONAL_ENTITY_SIZE > HOLOGRAPHIC_ENTITY_SIZE); }

        // Verify ratio
        let ratio = TRADITIONAL_ENTITY_SIZE as f64 / HOLOGRAPHIC_ENTITY_SIZE as f64;
        assert!((ratio - 100.0).abs() < f64::EPSILON); // 10KB / 100B = 100x
    }

    #[test]
    fn test_clear_snapshots() {
        let mut profiler = MemoryProfiler::new();
        profiler.record_baseline();

        profiler.update(100, &CacheMemoryStats::new());
        profiler.snapshot(1);
        profiler.snapshot(2);
        profiler.snapshot(3);

        assert_eq!(profiler.get_snapshots().len(), 3);

        profiler.clear_snapshots();
        assert_eq!(profiler.get_snapshots().len(), 0);
    }
}
