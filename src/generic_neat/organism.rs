use crate::generic_neat::evaluate;
use crate::generic_neat::genome::Genome;
use crate::generic_neat::innovation::InnovationLog;
use crate::generic_neat::innovation::InnovationTime;
use std::cmp;

#[derive(Clone)]
pub struct Organism {
    pub genome: Genome,
    pub fitness: f64,
    pub adjusted_fitness: f64,
    pub generation: u64,
}

impl Organism {
    pub fn new(generation: u64, inputs: usize, outputs: usize) -> Organism {
        Organism {
            genome: Genome::new(inputs, outputs),
            fitness: 0.0,
            adjusted_fitness: 0.0,
            generation: generation,
        }
    }

    /// Breed organism with other organism
    pub fn crossover(&self, other: &Self) -> Self {
        Organism {
            genome: self
                .genome
                .crossover(&other.genome, self.fitness > other.fitness),
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
    pub fn fitness<P>(
        &mut self,
        environment: &dyn evaluate::Environment<P>,
        developer: &dyn evaluate::Develop<P>,
    ) -> f64 {
        let mut phenotype = developer.develop(&self.genome);
        environment.fitness(&mut phenotype)
    }

    /// Mutate organism
    pub fn mutate(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        self.genome.mutate(log, global_innovation);
    }

    /// Genetic distance to other organism
    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }
}
