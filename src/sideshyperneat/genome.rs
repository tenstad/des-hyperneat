use crate::cppn::{genome::Genome as CppnGenome, node::Node as CppnNode};
use crate::deshyperneat::genome::Genome as DesGenome;
use crate::eshyperneat::genome::insert_identity;
use crate::sideshyperneat::{
    conf::{Config, SIDESHYPERNEAT},
    link::Link,
    node::Node,
    state::State,
};
use evolution::{
    genome::{GenericGenome as GenericEvolvableGenome, Genome as EvolvableGenome},
    neat::{
        conf::ConfigProvider,
        genome::NeatGenome,
        node::{NeatNode, NodeExtension, NodeRef},
        state::{InitConfig, StateProvider},
    },
};
use rand::Rng;

#[derive(Clone)]
pub struct Genome {
    pub cppn: CppnGenome,
    pub topology: NeatGenome<Node, Link>,
    pub des_genome: Option<DesGenome>,
}

impl EvolvableGenome for Genome {
    type Config = Config;
    type InitConfig = InitConfig;
    type State = State;
}

impl GenericEvolvableGenome<Config, State, InitConfig> for Genome {
    fn new(config: &Config, _init_config: &InitConfig, state: &mut State) -> Self {
        let mut topology =
            NeatGenome::<Node, Link>::new(&config.topology, &InitConfig::new(3, 1), state);
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
            cppn: CppnGenome::new(&config.cppn, &InitConfig::new(4, 2), &mut state.cppn_state),
            topology,
            des_genome: None,
        }
    }

    fn crossover(&self, config: &Config, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
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

    fn mutate(&mut self, config: &Config, state: &mut State) {
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
            .collect::<Vec<(usize, bool)>>()
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
    }

    fn distance(&self, config: &Config, other: &Self) -> f64 {
        0.5 * self.cppn.distance(&config.cppn, &other.cppn)
            + 0.5 * self.topology.distance(&config.topology, &other.topology)
    }
}

impl Genome {
    fn add_cppn_output(&mut self, config: &Config, id: usize, state: &mut State) {
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
