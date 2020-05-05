use crate::eshyperneat::conf::ESHYPERNEAT;
use network::connection;
use network::connection::{Connection, Target};
use network::execute;
use std::collections::HashSet;

struct QuadPoint {
    x: f64,
    y: f64,
    width: f64,
    weight: f64,
    depth: usize,
    variance: f64,
    children: Option<[Box<QuadPoint>; 4]>,
}

impl QuadPoint {
    fn new(x: f64, y: f64, width: f64, depth: usize, f: &mut dyn FnMut(f64, f64) -> f64) -> Self {
        Self {
            x,
            y,
            width,
            weight: f(x, y),
            depth,
            variance: 0.0,
            children: None,
        }
    }

    fn children(&self) -> impl Iterator<Item = &Box<QuadPoint>> {
        self.children.iter().flatten()
    }

    fn leaf_weights(&self, weights: &mut Vec<f64>, root: bool, branch: bool) {
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.leaf_weights(weights, true, branch);
            }
        }
        if branch && root {
            weights.push(self.weight);
        }
    }

    fn children_mut(&mut self) -> impl Iterator<Item = &mut Box<QuadPoint>> {
        self.children.iter_mut().flatten()
    }

    fn calc_variance(&mut self, average: bool, delta_weight: f64, root: bool, branch: bool) -> f64 {
        if delta_weight == 0.0 {
            return 0.0;
        };

        let mut weights = Vec::new();
        self.leaf_weights(&mut weights, root, branch);
        if weights.len() == 0 {
            return 0.0;
        }

        //let avg_w = weights.iter().sum::<f64>() / weights.len() as f64;
        let median_w =
            *order_stat::median_of_medians_by(&mut weights, |x, y| x.partial_cmp(y).unwrap()).1;

        let squares = weights
            .iter()
            .map(|w| ((median_w - w) / delta_weight).powi(2));
        self.variance = if average {
            squares.sum::<f64>() / weights.len() as f64
        } else {
            squares.max_by(|a, b| a.partial_cmp(&b).unwrap()).unwrap()
        };
        self.variance
    }

    fn create_children(&mut self, f: &mut dyn FnMut(f64, f64) -> f64) -> (f64, f64) {
        let width = self.width / 2.0;
        let depth = self.depth + 1;

        let mut child =
            |x: f64, y: f64| Box::new(QuadPoint::new(self.x + x, self.y + y, width, depth, f));

        self.children = Some([
            child(-width, -width),
            child(-width, width),
            child(width, width),
            child(width, -width),
        ]);

        (
            self.children()
                .map(|c| c.weight)
                .min_by(|a, b| a.partial_cmp(&b).unwrap())
                .unwrap(),
            self.children()
                .map(|c| c.weight)
                .max_by(|a, b| a.partial_cmp(&b).unwrap())
                .unwrap(),
        )
    }

    fn expand(&mut self, delta_weight: f64) -> impl Iterator<Item = &mut Box<QuadPoint>> {
        if self.depth + 1 < ESHYPERNEAT.initial_resolution
            || (self.depth + 1 < ESHYPERNEAT.max_resolution
                && self.calc_variance(false, delta_weight, true, true)
                    > ESHYPERNEAT.division_threshold)
        {
            self.children_mut().take(4)
        } else {
            self.children_mut().take(0)
        }
    }

    fn extract(
        &mut self,
        f: &mut dyn FnMut(f64, f64) -> f64,
        connections: &mut Vec<Target<(f64, f64), f64>>,
        delta_weight: f64,
    ) -> impl Iterator<Item = &mut Box<QuadPoint>> {
        let width = self.width;

        for child in self.children_mut() {
            if child.calc_variance(false, delta_weight, false, true)
                <= ESHYPERNEAT.variance_threshold
            {
                let d_left = (child.weight - f(child.x - width, child.y)).abs();
                let d_right = (child.weight - f(child.x + width, child.y)).abs();
                let d_up = (child.weight - f(child.x, child.y - width)).abs();
                let d_down = (child.weight - f(child.x, child.y + width)).abs();

                if d_up.min(d_down).max(d_left.min(d_right)) >= ESHYPERNEAT.band_threshold {
                    connections.push(Target::new((child.x, child.y), child.weight));
                }
            }
        }

        self.children_mut().filter_map(|child| {
            if child.variance > ESHYPERNEAT.variance_threshold {
                Some(child)
            } else {
                None
            }
        })
    }
}

pub fn find_connections(
    x: f64,
    y: f64,
    cppn: &mut execute::Executor,
    reverse: bool,
) -> Vec<Target<(f64, f64), f64>> {
    let mut f = |x2, y2| {
        cppn.execute(
            &(if reverse {
                vec![x2, y2, x, y]
            } else {
                vec![x, y, x2, y2]
            }),
        )[0]
    };

    let mut connections = Vec::<Target<(f64, f64), f64>>::new();
    let mut root = Box::new(QuadPoint::new(0.0, 0.0, 1.0, 1, &mut f));
    let mut min_weight = root.weight;
    let mut max_weight = root.weight;

    let mut leaves = vec![&mut root];
    while leaves.len() > 0 {
        for leaf in leaves.iter_mut() {
            let (mi, ma) = leaf.create_children(&mut f);
            min_weight = min_weight.min(mi);
            max_weight = max_weight.max(ma);
        }

        leaves = leaves
            .drain(..)
            .flat_map(|leaf| leaf.expand(max_weight - min_weight))
            .collect();
    }
    if min_weight == max_weight {
        return connections;
    }

    let mut leaves = vec![&mut root];
    while leaves.len() > 0
        && (ESHYPERNEAT.max_discoveries == 0 || connections.len() < ESHYPERNEAT.max_discoveries)
    {
        leaves = leaves
            .drain(..)
            .flat_map(|leaf| leaf.extract(&mut f, &mut connections, max_weight - min_weight))
            .collect();
    }
    for leaf in leaves.iter() {
        connections.push(Target::new((leaf.x, leaf.y), leaf.weight))
    }

    if ESHYPERNEAT.max_outgoing > 0 && connections.len() > ESHYPERNEAT.max_outgoing {
        connections.sort_by(|a, b| b.edge.abs().partial_cmp(&a.edge.abs()).unwrap());
        connections.truncate(ESHYPERNEAT.max_outgoing);
    }
    connections
}

pub fn explore_substrate(
    inputs: Vec<(i64, i64)>,
    outputs: &Vec<(i64, i64)>,
    cppn: &mut execute::Executor,
    depth: usize,
    reverse: bool,
    crossing_substrate: bool,
) -> (Vec<Vec<(i64, i64)>>, Vec<Connection<(i64, i64), f64>>) {
    let outputs = outputs.iter().cloned().collect::<HashSet<(i64, i64)>>();
    let mut visited = if !crossing_substrate {
        inputs.iter().cloned().collect::<HashSet<(i64, i64)>>()
    } else {
        // Have not visited the input nodes because we are locating nodes within a new substrates
        HashSet::<(i64, i64)>::new()
    };
    let mut nodes: Vec<Vec<(i64, i64)>> = vec![inputs];
    let mut connections = Vec::<Connection<(i64, i64), f64>>::new();

    for d in 0..depth {
        let mut discoveries = Vec::<Connection<(i64, i64), f64>>::new();
        // Search from all nodes within final depth layer
        for (x, y) in nodes[d].iter() {
            discoveries.extend(
                find_connections(
                    *x as f64 / ESHYPERNEAT.resolution,
                    *y as f64 / ESHYPERNEAT.resolution,
                    cppn,
                    reverse,
                )
                .iter()
                .map(|target| {
                    Target::new(
                        (
                            (target.node.0 * ESHYPERNEAT.resolution) as i64,
                            (target.node.1 * ESHYPERNEAT.resolution) as i64,
                        ),
                        target.edge,
                    )
                })
                .filter(|target| !visited.contains(&target.node))
                .map(|target| Connection::new((*x, *y), target.node, target.edge)),
            );
        }

        // Store all new connections
        for connection in discoveries.iter() {
            if reverse {
                connections.push(Connection::new(
                    connection.to,
                    connection.from,
                    connection.edge,
                ));
            } else {
                connections.push(Connection::new(
                    connection.from,
                    connection.to,
                    connection.edge,
                ));
            }
        }

        // Collect all unique target nodes
        // Avoid furhter exploration from potential output nodes
        let next_nodes = discoveries
            .iter()
            .map(|connection| connection.to)
            .filter(|n| !outputs.contains(n))
            .collect::<HashSet<(i64, i64)>>()
            .into_iter()
            .collect::<Vec<(i64, i64)>>();

        // Stop search if there are no more nodes to search
        if next_nodes.len() == 0 {
            break;
        }

        visited.extend(next_nodes.iter());
        nodes.push(next_nodes);
    }

    (nodes, connections)
}
