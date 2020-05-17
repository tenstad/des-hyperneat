use crate::develop::Develop;
use crate::environment::{Environment, EnvironmentDescription};
use crate::genome::Genome;
use serde::Serialize;

pub trait Algorithm<E: Environment> {
    type Config: Serialize + Default;
    type Genome: Genome + 'static;
    type Developer: Develop<Self::Genome, Phenotype = E::Phenotype> + 'static;

    fn genome_config(e: &EnvironmentDescription) -> <Self::Genome as Genome>::Config;
    fn genome_init_config(e: &EnvironmentDescription) -> <Self::Genome as Genome>::InitConfig;
}
