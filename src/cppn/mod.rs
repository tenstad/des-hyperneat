use conf::MethodConfig;
use developer::Developer;
use evolution::{
    algorithm::Algorithm,
    environment::Environment,
    environment::EnvironmentDescription,
    evolve,
    neat::{conf::NeatConfig, state::InitConfig},
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

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Cppn {
    type Genome = Genome;
    type Developer = Developer;

    fn genome_config(_: &EnvironmentDescription) -> NeatConfig {
        NeatConfig::default()
    }

    fn genome_init_config(e: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(e.inputs, e.outputs)
    }
}

pub fn cppn<E: Environment<Phenotype = Executor> + Default + 'static>() {
    evolve::<E, Cppn, Logger, MethodConfig>();
}
