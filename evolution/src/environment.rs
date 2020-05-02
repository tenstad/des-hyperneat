pub trait Environment<P>: Default {
    fn fitness(&self, phenotype: &mut P) -> f64;
    fn description(&self) -> EnvironmentDescription;
}

#[derive(new, Copy, Clone, Default)]
pub struct EnvironmentDescription {
    pub inputs: usize,
    pub outputs: usize,
}
