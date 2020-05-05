use crate::eshyperneat::conf::ESHYPERNEAT;
use network::{
    connection::{Connection, Target},
    execute,
};

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

    fn children_mut(&mut self) -> impl Iterator<Item = &mut Box<QuadPoint>> {
        self.children.iter_mut().flatten()
    }

    /// Collect weight of all nodes in tree. If root is true, collect the root's weight. If
    /// internal is true, collect all internal node weights. Allways collect leaf node weights.
    fn collect_leaf_weights(&self, weights: &mut Vec<f64>, root: bool, internal: bool) {
        if (root && !ESHYPERNEAT.only_leaf_variance) || self.children.is_none() {
            weights.push(self.weight);
        }
        for child in self.children() {
            child.collect_leaf_weights(weights, internal, internal);
        }
    }

    fn calc_variance(&mut self, delta_weight: f64, root: bool, branch: bool) -> f64 {
        if delta_weight == 0.0 {
            return 0.0;
        }

        let mut weights = Vec::new();
        self.collect_leaf_weights(&mut weights, root, branch);

        let len = weights.len() as f64;
        if len == 0.0 {
            return 0.0;
        }

        let cmp = |a: &f64, b: &f64| a.partial_cmp(&b).unwrap();

        let centroid = if ESHYPERNEAT.median_variance {
            // median weight
            *order_stat::median_of_medians_by(&mut weights, cmp).1
        } else {
            // mean weight
            weights.iter().sum::<f64>() / len
        };
        let dw = if ESHYPERNEAT.relative_variance {
            delta_weight
        } else {
            1.0
        };

        let squares = weights.iter().map(|w| ((centroid - w) / dw).powi(2));

        self.variance = if ESHYPERNEAT.max_variance {
            // max square offset
            squares.max_by(cmp).unwrap()
        } else {
            // mean square offset
            squares.sum::<f64>() / len
        };

        self.variance
    }

    /// Creates the four children of a node. Returns their min and max weight value.
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

        let cmp = |a: &f64, b: &f64| a.partial_cmp(&b).unwrap();
        (
            self.children().map(|c| c.weight).min_by(cmp).unwrap(),
            self.children().map(|c| c.weight).max_by(cmp).unwrap(),
        )
    }

    /// Yields all children if this parent (self) should be expanded
    fn expand(&mut self, delta_weight: f64) -> impl Iterator<Item = &mut Box<QuadPoint>> {
        let expand = self.depth + 1 < ESHYPERNEAT.initial_resolution
            || (self.depth + 1 < ESHYPERNEAT.max_resolution
                && self.calc_variance(delta_weight, true, true) > ESHYPERNEAT.division_threshold);

        self.children_mut().take(4 * (expand as usize))
    }

    /// Yields children with high variance. Pushes children with low
    /// variance to connections, if their band value is above band threshold.
    fn extract(
        &mut self,
        f: &mut dyn FnMut(f64, f64) -> f64,
        connections: &mut Vec<Target<(f64, f64), f64>>,
        delta_weight: f64,
    ) -> impl Iterator<Item = &mut Box<QuadPoint>> {
        let width = self.width;

        for child in self.children_mut() {
            if child.calc_variance(delta_weight, false, true) <= ESHYPERNEAT.variance_threshold {
                let d_left = (child.weight - f(child.x - width, child.y)).abs();
                let d_right = (child.weight - f(child.x + width, child.y)).abs();
                let d_up = (child.weight - f(child.x, child.y - width)).abs();
                let d_down = (child.weight - f(child.x, child.y + width)).abs();
                let band_value = d_up.min(d_down).max(d_left.min(d_right));

                if band_value >= ESHYPERNEAT.band_threshold {
                    connections.push(Target::new((child.x, child.y), child.weight));
                }
            }
        }

        // Use stored variance calculated in previous loop
        self.children_mut()
            .filter(|child| child.variance > ESHYPERNEAT.variance_threshold)
    }
}

/// Single iteration search for new nodes and connections from a given point.
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

    // If all weight values are the same, no nodes will be collected.
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
    // If the collection was limited by max_discoveris, nodes at the current depth in the tree
    // are included, since either they or their children would be if the seach continues.
    for leaf in leaves.iter() {
        connections.push(Target::new((leaf.x, leaf.y), leaf.weight))
    }

    // Only return the weights with the highest absolute value.
    if ESHYPERNEAT.max_outgoing > 0 && connections.len() > ESHYPERNEAT.max_outgoing {
        connections.sort_by(|a, b| b.edge.abs().partial_cmp(&a.edge.abs()).unwrap());
        connections.truncate(ESHYPERNEAT.max_outgoing);
    }

    connections
}

/// Iteratively explore substrate by calling find_connections on discovered nodes
pub fn explore_substrate(
    inputs: Vec<(i64, i64)>,
    outputs: &Vec<(i64, i64)>,
    cppn: &mut execute::Executor,
    depth: usize,
    reverse: bool,
    allow_connections_to_input: bool,
) -> (Vec<Vec<(i64, i64)>>, Vec<Connection<(i64, i64), f64>>) {
    let outputs = outputs.iter().cloned().collect::<HashSet<(i64, i64)>>();
    let mut visited = if allow_connections_to_input {
        HashSet::<(i64, i64)>::new()
    } else {
        inputs.iter().cloned().collect::<HashSet<(i64, i64)>>()
    };
    let mut nodes: Vec<Vec<(i64, i64)>> = vec![inputs];
    let mut connections = Vec::<Connection<(i64, i64), f64>>::new();

    for d in 0..depth {
        let mut discoveries = Vec::<Connection<(i64, i64), f64>>::new();
        // Search from all nodes within previous layer of discoveries
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

        // Store all new connections in correct direction
        connections.extend(discoveries.iter().map(|connection| {
            if reverse {
                Connection::new(connection.to, connection.from, connection.edge)
            } else {
                Connection::new(connection.from, connection.to, connection.edge)
            }
        }));

        // Collect all unique target nodes
        // Avoid furhter exploration from output nodes
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
