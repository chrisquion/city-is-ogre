use std::io::prelude::*;
use std::path::Path;
use ron::de;
use ron::ser::{*};
use std::{collections::HashMap, fs::File};
use std::io::BufReader;
use crate::core::character::{*};

use crate::inventory::Inventory;
use crate::core::background::{*};
use crate::core::stats::{*};
use crate::core::character::{Character, Quirk};
use crate::core::spacetime::{Position, Velocity, Time};
use crate::core::character::CanTalk;
use legion::systems::CommandBuffer;
use crate::util::{*};

pub fn test_write_character_db() {
    let alias = String::from("Cosmo the Space Rat D");
    let mods_a: Vec<Modifier> = Vec::new();
    let mods_b: Vec<Modifier> = Vec::new();
    let culture = Heritage::new(String::from("Winnesota"), mods_a);
    let heritage = Heritage::new(String::from("SpaceRat"), mods_b);
    let bg = Background::new(culture, heritage, 0);
    let stats = Stats::new();
    let inv = Inventory::new();
    let equip = Inventory::new();
    let test_character: Character = Character::will_into_existence(alias, bg, stats, inv, equip);
    let pretty_config = PrettyConfig::default();
    let test_character_ron_str = to_string_pretty(&test_character, pretty_config).expect("Serialization failed");

    let mut file = match File::create("generated_characters.ron") {
        Ok(f) => f,
        Err(why) => panic!("Error creating file: {}", why),
    };
    write!(file, "{}", test_character_ron_str);
}