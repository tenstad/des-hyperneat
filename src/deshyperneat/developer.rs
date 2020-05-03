use crate::cppn::developer::Developer as CppnDeveloper;
use crate::deshyperneat::desgenome::DesGenome;
use crate::eshyperneat::{conf::ESHYPERNEAT, search};
use crate::hyperneat::substrate;
use evolution::{
    develop::Develop,
    environment::EnvironmentDescription,
    neat::{genome::GenomeComponent, node::NodeRef},
};
use network::{
    connection,
    execute::{Action, Executor},
};
use std::collections::{HashMap, HashSet};

pub struct Developer {
    neat_developer: CppnDeveloper,
    input_nodes: Vec<(i64, i64)>,
    output_nodes: Vec<(i64, i64)>,
    substrate_input_nodes: Vec<(NodeRef, i64, i64)>,
    substrate_output_nodes: Vec<(NodeRef, i64, i64)>,
}

impl From<EnvironmentDescription> for Developer {
    fn from(description: EnvironmentDescription) -> Self {
        let input_nodes = substrate::horizontal_row(description.inputs, 0);
        let output_nodes = substrate::horizontal_row(description.outputs, 0);
        let substrate_input_nodes = input_nodes
            .iter()
            .map(|node| (NodeRef::Input(0), node.0, node.1))
            .collect::<Vec<(NodeRef, i64, i64)>>();
        let substrate_output_nodes = output_nodes
            .iter()
            .map(|node| (NodeRef::Output(0), node.0, node.1))
            .collect::<Vec<(NodeRef, i64, i64)>>();
        Self {
            neat_developer: CppnDeveloper::from(description),
            input_nodes,
            output_nodes,
            substrate_input_nodes,
            substrate_output_nodes,
        }
    }
}

impl Developer {
    pub fn connections<G: DesGenome>(
        &self,
        genome: &G,
    ) -> connection::Connections<(NodeRef, i64, i64), f64> {
        // Init assembled network
        let mut assembled_connections = connection::Connections::<(NodeRef, i64, i64), f64>::new();

        // Init known nodes with the input and output nodes
        let mut substrate_nodes = HashMap::<NodeRef, HashSet<(i64, i64)>>::new();
        substrate_nodes.insert(
            NodeRef::Input(0),
            self.input_nodes
                .iter()
                .cloned()
                .collect::<HashSet<(i64, i64)>>(),
        );
        substrate_nodes.insert(
            NodeRef::Output(0),
            self.output_nodes
                .iter()
                .cloned()
                .collect::<HashSet<(i64, i64)>>(),
        );
        // All hidden substrates are empty
        for node_ref in genome.get_core().hidden_nodes.keys() {
            substrate_nodes.insert(*node_ref, HashSet::new());
        }

        // Iterative network completion in topologically sorted order
        let order = genome.get_core().connections.sort_topologically();
        for element in order.iter() {
            match element {
                connection::OrderedAction::Edge(from, to, _) => {
                    // Develop the link's cppn
                    let mut cppn = self
                        .neat_developer
                        .develop(genome.get_link_cppn(*from, *to));

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
                        ),
                        NodeRef::Output(_) => search::explore_substrate(
                            self.output_nodes.clone(),
                            &vec![],
                            &mut cppn,
                            1,
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
                    for connection in connections.get_all_connections() {
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
                        let mut cppn = self.neat_developer.develop(genome.get_node_cppn(*node_ref));

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
                            genome.get_depth(*node_ref),
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
                        for connection in connections.get_all_connections() {
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

        assembled_connections.prune(&self.substrate_input_nodes, &self.substrate_output_nodes);
        assembled_connections
    }
}

impl<G: DesGenome> Develop<G, Executor> for Developer {
    fn develop(&self, genome: &G) -> Executor {
        // Init assembled network
        let mut assembled_connections = connection::Connections::<(NodeRef, i64, i64), f64>::new();

        // Init known nodes with the input and output nodes
        let mut substrate_nodes = HashMap::<NodeRef, HashSet<(i64, i64)>>::new();
        substrate_nodes.insert(
            NodeRef::Input(0),
            self.input_nodes
                .iter()
                .cloned()
                .collect::<HashSet<(i64, i64)>>(),
        );
        substrate_nodes.insert(
            NodeRef::Output(0),
            self.output_nodes
                .iter()
                .cloned()
                .collect::<HashSet<(i64, i64)>>(),
        );
        // All hidden substrates are empty
        for node_ref in genome.get_core().hidden_nodes.keys() {
            substrate_nodes.insert(*node_ref, HashSet::new());
        }

        // Iterative network completion in topologically sorted order
        let order = genome.get_core().connections.sort_topologically();
        for element in order.iter() {
            match element {
                connection::OrderedAction::Edge(from, to, _) => {
                    // Develop the link's cppn
                    let mut cppn = self
                        .neat_developer
                        .develop(genome.get_link_cppn(*from, *to));

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
                        ),
                        NodeRef::Output(_) => search::explore_substrate(
                            self.output_nodes.clone(),
                            &vec![],
                            &mut cppn,
                            1,
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
                    for connection in connections.get_all_connections() {
                        assembled_connections.add(
                            (*from, connection.from.0, connection.from.1),
                            (*to, connection.to.0, connection.to.1),
                            connection.edge
                                * genome
                                    .get_core()
                                    .links
                                    .get(&(*from, *to))
                                    .unwrap()
                                    .get_core()
                                    .weight,
                        );
                    }
                }
                connection::OrderedAction::Node(node_ref) => match node_ref {
                    NodeRef::Hidden(_) => {
                        // Develop the node's cppn
                        let mut cppn = self.neat_developer.develop(genome.get_node_cppn(*node_ref));

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
                            genome.get_depth(*node_ref),
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
                        for connection in connections.get_all_connections() {
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
        for node_ref in genome.get_core().hidden_nodes.keys() {
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
            .substrate_input_nodes
            .iter()
            .cloned()
            .chain(hidden_nodes.drain(..))
            .chain(self.substrate_output_nodes.iter().cloned())
            .collect::<Vec<(NodeRef, i64, i64)>>();

        let first_output_id = nodes.len() - self.output_nodes.len();
        let inputs = (0..self.input_nodes.len()).collect();
        let outputs = (first_output_id..(first_output_id + self.output_nodes.len())).collect();

        // Create mapping from nodes to array index in Network's node vector
        let node_mapping: HashMap<(NodeRef, i64, i64), usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (*node, i))
            .collect();

        // Remove any node not on a path between input and output nodes
        assembled_connections.prune(&self.substrate_input_nodes, &self.substrate_output_nodes);

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
            .collect();

        Executor::create(nodes.len(), inputs, outputs, actions)
    }
}
