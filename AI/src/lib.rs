pub mod neuron;
pub mod neuron_network;
pub mod camel_cup_extensions;

pub use camel_cup::CamelCup;

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
