use std::ops::Range;

use flecs::{World};

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();
    world.component::<ComponentA>();
    world.component::<ComponentB>();
    world.component::<ComponentC>();
    world.component::<ComponentD>();
    world.component::<ComponentE>();
    world.component::<ComponentF>();
    world.component::<ComponentG>();
    world.component::<ComponentH>();
    world.component::<ComponentI>();
    world.component::<ComponentJ>();

    // spawn entities
    {
        for i in count.clone() {
            world.entity().set(ComponentA(i));
        }

        for i in count.clone() {
            world.entity().set(ComponentA(i)).set(ComponentB(i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i))
                .set(ComponentE(i, i, i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i))
                .set(ComponentE(i, i, i, i, i))
                .set(ComponentF(i, i, i, i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i))
                .set(ComponentE(i, i, i, i, i))
                .set(ComponentF(i, i, i, i, i, i))
                .set(ComponentG(i, i, i, i, i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i))
                .set(ComponentE(i, i, i, i, i))
                .set(ComponentF(i, i, i, i, i, i))
                .set(ComponentG(i, i, i, i, i, i, i))
                .set(ComponentH(i, i, i, i, i, i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i))
                .set(ComponentE(i, i, i, i, i))
                .set(ComponentF(i, i, i, i, i, i))
                .set(ComponentG(i, i, i, i, i, i, i))
                .set(ComponentH(i, i, i, i, i, i, i, i))
                .set(ComponentI(i, i, i, i, i, i, i, i, i));
        }

        for i in count.clone() {
            world
                .entity()
                .set(ComponentA(i))
                .set(ComponentB(i, i))
                .set(ComponentC(i, i, i))
                .set(ComponentD(i, i, i, i))
                .set(ComponentE(i, i, i, i, i))
                .set(ComponentF(i, i, i, i, i, i))
                .set(ComponentG(i, i, i, i, i, i, i))
                .set(ComponentH(i, i, i, i, i, i, i, i))
                .set(ComponentI(i, i, i, i, i, i, i, i, i))
                .set(ComponentJ(i, i, i, i, i, i, i, i, i, i));
        }
    }

    // run systems
    {
        world
            .system()
            .each_mut::<(ComponentA, ())>(|_e, c| {
                c.0 .0 += 1;
            })
            .run(0.0);

        world
            .system()
            .each_mut::<(ComponentA, ComponentB)>(|_e, c| {
                c.0 .0 += 1;
                c.1 .0 += 1;
            })
            .run(0.0);

        world
            .system()
            .each_mut::<(ComponentA, ComponentB, ComponentC)>(|_e, c| {
                c.0 .0 += 1;
                c.1 .0 += 1;
                c.2 .0 += 1;
            })
            .run(0.0);

        world
            .system()
            .each_mut::<(ComponentA, ComponentB, ComponentC, ComponentD)>(|_e, c| {
                c.0 .0 += 1;
                c.1 .0 += 1;
                c.2 .0 += 1;
                c.3 .0 += 1;
            })
            .run(0.0);

        world
            .system()
            .each_mut::<(ComponentA, ComponentB, ComponentC, ComponentD, ComponentE)>(|_e, c| {
                c.0 .0 += 1;
                c.1 .0 += 1;
                c.2 .0 += 1;
                c.3 .0 += 1;
                c.4 .0 += 1;
            })
            .run(0.0);
    }

    world
}

#[derive(Debug, PartialEq)]
struct ComponentA(usize);

#[derive(Debug, PartialEq)]
struct ComponentB(usize, usize);

#[derive(Debug, PartialEq)]
struct ComponentC(usize, usize, usize);

#[derive(Debug, PartialEq)]
struct ComponentD(usize, usize, usize, usize);

#[derive(Debug, PartialEq)]
struct ComponentE(usize, usize, usize, usize, usize);

#[derive(Debug, PartialEq)]
struct ComponentF(usize, usize, usize, usize, usize, usize);

#[derive(Debug, PartialEq)]
struct ComponentG(usize, usize, usize, usize, usize, usize, usize);

#[derive(Debug, PartialEq)]
struct ComponentH(usize, usize, usize, usize, usize, usize, usize, usize);

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
