//! # Benchmark: mass-query
//!
//! ## Program:
//! 1. Spawns 10x the given amount of entities each with 1, 2, 3, 4, 5, 6, 7, 8, 9 and 10 components.
//! 2. Executes 5 systems for querying for 5 components, each mutating them by 1.
//!

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};

const ENTITY_COUNTS: [usize; 3] = [100, 1_000, 10_000];

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mass-query");

    for i in ENTITY_COUNTS {
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::mass_query::bevy_ecs::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("hecs", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::mass_query::hecs::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("legion", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::mass_query::legion::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("shipyard", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::mass_query::shipyard::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("flecs", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::mass_query::flecs::bench(1..*input)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench,);

criterion_main!(benches);
