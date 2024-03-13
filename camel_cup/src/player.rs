use crate::*;

#[derive(Debug)]
pub struct Player {
    pub(crate) name: String,
    pub(crate) money: u16,
    pub(crate) placeable_card: PlaceCard,
    pub(crate) tip_cards: Vec<TipCard>,
    pub(crate) overall_tip_cards: Vec<OverallTipcard>,
}
impl Player {
    pub fn new(options: &mut Options) -> Player {
        let name = options.player_names.pop().unwrap();
        let name = match name {
            Some(name) => name,
            None => format!("Player {}", options.player_names.len()),
        };
        Player {
            name,
            money: 3,
            placeable_card: PlaceCard::new(),
            tip_cards: Vec::new(),
            overall_tip_cards: OverallTipcard::new_vec(&options.camel_colors, options.player_names.len()),
        }
    }

    pub fn new_vec(options: &mut Options) -> Vec<Player> {
        let mut result = Vec::with_capacity(options.player_names.len());
        while let Some(_) = options.player_names.last() {
            result.insert(0, Player::new(options));
        }
        result
    }

    pub fn modify_money(&mut self, amount: i8) {
        if self.money as i32 + (amount as i32) < 0 {
            self.money = 0;
            return;
        }
        self.money = (self.money as i32 + (amount as i32)) as u16
    }

    pub fn evaluate_tip_cards(&mut self, camels: &Vec<Camel>) -> Vec<TipCard> {
        let cards = Vec::new();
        while let Some(card) = self.tip_cards.pop() {
            let amount = card.evaluate(camels);
            self.modify_money(amount)
        }
        cards
    }

    pub fn number_out_of_bounds(game: &CamelCup, player_number: usize) -> Result<(), &'static str> {
        match player_number >= game.players.len() {
            true => Err("player_number is out of bounds"),
            false => Ok(()),
        }
    }
}

impl CamelCup {
    pub fn end_game_players_display(&mut self) {
        self.players.sort_by(|a, b| b.money.cmp(&a.money));
        for (i, player) in self.players.iter().enumerate() {
            println!("{}: Player {} has {} money", i, player.name, player.money);
        }
        println!("Press enter to exit");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    }
}