pub trait GenericGenome<S: Default, I>: Clone + Send {
    fn new(init_config: &I, state: &mut S) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, state: &mut S);
    fn distance(&self, _other: &Self) -> f64 {
        0.0
    }
}

pub trait Genome: GenericGenome<<Self as Genome>::State, <Self as Genome>::InitConfig> {
    type InitConfig;
    type State: Default;
}
