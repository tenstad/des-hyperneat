use crate::neat::dot::genome_to_dot;
use crate::neat::genome::Genome;
use evolution::log;
use evolution::population::Population;

pub struct Logger {
    default_logger: log::Logger,
    log_interval: usize,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            default_logger: log::Logger::default(),
            log_interval: 10,
        }
    }
}

impl log::Log<Genome> for Logger {
    fn log(&mut self, iteration: usize, population: &Population<Genome>) {
        self.default_logger.log(iteration, population);

        if iteration % self.log_interval == 0 {
            if let Some(best) = &population.best() {
                genome_to_dot(String::from("g.dot"), &best.genome).ok();
            }
        }
    }
}
