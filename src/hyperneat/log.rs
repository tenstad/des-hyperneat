use crate::hyperneat::img;
use crate::neat::genome::Genome as NeatGenome;
use crate::neat::log::Logger as NeatLogger;
use crate::neat::phenotype::Developer;
use evolution::environment::EnvironmentDescription;
use evolution::genome::Develop;
use evolution::log::Log;
use evolution::population::Population;

pub struct Logger {
    neat_logger: NeatLogger,
    developer: Developer,
    log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            neat_logger: NeatLogger::from(description),
            developer: Developer::from(description),
            log_interval: 10,
        }
    }
}

impl Log<NeatGenome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<NeatGenome>) {
        self.neat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let mut phenotype = self.developer.develop(&population.best().unwrap().genome);

            img::plot_weights(&mut phenotype, -1.0, -1.0, 1.0, 256)
                .save("w.png")
                .ok();
        }
    }
}
