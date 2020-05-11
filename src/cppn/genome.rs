use crate::cppn::{conf::CPPN, node::Node};
use evolution::{
    genome::{GenericGenome as GenericEvolvableGenome, Genome as EvolvableGenome},
    neat::{
        conf::NeatConfig,
        genome::NeatGenome,
        link::NeatLink,
        node::NodeRef,
        state::{InitConfig, NeatState},
    },
};
use network::activation;
use rand::Rng;

#[derive(Clone)]
pub struct Genome {
    pub neat: NeatGenome<Node, NeatLink>,
}

impl EvolvableGenome for Genome {
    type Config = NeatConfig;
    type InitConfig = InitConfig;
    type State = NeatState;
}

impl GenericEvolvableGenome<NeatConfig, NeatState, InitConfig> for Genome {
    fn new(config: &NeatConfig, init_config: &InitConfig, state: &mut NeatState) -> Self {
        Self {
            neat: NeatGenome::<Node, NeatLink>::new(config, init_config, state),
        }
    }

    fn crossover(
        &self,
        config: &NeatConfig,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        Self {
            neat: self
                .neat
                .crossover(config, &other.neat, fitness, other_fitness),
        }
    }

    fn mutate(&mut self, config: &NeatConfig, state: &mut NeatState) {
        self.neat.mutate(config, state);

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

    fn distance(&self, config: &NeatConfig, other: &Self) -> f64 {
        self.neat.distance(config, &other.neat)
    }
}

impl Genome {
    pub fn get_activation(&self, node_ref: &NodeRef) -> activation::Activation {
        match node_ref {
            NodeRef::Input(_) => self.neat.inputs.get(node_ref).unwrap().activation,
            NodeRef::Hidden(_) => self.neat.hidden_nodes.get(node_ref).unwrap().activation,
            NodeRef::Output(_) => self.neat.outputs.get(node_ref).unwrap().activation,
        }
    }

    pub fn get_bias(&self, node_ref: &NodeRef) -> f64 {
        match node_ref {
            NodeRef::Input(_) => self.neat.inputs.get(node_ref).unwrap().bias,
            NodeRef::Hidden(_) => self.neat.hidden_nodes.get(node_ref).unwrap().bias,
            NodeRef::Output(_) => self.neat.outputs.get(node_ref).unwrap().bias,
        }
    }

    fn mutate_hidden_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.neat.hidden_nodes.len());
            if let Some(node) = self.neat.hidden_nodes.values_mut().skip(link_index).next() {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * CPPN.mutate_hidden_bias_size;
            }
        }
    }

    fn mutate_hidden_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.neat.hidden_nodes.len());
            if let Some(node) = self.neat.hidden_nodes.values_mut().skip(link_index).next() {
                node.activation = CPPN.hidden_activations.random();
            }
        }
    }

    fn mutate_output_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.neat.outputs.len());
            if let Some(node) = self.neat.outputs.values_mut().skip(link_index).next() {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * CPPN.mutate_output_bias_size;
            }
        }
    }

    fn mutate_output_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.neat.outputs.len());
            if let Some(node) = self.neat.outputs.values_mut().skip(link_index).next() {
                node.activation = CPPN.output_activations.random();
            }
        }
    }
}
