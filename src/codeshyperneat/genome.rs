use crate::codeshyperneat::{link::Link, node::Node, state::State};
use crate::cppn::genome::Genome as CppnGenome;
use evolution::{
    neat::{
        genome::Genome as NeatGenome, genome_core::GenomeCore, node::NodeRef, state::InitConfig,
    },
    population::Population,
};
use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;

type NeatCore = GenomeCore<Node, Link>;

impl evolution::genome::Genome for Genome {
    type InitConfig = InitConfig;
    type State = State;
}

#[derive(Clone)]
pub struct Genome {
    pub core: NeatCore,
}

impl NeatGenome<State> for Genome {
    type Init = InitConfig;
    type Node = Node;
    type Link = Link;

    fn new(init_config: &Self::Init, state: &mut State) -> Self {
        Self {
            core: GenomeCore::<Self::Node, Self::Link>::new(init_config, state),
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

    fn mutate(&mut self, state: &mut State) {
        self.core.mutate(state);

        let mut rng = rand::thread_rng();

        if state.custom.species > 0 {
            if rng.gen::<f64>() < 0.05 {
                if let Some(key) = self
                    .core
                    .links
                    .keys()
                    .cloned()
                    .collect::<Vec<(NodeRef, NodeRef)>>()
                    .choose(&mut rng)
                {
                    self.core.links.get_mut(&key).unwrap().module_species =
                        rng.gen_range(0, state.custom.species);
                }
            }

            if rng.gen::<f64>() < 0.05 {
                if let Some(key) = self
                    .core
                    .hidden_nodes
                    .keys()
                    .cloned()
                    .collect::<Vec<NodeRef>>()
                    .choose(&mut rng)
                {
                    self.core.hidden_nodes.get_mut(&key).unwrap().module_species =
                        rng.gen_range(0, state.custom.species);
                }
            }
        }

        if rng.gen::<f64>() < 0.05 {
            if let Some(key) = self
                .core
                .hidden_nodes
                .keys()
                .cloned()
                .collect::<Vec<NodeRef>>()
                .choose(&mut rng)
            {
                let mut node = self.core.hidden_nodes.get_mut(&key).unwrap();
                if node.depth == 0 {
                    node.depth = 1;
                } else {
                    node.depth = if rng.gen::<f64>() < 0.5 {
                        (node.depth + 1).min(5)
                    } else {
                        node.depth - 1
                    };
                }
            }
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        self.core.distance(&other.core)
    }
}

impl Genome {
    pub fn select_modules<S>(
        &self,
        modules: &Population<CppnGenome, S>,
    ) -> HashMap<usize, (usize, CppnGenome)> {
        let mut rng = rand::thread_rng();
        let mut genomes = HashMap::<usize, (usize, CppnGenome)>::new();

        for module_species in self
            .core
            .inputs
            .values()
            .chain(self.core.hidden_nodes.values())
            .chain(self.core.outputs.values())
            .map(|node| node.module_species)
            .chain(self.core.links.values().map(|link| link.module_species))
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
