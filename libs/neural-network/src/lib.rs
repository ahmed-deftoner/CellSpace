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
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    } 
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    
    pub fn random(layers: Vec<LayerTopology>) -> Self {
        assert!(layers.len() > 1);
        let mut built_layers = Vec::new();

        for i in 0..(layers.len() - 1) {
            let input_neurons = layers[i].neurons;
            let output_neurons = layers[i + 1].neurons;

            built_layers.push(Layer::random(
                input_neurons,
                output_neurons,
            ));
        }

        Self { layers: built_layers }
    }
}