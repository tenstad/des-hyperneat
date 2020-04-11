use crate::conf;
use crate::network::basic_order;
use crate::network::connection;
use crate::network::order;

pub struct Network {
    pub length: usize,
    pub actions: Vec<Action>,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

pub type Point = (u64, u64);

pub enum Action {
    Activation(usize, f64, f64),            // node, x, y
    Link(usize, usize, f64, f64, f64, f64), // from, to, x0, y0, x1, y1
}

impl Network {
    pub fn layered(layer_sizes: Vec<usize>) -> Network {
        let vertical_distance =
            (conf::ESHYPERNEAT.resolution / (layer_sizes.len() as f64 - 1.0)) as u64;
        let offset = (conf::ESHYPERNEAT.resolution / 2.0) as u64;
        let layers: Vec<Vec<Point>> = layer_sizes
            .iter()
            .enumerate()
            .map(|(j, n)| {
                let horizontal_distance = (conf::ESHYPERNEAT.resolution / (*n as f64 - 1.0)) as u64;
                (0..*n)
                    .map(|i| {
                        (
                            horizontal_distance * i as u64 - offset,
                            vertical_distance * j as u64 - offset,
                        )
                    })
                    .collect()
            })
            .collect();

        let mut connections = connection::TogglableConnections::<Point, ()>::new();
        for i in 0..(layers.len() - 1) {
            for from in layers[i].iter() {
                for to in layers[i + 1].iter() {
                    connections.add_enabled(*from, *to, ());
                }
            }
        }

        Network::create(
            layers.first().unwrap().iter().cloned().collect(),
            layers
                .iter()
                .skip(1)
                .take(layer_sizes.len() - 2)
                .flatten()
                .cloned()
                .collect(),
            layers.last().unwrap().iter().cloned().collect(),
            connections,
        )
    }

    pub fn create(
        inputs: Vec<Point>,
        hiddens: Vec<Point>,
        outputs: Vec<Point>,
        connections: connection::TogglableConnections<Point, ()>,
    ) -> Network {
        let mut order = basic_order::BasicOrder::<Point>::new();
        order.sort_topologically(&connections);

        let all_nodes: Vec<Point> = inputs
            .iter()
            .chain(hiddens.iter())
            .chain(outputs.iter())
            .cloned()
            .collect();
        let index_of = |node: Point| all_nodes.iter().position(|x| node == *x).unwrap();

        let actions = order
            .iter()
            .map(|action| match action {
                order::Action::Activation(node) => Action::Activation(
                    index_of(*node),
                    node.0 as f64 / conf::ESHYPERNEAT.resolution,
                    node.1 as f64 / conf::ESHYPERNEAT.resolution,
                ),
                order::Action::Link(from, to) => Action::Link(
                    index_of(*from),
                    index_of(*to),
                    from.0 as f64 / conf::ESHYPERNEAT.resolution,
                    from.1 as f64 / conf::ESHYPERNEAT.resolution,
                    to.0 as f64 / conf::ESHYPERNEAT.resolution,
                    to.1 as f64 / conf::ESHYPERNEAT.resolution,
                ),
            })
            .collect();

        let input_length = inputs.len();
        let cumulative_hidden_length = input_length + hiddens.len();
        let cumulative_output_length = cumulative_hidden_length + outputs.len();

        Network {
            length: cumulative_output_length,
            actions,
            inputs: (0..input_length).collect(),
            outputs: (cumulative_hidden_length..cumulative_output_length).collect(),
        }
    }
}
