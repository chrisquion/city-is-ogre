use super::stats::Modifier;
use std::fmt; 

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

