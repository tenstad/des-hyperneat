use crate::conf;
use crate::network::connection;
use std::collections::HashMap;

pub struct Network {
    pub length: usize,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub actions: Vec<Action>,
}

pub type Point = (i64, i64);

pub enum Action {
    Activation(usize, f64, f64),            // node, x, y
    Link(usize, usize, f64, f64, f64, f64), // from, to, x0, y0, x1, y1
}

impl Network {
    pub fn layered(layer_sizes: Vec<usize>) -> Network {
        let vertical_distance =
            (2.0 * conf::ESHYPERNEAT.resolution / (layer_sizes.len() as f64 - 1.0)) as i64;
        let layers: Vec<Vec<Point>> = layer_sizes
            .iter()
            .enumerate()
            .map(|(j, n)| {
                let horizontal_distance =
                    (2.0 * conf::ESHYPERNEAT.resolution / (*n as f64 - 1.0)) as i64;
                (0..*n)
                    .map(|i| {
                        (
                            horizontal_distance * i as i64 - conf::ESHYPERNEAT.resolution as i64,
                            vertical_distance * j as i64 - conf::ESHYPERNEAT.resolution as i64,
                        )
                    })
                    .collect()
            })
            .collect();

        let mut connections = connection::Connections::<Point, ()>::new();
        for i in 0..(layers.len() - 1) {
            for from in layers[i].iter() {
                for to in layers[i + 1].iter() {
                    connections.add(*from, *to, ());
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
        connections: connection::Connections<Point, ()>,
    ) -> Network {
        // Create mapping from Point to array index in Network's node vector
        let node_mapping: HashMap<Point, usize> = inputs
            .iter()
            .chain(hiddens.iter())
            .chain(outputs.iter())
            .enumerate()
            .map(|(i, point)| (*point, i))
            .collect();

        let actions = connections
            .sort_topologically()
            .iter()
            .map(|action| match action {
                connection::OrderedAction::Activation(node) => Action::Activation(
                    *node_mapping.get(node).unwrap(),
                    node.0 as f64 / conf::ESHYPERNEAT.resolution,
                    node.1 as f64 / conf::ESHYPERNEAT.resolution,
                ),
                connection::OrderedAction::Link(from, to, _) => Action::Link(
                    *node_mapping.get(from).unwrap(),
                    *node_mapping.get(to).unwrap(),
                    from.0 as f64 / conf::ESHYPERNEAT.resolution,
                    from.1 as f64 / conf::ESHYPERNEAT.resolution,
                    to.0 as f64 / conf::ESHYPERNEAT.resolution,
                    to.1 as f64 / conf::ESHYPERNEAT.resolution,
                ),
            })
            .collect();
        let non_output_count = inputs.len() + hiddens.len();

        Network {
            length: node_mapping.len(),
            inputs: (0..inputs.len()).collect(),
            outputs: (non_output_count..(non_output_count + outputs.len())).collect(),
            actions,
        }
    }
}
