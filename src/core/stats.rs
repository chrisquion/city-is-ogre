use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter, Deserialize)]    
pub enum StatType {
    Finesse,
    Strength,
    Speed,
    Instinct,
    Senses,
    Memory,
    Intuition,
    Ruthlessness,
    Resolve,
}    
use StatType::{*};

#[derive(Clone, Debug, PartialEq, Deserialize)]
// there are 9 stats
pub struct Stats(Stat, Stat, Stat, Stat, Stat, Stat, Stat, Stat, Stat); 

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Stat {
    s_type: StatType,
    base_value: i32, 
    current_value: i32
}

impl Stat {
    pub fn new(s_type: StatType) -> Stat {
        let mut base_value = 0;
        let mut current_value = 0;
        Stat{
            s_type,
            base_value,
            current_value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Modifier {
    name: String,
    /*
    mod_type: ModType,
    target: Character,
    target_stats: Vec<Stat>
    */
}

impl Stats {
    pub fn new() -> Stats {
        Stats(
            Stat::new(Finesse), Stat::new(Strength), Stat::new(Speed),
            Stat::new(Instinct), Stat::new(Senses), Stat::new(Memory),
            Stat::new(Intuition), Stat::new(Ruthlessness), Stat::new(Resolve)
        )
    }
}