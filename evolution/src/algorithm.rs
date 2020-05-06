use crate::develop::Develop;
use crate::environment::{Environment, EnvironmentDescription};
use crate::genome::Genome;
use crate::log::Log;

pub trait Algorithm<E: Environment> {
    type Genome: Genome + 'static;
    type Developer: Develop<Self::Genome, E::Phenotype>;
    type Logger: Log<Self::Genome, E::Stats>;

    fn genome_init_config(e: &EnvironmentDescription) -> <Self::Genome as Genome>::InitConfig;
}
