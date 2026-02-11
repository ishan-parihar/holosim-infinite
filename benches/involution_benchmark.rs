// Involution Process Benchmark
//
// This benchmark measures the performance of the Involution process,
// which creates the potential for entities by folding the complete
// 22-Archetype structure into a HolographicSeed.
//
// Knowledge Base Reference:
// - COSMOLOGICAL-ARCHITECTURE.md Section 5.1
// - ARCHITECTURE_AUDIT_REPORT.md Section 2.4 (Critical Gap #4)
//
// PHASE 4.2 TASK 3.1: Benchmark for Involution Process
// Duration: 0.5 day
// Status: IN PROGRESS

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use holonic_realms::holographic_seed::HolographicSeed;
use holonic_realms::involution_process::InvolutionProcess;
use holonic_realms::soul_stream::SoulStream;
use holonic_realms::types::Density;

/// Benchmark the complete Involution process
///
/// This measures the time it takes to create a complete HolographicSeed
/// through the 7-step Involution process (Violet → Indigo → Blue → Green → Yellow → Orange → Red).
fn bench_involution_process_complete(c: &mut Criterion) {
    let mut group = c.benchmark_group("involution_process_complete");

    group.bench_function("single_involution", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            black_box(involution.create_potential(black_box(Density::D3)));
        });
    });

    group.finish();
}

/// Benchmark Involution process with different densities
///
/// This measures how the Involution process performs for entities
/// of different densities (D1 through D7).
fn bench_involution_process_by_density(c: &mut Criterion) {
    let mut group = c.benchmark_group("involution_process_by_density");

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
                    black_box(involution.create_potential(black_box(density)));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark each layer of the Involution process
///
/// This measures the performance of each of the 7 layers:
/// Layer 0: Violet-Ray Realm (Intelligent Infinity + Free Will)
/// Layer 1: Indigo-Ray Realm (Logos/Love → 21 Archetypes)
/// Layer 2: Blue-Ray Realm (Light/Law of Light)
/// Layer 3: Green-Ray Realm (Dimensions/Universes + Veil)
/// Layer 4: Yellow-Ray Realm (Solar/Planetary Logos)
/// Layer 5: Orange-Ray Realm (Soul Stream)
/// Layer 6: Red-Ray Realm (Co-Creator Spawns)
fn bench_involution_layers(c: &mut Criterion) {
    let mut group = c.benchmark_group("involution_layers");

    // Layer 0: Violet-Ray Realm
    group.bench_function("layer0_violet_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            black_box(involution.create_layer0_violet_ray());
        });
    });

    // Layer 1: Indigo-Ray Realm
    group.bench_function("layer1_indigo_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let layer0 = involution.create_layer0_violet_ray();
            black_box(involution.create_layer1_indigo_ray(black_box(&layer0)));
        });
    });

    // Layer 2: Blue-Ray Realm
    group.bench_function("layer2_blue_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let layer0 = involution.create_layer0_violet_ray();
            let layer1 = involution.create_layer1_indigo_ray(&layer0);
            black_box(involution.create_layer2_blue_ray(black_box(&layer1)));
        });
    });

    // Layer 3: Green-Ray Realm
    group.bench_function("layer3_green_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let layer0 = involution.create_layer0_violet_ray();
            let layer1 = involution.create_layer1_indigo_ray(&layer0);
            let layer2 = involution.create_layer2_blue_ray(&layer1);
            black_box(involution.create_layer3_green_ray(black_box(&layer2)));
        });
    });

    // Layer 4: Yellow-Ray Realm
    group.bench_function("layer4_yellow_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let layer0 = involution.create_layer0_violet_ray();
            let layer1 = involution.create_layer1_indigo_ray(&layer0);
            let layer2 = involution.create_layer2_blue_ray(&layer1);
            let layer3 = involution.create_layer3_green_ray(&layer2);
            black_box(involution.create_layer4_yellow_ray(black_box(&layer3)));
        });
    });

    // Layer 5: Orange-Ray Realm
    group.bench_function("layer5_orange_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let layer0 = involution.create_layer0_violet_ray();
            let layer1 = involution.create_layer1_indigo_ray(&layer0);
            let layer2 = involution.create_layer2_blue_ray(&layer1);
            let layer3 = involution.create_layer3_green_ray(&layer2);
            let layer4 = involution.create_layer4_yellow_ray(&layer3);
            black_box(involution.create_layer5_orange_ray(black_box(&layer4)));
        });
    });

    // Layer 6: Red-Ray Realm
    group.bench_function("layer6_red_ray", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let layer0 = involution.create_layer0_violet_ray();
            let layer1 = involution.create_layer1_indigo_ray(&layer0);
            let layer2 = involution.create_layer2_blue_ray(&layer1);
            let layer3 = involution.create_layer3_green_ray(&layer2);
            let layer4 = involution.create_layer4_yellow_ray(&layer3);
            let layer5 = involution.create_layer5_orange_ray(&layer4);
            black_box(involution.create_layer6_red_ray(black_box(&layer5)));
        });
    });

    group.finish();
}

/// Benchmark multiple Involution processes
///
/// This measures the performance of creating multiple seeds
/// in sequence, which is useful for understanding scalability.
fn bench_multiple_involution_processes(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_involution_processes");

    for num_seeds in [1, 10, 50, 100, 500] {
        group.throughput(Throughput::Elements(num_seeds as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(num_seeds),
            &num_seeds,
            |b, &num_seeds| {
                b.iter(|| {
                    let mut seeds = Vec::with_capacity(num_seeds);
                    let mut involution = InvolutionProcess::new();
                    for _ in 0..num_seeds {
                        seeds.push(black_box(
                            involution.create_potential(black_box(Density::D3)),
                        ));
                    }
                    black_box(seeds);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Involution process memory allocation
///
/// This measures the memory overhead of creating HolographicSeeds.
fn bench_involution_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("involution_memory");

    group.bench_function("create_and_drop_seed", |b| {
        b.iter(|| {
            let mut involution = InvolutionProcess::new();
            let seed = involution.create_potential(black_box(Density::D3));
            black_box(seed);
            // Seed is dropped here
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_involution_process_complete,
    bench_involution_process_by_density,
    bench_involution_layers,
    bench_multiple_involution_processes,
    bench_involution_memory
);
criterion_main!(benches);
