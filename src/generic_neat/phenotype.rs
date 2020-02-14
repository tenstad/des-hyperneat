use crate::generic_neat::genome;

pub trait Develop<I, H, O, L, P> {
    fn develop(&self, genome: &genome::Genome<I, H, O, L>) -> P;
}
