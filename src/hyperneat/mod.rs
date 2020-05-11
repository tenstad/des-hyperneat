pub mod conf;
pub mod developer;
pub mod img;
pub mod log;
pub mod substrate;

use crate::cppn::genome::Genome;
use developer::Developer;
use evolution::{
    algorithm::Algorithm,
    environment::{Environment, EnvironmentDescription},
    evolve,
    neat::{conf::NeatConfig, state::InitConfig},
};
use log::Logger;
use network::execute::Executor;

pub struct Hyperneat;

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Hyperneat {
    type Genome = Genome;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_config(_: &EnvironmentDescription) -> NeatConfig {
        NeatConfig::default()
    }

    fn genome_init_config(_: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(4, 2)
    }
}

pub fn hyperneat<E: Environment<Phenotype = Executor> + Default + 'static>() {
    evolve::<E, Hyperneat>();
}
