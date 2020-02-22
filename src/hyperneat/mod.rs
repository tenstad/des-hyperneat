pub mod dataset_environment;
pub mod img;
mod phenotype;
pub mod substrate;

use crate::generic_neat;
use crate::generic_neat::evaluate;
use crate::network::execute;

struct Logger {
    developer: crate::neat::phenotype::Developer,
    iter: u64,
}

impl Default for Logger {
    fn default() -> Logger {
        Logger {
            developer: crate::neat::phenotype::Developer::default(),
            iter: 0,
        }
    }
}

impl crate::generic_neat::log::Log for Logger {
    fn log(&mut self, organism: &crate::generic_neat::organism::Organism) {
        self.iter += 1;

        if self.iter % 20 == 0 {
            let developer: &dyn evaluate::Develop<execute::Executor> = &self.developer;
            crate::hyperneat::img::plot_weights(
                developer.develop(&organism.genome),
                0.0,
                0.0,
                10.0,
                256,
                "w0_0.png",
            );
        }

        if self.iter % 20 == 0 {
            let developer: &dyn evaluate::Develop<execute::Executor> = &self.developer;
            crate::hyperneat::img::plot_weights(
                developer.develop(&organism.genome),
                -1.0,
                -1.0,
                10.0,
                256,
                "w-1_-1.png",
            );
        }
    }
}

pub fn hyperneat<E: evaluate::Environment<execute::Executor> + Default>() {
    generic_neat::neat::<execute::Executor, E, phenotype::Developer, Logger>();
}
