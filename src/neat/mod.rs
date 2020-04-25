pub mod conf;
pub mod dot;
pub mod genome;
pub mod link;
pub mod log;
pub mod node;
pub mod phenotype;
pub mod state;

use evolution::{environment::Environment, evolve};
use genome::Genome;
use log::Logger;
use network::execute::Executor;
use phenotype::Developer;

pub fn neat<E: Environment<Executor> + Default>() {
    evolve::<Genome, Executor, Developer, E, Logger>();
}
