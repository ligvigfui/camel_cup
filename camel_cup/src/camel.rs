#[derive(Debug, Clone)]
pub struct Camel {
    pub(crate) display_color: String,
    pub(crate) color: String,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) moved: bool,
}
pub fn setup_camels () -> Vec<Camel> {
    let mut camels = Vec::new();
    for color in vec![("white", "\x1B[1m"), ("blue", "\x1b[34m"), ("green","\x1b[32m"), ("yellow","\x1b[33m"), ("orange", "\x1b[38;2;255;165;0m")] {
        camels.push(Camel {display_color: color.1.to_string(), color: color.0.to_string(), x: 0, y: 0, moved: false });
    }
    camels
}