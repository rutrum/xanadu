pub mod commands;
pub mod player;
pub mod world;
pub mod item;

use convert_case::{Case, Casing};
use std::str::FromStr;

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
