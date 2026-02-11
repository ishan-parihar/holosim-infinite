// Evolution Process Benchmark
//
// This benchmark measures the performance of the Evolution process,
// which activates the potential that was created during Involution.
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 6.1
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.4 (Critical Gap #4)
//
// PHASE 4.2 TASK 3.2: Benchmark for Evolution Process
// Duration: 0.5 day
// Status: IN PROGRESS

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use holonic_realms::entity_state::{Catalyst, CatalystType};
use holonic_realms::evolution_process::EvolutionProcess;
use holonic_realms::holographic_seed::HolographicSeed;
use holonic_realms::involution_process::InvolutionProcess;
use holonic_realms::types::Density;

/// Benchmark the complete Evolution process
///
/// This measures the time it takes to activate potential
/// in a HolographicSeed through the Evolution process.
fn bench_evolution_process_complete(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolution_process_complete");

    // Create a seed first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);

    group.bench_function("single_evolution", |b| {
        b.iter(|| {
            let mut evolution = EvolutionProcess::new();
            let catalyst = Catalyst::new(black_box(0.5), CatalystType::Experience);
            black_box(evolution.activate_potential(black_box(&seed), black_box(catalyst)));
        });
    });

    group.finish();
}

/// Benchmark Evolution process with different catalyst intensities
///
/// This measures how the Evolution process performs with different
/// catalyst intensities (0.0 to 1.0).
fn bench_evolution_by_catalyst_intensity(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolution_by_catalyst_intensity");

    // Create a seed first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);

    for intensity in [0.1, 0.3, 0.5, 0.7, 0.9, 1.0] {
        group.bench_with_input(
            BenchmarkId::from_parameter(intensity),
            &intensity,
            |b, &intensity| {
                b.iter(|| {
                    let mut evolution = EvolutionProcess::new();
                    let catalyst = Catalyst::new(black_box(intensity), CatalystType::Experience);
                    black_box(evolution.activate_potential(black_box(&seed), black_box(catalyst)));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Evolution process with different catalyst types
///
/// This measures how the Evolution process performs with different
// catalyst types (Experience, Choice, Distortion).
fn bench_evolution_by_catalyst_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolution_by_catalyst_type");

    // Create a seed first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);

    for catalyst_type in [
        CatalystType::Experience,
        CatalystType::Choice,
        CatalystType::Distortion,
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", catalyst_type)),
            &catalyst_type,
            |b, &catalyst_type| {
                b.iter(|| {
                    let mut evolution = EvolutionProcess::new();
                    let catalyst = Catalyst::new(black_box(0.5), black_box(catalyst_type));
                    black_box(evolution.activate_potential(black_box(&seed), black_box(catalyst)));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark multiple Evolution processes
///
/// This measures the performance of activating potential in multiple
/// seeds in sequence, which is useful for understanding scalability.
fn bench_multiple_evolution_processes(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_evolution_processes");

    // Create seeds first
    let mut involution = InvolutionProcess::new();
    let seeds: Vec<HolographicSeed> = (0..500)
        .map(|_| involution.create_potential(Density::D3))
        .collect();

    for num_activations in [1, 10, 50, 100, 500] {
        group.throughput(Throughput::Elements(num_activations as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(num_activations),
            &num_activations,
            |b, &num_activations| {
                b.iter(|| {
                    let mut evolution = EvolutionProcess::new();
                    let catalyst = Catalyst::new(black_box(0.5), CatalystType::Experience);
                    for i in 0..num_activations {
                        black_box(evolution.activate_potential(
                            black_box(&seeds[i % seeds.len()]),
                            black_box(catalyst.clone()),
                        ));
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Evolution process with multiple catalysts
///
/// This measures the performance of processing multiple catalysts
// in a single Evolution process, simulating a complete evolution cycle.
fn bench_evolution_multiple_catalysts(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolution_multiple_catalysts");

    // Create a seed first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);

    for num_catalysts in [1, 10, 50, 100] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_catalysts),
            &num_catalysts,
            |b, &num_catalysts| {
                b.iter(|| {
                    let mut evolution = EvolutionProcess::new();
                    for _ in 0..num_catalysts {
                        let catalyst = Catalyst::new(black_box(0.5), CatalystType::Experience);
                        black_box(
                            evolution.activate_potential(black_box(&seed), black_box(catalyst)),
                        );
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Evolution process memory allocation
///
/// This measures the memory overhead of activating potential.
fn bench_evolution_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("evolution_memory");

    // Create a seed first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);

    group.bench_function("activate_and_drop_result", |b| {
        b.iter(|| {
            let mut evolution = EvolutionProcess::new();
            let catalyst = Catalyst::new(black_box(0.5), CatalystType::Experience);
            let result = evolution.activate_potential(black_box(&seed), black_box(catalyst));
            black_box(result);
            // Result is dropped here
        });
    });

    group.finish();
}

/// Benchmark Involution + Evolution complete cycle
///
/// This measures the performance of the complete Involution → Evolution cycle.
fn bench_involution_evolution_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("involution_evolution_cycle");

    group.bench_function("complete_cycle", |b| {
        b.iter(|| {
            // Involution: Create potential
            let mut involution = InvolutionProcess::new();
            let seed = involution.create_potential(black_box(Density::D3));

            // Evolution: Activate potential
            let mut evolution = EvolutionProcess::new();
            let catalyst = Catalyst::new(black_box(0.5), CatalystType::Experience);
            let result = evolution.activate_potential(black_box(&seed), black_box(catalyst));

            black_box(result);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_evolution_process_complete,
    bench_evolution_by_catalyst_intensity,
    bench_evolution_by_catalyst_type,
    bench_multiple_evolution_processes,
    bench_evolution_multiple_catalysts,
    bench_evolution_memory,
    bench_involution_evolution_cycle
);
criterion_main!(benches);
