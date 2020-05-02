use crate::cppn::conf::CPPN;
use evolution::neat::{
    genome::GenomeComponent,
    node::{NodeCore, NodeRef},
    state::PopulationState,
};
use network::activation::Activation;
use rand::Rng;

#[derive(Clone)]
pub struct Node {
    pub neat_node: NodeCore,
    pub activation: Activation,
    pub bias: f64,
}

impl GenomeComponent<NodeCore, PopulationState> for Node {
    fn new(neat_node: NodeCore, _: &mut PopulationState) -> Self {
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

    fn get_neat(&self) -> &NodeCore {
        &self.neat_node
    }

    fn get_neat_mut(&mut self) -> &mut NodeCore {
        &mut self.neat_node
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            neat_node: GenomeComponent::<NodeCore, ()>::crossover(
                &self.neat_node,
                &other.neat_node,
                fitness,
                other_fitness,
            ),
            bias: (self.bias + other.bias) / 2.0,
            activation: if rand::thread_rng().gen::<bool>() {
                self.activation
            } else {
                other.activation
            },
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let mut distance =
            GenomeComponent::<NodeCore, ()>::distance(&self.neat_node, &other.neat_node);
        distance += 0.5 * ((self.activation != other.activation) as u8) as f64;
        distance += 0.5 * (self.bias - other.bias).abs().tanh();
        distance
    }
}
