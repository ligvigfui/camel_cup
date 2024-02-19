

impl CamelCup {
    pub fn game_state_to_input(&self) -> Vec<f64> {
        let mut inputs = Vec::new();
        //24 inputs
        for i in 0..8 {
            if i > self.players.len()-1 {
                inputs.push(0 as f64);
                inputs.push(0 as f64);
                inputs.push(0 as f64);
                continue;}
            inputs.push(self.players[i].placeable_card.faceup as i32 as f64);
            inputs.push(self.players[i].money as f64/100 as f64);
            inputs.push(self.players[i].placeable_card.x as f64);
        }
        //24 + 10 = 34 inputs
        let mut camels = self.camels.clone();
        camels.sort_by(|a, b| a.color.partial_cmp(&b.color).unwrap());
        for i in 0..camels.len() {
            inputs.push((self.camels[i].x/self.options.map_len) as f64);
            inputs.push((self.camels[i].y/self.camels.len() as u8) as f64);
        }
        //34+5 = 39 inputs
        for color in &self.tip_cards {
            match color.last() {
                Some(card) => {
                    inputs.push((card.value.get(&Place::Top(1)).unwrap() / 5) as f64);
                },
                None => {
                    inputs.push(0 as f64);
                }
            }
        }
        //39+2 = 41 inputs
        inputs.push((self.winer_oweralltipcards.len()/5) as f64);
        inputs.push((self.loser_oweralltipcards.len()/5) as f64);
        inputs
    }
    
    //_______________________________________________________________________________________
    pub fn game_winners_ai(&mut self) -> Vec<u16> {
        let mut points = Vec::new();
        for i in 0..self.players.len() {
            points.push(self.players[i].money);
        }
        points
    }
}