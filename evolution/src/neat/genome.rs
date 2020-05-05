use crate::neat::{
    genome_core::GenomeCore,
    link::{DefaultLink, LinkCore},
    node::{DefaultNode, NodeCore},
    state::{InitConfig, NeatStateProvider, StateCore},
};

pub type DefaultNeatGenome = GenomeCore<DefaultNode, DefaultLink, StateCore>;
impl crate::genome::Genome for DefaultNeatGenome {
    type InitConfig = InitConfig;
    type State = StateCore;
}

pub trait Genome: Clone + Send {
    type Init;
    type State: NeatStateProvider;
    type Node: Node<Self::State>;
    type Link: Link<Self::State>;

    fn new(init_config: &Self::Init, state: &mut Self::State) -> Self;
    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link, Self::State>;
    fn get_core_mut(&mut self) -> &mut GenomeCore<Self::Node, Self::Link, Self::State>;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut Self::State);
    fn distance(&self, other: &Self) -> f64;
}

pub trait Node<S>: Clone + Send {
    fn new(core: NodeCore, state: &mut S) -> Self;
    fn get_core(&self) -> &NodeCore;
    fn get_core_mut(&mut self) -> &mut NodeCore;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

pub trait Link<S>: Clone + Send {
    fn new(core: LinkCore, state: &mut S) -> Self;
    fn identity(core: LinkCore, state: &mut S) -> Self;
    fn clone_with(&self, core: LinkCore, state: &mut S) -> Self;
    fn get_core(&self) -> &LinkCore;
    fn get_core_mut(&mut self) -> &mut LinkCore;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

impl<N: Node<G::State>, L: Link<G::State>, G: Genome<Node = N, Link = L>>
    crate::genome::GenericGenome<G::State, G::Init> for G
{
    fn new(init_config: &G::Init, state: &mut G::State) -> Self {
        G::new(init_config, state)
    }
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        G::crossover(self, other, fitness, other_fitness)
    }
    fn mutate(&mut self, state: &mut G::State) {
        G::mutate(self, state)
    }
    fn distance(&self, other: &Self) -> f64 {
        G::distance(self, other)
    }
}
