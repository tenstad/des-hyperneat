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
    type Node: GenomeComponent<NodeCore, Self::State>;
    type Link: GenomeComponent<LinkCore, Self::State>;

    fn new(init_config: &Self::Init, state: &mut Self::State) -> Self;
    fn get_core(&self) -> &GenomeCore<Self::Node, Self::Link, Self::State>;
    fn get_core_mut(&mut self) -> &mut GenomeCore<Self::Node, Self::Link, Self::State>;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut Self::State);
    fn distance(&self, other: &Self) -> f64;
}

pub trait GenomeComponent<T, S>: Clone + Send {
    fn new(core_component: T, state: &mut S) -> Self;
    fn get_core(&self) -> &T;
    fn get_core_mut(&mut self) -> &mut T;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

impl<
        N: GenomeComponent<NodeCore, G::State>,
        L: GenomeComponent<LinkCore, G::State>,
        G: Genome<Node = N, Link = L>,
    > crate::genome::GenericGenome<G::State, G::Init> for G
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
