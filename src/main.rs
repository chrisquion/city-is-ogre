mod core;
mod inventory;
mod util;

use inventory::Inventory;
use crate::core::background::{*};
use crate::core::stats::{*};
use crate::core::character::{Character, Quirk};
use crate::core::spacetime::{Position, Velocity, Time};
use crate::core::character::CanTalk;
use legion::systems::CommandBuffer;
use util::{*};

use legion::*;

fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut read_spacetime_query = <(Entity, Read<Character>, Read<Position>, Read<Velocity>)>::query();
    let mut command_buffer = CommandBuffer::new(&world);
    let character_db_data = asset_loader::test_read_character_db();

    println!("{}", character_db_data);

    let alias = String::from("Cosmo the Space Rat");
    let mods_a: Vec<Modifier> = Vec::new();
    let mods_b: Vec<Modifier> = Vec::new();
    let culture = Heritage::new(String::from("Winnesota"), mods_a);
    let heritage = Heritage::new(String::from("SpaceRat"), mods_b);
    let bg = Background::new(culture, heritage, 0);
    let stats = Stats::new();
    let inv = Inventory::new();
    let equip = Inventory::new();

    // push a component tuple into the world to create an entity
    let entity: Entity = world.push((Character::will_into_existence(alias, bg, stats, inv, equip), Position { x: 0.0, y: 0.0 }, Velocity { dx: 0.0, dy: 0.0 }));

    let system = SystemBuilder::new("GreetAll")
    .with_query(read_spacetime_query)
    .build(|command_buffer, world, time, query| {
        for (entity, character, pos, vel) in query.iter_mut(world) {
            character.say_hi();
        }
    });
}
