use std::io::prelude::*;
use std::path::Path;
use ron::de;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

pub fn test_read_character_db() -> String {
    let character_db_path = Path::new("src/data/character_db.ron");
    let display = character_db_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&character_db_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, file_contents),
    }

    let c = match de::from_str<Character>(&mut file_contents) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load Character: {}", e);

            std::process::exit(1);
        }
    };

    c
}