// ============================================================================
// TASK 5: PROFILE AND OPTIMIZE MEMORY
// ============================================================================
// Objective: Profile memory usage and optimize memory allocation patterns.
//
// Tasks:
// - Profile memory usage
// - Optimize Arc sharing
// - Optimize archetype activation storage
// - Optimize field storage
// - Implement memory pooling
// ============================================================================

use crate::archetypes::LambdaMeasurement;
use crate::energy_fields::Vector3;
use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

// Placeholder for ArchetypeActivation (used in performance optimization)
#[derive(Debug, Clone)]
pub struct ArchetypeActivation {
    pub archetype_id: u8,
    pub activation_level: f64,
    pub coherence: f64,
    pub polarization: f64,
    pub lambda: LambdaMeasurement,
}

// ============================================================================
// MEMORY PROFILE
// ============================================================================

/// Memory usage profile for different components
#[derive(Debug, Clone, Default)]
pub struct MemoryProfile {
    /// Total memory used (in bytes)
    pub total_memory_bytes: u64,
    /// Memory used by holographic seeds (in bytes)
    pub seed_memory_bytes: u64,
    /// Memory used by archetype activations (in bytes)
    pub archetype_memory_bytes: u64,
    /// Memory used by field data (in bytes)
    pub field_memory_bytes: u64,
    /// Memory used by Arc references (in bytes)
    pub arc_memory_bytes: u64,
    /// Number of Arc references
    pub arc_count: u64,
    /// Number of allocations
    pub allocation_count: u64,
    /// Number of deallocations
    pub deallocation_count: u64,
    /// Peak memory usage (in bytes)
    pub peak_memory_bytes: u64,
}

impl MemoryProfile {
    /// Get total memory in kilobytes
    pub fn total_memory_kb(&self) -> f64 {
        self.total_memory_bytes as f64 / 1024.0
    }

    /// Get total memory in megabytes
    pub fn total_memory_mb(&self) -> f64 {
        self.total_memory_bytes as f64 / (1024.0 * 1024.0)
    }

    /// Get memory usage percentage
    pub fn memory_usage_percentage(&self, component_bytes: u64) -> f64 {
        if self.total_memory_bytes == 0 {
            return 0.0;
        }
        (component_bytes as f64 / self.total_memory_bytes as f64) * 100.0
    }

    /// Update total memory and track peak
    pub fn update_total_memory(&mut self, memory_bytes: u64) {
        self.total_memory_bytes = memory_bytes;
        if memory_bytes > self.peak_memory_bytes {
            self.peak_memory_bytes = memory_bytes;
        }
    }

    /// Record an allocation
    pub fn record_allocation(&mut self, size: u64) {
        self.allocation_count += 1;
        self.total_memory_bytes += size;
        if self.total_memory_bytes > self.peak_memory_bytes {
            self.peak_memory_bytes = self.total_memory_bytes;
        }
    }

    /// Record a deallocation
    pub fn record_deallocation(&mut self, size: u64) {
        self.deallocation_count += 1;
        self.total_memory_bytes = self.total_memory_bytes.saturating_sub(size);
    }
}

// ============================================================================
// MEMORY USAGE REPORT
// ============================================================================

/// Detailed memory usage report
#[derive(Debug, Clone)]
pub struct MemoryUsageReport {
    /// Memory profile
    pub profile: MemoryProfile,
    /// Timestamp of the report
    pub timestamp: Instant,
    /// Memory efficiency score (0.0 to 1.0)
    pub efficiency_score: f64,
    /// Memory leak detected
    pub memory_leak_detected: bool,
    /// Recommendations for optimization
    pub recommendations: Vec<String>,
}

impl MemoryUsageReport {
    /// Create a new memory usage report
    pub fn new(profile: MemoryProfile) -> Self {
        let efficiency_score = Self::calculate_efficiency_score(&profile);
        let memory_leak_probability = Self::detect_memory_leak(&profile);
        let memory_leak_detected = memory_leak_probability > 0.5; // Threshold
        let recommendations = Self::generate_recommendations(&profile);

        Self {
            profile,
            timestamp: Instant::now(),
            efficiency_score,
            memory_leak_detected,
            recommendations,
        }
    }

    /// Calculate memory efficiency score
    fn calculate_efficiency_score(profile: &MemoryProfile) -> f64 {
        if profile.allocation_count == 0 {
            return 1.0;
        }

        // Efficiency based on deallocation/allocation ratio
        let deallocation_ratio = if profile.allocation_count > 0 {
            profile.deallocation_count as f64 / profile.allocation_count as f64
        } else {
            0.0
        };

        // Penalty for high arc count
        let arc_penalty = if profile.arc_count > 1000 {
            (profile.arc_count as f64 - 1000.0) / 10000.0
        } else {
            0.0
        };

        let efficiency = deallocation_ratio - arc_penalty;
        efficiency.max(0.0).min(1.0)
    }

    /// Detect potential memory leak (returns probability 0.0 to 1.0)
    fn detect_memory_leak(profile: &MemoryProfile) -> f64 {
        if profile.allocation_count == 0 {
            return 0.0;
        }

        // Consider leak if deallocation ratio is too low
        let deallocation_ratio =
            profile.deallocation_count as f64 / profile.allocation_count as f64;
        if deallocation_ratio < 0.5 {
            1.0 - deallocation_ratio // Higher probability for lower deallocation ratio
        } else {
            0.0
        }
    }

    /// Generate optimization recommendations
    fn generate_recommendations(profile: &MemoryProfile) -> Vec<String> {
        let mut recommendations = Vec::new();

        if profile.arc_count > 1000 {
            recommendations.push(
                "Consider reducing Arc cloning overhead by using reference pooling".to_string(),
            );
        }

        if profile.seed_memory_bytes > profile.total_memory_bytes * 50 / 100 {
            recommendations.push(
                "Holographic seed memory usage is high - consider shared references".to_string(),
            );
        }

        if profile.archetype_memory_bytes > profile.total_memory_bytes * 30 / 100 {
            recommendations
                .push("Archetype activation memory usage is high - consider caching".to_string());
        }

        if profile.field_memory_bytes > profile.total_memory_bytes * 40 / 100 {
            recommendations
                .push("Field memory usage is high - consider spatial partitioning".to_string());
        }

        if profile.allocation_count > 10000
            && profile.deallocation_count < profile.allocation_count / 2
        {
            recommendations.push(
                "Potential memory leak detected - review allocation/deallocation patterns"
                    .to_string(),
            );
        }

        recommendations
    }
}

// ============================================================================
// MEMORY POOL CONFIG
// ============================================================================

/// Configuration for memory pool
#[derive(Debug, Clone)]
pub struct MemoryPoolConfig {
    /// Maximum pool size (in bytes)
    pub max_pool_size_bytes: u64,
    /// Initial pool size (in bytes)
    pub initial_pool_size_bytes: u64,
    /// Block size for allocations (in bytes)
    pub block_size_bytes: u64,
    /// Enable/disable automatic pool expansion
    pub auto_expand: bool,
    /// Enable/disable automatic pool shrinking
    pub auto_shrink: bool,
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            max_pool_size_bytes: 100 * 1024 * 1024,    // 100 MB
            initial_pool_size_bytes: 10 * 1024 * 1024, // 10 MB
            block_size_bytes: 1024,                    // 1 KB
            auto_expand: true,
            auto_shrink: true,
        }
    }
}

// ============================================================================
// MEMORY OPTIMIZER
// ============================================================================

/// Optimizer for memory usage
#[derive(Debug)]
pub struct MemoryOptimizer {
    /// Memory profile
    profile: MemoryProfile,
    /// Memory pool configuration
    pool_config: MemoryPoolConfig,
    /// Arc reference tracking
    arc_tracking: HashMap<usize, u64>, // Pointer address -> reference count
    /// Statistics
    stats: MemoryProfile,
    /// Enable/disable optimization
    optimization_enabled: bool,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer
    pub fn new() -> Self {
        Self {
            profile: MemoryProfile::default(),
            pool_config: MemoryPoolConfig::default(),
            arc_tracking: HashMap::new(),
            stats: MemoryProfile::default(),
            optimization_enabled: true,
        }
    }

    /// Create a new optimizer with custom pool configuration
    pub fn with_pool_config(pool_config: MemoryPoolConfig) -> Self {
        Self {
            profile: MemoryProfile::default(),
            pool_config,
            arc_tracking: HashMap::new(),
            stats: MemoryProfile::default(),
            optimization_enabled: true,
        }
    }

    /// Enable or disable optimization
    pub fn set_optimization_enabled(&mut self, enabled: bool) {
        self.optimization_enabled = enabled;
    }

    /// Track Arc creation
    pub fn track_arc_creation<T>(&mut self, arc: &Arc<T>) {
        if !self.optimization_enabled {
            return;
        }

        let address = arc.as_ref() as *const T as usize;
        let ref_count = Arc::strong_count(arc) as u64;

        self.arc_tracking.insert(address, ref_count);
        self.profile.arc_count = self.arc_tracking.len() as u64;
        self.profile.arc_memory_bytes =
            self.profile.arc_count * std::mem::size_of::<Arc<T>>() as u64;
        self.stats
            .record_allocation(std::mem::size_of::<Arc<T>>() as u64);
    }

    /// Track Arc clone
    pub fn track_arc_clone<T>(&mut self, arc: &Arc<T>) {
        if !self.optimization_enabled {
            return;
        }

        let address = arc.as_ref() as *const T as usize;
        if let Some(ref_count) = self.arc_tracking.get_mut(&address) {
            *ref_count += 1;
        }
    }

    /// Track Arc drop
    pub fn track_arc_drop<T>(&mut self, arc: &Arc<T>) {
        if !self.optimization_enabled {
            return;
        }

        let address = arc.as_ref() as *const T as usize;
        if let Some(ref_count) = self.arc_tracking.get_mut(&address) {
            if *ref_count > 0 {
                *ref_count -= 1;
            }

            // Remove from tracking if reference count is 0
            if *ref_count == 0 {
                self.arc_tracking.remove(&address);
                self.profile.arc_count = self.arc_tracking.len() as u64;
                self.profile.arc_memory_bytes =
                    self.profile.arc_count * std::mem::size_of::<Arc<T>>() as u64;
                self.stats
                    .record_deallocation(std::mem::size_of::<Arc<T>>() as u64);
            }
        }
    }

    /// Optimize Arc sharing by deduplicating identical references
    pub fn optimize_arc_sharing<T>(&mut self, arcs: &mut Vec<Arc<T>>) -> usize {
        if !self.optimization_enabled {
            return 0;
        }

        let mut dedup_count = 0;
        let mut seen: HashMap<usize, Arc<T>> = HashMap::new();

        for i in 0..arcs.len() {
            let address = arcs[i].as_ref() as *const T as usize;

            if let Some(existing) = seen.get(&address) {
                // Replace with existing reference
                arcs[i] = existing.clone();
                dedup_count += 1;
            } else {
                seen.insert(address, arcs[i].clone());
            }
        }

        dedup_count
    }

    /// Estimate memory usage for holographic seed
    pub fn estimate_seed_memory(&self, seed: &HolographicSeed) -> u64 {
        std::mem::size_of_val(seed) as u64
    }

    /// Estimate memory usage for archetype activation
    pub fn estimate_archetype_memory(&self, activation: &ArchetypeActivation) -> u64 {
        std::mem::size_of_val(activation) as u64
    }

    /// Estimate memory usage for field data
    pub fn estimate_field_memory(&self, field: &Vector3) -> u64 {
        std::mem::size_of_val(field) as u64
    }

    /// Generate memory usage report
    pub fn generate_report(&self) -> MemoryUsageReport {
        MemoryUsageReport::new(self.profile.clone())
    }

    /// Get memory profile
    pub fn profile(&self) -> &MemoryProfile {
        &self.profile
    }

    /// Get memory statistics
    pub fn stats(&self) -> &MemoryProfile {
        &self.stats
    }

    /// Get Arc tracking information
    pub fn arc_tracking(&self) -> &HashMap<usize, u64> {
        &self.arc_tracking
    }

    /// Get pool configuration
    pub fn pool_config(&self) -> &MemoryPoolConfig {
        &self.pool_config
    }

    /// Reset memory profile
    pub fn reset_profile(&mut self) {
        self.profile = MemoryProfile::default();
        self.arc_tracking.clear();
    }

    /// Get memory efficiency score
    pub fn efficiency_score(&self) -> f64 {
        MemoryUsageReport::new(self.profile.clone()).efficiency_score
    }

    /// Check for memory leak
    pub fn detect_memory_leak(&self) -> bool {
        MemoryUsageReport::new(self.profile.clone()).memory_leak_detected
    }

    /// Get optimization recommendations
    pub fn recommendations(&self) -> Vec<String> {
        MemoryUsageReport::new(self.profile.clone()).recommendations
    }
}

// ============================================================================
// OPTIMIZATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_profile_basic() {
        let mut profile = MemoryProfile::default();

        profile.record_allocation(1024);
        profile.record_allocation(2048);
        profile.record_deallocation(1024);

        assert_eq!(profile.allocation_count, 2);
        assert_eq!(profile.deallocation_count, 1);
        assert_eq!(profile.total_memory_bytes, 2048);
    }

    #[test]
    fn test_memory_profile_peak_memory() {
        let mut profile = MemoryProfile::default();

        profile.record_allocation(1024);
        profile.record_allocation(2048);
        profile.record_deallocation(1024);

        assert_eq!(profile.peak_memory_bytes, 3072);
    }

    #[test]
    fn test_memory_profile_memory_usage_percentage() {
        let mut profile = MemoryProfile::default();

        profile.total_memory_bytes = 1000;
        let percentage = profile.memory_usage_percentage(500);

        assert!((percentage - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_memory_usage_report_basic() {
        let profile = MemoryProfile::default();
        let report = MemoryUsageReport::new(profile);

        assert!(report.efficiency_score >= 0.0);
        assert!(report.efficiency_score <= 1.0);
    }

    #[test]
    fn test_memory_usage_report_efficiency_score() {
        let mut profile = MemoryProfile::default();
        profile.allocation_count = 100;
        profile.deallocation_count = 80;

        let report = MemoryUsageReport::new(profile);
        assert!((report.efficiency_score - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_memory_usage_report_memory_leak_detection() {
        let mut profile = MemoryProfile::default();
        profile.allocation_count = 100;
        profile.deallocation_count = 30;

        let report = MemoryUsageReport::new(profile);
        assert!(report.memory_leak_detected);
    }

    #[test]
    fn test_memory_pool_config_default() {
        let config = MemoryPoolConfig::default();

        assert_eq!(config.max_pool_size_bytes, 100 * 1024 * 1024);
        assert_eq!(config.initial_pool_size_bytes, 10 * 1024 * 1024);
        assert_eq!(config.block_size_bytes, 1024);
        assert!(config.auto_expand);
        assert!(config.auto_shrink);
    }

    #[test]
    fn test_memory_optimizer_basic() {
        let optimizer = MemoryOptimizer::new();

        assert_eq!(optimizer.profile().total_memory_bytes, 0);
        assert!(optimizer.optimization_enabled);
    }

    #[test]
    fn test_memory_optimizer_arc_tracking() {
        let mut optimizer = MemoryOptimizer::new();
        let arc = Arc::new(42);

        optimizer.track_arc_creation(&arc);
        assert_eq!(optimizer.arc_tracking().len(), 1);

        optimizer.track_arc_clone(&arc);
        assert_eq!(optimizer.arc_tracking().len(), 1);

        optimizer.track_arc_drop(&arc);
        assert_eq!(optimizer.arc_tracking().len(), 0);
    }

    #[test]
    fn test_memory_optimizer_arc_sharing_optimization() {
        let mut optimizer = MemoryOptimizer::new();
        let shared_arc = Arc::new(42);

        let mut arcs = vec![
            shared_arc.clone(),
            shared_arc.clone(),
            Arc::new(42),
            shared_arc.clone(),
        ];

        let dedup_count = optimizer.optimize_arc_sharing(&mut arcs);
        assert!(dedup_count > 0);
    }

    #[test]
    fn test_memory_optimizer_estimates() {
        let optimizer = MemoryOptimizer::new();

        let seed = HolographicSeed::new_from_source();
        let seed_memory = optimizer.estimate_seed_memory(&seed);
        assert!(seed_memory > 0);

        let activation = ArchetypeActivation {
            archetype_id: 1,
            activation_level: 0.5,
            coherence: 0.8,
            polarization: 0.7,
            lambda: crate::archetypes::LambdaMeasurement {
                value: 0.6,
                permeability: 0.5,
                coherence: 0.8,
            },
        };
        let archetype_memory = optimizer.estimate_archetype_memory(&activation);
        assert!(archetype_memory > 0);

        let field = Vector3::new(1.0, 2.0, 3.0);
        let field_memory = optimizer.estimate_field_memory(&field);
        assert!(field_memory > 0);
    }

    #[test]
    fn test_memory_optimizer_generate_report() {
        let optimizer = MemoryOptimizer::new();
        let report = optimizer.generate_report();

        assert!(report.efficiency_score >= 0.0);
        assert!(report.efficiency_score <= 1.0);
    }

    #[test]
    fn test_memory_optimizer_efficiency_score() {
        let mut optimizer = MemoryOptimizer::new();
        optimizer.profile.allocation_count = 100;
        optimizer.profile.deallocation_count = 90;

        let score = optimizer.efficiency_score();
        assert!((score - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_memory_optimizer_detect_memory_leak() {
        let mut optimizer = MemoryOptimizer::new();
        optimizer.profile.allocation_count = 100;
        optimizer.profile.deallocation_count = 30;

        assert!(optimizer.detect_memory_leak());
    }

    #[test]
    fn test_memory_optimizer_recommendations() {
        let mut optimizer = MemoryOptimizer::new();
        optimizer.profile.arc_count = 2000;

        let recommendations = optimizer.recommendations();
        assert!(!recommendations.is_empty());
    }
}
