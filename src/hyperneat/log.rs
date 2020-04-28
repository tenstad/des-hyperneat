use crate::cppn::{developer::Developer, genome::Genome, log::Logger as CppnLogger};
use crate::hyperneat::img;
use evolution::{
    develop::Develop, environment::EnvironmentDescription, log, population::Population,
};

pub struct Logger {
    cppn_logger: CppnLogger,
    developer: Developer,
    log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            cppn_logger: CppnLogger::from(description),
            developer: Developer::from(description),
            log_interval: 10,
        }
    }
}

impl log::Log<Genome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome>) {
        self.cppn_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let mut phenotype = self.developer.develop(&population.best().unwrap().genome);

            img::plot_weights(&mut phenotype, -1.0, -1.0, 1.0, 256)
                .save("w.png")
                .ok();
        }
    }
}
