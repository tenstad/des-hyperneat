use crate::deshyperneat::log::Logger as DeshyperneatLogger;
use crate::sideshyperneat::{dot::genome_to_dot, genome::Genome};
use evolution::{
    environment::{EnvironmentDescription, Stats},
    log,
    population::Population,
};

pub struct Logger {
    deshyperneat_logger: DeshyperneatLogger,
    log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            deshyperneat_logger: DeshyperneatLogger::from(description),
            log_interval: 10,
        }
    }
}

impl<S: Stats> log::Log<Genome, S> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome, S>) {
        self.deshyperneat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            if let Some(best) = &population.best() {
                genome_to_dot("g", &best.genome).ok();
            }
        }
    }
}
