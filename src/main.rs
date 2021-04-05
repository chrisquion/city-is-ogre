mod core;
mod inventory;
use inventory::Inventory;
use crate::core::background;
use crate::core::stats::Stats;
use crate::core::character;
use crate::core::character::CanTalk;

fn main() {
    println!("Choose a culture:");
    println!("Choose a heritage:");
    let test_culture = background::Heritage::new(String::from("Steep Steppes"), Vec::new());
    let test_heritage = background::Heritage::new(String::from("Sunburnt"), Vec::new());
    let test_bg = background::Background::new(test_culture, test_heritage, 329524);
    let test_stats = Stats::new();
    let test_inventory = Inventory::new();
    let test_equipment = Inventory::new();


    let hennesy_james = test_character_creation(String::from("Hennesy James"));
    let hennesy_james2 = test_character_creation(String::from("Hennesy James2"));
    hennesy_james.say_hi();
}

pub fn test_character_creation(alias: String) -> character::Character{
  
    let soul = character::will_into_existence(alias, test_bg, test_stats, test_inventory, test_equipment, false);
    soul
}
