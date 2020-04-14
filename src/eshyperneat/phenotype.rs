use crate::conf;
use crate::eshyperneat::search;
use crate::generic_neat::evaluate;
use crate::generic_neat::genome;
use crate::hyperneat::substrate;
use crate::neat::phenotype::Developer as NeatDeveloper;
use crate::network::activation;
use crate::network::connection;
use crate::network::execute;
use crate::network::execute::Executor as P;
use std::collections::HashMap;

pub struct Developer {
    neat_developer: NeatDeveloper,
}

impl Default for Developer {
    fn default() -> Self {
        Self {
            neat_developer: NeatDeveloper::default(),
        }
    }
}

impl evaluate::Develop<P> for Developer {
    fn develop(&self, genome: &genome::Genome) -> P {
        let mut neat_executor = self.neat_developer.develop(genome);

        let num_inputs = 13;
        let num_outputs = 3;
        let depth = 10;
        let input_nodes =
            substrate::horizontal_row(num_inputs, -conf::ESHYPERNEAT.resolution as i64);
        let output_nodes =
            substrate::horizontal_row(num_inputs, conf::ESHYPERNEAT.resolution as i64);

        let (layers, connections) = search::explore_substrate(
            input_nodes.iter().cloned().collect(),
            &mut neat_executor,
            depth,
        );

        /*for layer in layers.iter() {
            println!("{:?}", layer);
        }
        println!("");*/

        let mut nodes = input_nodes
            .iter()
            .chain(
                layers
                    .iter()
                    .flatten()
                    .filter(|n| !output_nodes.contains(n)),
            )
            .cloned()
            .collect::<Vec<(i64, i64)>>();

        let inputs = (0..num_inputs).collect();
        let outputs = (nodes.len()..(nodes.len() + num_outputs)).collect();

        nodes.extend(output_nodes.iter());

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
                    *node_mapping.get(node).unwrap(),
                    0.0,
                    activation::Activation::ReLU,
                ),
            })
            .collect();

        // Create neural network executor
        execute::Executor::create(nodes.len(), inputs, outputs, actions)
    }
}
