use crate::cppn::{developer::Developer, genome::Genome, log::Logger as CppnLogger};
use crate::hyperneat::img;
use evolution::{
    develop::Develop, environment::EnvironmentDescription, stats::GetPopulationStats, log,
    population::Population,
};
use serde::Serialize;

pub struct Logger {
    cppn_logger: CppnLogger,
    developer: Developer,
    log_interval: u64,
}

impl log::Log<Genome> for Logger {
    fn new<C: Serialize>(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            cppn_logger: CppnLogger::new(description, config),
            developer: Developer::from(description.clone()),
            log_interval: 10,
        }
    }

    fn log<S: GetPopulationStats>(
        &mut self,
        iteration: u64,
        population: &Population<Genome>,
        stats: &S,
    ) {
        self.cppn_logger.log(iteration, population, stats);

        if iteration % self.log_interval == 0 {
            let (mut phenotype, _) = self
                .developer
                .develop(population.best().unwrap().genome.clone());

            img::plot_weights(&mut phenotype, -1.0, -1.0, 1.0, 256)
                .save("w.png")
                .ok();
        }
    }
}
