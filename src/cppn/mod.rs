use developer::Developer;
use evolution::{
    algorithm::Algorithm, environment::Environment, environment::EnvironmentDescription, evolve,
    neat::genome_core::InitConfig,
};
use genome::Genome;
use log::Logger;
use network::execute::Executor;

pub mod conf;
pub mod developer;
pub mod dot;
pub mod genome;
pub mod log;
pub mod node;

pub struct Cppn;

impl Algorithm for Cppn {
    type Genome = Genome;
    type Phenotype = Executor;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_init_config(e: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(e.inputs, e.outputs)
    }
}

pub fn cppn<E: Environment<Executor> + Default>() {
    evolve::<Cppn, E>();
}
