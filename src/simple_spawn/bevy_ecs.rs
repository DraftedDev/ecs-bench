use std::ops::Range;

use bevy_ecs::world::World;

pub fn bench(count: Range<usize>) -> World {
    let mut world = World::new();

    for _ in count {
        world.spawn(());
    }

    world.clear_entities();

    world
}
