use crate::cppn::conf::CPPN;
use evolution::neat::{
    genome::{GenomeComponent, NeatCore},
    node::{NodeCore, NodeRef},
};
use network::activation::Activation;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Node {
    pub neat_node: NodeCore,
    pub activation: Activation,
    pub bias: f64,
}

impl NeatCore<NodeCore> for Node {
    fn get_neat(&self) -> &NodeCore {
        &self.neat_node
    }

    fn get_neat_mut(&mut self) -> &mut NodeCore {
        &mut self.neat_node
    }
}

impl GenomeComponent<NodeCore> for Node {
    fn new(neat_node: NodeCore) -> Self {
        Self {
            neat_node: neat_node,
            bias: 0.0,
            activation: match neat_node.node_ref {
                NodeRef::Input(_) => Activation::None,
                NodeRef::Hidden(_) => CPPN.hidden_activations.random(),
                NodeRef::Output(_) => CPPN.output_activations.random(),
            },
        }
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            neat_node: self
                .neat_node
                .crossover(&other.neat_node, fitness, other_fitness),
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }
}
