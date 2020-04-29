pub trait Evolvable<S: Default>: Clone + Send {
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, population_state: &mut S);
    fn distance(&self, other: &Self) -> f64 {
        0.0
    }
}

pub trait Genome: Evolvable<<Self as Genome>::PopulationState> {
    type InitConfig;
    type PopulationState: Default;

    fn new(init_config: &Self::InitConfig) -> Self;
}
