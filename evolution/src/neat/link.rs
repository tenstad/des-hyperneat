use crate::neat::{
    genome::{GetCore, Link},
    node::NodeRef,
};

/// Link between two nodes
#[derive(Clone, Debug, GetCore)]
#[core]
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

    pub fn crossover(&self, other: &Self, _fitness: &f64, _other_fitness: &f64) -> Self {
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

    pub fn distance(&self, other: &Self) -> f64 {
        0.5 * (self.weight - other.weight).abs().tanh()
            + 0.5 * ((self.enabled != other.enabled) as u8) as f64
    }
}

impl Link for LinkCore {
    type Config = ();
    type State = ();

    fn new(_: &Self::Config, core: LinkCore, _: &mut Self::State) -> Self {
        core
    }

    fn identity(_: &Self::Config, core: LinkCore, _: &mut Self::State) -> Self {
        core
    }

    fn clone_with(&self, _: &Self::Config, core: LinkCore, _: &mut Self::State) -> Self {
        core
    }

    fn crossover(
        &self,
        _: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self {
        self.crossover(&other, fitness, other_fitness)
    }

    fn distance(&self, _: &Self::Config, other: &Self) -> f64 {
        self.distance(&other)
    }
}
