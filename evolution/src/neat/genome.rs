use crate::neat::{
    genome_core::GenomeCore,
    link::LinkCore,
    node::NodeCore,
    state::{InitConfig, NeatStateProvider, PopulationState},
};

pub type DefaultNeatGenome = GenomeCore<NodeCore, LinkCore>;
impl crate::genome::Genome for GenomeCore<NodeCore, LinkCore> {
    type InitConfig = InitConfig;
    type PopulationState = PopulationState;
}

pub trait Genome: Clone + Send {
    type Init;
    type State: NeatStateProvider;
    type Node: GenomeComponent<NodeCore>;
    type Link: GenomeComponent<LinkCore>;

    fn new(init_config: &Self::Init) -> Self;
    fn get_neat(&self) -> &GenomeCore<Self::Node, Self::Link>;
    fn get_neat_mut(&mut self) -> &mut GenomeCore<Self::Node, Self::Link>;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut Self::State);
    fn distance(&self, other: &Self) -> f64;
}

pub trait GenomeComponent<T>: Clone + Send {
    fn new(core_component: T) -> Self;
    fn get_neat(&self) -> &T;
    fn get_neat_mut(&mut self) -> &mut T;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

impl<N: GenomeComponent<NodeCore>, L: GenomeComponent<LinkCore>, G: Genome<Node = N, Link = L>>
    crate::genome::GenericGenome<G::State, G::Init> for G
{
    fn new(init_config: &G::Init) -> Self {
        G::new(init_config)
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
