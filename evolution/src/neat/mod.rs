use crate::algorithm::Algorithm;
use crate::environment::EnvironmentDescription;
use crate::log::Logger;
use crate::{environment::Environment, evolve};
use developer::Developer;
use genome::DefaultNeatGenome as Genome;
use network::execute::Executor;

pub mod conf;
pub mod developer;
pub mod genome;
pub mod genome_core;
pub mod link;
pub mod node;
pub mod state;

pub struct Neat;

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Neat {
    type Genome = Genome;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_init_config(e: &EnvironmentDescription) -> state::InitConfig {
        state::InitConfig::new(e.inputs, e.outputs)
    }
}

pub fn neat<E: Environment<Phenotype = Executor> + Default + 'static>() {
    evolve::<E, Neat>();
}
