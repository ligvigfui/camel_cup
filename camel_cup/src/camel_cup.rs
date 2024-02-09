use rand::Rng;
use std::io;

use crate::*;

#[derive(Debug)]
pub struct CamelCup {
    current_player: usize,
    options: Options,
    camels: Vec<Camel>,
    tip_cards: Vec<TipCard>,
    players: Vec<Player>,
    winer_oweralltipcards: Vec<OwerallTipcard>,
    loser_oweralltipcards: Vec<OwerallTipcard>,
    common_oweralltipcards: Vec<OwerallTipcard>,
}

impl CamelCup {
    pub fn new(options: Options) -> Result<CamelCup, String> {
        Ok(CamelCup {
            current_player: 0,
            options,
            camels: Camel::new_vec(&options.camel_colors),
            tip_cards: TipCard::new_vec(&options.camel_colors, options.leg_tips)?,
            players: Player::new_vec(options.player_names),
            winer_oweralltipcards: Vec::new(),
            loser_oweralltipcards: Vec::new(),
            common_oweralltipcards: OwerallTipcard::new_vec(options.camel_colors, options.player_names.len()),
        })
    }
    pub fn a_3_player_new_game() -> CamelCup {
        CamelCup::a_n_player_game(3)
    }
    pub fn a_n_player_game(n: usize) -> CamelCup {
        let mut players = Vec::with_capacity(n);
        for i in 0..n {
            players.push(None);
        }
        CamelCup::new(Options::new(players)).unwrap()
    }
    pub fn move_tip_card(&mut self, player_number: usize, tip_card_color: Color) -> Result<(), &'static str> {
        let mut max_amount = 0;
        for card in self.tip_cards.iter_mut() {
            if card.color == tip_card_color {
                max_amount = if max_amount < *card.value.get(&Place::Top(1)).unwrap() {*card.value.get(&Place::Top(1)).unwrap()} else  {max_amount};
            }
        };
        match max_amount {
            0 => Err("No more cards of this color left"),
            _ => { 
                todo!();
                Ok(())
            }
        }
    }
    
    pub fn place_card(&mut self, player_number: usize, x: u8, faceup: bool) -> Result<(), &'static str> {
        if x < 1 || x > self.options.map_len {
            return Err("x is out of bounds");
        }
        if player_number >= self.players.len() {
            return Err("player_number is out of bounds");
        }
        for (i, player) in self.players.iter().enumerate() {
            if player.placeable_card.x == x && self.players[player_number].placeable_card.x != x{
                return Err("x is already occupied by some other player");
            }
            else if player.placeable_card.x == x+1 && self.players[player_number].placeable_card.x != x+1{
                return Err("x nearby is already occupied by some other player");
            }
            else if player.placeable_card.x == x-1 && self.players[player_number].placeable_card.x != x-1 && x-1 != 0{
                return Err("x nearby is already occupied by some other player");
            }
            if player_number == i && player.placeable_card.x == x && player.placeable_card.faceup == faceup {
                return Err("You can't place Your card the same");
            }
        }
        for camel in self.camels.iter() {
            if camel.x == x {
                return Err("x is already occupied by some camel(s)");
            }
        }
        self.players[player_number].placeable_card.x = x;
        self.players[player_number].placeable_card.faceup = faceup;
        Ok(())
    }
    pub fn current_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }
    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
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
            if camel.x > self.options.map_len {
                return true;
            }
        }
        false
    }

    //______________________________________________________________________________________________________________________
    pub fn end_game_bet(&mut self, player_number: usize, winer: bool, color: Color) -> Result<(), &'static str> {
        if player_number >= self.players.len() {
            return Err("player_number is out of bounds");
        }
        for (i, endgametipcard) in self.common_oweralltipcards.iter().enumerate() {
            if endgametipcard.color == color && endgametipcard.owner == player_number {
                if winer {
                    self.winer_oweralltipcards.push(self.common_oweralltipcards.remove(i));
                    return Ok(());
                } else {
                    self.loser_oweralltipcards.push(self.common_oweralltipcards.remove(i));
                    return Ok(());
                }
            }
        }
        Err("You already bet on this color")
    }

    //______________________________________________________________________________________________________________________
    pub fn end_game_evaluate(&mut self, human: bool) {
        if human {println!("{:?}", self);}
        let mut winer_reward = vec![2, 3, 5, 8];
        let mut loser_reward = winer_reward.clone();
        for endgametipcard in self.winer_oweralltipcards.iter() {
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
        for endgametipcard in self.loser_oweralltipcards.iter() {
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

    //______________________________________________________________________________________________________________________
    fn order_camels(&mut self){
        let e:u8 = self.camels.len().try_into().unwrap();
        self.camels.sort_by(|a, b|{ 
            let c = a.x*e+a.y; 
            let d:u8 = b.x*e+b.y;
            d.cmp(&c)
        });
    }

    
    //______________________________________________________________________________________________________________________
    fn set_camel_position(&mut self, camel_color: Color, x: u8, y: u8) {
        //! test fn only
        /*self.camels.iter().for_each(|(camel)| {
            /*if camel.x == x && camel.y == y {
                return Err("x and y are already occupied by some other camel");
            }*/
        });*/
        for camel in self.camels.iter_mut() {
            if camel.color == camel_color {
                camel.x = x;
                camel.y = y;
                camel.moved = true;
                return;
            }
        }
        panic!("No camel with this color");
    }

    //______________________________________________________________________________________________________________________
    pub fn move_camel(&mut self, camel_color: Color, amount: i8) -> Result<(), &'static str> {
        if amount <= 0 {
            return Err("amount must be positive");
        }
        if amount > self.options.dice_max_number {
            return Err("amount is too big");
        }
        if !self.camels.iter().any(|camel| camel.color == camel_color) {
            return Err("No camel with this color");
        }
        let mut x = 0;
        let mut y = 0;
        let mut camels_above = Vec::new();
        let mut amount = amount;
        for (camel_number, camel) in self.camels.iter_mut().enumerate() {
            if camel.color == camel_color {
                //println!("{} {} {} {}", camel.x, camel.y, camel.moved, camel.color);
                if camel.moved {
                    return Err("camel already moved this turn");
                }
                if camel.x == 0 {
                    camels_above.push(camel_number);
                }
                x = camel.x;
                y = camel.y;
                camel.moved = true;
                
            }
        }
        // x 0 y 0
        for (camel_number, camel) in self.camels.iter().enumerate() {
            if camel.x == x && x !=0 && camel.y >= y {
                camels_above.push(camel_number);
            }
        }
        //println!("{} {} {:?}", x, y, camels_above);
        let mut case1 = true;
        for player in self.players.iter_mut() {
            if player.placeable_card.x == (x as i16 + amount as i16) as u8 {
                player.money += 1;
                if player.placeable_card.faceup {
                    case1 = true;
                    amount += 1;
                } else {
                    case1 = false;
                    amount -= 1;
                }
            }
        }
        self.players[self.current_player].money += 1;
        let mut camels_below = 0;
        if case1 {
            //camels are moved above
            for camel in self.camels.iter_mut() {
                if camel.x == (x as i16 + amount as i16) as u8 {
                    camels_below += 1;
                }
            }
            for camel in camels_above.iter() {
                self.camels[*camel].x += amount as u8; // todo check for potential overflow
                self.camels[*camel].y += camels_below - y;
            }
        } else {
            //camels are moved below 
            //elevates the moving camels
            for (camel_number, camel) in self.camels.iter_mut().enumerate() {
                if camel.x == (x as i16 + amount as i16) as u8 && !camels_above.contains(&camel_number){
                    camel.y += camels_above.len() as u8;
                }
            }
            //moves the moving camels below
            for camel in camels_above.iter() {
                //println!("{} {} {}", camel, self.camels[*camel].x, self.camels[*camel].y);
                self.camels[*camel].x += amount as u8; // todo check for potential overflow
                self.camels[*camel].y -= y;
                //println!("{} {} {}", camel, self.camels[*camel].x, self.camels[*camel].y);
            }
        }
        self.order_camels();
        Ok(())
    }

    //_______________________________________________________________________________________
    fn camel_test_helper(&self, color: Color, assert_x: u8, assert_y: u8, assert_moved: bool) {
        let camel = self.camels.iter().find(|camel| camel.color == color).unwrap();
        assert_eq!(assert_x, camel.x);
        assert_eq!(assert_y, camel.y);
        assert_eq!(assert_moved, camel.moved);
    }

    //_______________________________________________________________________________________
    pub fn evaluate_end_turn(&mut self) {
        for player in self.players.iter_mut() {
            for card in player.tip_cards.iter_mut() {
                let amount = card.evaluate(&self.camels);
                player.modify_money(amount)
            }
        }
    }

    //_______________________________________________________________________________________
    pub fn reset_turn(&mut self) {
        for camel in self.camels.iter_mut() {
            camel.moved = false;
        }
        for player in self.players.iter_mut() {
            player.tip_cards.clear();
        }
        self.tip_cards = TipCard::new_vec(&self.options.camel_colors, self.options.leg_tips).unwrap()
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
                    print!("{} {},  ", Color!(card.color).foreground(card.color), card.value.get(&Place::Top(1)).unwrap());
                }
                println!();
            }
        }
        //display tip cards
        print!("{}", self.display_tip_cards());
        //display endgame info
        println!("{} cards bet on the winer camel", Color!(self.winer_oweralltipcards.len()).bold(true));
        println!("{} cards bet on the losing camel", Color!(self.loser_oweralltipcards.len()).bold(true));
    }


    //_______________________________________________________________________________________
    fn display_tip_cards(&self) -> String {
        let mut display = String::new();
        for cards in self.tip_cards.iter() {
            if cards.len() != 0 {
                display.push_colored(Color!(format!("{}\t", cards[0].color.to_string())).foreground(cards[0].color));
            } else {
                continue;
            }
            let mut card_values = String::new();
            for (i, card) in cards.iter().enumerate() {
                card_values.push_str(&card.value.get(&Place::Top(1)).unwrap().to_string()[..]);
                if i != cards.len() {
                    card_values.push_str(",\t");
                }
            }
            display.push_colored(Color!(card_values).bold(true))
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
                        .foreground(camel.color)
                    );
                } else if i == j {
                    display.push_str("   ");
                }
            }
            for j in 1..self.options.map_len+1 {
                let mut found = false;
                for camel in self.camels.iter() {
                    if camel.x == j && camel.y == i as u8 {
                        display.push_colored(
                            Color!(&camel.color.to_string()[0..3])
                            .foreground(camel.color)
                        );
                        found = true;
                    }
                }
                if !found {
                    let _ = &display.push_str("   ");
                }
            }
            let _ = &display.push_str("\n");
        }
        for i in 0..self.options.map_len+1 {
            if i > 9 {
                let _ = &display.push_str(&format!("_{}", i));
            } else {
                let _ = &display.push_str(&format!("_{}_", i));
            }
        }
        let _ = &display.push_str("__winner's_on_top__\n   ");
        let mut player_found = false;
        for i in 1..self.options.map_len+1 {
            for (j, player) in self.players.iter().enumerate() {
                if player.placeable_card.x == i && j == self.current_player{
                    let _ = &display.push_str("\x1b[31m");
                    if player.placeable_card.faceup {
                        let _ = &display.push_str(" + ");
                    } else {
                        let _ = &display.push_str(" - ");
                    }
                    let _ = &display.push_str("\x1b[0m");
                    player_found = true;
                } else if player.placeable_card.x == i {
                    if player.placeable_card.faceup {
                        let _ = &display.push_str(" + ");
                    } else {
                        let _ = &display.push_str(" - ");
                    }
                    player_found = true;
                }
            }
            if !player_found {
                let _ = &display.push_str("   ");
            }
            player_found = false;
        }
        let _ = &display.push_str("\n");
        display
    }

    //_______________________________________________________________________________________
    pub fn turn(&mut self) -> Result<(), &'static str> {
        println!("What do you want to do from the following options? (type the number)");
        let options = vec!["place your card", "move camel", "bet on camel", "bet on endgame"];
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i, option);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number (0-3)"),
        };
        match input {
            0 => self.human_place_card()?,
            1 => self.rand_move_camel()?,
            2 => self.human_bet_on_camel()?,
            3 => self.human_bet_on_endgame()?,
            _ => return Err("Please type a number (0-3)"),
        }
        Ok(())
    }

    //_______________________________________________________________________________________
    pub fn rand_move_camel(&mut self) -> Result<(), &'static str>{
        let random =rand::thread_rng().gen_range(0..self.not_moved_camels().len());
        let color = self.not_moved_camels()[random];
        let amount = rand::thread_rng().gen_range(1..4);
        self.move_camel(color, amount)
    }

    //_______________________________________________________________________________________
    fn not_moved_camels(&self) -> Vec<Color> {
        let mut not_moved_camels = Vec::new();
        for camel in self.camels.iter() {
            if !camel.moved {
                not_moved_camels.push(camel.color);
            }
        }
        not_moved_camels
    }

    //_______________________________________________________________________________________
    fn human_place_card(&mut self) -> Result<(), &'static str> {
        println!("Where do you want to place your card? (type the number)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number"),
        };
        if input > self.options.map_len {
            return Err("Please type a number (1-16)");
        }
        println!("Do you want to place your card faceup or facedown? (type the number)");
        let options = vec!["faceup", "facedown"];
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i, option);
        }
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        let input2: usize = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number (0-1)"),
        };
        if input2 > 1 {
            return Err("Please type a number (0-1)");
        }
        self.place_card(self.current_player, input, input2 == 0)?;
        Ok(())
    }

    //_______________________________________________________________________________________
    fn human_bet_on_camel(&mut self) -> Result<(), &'static str> {
        clear_screen();
        println!("{}", self.display_camels());
        println!("Which camel do you want to bet on? (type the number)");
        let mut smoth_count = Vec::new();
        for (i, tip_cards) in self.tip_cards.iter().enumerate() {
            if tip_cards.len() > 0 {
                smoth_count.push(i);
            } else {
                continue;
            }
        }
        for (i, count) in smoth_count.iter().enumerate() {
            println!(
                "{}: {} \t{}",
                i,
                Color!(self.tip_cards[*count][0].color.to_string())
                    .foreground(self.tip_cards[*count][0].color),
                self.tip_cards[*count][self.tip_cards[*count].len() - 1].value.get(&Place::Top(1)).unwrap()
            );
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number (0-displayed max)"),
        };
        if input > smoth_count.len()-1 {
            return Err("Please type a number (0-displayed max)");
        }
        self.move_tip_card(
            self.current_player,
            self.tip_cards[smoth_count[input]][0].color
        )
    }

    //_______________________________________________________________________________________
    fn human_bet_on_endgame(&mut self) -> Result<(), &'static str> {
        println!("Do you want to bet on the winer camel or the loser camel? (type the number)");
        let options = vec!["winer", "loser"];
        for (i, option) in options.iter().enumerate() {
            println!("{}: {}", i, option);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number (0-1)"),
        };
        if input > 1 {
            return Err("Invalid input");
        }
        clear_screen();
        println!("{}", self.display_camels());
        if input == 0 {println!("Currently {} cards bet on the winner camel", self.winer_oweralltipcards.len());}
        else {println!("Currently {} cards bet on the loser camel", self.loser_oweralltipcards.len());}
        println!("Which camel do you want to bet on? (type the number)");
        let mut endgametipcards = Vec::new();
        for (i, endgametipcard) in self.common_oweralltipcards.iter().enumerate() {
            if endgametipcard.owner == self.current_player {
                endgametipcards.push(i);
            }
        }
        for (i, common_i) in endgametipcards.iter().enumerate() {
            println!("{}: {}", i, Color!(self.common_oweralltipcards[*common_i].color).foreground(self.common_oweralltipcards[*common_i].color));
        }
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        let input2: usize = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number (0-displayed max)"), 
        };
        if input2 > endgametipcards.len()-1 {
            return Err("Please type a number (0-displayed max)");
        }
        self.end_game_bet(
            self.current_player, 
            input == 0, 
            self.common_oweralltipcards[endgametipcards[input2]].color
        )?;
        Ok(())
    }

    //_______________________________________________________________________________________
    pub fn end_game_players_display(&mut self) {
        self.players.sort_by(|a, b| b.money.cmp(&a.money));
        for (i, player) in self.players.iter().enumerate() {
            println!("{}: Player {} has {} money", i, player.name, player.money);
        }
        println!("Press enter to exit");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    }

    //_______________________________________________________________________________________
    pub fn game_state_to_input(&self) -> Vec<f64> {
        let mut inputs = Vec::new();
        //24 inputs
        for i in 0..8 {
            if i > self.players.len()-1 {
                inputs.push(0 as f64);
                inputs.push(0 as f64);
                inputs.push(0 as f64);
                continue;}
            inputs.push(self.players[i].placeable_card.faceup as i32 as f64);
            inputs.push((self.players[i].money/100) as f64);
            inputs.push(self.players[i].placeable_card.x as f64);
        }
        //24 + 10 = 34 inputs
        let mut camels = self.camels.clone();
        camels.sort_by(|a, b| a.color.partial_cmp(&b.color).unwrap());
        for i in 0..camels.len() {
            inputs.push((self.camels[i].x/self.options.map_len) as f64);
            inputs.push((self.camels[i].y/self.camels.len() as u8) as f64);
        }
        //34+5 = 39 inputs
        for color in &self.tip_cards {
            match color.last() {
                Some(card) => {
                    inputs.push((card.value.get(&Place::Top(1)).unwrap() / 5) as f64);
                },
                None => {
                    inputs.push(0 as f64);
                }
            }
        }
        //39+2 = 41 inputs
        inputs.push((self.winer_oweralltipcards.len()/5) as f64);
        inputs.push((self.loser_oweralltipcards.len()/5) as f64);
        inputs
    }

    //_______________________________________________________________________________________
    pub fn game_winners_ai(&mut self) -> Vec<u16> {
        let mut points = Vec::new();
        for i in 0..self.players.len() {
            points.push(self.players[i].money);
        }
        points
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = CamelCup::a_3_player_new_game();
        assert_eq!(game.options.map_len, 16);
        assert_eq!(game.camels.len(), 5);
        assert_eq!(game.tip_cards.len(), 5);
        assert_eq!(game.tip_cards[0].len(), 3);
        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[0].name, "Player0");
        assert_eq!(game.players[0].money, 3);
        assert_eq!(game.players[0].placeable_card.x, 0);
        assert_eq!(game.players[0].placeable_card.faceup, false);
        assert_eq!(game.players[0].tip_cards.len(), 0);
        assert_eq!(game.winer_oweralltipcards.len(), 0);
        assert_eq!(game.current_player, 0);
        assert_eq!(game.loser_oweralltipcards.len(), 0);
        assert_eq!(game.common_oweralltipcards.len(), 15);
        }
    #[test]
    fn test_get_tip_card() {
        let mut game = CamelCup::a_3_player_new_game();
        game.move_tip_card(0, Color::White).unwrap();
        assert_eq!(game.players[0].tip_cards.len(), 1);
        assert_eq!(game.players[0].tip_cards[0].color, Color::White);
        assert_eq!(game.players[0].tip_cards[0].value.get(&Place::Top(1)).unwrap(), &5);
        assert_eq!(game.tip_cards.len(), 5);
        assert_eq!(game.tip_cards[0].len(), 2);
        assert_eq!(game.tip_cards[0].pop().unwrap().value.get(&Place::Top(1)).unwrap(), &3);
        assert_eq!(game.tip_cards[0].pop().unwrap().value.get(&Place::Top(1)).unwrap(), &2);
    }

    #[test]
    fn test_get_tip_card_error() {
        let mut game = CamelCup::a_3_player_new_game();
        game.move_tip_card(0, Color::White).unwrap();
        game.move_tip_card(0, Color::White).unwrap();
        game.move_tip_card(0, Color::White).unwrap();
        assert_eq!(game.move_tip_card(0, Color::White), Err("No more cards of this color left"));
    }

    #[test]
    fn test_place_card() {
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.place_card(0, 1, true), Ok(()));
        assert_eq!(game.place_card(0, 1, false), Ok(()));
        assert_eq!(game.place_card(0, 2, true), Ok(()));
        assert_eq!(game.place_card(1, 2, false), Err("x is already occupied by some other player"));
        assert_eq!(game.place_card(1, 1, true), Err("x nearby is already occupied by some other player"));
        assert_eq!(game.place_card(1, 210, true), Err("x is out of bounds"));
        assert_eq!(game.place_card(1, 17, false), Err("x is out of bounds"));
        assert_eq!(game.place_card(2021, 4, true), Err("player_number is out of bounds"));
        game.camels[0].x = 4;
        assert_eq!(game.place_card(1, 4, true), Err("x is already occupied by some camel(s)"));
        assert_eq!(game.place_card(0, 2, true), Err("You can't place Your card the same"));
    }

    #[test]
    fn test_next_player() {
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.current_player(), 0);
        game.next_player();
        assert_eq!(game.current_player(), 1);
        game.next_player();
        assert_eq!(game.current_player(), 2);
        game.next_player();
        assert_eq!(game.current_player(), 0);
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
    fn test_end_game_bet(){
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.end_game_bet(0, true, Color::White), Ok(()));
        assert_eq!(game.end_game_bet(110, false, Color::White), Err("player_number is out of bounds"));
        assert_eq!(game.end_game_bet(0, true, Color::White), Err("You already bet on this color"));
        assert_eq!(game.end_game_bet(1, true, Color::White), Ok(()));
        assert_eq!(game.end_game_bet(1, false, Color::White), Err("You already bet on this color"));
        assert_eq!(game.end_game_bet(2, false, Color::White), Ok(()));
        assert_eq!(game.common_oweralltipcards.len(), 12);
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
        game.end_game_bet(0, true, Color::White).unwrap();
        game.end_game_bet(1, false, Color::White).unwrap();
        game.end_game_bet(2, true, Color::White).unwrap();
        assert_eq!(game.players[0].money, 3);
        assert_eq!(game.players[1].money, 3);
        assert_eq!(game.players[2].money, 3);
        assert_eq!(game.winer_oweralltipcards.len(), 2);
        assert_eq!(game.loser_oweralltipcards.len(), 1);
        assert_eq!(game.common_oweralltipcards.len(), 27);
        game.end_game_bet(3, false, Color::RGB(Some("Orange".to_string()), 255, 165, 0)).unwrap();
        game.end_game_bet(4, false, Color::RGB(Some("Orange".to_string()), 255, 165, 0)).unwrap();
        game.end_game_bet(5, true, Color::RGB(Some("Orange".to_string()), 255, 165, 0)).unwrap();
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
    fn test_move_camel() {
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.move_camel(Color::White, 0), Err("amount must be positive"));
        assert_eq!(game.move_camel(Color::White, 4), Err("amount is too big"));
        assert_eq!(game.move_camel(Color::BrightBlack, 1), Err("No camel with this color"));
        game.place_card(0, 1, true).unwrap();
        game.next_player();
        //1
        game.place_card(1, 3, false).unwrap();
        game.next_player();
        //2
        game.move_camel(Color::White, 1).unwrap();
        game.next_player();
        //0
        assert_eq!(game.camels[0].x, 2);
        assert_eq!(game.camels[0].y, 0);
        assert_eq!(game.camels[0].moved, true);
        assert_eq!(game.players[0].money, 4);
        assert_eq!(game.players[2].money, 4);
        assert_eq!(game.move_camel(Color::White, 1), Err("camel already moved this turn"));
        game.move_camel(Color::Green, 2).unwrap();
        game.next_player();
        //1
        game.camel_test_helper(Color::Green, 2, 1, true);
        assert_eq!(game.players[0].money, 5);
        assert_eq!(game.players[2].money, 4);
        game.move_camel(Color::Blue, 3).unwrap();
        game.next_player();
        //2
        game.camel_test_helper(Color::Blue, 2, 0, true);
        assert_eq!(game.players[0].money, 5);
        assert_eq!(game.players[2].money, 4);
        assert_eq!(game.players[1].money, 5);
        for camel in game.camels.iter_mut() {
            camel.moved = false;
        }
        game.move_camel(Color::White, 1).unwrap();
        game.camel_test_helper(Color::White, 2, 0, true);
        game.camel_test_helper(Color::Green, 2, 1, false);
        game.camel_test_helper(Color::Blue, 2, 2, false);
        game.move_camel(Color::Green, 3).unwrap();
        game.camel_test_helper(Color::White, 2, 0, true);
        game.camel_test_helper(Color::Green, 5, 0, true);
        game.camel_test_helper(Color::Blue, 5, 1, false);
        
    }
    
    #[test]
    fn evaluate_end_turn(){
        let mut game = CamelCup::a_n_player_game(5);
        game.move_tip_card(game.current_player, Color::White).unwrap();
        game.next_player();
        game.move_tip_card(game.current_player, Color::White).unwrap();
        game.next_player();
        game.move_tip_card(game.current_player, Color::White).unwrap();
        game.next_player();
        assert_eq!(game.move_tip_card(game.current_player, Color::White), Err("No more cards of this color left"));
        game.move_tip_card(game.current_player, Color::Green).unwrap();
        game.next_player();
        game.move_tip_card(game.current_player, Color::Blue).unwrap();
        game.next_player();
        game.move_camel(Color::White, 3).unwrap();
        game.next_player();
        game.move_camel(Color::Green, 2).unwrap();
        game.next_player();
        game.move_camel(Color::Blue, 1).unwrap();
        game.next_player();
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
    fn reset() {
        let mut game = CamelCup::a_n_player_game(5);
        game.move_tip_card(game.current_player, Color::White).unwrap();
        game.reset_turn();
        game.move_tip_card(game.current_player, Color::White).unwrap();
        assert_eq!(game.players[0].tip_cards[0].color, Color::White);
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


}
