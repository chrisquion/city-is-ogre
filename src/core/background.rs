use serde::Deserialize;
use super::stats::Modifier;
use crate::core::character::Quirk;
use std::fmt; 


#[derive(Debug, Deserialize)]
pub struct Background {
    culture: Heritage, // the culture the character grew up in
    heritage: Heritage, // the genetic makeup of the character
    timeline: i32, // the timeline the character is from
}    
#[derive(Debug, Deserialize)]
pub struct Heritage {
    name: String,
    modifiers: Vec<Modifier>
}

impl fmt::Display for Background {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.culture, self.heritage)
    }
}
impl fmt::Display for Heritage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.name)
    }
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

impl Background {
    pub fn new_empty_start() -> Background {
        let empty_mods_a: Vec<Modifier> = Vec::new();
        let empty_mods_b: Vec<Modifier> = Vec::new();
        let culture: Heritage = Heritage::new(String::from(""), empty_mods_a);
        let heritage: Heritage = Heritage::new(String::from(""), empty_mods_b);
        let timeline = 0;
        Background {
            culture, // the culture the character grew up in
            heritage,// the genetic makeup of the character
            timeline, // the timeline the character is from
        }
    }
}