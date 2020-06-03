use xanadu::world::{World, Direction};
use xanadu::commands::Command;
use std::str::FromStr;

fn main() {
    println!("*** Welcome to Xanadu ***");
    let badlands = World::badlands();
    let mut current = "Forest";

    loop {
        badlands.print_current(current);
        println!("Where would you like to go?");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!();
        match Command::from_str(&buffer) {
            Err(()) => (),
            Ok(command) => match command {
                Command::Look => println!("There's nothing to see!"),
                Command::Help => println!("Open your eyes"),
                Command::Inventory => println!("Unimplemented"),
                Command::Move(dir) => match badlands.next_locale(current, dir) {
                    Some(a) => current = a,
                    None => println!("Can't go that way!"),
                },
            }
        }
        continue;
        let maybe = Direction::from_str(&buffer);
        match maybe {
            Ok(dir) => match badlands.next_locale(current, dir) {
                Some(a) => current = a,
                None => println!("Can't go that way!"),
            },
            Err(()) => println!("That's not a direction!"),
        }
    }
}
