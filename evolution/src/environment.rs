pub trait Environment<P>: Default {
    fn fitness(&self, phenotype: &mut P) -> f64;
    fn description(&self) -> EnvironmentDescription;
}

#[derive(new, Copy, Clone, Default)]
pub struct EnvironmentDescription {
    pub inputs: usize,
    pub outputs: usize,
}

#[derive(new)]
pub struct DummyEnvironment {
    description: EnvironmentDescription,
}

impl Default for DummyEnvironment {
    fn default() -> Self {
        Self {
            description: EnvironmentDescription::default(),
        }
    }
}

impl Environment<()> for DummyEnvironment {
    fn description(&self) -> EnvironmentDescription {
        self.description.clone()
    }

    fn fitness(&self, _: &mut ()) -> f64 {
        0.0
    }
}
