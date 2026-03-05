//! Benchmark Registry
//!
//! From MASTER_R&D_ROADMAP.md Phase 8: "Benchmark registry - Register all performance benchmarks"
//!
//! This module provides a central registry for all performance benchmarks,
//! enabling:
//! - Registration of benchmarks by name or category
//! - Discovery of benchmarks via module registration
//! - Execution of benchmarks by name or category
//! - Listing of available benchmarks

use super::{BenchmarkCategory, PerformanceTest as PerfTest};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Benchmark registry for managing and executing performance tests
///
/// From MASTER_R&D_ROADMAP.md: "Register all performance benchmarks"
pub struct BenchmarkRegistry {
    /// Registered benchmarks by name
    benchmarks: HashMap<String, Arc<dyn PerfTest>>,

    /// Index of benchmarks by category
    by_category: HashMap<BenchmarkCategory, Vec<String>>,
}

impl BenchmarkRegistry {
    /// Create a new benchmark registry
    pub fn new() -> Self {
        BenchmarkRegistry {
            benchmarks: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    /// Register a benchmark
    ///
    /// From MASTER_R&D_ROADMAP.md: "Register all performance benchmarks"
    pub fn register<T: PerfTest + 'static>(&mut self, benchmark: T) {
        let name = benchmark.name().to_string();
        let category = benchmark.category();

        // Store the benchmark
        self.benchmarks.insert(name.clone(), Arc::new(benchmark));

        // Update category index
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Register a boxed benchmark
    pub fn register_boxed(&mut self, benchmark: Box<dyn PerfTest>) {
        let name = benchmark.name().to_string();
        let category = benchmark.category();

        // Store the benchmark
        self.benchmarks.insert(name.clone(), benchmark.into());

        // Update category index
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(name);
    }

    /// Get a benchmark by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn PerfTest>> {
        self.benchmarks.get(name).cloned()
    }

    /// Get all benchmarks in a category
    pub fn get_by_category(&self, category: BenchmarkCategory) -> Vec<Arc<dyn PerfTest>> {
        self.by_category
            .get(&category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.benchmarks.get(name).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all registered benchmarks
    pub fn all(&self) -> Vec<Arc<dyn PerfTest>> {
        self.benchmarks.values().cloned().collect()
    }

    /// Get all benchmark names
    pub fn all_names(&self) -> Vec<String> {
        self.benchmarks.keys().cloned().collect()
    }

    /// Get all categories that have benchmarks
    pub fn categories(&self) -> Vec<BenchmarkCategory> {
        self.by_category.keys().cloned().collect()
    }

    /// Get the count of registered benchmarks
    pub fn count(&self) -> usize {
        self.benchmarks.len()
    }

    /// Check if a benchmark is registered
    pub fn contains(&self, name: &str) -> bool {
        self.benchmarks.contains_key(name)
    }

    /// Remove a benchmark by name
    pub fn remove(&mut self, name: &str) -> Option<Arc<dyn PerfTest>> {
        if let Some(benchmark) = self.benchmarks.remove(name) {
            // Remove from category index
            let category = benchmark.category();
            if let Some(benchmarks) = self.by_category.get_mut(&category) {
                benchmarks.retain(|n| n != name);
            }
            Some(benchmark)
        } else {
            None
        }
    }

    /// Clear all benchmarks
    pub fn clear(&mut self) {
        self.benchmarks.clear();
        self.by_category.clear();
    }

    /// Generate a summary report
    pub fn summary_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Benchmark Registry Summary\n");
        report.push_str(&"=".repeat(60));
        report.push('\n');

        report.push_str(&format!("Total benchmarks: {}\n\n", self.count()));

        // Group by category
        for category in &self.categories() {
            let benchmarks = self.get_by_category(*category);
            report.push_str(&format!("\nCategory: {}\n", category.as_str()));
            report.push_str(&"-".repeat(40));
            report.push('\n');

            for benchmark in &benchmarks {
                report.push_str(&format!(
                    "  - {}: {}\n",
                    benchmark.name(),
                    benchmark.description()
                ));
            }
        }

        report
    }
}

impl Default for BenchmarkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global benchmark registry instance
///
/// Provides a singleton registry that can be accessed from anywhere in the codebase.
static GLOBAL_REGISTRY: RwLock<Option<BenchmarkRegistry>> = RwLock::new(None);

/// Get or create the global benchmark registry
pub fn global_registry() -> std::sync::RwLockWriteGuard<'static, Option<BenchmarkRegistry>> {
    GLOBAL_REGISTRY.write().unwrap_or_else(|e| e.into_inner())
}

/// Initialize the global benchmark registry with default benchmarks
pub fn initialize_global_registry() {
    let mut guard = global_registry();
    if guard.is_none() {
        let mut registry = BenchmarkRegistry::new();

        // Register default benchmarks from each module
        register_mera_benchmarks(&mut registry);
        register_fractal_cache_benchmarks(&mut registry);
        register_scale_transition_benchmarks(&mut registry);
        register_density_transition_benchmarks(&mut registry);
        register_physics_engine_benchmarks(&mut registry);

        *guard = Some(registry);
    }
}

/// Register MERA compression benchmarks
#[cfg(test)]
fn register_mera_benchmarks(_registry: &mut BenchmarkRegistry) {
    // Benchmarks will be registered by the mera_compression module
}

/// Register fractal cache benchmarks
#[cfg(test)]
fn register_fractal_cache_benchmarks(_registry: &mut BenchmarkRegistry) {
    // Benchmarks will be registered by the fractal_cache module
}

/// Register scale transition benchmarks
#[cfg(test)]
fn register_scale_transition_benchmarks(_registry: &mut BenchmarkRegistry) {
    // Benchmarks will be registered by the scale_transitions module
}

/// Register density transition benchmarks
#[cfg(test)]
fn register_density_transition_benchmarks(_registry: &mut BenchmarkRegistry) {
    // Benchmarks will be registered by the density_transitions module
}

/// Register physics engine benchmarks
#[cfg(test)]
fn register_physics_engine_benchmarks(_registry: &mut BenchmarkRegistry) {
    // Benchmarks will be registered by the physics_engine module
}

/// Benchmark builder for creating benchmarks with configuration
pub struct BenchmarkBuilder<T: PerfTest + 'static> {
    benchmark: Option<T>,
}

impl<T: PerfTest + 'static> BenchmarkBuilder<T> {
    /// Create a new benchmark builder
    pub fn new(benchmark: T) -> Self {
        BenchmarkBuilder {
            benchmark: Some(benchmark),
        }
    }

    /// Build and register the benchmark
    pub fn build_and_register(self, registry: &mut BenchmarkRegistry) {
        if let Some(benchmark) = self.benchmark {
            registry.register(benchmark);
        }
    }
}

/// Helper macro for registering benchmarks
///
/// Usage:
/// ```rust
/// register_benchmark!(registry, MyBenchmark);
/// ```
#[macro_export]
macro_rules! register_benchmark {
    ($registry:expr, $benchmark:expr) => {
        $registry.register($benchmark);
    };
}

/// Helper macro for registering multiple benchmarks
///
/// Usage:
/// ```rust
/// register_benchmarks!(registry,
///     benchmark1,
///     benchmark2,
///     benchmark3
/// );
/// ```
#[macro_export]
macro_rules! register_benchmarks {
    ($registry:expr, $($benchmark:expr),* $(,)?) => {
        $(
            $registry.register($benchmark);
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::performance_tests::{BenchmarkCategory, PerformanceTest};

    /// Mock benchmark for testing
    struct MockBenchmark {
        name: String,
        category: BenchmarkCategory,
    }

    impl PerformanceTest for MockBenchmark {
        fn name(&self) -> &str {
            &self.name
        }

        fn category(&self) -> BenchmarkCategory {
            self.category
        }

        fn description(&self) -> &str {
            "Mock benchmark"
        }

        fn run_iteration(&self) -> std::time::Duration {
            std::time::Duration::from_millis(10)
        }

        fn iterations(&self) -> usize {
            10
        }
    }

    #[test]
    fn test_registry_register() {
        let mut registry = BenchmarkRegistry::new();

        let benchmark = MockBenchmark {
            name: "test_benchmark".to_string(),
            category: BenchmarkCategory::Custom("test"),
        };

        registry.register(benchmark);

        assert_eq!(registry.count(), 1);
        assert!(registry.contains("test_benchmark"));
    }

    #[test]
    fn test_registry_get_by_category() {
        let mut registry = BenchmarkRegistry::new();

        let benchmark1 = MockBenchmark {
            name: "benchmark1".to_string(),
            category: BenchmarkCategory::MeraCompression,
        };

        let benchmark2 = MockBenchmark {
            name: "benchmark2".to_string(),
            category: BenchmarkCategory::MeraCompression,
        };

        let benchmark3 = MockBenchmark {
            name: "benchmark3".to_string(),
            category: BenchmarkCategory::FractalCache,
        };

        registry.register(benchmark1);
        registry.register(benchmark2);
        registry.register(benchmark3);

        let mera_benchmarks = registry.get_by_category(BenchmarkCategory::MeraCompression);
        assert_eq!(mera_benchmarks.len(), 2);

        let cache_benchmarks = registry.get_by_category(BenchmarkCategory::FractalCache);
        assert_eq!(cache_benchmarks.len(), 1);
    }

    #[test]
    fn test_registry_summary() {
        let mut registry = BenchmarkRegistry::new();

        let benchmark = MockBenchmark {
            name: "test_benchmark".to_string(),
            category: BenchmarkCategory::Custom("test"),
        };

        registry.register(benchmark);

        let summary = registry.summary_report();
        assert!(summary.contains("Total benchmarks: 1"));
        assert!(summary.contains("test_benchmark"));
    }

    #[test]
    fn test_registry_remove() {
        let mut registry = BenchmarkRegistry::new();

        let benchmark = MockBenchmark {
            name: "test_benchmark".to_string(),
            category: BenchmarkCategory::Custom("test"),
        };

        registry.register(benchmark);
        assert_eq!(registry.count(), 1);

        registry.remove("test_benchmark");
        assert_eq!(registry.count(), 0);
        assert!(!registry.contains("test_benchmark"));
    }
}
