use crate::environment::EnvironmentDescription;

pub trait Evolvable: Clone + Send {
    type PopulationState: Default;

    fn crossover(&self, other: &Self, fitness: &f64, other_fitness: &f64) -> Self;
    fn mutate(&mut self, population_state: &mut Self::PopulationState);
    fn distance(&self, other: &Self) -> f64 {
        0.0
    }
}

pub trait Genome: Evolvable {
    type InitConfig;

    fn new(init_config: &Self::InitConfig) -> Self;
}

pub trait Develop<G: Genome, P>: From<EnvironmentDescription> {
    fn init_config(&self) -> &G::InitConfig;
    fn develop(&self, genome: &G) -> P;
}
