use convert_case::{Case, Casing};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Default, Debug)]
pub struct ItemMap {
    items: HashMap<String, HashSet<Item>>,
}

impl ItemMap {
    pub fn new() -> ItemMap {
        Default::default()
    }

    pub fn take(&mut self, current: &str, item_str: &str) -> Result<Item, ItemError> {
        let items = self.items.get_mut(current).ok_or(ItemError::NoExist)?;
        let item = items
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .ok_or(ItemError::NoExist)?
            .clone();
        Ok(items.take(&item).unwrap())
    }

    pub fn get(&self, current: &str, item_str: &str) -> Result<&Item, ItemError> {
        let items = self.items.get(current).ok_or(ItemError::NoExist)?;
        items
            .iter()
            .find(|x| x.name == item_str || x.aliases.contains(&item_str.to_string()))
            .ok_or(ItemError::NoExist)
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
    pub read: Option<String>,
}

impl Item {
    /// Returns the name of the Item with "a" or "an" preceding it
    pub fn article_name(&self) -> String {
        if "aeiou".contains(self.name.chars().next().unwrap()) {
            format!("an {}", self.name)
        } else {
            format!("a {}", self.name)
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name.to_case(Case::Title),
            self.description
        )
    }
}

#[derive(Debug)]
pub enum ItemError {
    NoExist,
    CannotRead,
}

impl Error for ItemError {}

impl fmt::Display for ItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ItemError::*;
        match self {
            NoExist => write!(f, "Not sure what item you are talking about."),
            CannotRead => write!(f, "You can't read that."),
        }
    }
}
