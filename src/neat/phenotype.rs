use crate::neat::genome::{Genome as NeatGenome, InitConfig};
use crate::neat::node::NodeRef;
use evolution::environment::EnvironmentDescription;
use evolution::genome::Develop;
use network::{connection, execute, execute::Executor};
use std::collections::HashMap;

pub struct Developer {
    init_config: InitConfig,
}

impl From<EnvironmentDescription> for Developer {
    fn from(description: EnvironmentDescription) -> Self {
        Developer {
            init_config: InitConfig::new(description.inputs, description.outputs),
        }
    }
}

impl Develop<NeatGenome, Executor> for Developer {
    fn init_config(&self) -> &InitConfig {
        &self.init_config
    }

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
    use crate::neat::genome::Genome as NeatGenome;
    use crate::neat::{link, node::NodeRef};
    use evolution::environment::{DummyEnvironment, Environment};
    use evolution::{genome::Develop, genome::Genome};
    use network::activation;

    #[test]
    fn test_develop() {
        let environment = DummyEnvironment::new(EnvironmentDescription::new(4, 2));
        let developer = Developer::from(environment.description());
        let mut genome = NeatGenome::new(developer.init_config());
        let link = link::Link::new(NodeRef::Input(1), NodeRef::Output(1), 3.0, 0);

        genome.insert_link(link);
        genome.split_link(link, 0, 1);

        genome
            .inputs
            .get_mut(&NodeRef::Input(1))
            .unwrap()
            .activation = activation::Activation::None;
        genome
            .hidden_nodes
            .get_mut(&NodeRef::Hidden(0))
            .unwrap()
            .activation = activation::Activation::Exp;
        genome
            .outputs
            .get_mut(&NodeRef::Output(1))
            .unwrap()
            .activation = activation::Activation::Sine;

        let mut phenotype = developer.develop(&genome);

        let result = phenotype.execute(&vec![5.0, 7.0, -1.0, -1.0]);
        assert_eq!(
            result,
            vec![
                0.0,
                activation::Activation::Sine
                    .activate(3.0 * activation::Activation::Exp.activate(7.0))
            ]
        );
    }
}
