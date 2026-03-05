//! Performance Testing Infrastructure
//!
//! From MASTER_R&D_ROADMAP.md Phase 8: Performance Testing Infrastructure (Week 114)
//!
//! This module provides a comprehensive performance testing framework for the
//! Holonic Realms holographic simulation engine.
//!
//! Features:
//! - Statistical benchmarking with mean, median, p95, p99, std_dev
//! - Performance regression detection
//! - Continuous performance monitoring
//! - Warm-up routines for Rust compilation/initialization effects
//! - Performance assertions with threshold checking
//! - Human-readable performance reports
//!
//! Performance Targets:
//! - MERA compression: 100x reduction
//! - Fractal cache: O(1) access
//! - Scale transitions: <50ms
//! - Density transitions: <100ms
//! - Physics computation: O(1) interference

pub mod benchmark;
pub mod metrics;
pub mod registry;

// Performance test modules
#[cfg(test)]
pub mod density_transitions;
#[cfg(test)]
pub mod fractal_cache;
#[cfg(test)]
pub mod mera_compression;
#[cfg(test)]
pub mod physics_engine;
#[cfg(test)]
pub mod scale_transitions;

pub use benchmark::{BenchmarkRunner, PerformanceAssertionBuilder};
pub use metrics::{CompressionRatioMetric, LatencyMetric, MemoryMetric, ThroughputMetric};
pub use registry::BenchmarkRegistry;

use crate::types::Float;
use std::time::Duration;

/// Performance Test trait
///
/// All performance tests must implement this trait to be compatible
/// with the benchmark execution engine.
///
/// From MASTER_R&D_ROADMAP.md: "Use statistical analysis (mean, median, percentiles)
/// for reliable performance measurement"
pub trait PerformanceTest: Send + Sync {
    /// Get the name of this benchmark
    fn name(&self) -> &str;

    /// Get the category of this benchmark
    fn category(&self) -> BenchmarkCategory;

    /// Get a description of what this benchmark measures
    fn description(&self) -> &str;

    /// Perform a single iteration of the benchmark
    ///
    /// This method should perform the operation being benchmarked and
    /// return the time taken for that iteration.
    fn run_iteration(&self) -> Duration;

    /// Perform warm-up iterations
    ///
    /// Warm-up iterations account for Rust compilation/initialization effects.
    /// Default implementation does 10 warm-up iterations.
    fn warmup(&self) {
        for _ in 0..10 {
            self.run_iteration();
        }
    }

    /// Get the number of benchmark iterations to run
    ///
    /// Default is 100 iterations for statistical significance.
    fn iterations(&self) -> usize {
        100
    }
}

/// Benchmark category for grouping related tests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkCategory {
    /// MERA compression performance tests
    MeraCompression,

    /// Fractal cache performance tests
    FractalCache,

    /// Scale transition performance tests
    ScaleTransitions,

    /// Density transition performance tests
    DensityTransitions,

    /// Physics engine performance tests
    PhysicsEngine,

    /// Memory performance tests
    Memory,

    /// Throughput performance tests
    Throughput,

    /// Custom category
    Custom(&'static str),
}

impl BenchmarkCategory {
    /// Get the category name as a string
    pub fn as_str(&self) -> &str {
        match self {
            BenchmarkCategory::MeraCompression => "mera_compression",
            BenchmarkCategory::FractalCache => "fractal_cache",
            BenchmarkCategory::ScaleTransitions => "scale_transitions",
            BenchmarkCategory::DensityTransitions => "density_transitions",
            BenchmarkCategory::PhysicsEngine => "physics_engine",
            BenchmarkCategory::Memory => "memory",
            BenchmarkCategory::Throughput => "throughput",
            BenchmarkCategory::Custom(name) => name,
        }
    }
}

/// Statistical benchmark result
///
/// Contains comprehensive statistical metrics for performance analysis.
/// From MASTER_R&D_ROADMAP.md: "Include warm-up iterations to account for JIT/compilation effects"
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmark
    pub name: String,

    /// Category of the benchmark
    pub category: BenchmarkCategory,

    /// Mean (average) execution time
    pub mean: Duration,

    /// Median (50th percentile) execution time
    pub median: Duration,

    /// 95th percentile execution time
    pub p95: Duration,

    /// 99th percentile execution time
    pub p99: Duration,

    /// Standard deviation of execution times
    pub std_dev: Duration,

    /// Minimum execution time
    pub min: Duration,

    /// Maximum execution time
    pub max: Duration,

    /// Number of iterations performed
    pub iterations: usize,

    /// Number of warm-up iterations performed
    pub warmup_iterations: usize,

    /// Total benchmark duration (including warm-up)
    pub total_duration: Duration,
}

impl BenchmarkResult {
    /// Create a new benchmark result from a vector of durations
    pub fn from_durations(
        name: String,
        category: BenchmarkCategory,
        durations: Vec<Duration>,
        warmup_iterations: usize,
        total_duration: Duration,
    ) -> Self {
        let iterations = durations.len();

        if iterations == 0 {
            return BenchmarkResult {
                name,
                category,
                mean: Duration::ZERO,
                median: Duration::ZERO,
                p95: Duration::ZERO,
                p99: Duration::ZERO,
                std_dev: Duration::ZERO,
                min: Duration::ZERO,
                max: Duration::ZERO,
                iterations,
                warmup_iterations,
                total_duration,
            };
        }

        // Convert to nanoseconds for calculations
        let mut nanos: Vec<u128> = durations.iter().map(|d| d.as_nanos()).collect();
        nanos.sort_unstable();

        // Calculate mean
        let mean_nanos: u128 = nanos.iter().sum::<u128>() / iterations as u128;

        // Calculate median (p50)
        let median_nanos = if iterations.is_multiple_of(2) {
            (nanos[iterations / 2 - 1] + nanos[iterations / 2]) / 2
        } else {
            nanos[iterations / 2]
        };

        // Calculate p95
        let p95_index = ((iterations as f64) * 0.95) as usize;
        let p95_nanos = nanos[p95_index.min(iterations - 1)];

        // Calculate p99
        let p99_index = ((iterations as f64) * 0.99) as usize;
        let p99_nanos = nanos[p99_index.min(iterations - 1)];

        // Calculate standard deviation
        let variance: Float = nanos
            .iter()
            .map(|&x| {
                let diff = x as Float - mean_nanos as Float;
                diff * diff
            })
            .sum::<Float>()
            / iterations as Float;
        let std_dev_nanos = variance.sqrt() as u128;

        BenchmarkResult {
            name,
            category,
            mean: Duration::from_nanos(mean_nanos as u64),
            median: Duration::from_nanos(median_nanos as u64),
            p95: Duration::from_nanos(p95_nanos as u64),
            p99: Duration::from_nanos(p99_nanos as u64),
            std_dev: Duration::from_nanos(std_dev_nanos as u64),
            min: Duration::from_nanos(nanos[0] as u64),
            max: Duration::from_nanos(nanos[iterations - 1] as u64),
            iterations,
            warmup_iterations,
            total_duration,
        }
    }

    /// Check if this result meets a performance assertion
    pub fn meets_assertion(&self, assertion: &PerformanceAssertion) -> bool {
        match assertion {
            PerformanceAssertion::MaxMean(threshold) => self.mean <= *threshold,
            PerformanceAssertion::MaxMedian(threshold) => self.median <= *threshold,
            PerformanceAssertion::MaxP95(threshold) => self.p95 <= *threshold,
            PerformanceAssertion::MaxP99(threshold) => self.p99 <= *threshold,
            PerformanceAssertion::MinThroughput(threshold) => {
                let throughput = 1.0 / self.mean.as_secs_f64();
                throughput >= *threshold
            }
        }
    }

    /// Generate a human-readable report
    pub fn report(&self) -> String {
        format!(
            "Benchmark: {} ({})\n\
             Iterations: {} (warmup: {})\n\
             Mean:      {:.6} ms\n\
             Median:    {:.6} ms\n\
             P95:       {:.6} ms\n\
             P99:       {:.6} ms\n\
             Std Dev:   {:.6} ms\n\
             Min:       {:.6} ms\n\
             Max:       {:.6} ms\n\
             Total:     {:.3} s",
            self.name,
            self.category.as_str(),
            self.iterations,
            self.warmup_iterations,
            self.mean.as_secs_f64() * 1000.0,
            self.median.as_secs_f64() * 1000.0,
            self.p95.as_secs_f64() * 1000.0,
            self.p99.as_secs_f64() * 1000.0,
            self.std_dev.as_secs_f64() * 1000.0,
            self.min.as_secs_f64() * 1000.0,
            self.max.as_secs_f64() * 1000.0,
            self.total_duration.as_secs_f64(),
        )
    }
}

/// Performance assertion for threshold checking
///
/// From MASTER_R&D_ROADMAP.md: "Support performance assertions (assert performance < threshold)"
#[derive(Debug, Clone)]
pub enum PerformanceAssertion {
    /// Assert that mean execution time is below threshold
    MaxMean(Duration),

    /// Assert that median execution time is below threshold
    MaxMedian(Duration),

    /// Assert that P95 execution time is below threshold
    MaxP95(Duration),

    /// Assert that P99 execution time is below threshold
    MaxP99(Duration),

    /// Assert that minimum throughput (operations/second) is above threshold
    MinThroughput(Float),
}

impl PerformanceAssertion {
    /// Create a max mean assertion
    pub fn max_mean_ms(ms: u64) -> Self {
        PerformanceAssertion::MaxMean(Duration::from_millis(ms))
    }

    /// Create a max median assertion
    pub fn max_median_ms(ms: u64) -> Self {
        PerformanceAssertion::MaxMedian(Duration::from_millis(ms))
    }

    /// Create a max P95 assertion
    pub fn max_p95_ms(ms: u64) -> Self {
        PerformanceAssertion::MaxP95(Duration::from_millis(ms))
    }

    /// Create a max P99 assertion
    pub fn max_p99_ms(ms: u64) -> Self {
        PerformanceAssertion::MaxP99(Duration::from_millis(ms))
    }

    /// Create a minimum throughput assertion
    pub fn min_throughput_ops_per_sec(ops: Float) -> Self {
        PerformanceAssertion::MinThroughput(ops)
    }

    /// Get a description of this assertion
    pub fn description(&self) -> String {
        match self {
            PerformanceAssertion::MaxMean(threshold) => {
                format!("Mean ≤ {}", format_duration(*threshold))
            }
            PerformanceAssertion::MaxMedian(threshold) => {
                format!("Median ≤ {}", format_duration(*threshold))
            }
            PerformanceAssertion::MaxP95(threshold) => {
                format!("P95 ≤ {}", format_duration(*threshold))
            }
            PerformanceAssertion::MaxP99(threshold) => {
                format!("P99 ≤ {}", format_duration(*threshold))
            }
            PerformanceAssertion::MinThroughput(threshold) => {
                format!("Throughput ≥ {} ops/sec", threshold)
            }
        }
    }
}

/// Format a duration in a human-readable way
fn format_duration(duration: Duration) -> String {
    let ms = duration.as_secs_f64() * 1000.0;
    if ms < 1.0 {
        format!("{:.3} μs", ms * 1000.0)
    } else if ms < 1000.0 {
        format!("{:.3} ms", ms)
    } else {
        format!("{:.3} s", ms / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result_from_durations() {
        let durations = vec![
            Duration::from_millis(10),
            Duration::from_millis(12),
            Duration::from_millis(11),
            Duration::from_millis(13),
            Duration::from_millis(9),
        ];

        let result = BenchmarkResult::from_durations(
            "test_benchmark".to_string(),
            BenchmarkCategory::Custom("test"),
            durations,
            10,
            Duration::from_secs(1),
        );

        assert_eq!(result.name, "test_benchmark");
        assert_eq!(result.iterations, 5);
        assert_eq!(result.warmup_iterations, 10);
        assert!(result.mean.as_millis() >= 9 && result.mean.as_millis() <= 13);
    }

    #[test]
    fn test_performance_assertion_max_mean() {
        let assertion = PerformanceAssertion::max_mean_ms(50);

        let result = BenchmarkResult {
            name: "test".to_string(),
            category: BenchmarkCategory::Custom("test"),
            mean: Duration::from_millis(40),
            median: Duration::from_millis(35),
            p95: Duration::from_millis(45),
            p99: Duration::from_millis(50),
            std_dev: Duration::from_millis(5),
            min: Duration::from_millis(30),
            max: Duration::from_millis(50),
            iterations: 100,
            warmup_iterations: 10,
            total_duration: Duration::from_secs(1),
        };

        assert!(result.meets_assertion(&assertion));
    }

    #[test]
    fn test_performance_assertion_fails() {
        let assertion = PerformanceAssertion::max_mean_ms(50);

        let result = BenchmarkResult {
            name: "test".to_string(),
            category: BenchmarkCategory::Custom("test"),
            mean: Duration::from_millis(60),
            median: Duration::from_millis(55),
            p95: Duration::from_millis(65),
            p99: Duration::from_millis(70),
            std_dev: Duration::from_millis(5),
            min: Duration::from_millis(50),
            max: Duration::from_millis(70),
            iterations: 100,
            warmup_iterations: 10,
            total_duration: Duration::from_secs(1),
        };

        assert!(!result.meets_assertion(&assertion));
    }
}
