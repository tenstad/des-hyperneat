use crate::neat::genome::Genome;

pub struct Individual {
    genome: Genome,
    fitness: f64,
    generation: u64,
}

impl Individual {
    pub fn new(generation: u64, inputs: u64, outputs: u64) -> Individual {
        Individual {
            genome: Genome::new(inputs, outputs),
            fitness: 0.0,
            generation: generation,
        }
    }

    pub fn distance(&self, other: &Self) -> f64 {
        self.genome.distance(&other.genome)
    }
}
