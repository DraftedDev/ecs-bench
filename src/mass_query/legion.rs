use std::ops::Range;

use legion::{IntoQuery, World, Write};

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::default();

    // spawn entities
    {
        for i in count.clone() {
            world.push((ComponentA(i),));
        }

        for i in count.clone() {
            world.push((ComponentA(i), ComponentB(i, i)));
        }

        for i in count.clone() {
            world.push((ComponentA(i), ComponentB(i, i), ComponentC(i, i, i)));
        }

        for i in count.clone() {
            world.push((
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
            ));
        }

        for i in count.clone() {
            world.push((
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
            ));
        }

        for i in count.clone() {
            world.push((
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
            ));
        }

        for i in count.clone() {
            world.push((
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
            ));
        }

        for i in count.clone() {
            world.push((
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
                ComponentH(i, i, i, i, i, i, i, i),
            ));
        }

        for i in count.clone() {
            world.push((
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
                ComponentG(i, i, i, i, i, i, i),
                ComponentH(i, i, i, i, i, i, i, i),
                ComponentI(i, i, i, i, i, i, i, i, i),
            ));
        }

        for i in count.clone() {
            world.push((
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
            ));
        }
    }

    // run systems
    {
        Write::<ComponentA>::query()
            .iter_mut(&mut world)
            .for_each(|e| {
                e.0 += 1;
            });

        Write::<(ComponentA, ComponentB)>::query()
            .iter_mut(&mut world)
            .for_each(|e| {
                e.0 .0 += 1;
                e.1 .0 += 1;
            });

        Write::<(ComponentA, ComponentB, ComponentC)>::query()
            .iter_mut(&mut world)
            .for_each(|e| {
                e.0 .0 += 1;
                e.1 .0 += 1;
                e.2 .0 += 1;
            });

        Write::<(ComponentA, ComponentB, ComponentC, ComponentD)>::query()
            .iter_mut(&mut world)
            .for_each(|e| {
                e.0 .0 += 1;
                e.1 .0 += 1;
                e.2 .0 += 1;
                e.3 .0 += 1;
            });

        Write::<(ComponentA, ComponentB, ComponentC, ComponentD, ComponentE)>::query()
            .iter_mut(&mut world)
            .for_each(|e| {
                e.0 .0 += 1;
                e.1 .0 += 1;
                e.2 .0 += 1;
                e.3 .0 += 1;
                e.4 .0 += 1;
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
