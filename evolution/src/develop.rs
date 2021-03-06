use crate::environment::EnvironmentDescription;
use crate::stats::Stats;

pub trait Develop<G>: From<EnvironmentDescription> {
    type Phenotype;
    type Stats: Stats;

    fn develop(&self, genome: G) -> (Self::Phenotype, Self::Stats);
}
