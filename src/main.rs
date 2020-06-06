use std::str::FromStr;
use xanadu::commands::Command;
use xanadu::world::WorldBuilder;

fn main() {
    println!("*** Welcome to Xanadu ***");
    let mut badlands = WorldBuilder::from_file("badlands.json").build();
    let mut current = "Forest".to_string();

    loop {
        badlands.print_current(&current);
        println!("What would you like to do?");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!();
        match Command::from_str(&buffer) {
            Err(s) => println!("{}", s),
            Ok(command) => match command {
                Command::Look => println!("There's nothing to see!"),
                Command::Help => println!("Open your eyes"),
                Command::Inventory => println!("Unimplemented"),
                Command::Move(dir) => match badlands.next_locale(&current, dir) {
                    Some(a) => current = a,
                    None => println!("Can't go that way!"),
                },
                Command::Take(item) => match badlands.take_item(&current, &item) {
                    Some(i) => println!("You picked up the {}.", i.name),
                    None => println!("That's not something you can take."),
                },
            },
        }
    }
}
