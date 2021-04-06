extern crate enum_map;
use crate::inventory::Inventory;
use crate::core::stats;
use crate::core::stats::{Stats, Modifier};
use crate::core::background::Background;
use std::fmt; 


// use enum_map::{enum_map, Enum, EnumMap}; // idea: use this as the indexing-by-stattype tool 

#[derive(Debug)]
pub struct Character {
    pub alias: String,
    pub background: Background,
    pub stats: stats::Stats,
    pub inventory: Inventory,
    pub equipment: Inventory,
    pub active: bool
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, from {}]", self.alias, self.background)
    }
}

pub struct Quirk {
    name: String,
    modifiers: Vec<Modifier>,
}

impl Quirk {
    pub fn default() -> Quirk {
        let empty_mods_vec: Vec<Modifier> = Vec::new();
        Quirk { 
            name: String::from("Default Quirk"),
            modifiers: empty_mods_vec,
        }
       
    }

    pub fn new(name: String) -> Quirk {
        let modifiers: Vec<Modifier> = Vec::new();
        Quirk { 
            name: name,
            modifiers: modifiers,
        }
    }
}

pub trait CanTalk {
    fn say_hi(&self);
}

impl CanTalk for Character {
    fn say_hi(&self) {
        println!("Hi, I'm {} from {}", self.alias, self.background);
    }
}

#[derive(Debug)]
enum ModType {
    Physical,
    Mental,
}

pub fn will_into_existence(alias: String, background: Background, stats: Stats, inventory: Inventory, equipment: Inventory, active: bool) -> Character {
    Character {
        alias,
        background,
        stats,
        inventory,
        equipment,
        active
    }
}