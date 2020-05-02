use crate::cppn::{conf::CPPN, node::Node};
use evolution::neat::{
    genome::Genome as NeatGenome,
    genome_core::GenomeCore,
    link::LinkCore,
    node::NodeRef,
    state::{InitConfig, PopulationState},
};
use network::activation;
use rand::Rng;

type NeatGenomeType = GenomeCore<Node, LinkCore, PopulationState>;

impl evolution::genome::Genome for Genome {
    type InitConfig = InitConfig;
    type PopulationState = PopulationState;
}

#[derive(Clone)]
pub struct Genome {
    pub neat_genome: NeatGenomeType,
}

impl NeatGenome for Genome {
    type Init = InitConfig;
    type State = PopulationState;
    type Node = Node;
    type Link = LinkCore;

    fn new(init_config: &Self::Init, population_state: &mut Self::State) -> Self {
        Self {
            neat_genome: NeatGenomeType::new(init_config, population_state),
        }
    }

    fn get_neat(&self) -> &NeatGenomeType {
        &self.neat_genome
    }

    fn get_neat_mut(&mut self) -> &mut NeatGenomeType {
        &mut self.neat_genome
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            neat_genome: self
                .neat_genome
                .crossover(&other.neat_genome, fitness, other_fitness),
        }
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

    fn distance(&self, other: &Self) -> f64 {
        self.get_neat().distance(other.get_neat())
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
