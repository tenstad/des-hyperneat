use crate::generic_neat::node::NodeRef;

/// Link between two nodes
#[derive(Copy, Clone, Debug)]
pub struct Link {
    pub from: NodeRef,
    pub to: NodeRef,
    pub weight: f64,
    pub enabled: bool,
    pub split: bool,     // Link has been split
    pub innovation: u64, // Global innovation number
}

impl Link {
    pub fn new(from: NodeRef, to: NodeRef, weight: f64, innovation: u64) -> Link {
        Link {
            from,
            to,
            weight,
            enabled: true,
            split: false,
            innovation,
        }
    }

    pub fn crossover(&self, other: &Link) -> Link {
        assert_eq!(self.from, other.from);
        assert_eq!(self.to, other.to);
        assert_eq!(self.innovation, other.innovation);

        Link {
            from: self.from,
            to: self.to,
            weight: (self.weight + other.weight) / 2.0,
            enabled: self.enabled || other.enabled,
            split: self.split && other.split,
            innovation: self.innovation,
        }
    }

    pub fn distance(&self, other: &Link) -> f64 {
        0.5 * (self.weight - other.weight).tanh().abs()
            + 0.5 * ((self.enabled == other.enabled) as u64) as f64
    }
}
