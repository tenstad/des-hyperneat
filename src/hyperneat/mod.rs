mod phenotype;
mod environment;
pub mod substrate;

use crate::generic_neat;
use crate::generic_neat::environment::Environment;
use crate::network::evaluate::Evaluator as P;

pub fn hyperneat(environment: &dyn Environment<P>, network: substrate::Network) {
    let developer = phenotype::Developer::create(network);
    let environment = environment::HyperneatEnvironment::from_environment(environment);
    generic_neat::neat(&environment, &developer);
}
