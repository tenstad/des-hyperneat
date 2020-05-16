use crate::algorithm::Algorithm;
use crate::environment::EnvironmentDescription;
use crate::log::Logger;
use crate::neat::genome::DefaultNeatGenome;
use crate::{conf::NoConfig, environment::Environment, evolve};
use developer::Developer;
use network::execute::Executor;
use serde::Serialize;

pub mod conf;
pub mod developer;
pub mod genome;
pub mod link;
pub mod node;
pub mod state;

pub struct Neat;

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Neat {
    type Genome = DefaultNeatGenome;
    type Developer = Developer;

    fn genome_config(_: &EnvironmentDescription) -> conf::NeatConfig {
        conf::NeatConfig::default()
    }

    fn genome_init_config(e: &EnvironmentDescription) -> state::InitConfig {
        state::InitConfig::new(e.inputs, e.outputs)
    }
}

pub fn neat<E: Environment<Phenotype = Executor> + Default + 'static, N: Serialize + Default>() {
    evolve::<E, Neat, Logger, NoConfig, N>();
}
