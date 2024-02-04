#[derive(Debug)]
pub struct Options {
    pub(crate) map_len: u8,
    pub(crate) player_count: u8,
    pub(crate) camel_count: u8,
    pub(crate) dice_max_number: i8,
    pub(crate) tip_card_count: u8,
    pub(crate) endgame_tip_card_count: u8,
}
impl Options {
    pub fn new() -> Options {
        Options {
            map_len: 16,
            player_count: 2,
            camel_count: 5,
            dice_max_number: 3,
            tip_card_count: 3,
            endgame_tip_card_count: 5,
        }
    }
}