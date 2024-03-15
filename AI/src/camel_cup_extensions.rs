use camel_cup::{HashMap, Place};

pub(crate) use crate::*;

pub trait Shit {
    fn game_state_to_input(&self) -> Vec<f64>;
    fn game_winners_ai(&mut self) -> Vec<u16>;
}

impl Shit for CamelCup {
    fn game_state_to_input(&self) -> Vec<f64> {
        let mut inputs = Vec::new();
        //tipcards in camel order
        let mut tipcard_map: HashMap<camel_cup::Color, i8> = HashMap::new();
        for tip_card in &self.tip_cards {
            match tipcard_map.get_mut(&tip_card.color) {
                Some(number) => if *number < *tip_card.values.get(&Place::Top(1)).unwrap() {
                    *number = *tip_card.values.get(&Place::Top(1)).unwrap();
                }
                None => {
                    tipcard_map.insert(tip_card.color.clone(), *tip_card.values.get(&Place::Top(1)).unwrap());
                }
            }
        }
        //20 - camels position & moved
        for camel in &self.camels {
            inputs.push(camel.x as f64 / 16.0);
            inputs.push(camel.y as f64 / 5.0);
            inputs.push(camel.moved as i32 as f64);
            inputs.push(match tipcard_map.get(&camel.color) {
                Some(&number) => number as f64 / 5.0,
                None => 0.0
            });
        }
        //64 players with tipcards in camel order, place cards place and faceup and money
        fn add_player_informations(cc: &CamelCup, i: usize, inputs: &mut Vec<f64>) {
            inputs.push(cc.players[i].money as f64 / 50.0);
            for camel_colors in 0..5 {
                let mut tip_value = 0;
                for tip_card in &cc.players[i].tip_cards {
                    if tip_card.color == cc.camels[camel_colors].color {
                        tip_value += tip_card.values.get(&Place::Top(1)).unwrap()
                    }
                }
                inputs.push(tip_value as f64 / 10.0);
            }
            inputs.push(cc.players[i].placeable_card.x as f64 / 16.0);
            inputs.push(cc.players[i].placeable_card.faceup as i32 as f64);
        }
        for i in self.current_player..self.players.len() {
            add_player_informations(self, i, &mut inputs);
        }
        for i in 0..self.current_player {
            add_player_informations(self, i, &mut inputs);
        }
        for _ in self.players.len()*8..8*8 {
            inputs.push(0.0);
        }
        //overall winner count
        inputs.push(self.winer_overalltipcards.len() as f64 / 15.0);
        //overall loser count
        inputs.push(self.loser_overalltipcards.len() as f64 / 15.0);
        inputs
    }
    
    //_______________________________________________________________________________________
    fn game_winners_ai(&mut self) -> Vec<u16> {
        let mut points = Vec::new();
        for i in 0..self.players.len() {
            points.push(self.players[i].money);
        }
        points
    }
}