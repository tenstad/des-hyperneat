use crate::cppn::developer::Developer as CppnDeveloper;
use crate::deshyperneat::{conf::DESHYPERNEAT, desgenome::DesGenome};
use crate::eshyperneat::{conf::ESHYPERNEAT, search};
use crate::hyperneat::substrate;
use bson;
use evolution::{
    develop::Develop,
    environment::EnvironmentDescription,
    neat::{developer::NetworkStats, genome::GetNeat, node::NodeRef, state::InitConfig},
    stats::Stats,
};
use network::{
    connection,
    execute::{Action, Executor},
};
use serde::Serialize;
use serde_json;
use std::collections::{HashMap, HashSet};

pub struct Developer {
    cppn_developer: CppnDeveloper,
    input_nodes: Vec<Vec<(i64, i64)>>,
    output_nodes: Vec<Vec<(i64, i64)>>,
    flattened_inputs: Vec<(NodeRef, i64, i64)>,
    flattened_outputs: Vec<(NodeRef, i64, i64)>,
}

impl From<EnvironmentDescription> for Developer {
    fn from(description: EnvironmentDescription) -> Self {
        let r = ESHYPERNEAT.resolution as i64;
        let input_nodes = parse_nodes(&DESHYPERNEAT.input_config, r, description.inputs);
        let output_nodes = parse_nodes(&DESHYPERNEAT.output_config, r, description.outputs);

        let flattened_inputs = input_nodes
            .iter()
            .enumerate()
            .flat_map(|(i, nodes)| {
                nodes
                    .iter()
                    .map(move |node| (NodeRef::Input(i as u64), node.0, node.1))
            })
            .collect::<Vec<(NodeRef, i64, i64)>>();
        let flattened_outputs = output_nodes
            .iter()
            .enumerate()
            .flat_map(|(i, nodes)| {
                nodes
                    .iter()
                    .map(move |node| (NodeRef::Output(i as u64), node.0, node.1))
            })
            .collect::<Vec<(NodeRef, i64, i64)>>();

        Self {
            cppn_developer: CppnDeveloper::from(description),
            input_nodes,
            output_nodes,
            flattened_inputs,
            flattened_outputs,
        }
    }
}

#[derive(Serialize, new)]
pub struct MultiSubstrateNetworkStats {
    #[serde(with = "bson::compat::u2f")]
    pub hidden_substrates: u64,
    pub hidden_substrate_node_counts: Vec<i64>,
    pub network_stats: NetworkStats,
}

impl Stats for MultiSubstrateNetworkStats {}

impl Developer {
    pub fn connections<G: DesGenome>(
        &self,
        genome: G,
    ) -> connection::Connections<(NodeRef, i64, i64), f64> {
        // Let the genome prepeare to provide cppns and depth
        let mut genome = genome;
        genome.init_desgenome();

        // Init assembled network
        let mut assembled_connections = connection::Connections::<(NodeRef, i64, i64), f64>::new();

        // Init known nodes with the input and output nodes
        let mut substrate_nodes = HashMap::<NodeRef, HashSet<(i64, i64)>>::new();
        for (i, nodes) in self.input_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Input(i as u64),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
        for (i, nodes) in self.output_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Output(i as u64),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
        // All hidden substrates are empty
        for node_ref in genome.get_neat().hidden_nodes.keys() {
            substrate_nodes.insert(*node_ref, HashSet::new());
        }

        // Iterative network completion in topologically sorted order
        let order = genome.get_neat().connections.sort_topologically();
        for element in order.iter() {
            match element {
                connection::OrderedAction::Edge(from, to, _) => {
                    // Develop the link's cppn
                    let mut cppn = self
                        .cppn_developer
                        .develop(genome.get_link_cppn(*from, *to).clone())
                        .0;

                    // Search for connections
                    let (layers, connections) = match to {
                        NodeRef::Hidden(_) => search::explore_substrate(
                            substrate_nodes
                                .get(from)
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<Vec<(i64, i64)>>(),
                            &vec![],
                            &mut cppn,
                            1,
                            false,
                            true,
                        ),
                        NodeRef::Output(_) => search::explore_substrate(
                            substrate_nodes
                                .get(to)
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<Vec<(i64, i64)>>(),
                            &vec![],
                            &mut cppn,
                            1,
                            true,
                            true,
                        ),
                        NodeRef::Input(_) => panic!("target is input substrate"),
                    };

                    // Add discovered nodes to target substrate
                    let nodes = substrate_nodes.get_mut(to).unwrap();
                    // First layer contains source nodes
                    // Never more than a single layer of new nodes since depth = 1
                    for node in layers.iter().skip(1).take(1).flatten() {
                        nodes.insert(*node);
                    }

                    // Add discovered connections to assembled network
                    for connection in connections.iter() {
                        assembled_connections.add(
                            (*from, connection.from.0, connection.from.1),
                            (*to, connection.to.0, connection.to.1),
                            connection.edge,
                        );
                    }
                }
                connection::OrderedAction::Node(node_ref) => match node_ref {
                    NodeRef::Hidden(_) => {
                        // Develop the node's cppn
                        let mut cppn = self
                            .cppn_developer
                            .develop(genome.get_node_cppn(node_ref).clone())
                            .0;

                        // Develop substrate
                        let (layers, connections) = search::explore_substrate(
                            substrate_nodes
                                .get(node_ref)
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<Vec<(i64, i64)>>(),
                            &vec![],
                            &mut cppn,
                            genome.get_depth(node_ref),
                            false,
                            false,
                        );

                        // Add discovered nodes to target substrate
                        let nodes = substrate_nodes.get_mut(node_ref).unwrap();
                        // First layer contains source nodes
                        for layer in layers.iter().skip(1) {
                            for node in layer.iter() {
                                nodes.insert(*node);
                            }
                        }
                        // Add discovered connections to assembled network
                        for connection in connections.iter() {
                            assembled_connections.add(
                                (*node_ref, connection.from.0, connection.from.1),
                                (*node_ref, connection.to.0, connection.to.1),
                                connection.edge,
                            );
                        }
                    }
                    _ => {}
                },
            }
        }

        assembled_connections.prune(&self.flattened_inputs, &self.flattened_outputs);
        assembled_connections
    }
}

impl<G: DesGenome> Develop<G> for Developer {
    type Phenotype = Executor;
    type Stats = MultiSubstrateNetworkStats;

    fn develop(&self, genome: G) -> (Self::Phenotype, Self::Stats) {
        // Let the genome prepeare to provide cppns and depth
        let mut genome = genome;
        genome.init_desgenome();

        // Init assembled network
        let mut assembled_connections = connection::Connections::<(NodeRef, i64, i64), f64>::new();

        // Init known nodes with the input and output nodes
        let mut substrate_nodes = HashMap::<NodeRef, HashSet<(i64, i64)>>::new();
        for (i, nodes) in self.input_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Input(i as u64),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
        for (i, nodes) in self.output_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Output(i as u64),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
        // All hidden substrates are empty
        for node_ref in genome.get_neat().hidden_nodes.keys() {
            substrate_nodes.insert(*node_ref, HashSet::new());
        }

        // Iterative network completion in topologically sorted order
        let order = genome.get_neat().connections.sort_topologically();
        for element in order.iter() {
            match element {
                connection::OrderedAction::Edge(from, to, _) => {
                    // Develop the link's cppn
                    let mut cppn = self
                        .cppn_developer
                        .develop(genome.get_link_cppn(*from, *to).clone())
                        .0;

                    // Search for connections
                    let (layers, connections) = match to {
                        NodeRef::Hidden(_) => search::explore_substrate(
                            substrate_nodes
                                .get(from)
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<Vec<(i64, i64)>>(),
                            &vec![],
                            &mut cppn,
                            1,
                            false,
                            true,
                        ),
                        NodeRef::Output(_) => search::explore_substrate(
                            substrate_nodes
                                .get(to)
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<Vec<(i64, i64)>>(),
                            &vec![],
                            &mut cppn,
                            1,
                            true,
                            true,
                        ),
                        NodeRef::Input(_) => panic!("target is input substrate"),
                    };

                    // Add discovered nodes to target substrate
                    let nodes = substrate_nodes.get_mut(to).unwrap();
                    // First layer contains source nodes
                    // Never more than a single layer of new nodes since depth = 1
                    for node in layers.iter().skip(1).take(1).flatten() {
                        nodes.insert(*node);
                    }

                    // Add discovered connections to assembled network
                    for connection in connections.iter() {
                        assembled_connections.add(
                            (*from, connection.from.0, connection.from.1),
                            (*to, connection.to.0, connection.to.1),
                            connection.edge
                                * genome
                                    .get_neat()
                                    .links
                                    .get(&(*from, *to))
                                    .unwrap()
                                    .neat()
                                    .weight,
                        );
                    }
                }
                connection::OrderedAction::Node(node_ref) => match node_ref {
                    NodeRef::Hidden(_) => {
                        // Develop the node's cppn
                        let mut cppn = self
                            .cppn_developer
                            .develop(genome.get_node_cppn(node_ref).clone())
                            .0;

                        // Develop substrate
                        let (layers, connections) = search::explore_substrate(
                            substrate_nodes
                                .get(node_ref)
                                .unwrap()
                                .iter()
                                .cloned()
                                .collect::<Vec<(i64, i64)>>(),
                            &vec![],
                            &mut cppn,
                            genome.get_depth(node_ref),
                            false,
                            false,
                        );
                        // Add discovered nodes to target substrate
                        let nodes = substrate_nodes.get_mut(node_ref).unwrap();
                        // First layer contains source nodes
                        for layer in layers.iter().skip(1) {
                            for node in layer.iter() {
                                nodes.insert(*node);
                            }
                        }

                        // Add discovered connections to assembled network
                        for connection in connections.iter() {
                            assembled_connections.add(
                                (*node_ref, connection.from.0, connection.from.1),
                                (*node_ref, connection.to.0, connection.to.1),
                                connection.edge,
                            );
                        }
                    }
                    _ => {}
                },
            }
        }

        // Collect all hidden nodes (in all hidden substrates)
        let mut hidden_nodes = Vec::<(NodeRef, i64, i64)>::new();
        for node_ref in genome.get_neat().hidden_nodes.keys() {
            hidden_nodes.extend(
                substrate_nodes
                    .get(node_ref)
                    .unwrap()
                    .iter()
                    .map(|node| (*node_ref, node.0, node.1)),
            );
        }

        // Collect all nodes (in all substrates)
        let nodes = self
            .flattened_inputs
            .iter()
            .cloned()
            .chain(hidden_nodes.drain(..))
            .chain(self.flattened_outputs.iter().cloned())
            .collect::<Vec<(NodeRef, i64, i64)>>();

        let first_output_id = nodes.len() - self.flattened_outputs.len();
        let inputs = (0..self.flattened_inputs.len()).collect();
        let outputs = (first_output_id..(first_output_id + self.flattened_outputs.len())).collect();

        // Create mapping from nodes to array index in Network's node vector
        let node_mapping: HashMap<(NodeRef, i64, i64), usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (*node, i))
            .collect();

        // Remove any node not on a path between input and output nodes
        assembled_connections.prune(&self.flattened_inputs, &self.flattened_outputs);

        // Map topologically sorted order to neural network actions.
        let actions = assembled_connections
            .sort_topologically()
            .iter()
            .map(|action| match action {
                connection::OrderedAction::Edge(from, to, weight) => Action::Link(
                    *node_mapping
                        .get(from)
                        .expect("map does not contain source node"),
                    *node_mapping
                        .get(to)
                        .expect("map does not contain target node"),
                    *weight,
                ),
                connection::OrderedAction::Node(node) => Action::Activation(
                    *node_mapping
                        .get(node)
                        .expect("map does not contain activation node"),
                    0.0,
                    if *node_mapping.get(node).unwrap() < first_output_id {
                        ESHYPERNEAT.hidden_activation
                    } else {
                        ESHYPERNEAT.output_activation
                    },
                ),
            })
            .collect::<Vec<_>>();

        let hidden_substrate_nodes = assembled_connections
            .get_all_nodes()
            .iter()
            .filter_map(|node| {
                if let (NodeRef::Hidden(id), _, _) = node {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect::<Vec<u64>>();
        let mut hidden_substrate_node_counts = HashMap::<u64, i64>::new();
        for node_id in hidden_substrate_nodes.iter() {
            hidden_substrate_node_counts.insert(
                *node_id,
                hidden_substrate_node_counts.get(node_id).unwrap_or(&0) + 1,
            );
        }

        let stats = MultiSubstrateNetworkStats {
            hidden_substrates: hidden_substrate_node_counts.len() as u64,
            hidden_substrate_node_counts: hidden_substrate_node_counts
                .values()
                .cloned()
                .collect::<Vec<i64>>(),
            network_stats: NetworkStats {
                nodes: assembled_connections.get_all_nodes().len() as u64,
                edges: assembled_connections.get_all_connections().len() as u64,
            },
        };
        let network = Executor::create(nodes.len(), inputs, outputs, actions);

        (network, stats)
    }
}

pub fn parse_nodes(conf: &String, r: i64, num: u64) -> Vec<Vec<(i64, i64)>> {
    match &conf[..] {
        "line" => vec![substrate::horizontal_row(num, 0)],
        "separate" => vec![vec![(0, 0)]; num as usize],
        _ => serde_json::from_str::<Vec<Vec<(i64, i64)>>>(conf)
            .expect("unable to parse nodes")
            .iter()
            .map(|nodes| nodes.iter().map(|node| (node.0 * r, node.1 * r)).collect())
            .collect(),
    }
}

pub fn parse_num_substrates(conf: &String, num: u64) -> u64 {
    match &conf[..] {
        "line" => 1,
        "separate" => num,
        _ => serde_json::from_str::<Vec<Vec<(i64, i64)>>>(conf)
            .expect("unable to parse num substrates")
            .len() as u64,
    }
}

pub fn topology_init_config(init_config: &EnvironmentDescription) -> InitConfig {
    let inputs = parse_num_substrates(&DESHYPERNEAT.input_config, init_config.inputs);
    let outputs = parse_num_substrates(&DESHYPERNEAT.output_config, init_config.outputs);
    InitConfig::new(inputs, outputs)
}
