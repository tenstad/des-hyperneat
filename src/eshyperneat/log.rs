use crate::cppn::{developer::Developer as CppnDeveloper, genome::Genome};
use crate::eshyperneat::{conf::ESHYPERNEAT, developer::Developer, figure::save_fig_to_file, img};
use crate::hyperneat::log::Logger as HyperneatLogger;
use evolution::{
    develop::Develop,
    environment::{EnvironmentDescription, Stats},
    log::{self},
    population::Population,
};
use serde::Serialize;

pub struct Logger {
    hyperneat_logger: HyperneatLogger,
    neat_developer: CppnDeveloper,
    developer: Developer,
    log_interval: u64,
}

impl<C: Serialize> log::CreateLog<C> for Logger {
    fn new(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            hyperneat_logger: HyperneatLogger::new(description, config),
            neat_developer: CppnDeveloper::from(description.clone()),
            developer: Developer::from(description.clone()),
            log_interval: 10,
        }
    }
}

impl<S: Stats> log::LogEntry<Genome, S> for Logger {
    fn log(&mut self, iteration: u64, population: &Population<Genome, S>) {
        self.hyperneat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let mut phenotype = self
                .neat_developer
                .develop(population.best().unwrap().genome.clone());

            img::plot_weights(&mut phenotype, 0.0, 0.0, 1.0, 256)
                .save("w.png")
                .ok();

            save_fig_to_file(
                self.developer.connections(&mut phenotype),
                "g.tex",
                0.5 / ESHYPERNEAT.resolution,
                4.0,
            );
        }
    }
}

impl<C: Serialize, S: Stats> log::Log<C, Genome, S> for Logger {}
