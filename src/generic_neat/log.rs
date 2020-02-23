use crate::generic_neat::population::Population;
use crate::generic_neat::dot;

pub trait Log {
    fn log(&mut self, iteration: u64, population: &Population, fitness: f64, accuracy: f64);
}

pub struct DefaultLogger {}

impl Log for DefaultLogger {
    fn log(&mut self, iteration: u64, population: &Population, fitness: f64, accuracy: f64) {
        if iteration % 10 == 0 {
            println!("Iter: {}\t Fitness: {} \t Acc: {}", iteration, fitness, accuracy);
            println!("{}", population);
            dot::genome_to_dot(String::from("g.dot"), &population.best().unwrap().genome).ok();
        }
    }
}

impl Default for DefaultLogger {
    fn default() -> DefaultLogger {
        DefaultLogger {}
    }
}
