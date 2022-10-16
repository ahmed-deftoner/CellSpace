struct Neuron {
    weights: Vec<f32>,
    bias: f32
}

struct Layer {
    neurons: Vec<Neuron>
}

pub struct Network {
    layers: Vec<Layer>
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs.iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        (self.bias + output).max(0.0)
    } 

    pub fn random(output: usize) -> Self {
        let bias = todo!();

        let weights = (0..output)
            .map(|_| todo!())
            .collect();
        Self { weights, bias }
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    } 

    pub fn random(input: usize, output: usize) -> Self {
        let neurons = (0..output)
            .map(|_| Neuron::random(input))
            .collect();
        Self { neurons }
    }
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    
    pub fn random(layers: &[LayerTopology]) -> Self {
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