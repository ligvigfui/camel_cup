use std::{io, mem::take};

use crate::*;

#[derive(Debug)]
pub struct CamelCup {
    pub current_player: usize,
    pub map_len: u8,
    pub dice_numbers: Vec<i8>,
    pub overall_tips_function: HashMap<Place, i8>,
    pub camels: Vec<Camel>,
    pub tip_cards: Vec<TipCard>,
    pub players: Vec<Player>,
    pub winer_overalltipcards: Vec<overallTipcard>,
    pub loser_overalltipcards: Vec<overallTipcard>,
}

impl CamelCup {
    pub fn new(options: &mut Options) -> Result<CamelCup, String> {
        Ok(CamelCup {
            current_player: 0,
            map_len: take(&mut options.map_len),
            dice_numbers: take(&mut options.dice_numbers),
            tip_cards: TipCard::new_vec(&options)?,
            players: Player::new_vec(options),
            camels: Camel::new_vec(&mut take(&mut options.camel_colors)),
            winer_overalltipcards: Vec::new(),
            loser_overalltipcards: Vec::new(),
            overall_tips_function: take(&mut options.overall_tips_function),
        })
    }
    pub fn a_3_player_new_game() -> CamelCup {
        CamelCup::a_n_player_game(3)
    }
    pub fn a_n_player_game(n: usize) -> CamelCup {
        let mut players = Vec::with_capacity(n);
        for _ in 0..n {
            players.push(None);
        }
        CamelCup::new(&mut Options::new(players)).unwrap()
    }
    pub fn current_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }
    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    pub(crate) fn map_len_out_of_bounds(&self, x: u8) -> Result<(), String> {
        match x {
            ix if ix <= 1 => Err("x must be at least 2".to_string()),
            ix if ix > self.map_len => Err(format!("x must be at most {}", self.map_len)),
            _ => Ok(())
        }
    }

    //______________________________________________________________________________________________________________________
    pub fn end_turn_check(&self) -> bool {
        let mut end_of_turn = true;
        for camel in self.camels.iter() {
            end_of_turn = end_of_turn && camel.moved;
        }
        end_of_turn
    }

    //______________________________________________________________________________________________________________________
    pub fn end_game_check(&self) -> bool {
        for camel in self.camels.iter() {
            if camel.x > self.map_len {
                return true;
            }
        }
        false
    }

    //______________________________________________________________________________________________________________________
    pub fn end_game_evaluate(&mut self, human: bool) {
        if human {println!("{:?}", self);}
        let mut winer_reward = vec![2, 3, 5, 8];
        let mut loser_reward = winer_reward.clone();
        for endgametipcard in self.winer_overalltipcards.iter() {
            if endgametipcard.color == self.camels[0].color {
                if human {println!("poping value from winer_reward {}", winer_reward[winer_reward.len()-1]);}
                self.players[endgametipcard.owner].money += match winer_reward.pop() {
                    Some(score) => score,
                    _ => 1,
                };
            } else {
                if human {println!("{} lost 1", self.players[endgametipcard.owner].name);}
                self.players[endgametipcard.owner].money -= 1;
            }
        }
        for endgametipcard in self.loser_overalltipcards.iter() {
            if endgametipcard.color == self.camels[self.camels.len()-1].color {
                if human {println!("poping value from loser_reward {}", loser_reward[loser_reward.len()-1]);}
                self.players[endgametipcard.owner].money += match loser_reward.pop() {
                    Some(score) => score,
                    _ => 1,
                };
            } else {
                if human {println!("{} lost 1", self.players[endgametipcard.owner].name);}
                self.players[endgametipcard.owner].money -= 1;
            }
        }
        if human {
            println!("Press enter to continue");
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    //_______________________________________________________________________________________
    pub fn evaluate_end_turn(&mut self) {
        for player in self.players.iter_mut() {
            let mut cards = player.evaluate_tip_cards(&self.camels);
            self.tip_cards.append(&mut cards);
        }
    }

    //_______________________________________________________________________________________
    pub fn display (&self) {
        //display current player
        println!("Current player: {}", ColoredString::new(&self.players[self.current_player].name).bold(true));
        //display camels
        //display map
        //display player cards
        println!("{}", self.display_camels());
        //display tip cards
        //display player money
        for player in self.players.iter() {
            println!("{}: {}", player.name, player.money);
            if player.tip_cards.len() > 0 {
                print!("{}'s cards: ", player.name);
                for card in player.tip_cards.iter() {
                    print!("{} {},  ", Color!(card.color).foreground(&card.color), card.values.get(&Place::Top(1)).unwrap());
                }
                println!();
            }
        }
        //display tip cards
        print!("{}", self.display_tip_cards());
        //display endgame info
        println!("{} cards bet on the winer camel", Color!(self.winer_overalltipcards.len()).bold(true));
        println!("{} cards bet on the losing camel", Color!(self.loser_overalltipcards.len()).bold(true));
    }

    //_______________________________________________________________________________________
    fn display_tip_cards(&self) -> String {
        let mut display = String::new();
        let mut cards: HashMap<&Color, Vec<String>> = HashMap::new();
        for card in self.tip_cards.iter() {
            if cards.contains_key(&card.color) {
                cards.get_mut(&card.color).unwrap().push(card.values.get(&Place::Top(1)).unwrap().to_string());
            } else {
                cards.insert(&card.color, vec![card.values.get(&Place::Top(1)).unwrap().to_string()]);
            }
        }
        for (&color, card_values) in cards.iter_mut() {
            card_values.sort_by(|a, b| b.cmp(a));
            display.push_colored(Color!("{: >8}: {}\n", color.to_string(), card_values.join("\t")).foreground(color));
        }
        display
    }

    //_______________________________________________________________________________________
    fn display_camels(&self) -> String {
        let mut display = String::new();
        for i in (0..=self.camels.len()-1).rev() {
            for (j, camel) in self.camels.iter().enumerate() {
                if !camel.moved && i == j{
                    display.push_colored(
                        Color!(&camel.color.to_string()[0..3])
                        .foreground(&camel.color)
                    );
                } else if i == j {
                    display.push_str("   ");
                }
            }
            for j in 1..self.map_len+1 {
                let mut found = false;
                for camel in self.camels.iter() {
                    if camel.x == j && camel.y == i as u8 {
                        display.push_colored(
                            Color!(&camel.color.to_string()[0..3])
                            .foreground(&camel.color)
                        );
                        found = true;
                    }
                }
                if !found {
                    display.push_str("   ");
                }
            }
            display.push_str("\n");
        }
        for i in 0..self.map_len+1 {
            display.push_str(&format!("{: ^3}", i));
        }
        display.push_str("  winner's on top\n");
        let mut player_found = false;
        for i in 1..self.map_len+1 {
            for (j, player) in self.players.iter().enumerate() {
                if player.placeable_card.x == i && j == self.current_player {
                    if player.placeable_card.faceup {
                        display.push_colored(Color!(" + ").bold(true))
                    } else {
                        display.push_colored(Color!(" - ").bold(true))
                    }
                    player_found = true;
                } else if player.placeable_card.x == i {
                    if player.placeable_card.faceup {
                        display.push_str(" + ");
                    } else {
                        display.push_str(" - ");
                    }
                    player_found = true;
                }
            }
            if !player_found {
                display.push_str("   ");
            }
            player_found = false;
        }
        display.push_str("\n");
        display
    }

    //_______________________________________________________________________________________
    pub fn human_turn(&mut self) -> Result<(), String> {
        println!("What do you want to do from the following options? (type the number)");
        let options = vec!["place your card", "move camel", "bet on camel", "bet on endgame"];
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i, option);
        }
        let input = read_usize(3)?;
        match input {
            0 => self.human_place_card()?,
            1 => self.rand_move_camel()?,
            2 => self.human_bet_on_camel()?,
            3 => self.human_bet_on_endgame()?,
            _ => return Err("Please type a number (0-3)".to_string()),
        }
        Ok(())
    }

    pub(crate) fn not_moved_camels(&self) -> Vec<Color> {
        let mut not_moved_camels = Vec::new();
        for camel in self.camels.iter() {
            if !camel.moved {
                not_moved_camels.push(camel.color.clone());
            }
        }
        not_moved_camels
    }

    //_______________________________________________________________________________________
    fn human_place_card(&mut self) -> Result<(), String> {
        println!("Where do you want to place your card? (type the number)");
        let input = read_usize(self.map_len as usize)?;
        println!("Do you want to place your card faceup or facedown? (type the number)");
        let options = vec!["faceup", "facedown"];
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i, option);
        }
        let faceup = read_usize(options.len() - 1)?;
        match self.place_card(input as u8, faceup == 0) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    //_______________________________________________________________________________________
    fn human_bet_on_camel(&mut self) -> Result<(), String> {
        clear_screen();
        println!("{}", self.display_camels());
        println!("Which camel do you want to bet on? (type the number)");
        let mut color_value = HashMap::new();
        for tip_card in self.tip_cards.iter() {
            if let Some(&value) = color_value.get(&tip_card.color) {
                if value >= tip_card.values.get(&Place::Top(1)).unwrap() {
                    continue;
                }
            }
            color_value.insert(&tip_card.color, tip_card.values.get(&Place::Top(1)).unwrap());
        }
        let mut smoth_count: Vec<(&Color, i8)> = vec![];
        for (&ref color, &value) in color_value.into_iter() {
            smoth_count.push((&color, value));
        }
        for (i, (color, value)) in smoth_count.iter().enumerate() {
            println!(
                "{}: {} \t{}",
                i,
                Color!(color).foreground(&color),
                value
            );
        }
        let input = read_usize(smoth_count.len() - 1)?;
        let color = smoth_count[input].0;
        self.move_tip_card(&color.clone())?;
        Ok(())
    }

    //_______________________________________________________________________________________
    fn human_bet_on_endgame(&mut self) -> Result<(), String> {
        println!("Do you want to bet on the winer camel or the loser camel? (type the number)");
        let options = vec!["winer", "loser"];
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i, option);
        }
        let input = read_usize(options.len() - 1)?;
        clear_screen();
        println!("{}", self.display_camels());
        if input == 0 {println!("Currently {} cards bet on the winner camel", self.winer_overalltipcards.len());}
        else {println!("Currently {} cards bet on the loser camel", self.loser_overalltipcards.len());}
        println!("Which camel do you want to bet on? (type the number)");
        for (i, overall_tip_card) in self.current_player().overall_tip_cards.iter().enumerate() {
            println!("{}: {}", i, Color!(overall_tip_card.color).foreground(&overall_tip_card.color));
        }
        let input2 = read_usize(self.current_player().overall_tip_cards.len() - 1)?;
        let color = self.current_player().overall_tip_cards[input2].color.clone();
        self.end_game_bet(
            input == 0,
            &color
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = CamelCup::a_3_player_new_game();
        assert_eq!(game.map_len, 16);
        assert_eq!(game.camels.len(), 5);
        assert_eq!(game.tip_cards.len(), 15);
        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[0].name, "Player 0");
        assert_eq!(game.players[0].money, 3);
        assert_eq!(game.players[0].placeable_card.x, 0);
        assert_eq!(game.players[0].placeable_card.faceup, false);
        assert_eq!(game.players[0].tip_cards.len(), 0);
        assert_eq!(game.winer_overalltipcards.len(), 0);
        assert_eq!(game.current_player, 0);
        assert_eq!(game.loser_overalltipcards.len(), 0);
    }

    #[test]
    fn test_next_player() {
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.current_player, 0);
        game.next_player();
        assert_eq!(game.current_player, 1);
        game.next_player();
        assert_eq!(game.current_player, 2);
        game.next_player();
        assert_eq!(game.current_player, 0);
    }

    #[test]
    fn test_end_turn_check(){
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.end_turn_check(), false);
        for camel in game.camels.iter_mut() {
            camel.moved = true;
        }
        assert_eq!(game.end_turn_check(), true);
    }

    #[test]
    fn test_order_camels(){
        let mut game = CamelCup::a_3_player_new_game();
        for i in 0..game.camels.len() {
            game.camels[i].moved = true;
        }
        game.set_camel_position(Color::White, 1, 1);
        game.set_camel_position(Color::RGB(Some("Orange".to_string()), 255, 165, 0), 2, 1);
        game.set_camel_position(Color::Blue, 3, 1);
        game.set_camel_position(Color::Green, 4,1);
        game.set_camel_position(Color::Yellow, 5, 1);
        game.order_camels();
        assert_eq!(game.camels[0].color, Color::Yellow);
        assert_eq!(game.camels[1].color, Color::Green);
        assert_eq!(game.camels[4].color, Color::White);
        game.set_camel_position(Color::White, 1, 1);
        game.set_camel_position(Color::RGB(Some("Orange".to_string()), 255, 165, 0), 1, 2);
        game.set_camel_position(Color::Blue, 1, 3);
        game.set_camel_position(Color::Green, 1,4);
        game.set_camel_position(Color::Yellow, 1, 5);
        game.order_camels();
        assert_eq!(game.camels[0].color, Color::Yellow);
        assert_eq!(game.camels[1].color, Color::Green);
        assert_eq!(game.camels[4].color, Color::White);
        game.set_camel_position(Color::White, 1, 1);
        game.set_camel_position(Color::RGB(Some("Orange".to_string()), 255, 165, 0), 3, 1);
        game.set_camel_position(Color::Blue, 6, 1);
        game.set_camel_position(Color::Green, 1,2);
        game.set_camel_position(Color::Yellow, 5, 1);
        game.order_camels();
        assert_eq!(game.camels[0].color, Color::Blue);
        assert_eq!(game.camels[1].color, Color::Yellow);
        assert_eq!(game.camels[4].color, Color::White);
    }

    
    #[test]
    fn test_end_game_evaluate(){
        let mut game = CamelCup::a_n_player_game(6);
        game.end_game_bet(true, &Color::White).unwrap();
        game.end_game_bet(false, &Color::White).unwrap();
        game.end_game_bet(true, &Color::White).unwrap();
        assert_eq!(game.players[0].money, 3);
        assert_eq!(game.players[1].money, 3);
        assert_eq!(game.players[2].money, 3);
        assert_eq!(game.winer_overalltipcards.len(), 2);
        assert_eq!(game.loser_overalltipcards.len(), 1);
        game.end_game_bet(false, &Color::RGB(Some("Orange".to_string()), 255, 165, 0)).unwrap();
        game.end_game_bet(false, &Color::RGB(Some("Orange".to_string()), 255, 165, 0)).unwrap();
        game.end_game_bet(true, &Color::RGB(Some("Orange".to_string()), 255, 165, 0)).unwrap();
        game.camels[0].x = 17;
        game.camels[1].x = 1;
        game.camels[2].x = 1;
        game.camels[3].x = 1;
        game.camels[4].x = 0;
        game.camels[2].y = 1;
        game.camels[3].y = 1;
        game.camels[0].moved = true;
        game.end_game_evaluate(false);
        assert_eq!(game.players[0].money, 11);
        assert_eq!(game.players[1].money, 2);
        assert_eq!(game.players[2].money, 8);
        assert_eq!(game.players[3].money, 11);
        assert_eq!(game.players[4].money, 8);
        assert_eq!(game.players[5].money, 2);
    }

    #[test]
    fn evaluate_end_turn(){
        let mut game = CamelCup::a_n_player_game(5);
        game.move_tip_card(&Color::White).unwrap();
        game.move_tip_card(&Color::White).unwrap();
        game.move_tip_card(&Color::White).unwrap();
        assert_eq!(game.move_tip_card(&Color::White), Err("No more cards of this color left"));
        game.move_tip_card(&Color::Green).unwrap();
        game.move_tip_card(&Color::Blue).unwrap();
        game.move_camel(Color::White, 3).unwrap();
        game.move_camel(Color::Green, 2).unwrap();
        game.move_camel(Color::Blue, 1).unwrap();
        game.camels[3].moved = true;
        game.camels[4].moved = true;
        game.evaluate_end_turn();
        assert_eq!(game.players[0].money, 9);
        assert_eq!(game.players[1].money, 7);
        assert_eq!(game.players[2].money, 6);
        assert_eq!(game.players[3].money, 4);
        assert_eq!(game.players[4].money, 2);
    }

    #[test]
    fn display_camels() {
        let mut game = CamelCup::a_3_player_new_game();
        let string = 
        "\u{1b}[38;2;255;165;0mora\u{1b}[0m                                                \n".to_owned() +
        "\u{1b}[33myel\u{1b}[0m                                                \n" +
        "\u{1b}[32mgre\u{1b}[0m                                                \n" +
        "\u{1b}[34mblu\u{1b}[0m                                                \n" +
        "\u{1b}[1mwhi\u{1b}[0m                                                \n" +
        "_0__1__2__3__4__5__6__7__8__9__10_11_12_13_14_15_16__winner's_on_top__\n" +
        "                                                   \n";
        assert_eq!(game.display_camels(), string);
        game.move_camel(Color::White, 1).unwrap();
        println!("{:?}", game.camels);
        println!("{}", game.display_camels());
        let string = 
        "\u{1b}[38;2;255;165;0mora\u{1b}[0m                                                \n".to_owned() +
        "\u{1b}[33myel\u{1b}[0m                                                \n" +
        "\u{1b}[32mgre\u{1b}[0m                                                \n" +
        "\u{1b}[34mblu\u{1b}[0m                                                \n" +
        "   \u{1b}[1mwhi\u{1b}[0m                                             \n" +
        "_0__1__2__3__4__5__6__7__8__9__10_11_12_13_14_15_16__winner's_on_top__\n" +
        "                                                   \n";
        assert_eq!(game.display_camels(), string);
        game.move_camel(Color::Blue, 1).unwrap();
        println!("{:?}", game.camels);
        println!("{}", game.display_camels());
        assert_eq!(game.camels[0].x, 1);
        assert_eq!(game.camels[0].y, 1);
        let string = "\u{1b}[38;2;255;165;0mora\u{1b}[0m                                                \n".to_owned() +
        "\u{1b}[33myel\u{1b}[0m                                                \n" +
        "\u{1b}[32mgre\u{1b}[0m                                                \n" +
        "   \u{1b}[34mblu\u{1b}[0m                                             \n" +
        "   \u{1b}[1mwhi\u{1b}[0m                                             \n" +
        "_0__1__2__3__4__5__6__7__8__9__10_11_12_13_14_15_16__winner's_on_top__\n" +
        "                                                   \n";
        assert_eq!(game.display_camels(), string);
        game.move_camel(Color::Green, 3).unwrap();
        println!("{}", game.display_camels());
        let string = "\u{1b}[38;2;255;165;0mora\u{1b}[0m                                                \n".to_owned() +
        "\u{1b}[33myel\u{1b}[0m                                                \n" +
        "                                                   \n" +
        "   \u{1b}[34mblu\u{1b}[0m                                             \n" +
        "   \u{1b}[1mwhi\u{1b}[0m   \u{1b}[32mgre\u{1b}[0m                                       \n" +
        "_0__1__2__3__4__5__6__7__8__9__10_11_12_13_14_15_16__winner's_on_top__\n" +
        "                                                   \n";
        assert_eq!(game.display_camels(), string);
        game.move_camel(Color::RGB(Some("Orange".to_string()), 255, 165, 0), 2).unwrap();
        println!("{}", game.display_camels());
        let string = "\u{1b}[33myel\u{1b}[0m                                                \n".to_owned() +
        "                                                   \n" +
        "                                                   \n" +
        "   \u{1b}[34mblu\u{1b}[0m                                             \n" +
        "   \u{1b}[1mwhi\u{1b}[0m\u{1b}[38;2;255;165;0mora\u{1b}[0m\u{1b}[32mgre\u{1b}[0m                                       \n" +
        "_0__1__2__3__4__5__6__7__8__9__10_11_12_13_14_15_16__winner's_on_top__\n" +
        "                                                   \n";
        assert_eq!(game.display_camels(), string);
    }

    #[test]
    fn map_len_out_of_bounds() {
        let game = CamelCup::a_3_player_new_game();
        assert!(CamelCup::map_len_out_of_bounds(&game, 1).is_err());
        assert!(CamelCup::map_len_out_of_bounds(&game, 20).is_err());
        assert!(CamelCup::map_len_out_of_bounds(&game, 2).is_ok())
    }
}
