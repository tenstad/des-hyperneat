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
    cppn: &mut execute::Executor,
    depth: usize,
    reverse: bool,
) -> (
    Vec<Vec<(i64, i64)>>,
    connection::Connections<(i64, i64), f64>,
) {
    let mut cppn = cppn;

    let mut nodes = inputs.iter().cloned().collect::<HashSet<(i64, i64)>>();
    let mut layers: Vec<Vec<(i64, i64)>> = vec![inputs];
    let mut connections = connection::Connections::<(i64, i64), f64>::new();

    for _ in 0..depth {
        let mut new_connections = Vec::<Connection<(i64, i64), f64>>::new();
        for (x, y) in layers.last().unwrap() {
            new_connections.extend(
                find_connections(
                    *x as f64 / conf::ESHYPERNEAT.resolution,
                    *y as f64 / conf::ESHYPERNEAT.resolution,
                    &mut cppn,
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
                .filter(|target| !nodes.contains(&(target.node.0, target.node.1)))
                .map(move |target| {
                    Connection::new((*x, *y), (target.node.0, target.node.1), target.edge)
                }),
            );
        }

        if new_connections.len() == 0 {
            break;
        }

        for connection in new_connections.iter() {
            nodes.insert((connection.to.0, connection.to.1));
            if reverse {
                connections.add(
                    (connection.to.0, connection.to.1),
                    (connection.from.0, connection.from.1),
                    connection.edge,
                );
            } else {
                connections.add(
                    (connection.from.0, connection.from.1),
                    (connection.to.0, connection.to.1),
                    connection.edge,
                );
            }
        }

        layers.push(
            new_connections
                .iter()
                .map(|connection| (connection.to.0, connection.to.1))
                .collect::<HashSet<(i64, i64)>>()
                .into_iter()
                .collect::<Vec<(i64, i64)>>(),
        );
    }

    (layers, connections)
}
