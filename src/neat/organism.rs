use crate::neat::environment::Environment;
use crate::neat::genome::Genome;
use crate::neat::population::InnovationLog;
use crate::neat::population::InnovationTime;
use std::cmp;

pub struct Organism {
    pub genome: Genome,
    pub fitness: f64,
    pub sharing: f64,
    pub generation: u64,
}

impl Organism {
    pub fn new(generation: u64, inputs: u64, outputs: u64) -> Organism {
        Organism {
            genome: Genome::new(inputs, outputs),
            fitness: 0.0,
            sharing: 0.0,
            generation: generation,
        }
    }

    pub fn crossover(&self, other: &Self) -> Self {
        Organism {
            genome: self
                .genome
                .crossover(&other.genome, self.fitness > other.fitness),
            fitness: 0.0,
            sharing: 0.0,
            generation: self.generation + 1,
        }
    }

    pub fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }

    pub fn evaluate(&mut self, environment: &dyn Environment, sharing: u64) {
        self.fitness = environment.evaluate(&self.genome);
        self.sharing = sharing as f64;
    }

    pub fn mutate(&mut self, log: &mut InnovationLog, global_innovation: &mut InnovationTime) {
        self.genome.mutate(log, global_innovation);
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }
}
