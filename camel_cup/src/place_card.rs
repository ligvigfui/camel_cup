use crate::{CamelCup, Player};

#[derive(Debug)]
pub struct PlaceCard {
    pub x: u8,
    pub faceup: bool,
}
impl PlaceCard {
    pub fn new() -> Self {
        PlaceCard {
            x: 0,
            faceup: false,
        }
    }
}

impl CamelCup {
    
    pub fn place_card_player(&mut self, player_number: usize, x: u8, faceup: bool) -> Result<(), String> {
        CamelCup::map_len_out_of_bounds(&self, x)?;
        Player::number_out_of_bounds(&self, player_number)?;
        for (i, player) in self.players.iter().enumerate() {
            if player.placeable_card.x == x && self.players[player_number].placeable_card.x != x{
                return Err(format!("tile {x} is already occupied by some other player"));
            }
            if player.placeable_card.x == x+1 && self.players[player_number].placeable_card.x != x+1{
                return Err(format!("tile {x} nearby is already occupied by some other player"));
            }
            if player.placeable_card.x == x-1 && self.players[player_number].placeable_card.x != x-1 && x-1 != 0{
                return Err(format!("tile {x} nearby is already occupied by some other player"));
            }
            if player_number == i && player.placeable_card.x == x && player.placeable_card.faceup == faceup {
                return Err("You can't place Your card the same".to_string());
            }
        }
        for camel in self.camels.iter() {
            if camel.x == x {
                return Err(format!("tile {x} is already occupied by some camel(s)"));
            }
        }
        self.players[player_number].placeable_card.x = x;
        self.players[player_number].placeable_card.faceup = faceup;
        Ok(())
    }
    pub fn place_card(&mut self, x: u8, faceup: bool) -> Result<(), String> {
        self.place_card_player(self.current_player, x, faceup)?;
        self.next_player();
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn place_card_ok() {
        let mut game = CamelCup::a_3_player_new_game();
        assert!(game.place_card(2, true).is_ok());
        assert!(game.place_card(4, true).is_ok());
        let _ = game.place_card(7, true);
        assert!(game.place_card(2, false).is_ok());
        assert!(game.place_card(5, true).is_ok());
    }

    #[test]
    fn place_card_out() {
        let mut game = CamelCup::a_3_player_new_game();
        assert!(game.place_card(1, true).is_err());
        assert!(game.place_card(20, true).is_err());
    }

    #[test]
    fn place_card_camel_present() {
        let mut game = CamelCup::a_3_player_new_game();
        game.camels[0].x = 2;
        assert!(game.place_card(2, true).is_err());
    }

    #[test]
    fn place_card_card_present() {
        let mut game = CamelCup::a_3_player_new_game();
        let _ = game.place_card(2, true);
        assert!(game.place_card(2, false).is_err());
    }
    

    #[test]
    fn place_card_card_near() {
        let mut game = CamelCup::a_3_player_new_game();
        let _ = game.place_card(2, true);
        assert!(game.place_card(3, false).is_err());
    }
    
    #[test]
    fn place_card_same() {
        let mut game = CamelCup::a_3_player_new_game();
        assert!(game.place_card(2, true).is_ok());
        assert!(game.place_card(4, true).is_ok());
        let _ = game.place_card(6, true);
        assert!(game.place_card(2, true).is_err());
    }
}