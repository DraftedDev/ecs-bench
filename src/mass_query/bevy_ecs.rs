use std::ops::Range;

use bevy_ecs::component::Component;


use bevy_ecs::system::{Query, RunSystemOnce};
use bevy_ecs::world::World;

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    // spawn entities
    {
        world.spawn_batch(count.clone().map(|i| ComponentA(i)));
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
    }

    // run systems
    {
        world.run_system_once(|mut q: Query<&mut ComponentA>| {
            q.for_each_mut(|mut a| {
                a.0 += 1;
            });
        });

        world.run_system_once(|mut q: Query<(&mut ComponentA, &mut ComponentB)>| {
            q.for_each_mut(|(mut a, mut b)| {
                a.0 += 1;
                b.0 += 1;
            });
        });

        world.run_system_once(
            |mut q: Query<(&mut ComponentA, &mut ComponentB, &mut ComponentC)>| {
                q.for_each_mut(|(mut a, mut b, mut c)| {
                    a.0 += 1;
                    b.0 += 1;
                    c.0 += 1;
                });
            },
        );

        world.run_system_once(
            |mut q: Query<(
                &mut ComponentA,
                &mut ComponentB,
                &mut ComponentC,
                &mut ComponentD,
            )>| {
                q.for_each_mut(|(mut a, mut b, mut c, mut d)| {
                    a.0 += 1;
                    b.0 += 1;
                    c.0 += 1;
                    d.0 += 1;
                });
            },
        );

        world.run_system_once(
            |mut q: Query<(
                &mut ComponentA,
                &mut ComponentB,
                &mut ComponentC,
                &mut ComponentD,
                &mut ComponentE,
            )>| {
                q.for_each_mut(|(mut a, mut b, mut c, mut d, mut e)| {
                    a.0 += 1;
                    b.0 += 1;
                    c.0 += 1;
                    d.0 += 1;
                    e.0 += 1;
                });
            },
        );
    }

    world
}

#[derive(Component)]
#[allow(dead_code)]
struct ComponentA(usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentB(usize, usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentC(usize, usize, usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentD(usize, usize, usize, usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentE(usize, usize, usize, usize, usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentF(usize, usize, usize, usize, usize, usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentG(usize, usize, usize, usize, usize, usize, usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentH(usize, usize, usize, usize, usize, usize, usize, usize);

#[derive(Component)]
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

#[derive(Component)]
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
