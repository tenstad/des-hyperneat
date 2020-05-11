use crate::genome::Genome;
use std::cmp;

#[derive(Clone)]
pub struct Organism<G, S> {
    pub genome: G,
    pub fitness: Option<f64>,
    pub adjusted_fitness: Option<f64>,
    pub stats: Option<S>,
    pub generation: u64,
}

impl<G: Genome, S> Organism<G, S> {
    pub fn new(config: &G::Config, init_config: &G::InitConfig, state: &mut G::State) -> Self {
        Self {
            genome: G::new(config, init_config, state),
            fitness: None,
            adjusted_fitness: None,
            stats: None,
            generation: 0,
        }
    }

    /// Breed organism with other organism
    pub fn crossover(&self, config: &G::Config, other: &Self) -> Self {
        Organism {
            genome: self.genome.crossover(
                config,
                &other.genome,
                &self.fitness.unwrap(),
                &other.fitness.unwrap(),
            ),
            fitness: None,
            adjusted_fitness: None,
            stats: None,
            generation: self.generation + 1,
        }
    }

    /// Compare to other organism based on non-adjusted fitness
    pub fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }

    /// Mutate organism
    pub fn mutate(&mut self, config: &G::Config, state: &mut G::State) {
        self.genome.mutate(config, state);
    }

    /// Genetic distance to other organism
    pub fn distance(&self, config: &G::Config, other: &Self) -> f64 {
        self.genome.distance(config, &other.genome)
    }

    /// Produce an elite for the next generation
    pub fn as_elite(&self) -> Self {
        Self {
            genome: self.genome.clone(),
            fitness: None,
            adjusted_fitness: None,
            stats: None,
            generation: self.generation + 1,
        }
    }
}
