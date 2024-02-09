use crate::*;

#[derive(Debug)]
pub struct Player {
    pub(crate) name: String,
    pub(crate) money: u16,
    pub(crate) placeable_card: PlaceCard,
    pub(crate) tip_cards: Vec<TipCard>,
    pub(crate) owerall_tip_cards: Vec<OwerallTipcard>,
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
            placeable_card: PlaceCard::new(),
            tip_cards: Vec::new(),
            owerall_tip_cards: Vec::new(),
        }
    }

    pub fn new_vec(names: Vec<Option<String>>) -> Vec<Player> {
        let mut result = Vec::with_capacity(names.len());
        for (i, &name) in names.iter().enumerate() {
            result.push(Player::new(name, i));
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
}
