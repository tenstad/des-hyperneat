use crate::deshyperneat::log::Logger as DeshyperneatLogger;
use crate::sideshyperneat::{dot::genome_to_dot, genome::Genome};
use evolution::{
    environment::{EnvironmentDescription, Stats},
    log,
    population::Population,
};
use serde::Serialize;

pub struct Logger {
    deshyperneat_logger: DeshyperneatLogger,
    log_interval: u64,
}

impl<C: Serialize> log::CreateLog<C> for Logger {
    fn new(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            deshyperneat_logger: DeshyperneatLogger::new(description, config),
            log_interval: 10,
        }
    }
}

impl<S: Stats> log::LogEntry<Genome, S> for Logger {
    fn log(&mut self, iteration: u64, population: &Population<Genome, S>) {
        self.deshyperneat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            if let Some(best) = &population.best() {
                genome_to_dot("g", &best.genome).ok();
            }
        }
    }
}

impl<C: Serialize, S: Stats> log::Log<C, Genome, S> for Logger {}
