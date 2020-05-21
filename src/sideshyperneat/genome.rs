use crate::cppn::{genome::Genome as CppnGenome, node::Node as CppnNode};
use crate::deshyperneat::{conf::DESHYPERNEAT, genome::Genome as DesGenome};
use crate::eshyperneat::genome::insert_identity;
use crate::sideshyperneat::{
    conf::{GenomeConfig, SIDESHYPERNEAT},
    link::Link,
    node::Node,
    state::State,
};
use evolution::{
    genome::{GenericGenome as GenericEvolvableGenome, Genome as EvolvableGenome},
    neat::{
        conf::ConfigProvider,
        genome::{NeatGenome, NeatGenomeStats},
        node::{NeatNode, NodeExtension, NodeRef},
        state::{InitConfig, StateProvider},
    },
    stats::Stats,
};
use rand::{seq::SliceRandom, Rng};
use serde::Serialize;

#[derive(Clone)]
pub struct Genome {
    pub cppn: CppnGenome,
    pub topology: NeatGenome<Node, Link>,
    pub des_genome: Option<DesGenome>,
}

#[derive(Serialize)]
pub struct SiDESgenomeStats {
    topology: NeatGenomeStats,
    cppn: NeatGenomeStats,
}
impl Stats for SiDESgenomeStats {}

impl EvolvableGenome for Genome {
    type Config = GenomeConfig;
    type InitConfig = InitConfig;
    type State = State;
    type Stats = SiDESgenomeStats;
}

impl GenericEvolvableGenome<GenomeConfig, State, InitConfig, SiDESgenomeStats> for Genome {
    fn new(config: &GenomeConfig, init_config: &InitConfig, state: &mut State) -> Self {
        let cppn = CppnGenome::new(&config.cppn, &InitConfig::new(4, 2), &mut state.cppn_state);
        let topology = NeatGenome::new(&config.topology, init_config, state);

        Self {
            cppn,
            topology,
            des_genome: None,
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
            cppn: self
                .cppn
                .crossover(&config.cppn, &other.cppn, fitness, other_fitness),
            topology: self.topology.crossover(
                &config.topology,
                &other.topology,
                fitness,
                other_fitness,
            ),
            des_genome: None,
        }
    }

    fn mutate(&mut self, config: &GenomeConfig, state: &mut State) {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < SIDESHYPERNEAT.topology_mutation_probability {
            self.topology.mutate(&config.topology, state);
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
            .collect::<Vec<(u64, bool)>>()
            .iter()
        {
            if !self
                .cppn
                .neat
                .outputs
                .contains_key(&NodeRef::Output(*output_id))
            {
                if *is_identity {
                    insert_identity(
                        &config.cppn,
                        &mut self.cppn,
                        &mut state.cppn_state,
                        *output_id,
                    )
                } else {
                    self.add_cppn_output(config, *output_id, state);
                }
            }
        }

        if rng.gen::<f64>() < SIDESHYPERNEAT.cppn_mutation_probability {
            self.cppn.mutate(&config.cppn, &mut state.cppn_state);
        }

        if rng.gen::<f64>() < DESHYPERNEAT.mutate_node_depth_probability {
            if let Some(node_ref) = self
                .topology
                .inputs
                .keys()
                .chain(self.topology.hidden_nodes.keys())
                .chain(self.topology.outputs.keys())
                .cloned()
                .collect::<Vec<NodeRef>>()
                .choose(&mut rng)
            {
                let (mut node, limit) = match node_ref {
                    NodeRef::Input(_) => (
                        self.topology.inputs.get_mut(&node_ref).unwrap(),
                        DESHYPERNEAT.max_input_substrate_depth,
                    ),
                    NodeRef::Hidden(_) => (
                        self.topology.hidden_nodes.get_mut(&node_ref).unwrap(),
                        DESHYPERNEAT.max_hidden_substrate_depth,
                    ),
                    NodeRef::Output(_) => (
                        self.topology.outputs.get_mut(&node_ref).unwrap(),
                        DESHYPERNEAT.max_output_substrate_depth,
                    ),
                };
                mutate_node(&mut node, limit, &mut rng);
            }
        }
    }

    fn distance(&self, config: &GenomeConfig, other: &Self) -> f64 {
        0.5 * self.cppn.distance(&config.cppn, &other.cppn)
            + 0.5 * self.topology.distance(&config.topology, &other.topology)
    }

    fn get_stats(&self) -> SiDESgenomeStats {
        SiDESgenomeStats {
            topology: self.topology.get_stats(),
            cppn: self.cppn.get_stats(),
        }
    }
}

impl Genome {
    fn add_cppn_output(&mut self, config: &GenomeConfig, id: u64, state: &mut State) {
        let node_ref = NodeRef::Output(id);
        self.cppn.neat.outputs.insert(
            node_ref,
            CppnNode::new(
                &config.cppn.neat_node(),
                NeatNode::new(NodeRef::Output(id)),
                state.cppn_state.node_mut(),
            ),
        );
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
