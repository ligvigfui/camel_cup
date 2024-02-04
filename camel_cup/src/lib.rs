pub mod camel_cup;
pub mod camel;
pub mod colored_strings;
pub mod endgame_tip_card;
pub mod options;
pub mod place_card;
pub mod player;
pub mod tip_card;

pub use crate::{
    camel_cup::*,
    camel::*,
    colored_strings::*,
    endgame_tip_card::*,
    options::*,
    place_card::*,
    player::*,
    tip_card::*,
};

pub fn clear_screen() {
    print!("\x1B[2J");
}