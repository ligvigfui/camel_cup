use std::{env, fs::{self, File}, io::{self, Read, Write}, sync::{Arc, Mutex}, thread};

use AI::neuron_network::NeuronNetwork;
use ai_camel_cup::save_and_exit;
use rand::Rng;

pub mod ai_camel_cup {
    use std::{sync::{Arc, Mutex}, fs::File, io::Write};

    use AI::{camel_cup_extensions::Shit, neuron_network::NeuronNetwork, CamelCup};
    use rand::Rng;

    pub fn game(ai_players: &Vec<NeuronNetwork>) -> Vec<f64> {
        let mut game = CamelCup::a_n_player_game(ai_players.len());
        let mut turns = 0.0;
        let mut player_turns = 0;
        while !game.end_game_check() {
            while !game.end_turn_check() {
                let input = game.game_state_to_input();
                let output = ai_players[game.current_player].get_output(input);
                output_to_action(output, &mut game);
                game.next_player();
                player_turns += 1;
                if player_turns > 50 {
                    break;
                }
            }
            player_turns = 0;
            game.evaluate_end_turn();
            turns += 1.0;
            if turns > 200.0 {
                break;
            }
        }
        game.end_game_evaluate(false);
        //gets mony for each player same orcer as players
        let score = game.game_winners_ai();
        let mut new_score = Vec::new();
        //gets (i, score/turn) for each player to new_score
        for i in 0..score.len() {
            new_score.push((i as usize, (score[i] as f64) / turns));
        }
        //sorts new_score by score/turn
        new_score.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for i in 0..new_score.len() {
            new_score[i].1 *= 5.0 - 2.5*(i/(new_score.len()-1)) as f64;
        }
        //sorts new_score by player number 0 to n
        new_score.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut final_score = Vec::new();
        for i in 0..new_score.len() {
            final_score.push(new_score[i].1);
        }
        final_score
    }

    pub fn save_and_exit(neural_networks: &Arc<Mutex<Option<Vec<NeuronNetwork>>>>, gen: usize) {
        println!("starting save");
        //save the neural networks
        let mut file = File::create(format!("generations/{}.json", gen)).unwrap();
        let networks = neural_networks.lock().unwrap().take().unwrap();
        println!("networks {:?}", networks.len());
        file.write(serde_json::to_string(&networks).unwrap().as_bytes()).unwrap();
        println!("finished save");
        std::process::exit(0);
    }

    pub fn output_to_action(output: Vec<f64>, game: &mut CamelCup) {
        let mut ordered_output = Vec::new();
        for i in 0..output.len() {
            ordered_output.push((i, output[i]));
        }
        ordered_output.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for i in 0..ordered_output.len() {
            match 
                match ordered_output[i].0 {
                    0 => game.rand_move_camel(),
                    1..=15 => game.place_card(ordered_output[i].0 as u8 + 1, true),
                    16..=30 => game.place_card(ordered_output[i].0 as u8 - 14, false),
                    31..=35 => game.move_tip_card(&game.camels[ordered_output[i].0-31].color.clone()),
                    36..=40 => game.end_game_bet(true, &game.camels[ordered_output[i].0-36].color.clone()),
                    41..=45 => game.end_game_bet(false, &game.camels[ordered_output[i].0-41].color.clone()),
                    _ => panic!("You messed up big time"),
                }
            {
                Ok(_) => {
                    //println!("action: {}", ordered_output[i].0);
                    return;
                },
                Err(_) => {
                    //println!("failed action {}", ordered_output[i].0);
                    continue;
                },
            }
        }
    }

    pub fn pick_networks( neural_networks_new_to_score: &Arc<Mutex<Option<Vec<(NeuronNetwork, f64)>>>>, neural_networks_new_score_sum: &Arc<Mutex<Vec<(NeuronNetwork, f64, f64)>>>, batch_size: usize) {
        let mut networks_new_to_score = neural_networks_new_to_score.lock().unwrap();
        networks_new_to_score.as_mut().unwrap().clear();
        let mut networks_new_score_sum = neural_networks_new_score_sum.lock().unwrap();
        for _ in 0..batch_size {
            let mut rng = rand::thread_rng();
            let range = 0.0..networks_new_score_sum.last().unwrap().2;
            let random_number = rng.gen_range(range);
            //goed through all the networks
            for _ in 0..networks_new_score_sum.len() {
                if networks_new_score_sum[0].2 > random_number {
                    networks_new_to_score.as_mut().unwrap().push((networks_new_score_sum[0].0.clone(), networks_new_score_sum[0].1));
                    break;
                }
            }
        }
        networks_new_score_sum.clear();
    }

}





fn main() {
    //reinforcment learning
    
    //! fix missing NN from last gen
    //! load in data
    //! weighting function ?
    //! mutate function
    //! show time with each gen
    //! move 10% timer to threads
    
    env::set_var("RUST_BACKTRACE", "1");
    let thread_number = 8;
    let player_number = 8;
    let batch_size = 8000;
    let debug = false;
    //get current dir
    let env = env::current_dir().unwrap();
    if !env.ends_with("AI") {
        while let Some(component) = env.parent() {
            if component.ends_with("AI") {
                env::set_current_dir(component).unwrap();
                break;
            }
        }
    }

    let paths = fs::read_dir(env::current_dir().unwrap().as_path().join("generations")).unwrap();
    let mut gen = 0;
    for path in paths {
        let a = match path.unwrap().path().display().to_string().split("/").last().unwrap().split(".").next().unwrap().parse::<usize>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if a > gen {
            gen = a;
        }
    }

    let go_on = true;
    let neural_networks = Arc::new(Mutex::new(Some(Vec::<NeuronNetwork>::new())));
    let neural_networks_new_to_score = Arc::new(Mutex::new(Some(Vec::<(NeuronNetwork, f64)>::new())));
    let neural_networks_new_score_sum = Arc::new(Mutex::new(Vec::<(NeuronNetwork, f64, f64)>::new()));


    if gen != 0 {
        // todo tomorrow
    } else {
        println!("starting to create networks:");
        let mut handles = vec![];
        for _ in 0..thread_number {
            let network = Arc::clone(&neural_networks);
            let handle = thread::spawn(move || {
                loop {
                    let nn = NeuronNetwork::new(86, 46);
                    let mut networks = network.lock().unwrap();
                    if networks.as_mut().unwrap().len()  % (batch_size/10)== 0 {
                        print!("=");
                    }
                    if networks.as_mut().unwrap().len() == batch_size {
                        break;
                    }
                    networks.as_mut().unwrap().push(nn);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    let mut to_file = String::new();

    while go_on {
        
        //read the command.txt file
        let mut file = File::open("command.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        //check if the user wants to continue
        match contents.trim() {
            "c" => println!("\nStarting gen {}\n", &gen),
            "se" => {
                save_and_exit(&neural_networks, gen);
            }
            _ => panic!("please enter c/se in command.txt to continue"),
        }
        


        
    
    
        //84000 networks DONE
        //create or read in neural networks DONE
        //read human input DONE
        //start threads DONE
        //start games in threads DONE
        //get output from threads DONE
        let mut handles = vec![];
        println!("Gen {} starting to play games:", &gen);
        for _ in 0..thread_number {
            let neural_networks = Arc::clone(&neural_networks);
            let neuron_network_new_to_score = Arc::clone(&neural_networks_new_to_score);
            let handle = thread::spawn(move || {
                loop {
                    let mut players = Vec::<NeuronNetwork>::new();
                    {
                        let mut networks = neural_networks.lock().unwrap();
                        if networks.as_ref().unwrap().len() < player_number {
                            return;
                        }
                        for _ in 0..player_number {
                            let mut rng = rand::thread_rng();
                            let range = 0..networks.as_ref().unwrap().len();
                            let random_number = rng.gen_range(range);
                            players.push(networks.as_mut().unwrap().remove(random_number));
                        }
                    }
                    let game = ai_camel_cup::game(&players);
                    {
                        let mut networks_new = neuron_network_new_to_score.lock().unwrap();
                        for i in (0..players.len()).rev() {
                            networks_new.as_mut().unwrap().push((players.pop().unwrap(), game[i]));
                        }
                    }                
                }
            });
            handles.push(handle);
        }
        //wait for threads to finish DONE
        for handle in handles {
            handle.join().unwrap();
        }

        //_____________________________________________________________creating next gen_____________________________________________________________

        //sort by score
        println!("Gen {} calculating overall winer\n\t-ordering players", gen);
        //order by score, get maximum score DONE
        let max: f64;
        {
            let mut networks_new_to_score = neural_networks_new_to_score.lock().unwrap();
            networks_new_to_score.as_mut().unwrap().sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            max = networks_new_to_score.as_mut().unwrap().last().unwrap().1;
        }
        //add score to score sum
        println!("\t-calculating added score");
        let avarege: f64;
        {
            // pop from nn_n_t_s
            // let temp = (nn_n_t_s.pop().unwrap())
            //let temp2 = (temp.0, temp.1, temp.1+ nn_n_s_s.last().unwrap().2)
            let mut networks_new_to_score = neural_networks_new_to_score.lock().unwrap();
            let mut networks_new_score_sum = neural_networks_new_score_sum.lock().unwrap();
            let temp = networks_new_to_score.as_mut().unwrap().pop().unwrap();
            let temp2 = (temp.0, temp.1, temp.1);
            networks_new_score_sum.push(temp2);
            for _ in 0..networks_new_to_score.as_mut().unwrap().len() {
                let temp = networks_new_to_score.as_mut().unwrap().pop().unwrap();
                let temp2 = (temp.0, temp.1, temp.1+ networks_new_score_sum.last().unwrap().2);
                networks_new_score_sum.push(temp2);
            }
            assert_eq!(networks_new_score_sum[0].1, max);
            avarege = networks_new_score_sum.last().unwrap().2 / batch_size as f64;
        }
        //get max and avarege DONE
        println!("Gen {} max: {}, avarege score: {}", gen, max, avarege);
        //println!("CHECK 0 nnnts {} nnnss {} nn {}", neural_networks_new_to_score.lock().unwrap().as_ref().unwrap().len(), neural_networks_new_score_sum.lock().unwrap().len(), neural_networks.lock().unwrap().as_ref().unwrap().len());
        {
            // todo this in paralell
            let mut networks_new_to_score = neural_networks_new_to_score.lock().unwrap();
            let mut networks_new_score_sum = neural_networks_new_score_sum.lock().unwrap();
            let range = batch_size-batch_size/player_number..batch_size;
            //batch_size/player_number amunt of networs is cloned to the next gen DONE
            println!("\t-cloning winers");
            for i in range.rev() {
                networks_new_to_score.as_mut().unwrap().push((networks_new_score_sum[i].0.clone(), networks_new_score_sum[i].1));
            }
            println!("\t-picking random networks");
            //randomm pick the rest of the networks from pool
            // todo with O(log n) instead of O(n) (binary search)
            for _ in networks_new_to_score.as_ref().unwrap().len()..batch_size {
                let mut rng = rand::thread_rng();
                let range = 0.0..networks_new_score_sum.last().unwrap().2;
                let random_number = rng.gen_range(range);
                //goes through all the networks
                for i in 0..networks_new_score_sum.len() {
                    if networks_new_score_sum[i].2 > random_number {
                        networks_new_to_score.as_mut().unwrap().push((networks_new_score_sum[i].0.clone(), networks_new_score_sum[i].1));
                        break;
                    } 
                }
            }
            //println!("CHECK 1 nntss {} nnnss {} nn {}", networks_new_to_score.as_ref().unwrap().len(), networks_new_score_sum.len(), neural_networks.lock().unwrap().as_ref().unwrap().len());
            networks_new_score_sum.clear();
        }
        //println!("CHECK 2 nnnts {} nnnss {} nn {}", neural_networks_new_to_score.lock().unwrap().as_ref().unwrap().len(), neural_networks_new_score_sum.lock().unwrap().len(), neural_networks.lock().unwrap().as_ref().unwrap().len());










        
        // move to next gen
        println!("\t-skiping first {} networks", batch_size/player_number/2);
        //skip mutating the first batch*2/player_number networks
        for i in 0..batch_size/2/player_number {
            let mut networks_new_to_score = neural_networks_new_to_score.lock().unwrap();
            let mut networks = neural_networks.lock().unwrap();
            networks.as_mut().unwrap().push(networks_new_to_score.as_mut().unwrap().remove(i).0);
        }
        //mutate networks
        //update neural networks
        println!("Gen {} Starting to mutate networks", gen);
        let mut handles = vec![];
        for _ in 0..thread_number {
            let neural_networks = Arc::clone(&neural_networks);
            let neuron_network_new_to_score2 = Arc::clone(&neural_networks_new_to_score);
            let handle = thread::spawn(move || {
                loop {
                    //
                    let mut network2: NeuronNetwork;
                    let i: usize;
                    {
                        let mut networks_new = neuron_network_new_to_score2.lock().unwrap();
                        if networks_new.as_mut().unwrap().len() == 0 {
                            break;
                        }
                        let (network, _i) = networks_new.as_mut().unwrap().pop().unwrap();
                        i = networks_new.as_mut().unwrap().len();
                        network2 = network;
                    }
                    network2.mutate(i as f64 /batch_size as f64);
                    {
                        let mut networks = neural_networks.lock().unwrap();
                        networks.as_mut().unwrap().push(network2);
                    }
                }
            });
            handles.push(handle);
        }
        //wait for threads to finish DONE
        for handle in handles {
            handle.join().unwrap();
        }

        
        //save gen max and avarege to file data.txt witouth overwriting 
        let mut file = File::create("data.txt").unwrap();
        to_file.push_str(format!("Gen {} max: {} avarege score: {}\n", gen, max, avarege).as_str());
        file.write_all(to_file.as_bytes()).unwrap();
        if debug {
            let mut st = String::new();
            io::stdin().read_line(&mut st).unwrap();
        }


        gen += 1;
    }



}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pick_networks() {
    }
}