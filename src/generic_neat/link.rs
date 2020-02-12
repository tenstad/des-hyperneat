use crate::generic_neat::node::NodeRef;

/// Link between two nodes
#[derive(Copy, Clone, Debug)]
pub struct Link<T> {
    pub from: NodeRef,
    pub to: NodeRef,
    pub weight: f64,
    pub enabled: bool,
    pub split: bool,     // Link has been split
    pub innovation: u64, // Global innovation number
    pub custom: T,
}

pub trait Custom: Copy + Clone {
    fn new() -> Self;
    fn crossover(&self, other: &Self) -> Self;
}

impl<T: Custom> Link<T> {
    pub fn new(from: NodeRef, to: NodeRef, weight: f64, innovation: u64) -> Link<T> {
        Link {
            from,
            to,
            weight,
            enabled: true,
            split: false,
            innovation,
            custom: Custom::new(),
        }
    }

    pub fn crossover(&self, other: &Link<T>) -> Link<T> {
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
            custom: self.custom.crossover(&other.custom),
        }
    }

    pub fn distance(&self, other: &Link<T>) -> f64 {
        0.5 * (self.weight - other.weight).tanh().abs()
            + 0.5 * ((self.enabled == other.enabled) as u64) as f64
    }
}
