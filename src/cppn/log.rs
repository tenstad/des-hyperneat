use crate::cppn::{dot::genome_to_dot, genome::Genome};
use evolution::{
    environment::{EnvironmentDescription, Stats},
    log,
    population::Population,
};

pub struct Logger {
    default_logger: log::Logger,
    log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            default_logger: log::Logger::from(description),
            log_interval: 10,
        }
    }
}

impl<S: Stats> log::Log<Genome, S> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome, S>) {
        self.default_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            if let Some(best) = &population.best() {
                genome_to_dot(String::from("g.dot"), &best.genome).ok();
            }
        }
    }
}
