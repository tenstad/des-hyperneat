use crate::genome::{GenericGenome as GenericEvolutionGenome, Genome as EvolutionGenome};
use crate::neat::{
    conf::{ConfigProvider, NeatConfig},
    genome_core::GenomeCore,
    link::LinkCore,
    node::NodeCore,
    state::{InitConfig, StateCore, StateProvider},
};

pub type DefaultNeatGenome = GenomeCore<NodeCore, LinkCore>;
impl EvolutionGenome for DefaultNeatGenome {
    type InitConfig = InitConfig;
    type State = StateCore;
    type Config = NeatConfig;
}

pub trait Genome<
    C: ConfigProvider<
        <<Self as Genome<C, S>>::Node as Node>::Config,
        <<Self as Genome<C, S>>::Link as Link>::Config,
    >,
    S: StateProvider<
        <<Self as Genome<C, S>>::Node as Node>::State,
        <<Self as Genome<C, S>>::Link as Link>::State,
    >,
>: Clone + Send
{
    type Init;
    type Node: Node;
    type Link: Link;

    fn new(config: &C, init_config: &Self::Init, state: &mut S) -> Self;
    fn crossover(&self, config: &C, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, config: &C, state: &mut S);
    fn distance(&self, config: &C, other: &Self) -> f64;
}

pub trait GetCore<T> {
    fn get_core(&self) -> &T;
    fn get_core_mut(&mut self) -> &mut T;
}

pub trait Node: GetCore<NodeCore> + Clone + Send {
    type Config;
    type State;

    fn new(config: &Self::Config, core: NodeCore, state: &mut Self::State) -> Self;
    fn crossover(
        &self,
        config: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self;
    fn distance(&self, config: &Self::Config, other: &Self) -> f64;
}

pub trait Link: GetCore<LinkCore> + Clone + Send {
    type Config;
    type State;

    fn new(config: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self;
    fn identity(config: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self;
    fn clone_with(&self, config: &Self::Config, core: LinkCore, state: &mut Self::State) -> Self;
    fn crossover(
        &self,
        config: &Self::Config,
        other: &Self,
        fitness: &f64,
        other_fitness: &f64,
    ) -> Self;
    fn distance(&self, config: &Self::Config, other: &Self) -> f64;
}

impl<
        N: Node,
        L: Link,
        C: ConfigProvider<N::Config, L::Config>,
        S: StateProvider<N::State, L::State>,
        G: Genome<C, S, Node = N, Link = L>,
    > GenericEvolutionGenome<C, S, G::Init> for G
{
    fn new(config: &C, init_config: &G::Init, state: &mut S) -> Self {
        G::new(config, init_config, state)
    }
    fn crossover(&self, config: &C, other: &Self, fitness: &f64, other_fitness: &f64) -> Self {
        G::crossover(self, config, other, fitness, other_fitness)
    }
    fn mutate(&mut self, config: &C, state: &mut S) {
        G::mutate(self, config, state)
    }
    fn distance(&self, config: &C, other: &Self) -> f64 {
        G::distance(self, config, other)
    }
}
