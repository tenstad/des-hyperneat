use crate::neat::genome::Genome;
use std::cmp;

pub struct Organism {
    pub genome: Genome,
    pub fitness: f64,
    pub shared_fitness: f64,
    pub generation: u64,
}

impl Organism {
    pub fn new(generation: u64, inputs: u64, outputs: u64) -> Organism {
        Organism {
            genome: Genome::new(inputs, outputs),
            fitness: 0.0,
            shared_fitness: 0.0,
            generation: generation,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }

    pub fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }

    pub fn evaluate(&mut self, inputs: &Vec<f64>, targets: &Vec<f64>, sharing: u64) {
        let outputs = self.genome.evaluate(inputs);

        self.fitness = targets
            .iter()
            .enumerate()
            .map(|(i, target)| f64::powf(outputs.get(&(i as u64)).unwrap_or(&0.0) - target, 2.0))
            .sum::<f64>()
            / targets.len() as f64;
        
        self.shared_fitness = self.fitness / sharing as f64;
    }
}
