use std::ops::Range;

use hecs::{Entity, World};

pub fn bench(count_a: Range<usize>, count_b: Range<usize>) -> World {
    let mut world = World::new();

    world.spawn_batch(count_a.map(|i| (ComponentA(i),)));
    world.spawn_batch(count_b.map(|i| (ComponentB(i),)));

    {
        let entities = world
            .query::<&ComponentA>()
            .iter()
            .map(|(e, _)| e)
            .collect::<Vec<Entity>>();

        for e in entities {
            world.despawn(e).unwrap();
        }
    }

    world.clear();

    world
}

#[allow(dead_code)]
struct ComponentA(usize);

#[allow(dead_code)]
struct ComponentB(usize);
