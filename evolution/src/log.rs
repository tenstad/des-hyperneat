use crate::genome::Genome;
use crate::population::Population;

pub trait Log<G: Genome> {
    fn log(&mut self, iteration: usize, population: &Population<G>);
}

pub struct Logger {
    pub log_interval: usize,
}

impl Default for Logger {
    fn default() -> Self {
        Self { log_interval: 10 }
    }
}

impl<G: Genome> Log<G> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<G>) {
        if iteration % self.log_interval == 0 {
            print!("Iter: {}", iteration);
            if let Some(best) = &population.best() {
                print!("\t Fitness: {}", best.fitness);
            }
            println!("\n{}", population);
        }
    }
}
