use crate::Stats;
use serde::Serialize;

pub trait GenericGenome<C, S: Default, I, T>: Clone + Send {
    fn new(config: &C, init_config: &I, state: &mut S) -> Self;
    fn crossover(&self, config: &C, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, config: &C, state: &mut S);
    fn distance(&self, _config: &C, _other: &Self) -> f64 {
        0.0
    }
    fn get_stats(&self) -> T;
}

pub trait Genome:
    GenericGenome<
    <Self as Genome>::Config,
    <Self as Genome>::State,
    <Self as Genome>::InitConfig,
    <Self as Genome>::Stats,
>
{
    type Config: Clone + Serialize;
    type InitConfig;
    type State: Default;
    type Stats: Stats;
}
