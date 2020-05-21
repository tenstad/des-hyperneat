use crate::codeshyperneat::{link::Link, node::Node, state::State};
use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::conf::DESHYPERNEAT;
use evolution::{
    genome::{GenericGenome as GenericEvolvableGenome, Genome as EvolvableGenome},
    neat::{conf::NeatConfig, genome::NeatGenome, node::NodeRef, state::InitConfig},
    population::Population,
    stats::NoStats,
};
use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Genome {
    pub neat: NeatGenome<Node, Link>,
}

impl EvolvableGenome for Genome {
    type Config = NeatConfig;
    type InitConfig = InitConfig;
    type State = State;
    type Stats = NoStats;
}

impl GenericEvolvableGenome<NeatConfig, State, InitConfig, NoStats> for Genome {
    fn new(config: &NeatConfig, init_config: &InitConfig, state: &mut State) -> Self {
        Self {
            neat: NeatGenome::<Node, Link>::new(config, init_config, state),
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

    fn mutate(&mut self, config: &NeatConfig, state: &mut State) {
        self.neat.mutate(config, state);

        let mut rng = rand::thread_rng();

        if state.custom.species > 0 {
            if rng.gen::<f64>() < 0.05 {
                if let Some(key) = self
                    .neat
                    .links
                    .keys()
                    .cloned()
                    .collect::<Vec<(NodeRef, NodeRef)>>()
                    .choose(&mut rng)
                {
                    self.neat.links.get_mut(&key).unwrap().module_species =
                        rng.gen_range(0, state.custom.species);
                }
            }

            if rng.gen::<f64>() < 0.05 {
                if let Some(key) = self
                    .neat
                    .hidden_nodes
                    .keys()
                    .cloned()
                    .collect::<Vec<NodeRef>>()
                    .choose(&mut rng)
                {
                    self.neat.hidden_nodes.get_mut(&key).unwrap().module_species =
                        rng.gen_range(0, state.custom.species);
                }
            }
        }

        if rng.gen::<f64>() < DESHYPERNEAT.mutate_node_depth_probability {
            if let Some(node_ref) = self
                .neat
                .inputs
                .keys()
                .chain(self.neat.hidden_nodes.keys())
                .chain(self.neat.outputs.keys())
                .cloned()
                .collect::<Vec<NodeRef>>()
                .choose(&mut rng)
            {
                let (mut node, limit) = match node_ref {
                    NodeRef::Input(_) => (
                        self.neat.inputs.get_mut(&node_ref).unwrap(),
                        DESHYPERNEAT.max_input_substrate_depth,
                    ),
                    NodeRef::Hidden(_) => (
                        self.neat.hidden_nodes.get_mut(&node_ref).unwrap(),
                        DESHYPERNEAT.max_hidden_substrate_depth,
                    ),
                    NodeRef::Output(_) => (
                        self.neat.outputs.get_mut(&node_ref).unwrap(),
                        DESHYPERNEAT.max_output_substrate_depth,
                    ),
                };
                mutate_node(&mut node, limit, &mut rng);
            }
        }
    }

    fn distance(&self, config: &NeatConfig, other: &Self) -> f64 {
        self.neat.distance(config, &other.neat)
    }

    fn get_stats(&self) -> NoStats {
        NoStats {}
    }
}

impl Genome {
    pub fn select_modules(
        &self,
        modules: &Population<CppnGenome>,
    ) -> HashMap<u64, (usize, CppnGenome)> {
        let mut rng = rand::thread_rng();
        let mut genomes = HashMap::<u64, (usize, CppnGenome)>::new();

        for module_species in self
            .neat
            .inputs
            .values()
            .chain(self.neat.hidden_nodes.values())
            .chain(self.neat.outputs.values())
            .map(|node| node.module_species)
            .chain(self.neat.links.values().map(|link| link.module_species))
        {
            if !genomes.contains_key(&module_species) {
                let species = modules
                    .species
                    .get(&module_species)
                    .or_else(|| modules.extinct_species.get(&module_species))
                    .unwrap();

                let index = rng.gen_range(0, species.len());
                genomes.insert(
                    module_species,
                    (index, species.organisms[index].genome.clone()),
                );
            }
        }

        genomes
    }
}

fn mutate_node<R: Rng>(node: &mut Node, limit: u64, rng: &mut R) {
    if limit == 0 {
        assert_eq!(node.depth, 0);
        return;
    }

    if node.depth == 0 {
        node.depth += 1;
    } else if node.depth == limit {
        node.depth -= 1;
    } else {
        node.depth = if rng.gen::<bool>() {
            node.depth + 1
        } else {
            node.depth - 1
        };
    }

    node.depth = node.depth.min(limit).max(0);
}
