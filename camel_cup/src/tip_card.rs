use crate::Color;

#[derive(Debug)]
pub struct TipCard {
    pub(crate) display_color: String,
    pub(crate) color: String,
    pub(crate) value: i32,
}

pub fn setup_tip_cards() -> Vec<Vec<TipCard>> {
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

impl TipCard {
    pub fn new(color: Color, value: i32) -> TipCard {
        TipCard { color, value }
    }

    pub fn new_vec(colors: Vec<Color>, values: Vec<i32>) -> Vec<TipCard> {
        let mut tip_cards = Vec::new();
        for color in colors {
            for value in values.clone() {
                tip_cards.push(TipCard::new(color, value));
            }
        }
        tip_cards
    }
}