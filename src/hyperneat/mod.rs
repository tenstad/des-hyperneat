pub mod conf;
pub mod img;
pub mod log;
pub mod phenotype;
pub mod substrate;

use crate::neat::genome::Genome;
use evolution::{environment::Environment, evolve};
use log::Logger;
use network::execute::Executor;
use phenotype::Developer;

pub fn hyperneat<E: Environment<Executor> + Default>() {
    evolve::<Genome, Executor, Developer, E, Logger>();
}
