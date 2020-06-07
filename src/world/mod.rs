pub mod locale;

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use serde_json::Value;

use locale::{LocaleBuilder, Locale};
use crate::{Direction, item::{Item, ItemError, ItemMap}};
use crate::MovementError;

pub struct WorldBuilder {
    name: String,
    locales: HashMap<String, LocaleBuilder>,
    items: ItemMap,
}

impl WorldBuilder {
    pub fn new(name: String) -> WorldBuilder {
        WorldBuilder {
            name,
            locales: HashMap::new(),
            items: ItemMap::new(),
        }
    }

    fn add_locale(&mut self, locale: LocaleBuilder) {
        self.locales.insert(locale.name.clone(), locale);
    }

    pub fn build(self) -> World {
        let locales = self.locales.into_iter().map(|(x, lb)| (x, lb.build())).collect();
        World { 
            name: self.name,
            locales: locales,
            items: self.items,
        }
    }

    pub fn from_file(filename: &str) -> WorldBuilder {
        let contents = fs::read_to_string(filename).unwrap();
        let data: Value = serde_json::from_str(&contents).expect("Bad parse");

        let mut w = WorldBuilder::new(data["name"].to_string());
        if let Value::Array(locales) = &data["locales"] {
            for raw in locales {
                let key = raw["name"].as_str().unwrap();
                let mut lb = LocaleBuilder::new()
                    .with_name(key)
                    .with_description(raw["description"].as_str().unwrap());

                if let Value::Object(dirs) = &raw["adjacent"] {
                    for (dir, loc) in dirs {
                        lb = lb.add_adjacent(
                            Direction::from_str(dir).unwrap(),
                            loc.as_str().unwrap(),
                        )
                    }
                }

                if let Value::Array(items) = &raw["items"] {
                    for item in items {
                        let i = Item {
                            name: item["name"].as_str().unwrap().to_string().to_lowercase(),
                            description: item["description"].as_str().unwrap().to_string(),
                            aliases: item["aliases"].as_array().unwrap_or(&Vec::new()).clone().iter().map(|x| x.as_str().unwrap().to_string().to_lowercase()).collect(),
                            find: item["find"].as_str().unwrap().to_string(),
                            read: item["read"].as_str().map(|x| x.to_string()),
                        };
                        w.items.put(key, i);
                    }
                }
                w.add_locale(lb);
            }
        }
        w.validate();
        w
    }

    fn validate(&self) {
        let keys = self
            .locales
            .iter()
            .map(|(n, _)| n)
            .collect::<Vec<&String>>();
        if !self
            .locales
            .iter()
            .all(|(_, l)| l.adjacent.iter().all(|(_, next)| keys.contains(&next)))
        {
            panic!("Exists a location that doesn't exist");
        }
    }
}

#[derive(Debug)]
pub struct World {
    name: String,
    locales: HashMap<String, Locale>,
    items: ItemMap,
}

impl World {
    pub fn print_current(&self, current: &str) {
        let l = self.locales.get(&current.to_string()).unwrap();
        println!("{}", l);
        self.items.print_at(current);
    }

    pub fn take_item(&mut self, current: &str, item: &str) -> Result<Item, ItemError> {
        self.items.take(current, item)
    }

    pub fn next_locale(&self, current: &str, dir: Direction) -> Result<String, MovementError> {
        let locale = &self.locales.get(&current.to_string()).unwrap();
        locale.get_adjacent(dir)
    }

    pub fn get_item(&self, current: &str, item: &str) -> Result<&Item, ItemError> {
        self.items.get(current, item)
    }
}
