use serde::{Deserialize, Serialize};

use crate::get_weight;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Neuron {
    pub inputs: usize,
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Neuron {
    pub fn new(inputs: usize) -> Neuron {
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

    pub fn add_weight(&mut self) {
        self.weights.push(get_weight());
    }

    pub fn mutate(&mut self, hardness: f64) {
        for weight in self.weights.iter_mut() {
            *weight += rand::random::<f64>()*2.0*hardness - 1.0*hardness;
        }
        self.bias *= rand::random::<f64>()*2.0*hardness - 1.0*hardness;
    }

    pub fn get_output(&self, inputs: &Vec<f64>) -> f64 {
        let mut output = 0.0;
        for i in 0..inputs.len() {
            output += inputs[i] * self.weights[i];
        }
        output += self.bias;
        output = 1.0 / (1.0 + (-output).exp());
        output
    }
}