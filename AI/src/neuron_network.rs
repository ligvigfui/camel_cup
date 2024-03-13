use serde::{Serialize, Deserialize};

use crate::{get_weight, neuron::Neuron};

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
