use crate::environment::EnvironmentDescription;

pub trait Genome: Clone + Send {
    type InitConfig;
    type PopulationState: Default;

    fn new(init_config: &Self::InitConfig) -> Self;
    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, global_population_state: &mut Self::PopulationState);
    fn distance(&self, other: &Self) -> f64 {
        0.0
    }
}

pub trait Develop<G: Genome, P> {
    fn init_config(&self, decription: EnvironmentDescription) -> G::InitConfig;
    fn develop(&self, genome: &G) -> P;
}
