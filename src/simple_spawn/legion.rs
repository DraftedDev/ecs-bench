use std::ops::Range;

use legion::World;

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::default();

    for _ in count {
        world.push(());
    }

    world.clear();

    world
}
