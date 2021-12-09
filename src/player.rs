use crate::item::{Item, ItemError};
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
    pub fn remove_item(&mut self, item_str: &str) -> Result<Item, ItemError> {
        let item = self
            .inventory
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .ok_or(ItemError::NoExist)?
            .clone();
        Ok(self.inventory.take(&item).unwrap())
    }

    pub fn item_description(&self, item_str: &str) -> Result<&str, ItemError> {
        let desc = self
            .inventory
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .ok_or(ItemError::NoExist)?
            .description
            .as_str();
        Ok(desc)
    }

    pub fn read_item(&self, item_str: &str) -> Result<&String, ItemError> {
        self.inventory
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .ok_or(ItemError::NoExist)?
            .read
            .as_ref()
            .ok_or(ItemError::CannotRead)
    }

    pub fn print_inv(&self) {
        match self.inventory.len() {
            0 => println!("You don't have anything on you."),
            1 => println!(
                "You have {}.",
                self.inventory.iter().next().unwrap().article_name()
            ),
            _ => {
                let mut list = String::new();
                for (i, item) in self.inventory.iter().enumerate() {
                    if i == self.inventory.len() - 1 {
                        list.push_str(" and ");
                    } else {
                        list.push(' ');
                    }
                    list.push_str(&item.article_name())
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
    fn default() -> State {
        State::Alive
    }
}
