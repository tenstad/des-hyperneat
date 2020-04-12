pub mod dataset_environment;
pub mod phenotype;
pub mod log;

use crate::generic_neat;
use crate::generic_neat::evaluate;
use crate::network::execute::Executor as P;

pub fn neat<E: evaluate::Environment<P> + Default>() {
    generic_neat::neat::<P, E, phenotype::Developer, log::Logger>();
}
