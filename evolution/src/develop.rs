use crate::environment::EnvironmentDescription;

pub trait Develop<G, P>: From<EnvironmentDescription> {
    fn develop(&self, genome: &G) -> P;
}
