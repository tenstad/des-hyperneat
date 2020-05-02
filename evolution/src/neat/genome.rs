use crate::neat::{
    genome_core::GenomeCore,
    link::LinkCore,
    node::NodeCore,
    state::{InitConfig, NeatStateProvider, PopulationState},
};

pub type DefaultNeatGenome = GenomeCore<NodeCore, LinkCore, PopulationState>;
impl crate::genome::Genome for GenomeCore<NodeCore, LinkCore, PopulationState> {
    type InitConfig = InitConfig;
    type PopulationState = PopulationState;
}

pub trait Genome: Clone + Send {
    type Init;
    type State: NeatStateProvider;
    type Node: GenomeComponent<NodeCore, Self::State>;
    type Link: GenomeComponent<LinkCore, Self::State>;

    fn new(init_config: &Self::Init, state: &mut Self::State) -> Self;
    fn get_neat(&self) -> &GenomeCore<Self::Node, Self::Link, Self::State>;
    fn get_neat_mut(&mut self) -> &mut GenomeCore<Self::Node, Self::Link, Self::State>;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut Self::State);
    fn distance(&self, other: &Self) -> f64;
}

pub trait GenomeComponent<T, S>: Clone + Send {
    fn new(core_component: T, state: &mut S) -> Self;
    fn get_neat(&self) -> &T;
    fn get_neat_mut(&mut self) -> &mut T;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

impl<
        N: GenomeComponent<NodeCore, G::State>,
        L: GenomeComponent<LinkCore, G::State>,
        G: Genome<Node = N, Link = L>,
    > crate::genome::GenericGenome<G::State, G::Init> for G
{
    fn new(init_config: &G::Init, population_state: &mut G::State) -> Self {
        G::new(init_config, population_state)
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
