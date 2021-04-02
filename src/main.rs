mod core;
mod inventory;
use inventory::Inventory;
use crate::core::background;
use crate::core::stats::Stats;


fn main() {
    println!("Hello, world!");
    let hennesy_james = test_character_creation(String::from("Hennesy James"));
    let hennesy_james2 = test_character_creation(String::from("Hennesy James2"));
}

pub fn test_character_creation(alias: String) {
    let test_culture = background::Heritage::new(String::from("Steep Steppes"), Vec::new());
    let test_heritage = background::Heritage::new(String::from("Sunburnt"), Vec::new());
    let test_bg = background::Background::new(test_culture, test_heritage, 329524);
    let test_stats = Stats::new();
    let test_inventory = Inventory::new();
    let test_equipment = Inventory::new();
    let soul = core::creation::will_into_existence(alias, test_bg, test_stats, test_inventory, test_equipment, false);
    println!(": {}\n", soul);
}
