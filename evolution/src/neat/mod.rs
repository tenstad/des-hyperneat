use crate::log::Logger;
use crate::{environment::Environment, evolve};
use genome::DefaultNeatGenome as Genome;
use network::execute::Executor;
use phenotype::Developer;

pub mod conf;
pub mod genome;
pub mod genome_core;
pub mod link;
pub mod node;
pub mod phenotype;
pub mod state;

pub fn neat<E: Environment<Executor> + Default>() {
    evolve::<Genome, Executor, Developer, E, Logger>();
}
