use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats { // there are 9 stats // idea: use enum to index into array rather than hashmap to save save on size and insertion time?
    base_stats_map: HashMap<StatType, Stat>,
    modifiers: Vec<Modifier>,
    ultimate_stats_map: HashMap<StatType, Stat>,
}
#[derive(Debug)]
pub struct Stat {
    s_type: StatType,
    base_value: i32,
    current_value: i32,
}
#[derive(Debug)]
pub struct Modifier {
    name: String,
    /*
    mod_type: ModType,
    target: Character,
    target_stats: Vec<Stat>
    */
}

#[derive(Debug, PartialEq, Eq, Hash)]    
enum StatType {
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
impl Stats {
    pub fn new() -> Stats {
        let base_stats_map = HashMap::new();
        let ultimate_stats_map = HashMap::new();
        let modifiers: Vec<Modifier> = Vec::new();
        Stats {
            base_stats_map,
            modifiers,
            ultimate_stats_map
        }
    }
}
