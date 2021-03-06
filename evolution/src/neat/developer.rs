use crate::develop::Develop;
use crate::environment::EnvironmentDescription;
use crate::neat::{
    conf::NEAT,
    genome::NeatGenome,
    link::NeatLink,
    node::{NeatNode, NodeRef},
};
use crate::stats::Stats;
use bson;
use network::{connection, execute, execute::Executor};
use serde::Serialize;
use std::collections::HashMap;

pub struct Developer;

impl From<EnvironmentDescription> for Developer {
    fn from(_description: EnvironmentDescription) -> Self {
        Developer {}
    }
}

#[derive(Serialize, new)]
pub struct NetworkStats {
    #[serde(with = "bson::compat::u2f")]
    pub nodes: u64,
    #[serde(with = "bson::compat::u2f")]
    pub edges: u64,
}

impl Stats for NetworkStats {}

impl Develop<NeatGenome<NeatNode, NeatLink>> for Developer {
    type Phenotype = Executor;
    type Stats = NetworkStats;

    fn develop(&self, genome: NeatGenome<NeatNode, NeatLink>) -> (Self::Phenotype, Self::Stats) {
        // Sort genomes netowrk topologically
        let order = genome.connections.sort_topologically();

        // Create vector of all input node indexes, for insertion of nerual network inputs
        let num_input_nodes = genome.inputs.keys().map(|n| n.id()).max().unwrap() as usize + 1;
        let inputs = (0..num_input_nodes).collect::<Vec<usize>>();

        // Prepend input nodes to extraction of hidden nodes from topological sorting
        let mut nodes = inputs
            .iter()
            .map(|id| NodeRef::Input(*id as u64))
            .chain(order.iter().filter_map(|action| match action {
                connection::OrderedAction::Node(NodeRef::Hidden(id)) => Some(NodeRef::Hidden(*id)),
                _ => None,
            }))
            .collect::<Vec<NodeRef>>();

        // Create vector of all output node indexes, for extraction of nerual network execution result
        let num_output_nodes = genome.outputs.keys().map(|n| n.id()).max().unwrap() as usize + 1;
        let outputs = (nodes.len()..(nodes.len() + num_output_nodes)).collect();

        // Append all output nodes. Disconnected nodes (not present in topological sorting)
        // are added to make the output vector of the correct size. If num_output_nodes grows
        // with evolution, this could use the highest known num_output_nodes of all genomes.
        nodes.extend((0..(num_output_nodes)).map(|i| NodeRef::Output(i as u64)));

        // Create mapping from NodeRef to array index in Network's node vector
        let node_mapping: HashMap<NodeRef, usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node_ref)| (*node_ref, i))
            .collect();

        // Map topologically sorted order to neural network actions. Filter disabled edges, as
        // these are present in Connections to avoid cycles when re-enabling disabled edges.
        let actions = order
            .iter()
            .map(|action| match action {
                connection::OrderedAction::Edge(from, to, _) => {
                    let link = genome.links.get(&(*from, *to)).unwrap();
                    execute::Action::Link(
                        *node_mapping.get(from).unwrap(),
                        *node_mapping.get(to).unwrap(),
                        link.weight,
                    )
                }
                connection::OrderedAction::Node(node) => execute::Action::Activation(
                    *node_mapping.get(node).unwrap(),
                    0.0,
                    if let NodeRef::Output(_) = node {
                        NEAT.output_activation
                    } else {
                        network::activation::Activation::Sigmoid
                    },
                ),
            })
            .collect();

        // Create neural network executor
        let network = Executor::create(nodes.len(), inputs, outputs, actions);
        let stats = NetworkStats {
            nodes: nodes.len() as u64,
            edges: genome.links.len() as u64,
        };

        (network, stats)
    }
}
