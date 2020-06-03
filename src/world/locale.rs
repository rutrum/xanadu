use std::collections::HashMap;
use super::Direction;

pub struct Locale {
    pub name: String,
    pub description: String,
    pub adjacent: HashMap<Direction, String>,
}

impl Locale {
    pub fn new(name: &str, description: &str) -> Locale {
        Locale {
            name: name.to_string(),
            description: description.to_string(),
            adjacent: HashMap::new(),
        }
    }

    pub fn add_adj(&mut self, dir: Direction, name: &str) {
        self.adjacent.insert(dir, name.to_string());
    }

    pub fn print(&self) {
        println!("You are at the {}.", self.name);
        println!("{}", self.description);
    }
}
