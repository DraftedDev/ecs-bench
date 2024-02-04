use std::ops::Range;

use shipyard::{Component, IntoIter, ViewMut, World};

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    // spawn entities
    {
        world.bulk_add_entity(count.clone().map(|i| (ComponentA(i))));
        world.bulk_add_entity(count.clone().map(|i| (ComponentA(i), ComponentB(i, i))));
        world.bulk_add_entity(
            count
                .clone()
                .map(|i| (ComponentA(i), ComponentB(i, i), ComponentC(i, i, i))),
        );
        world.bulk_add_entity(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
            )
        }));
        world.bulk_add_entity(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
            )
        }));
        world.bulk_add_entity(count.clone().map(|i| {
            (
                ComponentA(i),
                ComponentB(i, i),
                ComponentC(i, i, i),
                ComponentD(i, i, i, i),
                ComponentE(i, i, i, i, i),
                ComponentF(i, i, i, i, i, i),
            )
        }));
        world.bulk_add_entity(count.clone().map(|i| {
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
        world.bulk_add_entity(count.clone().map(|i| {
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
        world.bulk_add_entity(count.clone().map(|i| {
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
        world.bulk_add_entity(count.clone().map(|i| {
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
        world.run(|mut ca: ViewMut<ComponentA>| {
            for c in (&mut ca).iter() {
                c.0 += 1;
            }
        });

        world.run(|mut ca: ViewMut<ComponentA>, mut cb: ViewMut<ComponentB>| {
            for (c1, c2) in (&mut ca, &mut cb).iter() {
                c1.0 += 1;
                c2.0 += 1;
            }
        });

        world.run(
            |mut ca: ViewMut<ComponentA>,
             mut cb: ViewMut<ComponentB>,
             mut cc: ViewMut<ComponentC>| {
                for (c1, c2, c3) in (&mut ca, &mut cb, &mut cc).iter() {
                    c1.0 += 1;
                    c2.0 += 1;
                    c3.0 += 1;
                }
            },
        );

        world.run(
            |mut ca: ViewMut<ComponentA>,
             mut cb: ViewMut<ComponentB>,
             mut cc: ViewMut<ComponentC>,
             mut cd: ViewMut<ComponentD>| {
                for (c1, c2, c3, c4) in (&mut ca, &mut cb, &mut cc, &mut cd).iter()
                {
                    c1.0 += 1;
                    c2.0 += 1;
                    c3.0 += 1;
                    c4.0 += 1;
                }
            },
        );

        world.run(
            |mut ca: ViewMut<ComponentA>,
             mut cb: ViewMut<ComponentB>,
             mut cc: ViewMut<ComponentC>,
             mut cd: ViewMut<ComponentD>,
             mut ce: ViewMut<ComponentE>| {
                for (c1, c2, c3, c4, c5) in
                    (&mut ca, &mut cb, &mut cc, &mut cd, &mut ce).iter()
                {
                    c1.0 += 1;
                    c2.0 += 1;
                    c3.0 += 1;
                    c4.0 += 1;
                    c5.0 += 1;
                }
            },
        );
    }

    world
}

#[allow(dead_code)]
#[derive(Component)]
struct ComponentA(usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentB(usize, usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentC(usize, usize, usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentD(usize, usize, usize, usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentE(usize, usize, usize, usize, usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentF(usize, usize, usize, usize, usize, usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentG(usize, usize, usize, usize, usize, usize, usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentH(usize, usize, usize, usize, usize, usize, usize, usize);

#[allow(dead_code)]
#[derive(Component)]
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
#[derive(Component)]
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
