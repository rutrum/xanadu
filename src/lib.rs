pub mod commands;
pub mod item;
pub mod player;
pub mod world;

use convert_case::{Case, Casing};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_case(Case::Flat).as_str() {
            "n" | "north" => Ok(Direction::North),
            "s" | "south" => Ok(Direction::South),
            "e" | "east" => Ok(Direction::East),
            "w" | "west" => Ok(Direction::West),
            "u" | "up" => Ok(Direction::Up),
            "d" | "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum MovementError {
    NoExist,
}

impl Error for MovementError {}

impl fmt::Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MovementError::*;
        match self {
            NoExist => write!(f, "You can't go that way."),
        }
    }
}
