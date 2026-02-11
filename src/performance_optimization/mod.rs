// ============================================================================
// PHASE 9: PERFORMANCE OPTIMIZATION
// ============================================================================
// Objective: Optimize performance while preserving emergence and holographic properties.
//
// Tasks:
// 1. Optimize Archetype Activation Calculations (5 days)
// 2. Optimize Holographic Reference Access (4 days)
// 3. Optimize Field Aggregation (4 days)
// 4. Optimize Force Calculations (4 days)
// 5. Profile and Optimize Memory (4 days)
//
// Deliverables:
// - Optimized archetype activation
// - Optimized holographic reference access
// - Optimized field aggregation
// - Optimized force calculations
// - Performance benchmarks
// - Memory usage reports
//
// Success Criteria:
// - Performance overhead < 10%
// - Memory usage efficient
// - O(1) access maintained
// - All optimizations preserve emergence
// ============================================================================

pub mod archetype_activation_optimization;
pub mod field_aggregation_optimization;
pub mod force_calculation_optimization;
pub mod holographic_reference_optimization;
pub mod memory_optimization;
pub mod performance_benchmarks;
pub mod profiling_tools;

// Re-export main types for convenience
pub use archetype_activation_optimization::{
    ArchetypeActivation, ArchetypeActivationCache, ArchetypeActivationOptimizer,
    ArchetypeActivationStats, CachedArchetypeActivation,
};
pub use field_aggregation_optimization::{
    FieldAggregationCache, FieldAggregationOptimizer, FieldAggregationStats,
    SpatialPartitioningOptimizer,
};
pub use force_calculation_optimization::{
    ArchetypeActivation as ForceArchetypeActivation, ForceCalculationCache,
    ForceCalculationOptimizer, ForceCalculationStats, ForceType,
};
pub use holographic_reference_optimization::{
    HolographicReferenceCache, HolographicReferenceOptimizer, HolographicReferenceStats,
    ReferencePool,
};
pub use memory_optimization::{
    ArchetypeActivation as MemoryArchetypeActivation, MemoryOptimizer, MemoryPoolConfig,
    MemoryProfile, MemoryUsageReport,
};
pub use performance_benchmarks::{
    BenchmarkResult, PerformanceBenchmark, PerformanceMetrics, PerformanceReport,
};
pub use profiling_tools::{
    HotSpotAnalysis, HotSpotSeverity, Profiler, ProfilingSession, ProfilingStats,
};
