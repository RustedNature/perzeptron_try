pub struct Perzeptron {
    input_neurons: Vec<InputNeuron>,
    weights: Vec<Weight>,
    output_neuron: OutputNeuron,
}
impl Perzeptron {
    pub fn new(num_of_inputs: u32) -> Self {
        let mut input_neurons: Vec<InputNeuron> = vec![];
        let mut weights: Vec<Weight> = vec![];
        for _ in 0..num_of_inputs {
            input_neurons.push(InputNeuron::new());
            weights.push(Weight::new());
        }
        Perzeptron {
            input_neurons,
            weights,
            output_neuron: OutputNeuron::new(),
        }
    }

    pub fn get_length_of_input_neurons_vec(&self) -> usize {
        self.input_neurons.len()
    }
    pub fn get_length_of_weights_vec(&self) -> usize {
        self.weights.len()
    }
}

struct InputNeuron {
    value: i32,
}
impl InputNeuron {
    pub fn new() -> Self {
        InputNeuron { value: 0 }
    }
}

struct OutputNeuron {
    value: bool,
}
impl OutputNeuron {
    pub fn new() -> Self {
        OutputNeuron { value: false }
    }
}

struct Weight {
    weight_factor: f64,
}
impl Weight {
    pub fn new() -> Self {
        Weight { weight_factor: 0.0 }
    }
}
