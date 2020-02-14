use crate::generic_neat::phenotype;
use crate::generic_neat::default::Hidden as H;
use crate::generic_neat::default::Input as I;
use crate::generic_neat::default::Link as L;
use crate::generic_neat::default::Output as O;
use crate::generic_neat::genome;
use crate::network::evaluate::Evaluator as P;

pub struct Developer;

impl phenotype::Develop<I, H, O, L, P> for Developer {
    fn develop(&self, genome: &genome::Genome<I, H, O, L>) -> P {
        genome.create_evaluator()
    }
}
