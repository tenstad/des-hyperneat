use crate::deshyperneat::{
    conf::{Config, DESHYPERNEAT},
    link::Link,
    node::Node,
    state::State,
};
use crate::eshyperneat::conf::ESHYPERNEAT;
use evolution::neat::{
    genome::{Genome as NeatGenome, GetCore},
    genome_core::GenomeCore,
    node::NodeRef,
    state::InitConfig,
};
use rand::{seq::SliceRandom, Rng};

type NeatCore = GenomeCore<Node, Link>;

impl evolution::genome::Genome for Genome {
    type Config = Config;
    type InitConfig = InitConfig;
    type State = State;
}

#[derive(Clone, GetCore)]
pub struct Genome {
    #[core]
    pub core: NeatCore,
}

impl NeatGenome<Config, State> for Genome {
    type Init = InitConfig;
    type Node = Node;
    type Link = Link;

    fn new(config: &Config, init_config: &Self::Init, state: &mut State) -> Self {
        Self {
            core: GenomeCore::<Self::Node, Self::Link>::new(config, init_config, state),
        }
    }

    fn crossover(&self, config: &Config, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            core: self
                .core
                .crossover(config, &other.core, fitness, other_fitness),
        }
    }

    fn mutate(&mut self, config: &Config, state: &mut State) {
        self.core.mutate(config, state);
        let mut rng = rand::thread_rng();

        let node_mut_prob = 3.0 / self.core.hidden_nodes.len() as f64;
        let link_mut_prob = 3.0 / self.core.links.len() as f64;

        for node in self.core.hidden_nodes.values_mut() {
            if rng.gen::<f64>() < node_mut_prob {
                node.cppn.mutate(
                    &config.cppn,
                    if DESHYPERNEAT.single_cppn_state {
                        &mut state.custom.single_cppn_state
                    } else {
                        state
                            .custom
                            .unique_cppn_states
                            .get_mut(&(node.core.node_ref, node.core.node_ref))
                            .unwrap()
                    },
                );
            }
        }
        for link in self.core.links.values_mut() {
            if rng.gen::<f64>() < link_mut_prob {
                link.cppn.mutate(
                    &config.cppn,
                    if DESHYPERNEAT.single_cppn_state {
                        &mut state.custom.single_cppn_state
                    } else {
                        let key = &(link.core.from, link.core.to);
                        state
                            .custom
                            .unique_cppn_states
                            .get_mut(
                                if let Some(redirect) = state.custom.cppn_state_redirects.get(key) {
                                    redirect
                                } else {
                                    key
                                },
                            )
                            .expect("cannot find unique link state")
                    },
                );
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
                        (node.depth + 1).min(ESHYPERNEAT.iteration_level)
                    } else {
                        node.depth - 1
                    };
                }
            }
        }
    }

    fn distance(&self, config: &Config, other: &Self) -> f64 {
        self.core.distance(config, &other.core)
    }
}
