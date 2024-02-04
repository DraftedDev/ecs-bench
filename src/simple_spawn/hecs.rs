use std::ops::Range;

use hecs::World;

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    for _ in count {
        world.spawn(((),));
    }

    world.clear();

    world
}
