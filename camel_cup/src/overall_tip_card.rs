use crate::*;

#[derive(Debug)]
pub struct OverallTipcard {
    pub(crate) color: Color,
    pub(crate) owner: usize,
}

impl OverallTipcard {
    pub fn new(color: Color, owner: usize) -> Self {
        OverallTipcard {
            color,
            owner
        }
    }

    pub fn new_vec(colors: &Vec<Color>, player_number: usize) -> Vec<OverallTipcard> {
        let mut endgame_tipcards = Vec::new();
        for color in colors {
            endgame_tipcards.push(Self::new(color.clone(), player_number));
        }
        endgame_tipcards
    }

    /*pub fn evaluate(&self, game: &CamelCup) -> i8 {
        
    }*/
}

impl CamelCup {
    pub fn end_game_bet_player(&mut self, player_number: usize, winer: bool, color: &Color) -> Result<(), &'static str> {
        Player::number_out_of_bounds(&self, player_number)?;
        for (i, endgametipcard) in self.players[player_number].overall_tip_cards.iter().enumerate() {
            if &endgametipcard.color == color && endgametipcard.owner == player_number {
                match winer {
                    true  => self.winer_overalltipcards.push(self.players[player_number].overall_tip_cards.remove(i)),
                    false => self.loser_overalltipcards.push(self.players[player_number].overall_tip_cards.remove(i)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec() {
        let game = CamelCup::a_3_player_new_game();
        assert!(game.players[0].overall_tip_cards.len() == 5);
        assert!(game.players[0].overall_tip_cards[0].owner == 0);
        assert!(game.players[0].overall_tip_cards[1].owner == 0);
        assert!(game.players[0].overall_tip_cards[2].owner == 0);
        assert!(game.players[0].overall_tip_cards[3].owner == 0);
        assert!(game.players[0].overall_tip_cards[4].owner == 0);
        assert!(game.players[1].overall_tip_cards.len() == 5);
        assert!(game.players[2].overall_tip_cards.len() == 5);
    }

    #[test]
    fn overall_tip(){
        let mut game = CamelCup::a_3_player_new_game();
        assert_eq!(game.end_game_bet(true, &Color::White), Ok(()));
        game.current_player = 0;
        assert_eq!(game.end_game_bet(true, &Color::White), Err("You already bet on this color"));
        game.current_player = 1;
        assert_eq!(game.end_game_bet(true, &Color::White), Ok(()));
        game.current_player = 1;
        assert_eq!(game.end_game_bet(false, &Color::White), Err("You already bet on this color"));
        game.current_player = 2;
        assert_eq!(game.end_game_bet(false, &Color::White), Ok(()));
    }
}