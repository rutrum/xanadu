use std::collections::HashMap;
use super::Direction;

#[derive(Debug)]
pub struct Locale {
    pub name: String,
    pub description: String,
    pub adjacent: HashMap<Direction, String>,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub description: String,
}

impl Locale {
    pub fn new(name: &str, description: &str) -> Locale {
        Locale {
            name: name.to_string(),
            description: description.to_string(),
            adjacent: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn add_adj(&mut self, dir: Direction, name: &str) {
        self.adjacent.insert(dir, name.to_string());
    }

    pub fn print(&self) {
        println!("You are at the {}.", self.name);
        println!("{}", self.description);
        for i in &self.items {
            println!("You see a {}.", i.name);
        }
    }

    pub fn take_item(&mut self, item_str: &str) -> Option<Item> {
        if let Some(index) = self.items.iter()
            .position(|x| *x.name.to_lowercase() == item_str.to_lowercase()) 
        {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}
