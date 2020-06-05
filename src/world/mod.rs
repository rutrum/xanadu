use std::collections::HashMap;
use std::fs;
use convert_case::{Casing, Case};
use std::str::FromStr;

pub mod locale;
pub use locale::{Item, Locale};

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

#[derive(Debug)]
pub struct World {
    name: String,
    locales: HashMap<String, Locale>,
}

impl World {
    pub fn new(name: String) -> World {
        World {
            name,
            locales: HashMap::new(),
        }
    }

    pub fn from_file(filename: &str) -> World {
        use serde_json::Value;
        let contents = fs::read_to_string(filename).unwrap();
        let data: Value = serde_json::from_str(&contents).expect("Bad parse");

        let mut w = World::new(data["name"].to_string());
        if let Value::Array(locales) = &data["locales"] {
            for locale_raw in locales {
                let mut l = Locale::new(
                    locale_raw["name"].as_str().unwrap(),
                    locale_raw["description"].as_str().unwrap(),
                );
                if let Value::Object(dirs) = &locale_raw["dirs"] {
                    for (dir, loc) in dirs {
                        l.add_adj(
                            Direction::from_str(dir).unwrap(),
                            loc.as_str().unwrap(),
                        );
                    }
                }
                if let Value::Array(items) = &locale_raw["items"] {
                    for item in items {
                        let i = Item {
                            name: item["name"].as_str().unwrap().to_string(),
                            description: item["description"].as_str().unwrap().to_string(),
                        };
                    l.items.push(i);
                    }
                }
                w.add_locale(l);
            }
        }
        w.validate();
        w
    }
    
    fn validate(&self) {
        let keys = self.locales.iter().map(|(n, _)| n).collect::<Vec<&String>>();
        if !self.locales.iter()
            .all(|(_, l)|
                l.adjacent.iter().all(|(_, next)| keys.contains(&next))
            )
        {
            panic!("Exists a location that doesn't exist");
        }
    }

    fn add_locale(&mut self, locale: Locale) {
        self.locales.insert(locale.name.clone(), locale);
    }

    pub fn print_current(&self, current: &str) {
        self.locales.get(&current.to_string()).unwrap().print();
    }

    pub fn take_item(&mut self, current: &str, item: &str) -> Option<Item> {
        self.locales.get_mut(&current.to_string()).unwrap().take_item(item)
    }

    pub fn next_locale(&self, current: &str, dir: Direction) -> Option<String> {
        let locale = &self.locales.get(&current.to_string()).unwrap();
        locale.adjacent.get(&dir).map(|x| x.to_string())
    }
}
