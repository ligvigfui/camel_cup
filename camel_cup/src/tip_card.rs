use crate::*;

#[derive(Debug)]
pub struct TipCard {
    pub(crate) color: Color,
    pub(crate) values: HashMap<Place, i8>,
}

impl TipCard {
    pub fn new(color: &Color, values: &HashMap<Place, i8>) -> Result<TipCard, &'static str> {
        // trow error if value contains more then 1 remaining
        let mut remaining = 0;
        for (place, _) in values {
            match place {
                Place::TopRemaining |
                Place::MiddleRemaining |
                Place::BottomRemaining => {
                    remaining += 1;
                }
                _ => {}
            }
        }
        if remaining > 1 {
            return Err("More then 1 remaining place in tip card");
        }
        Ok(TipCard {
            color: color.clone(),
            values: values.clone(),
        })
    }

    pub fn new_vec(options: &Options) -> Result<Vec<TipCard>, &'static str> {
        let mut tip_cards = Vec::new();
        for color in &options.camel_colors {
            for value in &options.leg_tips {
                tip_cards.push(TipCard::new(color, value)?);
            }
        }
        Ok(tip_cards)
    }

    pub fn evaluate(&self, camels: &[Camel]) -> i8 {
        let position = Camel::place(&self.color, camels);
        let mut result = 0;
        for (place, &value) in &self.values {
            match place {
                Place::Top(n) => {
                    if position == *n as usize {
                        return value;
                    }
                }
                Place::Bottom(n) => {
                    if position == camels.len() - *n as usize {
                        return value;
                    }
                }
                Place::TopRemaining |
                Place::MiddleRemaining |
                Place::BottomRemaining => {
                    result = value;
                }
                Place::Wrong => return 0,
            }
        }
        result
    }
}

impl CamelCup {
    pub(crate) fn move_tip_card_player(&mut self, player_number: usize, tip_card_color: &Color) -> Result<(), &'static str> {
        Player::number_out_of_bounds(&self, player_number)?;
        
        let mut max_amount = 0;
        for card in self.tip_cards.iter() {
            if &card.color == tip_card_color {
                max_amount = match max_amount < *card.values.get(&Place::Top(1)).unwrap() {
                    true => *card.values.get(&Place::Top(1)).unwrap(),
                    false => max_amount,
                };
            }
        };
        match max_amount {
            0 => Err("No more cards of this color left"),
            _ => {
                for (index, card) in self.tip_cards.iter().enumerate() {
                    if &card.color == tip_card_color && *card.values.get(&Place::Top(1)).unwrap() == max_amount {
                        let tip_card = self.tip_cards.remove(index);
                        self.players[player_number].tip_cards.push(tip_card);
                        return Ok(());
                    }
                }
                Err("tip card / impl CamelCup::move_tip_card_player - did not find card again")
            }
        }
    }
    pub fn move_tip_card(&mut self, tip_card_color: &Color) -> Result<(), &'static str> {
        self.move_tip_card_player(self.current_player, tip_card_color)?;
        self.next_player();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_get_tip_card() {
        let mut game = CamelCup::a_3_player_new_game();
        game.move_tip_card_player(0, &Color::White).unwrap();
        assert_eq!(game.players[0].tip_cards.len(), 1);
        assert_eq!(game.players[0].tip_cards[0].color, Color::White);
        assert_eq!(game.players[0].tip_cards[0].values.get(&Place::Top(1)).unwrap(), &5);
        assert_eq!(game.tip_cards.len(), 14);
        assert_eq!(game.tip_cards.iter().filter(|card| card.color == Color::White).count(), 2);
    }

    #[test]
    fn test_get_tip_card_error() {
        let mut game = CamelCup::a_3_player_new_game();
        game.move_tip_card_player(0, &Color::White).unwrap();
        game.move_tip_card_player(0, &Color::White).unwrap();
        game.move_tip_card_player(0, &Color::White).unwrap();
        assert_eq!(game.move_tip_card_player(0, &Color::White), Err("No more cards of this color left"));
    }

    #[test]
    fn test_evaluation() {
        let mut tip_card = TipCard::new(&Color::White, &HashMap::from([
            (Place::Top(1), 5),
            (Place::Top(2), 1),
            (Place::BottomRemaining, -1),
        ])).unwrap();
        let camels = Camel::new_vec(&mut vec![Color::White, Color::Red, Color::Green, Color::Blue]);
        assert_eq!(tip_card.evaluate(&camels), 5);
        tip_card.color = Color::Red;
        assert_eq!(tip_card.evaluate(&camels), 1);
        tip_card.color = Color::Green;
        assert_eq!(tip_card.evaluate(&camels), -1);
        tip_card.color = Color::Blue;
        assert_eq!(tip_card.evaluate(&camels), -1);
    }
}