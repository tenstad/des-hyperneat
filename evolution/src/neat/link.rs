use crate::neat::{genome::GetNeat, node::NodeRef};

pub trait LinkExtension: GetNeat<NeatLink> + Clone + Send {
    type Config;
    type State;

    fn new(config: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self;
    fn identity(config: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self;
    fn clone_with(&self, config: &Self::Config, neat: NeatLink, state: &mut Self::State) -> Self;
    fn crossover(
        &self,
        config: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self;
    fn distance(&self, config: &Self::Config, other: &Self) -> f64;
}

/// Link between two nodes
#[derive(Clone, Debug, GetNeat)]
#[neat]
pub struct NeatLink {
    pub from: NodeRef,
    pub to: NodeRef,
    pub weight: f64,
    pub enabled: bool,
    pub split: bool,       // Link has been split
    pub innovation: u64,   // Global innovation number
}

impl NeatLink {
    pub fn new(from: NodeRef, to: NodeRef, weight: f64, innovation: u64) -> Self {
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

impl LinkExtension for NeatLink {
    type Config = ();
    type State = ();

    fn new(_: &Self::Config, neat: NeatLink, _: &mut Self::State) -> Self {
        neat
    }

    fn identity(_: &Self::Config, neat: NeatLink, _: &mut Self::State) -> Self {
        neat
    }

    fn clone_with(&self, _: &Self::Config, neat: NeatLink, _: &mut Self::State) -> Self {
        neat
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
