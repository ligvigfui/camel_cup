use crate::*;

#[derive(Debug)]
pub struct OwerallTipcard {
    pub(crate) color: Color,
    pub(crate) owner: usize,
}

impl OwerallTipcard {
    pub fn new(color: Color, owner: usize) -> Self {
        OwerallTipcard {
            color,
            owner
        }
    }

    pub fn new_vec(colors: &Vec<Color>, player_number: usize) -> Vec<OwerallTipcard> {
        let mut endgame_tipcards = Vec::new();
        for i in 0..player_number {
            for color in colors {
                endgame_tipcards.push(Self::new(color.clone(), i));
            }
        }
        endgame_tipcards
    }

    /*pub fn evaluate(&self, game: &CamelCup) -> i8 {
        
    }*/
}

impl CamelCup {
    pub fn end_game_bet_player(&mut self, player_number: usize, winer: bool, color: &Color) -> Result<(), &'static str> {
        Player::number_out_of_bounds(&self, player_number)?;
        for (i, endgametipcard) in self.players[player_number].owerall_tip_cards.iter().enumerate() {
            if &endgametipcard.color == color && endgametipcard.owner == player_number {
                match winer {
                    true  => self.winer_oweralltipcards.push(self.players[player_number].owerall_tip_cards.remove(i)),
                    false => self.loser_oweralltipcards.push(self.players[player_number].owerall_tip_cards.remove(i)),
                }
                return Ok(());
            }
        }
        Err("You already bet on this color")
    }
    pub fn end_game_bet(&mut self, winer: bool, color: &Color) -> Result<(), &'static str> {
        self.end_game_bet_player(self.current_player, winer, color)?;
        self.next_player();
        Ok(())
    }
}