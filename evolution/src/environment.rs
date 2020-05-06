use std::fmt::{Display, Formatter, Result};

pub trait Stats: Send + Display {}

pub struct NoStats;
impl Stats for NoStats {}
impl Display for NoStats {
    fn fmt(&self, _: &mut Formatter) -> Result {
        Ok(())
    }
}

pub trait Environment: Default {
    type Stats: Stats;
    type Phenotype;

    fn evaluate(&self, phenotype: &mut Self::Phenotype) -> (f64, Self::Stats);
    fn description(&self) -> EnvironmentDescription;
}

#[derive(new, Copy, Clone, Default)]
pub struct EnvironmentDescription {
    pub inputs: usize,
    pub outputs: usize,
}
