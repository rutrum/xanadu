use std::str::FromStr;
use crate::world::Direction;

pub enum Command {
    Move(Direction),
    Take(String),
    Look,
    Inventory,
    Help,
}

impl FromStr for Command {
    type Err = &'static str; 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;
        let words = s.trim().split(' ').collect::<Vec<&str>>();
        match words.len() {
            1 => {
                match s.trim().to_lowercase().as_str() {
                    "h" | "help" => Ok(Help),
                    "i" | "inv" | "inventory" => Ok(Inventory),
                    "l" | "look" => Ok(Look),
                    _ => Err("That is not a valid command."),
                }
            }
            2 => {
                match words[0] {
                    "m" | "move" => {
                        match Direction::from_str(words[1]) {
                            Err(_) => Err("That's not a valid direction"),
                            Ok(dir) => Ok(Move(dir)),
                        }
                    }
                    "t" | "take" => {
                        Ok(Take(words[1].to_string()))
                    }
                    _ => Err("That is not a valid command."),
                }
            }
            _ => Err("Type something!"),
        }
    }
}
