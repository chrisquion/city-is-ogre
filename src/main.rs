mod core;
mod inventory;
mod util;

use inventory::Inventory;
use crate::core::background::{*};
use crate::core::stats::{*};
use crate::core::character::{Character, Quirk};
use crate::core::spacetime::{*};
use crate::core::character::CanTalk;
use legion::systems::CommandBuffer;
use util::{*};

use legion::*;

/*
>>~~~*~*~~~>>=====*=====**>>What's going on<<**=====*=====<<~~~*~*~~~<<
    The universe is a mish-mash. It's always been a mish, but it 
    hasn't always been a mash. Once the mash was added, the universe 
    started to spin, which terribly threw everyone in it for a tumble.
    

*/
fn main() {
    // get our plates, and utensils, and prepare for the mish-mash
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut read_spacetime_query = <(Entity, Read<Character>, Read<Position>, Read<Velocity>)>::query();
    let mut command_buffer = CommandBuffer::new(&world);
    let mut time = Time::let_there_be_time();
    let mut resources = Resources::default();
    resources.insert(time);

    // setup the things in the mish-mash
    asset_writer::test_write_character_db();
    let character_db_data = asset_loader::test_read_character_db();
    let alias = String::from("Cosmo the Space Rat A");
    let mods_a: Vec<Modifier> = Vec::new();
    let mods_b: Vec<Modifier> = Vec::new();
    let culture = Heritage::new(String::from("Winnesota"), mods_a);
    let heritage = Heritage::new(String::from("SpaceRat"), mods_b);
    let bg = Background::new(culture, heritage, 0);
    let stats = Stats::new();
    let inv = Inventory::new();
    let equip = Inventory::new();
    let characters: Vec<Character> = Vec::new();

    // push a component tuple into the world to create an entity
    let cosmo1: Entity = world.push(
        (Character::will_into_existence(alias, bg, stats, inv, equip), 
            Position { x: 0.0, y: 0.0 }, 
            Velocity { dx: 0.0, dy: 0.0 }
        )
    );
    let comso2: Entity = world.push(
        (character_db_data,
            Position { x: 0.0, y: 0.1 }, 
            Velocity { dx: 0.0, dy: 0.0 }
        )
    );

    
    let greeting_system = SystemBuilder::new("GreetAll")
    .with_query(read_spacetime_query)
    .build(|command_buffer, world, time, query| {
        for (entity, character, pos, vel) in query.iter_mut(world) {
            character.say_hi();
        }
    });

    let mut schedule = Schedule::builder()
    .add_system(greeting_system)
    .build();

    schedule.execute(&mut world, &mut resources);
}
