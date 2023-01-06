use std::{option::Option, io};
use std::error::Error;

use rand::Rng;

#[derive(Debug)]
struct Options {
    map_length: i32,
    step_max_amount: i32,
}
#[derive(Debug)]
struct CamelCup {
    current_player: usize,
    options: Options,
    camels: Vec<Camel>,
    tip_cards: Vec<Vec<TipCard>>,
    players: Vec<Player>,
    winer_endgametipcards: Vec<EndgameTipcard>,
    loser_endgametipcards: Vec<EndgameTipcard>,
    common_endgametipcards: Vec<EndgameTipcard>,
}
impl CamelCup {
    fn new(players: Vec<Player>) -> CamelCup {
        CamelCup {
            current_player: 0,
            options: Options {
                map_length: 16,
                step_max_amount: 3,
            },
            camels: setup_camels(),
            tip_cards: setup_tip_cards(),
            common_endgametipcards: setup_endgame_tipcards(players.len()),
            players,
            winer_endgametipcards: Vec::new(),
            loser_endgametipcards: Vec::new(),
        }
    }
    fn a_3_player_new_game() -> CamelCup {
        CamelCup::new(vec![Player::new(None, 0), Player::new(None, 1), Player::new(None, 2)])
    }
    fn a_n_player_game(n: i32) -> CamelCup {
        let mut players = Vec::new();
        for i in 0..n {
            players.push(Player::new(None, i));
        }
        CamelCup::new(players)
    }
    pub fn move_tip_card(&mut self, player_number: usize, tip_card_color: &str) -> Result<(), &'static str> {
        for color in self.tip_cards.iter_mut() {
            for card in color.iter_mut() {
                if card.color == tip_card_color {
                    let tip_card = match color.pop() {
                        Some(card) => card,
                        _ => return Err("How did you get here?"),
                    };
                    self.players[player_number].owned_tip_cards.push(tip_card);
                    return Ok(());
                }
            }
        };
        Err("No more cards of this color left")
    }
    pub fn place_card(&mut self, player_number: usize, x: i32, faceup: bool) -> Result<(), &'static str> {
        if x < 1 || x > self.options.map_length {
            return Err("x is out of bounds");
        }
        if player_number >= self.players.len() {
            return Err("player_number is out of bounds");
        }
        for player in self.players.iter() {
            if player.placeable_card.x == x && self.players[player_number].placeable_card.x != x{
                return Err("x is already occupied by some other player");
            }
            else if player.placeable_card.x == x+1 && self.players[player_number].placeable_card.x != x+1{
                return Err("x nearby is already occupied by some other player");
            }
            else if player.placeable_card.x == x-1 && self.players[player_number].placeable_card.x != x-1 && x-1 != 0{
                return Err("x nearby is already occupied by some other player");
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
    pub fn current_player(&self) -> usize {
        self.current_player
    }
    fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    //______________________________________________________________________________________________________________________
    fn end_turn_check(&self) -> bool {
        let mut end_of_turn = true;
        for camel in self.camels.iter() {
            end_of_turn = end_of_turn && camel.moved;
        }
        end_of_turn
    }

    //______________________________________________________________________________________________________________________
    fn end_game_check(&self) -> bool {
        for camel in self.camels.iter() {
            if camel.x > self.options.map_length {
                return true;
            }
        }
        false
    }

    //______________________________________________________________________________________________________________________
    pub fn end_game_bet(&mut self, player_number: usize, winer: bool, color: &str) -> Result<(), &'static str> {
        if player_number >= self.players.len() {
            return Err("player_number is out of bounds");
        }
        for (i, endgametipcard) in self.common_endgametipcards.iter().enumerate() {
            if endgametipcard.color == color && endgametipcard.owner == player_number {
                if winer {
                    self.winer_endgametipcards.push(self.common_endgametipcards.remove(i));
                    return Ok(());
                } else {
                    self.loser_endgametipcards.push(self.common_endgametipcards.remove(i));
                    return Ok(());
                }
            }
        }
        Err("You already bet on this color")
    }

    //______________________________________________________________________________________________________________________
    fn end_game_evaluate(&mut self) -> Result<(), &'static str> {
        if self.camels[0].x <= self.options.map_length {
            return Err("The game is not over yet");
        }
        println!("{:?}", self);
        let mut winer_reward = vec![2, 3, 5, 8];
        let mut loser_reward = winer_reward.clone();
        for endgametipcard in self.winer_endgametipcards.iter() {
            if endgametipcard.color == self.camels[0].color {
                println!("poping value from winer_reward {}", winer_reward[winer_reward.len()-1]);
                self.players[endgametipcard.owner].money += match winer_reward.pop() {
                    Some(score) => score,
                    _ => 1,
                };
            } else {
                println!("{} lost 1", self.players[endgametipcard.owner].name);
                self.players[endgametipcard.owner].money -= 1;
            }
        }
        for endgametipcard in self.loser_endgametipcards.iter() {
            if endgametipcard.color == self.camels[self.camels.len()-1].color {
                println!("poping value from loser_reward {}", loser_reward[loser_reward.len()-1]);
                self.players[endgametipcard.owner].money += match loser_reward.pop() {
                    Some(score) => score,
                    _ => 1,
                };
            } else {
                println!("{} lost 1", self.players[endgametipcard.owner].name);
                self.players[endgametipcard.owner].money -= 1;
            }
        }
        io::stdin().read_line(&mut String::new()).unwrap();
        Ok(())
    }

    //______________________________________________________________________________________________________________________
    fn order_camels(&mut self){
        let e:i32 = self.camels.len().try_into().unwrap();
        self.camels.sort_by(|a, b|{ 
            let c:i32 = a.x*e+a.y as i32; 
            let d:i32 = b.x*e+b.y as i32;
            d.cmp(&c)
        });
    }

    //______________________________________________________________________________________________________________________
    fn first_camel(&mut self) -> usize {
        self.order_camels();
        0
    }
    fn second_camel(&mut self) -> usize {
        self.order_camels();
        1
    }
    fn last_camel(&mut self) -> usize {
        self.order_camels();
        self.camels.len()-1
    }

    //______________________________________________________________________________________________________________________
    fn set_camel_position(&mut self, camel_color: &str, x: i32, y: i32) -> Result<(), &'static str> {
        //! test fn only
        for (camel_number, camel) in self.camels.iter().enumerate() {
            if camel.x == x && camel.y == y {
                return Err("x and y are already occupied by some other camel");
            }
        }
        for (camel_number, camel) in self.camels.iter_mut().enumerate() {
            if camel.color == camel_color {
                camel.x = x;
                camel.y = y;
                camel.moved = true;
                return Ok(());
            }
        }
        Err("No camel with this color")
    }

    //______________________________________________________________________________________________________________________
    pub fn move_camel(&mut self, camel_color: &str, amount: i32) -> Result<(), &'static str> {
        if amount <= 0 {
            return Err("amount must be positive");
        }
        if amount > self.options.step_max_amount {
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
                println!("{} {} {} {}", camel.x, camel.y, camel.moved, camel.color);
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
        println!("{} {} {:?}", x, y, camels_above);
        let mut case1 = true;
        for (player_number, player) in self.players.iter_mut().enumerate() {
            if player.placeable_card.x == x+amount {
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
            for (camel_number, camel) in self.camels.iter_mut().enumerate() {
                if camel.x == x+amount {
                    camels_below += 1;
                }
            }
            for camel in camels_above.iter() {
                self.camels[*camel].x += amount;
                self.camels[*camel].y += camels_below - y;
            }
        } else {
            //camels are moved below 
            //megemeli az itt levő camelt
            for (camel_number, camel) in self.camels.iter_mut().enumerate() {
                if camel.x == x+amount && !camels_above.contains(&camel_number){
                    camel.y += camels_above.len() as i32;
                }
            }
            //a mozgó camelt beteszi alá elvielg
            for camel in camels_above.iter() {
                println!("{} {} {}", camel, self.camels[*camel].x, self.camels[*camel].y);
                self.camels[*camel].x += amount;
                self.camels[*camel].y -= y;
                println!("{} {} {}", camel, self.camels[*camel].x, self.camels[*camel].y);
            }
        }
        self.order_camels();
        Ok(())
    }

    //_______________________________________________________________________________________
    fn camel_test_helper(&self, color: &str, assert_x: i32, assert_y: i32, assert_moved: bool) {
        let camel = self.camels.iter().find(|camel| camel.color == color).unwrap();
        assert_eq!(assert_x, camel.x);
        assert_eq!(assert_y, camel.y);
        assert_eq!(assert_moved, camel.moved);
    }

    //_______________________________________________________________________________________
    fn evaluate_end_turn(&mut self) -> Result<(), &'static str> {
        if self.camels.iter().any(|camel| camel.moved == false) {
            return Err("Not all camels moved");
        }
        for player in self.players.iter_mut() {
            for (i, card) in player.owned_tip_cards.iter_mut().enumerate() {
                if card.color == self.camels[0].color {
                    player.money += card.value;
                } else if card.color == self.camels[1].color {
                    player.money += 1;
                } else {
                    player.money -= 1;
                }
            }
        }
        Ok(())
    }

    //_______________________________________________________________________________________
    fn reset_turn(&mut self) {
        for camel in self.camels.iter_mut() {
            camel.moved = false;
        }
        for player in self.players.iter_mut() {
            player.owned_tip_cards.clear();
        }
        self.tip_cards = setup_tip_cards();
    }

    //_______________________________________________________________________________________
    fn display (&self) {
        //display current player
        println!("Current player: \x1b[1m{}\x1b[0m", self.players[self.current_player].name);
        //display camels
        //display map
        //display player cards
        println!("{}", self.display_camels());
        //display tip cards
        //display player money
        for player in self.players.iter() {
            println!("{}: {}", player.name, player.money);
            if player.owned_tip_cards.len() > 0 {
                print!("{}'s cards: ", player.name);
                for card in player.owned_tip_cards.iter() {
                    print!("{}{}\x1b[0m {},\t", card.display_color, card.color, card.value);
                }
                println!();
            }
        }
        //display tip cards
        print!("{}", self.display_tip_cards());
        //display endgame info
        println!("\x1b[1m{}\x1b[0m cards bet on the winer camel", self.winer_endgametipcards.len());
        println!("\x1b[11m{}\x1b[0m cards bet on the losing camel", self.loser_endgametipcards.len());
    }


    //_______________________________________________________________________________________
    fn display_tip_cards(&self) -> String {
        let mut display = String::new();
        for cards in self.tip_cards.iter() {
            if cards.len() != 0 {
                &display.push_str(&cards[0].display_color[..]);
                &display.push_str(&cards[0].color[..]);
                &display.push_str("\x1b[0m\x1b[1m:\t");
            } else {
                continue;
            }
            for card in cards.iter() {
                &display.push_str(&card.value.to_string()[..]);
                &display.push_str(",\t");
            }
            &display.push_str("\x1b[0m\n");
        }
        display
    }


    //_______________________________________________________________________________________
    fn display_camels(&self) -> String {
        let mut display = String::new();
        for i in (0..=self.camels.len()-1).rev() {
            for (j, camel) in self.camels.iter().enumerate() {
                if !camel.moved && i == j{
                    &display.push_str(&camel.display_color[..]);
                    &display.push_str(&camel.color[0..3]);
                    &display.push_str("\x1b[0m");
                } else if i == j {
                    &display.push_str("   ");
                }
            }
            for j in 1..self.options.map_length+1 {
                let mut found = false;
                for camel in self.camels.iter() {
                    if camel.x == j && camel.y == i.try_into().unwrap() {
                        &display.push_str(&camel.display_color[..]);
                        &display.push_str(&camel.color[0..3]);
                        &display.push_str("\x1b[0m");
                        found = true;
                    }
                }
                if !found {
                    &display.push_str("   ");
                }
            }
            &display.push_str("\n");
        }
        for i in 0..self.options.map_length+1 {
            if i > 9 {
                &display.push_str(&format!("_{}", i));
            } else {
                &display.push_str(&format!("_{}_", i));
            }
        }
        &display.push_str("__winner's_on_top__\n   ");
        let mut player_found = false;
        for i in 1..self.options.map_length+1 {
            for (j, player) in self.players.iter().enumerate() {
                if player.placeable_card.x == i && j == self.current_player{
                    &display.push_str("\x1b[31m");
                    if player.placeable_card.faceup {
                        &display.push_str(" + ");
                    } else {
                        &display.push_str(" - ");
                    }
                    &display.push_str("\x1b[0m");
                    player_found = true;
                } else if player.placeable_card.x == i {
                    if player.placeable_card.faceup {
                        &display.push_str(" + ");
                    } else {
                        &display.push_str(" - ");
                    }
                    player_found = true;
                }
            }
            if !player_found {
                &display.push_str("   ");
            }
            player_found = false;
        }
        &display.push_str("\n");
        display
    }

    //_______________________________________________________________________________________
    fn turn(&mut self) -> Result<(), &'static str> {
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
        let color = (self.not_moved_camels()[random]).to_string().clone();
        let amount = rand::thread_rng().gen_range(1..4);
        self.move_camel(&color[..], amount)
    }

    //_______________________________________________________________________________________
    fn not_moved_camels(&self) -> Vec<&str> {
        let mut not_moved_camels = Vec::new();
        for camel in self.camels.iter() {
            if !camel.moved {
                not_moved_camels.push(&camel.color[..]);
            }
        }
        not_moved_camels
    }

    //_______________________________________________________________________________________
    fn human_place_card(&mut self) -> Result<(), &'static str> {
        println!("Where do you want to place your card? (type the number)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Please type a number (1-16)"),
        };
        if input > self.options.map_length.try_into().unwrap() {
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
        print!("\x1b[2J");
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
            println!("{}: {}{}\x1b[0m \t{}", i, self.tip_cards[*count][0].display_color , self.tip_cards[*count][0].color, self.tip_cards[*count][self.tip_cards[*count].len() - 1].value); 
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
        let tip_card_color = self.tip_cards[smoth_count[input]][0].color.clone();
        self.move_tip_card(self.current_player, &tip_card_color)?;
        Ok(())
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
        print!("\x1b[2J");
        println!("{}", self.display_camels());
        if input == 0 {println!("Currently {} cards bet on the winner camel", self.winer_endgametipcards.len());}
        else {println!("Currently {} cards bet on the loser camel", self.loser_endgametipcards.len());}
        println!("Which camel do you want to bet on? (type the number)");
        let mut endgametipcards = Vec::new();
        for (i, endgametipcard) in self.common_endgametipcards.iter().enumerate() {
            if endgametipcard.owner == self.current_player {
                endgametipcards.push(i);
            }
        }
        for (i, common_i) in endgametipcards.iter().enumerate() {
            println!("{}: {}{}\x1b[0m", i, self.common_endgametipcards[*common_i].display_color, self.common_endgametipcards[*common_i].color);
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
        let tip_card_color = self.common_endgametipcards[endgametipcards[input2]].color.clone();
        self.end_game_bet(self.current_player, input == 0, &tip_card_color)?;
        Ok(())
    }

    
}





#[derive(Debug)]
struct Camel {
    display_color: String,
    color: String,
    x: i32,
    y: i32,
    moved: bool,
}
impl Camel {
    fn move_camel(&mut self, camels: &mut Vec<Camel>, amount: i32) {
        //TODO: implement
    }
}
fn setup_camels () -> Vec<Camel> {
    let mut camels = Vec::new();
    for color in vec![("white", "\x1B[1m"), ("blue", "\x1b[34m"), ("green","\x1b[32m"), ("yellow","\x1b[33m"), ("orange", "\x1b[38;2;255;165;0m")] {
        camels.push(Camel {display_color: color.1.to_string(), color: color.0.to_string(), x: 0, y: 0, moved: false });
    }
    camels
}
#[derive(Debug)]
struct TipCard {
    display_color: String,
    color: String,
    value: i32,
}

fn setup_tip_cards() -> Vec<Vec<TipCard>> {
    let mut tip_cards = Vec::new();
    for color in vec![("white", "\x1B[1m"), ("blue", "\x1b[34m"), ("green","\x1b[32m"), ("yellow","\x1b[33m"), ("orange", "\x1b[38;2;255;165;0m")] {
        let mut color_cards = Vec::new();
        for value in vec![2,3,5] {
            color_cards.push(TipCard {display_color: color.1.to_string(), color: color.0.to_string(), value });
        }
        tip_cards.push(color_cards);
    }
    tip_cards
}

#[derive(Debug)]
struct Player {
    name: String,
    money: i32,
    placeable_card: PlaceCard,
    owned_tip_cards: Vec<TipCard>,
}
impl Player {
    fn new(name: Option<String>, number: i32) -> Player {
        let name = match name {
            Some(name) => name,
            None => "Player".to_string() + &number.to_string(),
        };
        Player {
            name,
            money: 3,
            placeable_card: PlaceCard { x: 0, faceup: false },
            owned_tip_cards: Vec::new(),
        }
    }
}


#[derive(Debug)]
struct PlaceCard {
    x: i32,
    faceup: bool,
}

#[derive(Debug)]
struct EndgameTipcard {
    display_color: String,
    color: String,
    owner: usize,
}
fn setup_endgame_tipcards(player_number: usize) -> Vec<EndgameTipcard> {
    let mut endgame_tipcards = Vec::new();
    for i in 0..player_number {
        for color in vec![("white", "\x1B[1m"), ("blue", "\x1b[34m"), ("green","\x1b[32m"), ("yellow","\x1b[33m"), ("orange", "\x1b[38;2;255;165;0m")] {
            endgame_tipcards.push(EndgameTipcard {display_color: color.1.to_string(), color: color.0.to_string(), owner: i });
        }
    }
    endgame_tipcards
}

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
    let player_number: i32 = player_number.trim().parse().unwrap();
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
            names.push(Player::new(Some(name), i as i32));
        }
    } else {
        for i in 0..player_number {
            println!("debud: players are setting up without names {i}");
            names.push(Player::new(None, i as i32));
        }
    }
    print!("\x1b[2J");
    let mut game = CamelCup::new(names);
    game.camels[0].x = 16;
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
            game.end_game_evaluate();
            running = false;
        }
        game.next_player();
    }
    game.players.sort_by(|a, b| b.money.cmp(&a.money));
    for (i, player) in game.players.iter().enumerate() {
        println!("{}: Player {} has {} money", i, player.name, player.money);
    }
    println!("Press enter to exit");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_camel() {
        let mut camels = setup_camels();
        camels[0].x = 1;
        assert_eq!(camels[0].x, 1);
    }
    #[test]
    fn test_new_game() {
        let game = CamelCup::a_3_player_new_game();
        assert_eq!(game.options.map_length, 16);
        assert_eq!(game.camels.len(), 5);
        assert_eq!(game.tip_cards.len(), 5);
        assert_eq!(game.tip_cards[0].len(), 3);
        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[0].name, "Player0");
        assert_eq!(game.players[0].money, 3);
        assert_eq!(game.players[0].placeable_card.x, 0);
        assert_eq!(game.players[0].placeable_card.faceup, false);
        assert_eq!(game.players[0].owned_tip_cards.len(), 0);
        assert_eq!(game.winer_endgametipcards.len(), 0);
        assert_eq!(game.current_player, 0);
        assert_eq!(game.loser_endgametipcards.len(), 0);
        assert_eq!(game.common_endgametipcards.len(), 15);
        }
    #[test]
    fn test_get_tip_card() {
        let mut players = Vec::new();
        for i in 1..3 {
            players.push(Player::new(None, i));
        }
        let mut game = CamelCup::new(players);
        game.move_tip_card(0, "white").unwrap();
        assert_eq!(game.players[0].owned_tip_cards.len(), 1);
        assert_eq!(game.players[0].owned_tip_cards[0].color, "white");
        assert_eq!(game.players[0].owned_tip_cards[0].value, 5);
        assert_eq!(game.tip_cards.len(), 5);
        assert_eq!(game.tip_cards[0].len(), 2);
        assert_eq!(game.tip_cards[0].pop().unwrap().value, 3);
        assert_eq!(game.tip_cards[0].pop().unwrap().value, 2);
    }

    #[test]
    fn test_get_tip_card_error() {
        let mut players = Vec::new();
        for i in 1..3 {
            players.push(Player::new(None, i));
        }
        let mut game = CamelCup::new(players);
        game.move_tip_card(0, "white").unwrap();
        game.move_tip_card(0, "white").unwrap();
        game.move_tip_card(0, "white").unwrap();
        assert_eq!(game.move_tip_card(0, "white"), Err("No more cards of this color left"));
    }

    #[test]
    fn test_place_card() {
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.place_card(0, 1, true), Ok(()));
        assert_eq!(game.place_card(0, 1, false), Ok(()));
        assert_eq!(game.place_card(0, 2, true), Ok(()));
        assert_eq!(game.place_card(1, 2, false), Err("x is already occupied by some other player"));
        assert_eq!(game.place_card(1, 1, true), Err("x nearby is already occupied by some other player"));
        assert_eq!(game.place_card(1, -210, true), Err("x is out of bounds"));
        assert_eq!(game.place_card(1, 17, false), Err("x is out of bounds"));
        assert_eq!(game.place_card(2021, 4, true), Err("player_number is out of bounds"));
        game.camels[0].x = 4;
        assert_eq!(game.place_card(1, 4, true), Err("x is already occupied by some camel(s)"));
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
        assert_eq!(game.end_game_bet(0, true, "white"), Ok(()));
        assert_eq!(game.end_game_bet(110, false, "white"), Err("player_number is out of bounds"));
        assert_eq!(game.end_game_bet(0, true, "white"), Err("You already bet on this color"));
        assert_eq!(game.end_game_bet(1, true, "white"), Ok(()));
        assert_eq!(game.end_game_bet(1, false, "white"), Err("You already bet on this color"));
        assert_eq!(game.end_game_bet(2, false, "white"), Ok(()));
        assert_eq!(game.common_endgametipcards.len(), 12);
    }


    #[test]
    fn test_order_camels(){
        let mut game = CamelCup::a_3_player_new_game();
        for i in 0..game.camels.len() {
            game.camels[i].moved = true;
        }
        game.set_camel_position("white", 1, 1);
        game.set_camel_position("orange", 2, 1);
        game.set_camel_position("blue", 3, 1);
        game.set_camel_position("green", 4,1);
        game.set_camel_position("yellow", 5, 1);
        game.first_camel();
        assert_eq!(game.camels[0].color, "yellow");
        assert_eq!(game.camels[1].color, "green");
        assert_eq!(game.camels[4].color, "white");
        game.set_camel_position("white", 1, 1);
        game.set_camel_position("orange", 1, 2);
        game.set_camel_position("blue", 1, 3);
        game.set_camel_position("green", 1,4);
        game.set_camel_position("yellow", 1, 5);
        game.first_camel();
        assert_eq!(game.camels[0].color, "yellow");
        assert_eq!(game.camels[1].color, "green");
        assert_eq!(game.camels[4].color, "white");
        game.set_camel_position("white", 1, 1);
        game.set_camel_position("orange", 3, 1);
        game.set_camel_position("blue", 6, 1);
        game.set_camel_position("green", 1,2);
        game.set_camel_position("yellow", 5, 1);
        game.first_camel();
        assert_eq!(game.camels[0].color, "blue");
        assert_eq!(game.camels[1].color, "yellow");
        assert_eq!(game.camels[4].color, "white");
    }

    
    #[test]
    fn test_end_game_evaluate(){
        let players = vec![
            Player::new(None, 0),
            Player::new(None, 1),
            Player::new(None, 2),
            Player::new(None, 3),
            Player::new(None, 4),
            Player::new(None, 5),
        ];
        let mut game = CamelCup::new(players);
        game.end_game_bet(0, true, "white").unwrap();
        game.end_game_bet(1, false, "white").unwrap();
        game.end_game_bet(2, true, "white").unwrap();
        assert_eq!(game.players[0].money, 3);
        assert_eq!(game.players[1].money, 3);
        assert_eq!(game.players[2].money, 3);
        assert_eq!(game.winer_endgametipcards.len(), 2);
        assert_eq!(game.loser_endgametipcards.len(), 1);
        assert_eq!(game.common_endgametipcards.len(), 27);
        game.end_game_bet(3, false, "orange").unwrap();
        game.end_game_bet(4, false, "orange").unwrap();
        game.end_game_bet(5, true, "orange").unwrap();
        game.camels[0].x = 17;
        game.camels[1].x = 1;
        game.camels[2].x = 1;
        game.camels[3].x = 1;
        game.camels[4].x = 0;
        game.camels[2].y = 1;
        game.camels[3].y = 1;
        game.camels[0].moved = true;
        game.end_game_evaluate();
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
        assert_eq!(game.move_camel("white", 0), Err("amount must be positive"));
        assert_eq!(game.move_camel("white", 4), Err("amount is too big"));
        assert_eq!(game.move_camel("asdn dasdcsa", 1), Err("No camel with this color"));
        game.place_card(0, 1, true).unwrap();
        game.next_player();
        //1
        game.place_card(1, 3, false).unwrap();
        game.next_player();
        //2
        game.move_camel("white", 1).unwrap();
        game.next_player();
        //0
        assert_eq!(game.camels[0].x, 2);
        assert_eq!(game.camels[0].y, 0);
        assert_eq!(game.camels[0].moved, true);
        assert_eq!(game.players[0].money, 4);
        assert_eq!(game.players[2].money, 4);
        assert_eq!(game.move_camel("white", 1), Err("camel already moved this turn"));
        game.move_camel("green", 2).unwrap();
        game.next_player();
        //1
        game.camel_test_helper("green", 2, 1, true);
        assert_eq!(game.players[0].money, 5);
        assert_eq!(game.players[2].money, 4);
        game.move_camel("blue", 3).unwrap();
        game.next_player();
        //2
        game.camel_test_helper("blue", 2, 0, true);
        assert_eq!(game.players[0].money, 5);
        assert_eq!(game.players[2].money, 4);
        assert_eq!(game.players[1].money, 5);
        for camel in game.camels.iter_mut() {
            camel.moved = false;
        }
        game.move_camel("white", 1).unwrap();
        game.camel_test_helper("white", 2, 0, true);
        game.camel_test_helper("green", 2, 1, false);
        game.camel_test_helper("blue", 2, 2, false);
        game.move_camel("green", 3).unwrap();
        game.camel_test_helper("white", 2, 0, true);
        game.camel_test_helper("green", 5, 0, true);
        game.camel_test_helper("blue", 5, 1, false);
        
    }
    
    #[test]
    fn evaluate_end_turn(){
        let mut game = CamelCup::a_n_player_game(5);
        game.move_tip_card(game.current_player, "white").unwrap();
        game.next_player();
        game.move_tip_card(game.current_player, "white").unwrap();
        game.next_player();
        game.move_tip_card(game.current_player, "white").unwrap();
        game.next_player();
        assert_eq!(game.move_tip_card(game.current_player, "white"), Err("No more cards of this color left"));
        game.move_tip_card(game.current_player, "green").unwrap();
        game.next_player();
        game.move_tip_card(game.current_player, "blue").unwrap();
        game.next_player();
        game.move_camel("white", 3).unwrap();
        game.next_player();
        game.move_camel("green", 2).unwrap();
        game.next_player();
        game.move_camel("blue", 1).unwrap();
        game.next_player();
        game.camels[3].moved = true;
        assert_eq!(game.evaluate_end_turn(), Err("Not all camels moved"));
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
        game.move_tip_card(game.current_player, "white").unwrap();
        game.reset_turn();
        game.move_tip_card(game.current_player, "white").unwrap();
        assert_eq!(game.players[0].owned_tip_cards[0].color, "white");
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
        game.move_camel("white", 1).unwrap();
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
        game.move_camel("blue", 1).unwrap();
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
        game.move_camel("green", 3).unwrap();
        println!("{}", game.display_camels());
        let string = "\u{1b}[38;2;255;165;0mora\u{1b}[0m                                                \n".to_owned() +
        "\u{1b}[33myel\u{1b}[0m                                                \n" +
        "                                                   \n" +
        "   \u{1b}[34mblu\u{1b}[0m                                             \n" +
        "   \u{1b}[1mwhi\u{1b}[0m   \u{1b}[32mgre\u{1b}[0m                                       \n" +
        "_0__1__2__3__4__5__6__7__8__9__10_11_12_13_14_15_16__winner's_on_top__\n" +
        "                                                   \n";
        assert_eq!(game.display_camels(), string);
        game.move_camel("orange", 2).unwrap();
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
    fn god_damn_rng() {

    }
}
