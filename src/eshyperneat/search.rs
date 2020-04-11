use crate::conf;
use crate::network::execute;

#[derive(Debug)]
pub struct Connection {
    pub x: f64,
    pub y: f64,
    pub w: f64,
}

impl Connection {
    fn new(x: f64, y: f64, w: f64) -> Self {
        Connection { x, y, w }
    }
}

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
        QuadPoint {
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
                && self.variance() > conf::ESHYPERNEAT.diversity_threshold)
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

    fn extract(&self, f: &mut dyn FnMut(f64, f64) -> f64) -> Vec<Connection> {
        self.children()
            .flat_map(|child| {
                if child.variance() > conf::ESHYPERNEAT.variance_threshold {
                    child.extract(f)
                } else {
                    let d_left = (child.weight - f(child.x - self.width, child.y)).abs();
                    let d_right = (child.weight - f(child.x + self.width, child.y)).abs();
                    let d_up = (child.weight - f(child.x, child.y - self.width)).abs();
                    let d_down = (child.weight - f(child.x, child.y + self.width)).abs();

                    if b(d_up, d_down, d_left, d_right) < conf::ESHYPERNEAT.band_threshold {
                        vec![Connection::new(child.x, child.y, child.weight)]
                    } else {
                        Vec::new()
                    }
                }
            })
            .collect::<Vec<Connection>>()
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

pub fn find_connections(x: f64, y: f64, cppn: &mut execute::Executor) -> Vec<Connection> {
    let mut f = |x2, y2| cppn.execute(&vec![x, y, x2, y2])[0];
    let mut root = QuadPoint::new(0.0, 0.0, 1.0, 1, &mut f);
    root.expand(&mut f);
    root.extract(&mut f)
}
