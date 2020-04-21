pub mod conf;
pub mod img;
pub mod log;
pub mod phenotype;
pub mod search;

use crate::eshyperneat::{log::Logger, phenotype::Developer};
use crate::neat::genome::Genome;
use evolution::{environment::Environment, evolve};
use network::execute::Executor;

pub fn eshyperneat<E: Environment<Executor> + Default>() {
    evolve::<Genome, Executor, Developer, E, Logger>();
}
