use crate::generic_neat::dot;
use crate::generic_neat::log;
use crate::generic_neat::population::Population;
use crate::neat::phenotype::Developer;

pub struct Logger {
    developer: Developer,
    default_logger: log::Logger,
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            developer: Developer::default(),
            default_logger: log::Logger::default(),
        }
    }
}

impl log::Log for Logger {
    fn log(&mut self, iteration: u64, population: &Population) {
        self.default_logger.log(iteration, population);

        if iteration % 20 == 0 {
            dot::genome_to_dot(String::from("g.dot"), &population.best().unwrap().genome).ok();
        }
    }
}
