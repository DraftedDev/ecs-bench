use std::ops::Range;

use flecs::World;

pub fn bench(count_a: Range<usize>, count_b: Range<usize>) -> World {
    let mut world = World::new();
    world.component::<ComponentA>();
    world.component::<ComponentB>();

    for i in count_a {
        world.entity().set(ComponentA(i));
    }

    for i in count_b {
        world.entity().set(ComponentB(i));
    }

    world.remove_all_with_component::<ComponentA>();

    world.reset();

    world
}

#[derive(Debug, PartialEq)]
struct ComponentA(usize);

#[derive(Debug, PartialEq)]
struct ComponentB(usize);
