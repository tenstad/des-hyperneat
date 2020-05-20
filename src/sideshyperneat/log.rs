use crate::deshyperneat::log::Logger as DeshyperneatLogger;
use crate::sideshyperneat::{conf::SIDESHYPERNEAT, dot::genome_to_dot, genome::Genome};
use evolution::{
    environment::EnvironmentDescription, log, population::Population, stats::GetPopulationStats,
};
use serde::Serialize;

pub struct Logger {
    deshyperneat_logger: DeshyperneatLogger,
    log_interval: u64,
}

impl log::Log<Genome> for Logger {
    fn new<C: Serialize>(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            deshyperneat_logger: <DeshyperneatLogger as log::Log<Genome>>::new(description, config),
            log_interval: 10,
        }
    }

    fn log<S: GetPopulationStats>(
        &mut self,
        iteration: u64,
        population: &Population<Genome>,
        stats: &S,
    ) {
        self.deshyperneat_logger.log(iteration, population, stats);

        if SIDESHYPERNEAT.log_visualizations && iteration % self.log_interval == 0 {
            if let Some(best) = &population.best() {
                genome_to_dot("g", &best.genome).ok();
            }
        }
    }
}
