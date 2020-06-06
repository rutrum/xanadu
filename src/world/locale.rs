use crate::Direction;
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Debug)]
pub struct LocaleBuilder {
    pub name: String,
    pub description: String,
    pub adjacent: HashMap<Direction, String>,
}

impl LocaleBuilder {
    pub fn new() -> LocaleBuilder {
        Default::default()
    }

    pub fn add_adjacent(mut self, dir: Direction, key: &str) -> LocaleBuilder {
        self.adjacent.insert(dir, key.to_string());
        self
    }

    pub fn with_description(self, description: &str) -> LocaleBuilder {
        LocaleBuilder {
            description: description.to_string(),
            ..self
        }
    }

    pub fn with_name(self, name: &str) -> LocaleBuilder {
        LocaleBuilder {
            name: name.to_string(),
            ..self
        }
    }

    pub fn build(self) -> Locale {
        Locale {
            name: self.name,
            description: self.description,
            adjacent: self.adjacent,
        }
    }
}

#[derive(Debug)]
pub struct Locale {
    name: String,
    description: String,
    adjacent: HashMap<Direction, String>,
}

impl Locale {
    /// Returns the name of the locale used in hashmaps
    pub fn key(&self) -> String {
        self.name.clone()
    }

    /// Returns the name of the locale in the given direction if 
    /// that locale exists
    pub fn get_adjacent(&self, dir: Direction) -> Option<String> {
        self.adjacent.get(&dir).map(String::to_string)
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "You are at the {}.", self.name)?;
        write!(f, "{}", self.description)
    }
}
