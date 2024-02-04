use std::ops::Range;

use legion::{Entity, IntoQuery, Read, World};

pub fn bench(count_a: Range<usize>, count_b: Range<usize>) -> World {
    let mut world = World::default();

    for i in count_a {
        world.push((ComponentA(i),));
    }

    for i in count_b {
        world.push((ComponentB(i),));
    }

    for entity in Read::<(Entity, ComponentA)>::query()
        .iter(&world)
        .map(|&(e, _)| e)
        .collect::<Vec<Entity>>()
    {
        world.remove(entity);
    }

    world.clear();

    world
}

#[allow(dead_code)]
struct ComponentA(usize);

#[allow(dead_code)]
struct ComponentB(usize);
