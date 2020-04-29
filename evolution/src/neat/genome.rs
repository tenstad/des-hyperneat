use crate::neat::{genome_core::GenomeCore, link::LinkCore, node::NodeCore};

pub type DefaultNeatGenome = GenomeCore<NodeCore, LinkCore>;

pub trait Genome: Clone + Send {
    type Init;
    type State: Default;
    type Node: GenomeComponent<NodeCore, Self::State>;
    type Link: GenomeComponent<LinkCore, Self::State>;

    fn new(init_config: &Self::Init) -> Self;
    fn get_neat(&self) -> &GenomeCore<Self::Node, Self::Link>;
    fn get_neat_mut(&mut self) -> &mut GenomeCore<Self::Node, Self::Link>;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut Self::State);
    fn distance(&self, other: &Self) -> f64;
}

pub trait GenomeComponent<T, S: Default>: Clone + Send {
    fn new(core_component: T) -> Self;
    fn get_neat(&self) -> &T;
    fn get_neat_mut(&mut self) -> &mut T;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut S);
    fn distance(&self, other: &Self) -> f64;
}

impl<
        N: GenomeComponent<NodeCore, G::State>,
        L: GenomeComponent<LinkCore, G::State>,
        G: Genome<Node = N, Link = L>,
    > crate::genome::Evolvable<G::State> for G
{
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

impl<
        N: GenomeComponent<NodeCore, G::State>,
        L: GenomeComponent<LinkCore, G::State>,
        G: Genome<Node = N, Link = L>,
    > crate::genome::Genome for G
{
    type InitConfig = G::Init;
    type PopulationState = G::State;

    fn new(init_config: &G::Init) -> Self {
        G::new(init_config)
    }
}
