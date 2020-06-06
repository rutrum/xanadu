use crate::item::Item;
use std::collections::HashSet;

pub type Inventory = HashSet<Item>;

#[derive(Default)]
pub struct Player {
    inventory: Inventory,
    state: State,
}

impl Player {
    pub fn new() -> Player {
        Default::default()
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.insert(item);
    }

    pub fn remove_item(&mut self, item_str: &str) -> Option<Item> {
        self.inventory
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .cloned() // Be sure to clone the insides..not just the option!
            .map(|item| {
                // Found ref to item, now remove it
                self.inventory.take(&item)
            }).flatten()
    }

    pub fn item_description(&self, item_str: &str) -> Option<&str> {
        self.inventory
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .map(|x| x.description.as_str())
    }

    pub fn print_inv(&self) {
        match self.inventory.len() {
            0 => println!("Your pockets are empty."),
            _ => {
                let mut list = String::new(); 
                for (i, item) in self.inventory.iter().enumerate() {
                    if i == self.inventory.len() - 1 {
                        list.push_str(" and ");
                    } else {
                        list.push(' ');
                    }

                    if "aeiou".contains(item.name.chars().next().unwrap()) {
                        list.push_str("an ");
                    } else {
                        list.push_str("a ");
                    }
                    list.push_str(&item.name);
                }
                println!("You have{}.", list);
            }
        }
    }
}

pub enum State {
    Alive,
    Dead,
}

impl Default for State {
    fn default() -> State { State::Alive }
}
