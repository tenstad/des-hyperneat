use crate::network::basic_order;
use crate::network::connection;
use crate::network::order;

pub struct Network {
    pub length: usize,
    pub actions: Vec<Action>,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

pub type Point = (i64, u64, i64, u64);

pub enum Action {
    Activation(usize, f64, f64),            // node, x, y
    Link(usize, usize, f64, f64, f64, f64), // from, to, x0, y0, x1, y1
}

pub fn horizontal_point_line(y: f64, x0: f64, x1: f64, n: usize) -> Vec<Point> {
    assert!(x0 < x1);

    let map_2048 = |x: f64| (x * 2048.0).round() as i64;

    let distance = (x1 - x0) / (n as f64 - 1.0);
    let y = (y * 2048.0).round() as i64;

    (0..n)
        .map(|i| (map_2048(x0 + distance * i as f64), 2048, y, 2048))
        .collect()
}

impl Network {
    pub fn layered(layer_sizes: Vec<usize>) -> Network {
        let mut connections = connection::Connections::<Point>::new();

        let distance = 2.0 / ((layer_sizes.len() - 1) as f64);

        let layers: Vec<Vec<Point>> = layer_sizes
            .iter()
            .enumerate()
            .map(|(i, n)| horizontal_point_line(-1.0 + distance * (i as f64), -1.0, 1.0, *n))
            .collect();

        for i in 0..(layers.len() - 1) {
            for from in layers[i].iter() {
                for to in layers[i + 1].iter() {
                    connections.add_enabled(*from, *to);
                }
            }
        }

        Network::create(
            connections,
            layers.first().unwrap().iter().cloned().collect(),
            layers.last().unwrap().iter().cloned().collect(),
        )
    }

    pub fn create(
        connections: connection::Connections<Point>,
        inputs: Vec<Point>,
        outputs: Vec<Point>,
    ) -> Network {
        let mut order = basic_order::BasicOrder::<Point>::new();
        order.sort_topologically(&connections);

        let hiddens: Vec<Point> = order
            .iter()
            .filter(|action| {
                if let order::Action::Activation(node) = action {
                    !inputs.contains(node) && !outputs.contains(node)
                } else {
                    false
                }
            })
            .map(|action| {
                if let order::Action::Activation(node) = action {
                    node
                } else {
                    assert!(false);
                    &(0, 0, 0, 0)
                }
            })
            .cloned()
            .collect();

        // Slow but only performed on startup
        let all_nodes: Vec<Point> = inputs
            .iter()
            .chain(hiddens.iter())
            .chain(outputs.iter())
            .cloned()
            .collect();
        let index_of = |node| all_nodes.iter().position(|x| node == x).unwrap();

        let input_length = inputs.len();
        let cumulative_hidden_length = input_length + hiddens.len();
        let cumulative_output_length = cumulative_hidden_length + outputs.len();

        let actions = order
            .iter()
            .map(|action| match action {
                order::Action::Activation(node) => Action::Activation(
                    index_of(node),
                    (node.0 as f64) / (node.1 as f64),
                    (node.2 as f64) / (node.3 as f64),
                ),
                order::Action::Link(from, to) => Action::Link(
                    index_of(from),
                    index_of(to),
                    (from.0 as f64) / (from.1 as f64),
                    (from.2 as f64) / (from.3 as f64),
                    (to.0 as f64) / (to.1 as f64),
                    (to.2 as f64) / (to.3 as f64),
                ),
            })
            .collect();

        Network {
            length: cumulative_output_length,
            actions,
            inputs: (0..input_length).collect(),
            outputs: (cumulative_hidden_length..cumulative_output_length).collect(),
        }
    }
}
