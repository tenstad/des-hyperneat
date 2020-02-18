use crate::generic_neat::evaluate;
use crate::generic_neat::genome;
use crate::generic_neat::node::NodeRef;
use crate::network::execute;
use crate::network::execute::Executor as P;
use crate::network::order;
use std::collections::HashMap;

pub struct Developer;

impl Default for Developer {
    fn default() -> Developer {
        Developer {}
    }
}

impl evaluate::Develop<P> for Developer {
    fn develop(&self, genome: &genome::Genome) -> P {
        let input_length = genome.inputs.len();
        let cumulative_hidden_length = input_length + genome.hidden_nodes.len(); // Length of input and hidden
        let cumulative_output_length = cumulative_hidden_length + genome.outputs.len(); // Length of input, hidden and output

        let mut input_keys: Vec<NodeRef> = genome.inputs.keys().cloned().collect();
        input_keys.sort();
        let mut output_keys: Vec<NodeRef> = genome.outputs.keys().cloned().collect();
        output_keys.sort();

        let node_mapper: HashMap<NodeRef, usize> = input_keys
            .iter()
            .enumerate()
            .map(|(i, node_ref)| (*node_ref, i))
            .chain(
                genome
                    .hidden_nodes
                    .keys()
                    .enumerate()
                    .map(|(i, node_ref)| (*node_ref, i + input_length)),
            )
            .chain(
                output_keys
                    .iter()
                    .enumerate()
                    .map(|(i, node_ref)| (*node_ref, i + cumulative_hidden_length)),
            )
            .collect();

        let actions = genome
            .order
            .iter()
            .map(|action| match action {
                order::Action::Link(from, to) => execute::Action::Link(
                    *node_mapper.get(from).unwrap(),
                    *node_mapper.get(to).unwrap(),
                    genome.links.get(&(*from, *to)).unwrap().weight,
                ),
                order::Action::Activation(node) => execute::Action::Activation(
                    *node_mapper.get(node).unwrap(),
                    genome.get_bias(node),
                    genome.get_activation(node),
                ),
            })
            .collect();

        execute::Executor::create(
            cumulative_output_length,
            input_keys.iter().map(|node| node.id() as usize).collect(),
            output_keys
                .iter()
                .map(|node| node.id() as usize + cumulative_hidden_length)
                .collect(),
            actions,
        )
    }
}
