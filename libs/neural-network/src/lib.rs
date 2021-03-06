use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;


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

pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(layers: &[LayerTopology]) -> Self {
        // Network with just one layer is technically doable, but doesn't
        // make much sense:
        assert!(layers.len() > 1);
    
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();
    
        Self { layers }
    }
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(&mut rng, input_neurons))
            .collect();

        Self { neurons }
    }
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }

    pub fn random(rng: &mut dyn rand::RngCore,output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}


#[test]
fn test_rand() {
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let neuron = Neuron::random(&mut rng, 4);

    assert_eq!(neuron.bias, -0.6255188);
    assert_eq!(neuron.weights.as_slice(), [0.67383957,
        0.8181262,
        0.26284897,
        0.5238807
    ].as_ref());
}

#[test]
fn test_propagation() {
    // Neuron propagation
    let neuron = Neuron {
        bias: 0.5,
        weights: vec![-0.3, 0.8],
    };

    // Control max
    approx::assert_relative_eq!(
        neuron.propagate(&[-10.0, -10.0]),
        0.0,
    );

    // Control dynamic
    approx::assert_relative_eq!(
        neuron.propagate(&[0.5, 1.0]),
        (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
    );
}
