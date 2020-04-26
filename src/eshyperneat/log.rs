use crate::cppn::{genome::Genome, phenotype::Developer as CppnDeveloper};
use crate::eshyperneat::{conf::ESHYPERNEAT, img, phenotype::Developer};
use crate::hyperneat::log::Logger as HyperneatLogger;
use evolution::{
    environment::EnvironmentDescription, genome::Develop, log, population::Population,
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

            self.developer.connections(&mut phenotype).save_fig_to_file(
                "g.tex",
                0.5 / ESHYPERNEAT.resolution,
                4.0,
            );
        }
    }
}
