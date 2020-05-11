use crate::deshyperneat::{
    conf::{Config, DESHYPERNEAT},
    link::Link,
    node::Node,
    state::State,
};
use crate::eshyperneat::conf::ESHYPERNEAT;
use evolution::{
    genome::{GenericGenome as GenericEvolvableGenome, Genome as EvolvableGenome},
    neat::{genome::NeatGenome, node::NodeRef, state::InitConfig},
};
use rand::{seq::SliceRandom, Rng};

#[derive(Clone)]
pub struct Genome {
    pub neat: NeatGenome<Node, Link>,
}

impl EvolvableGenome for Genome {
    type Config = Config;
    type InitConfig = InitConfig;
    type State = State;
}

impl GenericEvolvableGenome<Config, State, InitConfig> for Genome {
    fn new(config: &Config, init_config: &InitConfig, state: &mut State) -> Self {
        Self {
            neat: NeatGenome::<Node, Link>::new(config, init_config, state),
        }
    }

    fn crossover(&self, config: &Config, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            neat: self
                .neat
                .crossover(config, &other.neat, fitness, other_fitness),
        }
    }

    fn mutate(&mut self, config: &Config, state: &mut State) {
        self.neat.mutate(config, state);
        let mut rng = rand::thread_rng();

        let node_mut_prob = 3.0 / self.neat.hidden_nodes.len() as f64;
        let link_mut_prob = 3.0 / self.neat.links.len() as f64;

        for node in self.neat.hidden_nodes.values_mut() {
            if rng.gen::<f64>() < node_mut_prob {
                node.cppn.mutate(
                    &config.cppn,
                    if DESHYPERNEAT.single_cppn_state {
                        &mut state.custom.single_cppn_state
                    } else {
                        state
                            .custom
                            .unique_cppn_states
                            .get_mut(&(node.neat.node_ref, node.neat.node_ref))
                            .unwrap()
                    },
                );
            }
        }
        for link in self.neat.links.values_mut() {
            if rng.gen::<f64>() < link_mut_prob {
                link.cppn.mutate(
                    &config.cppn,
                    if DESHYPERNEAT.single_cppn_state {
                        &mut state.custom.single_cppn_state
                    } else {
                        let key = &(link.neat.from, link.neat.to);
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
                .neat
                .hidden_nodes
                .keys()
                .cloned()
                .collect::<Vec<NodeRef>>()
                .choose(&mut rng)
            {
                let mut node = self.neat.hidden_nodes.get_mut(&key).unwrap();
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
        self.neat.distance(config, &other.neat)
    }
}
