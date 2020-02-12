use crate::generic_neat::environment::Environment;
use crate::generic_neat::genome::Genome;
use crate::generic_neat::innovation::InnovationLog;
use crate::generic_neat::innovation::InnovationTime;
use crate::generic_neat::link;
use crate::generic_neat::node;
use std::cmp;

#[derive(Clone)]
pub struct Organism<I, H, O, L> {
    pub genome: Genome<I, H, O, L>,
    pub fitness: f64,
    pub adjusted_fitness: f64,
    pub generation: u64,
}

impl<I: node::Custom, H: node::Custom, O: node::Custom, L: link::Custom> Organism<I, H, O, L> {
    pub fn new(generation: u64, inputs: u64, outputs: u64) -> Organism<I, H, O, L> {
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

    /// Evaluate organism
    pub fn evaluate(&mut self, environment: &dyn Environment<I, H, O, L>) {
        self.fitness = environment.fitness(&self.genome);
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
