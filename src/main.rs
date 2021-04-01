mod character {
    pub mod stats;
}
mod inventory;
use inventory::Inventory;


fn main() {
    println!("Hello, world!");
    let hennesy_james = test_character_creation();
}

pub fn test_character_creation() {
    let test_culture = character::background::Heritage::new(String::from("Steep Steppes"), Vec::new());
    let test_heritage = character::background::Heritage::new(String::from("Sunburnt"), Vec::new());
    let test_bg = character::background::Background::new(test_culture, test_heritage, 329524);
    let test_stats = stats::Stats::new();
    let test_inventory = Inventory::new();
    let test_equipment = Inventory::new();
    let knight_errant = character::creation::will_into_existence(String::from("Knight Errant"), test_bg, test_stats, test_inventory, test_equipment, false);
    println!(": {}\n", knight_errant);
}
