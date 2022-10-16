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

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    } 
}

impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}