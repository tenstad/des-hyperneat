pub mod conf;
pub mod desgenome;
pub mod developer;
pub mod figure;
pub mod genome;
pub mod link;
pub mod log;
pub mod node;

use developer::Developer;
use evolution::{
    algorithm::Algorithm, environment::Environment, environment::EnvironmentDescription, evolve,
    neat::state::InitConfig,
};
use genome::Genome;
use log::Logger;
use network::execute::Executor;

pub struct Deshyperneat;

impl Algorithm for Deshyperneat {
    type Genome = Genome;
    type Phenotype = Executor;
    type Developer = Developer;
    type Logger = Logger;

    fn genome_init_config(_: &EnvironmentDescription) -> InitConfig {
        InitConfig::new(1, 1)
    }
}

pub fn deshyperneat<E: Environment<Executor> + Default>() {
    evolve::<Deshyperneat, E>();
}
