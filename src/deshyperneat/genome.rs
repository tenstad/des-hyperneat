use crate::deshyperneat::{
    conf::{GenomeConfig, DESHYPERNEAT},
    link::Link,
    node::Node,
    state::State,
};
use evolution::{
    genome::{GenericGenome as GenericEvolvableGenome, Genome as EvolvableGenome},
    neat::{
        genome::{NeatGenome, NeatGenomeStats},
        node::NodeRef,
        state::InitConfig,
    },
    stats::Stats,
};
use rand::{seq::SliceRandom, Rng};
use serde::Serialize;

#[derive(Clone)]
pub struct Genome {
    pub neat: NeatGenome<Node, Link>,
}

#[derive(Serialize)]
pub struct DESGenomeStats {
    topology: NeatGenomeStats,
    input_node_cppns: NeatGenomeStats,
    hidden_node_cppns: NeatGenomeStats,
    output_node_cppns: NeatGenomeStats,
    link_cppns: NeatGenomeStats,
}
impl Stats for DESGenomeStats {}

impl EvolvableGenome for Genome {
    type Config = GenomeConfig;
    type InitConfig = InitConfig;
    type State = State;
    type Stats = DESGenomeStats;
}

impl GenericEvolvableGenome<GenomeConfig, State, InitConfig, DESGenomeStats> for Genome {
    fn new(config: &GenomeConfig, init_config: &InitConfig, state: &mut State) -> Self {
        Self {
            neat: NeatGenome::<Node, Link>::new(config, init_config, state),
        }
    }

    fn crossover(
        &self,
        config: &GenomeConfig,
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

    fn mutate(&mut self, config: &GenomeConfig, state: &mut State) {
        self.neat.mutate(config, state);
        let mut rng = rand::thread_rng();

        let node_mut_prob = 3.0 / self.neat.hidden_nodes.len() as f64;
        let link_mut_prob = 3.0 / self.neat.links.len() as f64;

        for node in self
            .neat
            .hidden_nodes
            .values_mut()
            .chain(self.neat.inputs.values_mut())
            .chain(self.neat.outputs.values_mut())
        {
            if DESHYPERNEAT.mutate_all_components || rng.gen::<f64>() < node_mut_prob {
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
            if DESHYPERNEAT.mutate_all_components || rng.gen::<f64>() < link_mut_prob {
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

    fn distance(&self, config: &GenomeConfig, other: &Self) -> f64 {
        self.neat.distance(config, &other.neat)
    }

    fn get_stats(&self) -> DESGenomeStats {
        DESGenomeStats {
            topology: self.neat.get_stats(),
            input_node_cppns: accumulate_neat_stats(
                self.neat.inputs.values().map(|x| x.cppn.get_stats()),
            ),
            hidden_node_cppns: accumulate_neat_stats(
                self.neat.hidden_nodes.values().map(|x| x.cppn.get_stats()),
            ),
            output_node_cppns: accumulate_neat_stats(
                self.neat.outputs.values().map(|x| x.cppn.get_stats()),
            ),
            link_cppns: accumulate_neat_stats(self.neat.links.values().map(|x| x.cppn.get_stats())),
        }
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

fn accumulate_neat_stats(iter: impl Iterator<Item = NeatGenomeStats>) -> NeatGenomeStats {
    let (hidden_nodes, links, len) = iter.fold((0, 0, 0), |(hidden_nodes, links, len), stats| {
        (
            hidden_nodes + stats.hidden_nodes,
            links + stats.links,
            len + 1,
        )
    });

    if len == 0 {
        NeatGenomeStats {
            hidden_nodes: 0,
            links: 0,
        }
    } else {
        NeatGenomeStats {
            hidden_nodes: hidden_nodes / len,
            links: links / len,
        }
    }
}
