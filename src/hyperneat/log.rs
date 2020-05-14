use crate::cppn::{developer::Developer, genome::Genome, log::Logger as CppnLogger};
use crate::hyperneat::img;
use evolution::{
    develop::Develop,
    environment::{EnvironmentDescription, Stats},
    log,
    population::Population,
};
use serde::Serialize;

pub struct Logger {
    cppn_logger: CppnLogger,
    developer: Developer,
    log_interval: u64,
}

impl<C: Serialize> log::CreateLog<C> for Logger {
    fn new(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            cppn_logger: CppnLogger::new(description, config),
            developer: Developer::from(description.clone()),
            log_interval: 10,
        }
    }
}

impl<S: Stats> log::LogEntry<Genome, S> for Logger {
    fn log(&mut self, iteration: u64, population: &Population<Genome, S>) {
        self.cppn_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let mut phenotype = self
                .developer
                .develop(population.best().unwrap().genome.clone());

            img::plot_weights(&mut phenotype, -1.0, -1.0, 1.0, 256)
                .save("w.png")
                .ok();
        }
    }
}

impl<C: Serialize, S: Stats> log::Log<C, Genome, S> for Logger {}
