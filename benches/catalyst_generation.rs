// Criterion benchmark for catalyst generation and complex processing performance
// Task 13: Performance Optimization

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use organic_evolution::complex::Complex;
use organic_evolution::types::{ComplexType, Float, Position};

fn bench_catalyst_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("catalyst_generation");

    for complex_type in [ComplexType::Mind, ComplexType::Body, ComplexType::Spirit] {
        let mut complex = Complex::new(complex_type);
        let position = Position::new(1, 1, 1);

        group.bench_with_input(
            BenchmarkId::new("single_step", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    complex.process_step(black_box(0.5), black_box(&position));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("ten_steps", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    for _ in 0..10 {
                        complex.process_step(black_box(0.5), black_box(&position));
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("hundred_steps", format!("{:?}", complex_type)),
            &complex_type,
            |b, _| {
                b.iter(|| {
                    for _ in 0..100 {
                        complex.process_step(black_box(0.5), black_box(&position));
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_complex_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_operations");

    let mut mind_complex = Complex::new(ComplexType::Mind);
    let mut body_complex = Complex::new(ComplexType::Body);
    let mut spirit_complex = Complex::new(ComplexType::Spirit);

    group.bench_function("get_total_catalyst_intensity", |b| {
        b.iter(|| {
            black_box(mind_complex.get_total_catalyst_intensity());
        });
    });

    group.bench_function("get_total_bias", |b| {
        b.iter(|| {
            black_box(mind_complex.get_total_bias());
        });
    });

    group.bench_function("get_health_status", |b| {
        b.iter(|| {
            black_box(mind_complex.get_health_status());
        });
    });

    group.bench_function("get_pathology_diagnosis", |b| {
        b.iter(|| {
            black_box(mind_complex.get_pathology_diagnosis());
        });
    });

    group.bench_function("get_effective_processing_rate", |b| {
        b.iter(|| {
            black_box(mind_complex.get_effective_processing_rate());
        });
    });

    group.finish();
}

fn bench_veil_mechanics(c: &mut Criterion) {
    let mut group = c.benchmark_group("veil_mechanics");

    let mut complex = Complex::new(ComplexType::Mind);

    group.bench_function("modify_veil_by_experience", |b| {
        b.iter(|| {
            complex.modify_veil_by_experience(black_box(1.0));
        });
    });

    group.bench_function("modify_veil_by_polarity_sto", |b| {
        b.iter(|| {
            complex.modify_veil_by_polarity(
                black_box(crate::archetypes::common::Polarization::STO),
                black_box(1.0),
            );
        });
    });

    group.bench_function("modify_veil_by_polarity_sts", |b| {
        b.iter(|| {
            complex.modify_veil_by_polarity(
                black_box(crate::archetypes::common::Polarization::STS),
                black_box(1.0),
            );
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_catalyst_generation,
    bench_complex_operations,
    bench_veil_mechanics
);
criterion_main!(benches);
