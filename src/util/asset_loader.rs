use std::io::prelude::*;
use std::path::Path;
use ron::de;
use std::{collections::HashMap, fs::File};
use std::io::BufReader;
use crate::core::character::{*};

pub fn test_read_character_db() -> Character {
    let character_db_path = Path::new("generated_characters.ron");
    let display = character_db_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&character_db_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    // println!("contents: {}", contents); 
    let character_test: Character = match de::from_str(&contents) {
        Ok(x) => x,
        Err(why) => {
            panic!("Error loading because {}", why);
        }
    };
    character_test
}