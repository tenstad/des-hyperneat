use crate::generic_neat::default::Hidden as H;
use crate::generic_neat::default::Input as I;
use crate::generic_neat::default::Link as L;
use crate::generic_neat::default::Output as O;
use crate::generic_neat::genome;
use crate::generic_neat::phenotype;
use crate::hyperneat::substrate;
use crate::network::evaluate::Evaluator as P;

pub struct Developer {
    network: substrate::Network,
}

impl Developer {
    pub fn create(network: substrate::Network) -> Self {
        Developer { network }
    }
}

impl phenotype::Develop<I, H, O, L, P> for Developer {
    fn develop(&self, genome: &genome::Genome<I, H, O, L>) -> P {
        self.network
            .create_evaluator(&mut genome.create_evaluator())
    }
}
