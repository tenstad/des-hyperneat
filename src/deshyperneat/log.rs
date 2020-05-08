use crate::deshyperneat::{desgenome::DesGenome, developer::Developer, figure::save_fig_to_file};
use crate::eshyperneat::conf::ESHYPERNEAT;
use evolution::{
    environment::{EnvironmentDescription, Stats},
    genome::Genome,
    log,
    population::Population,
};

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

impl<S: Stats, G: DesGenome + Genome> log::Log<G, S> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<G, S>) {
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
