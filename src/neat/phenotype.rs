use crate::generic_neat::evaluate;
use crate::generic_neat::genome;
use crate::generic_neat::link;
use crate::generic_neat::node::NodeRef;
use crate::network::activation;
use crate::network::connection;
use crate::network::execute;
use crate::network::execute::Executor as P;
use std::collections::HashMap;

pub struct Developer;

impl Default for Developer {
    fn default() -> Developer {
        Developer {}
    }
}

impl evaluate::Develop<P> for Developer {
    fn develop(&self, genome: &genome::Genome) -> P {
        let mut input_keys: Vec<NodeRef> = genome.inputs.keys().cloned().collect();
        input_keys.sort();
        let mut output_keys: Vec<NodeRef> = genome.outputs.keys().cloned().collect();
        output_keys.sort();

        let order = genome.connections.sort_topologically();
        let mut nodes = order
            .iter()
            .filter_map(|action| match action {
                connection::OrderedAction::Activation(NodeRef::Input(id)) => {
                    Some(NodeRef::Input(*id))
                }
                connection::OrderedAction::Activation(NodeRef::Hidden(id)) => {
                    Some(NodeRef::Hidden(*id))
                }
                _ => None,
            })
            .collect::<Vec<NodeRef>>();
        nodes.sort();
        let max_output_id = genome.outputs.keys().map(|n| n.id()).max().unwrap() as usize;
        nodes.extend((0..(max_output_id as u64 + 1)).map(|i| NodeRef::Output(i)));

        let node_mapper: HashMap<NodeRef, usize> = nodes
            .iter()
            .enumerate()
            .map(|(i, node_ref)| (*node_ref, i))
            .collect();

        let actions = order
            .iter()
            .filter_map(|action| match action {
                connection::OrderedAction::Link(from, to) => {
                    if genome.links.get(&(*from, *to)).unwrap().enabled {
                        Some(execute::Action::Link(
                            *node_mapper.get(from).unwrap(),
                            *node_mapper.get(to).unwrap(),
                            genome.links.get(&(*from, *to)).unwrap().weight,
                        ))
                    } else {
                        None
                    }
                }
                connection::OrderedAction::Activation(node) => Some(execute::Action::Activation(
                    *node_mapper.get(node).unwrap(),
                    genome.get_bias(node),
                    genome.get_activation(node),
                )),
            })
            .collect();

        let first_output_position = *node_mapper.get(&NodeRef::Output(0)).unwrap();

        execute::Executor::create(
            nodes.len(),
            nodes
                .iter()
                .filter_map(|node| {
                    if let NodeRef::Input(id) = node {
                        Some(*id as usize)
                    } else {
                        None
                    }
                })
                .collect(),
            (first_output_position..(first_output_position + max_output_id + 1)).collect(),
            actions,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_develop() {
        let mut genome = genome::Genome::new(4, 2);
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

        let developer: &dyn evaluate::Develop<execute::Executor> = &Developer::default();
        let mut phenotype = developer.develop(&genome);

        let result = phenotype.execute(&vec![5.0, 7.0]);
        assert_eq!(
            result,
            vec![
                0.0,
                activation::Activation::Sine.activate(3.0 * activation::Activation::Exp.activate(7.0))
            ]
        );
    }
}
