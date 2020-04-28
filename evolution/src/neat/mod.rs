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

impl Algorithm for Neat {
    type Genome = Genome;
    type Phenotype = Executor;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_init_config(e: &EnvironmentDescription) -> genome_core::InitConfig {
        genome_core::InitConfig::new(e.inputs, e.outputs)
    }
}

pub fn neat<E: Environment<Executor> + Default>() {
    evolve::<Neat, E>();
}
