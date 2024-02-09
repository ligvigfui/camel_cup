use crate::*;

#[derive(Debug, Clone)]
pub struct Camel {
    pub(crate) color: Color,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) moved: bool,
}

impl Camel {
    pub fn new(color: Color) -> Camel {
        Camel {
            color,
            x: 0,
            y: 0,
            moved: false,
        }
    }

    pub fn new_vec(colors: Vec<Color>) -> Vec<Camel> {
        let mut result = Vec::with_capacity(colors.len());
        for color in colors {
            result.push(Camel::new(color));
        }
        result
    }
}