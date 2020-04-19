use crate::generic_neat::evaluate;
use crate::generic_neat::log;
use crate::generic_neat::population::Population;
use crate::hyperneat::img;
use crate::neat::phenotype::Developer;
use network::execute;

#[derive(Default)]
pub struct Logger {
    developer: Developer,
    default_logger: log::Logger,
}

impl log::Log for Logger {
    fn log(&mut self, iteration: u64, population: &Population) {
        self.default_logger.log(iteration, population);

        if iteration % 20 == 0 {
            let developer: &dyn evaluate::Develop<execute::Executor> = &self.developer;
            let mut phenotype = developer.develop(&population.best().unwrap().genome);

            img::plot_weights(&mut phenotype, -1.0, -1.0, 1.0, 256)
                .save("w.png")
                .ok();
        }
    }
}
