use crate::conf::EVOLUTION;
use crate::environment::EnvironmentDescription;
use crate::genome::Genome;
use crate::population::Population;
use crate::stats::GetPopulationStats;
use db::{Entry, Mongo};
use serde::Serialize;
use serde_yaml;
use std::time::{Duration, SystemTime};

pub trait Log<G: Genome> {
    fn new<C: Serialize>(description: &EnvironmentDescription, config: &C) -> Self;
    fn log<S: GetPopulationStats>(&mut self, iteration: u64, population: &Population<G>, stats: &S);
}

pub struct Logger {
    pub log_interval: u64,
    pub log_seconds: u64,
    pub prev_log_time: SystemTime,
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
            log_interval: EVOLUTION.log_interval,
            log_seconds: EVOLUTION.log_sec_interval,
            prev_log_time: SystemTime::now(),
            entry,
        }
    }

    fn log<S: GetPopulationStats>(
        &mut self,
        iteration: u64,
        population: &Population<G>,
        stats: &S,
    ) {
        if iteration == 0 {
            self.prev_log_time =
                SystemTime::now() - Duration::from_secs(EVOLUTION.log_sec_interval);
        }

        let log = (self.log_interval > 0 && iteration % self.log_interval == 0)
            || (self.log_seconds > 0
                && SystemTime::elapsed(&self.prev_log_time).unwrap()
                    >= Duration::from_secs(self.log_seconds));

        if log {
            self.prev_log_time += Duration::from_secs(self.log_seconds);

            println!("Iter: {}", iteration);

            let population_stats = stats.population();

            if let Some(best) = population_stats.best() {
                println!("{}", serde_yaml::to_string(best).unwrap());
            }

            if let Some(entry) = &mut self.entry {
                entry.push(&population_stats, iteration);
            }

            println!("{}", population);
        }
    }
}
