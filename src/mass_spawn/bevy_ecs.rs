use std::ops::Range;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::With;
use bevy_ecs::world::World;

pub fn bench(count_a: Range<usize>, count_b: Range<usize>) -> World {
    let mut world = World::new();

    world.spawn_batch(count_a.map(|i| ComponentA(i)));

    world.spawn_batch(count_b.map(|i| ComponentB(i)));

    for e in world
        .query_filtered::<Entity, With<ComponentA>>()
        .iter_mut(&mut world)
        .collect::<Vec<Entity>>()
    {
        world.despawn(e);
    }

    world.clear_entities();

    world
}

#[derive(Component)]
#[allow(dead_code)]
struct ComponentA(usize);

#[derive(Component)]
#[allow(dead_code)]
struct ComponentB(usize);
