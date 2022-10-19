use rand::{Rng, RngCore};


struct Neuron {
    weights: Vec<f32>,
    bias: f32
}

struct Layer {
    neurons: Vec<Neuron>
}

#[derive(Clone, Debug)]
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
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output)
            .map(|_| rng.gen_range(-1.0..=1.0))
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

    pub fn random(rng: &mut dyn RngCore, input: usize, output: usize) -> Self {
        let neurons = (0..output)
            .map(|_| Neuron::random(input))
            .collect();
        Self { neurons }
    }
}

impl Network {
    crate fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Self::new(layers)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod random {
        use super::*;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 2 },
                    LayerTopology { neurons: 1 },
                ],
            );

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.layers[0].neurons.len(), 2);

            approx::assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);

            approx::assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                &[0.67383957, 0.8181262, 0.26284897].as_slice()
            );

            approx::assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238807);

            approx::assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                &[-0.5351684, 0.069369555, -0.7648182].as_slice()
            );

            assert_eq!(network.layers[1].neurons.len(), 1);

            approx::assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                &[-0.48879623, -0.19277143].as_slice()
            );
        }
    }

}