use crate::genome::{GenericGenome as GenericEvolutionGenome, Genome as EvolutionGenome};
use crate::neat::{
    genome_core::GenomeCore,
    link::LinkCore,
    node::NodeCore,
    state::{InitConfig, StateCore, StateProvider},
};

pub type DefaultNeatGenome = GenomeCore<NodeCore, LinkCore>;
impl EvolutionGenome for DefaultNeatGenome {
    type InitConfig = InitConfig;
    type State = StateCore;
}

pub trait GetCore<T> {
    fn get_core(&self) -> &T;
    fn get_core_mut(&mut self) -> &mut T;
}

pub trait Genome<
    S: StateProvider<
        <<Self as Genome<S>>::Node as Node>::State,
        <<Self as Genome<S>>::Link as Link>::State,
    >,
>:
    GetCore<GenomeCore<<Self as Genome<S>>::Node, <Self as Genome<S>>::Link>> + Clone + Send
{
    type Init;
    type Node: Node;
    type Link: Link;

    fn new(init_config: &Self::Init, state: &mut S) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut S);
    fn distance(&self, other: &Self) -> f64;
}

pub trait Node: GetCore<NodeCore> + Clone + Send {
    type State;

    fn new(core: NodeCore, state: &mut Self::State) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

pub trait Link: GetCore<LinkCore> + Clone + Send {
    type State;

    fn new(core: LinkCore, state: &mut Self::State) -> Self;
    fn identity(core: LinkCore, state: &mut Self::State) -> Self;
    fn clone_with(&self, core: LinkCore, state: &mut Self::State) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn distance(&self, other: &Self) -> f64;
}

impl<N: Node, L: Link, S: StateProvider<N::State, L::State>, G: Genome<S, Node = N, Link = L>>
    GenericEvolutionGenome<S, G::Init> for G
{
    fn new(init_config: &G::Init, state: &mut S) -> Self {
        G::new(init_config, state)
    }
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        G::crossover(self, other, fitness, other_fitness)
    }
    fn mutate(&mut self, state: &mut S) {
        G::mutate(self, state)
    }
    fn distance(&self, other: &Self) -> f64 {
        G::distance(self, other)
    }
}
