mod img;
pub mod log;
mod phenotype;
pub mod search;

use crate::generic_neat;
use crate::generic_neat::evaluate;
use network::execute;

pub fn hyperneat<E: evaluate::Environment<execute::Executor> + Default>() {
    generic_neat::neat::<execute::Executor, E, phenotype::Developer, log::Logger>();
}
