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

    pub fn new_vec(colors: Vec<Color>, player_number: usize) -> Vec<OwerallTipcard> {
        let mut endgame_tipcards = Vec::new();
        for i in 0..player_number {
            for color in colors {
                endgame_tipcards.push(Self::new(color, i));
            }
        }
        endgame_tipcards
    }
}