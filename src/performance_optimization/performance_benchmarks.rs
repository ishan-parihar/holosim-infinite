// ============================================================================
// PERFORMANCE BENCHMARKS
// ============================================================================
// Objective: Create comprehensive performance benchmarks for all systems.
//
// Tasks:
// - Benchmark archetype activation performance
// - Benchmark holographic reference access performance
// - Benchmark field aggregation performance
// - Benchmark force calculation performance
// - Benchmark memory usage
// - Generate performance reports
// ============================================================================

use super::archetype_activation_optimization::ArchetypeActivation;
use crate::energy_fields::Vector3;
use crate::entity_layer7::holographic_blueprint::HolographicSeed;
use crate::light::Photon;
use crate::matter::{Coordinate3D, Particle};
use std::sync::Arc;
use std::time::{Duration, Instant};

// Helper function to derive mass from particle energy (E = mc^2)
fn derive_mass_from_energy(energy: f64) -> f64 {
    const SPEED_OF_LIGHT_SQUARED: f64 = 8.98755179e16; // c^2 in (m/s)^2
    energy / SPEED_OF_LIGHT_SQUARED
}

// Helper function to convert QuantumPosition to Vector3
fn position_to_vector3(position: &crate::light::QuantumPosition) -> Vector3 {
    Vector3::new(position.x, position.y, position.z)
}

// ============================================================================
// BENCHMARK RESULT
// ============================================================================

/// Result of a single benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmark
    pub name: String,
    /// Number of iterations
    pub iterations: u64,
    /// Total duration
    pub total_duration: Duration,
    /// Average duration per iteration
    pub avg_duration: Duration,
    /// Minimum duration
    pub min_duration: Duration,
    /// Maximum duration
    pub max_duration: Duration,
    /// Median duration
    pub median_duration: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// Operations per second
    pub ops_per_second: f64,
    /// Memory usage (in bytes)
    pub memory_usage_bytes: u64,
}

impl BenchmarkResult {
    /// Calculate statistics from durations
    pub fn from_durations(name: String, durations: Vec<Duration>, memory_usage_bytes: u64) -> Self {
        let iterations = durations.len() as u64;
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / iterations as u32;
        let min_duration = *durations.iter().min().unwrap();
        let max_duration = *durations.iter().max().unwrap();

        // Calculate median
        let mut sorted_durations = durations.clone();
        sorted_durations.sort();
        let median_duration = if iterations % 2 == 0 {
            let mid = iterations as usize / 2;
            (sorted_durations[mid - 1] + sorted_durations[mid]) / 2
        } else {
            sorted_durations[iterations as usize / 2]
        };

        // Calculate standard deviation
        let avg_ns = avg_duration.as_nanos() as f64;
        let variance: f64 = durations
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - avg_ns;
                diff * diff
            })
            .sum::<f64>()
            / iterations as f64;
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);

        // Calculate operations per second
        let ops_per_second = if avg_duration.as_nanos() > 0 {
            1_000_000_000.0 / avg_duration.as_nanos() as f64
        } else {
            0.0
        };

        Self {
            name,
            iterations,
            total_duration,
            avg_duration,
            min_duration,
            max_duration,
            median_duration,
            std_dev,
            ops_per_second,
            memory_usage_bytes,
        }
    }

    /// Get average duration in nanoseconds
    pub fn avg_duration_ns(&self) -> u64 {
        self.avg_duration.as_nanos() as u64
    }

    /// Get average duration in microseconds
    pub fn avg_duration_us(&self) -> f64 {
        self.avg_duration.as_nanos() as f64 / 1000.0
    }

    /// Get average duration in milliseconds
    pub fn avg_duration_ms(&self) -> f64 {
        self.avg_duration.as_nanos() as f64 / 1_000_000.0
    }

    /// Compare with another benchmark result
    pub fn compare(&self, other: &BenchmarkResult) -> BenchmarkComparison {
        let speedup = if other.avg_duration_ns() > 0 {
            other.avg_duration_ns() as f64 / self.avg_duration_ns() as f64
        } else {
            0.0
        };

        let improvement = if other.avg_duration_ns() > 0 {
            ((other.avg_duration_ns() as f64 - self.avg_duration_ns() as f64)
                / other.avg_duration_ns() as f64)
                * 100.0
        } else {
            0.0
        };

        BenchmarkComparison {
            baseline_avg_ns: other.avg_duration_ns(),
            optimized_avg_ns: self.avg_duration_ns(),
            speedup,
            improvement_percent: improvement,
        }
    }
}

// ============================================================================
// BENCHMARK COMPARISON
// ============================================================================

/// Comparison between two benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkComparison {
    /// Baseline average duration in nanoseconds
    pub baseline_avg_ns: u64,
    /// Optimized average duration in nanoseconds
    pub optimized_avg_ns: u64,
    /// Speedup factor
    pub speedup: f64,
    /// Improvement percentage
    pub improvement_percent: f64,
}

// ============================================================================
// PERFORMANCE METRICS
// ============================================================================

/// Overall performance metrics
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Total benchmark time
    pub total_benchmark_time: Duration,
    /// Average archetype activation time (in nanoseconds)
    pub avg_archetype_activation_ns: u64,
    /// Average holographic reference access time (in nanoseconds)
    pub avg_reference_access_ns: u64,
    /// Average field aggregation time (in nanoseconds)
    pub avg_field_aggregation_ns: u64,
    /// Average force calculation time (in nanoseconds)
    pub avg_force_calculation_ns: u64,
    /// Total memory usage (in bytes)
    pub total_memory_usage_bytes: u64,
    /// Performance overhead percentage
    pub performance_overhead_percent: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Overall performance score (0.0 to 1.0)
    pub performance_score: f64,
}

impl PerformanceMetrics {
    /// Calculate overall performance score
    pub fn calculate_performance_score(&mut self) {
        // Score based on multiple factors
        let time_score = if self.avg_archetype_activation_ns > 0 {
            (1000.0 / self.avg_archetype_activation_ns as f64).min(1.0)
        } else {
            0.0
        };

        let memory_score = if self.total_memory_usage_bytes > 0 {
            (10_000_000.0 / self.total_memory_usage_bytes as f64).min(1.0)
        } else {
            0.0
        };

        let overhead_score = 1.0 - (self.performance_overhead_percent / 100.0).min(1.0);
        let cache_score = self.cache_hit_rate;

        // Weighted average
        self.performance_score =
            (time_score * 0.3 + memory_score * 0.2 + overhead_score * 0.3 + cache_score * 0.2)
                .max(0.0)
                .min(1.0);
    }
}

// ============================================================================
// PERFORMANCE REPORT
// ============================================================================

/// Comprehensive performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Benchmark results
    pub benchmark_results: Vec<BenchmarkResult>,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
    /// Timestamp of the report
    pub timestamp: Instant,
    /// Success status
    pub success: bool,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl PerformanceReport {
    /// Create a new performance report
    pub fn new(benchmark_results: Vec<BenchmarkResult>, metrics: PerformanceMetrics) -> Self {
        let mut report = Self {
            benchmark_results,
            metrics,
            timestamp: Instant::now(),
            success: true,
            recommendations: Vec::new(),
        };

        report.generate_recommendations();
        report
    }

    /// Generate optimization recommendations
    fn generate_recommendations(&mut self) {
        if self.metrics.performance_overhead_percent > 10.0 {
            self.recommendations.push(format!(
                "Performance overhead is {:.1}% - consider additional optimizations",
                self.metrics.performance_overhead_percent
            ));
        }

        if self.metrics.avg_archetype_activation_ns > 1000 {
            self.recommendations.push(format!(
                "Archetype activation is slow ({:.0} ns) - consider caching or SIMD optimization",
                self.metrics.avg_archetype_activation_ns
            ));
        }

        if self.metrics.avg_reference_access_ns > 500 {
            self.recommendations.push(format!(
                "Holographic reference access is slow ({:.0} ns) - consider reference pooling",
                self.metrics.avg_reference_access_ns
            ));
        }

        if self.metrics.cache_hit_rate < 0.5 {
            self.recommendations.push(format!(
                "Cache hit rate is low ({:.1}%) - consider increasing cache size",
                self.metrics.cache_hit_rate * 100.0
            ));
        }

        if self.metrics.performance_score < 0.5 {
            self.recommendations.push(format!(
                "Overall performance score is low ({:.1}) - comprehensive optimization recommended",
                self.metrics.performance_score
            ));
        }
    }

    /// Get summary of the report
    pub fn summary(&self) -> String {
        format!(
            "Performance Report ({} benchmarks)\n\
             - Average archetype activation: {:.0} ns\n\
             - Average reference access: {:.0} ns\n\
             - Average field aggregation: {:.0} ns\n\
             - Average force calculation: {:.0} ns\n\
             - Total memory usage: {:.2} MB\n\
             - Performance overhead: {:.1}%\n\
             - Cache hit rate: {:.1}%\n\
             - Performance score: {:.2}",
            self.benchmark_results.len(),
            self.metrics.avg_archetype_activation_ns,
            self.metrics.avg_reference_access_ns,
            self.metrics.avg_field_aggregation_ns,
            self.metrics.avg_force_calculation_ns,
            self.metrics.total_memory_usage_bytes as f64 / (1024.0 * 1024.0),
            self.metrics.performance_overhead_percent,
            self.metrics.cache_hit_rate * 100.0,
            self.metrics.performance_score
        )
    }
}

// ============================================================================
// PERFORMANCE BENCHMARK
// ============================================================================

/// Performance benchmark suite
pub struct PerformanceBenchmark {
    /// Holographic seed for benchmarks
    seed: Arc<HolographicSeed>,
    /// Benchmark results
    results: Vec<BenchmarkResult>,
}

impl PerformanceBenchmark {
    /// Create a new performance benchmark suite
    pub fn new(seed: Arc<HolographicSeed>) -> Self {
        Self {
            seed,
            results: Vec::new(),
        }
    }

    /// Run all benchmarks
    pub fn run_all(&mut self) -> PerformanceReport {
        let start_time = Instant::now();

        // Run individual benchmarks
        self.benchmark_archetype_activation();
        self.benchmark_holographic_reference_access();
        self.benchmark_field_aggregation();
        self.benchmark_force_calculation();
        self.benchmark_memory_usage();

        let total_time = start_time.elapsed();

        // Calculate metrics
        let mut metrics = PerformanceMetrics::default();
        metrics.total_benchmark_time = total_time;

        if let Some(result) = self
            .results
            .iter()
            .find(|r| r.name == "archetype_activation")
        {
            metrics.avg_archetype_activation_ns = result.avg_duration_ns();
        }

        if let Some(result) = self.results.iter().find(|r| r.name == "reference_access") {
            metrics.avg_reference_access_ns = result.avg_duration_ns();
        }

        if let Some(result) = self.results.iter().find(|r| r.name == "field_aggregation") {
            metrics.avg_field_aggregation_ns = result.avg_duration_ns();
        }

        if let Some(result) = self.results.iter().find(|r| r.name == "force_calculation") {
            metrics.avg_force_calculation_ns = result.avg_duration_ns();
        }

        if let Some(result) = self.results.iter().find(|r| r.name == "memory_usage") {
            metrics.total_memory_usage_bytes = result.memory_usage_bytes;
        }

        metrics.calculate_performance_score();

        PerformanceReport::new(self.results.clone(), metrics)
    }

    /// Benchmark archetype activation
    fn benchmark_archetype_activation(&mut self) {
        let iterations = 1000;
        let mut durations = Vec::with_capacity(iterations as usize);

        for _ in 0..iterations {
            let start = Instant::now();

            // Simulate archetype activation calculation
            let activation = ArchetypeActivation {
                archetype_id: 1,
                activation_level: 0.5,
                coherence: 0.8,
                polarization: 0.7,
                lambda_value: 0.6,
            };

            let _ = activation; // Use the result

            durations.push(start.elapsed());
        }

        let result = BenchmarkResult::from_durations(
            "archetype_activation".to_string(),
            durations,
            std::mem::size_of::<ArchetypeActivation>() as u64,
        );

        self.results.push(result);
    }

    /// Benchmark holographic reference access
    fn benchmark_holographic_reference_access(&mut self) {
        let iterations = 1000;
        let mut durations = Vec::with_capacity(iterations as usize);

        for _ in 0..iterations {
            let start = Instant::now();

            // Simulate holographic reference access
            let _ = self.seed.free_will.free_will_intensity;

            durations.push(start.elapsed());
        }

        let result = BenchmarkResult::from_durations(
            "reference_access".to_string(),
            durations,
            std::mem::size_of::<Arc<HolographicSeed>>() as u64,
        );

        self.results.push(result);
    }

    /// Benchmark field aggregation
    fn benchmark_field_aggregation(&mut self) {
        let iterations = 100;
        let mut durations = Vec::with_capacity(iterations as usize);

        // Create test photons
        let seed = Arc::new(HolographicSeed::new_from_source());
        let holographic_ref = crate::holographic_seed::HolographicSeedReference::new(seed);

        let photons: Vec<Arc<Photon>> = (0..100)
            .map(|_i| {
                Arc::new(Photon::new_with_holographic_reference(
                    holographic_ref.clone(),
                    1.0, // energy
                ))
            })
            .collect();

        for _ in 0..iterations {
            let start = Instant::now();

            // Simulate field aggregation
            let mut total_field = Vector3::new(0.0, 0.0, 0.0);
            for photon in &photons {
                let photon_pos =
                    Vector3::new(photon.position.x, photon.position.y, photon.position.z);
                total_field = total_field.add(&photon_pos);
            }

            let _ = total_field; // Use the result

            durations.push(start.elapsed());
        }

        let result = BenchmarkResult::from_durations(
            "field_aggregation".to_string(),
            durations,
            photons.len() as u64 * std::mem::size_of::<Photon>() as u64,
        );

        self.results.push(result);
    }

    /// Benchmark force calculation
    fn benchmark_force_calculation(&mut self) {
        let iterations = 100;
        let mut durations = Vec::with_capacity(iterations as usize);

        // Create test seed and holographic reference
        let seed = Arc::new(HolographicSeed::new_from_source());
        let holographic_ref = crate::holographic_seed::HolographicSeedReference::new(seed.clone());

        // Create test photon for light origin
        let _test_photon = Photon::new_with_holographic_reference(
            holographic_ref.clone(),
            1.0, // energy
        );

        // Create test particles (simplified - using Particle::emerge_from_light would be more accurate)
        let particle1 = {
            let position = Coordinate3D::new(0.0, 0.0, 0.0);
            Particle::emerge_from_light(1, [0.0; 22], position)
        };

        let particle2 = {
            let position = Coordinate3D::new(1.0, 0.0, 0.0);
            Particle::emerge_from_light(2, [0.0; 22], position)
        };

        for _ in 0..iterations {
            let start = Instant::now();

            // Simulate gravitational force calculation
            let diff = particle2.position.sub(&particle1.position);
            let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
            if distance_sq > 0.0 {
                let _distance = distance_sq.sqrt();
                let g_constant = 6.674e-11;
                let _force_magnitude = g_constant
                    * derive_mass_from_energy(particle1.energy)
                    * derive_mass_from_energy(particle2.energy)
                    / distance_sq;
            }

            durations.push(start.elapsed());
        }

        let result = BenchmarkResult::from_durations(
            "force_calculation".to_string(),
            durations,
            (std::mem::size_of::<Particle>() * 2) as u64,
        );

        self.results.push(result);
    }

    /// Benchmark memory usage
    fn benchmark_memory_usage(&mut self) {
        let iterations = 100;
        let mut durations = Vec::with_capacity(iterations as usize);

        let mut total_memory = 0u64;

        for _ in 0..iterations {
            let start = Instant::now();

            // Simulate memory allocation
            let _data: Vec<f64> = vec![0.0; 1000];
            total_memory += std::mem::size_of_val(&_data) as u64;

            durations.push(start.elapsed());
        }

        let result = BenchmarkResult::from_durations(
            "memory_usage".to_string(),
            durations,
            total_memory / iterations,
        );

        self.results.push(result);
    }

    /// Get benchmark results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

// ============================================================================
// BENCHMARK TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result_basic() {
        let durations = vec![
            Duration::from_nanos(100),
            Duration::from_nanos(200),
            Duration::from_nanos(300),
        ];

        let result = BenchmarkResult::from_durations("test_benchmark".to_string(), durations, 1024);

        assert_eq!(result.name, "test_benchmark");
        assert_eq!(result.iterations, 3);
        assert_eq!(result.avg_duration_ns(), 200);
    }

    #[test]
    fn test_benchmark_result_ops_per_second() {
        let durations = vec![Duration::from_nanos(100)];

        let result = BenchmarkResult::from_durations("test_benchmark".to_string(), durations, 1024);

        assert!((result.ops_per_second - 10_000_000.0).abs() < 1000.0);
    }

    #[test]
    fn test_benchmark_comparison() {
        let baseline_durations = vec![Duration::from_nanos(1000)];
        let optimized_durations = vec![Duration::from_nanos(500)];

        let baseline =
            BenchmarkResult::from_durations("baseline".to_string(), baseline_durations, 1024);

        let optimized =
            BenchmarkResult::from_durations("optimized".to_string(), optimized_durations, 1024);

        let comparison = optimized.compare(&baseline);
        assert!((comparison.speedup - 2.0).abs() < 0.1);
        assert!((comparison.improvement_percent - 50.0).abs() < 1.0);
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.total_benchmark_time.as_nanos(), 0);
        assert_eq!(metrics.performance_score, 0.0);
    }

    #[test]
    fn test_performance_metrics_calculate_score() {
        let mut metrics = PerformanceMetrics::default();
        metrics.avg_archetype_activation_ns = 500;
        metrics.total_memory_usage_bytes = 1_000_000;
        metrics.performance_overhead_percent = 5.0;
        metrics.cache_hit_rate = 0.8;

        metrics.calculate_performance_score();
        assert!(metrics.performance_score > 0.0);
        assert!(metrics.performance_score <= 1.0);
    }

    #[test]
    fn test_performance_report_basic() {
        let benchmark_results = vec![BenchmarkResult::from_durations(
            "test".to_string(),
            vec![Duration::from_nanos(100)],
            1024,
        )];

        let metrics = PerformanceMetrics::default();
        let report = PerformanceReport::new(benchmark_results, metrics);

        assert!(report.success);
        assert_eq!(report.benchmark_results.len(), 1);
    }

    #[test]
    fn test_performance_report_summary() {
        let benchmark_results = vec![
            BenchmarkResult::from_durations(
                "archetype_activation".to_string(),
                vec![Duration::from_nanos(500)],
                1024,
            ),
            BenchmarkResult::from_durations(
                "reference_access".to_string(),
                vec![Duration::from_nanos(300)],
                1024,
            ),
        ];

        let mut metrics = PerformanceMetrics::default();
        metrics.avg_archetype_activation_ns = 500;
        metrics.avg_reference_access_ns = 300;
        metrics.total_memory_usage_bytes = 1_000_000;
        metrics.performance_overhead_percent = 5.0;
        metrics.cache_hit_rate = 0.8;
        metrics.calculate_performance_score();

        let report = PerformanceReport::new(benchmark_results, metrics);
        let summary = report.summary();

        assert!(summary.contains("Performance Report"));
        assert!(summary.contains("archetype activation"));
    }

    #[test]
    fn test_performance_benchmark_run_all() {
        let seed = Arc::new(HolographicSeed::new_from_source());
        let mut benchmark = PerformanceBenchmark::new(seed);

        let report = benchmark.run_all();
        assert!(report.success);
        assert!(!report.benchmark_results.is_empty());
    }
}
