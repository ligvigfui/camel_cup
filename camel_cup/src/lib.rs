pub mod camel_cup;
pub mod camel;
pub mod colored_strings;
pub mod owerall_tip_card;
pub mod options;
pub mod place_card;
pub mod place;
pub mod player;
pub mod tip_card;

pub use crate::{
    camel_cup::*,
    camel::*,
    colored_strings::*,
    owerall_tip_card::*,
    options::*,
    place_card::*,
    place::*,
    player::*,
    tip_card::*,
};

pub use std::collections::HashMap;

pub fn clear_screen() {
    print!("\x1B[2J");
}