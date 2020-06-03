use std::str::FromStr;
use crate::world::Direction;

pub enum Command {
    Move(Direction),
    Look,
    Inventory,
    Help,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;
        let words = s.trim().split(" ").collect::<Vec<&str>>();
        match words.len() {
            1 => {
                match s.trim().to_lowercase().as_str() {
                    "help" => Ok(Help),
                    "inv" | "inventory" => Ok(Inventory),
                    "look" => Ok(Look),
                    _ => Err(()),
                }
            }
            2 => {
                match words[0] {
                    "move" => {
                        match Direction::from_str(words[1]) {
                            Err(_) => Err(()),
                            Ok(dir) => Ok(Move(dir)),
                        }
                    }
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}
