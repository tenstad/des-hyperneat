use crate::cppn::genome::Genome as CppnGenome;
use crate::deshyperneat::conf::DESHYPERNEAT;
use crate::deshyperneat::{desgenome::DesGenome, link::Link, node::Node};
use evolution::neat::{
    genome::Genome as NeatGenome,
    genome_core::GenomeCore,
    node::NodeRef,
    state::{InitConfig, NeatStateProvider, StateCore},
};
use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct State {
    pub core: StateCore,
    pub single_cppn_state: StateCore,
    pub unique_cppn_states: HashMap<(NodeRef, NodeRef), StateCore>,
}

impl NeatStateProvider for State {
    fn get_core(&self) -> &StateCore {
        &self.core
    }
    fn get_core_mut(&mut self) -> &mut StateCore {
        &mut self.core
    }
}

type NeatCore = GenomeCore<Node, Link, State>;

impl evolution::genome::Genome for Genome {
    type InitConfig = InitConfig;
    type State = State;
}

#[derive(Clone)]
pub struct Genome {
    pub core: NeatCore,
}

impl DesGenome for Genome {
    type State = State;
    type Node = Node;
    type Link = Link;

    fn get_node_cppn(&self, node: NodeRef) -> &CppnGenome {
        &self.core.get_node(node).unwrap().cppn
    }

    fn get_link_cppn(&self, source: NodeRef, target: NodeRef) -> &CppnGenome {
        &self.core.links.get(&(source, target)).unwrap().cppn
    }

    fn get_depth(&self, node: NodeRef) -> usize {
        self.core.get_node(node).unwrap().depth
    }

    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link, Self::State> {
        &self.core
    }
}

impl NeatGenome for Genome {
    type Init = InitConfig;
    type State = State;
    type Node = Node;
    type Link = Link;

    fn new(init_config: &Self::Init, state: &mut Self::State) -> Self {
        Self {
            core: GenomeCore::<Self::Node, Self::Link, Self::State>::new(init_config, state),
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

    fn mutate(&mut self, state: &mut Self::State) {
        self.core.mutate(state);
        let mut rng = rand::thread_rng();

        for node in self.core.hidden_nodes.values_mut() {
            node.cppn.mutate(if DESHYPERNEAT.single_cppn_state {
                &mut state.single_cppn_state
            } else {
                state
                    .unique_cppn_states
                    .get_mut(&(node.core.node_ref, node.core.node_ref))
                    .unwrap()
            });
        }
        for link in self.core.links.values_mut() {
            link.cppn.mutate(if DESHYPERNEAT.single_cppn_state {
                &mut state.single_cppn_state
            } else {
                state
                    .unique_cppn_states
                    .get_mut(&(link.core.from, link.core.to))
                    .unwrap()
            });
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
