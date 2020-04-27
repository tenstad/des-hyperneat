use crate::cppn::{conf::CPPN, node::Node};
use evolution::neat::{
    genome::{Genome as NeatGenome, NeatCore},
    genome_core::{GenomeCore, InitConfig},
    link::LinkCore,
    node::NodeRef,
    state::PopulationState,
};
use network::activation;
use rand::Rng;

#[derive(Clone)]
pub struct Genome {
    pub neat_genome: GenomeCore<Node, LinkCore>,
}

impl NeatCore<GenomeCore<Node, LinkCore>> for Genome {
    fn get_neat(&self) -> &GenomeCore<Node, LinkCore> {
        &self.neat_genome
    }

    fn get_neat_mut(&mut self) -> &mut GenomeCore<Node, LinkCore> {
        &mut self.neat_genome
    }

    fn mutate(&mut self, population_state: &mut PopulationState) {
        self.neat_genome.mutate(population_state);

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
}

impl NeatGenome for Genome {
    type Node = Node;
    type Link = LinkCore;

    fn new(init_config: &InitConfig) -> Self {
        Self {
            neat_genome: GenomeCore::<Self::Node, Self::Link>::new(init_config),
        }
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            neat_genome: self
                .get_neat()
                .crossover(other.get_neat(), fitness, other_fitness),
        }
    }
}

impl Genome {
    pub fn get_activation(&self, node_ref: &NodeRef) -> activation::Activation {
        match node_ref {
            NodeRef::Input(_) => self.neat_genome.inputs.get(node_ref).unwrap().activation,
            NodeRef::Hidden(_) => {
                self.neat_genome
                    .hidden_nodes
                    .get(node_ref)
                    .unwrap()
                    .activation
            }
            NodeRef::Output(_) => self.neat_genome.outputs.get(node_ref).unwrap().activation,
        }
    }

    pub fn get_bias(&self, node_ref: &NodeRef) -> f64 {
        match node_ref {
            NodeRef::Input(_) => self.neat_genome.inputs.get(node_ref).unwrap().bias,
            NodeRef::Hidden(_) => self.neat_genome.hidden_nodes.get(node_ref).unwrap().bias,
            NodeRef::Output(_) => self.neat_genome.outputs.get(node_ref).unwrap().bias,
        }
    }

    fn mutate_hidden_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat_genome.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.neat_genome.hidden_nodes.len());
            if let Some(node) = self
                .neat_genome
                .hidden_nodes
                .values_mut()
                .skip(link_index)
                .next()
            {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * CPPN.mutate_hidden_bias_size;
            }
        }
    }

    fn mutate_hidden_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat_genome.hidden_nodes.is_empty() {
            let link_index = rng.gen_range(0, self.neat_genome.hidden_nodes.len());
            if let Some(node) = self
                .neat_genome
                .hidden_nodes
                .values_mut()
                .skip(link_index)
                .next()
            {
                node.activation = CPPN.hidden_activations.random();
            }
        }
    }

    fn mutate_output_bias(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat_genome.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.neat_genome.outputs.len());
            if let Some(node) = self
                .neat_genome
                .outputs
                .values_mut()
                .skip(link_index)
                .next()
            {
                node.bias += (rng.gen::<f64>() - 0.5) * 2.0 * CPPN.mutate_output_bias_size;
            }
        }
    }

    fn mutate_output_activation(&mut self) {
        let mut rng = rand::thread_rng();

        if !self.neat_genome.outputs.is_empty() {
            let link_index = rng.gen_range(0, self.neat_genome.outputs.len());
            if let Some(node) = self
                .neat_genome
                .outputs
                .values_mut()
                .skip(link_index)
                .next()
            {
                node.activation = CPPN.output_activations.random();
            }
        }
    }
}
