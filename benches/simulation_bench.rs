//! HoloSim Infinite - Simulation Benchmarks
//!
//! Benchmarks for core simulation components using Criterion.
//! Run with: cargo bench
//! Dry-run with: cargo bench -- --test

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use holonic_realms::entity_layer7::layer7::{EntityId, EntityType, SubSubLogos};
use holonic_realms::simulation_v3::archetype_basis::{ArchetypeActivationProfile, ArchetypeBasis};
use holonic_realms::simulation_v3::mera_network::{
    MeraNetwork, MeraQuery, MeraScale, QueryType, Tensor,
};
use holonic_realms::simulation_v3::simulation_runner::{SimulationParameters, SimulationRunner};

// ============================================================================
// Entity Creation Benchmark
// ============================================================================

fn bench_entity_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_creation");

    for &count in &[10, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter_batched(
                || (),
                |_| {
                    let mut _entities = Vec::with_capacity(count);
                    for i in 0..count {
                        let entity = SubSubLogos::builder(
                            EntityId::new(format!("bench-{}", i)),
                            EntityType::Individual,
                        )
                        .build();
                        _entities.push(entity);
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

// ============================================================================
// MERA Compress/Decompress Benchmark
// ============================================================================

fn bench_mera_compress_decompress(c: &mut Criterion) {
    let mut group = c.benchmark_group("mera_compress_decompress");

    for &size in &[32, 64, 128] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let data: Vec<f64> = (0..size * size).map(|i| (i as f64) * 0.001).collect();
                    let tensor = Tensor {
                        shape: vec![size, size],
                        data,
                    };

                    let mut network = MeraNetwork::new();
                    network.initialize(tensor.clone()).unwrap();
                    network.build_hierarchy().unwrap();
                    (network, tensor)
                },
                |(mut network, tensor)| {
                    let _result = network.compress(tensor.clone());
                    let query = MeraQuery::new(MeraScale::Quantum, QueryType::Spatial);
                    let _decompressed = network.decompress(&query);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

// ============================================================================
// Archetype Basis Benchmark
// ============================================================================

fn bench_archetype_basis(c: &mut Criterion) {
    let mut group = c.benchmark_group("archetype_basis");

    for &dimension in &[32, 64, 128] {
        group.bench_with_input(
            BenchmarkId::from_parameter(dimension),
            &dimension,
            |b, &dimension| {
                b.iter_batched(
                    || {
                        let basis = ArchetypeBasis::new(dimension);
                        let mut coeffs = [0.0f64; 22];
                        for (i, c) in coeffs.iter_mut().enumerate() {
                            *c = (i as f64 * 0.1).sin();
                        }
                        let profile = ArchetypeActivationProfile::new(coeffs);
                        (basis, profile)
                    },
                    |(basis, profile)| {
                        let pattern = basis.reconstruct(&profile);
                        let _recovered = basis.project(&pattern);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

// ============================================================================
// Simulation Step Benchmark
// ============================================================================

fn bench_simulation_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulation_step");

    for &num_entities in &[10, 50] {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_entities),
            &num_entities,
            |b, &num_entities| {
                b.iter_batched(
                    || {
                        let params = SimulationParameters::new()
                            .with_num_entities(num_entities)
                            .with_num_steps(1);
                        SimulationRunner::new(params)
                    },
                    |mut runner| {
                        let _result = runner.run_simulation();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_entity_creation, bench_mera_compress_decompress, bench_archetype_basis, bench_simulation_step
);

criterion_main!(benches);
