use crate::develop::Develop;
use crate::environment::Environment;
use crate::genome::Genome;
use std::cmp;

#[derive(Clone)]
pub struct Organism<G> {
    pub genome: G,
    pub fitness: f64,
    pub adjusted_fitness: f64,
    pub generation: u64,
}

impl<G: Genome> Organism<G> {
    pub fn new(init_config: &G::InitConfig, population_state: &mut G::PopulationState) -> Self {
        Self {
            genome: G::new(init_config, population_state),
            fitness: 0.0,
            adjusted_fitness: 0.0,
            generation: 0,
        }
    }

    /// Breed organism with other organism
    pub fn crossover(&self, other: &Self) -> Self {
        Organism {
            genome: self
                .genome
                .crossover(&other.genome, &self.fitness, &other.fitness),
            fitness: 0.0,
            adjusted_fitness: 0.0,
            generation: self.generation + 1,
        }
    }

    /// Compare to other organism based on non-adjusted fitness
    pub fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }

    /// Fitness of organism in environment
    pub fn fitness<P, E: Environment<P>, D: Develop<G, P>>(
        &mut self,
        environment: &E,
        developer: &D,
    ) -> f64 {
        let mut phenotype = developer.develop(&self.genome);
        environment.fitness(&mut phenotype)
    }

    /// Mutate organism
    pub fn mutate(&mut self, population_state: &mut G::PopulationState) {
        self.genome.mutate(population_state);
    }

    /// Genetic distance to other organism
    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }

    /// Produce an elite for the next generation
    pub fn as_elite(&self) -> Self {
        let mut elite = self.clone();
        elite.generation += 1;
        elite
    }
}
