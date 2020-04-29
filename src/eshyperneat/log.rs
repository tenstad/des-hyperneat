use crate::cppn::{developer::Developer as CppnDeveloper, genome::Genome};
use crate::eshyperneat::{conf::ESHYPERNEAT, developer::Developer, figure::save_fig_to_file, img};
use crate::hyperneat::log::Logger as HyperneatLogger;
use evolution::{
    develop::Develop, environment::EnvironmentDescription, log, population::Population,
};

pub struct Logger {
    hyperneat_logger: HyperneatLogger,
    neat_developer: CppnDeveloper,
    developer: Developer,
    log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            hyperneat_logger: HyperneatLogger::from(description),
            neat_developer: CppnDeveloper::from(description),
            developer: Developer::from(description),
            log_interval: 10,
        }
    }
}

impl log::Log<Genome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome>) {
        self.hyperneat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let mut phenotype = self
                .neat_developer
                .develop(&population.best().unwrap().genome);

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
