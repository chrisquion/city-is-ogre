extern crate enum_map;
use crate::inventory::Inventory;
pub mod stats;
// use enum_map::{enum_map, Enum, EnumMap}; // idea: use this as the indexing-by-stattype tool 

#[derive(Debug)]
pub struct Character {
    alias: String,
    background: background::Background,
    stats: stats::Stats,
    inventory: Inventory,
    equipment: Inventory,
    active: bool
}

pub mod creation {
    use super::{Character, background::Background, Inventory, stats::Stats, stats::Stat}; // bring Character structs into scope
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
}

pub mod background {
    use super::stats::Modifier;
    #[derive(Debug)]
    pub struct Background {
        culture: Heritage,
        heritage: Heritage,
        timeline: i32,
    }    
    #[derive(Debug)]
    pub struct Heritage {
        name: String,
        modifiers: Vec<Modifier>
    }
    impl Heritage {
        pub fn new(name: String, modifiers: Vec<Modifier>) -> Heritage {
            Heritage {
                name, 
                modifiers
            }
        }
    }
    impl Background {
        pub fn new(culture: Heritage, heritage: Heritage, timeline: i32) -> Background {
            Background {
                culture,
                heritage,
                timeline,
            }
        }
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

