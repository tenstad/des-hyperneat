use crate::environment::{EnvironmentDescription, Stats};
use crate::genome::Genome;
use crate::population::Population;

pub trait Log<G: Genome, S: Stats>: From<EnvironmentDescription> {
    fn log(&mut self, iteration: usize, population: &Population<G, S>);
}

pub struct Logger {
    pub log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(_: EnvironmentDescription) -> Self {
        Self { log_interval: 10 }
    }
}

impl<G: Genome, S: Stats> Log<G, S> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<G, S>) {
        if iteration % self.log_interval == 0 {
            print!("Iter: {}", iteration);
            if let Some(best) = &population.best() {
                if let Some(fitness) = best.fitness {
                    print!("\t Fitness: {}", fitness);
                }
                if let Some(stats) = &best.stats {
                    print!("\n{}", stats);
                }
            }
            println!("\n{}", population);
        }
    }
}
