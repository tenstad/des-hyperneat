pub mod conf;
pub mod develop;
pub mod dot;
pub mod genome;
pub mod link;
pub mod log;
pub mod node;
pub mod state;

use crate::deshyperneat::developer::Developer;
use evolution::{
    algorithm::Algorithm, environment::Environment, environment::EnvironmentDescription, evolve,
    neat::state::InitConfig,
};
use genome::Genome;
use log::Logger;
use network::execute::Executor;

pub struct Sideshyperneat;

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Sideshyperneat {
    type Genome = Genome;
    type Developer = Developer;

    fn genome_config(_: &EnvironmentDescription) -> conf::Config {
        conf::Config::default()
    }

    fn genome_init_config(_: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(3, 1)
    }
}

pub fn sideshyperneat<E: Environment<Phenotype = Executor> + Default + 'static>() {
    evolve::<E, Sideshyperneat, Logger>();
}
