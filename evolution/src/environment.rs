use std::fmt::{Display, Formatter, Result};

pub trait Stats: Send + Display {
    fn space_separated(&self) -> String;
}

pub struct NoStats;
impl Stats for NoStats {
    fn space_separated(&self) -> String {
        String::from("")
    }
}
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
