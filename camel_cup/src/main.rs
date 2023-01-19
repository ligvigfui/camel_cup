use std::{error::Error, io};

use camel_cup::{CamelCup, Player};




fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to the Camel Up game!");
    //ask the user how many players are playing?
    let mut player_number = String::new();
    loop {
        println!("How many players are playing? (2-8)");
        player_number.clear();
        io::stdin().read_line(&mut player_number).expect("Failed to read line");
        let player_number: i32 = match player_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue
            },
        };
        if player_number > 1 && player_number < 9 {
            break;
        }
        println!("Number is outside of range");
    }
    let player_number = player_number.trim().parse().unwrap();
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
            names.push(Player::new(Some(name), i));
        }
    } else {
        for i in 0..player_number {
            println!("debud: players are setting up without names {i}");
            names.push(Player::new(None, i));
        }
    }
    print!("\x1b[2J");
    let mut game = CamelCup::new(names);
    let mut running = true;
    while running {
        loop {
            game.display();
            match game.turn() {
                Ok(_) => {
                    print!("\x1b[2J");
                    break;
                },
                Err(e) => {
                    print!("\x1b[2J"); println!("Error: {}", e);
                    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
                    continue;
                },
            }
        }
        if game.end_turn_check() {
            game.evaluate_end_turn();
            game.reset_turn();
        }
        if game.end_game_check() {
            game.evaluate_end_turn();
            game.end_game_evaluate(true);
            running = false;
        }
        game.next_player();
    }
    game.end_game_players_display();
    Ok(())
}