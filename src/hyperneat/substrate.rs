use crate::eshyperneat::conf::ESHYPERNEAT;
use crate::eshyperneat::figure::save_fig_to_file;
use crate::hyperneat::conf::HYPERNEAT;
use network::connection;
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

#[allow(dead_code)]
impl Network {
    pub fn load(inputs: u64, outputs: u64) -> Self {
        let input_layer = parse_nodes(
            &HYPERNEAT.input_config,
            ESHYPERNEAT.resolution,
            inputs,
            -1.0,
        );
        let hidden_layers = parse_hidden_nodes(
            &HYPERNEAT.hidden_layers,
            &HYPERNEAT.hidden_layer_sizes,
            ESHYPERNEAT.resolution,
        );
        let output_layer = parse_nodes(
            &HYPERNEAT.output_config,
            ESHYPERNEAT.resolution,
            outputs,
            1.0,
        );

        Self::layered_from_layers(input_layer, hidden_layers, output_layer)
    }

    pub fn layered(layer_sizes: Vec<u64>) -> Network {
        let layers = horizontal_rows(&layer_sizes);

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

    pub fn layered_from_layers(
        inputs: Vec<(i64, i64)>,
        hidden_layers: Vec<Vec<(i64, i64)>>,
        outputs: Vec<(i64, i64)>,
    ) -> Network {
        let mut layers = hidden_layers.clone();
        layers.insert(0, inputs.clone());
        layers.push(outputs.clone());

        let mut connections = connection::Connections::<Point, ()>::new();
        for i in 0..(layers.len() - 1) {
            for from in layers[i].iter() {
                for to in layers[i + 1].iter() {
                    connections.add(*from, *to, ());
                }
            }
        }

        let mut hidden_layers = hidden_layers;

        Network::create(
            inputs,
            hidden_layers.drain(..).flatten().collect(),
            outputs,
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

        if HYPERNEAT.log_visualizations {
            save_fig_to_file(
                connection::Connections::<(i64, i64), f64>::from(
                    connections
                        .get_all_connections()
                        .drain(..)
                        .map(|con| connection::Connection::new(con.from, con.to, 1.0))
                        .collect::<Vec<_>>(),
                ),
                "g.tex",
                0.5 / ESHYPERNEAT.resolution,
                4.0,
            );
        }

        let actions = connections
            .sort_topologically()
            .iter()
            .map(|action| match action {
                connection::OrderedAction::Node(node) => Action::Activation(
                    *node_mapping.get(node).unwrap(),
                    node.0 as f64 / ESHYPERNEAT.resolution,
                    node.1 as f64 / ESHYPERNEAT.resolution,
                ),
                connection::OrderedAction::Edge(from, to, _) => Action::Link(
                    *node_mapping.get(from).unwrap(),
                    *node_mapping.get(to).unwrap(),
                    from.0 as f64 / ESHYPERNEAT.resolution,
                    from.1 as f64 / ESHYPERNEAT.resolution,
                    to.0 as f64 / ESHYPERNEAT.resolution,
                    to.1 as f64 / ESHYPERNEAT.resolution,
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

pub fn horizontal_row(n: u64, y: i64) -> Vec<(i64, i64)> {
    if n == 1 {
        return vec![(0, y)];
    }

    let horizontal_distance = if n > 1 {
        (2.0 * ESHYPERNEAT.resolution / (n as f64 - 1.0)) as f64
    } else {
        0.0
    };
    let offset = if n > 1 {
        ESHYPERNEAT.resolution as i64
    } else {
        0
    };
    (0..n)
        .map(|i| ((horizontal_distance * i as f64) as i64 - offset, y))
        .collect()
}

pub fn horizontal_rows(layer_sizes: &Vec<u64>) -> Vec<Vec<Point>> {
    let vertical_distance = if layer_sizes.len() > 1 {
        (2.0 * ESHYPERNEAT.resolution / (layer_sizes.len() as f64 - 1.0)) as f64
    } else {
        0.0
    };
    let offset = if layer_sizes.len() > 1 {
        ESHYPERNEAT.resolution as i64
    } else {
        0
    };
    layer_sizes
        .iter()
        .enumerate()
        .map(|(j, n)| horizontal_row(*n, (vertical_distance * j as f64) as i64 - offset))
        .collect()
}

pub fn parse_nodes(conf: &String, r: f64, num: u64, y: f64) -> Vec<(i64, i64)> {
    match &conf[..] {
        "line" => horizontal_row(num, (r * y) as i64),
        _ => serde_json::from_str::<Vec<Vec<f64>>>(conf)
            .expect("unable to parse nodes")
            .iter()
            .map(|point| ((point[0] * r) as i64, (point[1] * r) as i64))
            .collect(),
    }
}

pub fn parse_hidden_nodes(
    hidden_layers: &String,
    hidden_layer_sizes: &String,
    r: f64,
) -> Vec<Vec<(i64, i64)>> {
    if hidden_layers != "" {
        serde_json::from_str::<Vec<Vec<Vec<f64>>>>(&hidden_layers)
            .expect("unable to parse nodes")
            .iter()
            .map(|points| {
                points
                    .iter()
                    .map(|point| ((point[0] * r) as i64, (point[1] * r) as i64))
                    .collect()
            })
            .collect()
    } else {
        let mut layer_sizes =
            serde_json::from_str::<Vec<u64>>(&hidden_layer_sizes).expect("unable to parse nodes");

        // Insert dummy I/O layers to space hidden layers veritcally
        layer_sizes.insert(0, 0);
        layer_sizes.push(0);

        horizontal_rows(&layer_sizes)
            .drain(..)
            .skip(1)
            .take(layer_sizes.len() - 2)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nodes() {
        let r = ESHYPERNEAT.resolution;
        let ri = ESHYPERNEAT.resolution as i64;

        let conf = "line".to_owned();
        let nodes = parse_nodes(&conf, r, 3, -1.0);
        assert_eq!(nodes, [(-ri, -ri), (0, -ri), (ri, -ri)]);

        let nodes = parse_nodes(&conf, r, 2, 1.0);
        assert_eq!(nodes, [(-ri, ri), (ri, ri)]);

        let nodes = parse_nodes(&conf, r, 1, 0.0);
        assert_eq!(nodes, [(0, 0)]);

        let conf = "[[-1, -1], [1, -0.5]]".to_owned();
        let nodes = parse_nodes(&conf, r, 1, 0.0);
        assert_eq!(nodes, [(-ri, -ri), (ri, -ri / 2)]);
    }

    #[test]
    fn test_parse_hidden_nodes() {
        let r = ESHYPERNEAT.resolution;
        let ri = ESHYPERNEAT.resolution as i64;

        let conf_layers = "".to_owned();
        let conf_sizes = "[2, 3, 2]".to_owned();
        let nodes = parse_hidden_nodes(&conf_layers, &conf_sizes, r);
        assert_eq!(
            nodes,
            vec![
                vec![(-ri, -ri / 2), (ri, -ri / 2)],
                vec![(-ri, 0), (0, 0), (ri, 0)],
                vec![(-ri, ri / 2), (ri, ri / 2)]
            ]
        );
    }
}
