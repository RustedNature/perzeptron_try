use crate::perzeptron::{self, *};
const INPUT_SIZE: u32 = 1920 * 1080;
#[test]
fn can_construct_perzeptron() {
    let perz = Perzeptron::new(INPUT_SIZE);
}

#[test]
fn correct_size_of_input_neurons() {
    let perz = Perzeptron::new(INPUT_SIZE);
    let num_of_input_neurons = perz.get_length_of_input_neurons_vec();
    assert!(num_of_input_neurons == INPUT_SIZE as usize)
}
#[test]
fn correct_size_of_weights() {
    let perz = Perzeptron::new(INPUT_SIZE);
    let num_of_weights = perz.get_length_of_weights_vec();
    assert!(num_of_weights == INPUT_SIZE as usize)
}

#[test]
fn get_expectect_output() {
    let mut perzeptron = Perzeptron::new(INPUT_SIZE);
    perzeptron.calculate_output();
    let out = perzeptron.get_output_neuron_value() == true;
    assert!(out)
}
