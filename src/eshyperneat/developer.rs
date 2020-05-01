use crate::cppn::{developer::Developer as CppnDeveloper, genome::Genome};
use crate::eshyperneat::{conf::ESHYPERNEAT, search};
use crate::hyperneat::substrate;
use evolution::{develop::Develop, environment::EnvironmentDescription};
use network::{
    activation, connection,
    execute::{Action, Executor},
};
use std::collections::{HashMap, HashSet};

pub struct Developer {
    neat_developer: CppnDeveloper,
    input_nodes: Vec<(i64, i64)>,
    output_nodes: Vec<(i64, i64)>,
    depth: usize,
}

impl From<EnvironmentDescription> for Developer {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            input_nodes: substrate::horizontal_row(
                description.inputs,
                -ESHYPERNEAT.resolution as i64,
            ),
            output_nodes: substrate::horizontal_row(
                description.outputs,
                ESHYPERNEAT.resolution as i64,
            ),
            neat_developer: CppnDeveloper::from(description),
            depth: ESHYPERNEAT.iteration_level + 1,
        }
    }
}

impl Developer {
    // Creates a phenotype with all 0 outputs.
    fn disconnected(&self) -> Executor {
        let num_outputs = self.output_nodes.len();
        Executor::create(
            num_outputs,
            Vec::new(),
            (0..num_outputs).collect(),
            Vec::new(),
        )
    }

    // Copy of part of the evolution below. This should be avoided
    // if there is an eqally fast option mergining the two
    pub fn connections(&self, cppn: &mut Executor) -> connection::Connections<(i64, i64), f64> {
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

impl Develop<Genome, Executor> for Developer {
    fn develop(&self, genome: &Genome) -> Executor {
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
                connection::OrderedAction::Link(from, to, weight) => Action::Link(
                    *node_mapping
                        .get(from)
                        .expect("map does not contain source node"),
                    *node_mapping
                        .get(to)
                        .expect("map does not contain target node"),
                    *weight,
                ),
                connection::OrderedAction::Activation(node) => Action::Activation(
                    *node_mapping
                        .get(node)
                        .expect("map does not contain activation node"),
                    cppn.execute(&vec![
                        0.0,
                        0.0,
                        node.0 as f64 / ESHYPERNEAT.resolution,
                        node.1 as f64 / ESHYPERNEAT.resolution,
                    ])[1],
                    if *node_mapping.get(node).unwrap() < first_output_id {
                        ESHYPERNEAT.hidden_activation
                    } else {
                        ESHYPERNEAT.output_activation
                    },
                ),
            })
            .collect();

        // Create neural network executor
        Executor::create(nodes.len(), inputs, outputs, actions)
    }
}
