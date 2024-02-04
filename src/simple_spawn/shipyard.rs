use std::ops::Range;

use shipyard::World;

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    for _ in count {
        world.add_entity(());
    }

    world.clear();

    world
}
