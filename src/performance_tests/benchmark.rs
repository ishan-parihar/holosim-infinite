//! Benchmark Execution Engine
//!
//! From MASTER_R&D_ROADMAP.md Phase 8: "Implement statistical benchmarking (multiple iterations, warm-up)"
//!
//! This module provides the core benchmark execution engine that:
//! - Runs benchmarks with warm-up iterations
//! - Calculates statistical metrics (mean, median, percentiles)
//! - Compares results against performance assertions
//! - Generates performance reports

use super::{
    BenchmarkCategory, BenchmarkResult, PerformanceAssertion, PerformanceTest,
};
use crate::types::Float;
use std::time::{Duration, Instant};

/// Benchmark runner for executing performance tests
///
/// From MASTER_R&D_ROADMAP.md: "Use statistical analysis (mean, median, percentiles)
/// for reliable performance measurement"
pub struct BenchmarkRunner {
    /// Whether to run warm-up iterations
    pub enable_warmup: bool,

    /// Whether to print progress
    pub verbose: bool,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new() -> Self {
        BenchmarkRunner {
            enable_warmup: true,
            verbose: true,
        }
    }

    /// Create a benchmark runner with custom settings
    pub fn with_settings(enable_warmup: bool, verbose: bool) -> Self {
        BenchmarkRunner {
            enable_warmup,
            verbose,
        }
    }

    /// Run a single benchmark and return the result
    ///
    /// This method:
    /// 1. Runs warm-up iterations (if enabled)
    /// 2. Runs the actual benchmark iterations
    /// 3. Calculates statistical metrics
    /// 4. Returns a BenchmarkResult
    pub fn run_benchmark(&self, test: &dyn PerformanceTest) -> BenchmarkResult {
        let name = test.name().to_string();
        let category = test.category();
        let iterations = test.iterations();
        let warmup_iterations = if self.enable_warmup {
            iterations / 10
        } else {
            0
        };

        if self.verbose {
            println!("Running benchmark: {} ({})", name, category.as_str());
            println!("Warm-up iterations: {}", warmup_iterations);
            println!("Benchmark iterations: {}", iterations);
        }

        let start_time = Instant::now();

        // Run warm-up iterations
        if self.enable_warmup && warmup_iterations > 0 {
            if self.verbose {
                print!("Warm-up: ");
                for i in 0..warmup_iterations {
                    test.run_iteration();
                    if i % 10 == 9 || i == warmup_iterations - 1 {
                        print!(". ");
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                    }
                }
                println!();
            } else {
                for _ in 0..warmup_iterations {
                    test.run_iteration();
                }
            }
        }

        // Run actual benchmark iterations
        let mut durations = Vec::with_capacity(iterations);

        if self.verbose {
            print!("Benchmark: ");
            for i in 0..iterations {
                let duration = test.run_iteration();
                durations.push(duration);

                if i % 10 == 9 || i == iterations - 1 {
                    print!(". ");
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
            }
            println!();
        } else {
            for _ in 0..iterations {
                durations.push(test.run_iteration());
            }
        }

        let total_duration = start_time.elapsed();

        BenchmarkResult::from_durations(
            name,
            category,
            durations,
            warmup_iterations,
            total_duration,
        )
    }

    /// Run a benchmark with performance assertions
    ///
    /// Returns the benchmark result and whether all assertions passed
    pub fn run_benchmark_with_assertions(
        &self,
        test: &dyn PerformanceTest,
        assertions: &[PerformanceAssertion],
    ) -> (BenchmarkResult, bool) {
        let result = self.run_benchmark(test);

        let all_passed = assertions
            .iter()
            .all(|assertion| result.meets_assertion(assertion));

        if self.verbose {
            println!("\n{}", result.report());

            if !assertions.is_empty() {
                println!("\nAssertions:");
                for assertion in assertions {
                    let passed = result.meets_assertion(assertion);
                    println!(
                        "  [{}] {}",
                        if passed { "✓" } else { "✗" },
                        assertion.description()
                    );
                }

                if all_passed {
                    println!("\n✓ All assertions passed");
                } else {
                    println!("\n✗ Some assertions failed");
                }
            }
        }

        (result, all_passed)
    }

    /// Run multiple benchmarks and generate a summary report
    pub fn run_suite(&self, tests: Vec<&dyn PerformanceTest>) -> BenchmarkSuiteResult {
        if self.verbose {
            println!("Running benchmark suite with {} tests\n", tests.len());
        }

        let mut results = Vec::new();
        let mut total_duration = Duration::ZERO;

        for test in tests {
            let result = self.run_benchmark(test);
            total_duration += result.total_duration;
            results.push(result);

            if self.verbose {
                println!("\n{}\n", "=".repeat(60));
            }
        }

        BenchmarkSuiteResult {
            results,
            total_duration,
        }
    }

    /// Run multiple benchmarks with assertions
    pub fn run_suite_with_assertions(
        &self,
        tests: Vec<(&dyn PerformanceTest, Vec<PerformanceAssertion>)>,
    ) -> BenchmarkSuiteResult {
        if self.verbose {
            println!("Running benchmark suite with {} tests\n", tests.len());
        }

        let mut results = Vec::new();
        let mut total_duration = Duration::ZERO;

        for (test, assertions) in tests {
            let (result, passed) = self.run_benchmark_with_assertions(test, &assertions);
            total_duration += result.total_duration;
            results.push((result, passed));

            if self.verbose {
                println!("\n{}\n", "=".repeat(60));
            }
        }

        // Convert to proper results format
        let benchmark_results: Vec<BenchmarkResult> = results.into_iter().map(|(r, _)| r).collect();

        BenchmarkSuiteResult {
            results: benchmark_results,
            total_duration,
        }
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of running a benchmark suite
pub struct BenchmarkSuiteResult {
    /// Results for each benchmark
    pub results: Vec<BenchmarkResult>,

    /// Total duration for the entire suite
    pub total_duration: Duration,
}

impl BenchmarkSuiteResult {
    /// Generate a summary report for the suite
    pub fn summary_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Benchmark Suite Summary\n");
        report.push_str(&"=".repeat(60));
        report.push('\n');

        report.push_str(&format!("Total benchmarks: {}\n", self.results.len()));
        report.push_str(&format!(
            "Total duration: {:.3} s\n\n",
            self.total_duration.as_secs_f64()
        ));

        // Group by category
        let mut by_category: std::collections::HashMap<BenchmarkCategory, Vec<&BenchmarkResult>> =
            std::collections::HashMap::new();

        for result in &self.results {
            by_category
                .entry(result.category)
                .or_insert_with(Vec::new)
                .push(result);
        }

        // Report each category
        for (category, results) in by_category.iter() {
            report.push_str(&format!("\nCategory: {}\n", category.as_str()));
            report.push_str(&"-".repeat(40));
            report.push('\n');

            for result in results {
                report.push_str(&format!(
                    "  {:30} Mean: {:7.3} ms  Median: {:7.3} ms  P95: {:7.3} ms\n",
                    result.name,
                    result.mean.as_secs_f64() * 1000.0,
                    result.median.as_secs_f64() * 1000.0,
                    result.p95.as_secs_f64() * 1000.0,
                ));
            }
        }

        // Overall statistics
        report.push_str("\n\nOverall Statistics\n");
        report.push_str(&"-".repeat(40));
        report.push('\n');

        if !self.results.is_empty() {
            let total_iterations: usize = self.results.iter().map(|r| r.iterations).sum();
            let avg_mean: Float = self
                .results
                .iter()
                .map(|r| r.mean.as_secs_f64() * 1000.0)
                .sum::<Float>()
                / self.results.len() as Float;

            report.push_str(&format!("Total iterations: {}\n", total_iterations));
            report.push_str(&format!("Average mean time: {:.3} ms\n", avg_mean));

            // Find fastest and slowest
            let fastest = self
                .results
                .iter()
                .min_by(|a, b| a.mean.cmp(&b.mean))
                .unwrap();
            let slowest = self
                .results
                .iter()
                .max_by(|a, b| a.mean.cmp(&b.mean))
                .unwrap();

            report.push_str(&format!(
                "Fastest benchmark: {} ({:.3} ms)\n",
                fastest.name,
                fastest.mean.as_secs_f64() * 1000.0
            ));
            report.push_str(&format!(
                "Slowest benchmark: {} ({:.3} ms)\n",
                slowest.name,
                slowest.mean.as_secs_f64() * 1000.0
            ));
        }

        report
    }

    /// Find benchmarks that failed their assertions
    pub fn find_failed(
        &self,
        assertions_map: &std::collections::HashMap<String, Vec<PerformanceAssertion>>,
    ) -> Vec<String> {
        self.results
            .iter()
            .filter(|result| {
                if let Some(assertions) = assertions_map.get(&result.name) {
                    !assertions.iter().all(|a| result.meets_assertion(a))
                } else {
                    false
                }
            })
            .map(|result| result.name.clone())
            .collect()
    }
}

/// Performance assertion helper struct
///
/// From MASTER_R&D_ROADMAP.md: "Support performance assertions (assert performance < threshold)"
#[derive(Debug, Clone)]
pub struct PerformanceAssertionBuilder {
    assertions: Vec<PerformanceAssertion>,
}

impl PerformanceAssertionBuilder {
    /// Create a new assertion builder
    pub fn new() -> Self {
        PerformanceAssertionBuilder {
            assertions: Vec::new(),
        }
    }

    /// Add a max mean assertion
    pub fn max_mean_ms(mut self, ms: u64) -> Self {
        self.assertions.push(PerformanceAssertion::max_mean_ms(ms));
        self
    }

    /// Add a max median assertion
    pub fn max_median_ms(mut self, ms: u64) -> Self {
        self.assertions
            .push(PerformanceAssertion::max_median_ms(ms));
        self
    }

    /// Add a max P95 assertion
    pub fn max_p95_ms(mut self, ms: u64) -> Self {
        self.assertions.push(PerformanceAssertion::max_p95_ms(ms));
        self
    }

    /// Add a max P99 assertion
    pub fn max_p99_ms(mut self, ms: u64) -> Self {
        self.assertions.push(PerformanceAssertion::max_p99_ms(ms));
        self
    }

    /// Add a minimum throughput assertion
    pub fn min_throughput(mut self, ops_per_sec: Float) -> Self {
        self.assertions
            .push(PerformanceAssertion::min_throughput_ops_per_sec(
                ops_per_sec,
            ));
        self
    }

    /// Build the assertions vector
    pub fn build(self) -> Vec<PerformanceAssertion> {
        self.assertions
    }
}

impl Default for PerformanceAssertionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::performance_tests::BenchmarkCategory;

    /// Mock performance test for testing
    struct MockTest {
        name: String,
        category: BenchmarkCategory,
        duration: Duration,
    }

    impl PerformanceTest for MockTest {
        fn name(&self) -> &str {
            &self.name
        }

        fn category(&self) -> BenchmarkCategory {
            self.category
        }

        fn description(&self) -> &str {
            "Mock test"
        }

        fn run_iteration(&self) -> Duration {
            self.duration
        }

        fn iterations(&self) -> usize {
            10
        }
    }

    #[test]
    fn test_benchmark_runner() {
        let runner = BenchmarkRunner::with_settings(false, false);
        let test = MockTest {
            name: "test_benchmark".to_string(),
            category: BenchmarkCategory::Custom("test"),
            duration: Duration::from_millis(10),
        };

        let result = runner.run_benchmark(&test);

        assert_eq!(result.name, "test_benchmark");
        assert_eq!(result.iterations, 10);
        assert!(result.mean.as_millis() >= 9 && result.mean.as_millis() <= 11);
    }

    #[test]
    fn test_performance_assertion_builder() {
        let assertions = PerformanceAssertionBuilder::new()
            .max_mean_ms(50)
            .max_p95_ms(100)
            .min_throughput(1000.0)
            .build();

        assert_eq!(assertions.len(), 3);
    }

    #[test]
    fn test_benchmark_suite_summary() {
        let runner = BenchmarkRunner::with_settings(false, false);
        let test1 = MockTest {
            name: "test1".to_string(),
            category: BenchmarkCategory::Custom("test"),
            duration: Duration::from_millis(10),
        };
        let test2 = MockTest {
            name: "test2".to_string(),
            category: BenchmarkCategory::Custom("test"),
            duration: Duration::from_millis(20),
        };

        let suite = runner.run_suite(vec![&test1, &test2]);
        let summary = suite.summary_report();

        assert!(summary.contains("test1"));
        assert!(summary.contains("test2"));
        assert!(summary.contains("Total benchmarks: 2"));
    }
}
