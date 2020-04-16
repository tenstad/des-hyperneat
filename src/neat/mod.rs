pub mod dataset_environment;
pub mod phenotype;

use crate::generic_neat::{self, evaluate, log};
use crate::network::execute::Executor as P;

pub fn neat<E: evaluate::Environment<P> + Default>() {
    generic_neat::neat::<P, E, phenotype::Developer, log::Logger>();
}
