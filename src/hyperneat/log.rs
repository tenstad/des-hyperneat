use crate::hyperneat::img;
use crate::neat::genome::Genome;
use crate::neat::log::Logger as NeatLogger;
use crate::neat::phenotype::Developer;
use evolution::genome::Develop;
use evolution::log::Log;
use evolution::population::Population;
use network::execute::Executor;

pub struct Logger {
    neat_logger: NeatLogger,
    developer: Developer,
    log_interval: usize,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            neat_logger: NeatLogger::default(),
            developer: Developer::default(),
            log_interval: 10,
        }
    }
}

impl Log<Genome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome>) {
        self.neat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let developer: &dyn Develop<Genome, Executor> = &self.developer;
            let mut phenotype = developer.develop(&population.best().unwrap().genome);

            img::plot_weights(&mut phenotype, -1.0, -1.0, 1.0, 256)
                .save("w.png")
                .ok();
        }
    }
}
