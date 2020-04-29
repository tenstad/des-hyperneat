pub mod conf;
pub mod developer;
pub mod img;
pub mod log;
pub mod substrate;

use crate::cppn::genome::Genome;
use developer::Developer;
use evolution::{
    algorithm::Algorithm, environment::Environment, environment::EnvironmentDescription, evolve,
    neat::state::InitConfig,
};
use log::Logger;
use network::execute::Executor;

pub struct Hyperneat;

impl Algorithm for Hyperneat {
    type Genome = Genome;
    type Phenotype = Executor;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_init_config(_: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(4, 2)
    }
}

pub fn hyperneat<E: Environment<Executor> + Default>() {
    evolve::<Hyperneat, E>();
}
