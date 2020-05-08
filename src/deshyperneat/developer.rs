use crate::cppn::developer::Developer as CppnDeveloper;
use crate::deshyperneat::desgenome::DesGenome;
use crate::eshyperneat::{conf::ESHYPERNEAT, search};
use crate::hyperneat::substrate;
use evolution::{
    develop::Develop,
    environment::EnvironmentDescription,
    neat::{genome::Link, node::NodeRef},
};
use network::{
    connection,
    execute::{Action, Executor},
};
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
        let _input_nodes = (0..description.inputs)
            .map(|_| vec![(0, 0)])
            .collect::<Vec<Vec<(i64, i64)>>>();
        let _output_nodes = (0..description.outputs)
            .map(|_| vec![(0, 0)])
            .collect::<Vec<Vec<(i64, i64)>>>();
        let input_nodes = vec![
            vec![(-r, -r), (-r, r), (r, -r), (r, r)],
            vec![(-r, -r), (-r, r), (r, -r), (r, r)],
            vec![(-r, -r), (-r, r), (r, -r), (r, r), (0, 0)],
        ];
        let output_nodes = vec![substrate::horizontal_row(description.outputs, 0)];
        let flattened_inputs = input_nodes
            .iter()
            .enumerate()
            .flat_map(|(i, nodes)| {
                nodes
                    .iter()
                    .map(move |node| (NodeRef::Input(i), node.0, node.1))
            })
            .collect::<Vec<(NodeRef, i64, i64)>>();
        let flattened_outputs = output_nodes
            .iter()
            .enumerate()
            .flat_map(|(i, nodes)| {
                nodes
                    .iter()
                    .map(move |node| (NodeRef::Output(i), node.0, node.1))
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

impl Developer {
    pub fn connections<G: DesGenome>(
        &self,
        genome: &G,
    ) -> connection::Connections<(NodeRef, i64, i64), f64> {
        // Init assembled network
        let mut assembled_connections = connection::Connections::<(NodeRef, i64, i64), f64>::new();

        // Init known nodes with the input and output nodes
        let mut substrate_nodes = HashMap::<NodeRef, HashSet<(i64, i64)>>::new();
        for (i, nodes) in self.input_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Input(i),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
        for (i, nodes) in self.output_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Output(i),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
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
                        .cppn_developer
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
                        let mut cppn = self.cppn_developer.develop(genome.get_node_cppn(*node_ref));

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

impl<G: DesGenome> Develop<G, Executor> for Developer {
    fn develop(&self, genome: &G) -> Executor {
        // Init assembled network
        let mut assembled_connections = connection::Connections::<(NodeRef, i64, i64), f64>::new();

        // Init known nodes with the input and output nodes
        let mut substrate_nodes = HashMap::<NodeRef, HashSet<(i64, i64)>>::new();
        for (i, nodes) in self.input_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Input(i),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
        for (i, nodes) in self.output_nodes.iter().enumerate() {
            substrate_nodes.insert(
                NodeRef::Output(i),
                nodes.iter().cloned().collect::<HashSet<(i64, i64)>>(),
            );
        }
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
                        .cppn_developer
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
                        let mut cppn = self.cppn_developer.develop(genome.get_node_cppn(*node_ref));

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
            .collect();

        Executor::create(nodes.len(), inputs, outputs, actions)
    }
}
