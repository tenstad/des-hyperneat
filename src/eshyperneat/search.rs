use crate::conf;
use network::connection;
use network::connection::{Connection, Target};
use network::execute;
use std::collections::HashSet;

struct QuadPoint {
    x: f64,
    y: f64,
    width: f64,
    weight: f64,
    depth: u32,
    children: Option<[Box<QuadPoint>; 4]>,
}

impl QuadPoint {
    fn new(x: f64, y: f64, width: f64, depth: u32, f: &mut dyn FnMut(f64, f64) -> f64) -> Self {
        Self {
            x,
            y,
            width,
            weight: f(x, y),
            depth,
            children: None,
        }
    }

    fn children(&self) -> impl Iterator<Item = &Box<QuadPoint>> {
        self.children.iter().flatten()
    }

    fn variance(&self) -> f64 {
        let w = self.children().map(|child| child.weight).sum::<f64>() / 4.0;
        self.children()
            .map(|child| (w - child.weight).powi(2))
            .sum::<f64>()
            / 4.0
    }

    fn expand(&mut self, f: &mut dyn FnMut(f64, f64) -> f64) {
        let width = self.width / 2.0;
        let depth = self.depth + 1;

        if self.depth < conf::ESHYPERNEAT.initial_depth
            || (self.depth < conf::ESHYPERNEAT.max_depth
                && self.variance() > conf::ESHYPERNEAT.division_threshold)
        {
            let mut child =
                |x: f64, y: f64| Box::new(QuadPoint::new(self.x + x, self.y + y, width, depth, f));
            let mut children = [
                child(-width, -width),
                child(-width, width),
                child(width, width),
                child(width, -width),
            ];

            for child in children.iter_mut() {
                child.expand(f);
            }

            self.children = Some(children);
        }
    }

    fn extract(&self, f: &mut dyn FnMut(f64, f64) -> f64) -> Vec<Target<(f64, f64), f64>> {
        self.children()
            .flat_map(|child| {
                if child.variance() > conf::ESHYPERNEAT.variance_threshold {
                    child.extract(f)
                } else {
                    let d_left = (child.weight - f(child.x - self.width, child.y)).abs();
                    let d_right = (child.weight - f(child.x + self.width, child.y)).abs();
                    let d_up = (child.weight - f(child.x, child.y - self.width)).abs();
                    let d_down = (child.weight - f(child.x, child.y + self.width)).abs();

                    if b(d_up, d_down, d_left, d_right) > conf::ESHYPERNEAT.band_threshold {
                        vec![Target::new((child.x, child.y), child.weight)]
                    } else {
                        Vec::new()
                    }
                }
            })
            .collect::<Vec<Target<(f64, f64), f64>>>()
    }
}

fn b(up: f64, down: f64, left: f64, right: f64) -> f64 {
    let mi_v = if up < down { up } else { down };
    let mi_h = if left < right { left } else { right };
    if mi_v > mi_h {
        mi_v
    } else {
        mi_h
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
    let mut root = QuadPoint::new(0.0, 0.0, 1.0, 1, &mut f);
    root.expand(&mut f);
    root.extract(&mut f)
}

pub fn explore_substrate(
    inputs: Vec<(i64, i64)>,
    outputs: &Vec<(i64, i64)>,
    cppn: &mut execute::Executor,
    depth: usize,
    reverse: bool,
) -> (
    Vec<Vec<(i64, i64)>>,
    connection::Connections<(i64, i64), f64>,
) {
    let outputs = outputs.iter().cloned().collect::<HashSet<(i64, i64)>>();
    let mut visited = inputs.iter().cloned().collect::<HashSet<(i64, i64)>>();
    let mut nodes: Vec<Vec<(i64, i64)>> = vec![inputs];
    let mut connections = connection::Connections::<(i64, i64), f64>::new();

    for d in 0..depth {
        let mut discoveries = Vec::<Connection<(i64, i64), f64>>::new();
        // Search from all nodes within final depth layer
        for (x, y) in nodes[d].iter() {
            discoveries.extend(
                find_connections(
                    *x as f64 / conf::ESHYPERNEAT.resolution,
                    *y as f64 / conf::ESHYPERNEAT.resolution,
                    cppn,
                    reverse,
                )
                .iter()
                .map(|target| {
                    Target::new(
                        (
                            (target.node.0 * conf::ESHYPERNEAT.resolution) as i64,
                            (target.node.1 * conf::ESHYPERNEAT.resolution) as i64,
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
                connections.add(connection.to, connection.from, connection.edge);
            } else {
                connections.add(connection.from, connection.to, connection.edge);
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
