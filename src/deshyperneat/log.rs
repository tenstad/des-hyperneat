use crate::deshyperneat::{desgenome::DesGenome, developer::Developer, figure::save_fig_to_file};
use crate::eshyperneat::conf::ESHYPERNEAT;
use evolution::{
    environment::{EnvironmentDescription, Stats},
    genome::Genome,
    log,
    population::Population,
};
use serde::Serialize;

pub struct Logger {
    default_logger: log::Logger,
    developer: Developer,
    log_interval: u64,
}

impl<C: Serialize> log::CreateLog<C> for Logger {
    fn new(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            default_logger: log::Logger::new(description, config),
            developer: Developer::from(description.clone()),
            log_interval: 10,
        }
    }
}

impl<G: Genome + DesGenome, S: Stats> log::LogEntry<G, S> for Logger {
    fn log(&mut self, iteration: u64, population: &Population<G, S>) {
        self.default_logger.log(iteration, population);

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

impl<C: Serialize, G: Genome + DesGenome, S: Stats> log::Log<C, G, S> for Logger {}
