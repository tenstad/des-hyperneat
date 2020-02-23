pub mod dataset_environment;
pub mod img;
mod log;
mod phenotype;
pub mod substrate;

use crate::generic_neat;
use crate::generic_neat::evaluate;
use crate::network::execute;

pub fn hyperneat<E: evaluate::Environment<execute::Executor> + Default>() {
    generic_neat::neat::<execute::Executor, E, phenotype::Developer, log::Logger>();
}
