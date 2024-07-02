use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct Options {
    pub(crate) map_len: u8,
    pub(crate) player_names: Vec<Option<String>>,
    pub(crate) camel_colors: Vec<Color>,
    pub(crate) dice_numbers: Vec<i8>,
    pub(crate) leg_tips: Vec<HashMap<Place, i8>>,
    pub(crate) overall_tips_function: HashMap<Place, i8>,
}

impl Options {
    pub fn new(player_names: Vec<Option<String>>) -> Options {
        Options {
            map_len: 16,
            player_names,
            camel_colors: vec![
                Color::White,
                Color::Blue,
                Color::Green,
                Color::Yellow,
                Color::RGB(RGB::named("Orange", 255, 165, 0)),
            ],
            dice_numbers: vec![1, 2, 3],
            leg_tips: vec![
                HashMap::from([
                    (Place::Top(1), 5),
                    (Place::Top(2), 1),
                    (Place::BottomRemaining, -1)
                ]),
                HashMap::from([
                    (Place::Top(1), 3),
                    (Place::Top(2), 1),
                    (Place::BottomRemaining, -1)
                ]),
                HashMap::from([
                    (Place::Top(1), 2),
                    (Place::Top(2), 1),
                    (Place::BottomRemaining, -1)
                ]),
            ],
            overall_tips_function:
                HashMap::from([
                    (Place::Top(1), 8),
                    (Place::Top(2), 5),
                    (Place::Top(3), 3),
                    (Place::Top(4), 2),
                    (Place::BottomRemaining, 1),
                    (Place::Wrong, -1)
                ]),
        }
    }
}