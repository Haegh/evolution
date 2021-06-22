pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        // No real error control because it just a test project
        assert_eq!(inputs.len(), self.weights.len());

        let mut output = 0.0;
        for (&input, &weight) in inputs.iter().zip(&self.weights) {
            output += input * weight;
        }
        output += self.bias;

        output.max(0.0)
    }
}
