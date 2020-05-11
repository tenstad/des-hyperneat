use crate::cppn::conf::CPPN;
use evolution::neat::{
    genome::GetNeat,
    node::{NeatNode, NodeExtension, NodeRef},
};
use network::activation::Activation;
use rand::Rng;

#[derive(Clone, GetNeat)]
pub struct Node {
    #[neat]
    pub neat: NeatNode,
    pub activation: Activation,
    pub bias: f64,
}

impl NodeExtension for Node {
    type Config = ();
    type State = ();

    fn new(_: &Self::Config, neat: NeatNode, _: &mut Self::State) -> Self {
        Self {
            neat,
            bias: 0.0,
            activation: match neat.node_ref {
                NodeRef::Input(_) => Activation::None,
                NodeRef::Hidden(_) => CPPN.hidden_activations.random(),
                NodeRef::Output(_) => CPPN.output_activations.random(),
            },
        }
    }

    fn crossover(
        &self,
        _: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        Self {
            neat: self.neat.crossover(&other.neat, fitness, other_fitness),
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        let mut distance = self.neat.distance(&other.neat);
        distance += 0.5 * ((self.activation != other.activation) as u8) as f64;
        distance += 0.5 * (self.bias - other.bias).abs().tanh();
        distance
    }
}
