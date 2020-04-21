pub trait Environment<P> {
    fn fitness(&self, phenotype: &mut P) -> f64;
    fn description(&self) -> EnvironmentDescription;
}

#[derive(new, Clone)]
pub struct EnvironmentDescription {
    pub inputs: usize,
    pub outputs: usize,
}
