//! Benchmarks for Φ calculation with different approximation methods.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use iit::*;

fn phi_calculation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("phi_calculation");

    // Benchmark different system sizes
    for size in [3, 4, 5, 6, 8, 10].iter() {
        group.bench_with_input(BenchmarkId::new("exact", size), size, |b, &size| {
            let mut system = fully_connected_system(size);
            let state = vec![1; size];
            system.set_state(state).unwrap();

            let mut config = PhiConfig::default();
            config.approximation = ApproximationMethod::Exact;
            config.max_exact_size = 20;
            system.set_config(config);

            b.iter(|| {
                let result = system.calculate_phi().unwrap();
                black_box(result);
            });
        });
    }

    group.finish();
}

fn approximation_methods_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("approximation_methods");

    let size = 10;
    let methods = vec![
        ApproximationMethod::Geometric,
        ApproximationMethod::Spectral,
        ApproximationMethod::MeanField,
        ApproximationMethod::Tau,
    ];

    for method in methods {
        group.bench_with_input(
            BenchmarkId::new("method", format!("{:?}", method)),
            &method,
            |b, &method| {
                let mut system = fully_connected_system(size);
                let state = vec![1; size];
                system.set_state(state).unwrap();

                let mut config = PhiConfig::default();
                config.approximation = method;
                config.max_exact_size = 0;
                system.set_config(config);

                b.iter(|| {
                    let result = system.calculate_phi().unwrap();
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

fn concept_identification_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("concept_identification");

    for size in [2, 3, 4].iter() {
        group.bench_with_input(BenchmarkId::new("concepts", size), size, |b, &size| {
            let mut system = fully_connected_system(size);
            let state = vec![1; size];
            system.set_state(state).unwrap();

            b.iter(|| {
                let ces = system.identify_concepts().unwrap();
                black_box(ces);
            });
        });
    }

    group.finish();
}

fn partition_enumeration_benchmark(c: &mut Criterion) {
    use iit::partition::{all_bipartitions, CutType};

    let mut group = c.benchmark_group("partition_enumeration");

    for size in [3, 4, 5, 6, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("bipartitions", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let partitions = all_bipartitions(size, CutType::Bidirectional);
                    black_box(partitions);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    phi_calculation_benchmark,
    approximation_methods_benchmark,
    concept_identification_benchmark,
    partition_enumeration_benchmark
);
criterion_main!(benches);
