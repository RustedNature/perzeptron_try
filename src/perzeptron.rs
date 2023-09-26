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

    pub fn calculate_output(&mut self) {
        let mut output_value: f64 = 0.0;
        for (input_neuron_number, input_neuron) in self.input_neurons.iter().enumerate() {
            output_value += input_neuron.get_input_value()
                * self.weights[input_neuron_number].get_wight_factor();
        }
        self.output_neuron.evaluate_output(output_value);
    }

    pub fn get_output_neuron_value(&self) -> bool {
        self.output_neuron.get_output()
    }
}

struct InputNeuron {
    value: f64,
}
impl InputNeuron {
    pub fn new() -> Self {
        InputNeuron { value: 1.0 }
    }
    pub fn get_input_value(&self) -> f64 {
        self.value
    }
}

struct OutputNeuron {
    value: bool,
    theta: f64,
}
impl OutputNeuron {
    pub fn new() -> Self {
        OutputNeuron {
            value: false,
            theta: 1.0,
        }
    }

    pub fn get_output(&self) -> bool {
        self.value
    }

    pub fn evaluate_output(&mut self, value: f64) {
        if value >= self.theta {
            self.value = true;
        }
    }
}

struct Weight {
    weight_factor: f64,
}
impl Weight {
    pub fn new() -> Self {
        Weight {
            weight_factor: 0.5785,
        }
    }

    pub fn get_wight_factor(&self) -> f64 {
        self.weight_factor
    }
}
