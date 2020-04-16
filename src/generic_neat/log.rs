use crate::generic_neat::population::Population;
use crate::generic_neat::dot;

pub trait Log {
    fn log(&mut self, iteration: u64, population: &Population);
}

#[derive(Default)]
pub struct Logger {}

impl Log for Logger {
    fn log(&mut self, iteration: u64, population: &Population) {
        if iteration % 10 == 0 {
            let best = &population.best().unwrap();
            println!("Iter: {}\t Fitness: {}", iteration, best.fitness);
            println!("{}", population);
            dot::genome_to_dot(String::from("g.dot"), &best.genome).ok();
        }
    }
}
