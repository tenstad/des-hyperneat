use crate::cppn::genome::Genome;
use evolution::neat::{
    genome::Genome as NeatGenome,
    link::{DefaultLink, LinkCore},
    node::NodeRef,
    state::{InitConfig, Innovation, StateCore},
};
use network::activation::Activation;

fn insert_link(
    genome: &mut Genome,
    state: &mut StateCore,
    from: NodeRef,
    to: NodeRef,
    weight: f64,
) {
    let innovation = state.next_innovation.innovation_number;
    state.next_innovation.innovation_number += 1;

    state
        .innovation_log
        .edge_additions
        .insert((from, to), innovation);

    genome
        .get_core_mut()
        .insert_link(DefaultLink::new(LinkCore::new(
            from, to, weight, innovation,
        )));
}

fn split_link(
    genome: &mut Genome,
    state: &mut StateCore,
    from: NodeRef,
    to: NodeRef,
    weight: f64,
    activation: Activation,
    bias: f64,
) -> NodeRef {
    let node_number = state.next_innovation.node_number;
    let innovation_number = state.next_innovation.innovation_number;
    state.next_innovation.node_number += 1;
    state.next_innovation.innovation_number += 2;

    state.innovation_log.node_additions.insert(
        genome
            .get_core()
            .links
            .get(&(from, to))
            .expect("cannot split nonexisting link")
            .core
            .innovation,
        Innovation {
            node_number: node_number,
            innovation_number: innovation_number,
        },
    );

    let new_node = NodeRef::Hidden(node_number);
    genome
        .get_core_mut()
        .split_link(from, to, node_number, innovation_number, state);
    let hidden_node = genome
        .get_core_mut()
        .hidden_nodes
        .get_mut(&new_node)
        .unwrap();
    hidden_node.activation = activation;
    hidden_node.bias = bias;

    genome
        .get_core_mut()
        .links
        .get_mut(&(from, new_node))
        .unwrap()
        .core
        .weight = weight;

    new_node
}

pub fn identity_genome() -> (Genome, StateCore) {
    let init_config = InitConfig::new(4, 2);
    let mut state = StateCore::default();
    let mut genome = Genome::new(&init_config, &mut state);

    insert_link(
        &mut genome,
        &mut state,
        NodeRef::Input(0),
        NodeRef::Output(0),
        5.0,
    );

    insert_link(
        &mut genome,
        &mut state,
        NodeRef::Input(1),
        NodeRef::Output(0),
        5.0,
    );

    let hidden_x = split_link(
        &mut genome,
        &mut state,
        NodeRef::Input(0),
        NodeRef::Output(0),
        5.0,
        Activation::Square,
        0.0,
    );

    let hidden_y = split_link(
        &mut genome,
        &mut state,
        NodeRef::Input(1),
        NodeRef::Output(0),
        5.0,
        Activation::Square,
        0.0,
    );

    insert_link(&mut genome, &mut state, NodeRef::Input(2), hidden_x, -5.0);
    insert_link(&mut genome, &mut state, NodeRef::Input(3), hidden_y, -5.0);

    let output = genome
        .get_core_mut()
        .outputs
        .get_mut(&NodeRef::Output(0))
        .unwrap();
    output.activation = Activation::Gaussian;
    output.bias = 0.0;

    (genome, state)
}
