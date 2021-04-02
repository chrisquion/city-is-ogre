extern crate enum_map;
use crate::inventory::Inventory;
use crate::core::stats;
use crate::core::background;
use std::fmt; 


// use enum_map::{enum_map, Enum, EnumMap}; // idea: use this as the indexing-by-stattype tool 

#[derive(Debug)]
pub struct Character {
    pub alias: String,
    pub background: background::Background,
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

struct State {
    active_modifiers: Vec<stats::Modifier>,
}

struct Trait {
    name: String,
    modifiers: Vec<stats::Modifier>,
}

#[derive(Debug)]
enum ModType {
    Physical,
    Mental,
}

