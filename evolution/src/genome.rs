pub trait GenericGenome<C, S: Default, I>: Clone + Send {
    fn new(config: &C, init_config: &I, state: &mut S) -> Self;
    fn crossover(&self, config: &C, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, config: &C, state: &mut S);
    fn distance(&self, _config: &C, _other: &Self) -> f64 {
        0.0
    }
}

pub trait Genome:
    GenericGenome<<Self as Genome>::Config, <Self as Genome>::State, <Self as Genome>::InitConfig>
{
    type Config: Clone;
    type InitConfig;
    type State: Default;
}
