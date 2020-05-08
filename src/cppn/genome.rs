use crate::cppn::{conf::CPPN, node::Node};
use evolution::neat::{
    genome::Genome as NeatGenome,
    genome_core::GenomeCore,
    link::DefaultLink,
    node::NodeRef,
    state::{InitConfig, StateCore},
};
use network::activation;
use rand::Rng;

pub type NeatCore = GenomeCore<Node, DefaultLink>;

impl evolution::genome::Genome for Genome {
    type InitConfig = InitConfig;
    type State = StateCore;
}

#[derive(Clone)]
pub struct Genome {
    pub core: NeatCore,
}

impl NeatGenome for Genome {
    type Init = InitConfig;
    type State = StateCore;
    type Node = Node;
    type Link = DefaultLink;

    fn new(init_config: &InitConfig, state: &mut StateCore) -> Self {
        Self {
            core: NeatCore::new(init_config, state),
        }
    }

    fn get_core(&self) -> &NeatCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut NeatCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
        }
    }

    fn mutate(&mut self, state: &mut StateCore) {
        self.core.mutate(state);

        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < CPPN.mutate_hidden_bias_probability {
            self.mutate_hidden_bias();
        }

        if rng.gen::<f64>() < CPPN.mutate_hidden_activation_probability {
            self.mutate_hidden_activation();
        }

        if rng.gen::<f64>() < CPPN.mutate_output_bias_probability {
            self.mutate_output_bias();
        }

        if rng.gen::<f64>() < CPPN.mutate_output_activation_probability {
            self.mutate_output_activation();
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        self.get_core().distance(other.get_core())
    }
}

impl Genome {
    pub fn get_activation(&self, node_ref: &NodeRef) -> activation::Activation {
        match node_ref {
            NodeRef::Input(_) => self.core.inputs.get(node_ref).unwrap().activation,
            NodeRef::Hidden(_) => self.core.hidden_nodes.get(node_ref).unwrap().activation,
            NodeRef::Output(_) => self.core.outputs.get(node_ref).unwrap().activation,
        }
    }

    pub fn get_bias(&self, node_ref: &NodeRef) -> f64 {
        match node_ref {
            NodeRef::Input(_) => self.core.inputs.get(node_ref).unwrap().bias,
            NodeRef::Hidden(_) => self.core.hidden_nodes.get(node_ref).unwrap().bias,
            NodeRef::Output(_) => self.core.outputs.get(node_ref).unwrap().bias,
        }
    }

    fn mutate_hidden_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.core.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.core.hidden_nodes.len());
            if let Some(node) = self.core.hidden_nodes.values_mut().skip(link_index).next() {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * CPPN.mutate_hidden_bias_size;
            }
        }
    }

    fn mutate_hidden_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.core.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.core.hidden_nodes.len());
            if let Some(node) = self.core.hidden_nodes.values_mut().skip(link_index).next() {
                node.activation = CPPN.hidden_activations.random();
            }
        }
    }

    fn mutate_output_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.core.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.core.outputs.len());
            if let Some(node) = self.core.outputs.values_mut().skip(link_index).next() {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * CPPN.mutate_output_bias_size;
            }
        }
    }

    fn mutate_output_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.core.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.core.outputs.len());
            if let Some(node) = self.core.outputs.values_mut().skip(link_index).next() {
                node.activation = CPPN.output_activations.random();
            }
        }
    }
}
