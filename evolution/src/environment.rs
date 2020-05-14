use serde::Serialize;

pub trait Stats: Send + Serialize {}

#[derive(Serialize)]
pub struct NoStats;
impl Stats for NoStats {}

pub trait Environment: Default {
    type Stats: Stats;
    type Phenotype;

    fn evaluate(&self, phenotype: &mut Self::Phenotype) -> (f64, Self::Stats);
    fn description(&self) -> EnvironmentDescription;
}

#[derive(new, Copy, Clone, Default, Serialize)]
pub struct EnvironmentDescription {
    pub inputs: u64,
    pub outputs: u64,
}
