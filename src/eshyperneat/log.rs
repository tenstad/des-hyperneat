use crate::eshyperneat::conf::ESHYPERNEAT;
use crate::eshyperneat::{img, phenotype::Developer};
use crate::hyperneat::log::Logger as HyperneatLogger;
use crate::neat::{genome::Genome, phenotype::Developer as NeatDeveloper};
use evolution::{genome::Develop, log, population::Population};
use network::execute;

pub struct Logger {
    hyperneat_logger: HyperneatLogger,
    neat_developer: NeatDeveloper,
    developer: Developer,
    log_interval: usize,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            hyperneat_logger: HyperneatLogger::default(),
            neat_developer: NeatDeveloper::default(),
            developer: Developer::default(),
            log_interval: 10,
        }
    }
}

impl log::Log<Genome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome>) {
        self.hyperneat_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            let neat_developer: &dyn Develop<Genome, execute::Executor> = &self.neat_developer;
            let mut phenotype = neat_developer.develop(&population.best().unwrap().genome);

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
