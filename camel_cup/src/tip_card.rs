use crate::*;

#[derive(Debug)]
pub struct TipCard {
    pub(crate) color: Color,
    pub(crate) value: HashMap<Place, i8>,
}

impl TipCard {
    pub fn new(color: Color, value: HashMap<Place, i8>) -> TipCard {
        TipCard { color, value }
    }

    pub fn new_vec(colors: Vec<Color>, values: Vec<HashMap<Place, i8>>) -> Vec<Vec<TipCard>> {
        let mut tip_cards = Vec::new();
        for color in colors {
            let mut color_tip_cards = Vec::new();
            for value in values {
                color_tip_cards.push(TipCard::new(color, value));
            }
            tip_cards.push(color_tip_cards);
        }
        tip_cards
    }
}