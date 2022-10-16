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

impl Network {
    pub fn propagate(inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}