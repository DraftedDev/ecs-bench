use std::ops::Range;

use shipyard::{Component, SparseSet, World};

pub fn bench(count_a: Range<usize>, count_b: Range<usize>) -> World {
    let mut world = World::new();

    world.bulk_add_entity(count_a.map(|i| ComponentA(i)));
    world.bulk_add_entity(count_b.map(|i| ComponentB(i)));

    world.delete_any::<SparseSet<ComponentA>>();

    world.clear();

    world
}

#[allow(dead_code)]
#[derive(Component)]
struct ComponentA(usize);

#[allow(dead_code)]
#[derive(Component)]
struct ComponentB(usize);
