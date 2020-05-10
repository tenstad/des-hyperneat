use crate::cppn::{conf::CPPN, Genome};
use evolution::{
    develop::Develop,
    environment::EnvironmentDescription,
    neat::{genome::Genome as NeatGenome, node::NodeRef},
};
use network::{connection, execute, execute::Executor};
use std::collections::HashMap;

pub struct Developer {}

impl From<EnvironmentDescription> for Developer {
    fn from(_: EnvironmentDescription) -> Self {
        Developer {}
    }
}

impl Develop<Genome, Executor> for Developer {
    fn develop(&self, genome: Genome) -> Executor {
        // Sort genomes netowrk topologically
        let order = genome.get_core().connections.sort_topologically();

        // Create vector of all input node indexes, for insertion of nerual network inputs
        let num_input_nodes = genome
            .get_core()
            .inputs
            .keys()
            .map(|n| n.id())
            .max()
            .unwrap() as usize
            + 1;
        let inputs = (0..num_input_nodes).collect::<Vec<usize>>();

        // Prepend input nodes to extraction of hidden nodes from topological sorting
        let mut nodes = inputs
            .iter()
            .map(|id| NodeRef::Input(*id))
            .chain(order.iter().filter_map(|action| match action {
                connection::OrderedAction::Node(NodeRef::Hidden(id)) => Some(NodeRef::Hidden(*id)),
                _ => None,
            }))
            .collect::<Vec<NodeRef>>();

        // Create vector of all output nodes. If pad_missing_outputs is false,
        // nonexisting output nodes will not be present in the network output. If
        // it is true, the network output will be padded with missing nodes.
        // Example with pad_missing_outputs = false and nodes Output(0), Output(2)
        // Output of netowork will the output of the two nodes: [O0, O2]

        // Example with pad_missing_outputs = false and nodes Output(0), Output(2)
        // Output of netowork will be padded with missing output nodes: [O0, O1, O2]
        let output_nodes = if CPPN.pad_missing_outputs {
            let num_output_nodes = genome
                .get_core()
                .outputs
                .keys()
                .map(|n| n.id())
                .max()
                .unwrap() as usize
                + 1;

            (0..(num_output_nodes))
                .map(|i| NodeRef::Output(i))
                .collect()
        } else {
            let mut output_nodes = genome
                .get_core()
                .outputs
                .keys()
                .cloned()
                .collect::<Vec<NodeRef>>();
            output_nodes.sort();
            output_nodes
        };
        let num_output_nodes = output_nodes.len();
        // Create vector of all output node indexes, for extraction of nerual network execution result
        let outputs = (nodes.len()..(nodes.len() + num_output_nodes)).collect();

        // Append all output nodes. Disconnected nodes (not present in topological sorting)
        // are added to make the output vector of the correct size. If num_output_nodes grows
        // with evolution, this could use the highest known num_output_nodes of all genomes.
        nodes.extend(output_nodes.iter());

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
                connection::OrderedAction::Edge(from, to, _) => {
                    let link = genome.get_core().links.get(&(*from, *to)).unwrap();
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
                connection::OrderedAction::Node(node) => Some(execute::Action::Activation(
                    *node_mapping.get(node).unwrap(),
                    genome.get_bias(node),
                    genome.get_activation(node),
                )),
            })
            .collect();

        // Create neural network executor
        Executor::create(nodes.len(), inputs, outputs, actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cppn::genome::Genome as CppnGenome;
    use evolution::neat::{
        genome::Genome,
        link::LinkCore,
        state::{InitConfig, StateCore},
    };
    use network::activation::Activation;

    #[test]
    fn test_develop() {
        let mut state = StateCore::default();
        let mut genome = CppnGenome::new(&InitConfig::new(4, 2), &mut state);
        let link = LinkCore::new(NodeRef::Input(1), NodeRef::Output(1), 3.0, 0);

        let core = genome.get_core_mut();
        core.insert_link(link.clone());
        core.split_link(link.from, link.to, 0, 1, &mut state);

        core.inputs.get_mut(&NodeRef::Input(1)).unwrap().activation = Activation::None;
        core.hidden_nodes
            .get_mut(&NodeRef::Hidden(0))
            .unwrap()
            .activation = Activation::Exp;
        core.outputs
            .get_mut(&NodeRef::Output(1))
            .unwrap()
            .activation = Activation::Sine;

        let mut phenotype = Developer::from(EnvironmentDescription::new(4, 2)).develop(genome);

        let result = phenotype.execute(&vec![5.0, 7.0, -1.0, -1.0]);
        assert_eq!(
            result,
            vec![
                0.0,
                Activation::Sine.activate(Activation::Exp.activate(3.0 * 7.0))
            ]
        );
    }
}
