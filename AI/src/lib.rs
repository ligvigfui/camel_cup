pub mod neuron;
pub mod neuron_network;
pub mod camel_cup_extensions;

pub use camel_cup::CamelCup;

fn get_weight() -> f64 {
    rand::random::<f64>()*2.0 - 1.0
}