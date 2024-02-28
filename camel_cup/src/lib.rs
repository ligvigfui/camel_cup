pub mod camel_cup;
pub mod camel;
pub mod colored_strings;
pub mod overall_tip_card;
pub mod options;
pub mod place_card;
pub mod place;
pub mod player;
pub mod tip_card;

pub use crate::{
    camel_cup::*,
    camel::*,
    colored_strings::*,
    overall_tip_card::*,
    options::*,
    place_card::*,
    place::*,
    player::*,
    tip_card::*,
};

pub use std::collections::HashMap;
use std::io;

pub fn clear_screen() {
    print!("\x1B[2J");
}
pub fn read_usize(max: usize) -> Result<usize, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let error = format!("Please type a number between 0 and {max}");
    let input_num: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(error),
    };
    match input_num <= max {
        true => Ok(input_num),
        false => Err(error),
    }
}
pub fn add(number1: u8, number2: i8) -> u8 {
    match number1 as i16 > -number2 as i16 {
        true  => if number1 as i16 + number2 as i16 > u8::MAX as i16 {
            u8::MAX
        } else {
            ( number1 as i16 + number2 as i16 ) as u8
        },
        false => u8::MIN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(0, 2), 2);
        assert_eq!(add(2, -1), 1);
        assert_eq!(add(2, -2), 0);
        assert_eq!(add(2, -3), 0);
        assert_eq!(add(200, 100), u8::MAX);
    }
}