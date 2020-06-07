use std::str::FromStr;
use std::io::{stdout, Write};
use xanadu::commands::Command;
use xanadu::world::WorldBuilder;
use xanadu::player::Player;

fn main() {
    println!("X A N A D U\n");
    let mut badlands = WorldBuilder::from_file("badlands.json").build();
    let mut current = "Forest".to_string();
    let mut player = Player::new();
    let mut just_moved = true;

    loop {
        if just_moved {
            badlands.print_current(&current);
            just_moved = false;
        }
        print!("\n> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        match Command::from_str(&buffer) {
            Err(s) => println!("{}", s),
            Ok(command) => match command {
                Command::Look => badlands.print_current(&current),
                Command::Help => println!("Open your eyes"),
                Command::Inventory => {
                    player.print_inv();
                }
                Command::Move(dir) => match badlands.next_locale(&current, dir) {
                    Ok(key) => {
                        current = key;
                        just_moved = true;
                    }
                    Err(e) => println!("{}", e),
                },
                Command::Examine(item_str) => match player.item_description(&item_str) {
                    Ok(desc) => println!("{}", desc),
                    Err(e) => println!("{}", e),
                }
                Command::Take(item) => match badlands.take_item(&current, &item) {
                    Ok(i) => {
                        println!("You picked up the {}.", i.name);
                        player.add_item(i);
                    }
                    Err(e) => println!("{}", e),
                },
                Command::Read(item) => match player.read_item(&item) {
                    Ok(msg) => {
                        println!("{}", msg);
                    }
                    Err(e) => println!("{}", e),
                }
            },
        }
    }
}
