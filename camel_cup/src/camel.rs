use rand::Rng;

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

    pub fn new_vec(colors: &mut Vec<Color>) -> Vec<Camel> {
        let mut result = Vec::with_capacity(colors.len());
        colors.reverse();
        while let Some(color) = colors.pop() {
            result.push(Camel::new(color));
        }
        result
    }

    pub fn place(color: &Color, camels: &[Camel]) -> usize {
        for (i, camel) in camels.iter().enumerate() {
            if camel.color == *color {return i + 1;}
        }
        usize::MAX
    }
}

impl CamelCup {
    pub fn move_camel(&mut self, camel_color: Color, amount: i8) -> Result<(), &'static str> {
        if !self.dice_numbers.contains(&amount) {
            return Err("amount not in possible dice rolls");
        }
        if !self.camels.iter().any(|camel| camel.color == camel_color) {
            return Err("No camel with this color");
        }
        let (mut x, mut y) = (0, 0);
        let mut camels_above = Vec::new();
        let mut amount = amount;
        for (camel_number, camel) in self.camels.iter_mut().enumerate() {
            if camel.color == camel_color {
                //println!("{} {} {} {}", camel.x, camel.y, camel.moved, camel.color);
                if camel.moved {
                    return Err("camel already moved this turn");
                }
                if camel.x == 0 {
                    camels_above.push(camel_number);
                }
                x = camel.x;
                y = camel.y;
                camel.moved = true;
                
            }
        }
        //. x 0 y 0
        for (camel_number, camel) in self.camels.iter().enumerate() {
            if camel.x == x && x !=0 && camel.y >= y {
                camels_above.push(camel_number);
            }
        }
        //println!("{} {} {:?}", x, y, camels_above);
        let mut case1 = true;
        for player in self.players.iter_mut() {
            if player.placeable_card.x == add(x, amount) {
                player.money += 1;
                if player.placeable_card.faceup {
                    case1 = true;
                    amount += 1;
                } else {
                    case1 = false;
                    amount -= 1;
                }
            }
        }
        self.players[self.current_player].money += 1;
        let mut camels_below = 0;
        if case1 {
            //camels are moved above
            for camel in self.camels.iter_mut() {
                if camel.x == add(x, amount) {
                    camels_below += 1;
                }
            }
            for camel in camels_above.iter() {
                self.camels[*camel].x = add(x, amount);
                self.camels[*camel].y += camels_below - y;
            }
        } else {
            //camels are moved below 
            //elevates the moving camels
            for (camel_number, camel) in self.camels.iter_mut().enumerate() {
                if camel.x == add(x, amount) && !camels_above.contains(&camel_number){
                    camel.y += camels_above.len() as u8;
                }
            }
            //moves the moving camels below
            for camel in camels_above.iter() {
                //println!("{} {} {}", camel, self.camels[*camel].x, self.camels[*camel].y);
                self.camels[*camel].x = add(x, amount);
                self.camels[*camel].y -= y;
                //println!("{} {} {}", camel, self.camels[*camel].x, self.camels[*camel].y);
            }
        }
        self.order_camels();
        self.next_player();
        Ok(())
    }

    pub fn rand_move_camel(&mut self) -> Result<(), &'static str>{
        let random = rand::thread_rng().gen_range(0..self.not_moved_camels().len());
        let color = self.not_moved_camels().remove(random);
        let amount = rand::thread_rng().gen_range(1..4);
        self.move_camel(color, amount)
    }

    pub fn order_camels(&mut self){
        let camel_len: u8 = self.camels.len().try_into().unwrap();
        self.camels.sort_by(|first, second|{ 
            let first_position = first.x*camel_len+first.y; 
            let second_position = second.x*camel_len+second.y;
            second_position.cmp(&first_position)
        });
    }
    
    pub fn get_place(&self, color: &Color) -> usize {
        for (i, camel) in self.camels.iter().enumerate() {
            if camel.color == *color {return i;}
        }
        usize::MAX
    }

    pub(crate) fn set_camel_position(&mut self, camel_color: Color, x: u8, y: u8) {
        //! test fn only
        /*self.camels.iter().for_each(|(camel)| {
            /*if camel.x == x && camel.y == y {
                return Err("x and y are already occupied by some other camel");
            }*/
        });*/
        for camel in self.camels.iter_mut() {
            if camel.color == camel_color {
                camel.x = x;
                camel.y = y;
                camel.moved = true;
                return;
            }
        }
        panic!("No camel with this color");
    }

    pub(crate) fn camel_test_helper(&self, color: Color, assert_x: u8, assert_y: u8, assert_moved: bool) {
        let camel = self.camels.iter().find(|camel| camel.color == color).unwrap();
        assert_eq!(assert_x, camel.x);
        assert_eq!(assert_y, camel.y);
        assert_eq!(assert_moved, camel.moved);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place() {
        let mut colors = vec![Color::Red, Color::Green, Color::Blue];
        let camels = Camel::new_vec(&mut colors);
        assert_eq!(Camel::place(&Color::Red, &camels), 1);
        assert_eq!(Camel::place(&Color::Green, &camels), 2);
        assert_eq!(Camel::place(&Color::Blue, &camels), 3);
    }
    
    #[test]
    fn test_move_camel() {
        let mut game = CamelCup::a_3_player_new_game();
        //current player: 0
        assert!(game.move_camel(Color::White, 0).is_err());
        assert_eq!(game.move_camel(Color::BrightBlack, 1), Err("No camel with this color"));
        game.place_card(2, true).unwrap();
        //1
        game.place_card(4, false).unwrap();
        //2
        game.move_camel(Color::White, 2).unwrap();
        //0
        assert_eq!(game.camels[0].color, Color::White);
        assert_eq!(game.camels[0].x, 3);
        assert_eq!(game.camels[0].y, 0);
        assert_eq!(game.camels[0].moved, true);
        assert_eq!(game.players[0].money, 4);
        assert_eq!(game.players[2].money, 4);
        assert_eq!(game.move_camel(Color::White, 1), Err("camel already moved this turn"));
        game.move_camel(Color::Green, 3).unwrap();
        //1
        game.camel_test_helper(Color::Green, 3, 1, true);
        assert_eq!(game.players[0].money, 5);
        assert_eq!(game.players[2].money, 4);
        game.move_camel(Color::Blue, 1).unwrap();
        //2
        game.camel_test_helper(Color::Blue, 1, 0, true);
        assert_eq!(game.players[0].money, 5);
        assert_eq!(game.players[2].money, 4);
        assert_eq!(game.players[1].money, 5);
        for camel in game.camels.iter_mut() {
            camel.moved = false;
        }
        game.move_camel(Color::White, 1).unwrap();
        game.camel_test_helper(Color::White, 3, 0, true);
        game.camel_test_helper(Color::Green, 3, 1, false);
        game.camel_test_helper(Color::Blue, 1, 0, false);
    }
}