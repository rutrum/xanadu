pub mod commands;
pub mod player;
pub mod world;

use convert_case::{Case, Casing};
use std::str::FromStr;
use std::fmt;
use std::collections::{HashSet, HashMap};

pub struct Player {
    inventory: Inventory,
    state: State,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_case(Case::Flat).as_str() {
            "n" | "north" => Ok(Direction::North),
            "s" | "south" => Ok(Direction::South),
            "e" | "east" => Ok(Direction::East),
            "w" | "west" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
pub struct ItemMap {
    items: HashMap<String, HashSet<Item>>,
}

impl ItemMap {
    pub fn new() -> ItemMap {
        Default::default()
    }

    pub fn take(&mut self, current: &str, item_str: &str) -> Option<Item> {
        self.items.get_mut(current).map(|items| {
            items
                .iter()
                .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
                .cloned() // Be sure to clone the insides..not just the option!
                .map(|item| {
                    // Found ref to item, now remove it
                    items.take(&item)
                }).flatten()
        }).flatten()
    }

    pub fn put(&mut self, key: &str, item: Item) {
        match self.items.get_mut(key) {
            Some(set) => {
                set.insert(item);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(item);
                self.items.insert(key.to_string(), set);
            }
        }
    }

    pub fn print_at(&self, key: &str) {
        if let Some(set) = self.items.get(key) {
            for i in set {
                println!("{}", i.find);
            }
        }
    }
}


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub find: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
