use crate::data::dataset;
use crate::generic_neat::genome;

pub trait Environment<I, H, O, L> {
    fn get_name(&self) -> &String;
    fn fitness(&self, genome: &genome::Genome<I, H, O, L>) -> f64;
    fn accuracy(&self, genome: &genome::Genome<I, H, O, L>) -> f64;
    fn get_dimensions(&self) -> &dataset::Dimensions;
}
