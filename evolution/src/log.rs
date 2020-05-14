use crate::environment::{EnvironmentDescription, Stats};
use crate::genome::Genome;
use crate::population::Population;
use db::{Entry, Mongo};
use serde::Serialize;
use serde_yaml;

pub trait Log<C: Serialize, G: Genome, S: Stats>: CreateLog<C> + LogEntry<G, S> {}

pub trait CreateLog<C: Serialize> {
    fn new(description: &EnvironmentDescription, config: &C) -> Self;
}

pub trait LogEntry<G: Genome, S: Stats> {
    fn log(&mut self, iteration: u64, population: &Population<G, S>);
}

pub struct Logger {
    pub log_interval: u64,
    pub entry: Entry,
}

impl<C: Serialize> CreateLog<C> for Logger {
    fn new(_: &EnvironmentDescription, config: &C) -> Self {
        Self {
            log_interval: 10,
            entry: Mongo::new().entry(config),
        }
    }
}

impl<G: Genome, S: Stats> LogEntry<G, S> for Logger {
    fn log(&mut self, iteration: u64, population: &Population<G, S>) {
        if iteration % self.log_interval == 0 {
            print!("Iter: {}", iteration);
            if let Some(best) = &population.best() {
                if let (Some(fitness), Some(stats)) = (best.fitness, &best.stats) {
                    self.entry.push(&stats);
                    let stats_str = serde_yaml::to_string(&stats).unwrap();
                    print!("\t Fitness: {}", fitness);
                    print!("\n{}", stats_str);
                }
            }
            println!("\n{}", population);
        }
    }
}

impl<C: Serialize, G: Genome, S: Stats> Log<C, G, S> for Logger {}
