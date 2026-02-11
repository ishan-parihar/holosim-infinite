// Entity Benchmark
//
// This benchmark measures the performance of Entity creation and operations,
// including Entity emergence from HolographicSeed.
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 6.2
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.3 (Critical Gap #3)
//
// PHASE 4.2 TASK 3.3: Benchmark for Entity Creation
// Duration: 0.5 day
// Status: IN PROGRESS

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use holonic_realms::entity::Entity;
use holonic_realms::entity_state::{Catalyst, CatalystType};
use holonic_realms::holographic_seed::HolographicSeed;
use holonic_realms::involution_process::InvolutionProcess;
use holonic_realms::soul_stream::SoulStream;
use holonic_realms::types::Density;

/// Benchmark Entity creation (emergence)
///
/// This measures the time it takes for an Entity to emerge
/// from a HolographicSeed.
fn bench_entity_emergence(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_emergence");

    group.bench_function("single_entity_emergence", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let seed = involution.create_potential(black_box(Density::D3));
            let soul_stream = SoulStream::new();
            black_box(Entity::emerge_from(
                black_box(1),
                black_box(seed),
                black_box(soul_stream),
            ));
        });
    });

    group.finish();
}

/// Benchmark Entity emergence with different densities
///
/// This measures how Entity emergence performs for entities
/// of different densities (D1 through D7).
fn bench_entity_emergence_by_density(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_emergence_by_density");

    for density in [
        Density::D1,
        Density::D2,
        Density::D3,
        Density::D4,
        Density::D5,
        Density::D6,
        Density::D7,
    ] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", density)),
            &density,
            |b, &density| {
                b.iter(|| {
                    let mut involution = InvolutionProcess::new();
                    let seed = involution.create_potential(black_box(density));
                    let soul_stream = SoulStream::new();
                    black_box(Entity::emerge_from(
                        black_box(1),
                        black_box(seed),
                        black_box(soul_stream),
                    ));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark multiple Entity creations
///
/// This measures the performance of creating multiple entities
/// in sequence, which is useful for understanding scalability.
fn bench_multiple_entity_creations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_entity_creations");

    for num_entities in [1, 10, 50, 100, 500] {
        group.throughput(Throughput::Elements(num_entities as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(num_entities),
            &num_entities,
            |b, &num_entities| {
                b.iter(|| {
                    let mut involution = InvolutionProcess::new();
                    let mut entities = Vec::with_capacity(num_entities);
                    for i in 0..num_entities {
                        let seed = involution.create_potential(black_box(Density::D3));
                        let soul_stream = SoulStream::new();
                        entities.push(black_box(Entity::emerge_from(
                            black_box(i),
                            black_box(seed),
                            black_box(soul_stream),
                        )));
                    }
                    black_box(entities);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Entity update with catalyst
///
/// This measures the performance of updating an Entity
/// with a catalyst.
fn bench_entity_update_with_catalyst(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_update_with_catalyst");

    // Create an entity first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);
    let soul_stream = SoulStream::new();
    let mut entity = Entity::emerge_from(1, seed, soul_stream);

    for catalyst_intensity in [0.1, 0.3, 0.5, 0.7, 0.9] {
        group.bench_with_input(
            BenchmarkId::from_parameter(catalyst_intensity),
            &catalyst_intensity,
            |b, &catalyst_intensity| {
                b.iter(|| {
                    let catalyst =
                        Catalyst::new(black_box(catalyst_intensity), CatalystType::Experience);
                    black_box(entity.update_with_catalyst(black_box(catalyst)));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Entity memory allocation
///
/// This measures the memory overhead of creating Entities.
fn bench_entity_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_memory");

    group.bench_function("create_and_drop_entity", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let seed = involution.create_potential(black_box(Density::D3));
            let soul_stream = SoulStream::new();
            let entity = Entity::emerge_from(black_box(1), black_box(seed), black_box(soul_stream));
            black_box(entity);
            // Entity is dropped here
        });
    });

    group.finish();
}

/// Benchmark Entity state retrieval
///
/// This measures the performance of retrieving various
/// state information from an Entity.
fn bench_entity_state_retrieval(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_state_retrieval");

    // Create an entity first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);
    let soul_stream = SoulStream::new();
    let entity = Entity::emerge_from(1, seed, soul_stream);

    group.bench_function("get_density", |b| {
        b.iter(|| {
            black_box(entity.get_density());
        });
    });

    group.bench_function("get_archetype_state", |b| {
        b.iter(|| {
            black_box(entity.get_archetype_state(black_box(0)));
        });
    });

    group.bench_function("get_vibrational_state", |b| {
        b.iter(|| {
            black_box(entity.get_vibrational_state());
        });
    });

    group.bench_function("get_polarization", |b| {
        b.iter(|| {
            black_box(entity.get_polarization());
        });
    });

    group.finish();
}

/// Benchmark Entity energy center operations
///
/// This measures the performance of operations on
/// the Entity's energy centers.
fn bench_entity_energy_center_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_energy_center_operations");

    // Create an entity first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);
    let soul_stream = SoulStream::new();
    let mut entity = Entity::emerge_from(1, seed, soul_stream);

    group.bench_function("get_energy_center_state", |b| {
        b.iter(|| {
            black_box(entity.get_energy_center_state(black_box(0)));
        });
    });

    group.bench_function("activate_energy_center", |b| {
        b.iter(|| {
            black_box(entity.activate_energy_center(black_box(0), black_box(0.5)));
        });
    });

    group.bench_function("get_all_energy_centers", |b| {
        b.iter(|| {
            black_box(entity.get_all_energy_centers());
        });
    });

    group.finish();
}

/// Benchmark Entity archetype operations
///
/// This measures the performance of operations on
/// the Entity's archetypes.
fn bench_entity_archetype_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_archetype_operations");

    // Create an entity first
    let mut involution = InvolutionProcess::new();
    let seed = involution.create_potential(Density::D3);
    let soul_stream = SoulStream::new();
    let mut entity = Entity::emerge_from(1, seed, soul_stream);

    group.bench_function("get_archetype_activation", |b| {
        b.iter(|| {
            black_box(entity.get_archetype_activation(black_box(0)));
        });
    });

    group.bench_function("set_archetype_activation", |b| {
        b.iter(|| {
            black_box(entity.set_archetype_activation(black_box(0), black_box(0.5)));
        });
    });

    group.bench_function("get_all_archetype_activations", |b| {
        b.iter(|| {
            black_box(entity.get_all_archetype_activations());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_entity_emergence,
    bench_entity_emergence_by_density,
    bench_multiple_entity_creations,
    bench_entity_update_with_catalyst,
    bench_entity_memory,
    bench_entity_state_retrieval,
    bench_entity_energy_center_operations,
    bench_entity_archetype_operations
);
criterion_main!(benches);
