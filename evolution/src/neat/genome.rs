use crate::genome::Evolvable;
use crate::neat::{
    genome_core::{GenomeCore, InitConfig},
    link::LinkCore,
    node::NodeCore,
    state::PopulationState,
};

pub type DefaultNeatGenome = GenomeCore<NodeCore, LinkCore>;

pub trait Genome:
    NeatCore<GenomeCore<<Self as Genome>::Node, <Self as Genome>::Link>> + Clone + Send
{
    type Node: GenomeComponent<NodeCore>;
    type Link: GenomeComponent<LinkCore>;

    fn new(init_config: &InitConfig) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
}

pub trait GenomeComponent<T: Evolvable<PopulationState = PopulationState>>:
    NeatCore<T> + Clone + Send
{
    fn new(core_component: T) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        Self::new(
            self.get_neat()
                .crossover(other.get_neat(), fitness, other_fitness),
        )
    }
}

pub trait NeatCore<T: Evolvable> {
    fn get_neat(&self) -> &T;
    fn get_neat_mut(&mut self) -> &mut T;

    fn mutate(&mut self, state: &mut T::PopulationState) {
        self.get_neat_mut().mutate(state)
    }
    fn distance(&self, other: &Self) -> f64 {
        self.get_neat().distance(other.get_neat())
    }
}

impl<N: GenomeComponent<NodeCore>, L: GenomeComponent<LinkCore>, G: Genome<Node = N, Link = L>>
    crate::genome::Evolvable for G
{
    type PopulationState = PopulationState;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        G::crossover(self, other, fitness, other_fitness)
    }
    fn mutate(&mut self, state: &mut PopulationState) {
        G::mutate(self, state)
    }
    fn distance(&self, other: &Self) -> f64 {
        G::distance(self, other)
    }
}

impl<N: GenomeComponent<NodeCore>, L: GenomeComponent<LinkCore>, G: Genome<Node = N, Link = L>>
    crate::genome::Genome for G
{
    type InitConfig = InitConfig;

    fn new(init_config: &InitConfig) -> Self {
        G::new(init_config)
    }
}
