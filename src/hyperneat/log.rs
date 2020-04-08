use crate::generic_neat::evaluate;
use crate::generic_neat::log;
use crate::generic_neat::population::Population;
use crate::hyperneat::img;
use crate::neat::phenotype::Developer;
use crate::network::execute;

pub struct Logger {
    developer: Developer,
    default_logger: log::DefaultLogger,
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            developer: Developer::default(),
            default_logger: log::DefaultLogger::default(),
        }
    }
}

impl log::Log for Logger {
    fn log(&mut self, iteration: u64, population: &Population) {
        self.default_logger.log(iteration, population);

        if iteration % 20 == 0 {
            let developer: &dyn evaluate::Develop<execute::Executor> = &self.developer;

            img::plot_weights(
                developer.develop(&population.best().unwrap().genome),
                0.0,
                0.0,
                1.0,
                256,
                "w.png",
            );
        }
    }
}
