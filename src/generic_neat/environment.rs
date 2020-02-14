use crate::data::dataset;

pub trait Environment<P> {
    fn get_name(&self) -> &String;
    fn fitness(&self, phenotype: &mut P) -> f64;
    fn accuracy(&self, phenotype: &mut P) -> f64;
    fn get_dimensions(&self) -> &dataset::Dimensions;
}
