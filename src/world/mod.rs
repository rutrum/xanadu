use std::collections::HashMap;
use std::fs;
use convert_case::{Casing, Case};
use toml;

pub mod locale;
pub use locale::Locale;

#[derive(Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn from_str(s: &str) -> Result<Direction, ()> {
        match s.to_case(Case::Flat).as_str() {
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "east" => Ok(Direction::East),
            "west" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

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

    pub fn badlands() -> World {
        let contents = fs::read_to_string("badlands.toml").unwrap();
        let data = contents.parse::<toml::Value>().unwrap();
        let mut w = World::new(data["meta"]["name"].to_string());
        let locales = data["locales"].as_table().unwrap();
        println!("{}", locales.len());
        for (_, data) in locales {
            let mut l = Locale::new(
                data["name"].as_str().unwrap(),
                data["description"].as_str().unwrap(),
            );
            for (dir, loc) in data["dirs"].as_table().unwrap() {
                l.add_adj(Direction::from_str(dir).unwrap(), loc.as_str().unwrap());
            }
            w.add_locale(l);
        }
        w.validate();
        w
    }
    
    fn validate(&self) {
        let keys = self.locales.iter().map(|(n, _)| n).collect::<Vec<&String>>();
        if !self.locales.iter()
            .all(|(_, l)| {
                l.adjacent.iter().all(|(_, next)| keys.contains(&next))
            })
        {
            panic!("Exists a location that doesn't exist");
        }
    }

    fn add_locale(&mut self, locale: Locale) {
        self.locales.insert(locale.name.clone(), locale);
    }

    pub fn print_current(&self, current: &str) {
        &self.locales.get(&current.to_string()).unwrap().print();
    }

    pub fn next_locale(&self, current: &str, dir: Direction) -> Option<&String> {
        let locale = &self.locales.get(&current.to_string()).unwrap();
        locale.adjacent.get(&dir)
    }
}
