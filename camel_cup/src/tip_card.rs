use crate::*;

#[derive(Debug)]
pub struct TipCard {
    pub(crate) color: Color,
    pub(crate) value: HashMap<Place, i8>,
}

impl TipCard {
    pub fn new(color: &Color, value: HashMap<Place, i8>) -> Result<TipCard, String> {
        // trow error if value contains more then 1 remaining
        let mut remaining = 0;
        for (place, _) in &value {
            match place {
                Place::BottomRemaining |
                Place::TopRemaining |
                Place::BottomRemaining => {
                    remaining += 1;
                }
                _ => {}
            }
        }
        if remaining > 1 {
            return Err("More then 1 remaining place in tip card".to_string());
        }
        Ok(TipCard {
            color: color.clone(),
            value
        })
    }

    pub fn new_vec(colors: &Vec<Color>, values: Vec<HashMap<Place, i8>>) -> Result<Vec<TipCard>, String> {
        let mut tip_cards = Vec::new();
        for color in colors {
            for value in values {
                tip_cards.push(TipCard::new(color, value)?);
            }
        }
        Ok(tip_cards)
    }

    pub fn evaluate(&self, camels: &[Camel]) -> i8 {
        let mut result = 0;
        for (place, value) in &self.value {
            match place {
                Place::Top(n) => {
                    if let Some(camel) = camels.iter().find(|camel| camel.color == self.color && camel.y == *n) {
                        result += value;
                    }
                }
                Place::BottomRemaining => {
                    for camel in camels.iter().filter(|camel| camel.color == self.color) {
                        result += value;
                    }
                }
                _ => {}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluation() {
        let mut tip_card = TipCard::new(&Color::White, vec![
            (Place::Top(1), 5),
            (Place::Top(2), 1),
            (Place::BottomRemaining, -1),
        ].into_iter().collect()).unwrap();
        let camels = Camel::new_vec(&vec![Color::White, Color::Red, Color::Green, Color::Blue]);
        assert_eq!(tip_card.evaluate(&camels), 5);
        tip_card.color = Color::Red;
        assert_eq!(tip_card.evaluate(&camels), 1);
        tip_card.color = Color::Green;
        assert_eq!(tip_card.evaluate(&camels), -1);
        tip_card.color = Color::Blue;
        assert_eq!(tip_card.evaluate(&camels), -1);
    }
}