pub mod conf;
pub mod developer;
pub mod figure;
pub mod genome;
pub mod img;
pub mod log;
pub mod search;

use crate::cppn::genome::Genome;
use crate::eshyperneat::{developer::Developer, log::Logger};
use evolution::{
    algorithm::Algorithm, environment::Environment, environment::EnvironmentDescription, evolve,
    neat::state::InitConfig,
};
use network::execute::Executor;

pub struct Eshyperneat;

impl<E: Environment<Phenotype = Executor>> Algorithm<E> for Eshyperneat {
    type Genome = Genome;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_init_config(_: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(4, 2)
    }
}

pub fn eshyperneat<E: Environment<Phenotype = Executor> + Default + 'static>() {
    evolve::<E, Eshyperneat>();
}
