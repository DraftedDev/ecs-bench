use std::ops::Range;

use hecs::{World};

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    // spawn entities
    {
        world.spawn_batch(count.clone().map(|i| (ComponentA(i),)));
        world.spawn_batch(count.clone().map(|i| (ComponentA(i), ComponentB(i, i))));
        world.spawn_batch(
            count
                .clone()
                .map(|i| (ComponentA(i), ComponentB(i, i), ComponentC(i, i, i))),
        );
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
            )
        }));
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
            )
        }));
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
            )
        }));
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
            )
        }));
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
                ComponentH(i, i, i, i, i, i, i, i),
            )
        }));
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
                ComponentH(i, i, i, i, i, i, i, i),
                ComponentI(i, i, i, i, i, i, i, i, i),
                ComponentJ(i, i, i, i, i, i, i, i, i, i),
            )
        }));
        world.spawn_batch(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
            )
        }));
    }

    // run systems
    {
        world
            .query_mut::<&mut ComponentA>()
            .into_iter()
            .for_each(|(_, c)| {
                c.0 += 1;
            });

        world
            .query_mut::<(&mut ComponentA, &mut ComponentB)>()
            .into_iter()
            .for_each(|(_, c)| {
                c.0 .0 += 1;
                c.1 .0 += 1;
            });

        world
            .query_mut::<(&mut ComponentA, &mut ComponentB, &mut ComponentC)>()
            .into_iter()
            .for_each(|(_, c)| {
                c.0 .0 += 1;
                c.1 .0 += 1;
                c.2 .0 += 1;
            });

        world
            .query_mut::<(
                &mut ComponentA,
                &mut ComponentB,
                &mut ComponentC,
                &mut ComponentD,
            )>()
            .into_iter()
            .for_each(|(_, c)| {
                c.0 .0 += 1;
                c.1 .0 += 1;
                c.2 .0 += 1;
                c.3 .0 += 1;
            });

        world
            .query_mut::<(
                &mut ComponentA,
                &mut ComponentB,
                &mut ComponentC,
                &mut ComponentD,
                &mut ComponentE,
            )>()
            .into_iter()
            .for_each(|(_, c)| {
                c.0 .0 += 1;
                c.1 .0 += 1;
                c.2 .0 += 1;
                c.3 .0 += 1;
                c.4 .0 += 1;
            });
    }

    world
}

#[allow(dead_code)]
struct ComponentA(usize);

#[allow(dead_code)]
struct ComponentB(usize, usize);

#[allow(dead_code)]
struct ComponentC(usize, usize, usize);

#[allow(dead_code)]
struct ComponentD(usize, usize, usize, usize);

#[allow(dead_code)]
struct ComponentE(usize, usize, usize, usize, usize);

#[allow(dead_code)]
struct ComponentF(usize, usize, usize, usize, usize, usize);

#[allow(dead_code)]
struct ComponentG(usize, usize, usize, usize, usize, usize, usize);

#[allow(dead_code)]
struct ComponentH(usize, usize, usize, usize, usize, usize, usize, usize);

#[allow(dead_code)]
struct ComponentI(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);

#[allow(dead_code)]
struct ComponentJ(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);
