// Phase 17.2: Performance Benchmarking
//
// This module provides comprehensive benchmarks for the dual-dimensional simulation
// to measure performance improvements across different optimization strategies.
//
// Benchmarks cover:
// - Baseline performance (no optimizations)
// - Parallel processing performance
// - Lazy loading performance
// - Batch energy flow performance
// - Memory pool performance
// - Combined optimizations performance
//
// Each benchmark is run at multiple scales to understand performance characteristics.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use holonic_realms::dual_dimensional_integration::DualDimensionalIntegration;
use std::time::Duration;

/// Create a simulation with specified number of entities
fn create_simulation(entity_count: usize) -> DualDimensionalIntegration {
    let mut integration = DualDimensionalIntegration::new();
    for i in 0..entity_count {
        integration.register_decision_engine(
            i as u64,
            holonic_realms::decision_engine::DecisionEngine::new(i as u64, 42),
        );
    }
    integration
}

/// Benchmark single simulation step
fn bench_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulation_step");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    for &entity_count in &[10, 50, 100, 500] {
        group.throughput(Throughput::Elements(entity_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(entity_count),
            &entity_count,
            |b, &count| {
                let mut integration = create_simulation(count);
                b.iter(|| {
                    integration.process_step();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark multiple simulation steps
fn bench_multiple_steps(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_steps");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);

    for &step_count in &[10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(step_count),
            &step_count,
            |b, &steps| {
                let mut integration = create_simulation(10);
                b.iter(|| {
                    for _ in 0..steps {
                        integration.process_step();
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark complete dual-dimensional integration
fn bench_dual_dimensional(c: &mut Criterion) {
    let mut group = c.benchmark_group("dual_dimensional");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);

    for &entity_count in &[10, 50, 100] {
        let mut integration = create_simulation(entity_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(entity_count),
            &entity_count,
            |b, _| {
                b.iter(|| {
                    integration.process_step();
                    integration.synchronize();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory pool operations
fn bench_memory_pools(c: &mut Criterion) {
    use holonic_realms::memory_pool::{EnergyFlowUpdatePool, StateChangePool};

    let mut group = c.benchmark_group("memory_pools");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Benchmark StateChange pool
    group.bench_function("state_change_pool_acquire_release", |b| {
        let pool = StateChangePool::with_defaults();
        pool.prepopulate(100);
        b.iter(|| {
            let obj = pool.acquire();
            pool.release(obj);
        });
    });

    // Benchmark EnergyFlowUpdate pool
    group.bench_function("energy_flow_pool_acquire_release", |b| {
        let pool = EnergyFlowUpdatePool::with_defaults();
        pool.prepopulate(100);
        b.iter(|| {
            let obj = pool.acquire();
            pool.release(obj);
        });
    });

    group.finish();
}

/// Benchmark lazy loading initialization
fn bench_lazy_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("lazy_loading");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    for &entity_count in &[10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(entity_count),
            &entity_count,
            |b, &count| {
                b.iter(|| {
                    // Create new integration each iteration to test lazy initialization
                    let mut integration = create_simulation(count);
                    // Process step triggers lazy initialization
                    integration.process_step();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark batch operations
fn bench_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_operations");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    for &entity_count in &[10, 50, 100, 500] {
        let mut integration = create_simulation(entity_count);

        group.bench_with_input(
            BenchmarkId::from_parameter(entity_count),
            &entity_count,
            |b, _| {
                b.iter(|| {
                    integration.process_step_batch();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark scaling characteristics
fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);

    // Test how performance scales with entity count
    for entity_count in [10, 20, 50, 100, 200, 500].iter() {
        group.throughput(Throughput::Elements(*entity_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(entity_count),
            entity_count,
            |b, &count| {
                let mut integration = create_simulation(count);
                b.iter(|| {
                    integration.process_step();
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_step,
    bench_multiple_steps,
    bench_dual_dimensional,
    bench_memory_pools,
    bench_lazy_loading,
    bench_batch_operations,
    bench_scaling
);

criterion_main!(benches);
