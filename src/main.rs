mod core;
mod inventory;
use inventory::Inventory;
use crate::core::background;
use crate::core::stats::{Stats};
use crate::core::character::{Character, Quirk};
use crate::core::character::CanTalk;

use legion::*;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

fn main() {
    let mut allBackgrounds: Vec<Quirk> = Vec::new();
    allBackgrounds.push(Quirk::new(String::from("EagleEye")));
    allBackgrounds.push(Quirk::new(String::from("FastHands")));
    let mut world = World::default();

    // push a component tuple into the world to create an entity
    let entity: Entity = world.push((Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));

    // or extend via an IntoIterator of tuples to add many at once (this is faster)
    let entities: &[Entity] = world.extend(vec![
        (Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 1.0, y: 1.0 }, Velocity { dx: 0.0, dy: 0.0 }),
        (Position { x: 2.0, y: 2.0 }, Velocity { dx: 0.0, dy: 0.0 }),
    ]);
}
