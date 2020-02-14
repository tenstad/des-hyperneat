pub mod dataset_environment;
mod phenotype;

use crate::generic_neat;
use crate::generic_neat::environment::Environment;
use crate::network::evaluate::Evaluator as P;

pub fn neat(environment: &dyn Environment<P>) {
    let developer = phenotype::Developer {};
    generic_neat::neat(environment, &developer);
}
