use crate::develop::Develop;
use crate::environment::EnvironmentDescription;
use crate::genome::Genome;
use crate::log::Log;

pub trait Algorithm {
    type Genome: Genome + 'static;
    type Phenotype;
    type Developer: Develop<Self::Genome, Self::Phenotype>;
    type Logger: Log<Self::Genome>;

    fn genome_init_config(
        e: &EnvironmentDescription,
    ) -> <<Self as Algorithm>::Genome as Genome>::InitConfig;
}
