use crate::cppn::{genome::Genome, node::Node as CppnNode};
use evolution::neat::{
    conf::{ConfigProvider, NeatConfig},
    genome::{Genome as NeatGenome, Node as NeatNode},
    link::LinkCore,
    node::{NodeCore, NodeRef},
    state::{InitConfig, StateCore, StateProvider},
};
use network::activation::Activation;

fn insert_link(
    genome: &mut Genome,
    state: &mut StateCore,
    from: NodeRef,
    to: NodeRef,
    weight: f64,
) {
    let innovation = state.get_connect_innovation(from, to);
    genome
        .core
        .insert_link(LinkCore::new(from, to, weight, innovation));
}

fn split_link(
    config: &NeatConfig,
    genome: &mut Genome,
    state: &mut StateCore,
    from: NodeRef,
    to: NodeRef,
    weight: f64,
    weight2: f64,
    activation: Activation,
    bias: f64,
) -> NodeRef {
    let innovation = state.get_split_innovation(
        genome
            .core
            .links
            .get(&(from, to))
            .expect("cannot split nonexisting link")
            .innovation,
    );
    let new_node = NodeRef::Hidden(innovation.node_number);

    genome.core.split_link(
        config,
        from,
        to,
        innovation.node_number,
        innovation.innovation_number,
        state,
    );
    let hidden_node = genome.core.hidden_nodes.get_mut(&new_node).unwrap();
    hidden_node.activation = activation;
    hidden_node.bias = bias;

    genome.core.links.get_mut(&(from, new_node)).unwrap().weight = weight;

    genome.core.links.get_mut(&(new_node, to)).unwrap().weight = weight2;

    new_node
}

pub fn insert_identity(
    config: &NeatConfig,
    genome: &mut Genome,
    state: &mut StateCore,
    output_id: usize,
) {
    if !genome
        .core
        .outputs
        .contains_key(&NodeRef::Output(output_id))
    {
        genome.core.outputs.insert(
            NodeRef::Output(output_id),
            CppnNode::new(
                config.get_node_config(),
                NodeCore::new(NodeRef::Output(output_id)),
                state.get_node_state_mut(),
            ),
        );
    }
    insert_link(
        genome,
        state,
        NodeRef::Input(0),
        NodeRef::Output(output_id),
        0.0,
    );
    insert_link(
        genome,
        state,
        NodeRef::Input(1),
        NodeRef::Output(output_id),
        0.0,
    );

    let hidden_x = split_link(
        config,
        genome,
        state,
        NodeRef::Input(0),
        NodeRef::Output(output_id),
        7.5,
        7.5,
        Activation::Square,
        0.0,
    );

    let hidden_y = split_link(
        config,
        genome,
        state,
        NodeRef::Input(1),
        NodeRef::Output(output_id),
        7.5,
        7.5,
        Activation::Square,
        0.0,
    );

    insert_link(genome, state, NodeRef::Input(2), hidden_x, -7.5);
    insert_link(genome, state, NodeRef::Input(3), hidden_y, -7.5);

    let output = genome
        .core
        .outputs
        .get_mut(&NodeRef::Output(output_id))
        .unwrap();
    output.activation = Activation::Gaussian;
    output.bias = 0.0;
}

pub fn identity_genome() -> (Genome, StateCore) {
    let config = NeatConfig::default();
    let init_config = InitConfig::new(4, 2);
    let mut state = StateCore::default();
    let mut genome = Genome::new(&config, &init_config, &mut state);

    insert_identity(&config, &mut genome, &mut state, 0);

    (genome, state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cppn::developer::Developer;
    use crate::eshyperneat::{conf::ESHYPERNEAT, search::find_connections};
    use evolution::{develop::Develop, environment::EnvironmentDescription};

    #[test]
    fn test_identity() {
        let (genome, _) = identity_genome();
        let developer = Developer::from(EnvironmentDescription::new(0, 0));
        let mut cppn = developer.develop(genome);
        let mut test_points = Vec::new();
        if ESHYPERNEAT.initial_resolution > 0 {
            test_points.push((0.5, -0.5));
        }
        if ESHYPERNEAT.initial_resolution > 1 {
            test_points.push((0.25, -0.25));
        }
        if ESHYPERNEAT.initial_resolution > 2 {
            test_points.push((0.125, 0.25 + 0.125));
        }
        if ESHYPERNEAT.initial_resolution > 3 {
            test_points.push((0.0625, 0.125 - 0.0625));
        }
        if ESHYPERNEAT.initial_resolution > 4 {
            test_points.push((0.03125, -0.0625 - 0.03125));
        }

        for (x, y) in test_points.iter() {
            println!("{} {}", x, y);
            let discoveries = find_connections(*x, *y, &mut cppn, false);
            assert_eq!(discoveries.len(), 1);
            assert_eq!(discoveries[0].node.0, *x);
            assert_eq!(discoveries[0].node.1, *y);
        }
    }
}
