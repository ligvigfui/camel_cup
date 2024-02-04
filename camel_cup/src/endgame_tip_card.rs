#[derive(Debug)]
pub struct EndgameTipcard {
    pub(crate) display_color: String,
    pub(crate) color: String,
    pub(crate) owner: usize,
}
pub fn setup_endgame_tipcards(player_number: usize) -> Vec<EndgameTipcard> {
    let mut endgame_tipcards = Vec::new();
    for i in 0..player_number {
        for color in vec![("white", "\x1B[1m"), ("blue", "\x1b[34m"), ("green","\x1b[32m"), ("yellow","\x1b[33m"), ("orange", "\x1b[38;2;255;165;0m")] {
            endgame_tipcards.push(EndgameTipcard {display_color: color.1.to_string(), color: color.0.to_string(), owner: i });
        }
    }
    endgame_tipcards
}