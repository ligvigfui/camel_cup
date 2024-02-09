#[derive(Debug)]
pub struct PlaceCard {
    pub(crate) x: u8,
    pub(crate) faceup: bool,
}
impl PlaceCard {
    pub fn new() -> Self {
        PlaceCard {
            x: 0,
            faceup: false,
        }
    }
}