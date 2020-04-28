use crate::develop::Develop;
use crate::environment::EnvironmentDescription;
use crate::neat::{genome::DefaultNeatGenome as NeatGenome, node::NodeRef};
use network::{connection, execute, execute::Executor};
use std::collections::HashMap;

pub struct Developer;

impl From<EnvironmentDescription> for Developer {
    fn from(description: EnvironmentDescription) -> Self {
        Developer {}
    }
}

impl Develop<NeatGenome, Executor> for Developer {
    fn develop(&self, genome: &NeatGenome) -> Executor {
        // Sort genomes netowrk topologically
        let order = genome.connections.sort_topologically();

        // Create vector of all input node indexes, for insertion of nerual network inputs
        let num_input_nodes = genome.inputs.keys().map(|n| n.id()).max().unwrap() as usize + 1;
        let inputs = (0..num_input_nodes).collect::<Vec<usize>>();

        // Prepend input nodes to extraction of hidden nodes from topological sorting
        let mut nodes = inputs
            .iter()
            .map(|id| NodeRef::Input(*id))
            .chain(order.iter().filter_map(|action| match action {
                connection::OrderedAction::Activation(NodeRef::Hidden(id)) => {
                    Some(NodeRef::Hidden(*id))
                }
                _ => None,
            }))
            .collect::<Vec<NodeRef>>();

        // Create vector of all output node indexes, for extraction of nerual network execution result
        let num_output_nodes = genome.outputs.keys().map(|n| n.id()).max().unwrap() as usize + 1;
        let outputs = (nodes.len()..(nodes.len() + num_output_nodes)).collect();

        // Append all output nodes. Disconnected nodes (not present in topological sorting)
        // are added to make the output vector of the correct size. If num_output_nodes grows
        // with evolution, this could use the highest known num_output_nodes of all genomes.
        nodes.extend((0..(num_output_nodes)).map(|i| NodeRef::Output(i)));

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
            .filter_map(|action| match action {
                connection::OrderedAction::Link(from, to, _) => {
                    let link = genome.links.get(&(*from, *to)).unwrap();
                    if link.enabled {
                        Some(execute::Action::Link(
                            *node_mapping.get(from).unwrap(),
                            *node_mapping.get(to).unwrap(),
                            link.weight,
                        ))
                    } else {
                        None
                    }
                }
                connection::OrderedAction::Activation(node) => Some(execute::Action::Activation(
                    *node_mapping.get(node).unwrap(),
                    0.0,
                    network::activation::Activation::Sigmoid,
                )),
            })
            .collect();

        // Create neural network executor
        Executor::create(nodes.len(), inputs, outputs, actions)
    }
}
