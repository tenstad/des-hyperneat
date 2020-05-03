use evolution::neat::node::NodeRef;
use figure;
use network::connection::Connections;
use std::collections::HashMap;
use std::iter;
use std::path::Path;

pub fn save_fig_to_file<P: AsRef<Path>>(
    connections: Connections<(NodeRef, i64, i64), f64>,
    fname: P,
    scale: f64,
    size: f64,
) {
    let max_weight = connections
        .get_all_connections()
        .iter()
        .map(|connection| connection.edge.abs())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);

    let mut fig = figure::Figure::new(1.0);

    let mut input_x = 0.0;
    let mut hidden_x = 0.0;
    let mut output_x = 0.0;
    let mut substrate_location = HashMap::<NodeRef, (f64, f64)>::new();
    for connection in connections.get_all_connections() {
        for node_ref in iter::once(connection.from.0).chain(iter::once(connection.to.0)) {
            if !substrate_location.contains_key(&node_ref) {
                match node_ref {
                    NodeRef::Input(_) => {
                        substrate_location.insert(node_ref, (input_x, 0.0));
                        input_x += 3.0;
                    }
                    NodeRef::Hidden(_) => {
                        substrate_location.insert(node_ref, (hidden_x, 3.0));
                        hidden_x += 3.0;
                    }
                    NodeRef::Output(_) => {
                        substrate_location.insert(node_ref, (output_x, 6.0));
                        output_x += 3.0;
                    }
                }
            }
        }
    }

    let input_offset = (hidden_x - input_x) / 2.0;
    let output_offset = (hidden_x - output_x) / 2.0;
    for (node_ref, position) in substrate_location.iter_mut() {
        match node_ref {
            NodeRef::Input(_) => {
                position.0 += input_offset;
            }
            NodeRef::Hidden(_) => {}
            NodeRef::Output(_) => {
                position.0 += output_offset;
            }
        }
    }

    for (x, y) in substrate_location.values() {
        fig.add(
            figure::substrate::SubstrateBuilder::default()
                .x(*x * size)
                .y(*y * size)
                .size(size)
                .cells(2)
                .build()
                .unwrap(),
        );
    }

    let mut nodes: HashMap<(NodeRef, i64, i64), figure::node::Node> = connections
        .get_all_nodes()
        .iter()
        .map(|node| {
            (
                *node,
                figure::node::NodeBuilder::default()
                    .x(node.1 as f64 * scale * size
                        + size / 2.0
                        + substrate_location.get(&node.0).unwrap().0 * size / 2.0)
                    .y(node.2 as f64 * scale * size
                        + size / 2.0
                        + substrate_location.get(&node.0).unwrap().1 * size / 2.0)
                    .fill("black")
                    .outline("white")
                    .size(1.0)
                    .edge_offset(0.0)
                    .build()
                    .unwrap(),
            )
        })
        .collect();

    for edge in connections.get_all_connections().iter().map(|connection| {
        figure::edge::EdgeBuilder::new(
            nodes.get(&connection.from).unwrap(),
            nodes.get(&connection.to).unwrap(),
        )
        .opacity((connection.edge.abs() / max_weight) * 0.8 + 0.2)
        .width(0.1)
        .build()
        .unwrap()
    }) {
        fig.add(edge);
    }

    for node in nodes.drain() {
        fig.add(node.1);
    }

    fig.save(fname);
}
