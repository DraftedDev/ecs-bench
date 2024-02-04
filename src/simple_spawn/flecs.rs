use std::ops::Range;

use flecs::World;

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    for _ in count {
        world.entity();
    }

    world.reset();

    world
}
