use std::{error::Error, io};

use camel_cup::{clear_screen, read_usize, CamelCup, Options};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to the Camel Up game!");
    //ask the user how many players are playing?
    let player_number;
    loop {
        println!("How many players are playing? (2-8)");
        match read_usize(8) {
            Ok(num) => {
                if num < 2 {
                    println!("This is a multiplayer game. Please type at least 2");
                    continue;
                }
                player_number = num;
                break;
            },
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
    }
    println!("Would you like to add a name to the players? (y/n)");
    let mut add_names = String::new();
    io::stdin().read_line(&mut add_names).expect("Failed to read line");
    add_names = add_names.trim().to_string();
    let mut names = Vec::new();
    if add_names == "y" {
        println!("Please type the names of the players one by one");
        for i in 0..player_number {
            println!("Player{i}'s name going to be: ");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed to read line");
            name = name.trim().to_string();
            names.push(Some(name));
        }
    } else {
        for i in 0..player_number {
            println!("debug: players are setting up without names {i}");
            names.push(None);
        }
    }
    clear_screen();
    let mut options = Options::new(names);
    let mut game = CamelCup::new(&mut options)?;
    let mut running = true;
    while running {
        loop {
            game.display();
            match game.human_turn() {
                Ok(_) => {
                    clear_screen();
                    break;
                },
                Err(e) => {
                    clear_screen();
                    println!("Error: {}", e);
                    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
                    continue;
                },
            }
        }
        if game.end_turn_check() {
            game.evaluate_end_turn();
        }
        if game.end_game_check() {
            game.evaluate_end_turn();
            game.end_game_evaluate(true);
            running = false;
        }
    }
    game.end_game_players_display();
    Ok(())
}