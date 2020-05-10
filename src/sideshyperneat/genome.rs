use crate::cppn::{genome::Genome as CppnGenome, node::Node as CppnNode};
use crate::deshyperneat::genome::Genome as DesGenome;
use crate::eshyperneat::genome::insert_identity;
use crate::sideshyperneat::{conf::SIDESHYPERNEAT, link::Link, node::Node, state::State};
use evolution::neat::{
    genome::{Genome as NeatGenome, GetCore, Node as NeatNode},
    genome_core::GenomeCore,
    link::LinkCore,
    node::{NodeCore, NodeRef},
    state::{InitConfig, StateProvider},
};
use rand::Rng;

impl evolution::genome::Genome for Genome {
    type InitConfig = InitConfig;
    type State = State;
}

pub type TopologyCore = GenomeCore<Node, Link>;
pub type CppnCore = GenomeCore<CppnNode, LinkCore>;

#[derive(Clone)]
pub struct Genome {
    pub cppn: CppnGenome,
    pub topology: TopologyCore,
    pub des_genome: Option<DesGenome>,
}

impl NeatGenome<State> for Genome {
    type Init = InitConfig;
    type Node = CppnNode;
    type Link = LinkCore;

    fn new(_init_config: &Self::Init, state: &mut State) -> Self {
        let mut topology = TopologyCore::new(&InitConfig::new(3, 1), state);
        topology
            .get_node_mut(&NodeRef::Input(0))
            .unwrap()
            .cppn_output_id = 0;
        topology
            .get_node_mut(&NodeRef::Input(1))
            .unwrap()
            .cppn_output_id = 1;
        topology
            .get_node_mut(&NodeRef::Input(2))
            .unwrap()
            .cppn_output_id = 2;
        topology
            .get_node_mut(&NodeRef::Output(0))
            .unwrap()
            .cppn_output_id = 3;
        Self {
            cppn: CppnGenome::new(&InitConfig::new(4, 2), &mut state.cppn_state),
            topology,
            des_genome: None,
        }
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            cppn: self.cppn.crossover(&other.cppn, fitness, other_fitness),
            topology: self
                .topology
                .crossover(&other.topology, fitness, other_fitness),
            des_genome: None,
        }
    }

    fn mutate(&mut self, state: &mut State) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < SIDESHYPERNEAT.topology_mutation_probability {
            self.topology.mutate(state);
        }

        // Add all missing cppn output nodes
        for (output_id, is_identity) in self
            .topology
            .hidden_nodes
            .values()
            .map(|node| (node.cppn_output_id, false))
            .chain(
                self.topology
                    .links
                    .values()
                    .map(|link| (link.cppn_output_id, link.is_identity)),
            )
            .collect::<Vec<(usize, bool)>>()
            .iter()
        {
            if !self
                .cppn
                .core
                .outputs
                .contains_key(&NodeRef::Output(*output_id))
            {
                if *is_identity {
                    insert_identity(&mut self.cppn, &mut state.cppn_state, *output_id)
                } else {
                    self.add_cppn_output(*output_id, state);
                }
            }
        }

        if rng.gen::<f64>() < SIDESHYPERNEAT.cppn_mutation_probability {
            self.cppn.mutate(&mut state.cppn_state);
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        0.5 * self.cppn.distance(&other.cppn) + 0.5 * self.topology.distance(&other.topology)
    }
}

impl GetCore<GenomeCore<CppnNode, LinkCore>> for Genome {
    fn get_core(&self) -> &CppnCore {
        &self.cppn.core
    }

    fn get_core_mut(&mut self) -> &mut CppnCore {
        &mut self.cppn.core
    }
}

impl Genome {
    fn add_cppn_output(&mut self, id: usize, state: &mut State) {
        let node_ref = NodeRef::Output(id);
        self.cppn.core.outputs.insert(
            node_ref,
            CppnNode::new(
                NodeCore::new(NodeRef::Output(id)),
                state.cppn_state.get_node_state_mut(),
            ),
        );
    }
}
