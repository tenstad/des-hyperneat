use crate::neat::genome::Genome;
use std::cmp;

pub struct Organism {
    pub genome: Genome,
    pub fitness: f64,
    pub generation: u64,
}

impl Organism {
    pub fn new(generation: u64, inputs: u64, outputs: u64) -> Organism {
        Organism {
            genome: Genome::new(inputs, outputs),
            fitness: 0.0,
            generation: generation,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }

    pub fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }
}
