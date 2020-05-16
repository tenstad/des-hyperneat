pub mod conf;
pub mod developer;
pub mod figure;
pub mod genome;
pub mod img;
pub mod log;
pub mod search;

use crate::cppn::genome::Genome;
use crate::eshyperneat::{developer::Developer, log::Logger};
use conf::MethodConfig;
use evolution::{
    algorithm::Algorithm,
    environment::{Environment, EnvironmentDescription},
    evolve,
    neat::{conf::NeatConfig, state::InitConfig},
};
use network::execute::Executor;
use serde::Serialize;

pub struct Eshyperneat;

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Eshyperneat {
    type Genome = Genome;
    type Developer = Developer;

    fn genome_config(_: &EnvironmentDescription) -> NeatConfig {
        NeatConfig::default()
    }

    fn genome_init_config(_: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(4, 2)
    }
}

pub fn eshyperneat<
    E: Environment<Phenotype = Executor> + Default + 'static,
    N: Serialize + Default,
>() {
    evolve::<E, Eshyperneat, Logger, MethodConfig, N>();
}
