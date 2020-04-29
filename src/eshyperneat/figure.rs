use figure;
use network::connection::Connections;
use std::collections::HashMap;
use std::path::Path;

pub fn save_fig_to_file<P: AsRef<Path>>(
    connections: Connections<(i64, i64), f64>,
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
    fig.add(
        figure::substrate::SubstrateBuilder::default()
            .size(size)
            .cells(2)
            .build()
            .unwrap(),
    );

    let mut nodes: HashMap<(i64, i64), figure::node::Node> = connections
        .get_all_nodes()
        .iter()
        .map(|node| {
            (
                (node.0, node.1),
                figure::node::NodeBuilder::default()
                    .x(node.0 as f64 * scale * size + size / 2.0)
                    .y(node.1 as f64 * scale * size + size / 2.0)
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
