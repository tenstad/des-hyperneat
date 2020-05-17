use crate::conf::EVOLUTION;
use crate::environment::EnvironmentDescription;
use crate::evaluate::GetPopulationStats;
use crate::genome::Genome;
use crate::population::Population;
use db::{Entry, Mongo};
use serde::Serialize;
use serde_yaml;

pub trait Log<G: Genome> {
    fn new<C: Serialize>(description: &EnvironmentDescription, config: &C) -> Self;
    fn log<S: GetPopulationStats>(&mut self, iteration: u64, population: &Population<G>, stats: &S);
}

pub struct Logger {
    pub log_interval: u64,
    pub entry: Option<Entry>,
}

impl<G: Genome> Log<G> for Logger {
    fn new<C: Serialize>(_: &EnvironmentDescription, config: &C) -> Self {
        let entry = if EVOLUTION.db_log {
            Some(Mongo::new().entry(config))
        } else {
            None
        };

        Self {
            log_interval: 10,
            entry,
        }
    }

    fn log<S: GetPopulationStats>(
        &mut self,
        iteration: u64,
        population: &Population<G>,
        stats: &S,
    ) {
        if iteration % self.log_interval == 0 {
            print!("Iter: {}", iteration);

            let population_stats = stats.population();

            if let Some(best) = population_stats.best() {
                print!("\n{}", serde_yaml::to_string(best).unwrap());
            }

            if let Some(entry) = &mut self.entry {
                entry.push(&population_stats, iteration);
            }

            println!("\n{}", population);
        }
    }
}
