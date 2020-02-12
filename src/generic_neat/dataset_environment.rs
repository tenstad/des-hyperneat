use crate::data::dataset::Dataset;
use crate::data::dataset::Dimensions;
use crate::generic_neat::environment::Environment;
use crate::generic_neat::genome::Genome;
use crate::generic_neat::link;
use crate::generic_neat::node;
use std::fmt;
use std::io::Error;

pub struct DatasetEnvironment {
    name: String,
    dataset: Dataset,
}

impl DatasetEnvironment {
    pub fn init(filename: &String) -> Result<DatasetEnvironment, Error> {
        Ok(DatasetEnvironment {
            name: filename.clone(),
            dataset: Dataset::load(filename)?,
        })
    }
}

impl<I: node::Custom, H: node::Custom, O: node::Custom, L: link::Custom> Environment<I, H, O, L>
    for DatasetEnvironment
{
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_dimensions(&self) -> &Dimensions {
        return &self.dataset.dimensions;
    }

    fn fitness(&self, genome: &Genome<I, H, O, L>) -> f64 {
        let mut evaluator = genome.create_evaluator();

        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| evaluator.evaluate(input))
            .collect();

        1.0 - self.dataset.mse(&outputs)
    }

    fn accuracy(&self, genome: &Genome<I, H, O, L>) -> f64 {
        let mut evaluator = genome.create_evaluator();

        let outputs: Vec<Vec<f64>> = self
            .dataset
            .inputs
            .iter()
            .map(|input| evaluator.evaluate(input))
            .collect();

        self.dataset.acc(&outputs)
    }
}

impl fmt::Display for DatasetEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}
