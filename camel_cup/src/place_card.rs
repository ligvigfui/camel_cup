use crate::CamelCup;

#[derive(Debug)]
pub struct PlaceCard {
    pub(crate) x: u8,
    pub(crate) faceup: bool,
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
    
    pub fn place_card_player(&mut self, player_number: usize, x: u8, faceup: bool) -> Result<(), &'static str> {
        if x < 1 || x > self.map_len {
            return Err("x is out of bounds");
        }
        if player_number >= self.players.len() {
            return Err("player_number is out of bounds");
        }
        for (i, player) in self.players.iter().enumerate() {
            if player.placeable_card.x == x && self.players[player_number].placeable_card.x != x{
                return Err("x is already occupied by some other player");
            }
            if player.placeable_card.x == x+1 && self.players[player_number].placeable_card.x != x+1{
                return Err("x nearby is already occupied by some other player");
            }
            if player.placeable_card.x == x-1 && self.players[player_number].placeable_card.x != x-1 && x-1 != 0{
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
    pub fn place_card(&mut self, x: u8, faceup: bool) -> Result<(), &'static str> {
        self.place_card_player(self.current_player, x, faceup)?;
        self.next_player();
        Ok(())
    }
}