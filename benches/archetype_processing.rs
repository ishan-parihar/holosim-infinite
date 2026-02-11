// Criterion benchmark for archetype processing performance
// Task 13: Performance Optimization

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use organic_evolution::archetypes::*;
use organic_evolution::complex::Complex;
use organic_evolution::types::{ComplexType, Float, Position};

fn bench_matrix_trait_method_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_trait_methods");

    for complex_type in [ComplexType::Mind, ComplexType::Body, ComplexType::Spirit] {
        let mut complex = Complex::new(complex_type);

        group.bench_with_input(
            BenchmarkId::new("get_structural_permeability", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.matrix.get_structural_permeability());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new(
                "calculate_reaching_intensity",
                format!("{:?}", complex_type),
            ),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.matrix.calculate_reaching_intensity());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("available_capacity", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.matrix.available_capacity());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("multiple_getter_calls", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    let _ = black_box(complex.matrix.get_structural_permeability());
                    let _ = black_box(complex.matrix.get_resource_access());
                    let _ = black_box(complex.matrix.get_willful_integration());
                    let _ = black_box(complex.matrix.get_conscious_coherence());
                    let _ = black_box(complex.matrix.get_integration_capacity());
                });
            },
        );
    }

    group.finish();
}

fn bench_potentiator_trait_method_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("potentiator_trait_methods");

    for complex_type in [ComplexType::Mind, ComplexType::Body, ComplexType::Spirit] {
        let mut complex = Complex::new(complex_type);

        group.bench_with_input(
            BenchmarkId::new("get_resource_accessibility", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.potentiator.get_resource_accessibility());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("calculate_receptivity", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.potentiator.calculate_receptivity());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new(
                "calculate_regulatory_intensity",
                format!("{:?}", complex_type),
            ),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.potentiator.calculate_regulatory_intensity());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new(
                "calculate_illumination_intensity",
                format!("{:?}", complex_type),
            ),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.potentiator.calculate_illumination_intensity());
                });
            },
        );
    }

    group.finish();
}

fn bench_catalyst_trait_method_calls(c: &mut Criterion) {
    let mut group = c.benchmark_group("catalyst_trait_methods");

    for complex_type in [ComplexType::Mind, ComplexType::Body, ComplexType::Spirit] {
        let mut complex = Complex::new(complex_type);

        group.bench_with_input(
            BenchmarkId::new("get_catalyst_inflow", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.catalyst.get_catalyst_inflow());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("get_activated_rungs", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    black_box(complex.catalyst.get_activated_rungs());
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("set_catalyst_inflow", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    complex.catalyst.set_catalyst_inflow(black_box(0.5));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_matrix_trait_method_calls,
    bench_potentiator_trait_method_calls,
    bench_catalyst_trait_method_calls
);
criterion_main!(benches);
