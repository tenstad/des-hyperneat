use crate::deshyperneat::{desgenome::DesGenome, developer::Developer, figure::save_fig_to_file};
use crate::eshyperneat::conf::ESHYPERNEAT;
use evolution::{
    environment::EnvironmentDescription, evaluate::GetPopulationStats, genome::Genome, log,
    population::Population,
};
use serde::Serialize;

pub struct Logger {
    default_logger: log::Logger,
    developer: Developer,
    log_interval: u64,
}

impl<G: Genome + DesGenome> log::Log<G> for Logger {
    fn new<C: Serialize>(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            default_logger: <log::Logger as log::Log<G>>::new(description, config),
            developer: Developer::from(description.clone()),
            log_interval: 10,
        }
    }

    fn log<S: GetPopulationStats>(
        &mut self,
        iteration: u64,
        population: &Population<G>,
        stats: &S,
    ) {
        self.default_logger.log(iteration, population, stats);

        if iteration % self.log_interval == 0 {
            save_fig_to_file(
                self.developer
                    .connections(population.best().unwrap().genome.clone()),
                "g.tex",
                0.5 / ESHYPERNEAT.resolution,
                4.0,
            );
        }
    }
}
