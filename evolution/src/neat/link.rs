use crate::neat::{genome::GenomeComponent, node::NodeRef, state::PopulationState};
/// Link between two nodes
#[derive(Copy, Clone, Debug)]
pub struct LinkCore {
    pub from: NodeRef,
    pub to: NodeRef,
    pub weight: f64,
    pub enabled: bool,
    pub split: bool,       // Link has been split
    pub innovation: usize, // Global innovation number
}

impl LinkCore {
    pub fn new(from: NodeRef, to: NodeRef, weight: f64, innovation: usize) -> Self {
        Self {
            from,
            to,
            weight,
            enabled: true,
            split: false,
            innovation,
        }
    }
}

impl GenomeComponent<LinkCore, PopulationState> for LinkCore {
    fn new(link: Self) -> Self {
        link
    }

    fn get_neat(&self) -> &Self {
        self
    }

    fn get_neat_mut(&mut self) -> &mut Self {
        self
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        assert_eq!(self.from, other.from);
        assert_eq!(self.to, other.to);
        assert_eq!(self.innovation, other.innovation);
        Self {
            from: self.from,
            to: self.to,
            weight: (self.weight + other.weight) / 2.0,
            enabled: self.enabled || other.enabled,
            split: self.split && other.split,
            innovation: self.innovation,
        }
    }

    fn mutate(&mut self, population_state: &mut PopulationState) {}

    fn distance(&self, other: &Self) -> f64 {
        0.5 * (self.weight - other.weight).tanh().abs()
            + 0.5 * ((self.enabled == other.enabled) as u64) as f64
    }
}
