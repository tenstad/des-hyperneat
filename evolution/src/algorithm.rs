use crate::develop::Develop;
use crate::environment::{Environment, EnvironmentDescription};
use crate::genome::Genome;

pub trait Algorithm<E: Environment> {
    type Genome: Genome + 'static;
    type Developer: Develop<Self::Genome, E::Phenotype>;

    fn genome_config(e: &EnvironmentDescription) -> <Self::Genome as Genome>::Config;
    fn genome_init_config(e: &EnvironmentDescription) -> <Self::Genome as Genome>::InitConfig;
}
