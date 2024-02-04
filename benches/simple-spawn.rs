//! # Benchmark: simple-spawn
//!
//! ## Program:
//! 1. Spawns the given number of entities (NO BULK SPAWN)
//! 2. Clears the world
//!

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};

const ENTITY_COUNTS: [usize; 6] = [5, 10, 50, 100, 500, 1_000];

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple-spawn");

    for i in ENTITY_COUNTS {
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::simple_spawn::bevy_ecs::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("hecs", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::simple_spawn::hecs::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("legion", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::simple_spawn::legion::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("shipyard", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::simple_spawn::shipyard::bench(1..*input)))
        });

        group.bench_with_input(BenchmarkId::new("flecs", i), &i, |b, input| {
            b.iter(|| black_box(|| ecs_bench::simple_spawn::flecs::bench(1..*input)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench,);

criterion_main!(benches);
