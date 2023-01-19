//disable unused warnings
#![allow(non_snake_case)]
#![allow(dead_code)]





// todo
// make neurons DONE
// make calculations    DONE
// mutate neurons   ?
// save and load neuron networks    DONE
// mutate neuron numbers    DONE
// mutate neuron layers     DONE
// 


pub mod neuron_network {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct NeuronNetwork {
        neurons: Vec<Vec<Neuron>>,
    }
    impl NeuronNetwork {
        pub fn new_empty() -> NeuronNetwork {
            NeuronNetwork {
                neurons: Vec::new(),
            }
        }

        pub fn new(inputs: usize, outputs: usize) -> NeuronNetwork {
            let mut layers = Vec::new();
            let mut neurons = Vec::new();
            for _ in 0..outputs {
                neurons.push(Neuron::new(inputs));
            }
            layers.push(neurons);
            NeuronNetwork {
                neurons: layers,
            }
        }
        fn add_layer(&mut self) {
            // get middle
            let middle = self.neurons.len()/2;
            // get number of outputs
            let inputs: usize;
            let layer: usize;
            if middle == 0 {
                layer = self.neurons[0][0].inputs;
                inputs = self.neurons[0][0].inputs;
            } else {
                layer = self.neurons[middle].len();
                inputs = self.neurons[middle-1].len();
            }
            // create new layer
            let mut neurons = Vec::new();
            // add neurons to layer
            for _ in 0..layer {
                neurons.push(Neuron::new(inputs));
            }
            // correct next layer weights
            for neuron in self.neurons[middle].iter_mut() {
                while neuron.weights.len() < layer {
                    neuron.weights.push(get_weight());
                }
                while neuron.weights.len() > layer {
                    neuron.weights.pop();
                }
            }
            // add layer to network
            self.neurons.insert(middle, neurons);

        }

        
        fn add_neuron_to_layer(&mut self, layer: usize) {
            let inputs: usize;
            if layer == 0 {
                inputs = self.neurons[0][0].inputs;
            } else {
                inputs = self.neurons[layer-1].len();
            }
            self.neurons[layer].push(Neuron::new(inputs));
            if self.neurons.len() > layer+1 {
                for neuron in self.neurons[layer+1].iter_mut() {
                    neuron.add_weight();
                }
            }
        }

        pub fn mutate(&mut self, hardness: f64) {
            if rand::random::<f64>() < hardness*0.01 {
                self.add_layer();
            }
            for i in 0..self.neurons.len()-1 {
                if rand::random::<f64>() < hardness*0.1 {
                    self.add_neuron_to_layer(i);
                }
            }
            //? remove neurons or layers
            for layer in self.neurons.iter_mut() {
                for neuron in layer.iter_mut() {
                    neuron.mutate(hardness);
                }
            }
        }

        pub fn get_output(&self, inputs: Vec<f64>) -> Vec<f64> {
            //iter through layers
            let mut layer_inputs = inputs;
            for layer in self.neurons.iter() {
                let mut layer_outputs = Vec::new();
                //iter through neurons
                for neuron in layer.iter() {
                    //get output of neuron
                    layer_outputs.push(neuron.get_output(&layer_inputs));
                }
                //set layer inputs to layer outputs
                layer_inputs = layer_outputs;
            }
            layer_inputs
        }


    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Neuron {
        inputs: usize,
        weights: Vec<f64>,
        bias: f64,
    }

    impl Neuron {
        fn new(inputs: usize) -> Neuron {
            let mut weights = Vec::new();
            for _ in 0..inputs {
                weights.push(get_weight());
            }
            let neuron = Neuron {
                inputs,
                weights,
                bias: get_weight(),
            };
            neuron
        }

        fn add_weight(&mut self) {
            self.weights.push(get_weight());
        }

        fn mutate(&mut self, hardness: f64) {
            for weight in self.weights.iter_mut() {
                *weight += rand::random::<f64>()*2.0*hardness - 1.0*hardness;
            }
            self.bias *= rand::random::<f64>()*2.0*hardness - 1.0*hardness;
        }

        fn get_output(&self, inputs: &Vec<f64>) -> f64 {
            let mut output = 0.0;
            for i in 0..inputs.len() {
                output += inputs[i] * self.weights[i];
            }
            output += self.bias;
            output = 1.0 / (1.0 + (-output).exp());
            output
        }
    }
    fn get_weight() -> f64 {
        rand::random::<f64>()*2.0 - 1.0
    }


    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_add_layer_to_network() {
            let mut network = NeuronNetwork::new(2, 3);
            assert_eq!(network.neurons.len(), 1);
            assert_eq!(network.neurons[0].len(), 3);
            network.add_layer();
            assert_eq!(network.neurons.len(), 2);
            assert_eq!(network.neurons[0].len(), 2);
            assert_eq!(network.neurons[1].len(), 3);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[1][0].weights.len(), 2);

            network = NeuronNetwork::new(3, 2);
            assert_eq!(network.neurons.len(), 1);
            assert_eq!(network.neurons[0].len(), 2);
            network.add_layer();
            assert_eq!(network.neurons.len(), 2);
            assert_eq!(network.neurons[0].len(), 3);
            assert_eq!(network.neurons[1].len(), 2);
            assert_eq!(network.neurons[0][0].weights.len(), 3);
            assert_eq!(network.neurons[0][1].weights.len(), 3);
            assert_eq!(network.neurons[0][2].weights.len(), 3);
            assert_eq!(network.neurons[1][0].weights.len(), 3);
            assert_eq!(network.neurons[1][1].weights.len(), 3);
            network.add_layer();
            assert_eq!(network.neurons.len(), 3);
            assert_eq!(network.neurons[0].len(), 3);
            assert_eq!(network.neurons[1].len(), 2);
            assert_eq!(network.neurons[2].len(), 2);
            assert_eq!(network.neurons[0][0].weights.len(), 3);
            assert_eq!(network.neurons[0][1].weights.len(), 3);
            assert_eq!(network.neurons[0][2].weights.len(), 3);
            assert_eq!(network.neurons[1][0].weights.len(), 3);
            assert_eq!(network.neurons[1][1].weights.len(), 3);
            assert_eq!(network.neurons[2][0].weights.len(), 2);
            assert_eq!(network.neurons[2][1].weights.len(), 2);

        }
    
        #[test]
        fn add_neuron_to_layer(){
            let mut network = NeuronNetwork::new(2, 3);
            assert_eq!(network.neurons.len(), 1);
            assert_eq!(network.neurons[0].len(), 3);
            network.add_neuron_to_layer(0);
            assert_eq!(network.neurons.len(), 1);
            assert_eq!(network.neurons[0].len(), 4);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[0][2].weights.len(), 2);
            assert_eq!(network.neurons[0][3].weights.len(), 2);
            network.add_layer();
            assert_eq!(network.neurons.len(), 2);
            assert_eq!(network.neurons[0].len(), 2);
            assert_eq!(network.neurons[1].len(), 4);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[1][0].weights.len(), 2);
            assert_eq!(network.neurons[1][1].weights.len(), 2);
            assert_eq!(network.neurons[1][2].weights.len(), 2);
            assert_eq!(network.neurons[1][3].weights.len(), 2);
            network.add_neuron_to_layer(1);
            assert_eq!(network.neurons.len(), 2);
            assert_eq!(network.neurons[0].len(), 2);
            assert_eq!(network.neurons[1].len(), 5);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[1][0].weights.len(), 2);
            assert_eq!(network.neurons[1][1].weights.len(), 2);
            assert_eq!(network.neurons[1][2].weights.len(), 2);
            assert_eq!(network.neurons[1][3].weights.len(), 2);
            assert_eq!(network.neurons[1][4].weights.len(), 2);
            network.add_neuron_to_layer(0);
            assert_eq!(network.neurons.len(), 2);
            assert_eq!(network.neurons[0].len(), 3);
            assert_eq!(network.neurons[1].len(), 5);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[0][2].weights.len(), 2);
            assert_eq!(network.neurons[1][0].weights.len(), 3);
            assert_eq!(network.neurons[1][1].weights.len(), 3);
            assert_eq!(network.neurons[1][2].weights.len(), 3);
            assert_eq!(network.neurons[1][3].weights.len(), 3);
            assert_eq!(network.neurons[1][4].weights.len(), 3);
            network.add_layer();
            assert_eq!(network.neurons.len(), 3);
            assert_eq!(network.neurons[0].len(), 3);
            assert_eq!(network.neurons[1].len(), 5);
            assert_eq!(network.neurons[2].len(), 5);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[0][2].weights.len(), 2);
            assert_eq!(network.neurons[1][0].weights.len(), 3);
            assert_eq!(network.neurons[1][1].weights.len(), 3);
            assert_eq!(network.neurons[1][2].weights.len(), 3);
            assert_eq!(network.neurons[1][3].weights.len(), 3);
            assert_eq!(network.neurons[1][4].weights.len(), 3);
            assert_eq!(network.neurons[2][0].weights.len(), 5);
            assert_eq!(network.neurons[2][1].weights.len(), 5);
            assert_eq!(network.neurons[2][2].weights.len(), 5);
            assert_eq!(network.neurons[2][3].weights.len(), 5);
            assert_eq!(network.neurons[2][4].weights.len(), 5);
            network.add_neuron_to_layer(1);
            assert_eq!(network.neurons.len(), 3);
            assert_eq!(network.neurons[0].len(), 3);
            assert_eq!(network.neurons[1].len(), 6);
            assert_eq!(network.neurons[2].len(), 5);
            assert_eq!(network.neurons[0][0].weights.len(), 2);
            assert_eq!(network.neurons[0][1].weights.len(), 2);
            assert_eq!(network.neurons[0][2].weights.len(), 2);
            assert_eq!(network.neurons[1][0].weights.len(), 3);
            assert_eq!(network.neurons[1][1].weights.len(), 3);
            assert_eq!(network.neurons[1][2].weights.len(), 3);
            assert_eq!(network.neurons[1][3].weights.len(), 3);
            assert_eq!(network.neurons[1][4].weights.len(), 3);
            assert_eq!(network.neurons[1][5].weights.len(), 3);
            assert_eq!(network.neurons[2][0].weights.len(), 6);
            assert_eq!(network.neurons[2][1].weights.len(), 6);
            assert_eq!(network.neurons[2][2].weights.len(), 6);
            assert_eq!(network.neurons[2][3].weights.len(), 6);
            assert_eq!(network.neurons[2][4].weights.len(), 6);



        }
    }
}
