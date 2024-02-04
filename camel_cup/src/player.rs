use crate::*;

#[derive(Debug)]
pub struct Player {
    pub(crate) name: String,
    pub(crate) money: i32,
    pub(crate) placeable_card: PlaceCard,
    pub(crate) owned_tip_cards: Vec<TipCard>,
}
impl Player {
    pub fn new(name: Option<String>, number: usize) -> Player {
        let name = match name {
            Some(name) => name,
            None => format!("Player {}", number),
        };
        Player {
            name,
            money: 3,
            placeable_card: PlaceCard { x: 0, faceup: false },
            owned_tip_cards: Vec::new(),
        }
    }
}
