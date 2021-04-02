use crate::core::character;
use crate::core::background::Background;
use crate::core::stats::Stats;
use crate::Inventory;

pub fn will_into_existence(alias: String, background: Background, stats: Stats, inventory: Inventory, equipment: Inventory, active: bool) -> character::Character {
    character::Character {
        alias,
        background,
        stats,
        inventory,
        equipment,
        active
    }
}