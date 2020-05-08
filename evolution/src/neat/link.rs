use crate::neat::{genome::Link, node::NodeRef, state::StateCore};

/// Link between two nodes
#[derive(Clone, Debug)]
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

#[derive(Clone, new)]
pub struct DefaultLink {
    pub core: LinkCore,
}

impl Link for DefaultLink {
    type State = StateCore;

    fn new(core: LinkCore, _: &mut Self::State) -> Self {
        Self { core }
    }

    fn identity(core: LinkCore, _: &mut Self::State) -> Self {
        Self { core }
    }

    fn clone_with(&self, core: LinkCore, _: &mut Self::State) -> Self {
        Self { core }
    }

    fn get_core(&self) -> &LinkCore {
        &self.core
    }

    fn get_core_mut(&mut self) -> &mut LinkCore {
        &mut self.core
    }

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self {
            core: self.core.crossover(&other.core, fitness, other_fitness),
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        self.core.distance(&other.core)
    }
}
