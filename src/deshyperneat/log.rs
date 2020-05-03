use crate::deshyperneat::figure::save_fig_to_file;
use crate::deshyperneat::{developer::Developer, genome::Genome};
use crate::eshyperneat::conf::ESHYPERNEAT;
use evolution::{environment::EnvironmentDescription, log, population::Population};

pub struct Logger {
    default_logger: log::Logger,
    developer: Developer,
    log_interval: usize,
}

impl From<EnvironmentDescription> for Logger {
    fn from(description: EnvironmentDescription) -> Self {
        Self {
            default_logger: log::Logger::from(description),
            developer: Developer::from(description),
            log_interval: 10,
        }
    }
}

impl log::Log<Genome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome>) {
        self.default_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            save_fig_to_file(
                self.developer
                    .connections(&population.best().unwrap().genome),
                "g.tex",
                0.5 / ESHYPERNEAT.resolution,
                4.0,
            );
        }
    }
}
