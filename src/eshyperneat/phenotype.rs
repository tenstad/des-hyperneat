use crate::conf;
use crate::eshyperneat::search;
use crate::generic_neat::evaluate;
use crate::generic_neat::genome;
use crate::hyperneat::substrate;
use crate::neat::phenotype::Developer as NeatDeveloper;
use network::activation;
use network::connection;
use network::execute;
use std::collections::{HashMap, HashSet};

pub struct Developer {
    neat_developer: NeatDeveloper,
    input_nodes: Vec<(i64, i64)>,
    output_nodes: Vec<(i64, i64)>,
    depth: usize,
}

impl Default for Developer {
    fn default() -> Self {
        Self {
            neat_developer: NeatDeveloper::default(),
            input_nodes: substrate::horizontal_row(13, -conf::ESHYPERNEAT.resolution as i64),
            output_nodes: substrate::horizontal_row(3, conf::ESHYPERNEAT.resolution as i64),
            depth: conf::ESHYPERNEAT.iteration_level + 1,
        }
    }
}

impl Developer {
    // Creates a phenotype with all 0 outputs.
    fn disconnected(&self) -> execute::Executor {
        let num_outputs = self.output_nodes.len();
        execute::Executor::create(
            num_outputs,
            Vec::new(),
            (0..num_outputs).collect(),
            Vec::new(),
        )
    }

    // Copy of part of the evolution below. This should be avoided
    // if there is an eqally fast option mergining the two
    pub fn connections(
        &self,
        cppn: &mut execute::Executor,
    ) -> connection::Connections<(i64, i64), f64> {
        // Forward search with depth
        let (_, mut connections) = search::explore_substrate(
            self.input_nodes.clone(),
            &self.output_nodes,
            cppn,
            self.depth,
            false,
        );

        // Backward output-connecting search with depth 1
        let (_, reverse_connections) =
            search::explore_substrate(self.output_nodes.clone(), &self.input_nodes, cppn, 1, true);

        connections.extend(&reverse_connections);
        connections.prune(&self.input_nodes, &self.output_nodes);

        connections
    }
}

impl evaluate::Develop<execute::Executor> for Developer {
    fn develop(&self, genome: &genome::Genome) -> execute::Executor {
        let mut cppn = self.neat_developer.develop(genome);

        // Forward search with depth
        let (layers, mut connections) = search::explore_substrate(
            self.input_nodes.clone(),
            &self.output_nodes,
            &mut cppn,
            self.depth,
            false,
        );

        // Backward output-connecting search with depth 1
        let (reverse_layers, reverse_connections) = search::explore_substrate(
            self.output_nodes.clone(),
            &self.input_nodes,
            &mut cppn,
            1,
            true,
        );

        connections.extend(&reverse_connections);
        connections.prune(&self.input_nodes, &self.output_nodes);

        // Make sure the order is inputs - hidden - outputs
        let nodes = self
            .input_nodes
            .iter()
            .cloned()
            .chain(
                layers
                    .iter()
                    .skip(1)
                    .flatten()
                    .chain(reverse_layers.iter().skip(1).flatten())
                    .cloned()
                    .collect::<HashSet<(i64, i64)>>()
                    .into_iter(),
            )
            .chain(self.output_nodes.iter().cloned())
            .collect::<Vec<(i64, i64)>>();

        let first_output_id = nodes.len() - self.output_nodes.len();
        let inputs = (0..self.input_nodes.len()).collect();
        let outputs = (first_output_id..(first_output_id + self.output_nodes.len())).collect();

        // Create mapping from nodes to array index in Network's node vector
        let node_mapping: HashMap<(i64, i64), usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node_ref)| (*node_ref, i))
            .collect();

        // Map topologically sorted order to neural network actions.
        let actions = connections
            .sort_topologically()
            .iter()
            .map(|action| match action {
                connection::OrderedAction::Link(from, to, weight) => execute::Action::Link(
                    *node_mapping
                        .get(from)
                        .expect("map does not contain source node"),
                    *node_mapping
                        .get(to)
                        .expect("map does not contain target node"),
                    *weight,
                ),
                connection::OrderedAction::Activation(node) => execute::Action::Activation(
                    *node_mapping
                        .get(node)
                        .expect("map does not contain activation node"),
                    cppn.execute(&vec![
                        0.0,
                        0.0,
                        node.0 as f64 / conf::ESHYPERNEAT.resolution,
                        node.1 as f64 / conf::ESHYPERNEAT.resolution,
                    ])[1],
                    if *node_mapping.get(node).unwrap() < first_output_id {
                        activation::Activation::ReLU
                    } else {
                        activation::Activation::Softmax
                    },
                ),
            })
            .collect();

        // Create neural network executor
        execute::Executor::create(nodes.len(), inputs, outputs, actions)
    }
}
