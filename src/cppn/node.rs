use crate::cppn::conf::CPPN;
use evolution::neat::{
    genome::Node as NeatNode,
    node::{NodeCore, NodeRef},
    state::StateCore,
};
use network::activation::Activation;
use rand::Rng;

#[derive(Clone)]
pub struct Node {
    pub core: NodeCore,
    pub activation: Activation,
    pub bias: f64,
}

impl NeatNode<StateCore> for Node {
    fn new(core: NodeCore, _: &mut StateCore) -> Self {
        Self {
            core,
            bias: 0.0,
            activation: match core.node_ref {
                NodeRef::Input(_) => Activation::None,
                NodeRef::Hidden(_) => CPPN.hidden_activations.random(),
                NodeRef::Output(_) => CPPN.output_activations.random(),
            },
        }
    }

    fn get_core(&self) -> &NodeCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut NodeCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance = self.core.distance(&other.core);
        distance += 0.5 * ((self.activation != other.activation) as u8) as f64;
        distance += 0.5 * (self.bias - other.bias).abs().tanh();
        distance
    }
}
