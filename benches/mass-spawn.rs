//! # Benchmark: mass-spawn
//!
//! ## Program:
//! 1. Spawns a bulk of entities with the same component A and B, both evenly distributed and with type usize.
//! 2. Deletes all the entities with component A.
//! 3. Clears all entities.
//!

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};

const ENTITY_COUNTS: [usize; 4] = [1_000, 10_000, 100_000, 1_000_000];

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mass-spawn");

    for i in ENTITY_COUNTS {
        group.bench_with_input(BenchmarkId::new("bevy_ecs", i), &i, |b, input| {
            b.iter(|| {
                black_box(|| {
                    ecs_bench::mass_spawn::bevy_ecs::bench(1..(*input / 2), (*input / 2)..*input)
                })
            })
        });

        group.bench_with_input(BenchmarkId::new("hecs", i), &i, |b, input| {
            b.iter(|| {
                black_box(|| {
                    ecs_bench::mass_spawn::hecs::bench(1..(*input / 2), (*input / 2)..*input)
                })
            })
        });

        group.bench_with_input(BenchmarkId::new("legion", i), &i, |b, input| {
            b.iter(|| {
                black_box(|| {
                    ecs_bench::mass_spawn::legion::bench(1..(*input / 2), (*input / 2)..*input)
                })
            })
        });

        group.bench_with_input(BenchmarkId::new("shipyard", i), &i, |b, input| {
            b.iter(|| {
                black_box(|| {
                    ecs_bench::mass_spawn::shipyard::bench(1..(*input / 2), (*input / 2)..*input)
                })
            })
        });

        group.bench_with_input(BenchmarkId::new("flecs", i), &i, |b, input| {
            b.iter(|| {
                black_box(|| {
                    ecs_bench::mass_spawn::flecs::bench(1..(*input / 2), (*input / 2)..*input)
                })
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench,);

criterion_main!(benches);
