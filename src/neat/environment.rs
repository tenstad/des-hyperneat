use crate::neat::genome::Genome;
use crate::data::dataset::Dimensions;

pub trait Environment {
    fn get_name(&self) -> &String;
    fn evaluate(&self, genome: &Genome) -> f64;
    fn get_dimensions(&self) -> &Dimensions;
}
