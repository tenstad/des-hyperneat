use crate::conf;
use crate::eshyperneat::img;
use crate::eshyperneat::phenotype;
use crate::generic_neat::evaluate;
use crate::generic_neat::log;
use crate::generic_neat::population::Population;
use crate::neat;
use network::execute;

#[derive(Default)]
pub struct Logger {
    neat_developer: neat::phenotype::Developer,
    developer: phenotype::Developer,
    default_logger: log::Logger,
}

impl log::Log for Logger {
    fn log(&mut self, iteration: u64, population: &Population) {
        self.default_logger.log(iteration, population);

        if iteration % 20 == 0 {
            let neat_developer: &dyn evaluate::Develop<execute::Executor> = &self.neat_developer;
            let mut phenotype = neat_developer.develop(&population.best().unwrap().genome);

            img::plot_weights(&mut phenotype, 0.0, 0.0, 1.0, 256)
                .save("w.png")
                .ok();

            self.developer.connections(&mut phenotype).save_fig_to_file(
                "g.tex",
                0.5 / conf::ESHYPERNEAT.resolution,
                4.0,
            );
        }
    }
}
