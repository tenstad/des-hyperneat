use crate::cppn::{developer::Developer as CppnDeveloper, genome::Genome};
use crate::eshyperneat::{conf::ESHYPERNEAT, developer::Developer, figure::save_fig_to_file, img};
use crate::hyperneat::log::Logger as HyperneatLogger;
use evolution::{
    develop::Develop,
    environment::EnvironmentDescription,
    evaluate::GetPopulationStats,
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

impl log::Log<Genome> for Logger {
    fn new<C: Serialize>(description: &EnvironmentDescription, config: &C) -> Self {
        Self {
            hyperneat_logger: HyperneatLogger::new(description, config),
            neat_developer: CppnDeveloper::from(description.clone()),
            developer: Developer::from(description.clone()),
            log_interval: 10,
        }
    }

    fn log<S: GetPopulationStats>(
        &mut self,
        iteration: u64,
        population: &Population<Genome>,
        stats: &S,
    ) {
        self.hyperneat_logger.log(iteration, population, stats);

        if iteration % self.log_interval == 0 {
            let (mut phenotype, _) = self
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
